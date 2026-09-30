//! The lane graph: trim radii, directed lanes, and the connectors that join
//! lanes on *different* roads through a junction.
//!
//! This is the module the previous port was missing.  Before it, a junction was
//! three points on a stub 12 m from the node and the renderer drew a 30 %-opacity
//! ghost ribbon; nothing connected a lane to any other lane, so there was no
//! through movement, no turn, and nothing for traffic to follow.  Everything
//! here is ported from the source kernel's `hd-map.js` so the derived topology
//! — and the geometry that expresses it — match the reference.

use std::collections::HashMap;

use urban::{
    CityFrameInfo, HdRoad, JunctionKind, ModernRoadClass, RoadCrossSection, SdNode, SdRoad,
    cross_section,
};

use crate::math::{Path, Rng, Vec2, Vec3, cubic_points, smoothstep};
use crate::spec::{JunctionSpec, Movement, classify_movement, lane_movement_sets};

/// Roadbed elevation of the carriageway surface, above the block datum.  Every
/// other surface in the crate is offset from this so kerbs, sidewalks and
/// parcels line up instead of floating.
pub const ROADBED_Y: f32 = 0.0;
/// Elevation of the junction box, identical to the roadbed: the box is
/// carriageway, not sidewalk.
pub const JUNCTION_Y: f32 = 0.0;
/// Structural deck thickness for a bridge or elevated road.
pub const BRIDGE_DECK: f32 = 1.25;
/// Pavement thickness for an at-grade road, drawn as the ribbon's skirt.
pub const ROAD_PAVEMENT: f32 = 0.25;
/// Vertical clearance an elevated road keeps above the road it crosses.
pub const STRUCTURE_HEIGHT: f32 = 6.4;

/// One directed motor lane, in travel direction, addressed by arc length.
#[derive(Debug, Clone)]
pub struct Lane {
    pub id: String,
    pub road: u32,
    /// Lane index counted from the median outward.
    pub index: u8,
    /// `+1` when the carriageway lies on the centreline's left normal.
    pub direction: i8,
    pub width: f32,
    pub offset: f32,
    pub use_kind: LaneUse,
    pub path: Path,
    pub from_node: u32,
    pub to_node: u32,
    pub allowed: Vec<Movement>,
    pub successors: Vec<String>,
    pub predecessors: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaneUse {
    Through,
    LeftTurn,
    RightTurn,
    Bike,
    Shoulder,
}

/// A turn or through movement between two lanes on different roads.
#[derive(Debug, Clone)]
pub struct Connector {
    pub id: String,
    pub node: u32,
    pub from_lane: String,
    pub to_lane: String,
    pub movement: Movement,
    pub turn_degrees: f32,
    pub path: Path,
    pub width: f32,
}

/// One road end as it meets a junction.
#[derive(Debug, Clone, Copy)]
pub struct Port {
    pub road: u32,
    /// `true` when the node is the road's `from` end.
    pub at_start: bool,
    /// Outward direction along the road, pointing away from the node.
    pub dir: Vec2,
    /// Station of the kerb face on the untrimmed road.
    pub station: f32,
    pub left: Vec2,
    pub right: Vec2,
    pub outer_left: Vec2,
    pub outer_right: Vec2,
    pub angle: f32,
}

/// Everything geometric about one junction.
#[derive(Debug, Clone)]
pub struct Junction {
    pub node: u32,
    pub kind: JunctionKind,
    pub roundabout: bool,
    pub centre: Vec2,
    pub y: f32,
    pub radius: f32,
    pub ports: Vec<Port>,
    /// Kerb face of the junction box: two port corners plus a cubic fillet
    /// between consecutive ports.  Nine points per port, which is the invariant
    /// the sidewalk collar relies on.
    pub ring: Vec<Vec2>,
    /// `ring` pushed out by the sidewalk width, used only for the kerb-return
    /// line and the corner paving.
    pub walk_ring: Vec<Vec2>,
    pub warnings: Vec<String>,
}

/// A road with its cross-section resolved and both ends trimmed back to the
/// junction edge.
#[derive(Debug, Clone)]
pub struct Road {
    pub id: u32,
    pub class: ModernRoadClass,
    pub section: RoadCrossSection,
    pub from_node: u32,
    pub to_node: u32,
    pub layer: i8,
    pub bridge: bool,
    /// Full centreline, kerb face to kerb face, including the junction boxes.
    pub centreline: Path,
    /// `centreline.trim(trim_start, length - trim_end)`.
    pub carriageway: Path,
    pub trim_start: f32,
    pub trim_end: f32,
    pub deck_thickness: f32,
    pub max_grade: f32,
    /// `true` when either end is a real crossing (degree ≥ 3), not a bend.
    pub crossing_start: bool,
    pub crossing_end: bool,
    /// Set-back from the junction edge at which longitudinal paint begins, in
    /// metres, on the **trimmed** `carriageway`.  The crossing, the stop line and
    /// the approach taper own the ground between the edge and this station, so
    /// this is where the road's own markings start.
    pub marking_start: f32,
    /// The same at the far end, as a set-back from that end's edge.
    pub marking_end: f32,
    pub lane_ids: Vec<String>,
}

impl Road {
    /// Does this road get sidewalks, kerbs and street trees?  Elevated
    /// structures and motorways do not.
    pub fn has_sidewalk(&self) -> bool {
        self.layer == 0 && self.section.sidewalk_metres >= 1.0
    }

    pub fn is_motorway(&self) -> bool {
        self.class == ModernRoadClass::Expressway
    }

    pub fn half_width(&self) -> f32 {
        self.section.half_width()
    }
}

/// The whole derived street network for one city.
#[derive(Debug, Default, Clone)]
pub struct Network {
    /// SD node count, so the router can size its distance table.
    pub node_count: usize,
    pub roads: Vec<Road>,
    pub lanes: Vec<Lane>,
    pub connectors: Vec<Connector>,
    pub junctions: Vec<Junction>,
    pub spec: JunctionSpec,
    /// Engineering problems found while deriving — an impossible ramp, a
    /// degenerate trim.  Reported, never hidden.
    pub warnings: Vec<String>,
    road_by_id: HashMap<u32, usize>,
    lane_by_id: HashMap<String, usize>,
}

impl Network {
    pub fn road(&self, id: u32) -> Option<&Road> {
        self.road_by_id.get(&id).map(|index| &self.roads[*index])
    }

    pub fn lane(&self, id: &str) -> Option<&Lane> {
        self.lane_by_id.get(id).map(|index| &self.lanes[*index])
    }

    pub fn junction(&self, node: u32) -> Option<&Junction> {
        self.junctions.iter().find(|junction| junction.node == node)
    }

    /// Lanes arriving at `node`.
    pub fn incoming(&self, node: u32) -> impl Iterator<Item = &Lane> {
        self.lanes.iter().filter(move |lane| lane.to_node == node)
    }

    /// Lanes leaving `node`.
    pub fn outgoing(&self, node: u32) -> impl Iterator<Item = &Lane> {
        self.lanes.iter().filter(move |lane| lane.from_node == node)
    }

    /// Roads incident to `node`.
    pub fn incident_roads(&self, node: u32) -> Vec<u32> {
        let mut result = Vec::new();
        for road in &self.roads {
            if road.from_node == node || road.to_node == node {
                result.push(road.id);
            }
        }
        result
    }

    /// Every lane that is a legal successor of `lane`, resolved through the
    /// connector graph.  Traffic follows this and nothing else.
    pub fn successors_of(&self, lane: &Lane) -> Vec<&Lane> {
        lane.successors
            .iter()
            .filter_map(|id| self.lane(id))
            .collect()
    }

    /// Largest legibility threshold: a junction narrower than this is really a
    /// bend, and drawing a box there is what shreds a street into fragments.
    pub fn max_trim(&self) -> f32 {
        self.junctions.iter().map(|j| j.radius).fold(0.0, f32::max)
    }
}

/// How a road's lanes map onto the movements they can make.
///
/// `index` is counted **from the median outward**, which is the order the
/// source kernel uses (`s = -(medW/2 + (k + 0.5) * laneW)`, k from 0) and the
/// order the `Lane::index` contract states.  Getting it backwards is not a
/// cosmetic bug: the innermost lane is the one that may turn left across oncoming
/// traffic and the curb lane is the one that carries the non-motorized lane and
/// the kerbside stop, so a swap paints a left-turn arrow in the kerb lane and
/// draws the waiting box in the wrong place.
fn lane_use_for(class: ModernRoadClass, index: u8) -> (LaneUse, Vec<Movement>) {
    let count = cross_section(class).motor_lanes_per_direction;
    let outermost = count.saturating_sub(1);
    match (count, index) {
        // Three lanes or more: the median lane is the protected left.
        (c, 0) if c >= 3 => (LaneUse::LeftTurn, vec![Movement::Left, Movement::Straight]),
        // The curb lane is the through-and-right lane on every undivided road.
        (_, i) if i == outermost && count >= 2 => (
            LaneUse::RightTurn,
            vec![Movement::Straight, Movement::Right],
        ),
        // A two-lane approach has no protected left, so its median lane is
        // shared: left and straight together, as Chinese practice has it.
        (2, 0) => (LaneUse::Through, vec![Movement::Left, Movement::Straight]),
        _ => (LaneUse::Through, vec![Movement::Straight]),
    }
}

/// Derive the whole network from the SD/HD plan.
pub fn derive(
    nodes: &[SdNode],
    sd_roads: &[SdRoad],
    hd_roads: &[HdRoad],
    frame: CityFrameInfo,
    spec: JunctionSpec,
    seed: u32,
) -> Network {
    let mut network = Network {
        node_count: nodes.len(),
        spec,
        ..Network::default()
    };
    let local: Vec<Vec2> = nodes
        .iter()
        .map(|node| {
            let [x, z] = frame.to_local(node.point);
            Vec2::new(x, z)
        })
        .collect();

    // --- incidence -----------------------------------------------------------
    let mut incidence: Vec<Vec<u32>> = vec![Vec::new(); nodes.len()];
    let sd_by_id: HashMap<u32, &SdRoad> = sd_roads.iter().map(|road| (road.id, road)).collect();
    for road in sd_roads {
        if let Some(list) = incidence.get_mut(road.from as usize) {
            list.push(road.id);
        }
        if let Some(list) = incidence.get_mut(road.to as usize) {
            list.push(road.id);
        }
    }

    let sections: HashMap<u32, RoadCrossSection> = hd_roads
        .iter()
        .map(|road| (road.id, road.cross_section()))
        .collect();

    // --- trim radii ----------------------------------------------------------
    // A degree-2 node is a continuation or a bend, not an intersection.  Sizing
    // its trim like a crossing is what digs a broken box out of every corner.
    let mut radius = vec![0.0_f32; nodes.len()];
    for (index, list) in incidence.iter().enumerate() {
        if list.len() < 2 {
            continue;
        }
        let widest = list
            .iter()
            .filter_map(|id| sections.get(id))
            .map(|section| section.width_metres)
            .fold(0.0_f32, f32::max);
        let mut value = if list.len() > 2 {
            widest * 0.8 + 4.0
        } else {
            (widest * 0.5 + 2.0).min(8.0)
        };
        let shortest = list
            .iter()
            .filter_map(|id| sd_by_id.get(id))
            .map(|road| {
                let other = if road.from as usize == index {
                    road.to as usize
                } else {
                    road.from as usize
                };
                local[index].distance(local.get(other).copied().unwrap_or_default())
            })
            .fold(f32::MAX, f32::min);
        if shortest.is_finite() {
            value = value.min(shortest * 0.4);
        }
        radius[index] = value;
    }

    // --- roads ---------------------------------------------------------------
    for hd in hd_roads {
        let Some(sd) = sd_by_id.get(&hd.id) else {
            continue;
        };
        let from_index = sd.from as usize;
        let to_index = sd.to as usize;
        if from_index >= local.len() || to_index >= local.len() || from_index == to_index {
            continue;
        }
        let section = hd.cross_section();
        let path_points: Vec<Vec2> = hd
            .centreline
            .iter()
            .map(|point| {
                let [x, z] = frame.to_local(*point);
                Vec2::new(x, z)
            })
            .collect();
        if path_points.len() < 2 {
            continue;
        }
        // An elevated road gets its height per vertex, and a plan centreline can
        // be as sparse as its two ends and a midpoint. Interpolating a ramp
        // between such vertices is a straight ramp from the junction to the crown,
        // which leaves the carriageway hanging metres off the ground at the
        // junction. A vertex every three metres lets the ramp ease in properly and
        // meet the junction at road level.
        let path_points: Vec<Vec2> = if hd.layer != 0 {
            let mut dense = vec![path_points[0]];
            for pair in path_points.windows(2) {
                let steps = ((pair[0].distance(pair[1]) / 3.0).ceil() as usize).max(1);
                for step in 1..=steps {
                    let t = step as f32 / steps as f32;
                    dense.push(pair[0] * (1.0 - t) + pair[1] * t);
                }
            }
            dense
        } else {
            path_points
        };
        let full_length = Path::flat(path_points.clone()).length();
        let trim_start = radius[from_index];
        let trim_end = radius[to_index];
        if full_length - trim_start - trim_end < 6.0 {
            // Too short to carry a carriageway between two boxes; the
            // intersection geometry itself will cover the gap.
            continue;
        }

        // Elevated roads ramp up away from the junction and hold a plateau
        // across whatever they cross, so a bridge never dips into traffic.
        let elevated = hd.layer != 0;
        let mut rise_end = full_length * 0.4;
        let mut fall_start = full_length * 0.6;
        if elevated {
            let crossings = crossing_stations(&sd_roads, &sections, &local, hd.id);
            if crossings.is_empty() {
                // A pure river crossing has nothing to attach to.  Putting the
                // plateau at mid-span leaves the approaches long enough that the
                // quintic's 1.875x peak slope stays inside a real gradient.
                rise_end = full_length * 0.5 - 5.0;
                fall_start = full_length * 0.5 + 5.0;
            } else {
                rise_end = crossings
                    .iter()
                    .map(|(station, width)| station - width * 0.5 - 10.0)
                    .fold(f32::INFINITY, f32::min);
                fall_start = crossings
                    .iter()
                    .map(|(station, width)| station + width * 0.5 + 10.0)
                    .fold(f32::NEG_INFINITY, f32::max);
            }
        }
        let start_y = ROADBED_Y;
        let end_y = ROADBED_Y;
        let deck = hd.layer as f32 * STRUCTURE_HEIGHT;
        let rise_len = (rise_end - trim_start - 10.0).max(1.0);
        let fall_len = (full_length - trim_end - 10.0 - fall_start).max(1.0);
        // Elevate along each centreline vertex's own station so a bridge ramps
        // up away from the junction, holds a plateau across whatever it
        // crosses, and comes back down before the next one.
        let mut vertices: Vec<Vec3> = Vec::with_capacity(path_points.len());
        let mut station = 0.0_f32;
        for index in 0..path_points.len() {
            if index > 0 {
                station += path_points[index - 1].distance(path_points[index]);
            }
            let elevation = if elevated {
                let rise = smoothstep((station - trim_start - 10.0) / rise_len);
                let fall = smoothstep((full_length - trim_end - 10.0 - station) / fall_len);
                let t = station / full_length.max(1.0);
                start_y + (end_y - start_y) * t + deck * rise.min(fall)
            } else {
                ROADBED_Y
            };
            vertices.push(Vec3::from_plan(path_points[index], elevation));
        }

        let centreline = Path::new(vertices);
        let carriageway = centreline.trim(trim_start, full_length - trim_end);
        let crossing_start = incidence[from_index].len() > 2;
        let crossing_end = incidence[to_index].len() > 2;
        let max_grade = centreline.max_grade();
        if max_grade > 0.065 {
            // Reported rather than silently accepted: the source project is
            // explicit that a short approach does not excuse an impossible ramp.
            network.warn(format!(
                "road {} reaches {:.1}% grade; extend the approach or lower the structure",
                hd.id,
                max_grade * 100.0
            ));
        }
        let id = hd.id;
        network.roads.push(Road {
            id,
            class: hd.class,
            section,
            from_node: sd.from,
            to_node: sd.to,
            layer: hd.layer,
            bridge: hd.bridge,
            centreline,
            carriageway,
            trim_start,
            trim_end,
            deck_thickness: if elevated { BRIDGE_DECK } else { ROAD_PAVEMENT },
            max_grade,
            crossing_start,
            crossing_end,
            // Seven metres of clear approach at a crossing: the 4 m crossing plus
            // the stop line and its clearance, which is exactly the ground the
            // crossing and the queue own.  A bend gets half a metre.
            marking_start: if crossing_start { 7.0 } else { 0.5 },
            marking_end: if crossing_end { 7.0 } else { 0.5 },
            lane_ids: Vec::new(),
        });
        network.road_by_id.insert(id, network.roads.len() - 1);
    }

    // --- lanes ---------------------------------------------------------------
    // A lane path starts and ends exactly at the kerb face of the junction box,
    // so its arc length is the distance a vehicle actually travels between two
    // decision points.
    for road_index in 0..network.roads.len() {
        let (road_id, section, from_node, to_node) = {
            let road = &network.roads[road_index];
            (road.id, road.section, road.from_node, road.to_node)
        };
        let class = network.roads[road_index].class;
        let carriageway = network.roads[road_index].carriageway.clone();
        let length = carriageway.length();
        let count = section.motor_lanes_per_direction;
        let mut ids = Vec::with_capacity((count as usize + 1) * 2);
        for direction in [1_i8, -1_i8] {
            for index in 0..count {
                // `RoadCrossSection::lane_offset` counts from the *kerb* inward,
                // which is the opposite of this crate's `Lane::index` contract
                // and of the source kernel's lane order.  Converting here, once,
                // is what keeps `index == 0` meaning "the median lane" for the
                // arrow stencils, the waiting box, the guide lanes and the
                // exported payload alike.
                let offset = section.lane_offset(direction, (count - 1 - index) as u8);
                // A lane path starts and ends exactly at the kerb face of the
                // junction box, so its arc length is the distance a vehicle
                // really travels between two decision points.
                let steps = ((length / 10.0).ceil() as usize).max(1);
                let mut points: Vec<Vec2> = Vec::with_capacity(steps + 1);
                for step in 0..=steps {
                    let station = length * step as f32 / steps as f32;
                    points.push(
                        carriageway.plan_at(station) + carriageway.tangent_at(station).left_normal() * offset,
                    );
                }
                if direction < 0 {
                    // Right-hand traffic: the `+1` carriageway runs forward along
                    // the centreline, the `-1` one runs back.
                    points.reverse();
                }
                let path = Path::flat(points);
                let (use_kind, _) = lane_use_for(class, index);
                let id = format!("road/{road_id}/lane/{}/{index}", if direction > 0 { "f" } else { "b" });
                ids.push(id.clone());
                network.lanes.push(Lane {
                    id,
                    road: road_id,
                    index,
                    direction,
                    width: section.motor_lane_width,
                    offset,
                    use_kind,
                    path,
                    from_node: if direction > 0 { from_node } else { to_node },
                    to_node: if direction > 0 { to_node } else { from_node },
                    allowed: Vec::new(),
                    successors: Vec::new(),
                    predecessors: Vec::new(),
                });
            }
            if let Some(offset) = section.bike_offset(direction) {
                let steps = 4;
                let mut points = Vec::with_capacity(steps + 1);
                for step in 0..=steps {
                    let station = length * step as f32 / steps as f32;
                    let position = carriageway.plan_at(station);
                    let tangent = carriageway.tangent_at(station);
                    points.push(position + tangent.left_normal() * offset);
                }
                if direction < 0 {
                    points.reverse();
                }
                let id = format!("road/{road_id}/lane/{}/bike", if direction > 0 { "f" } else { "b" });
                ids.push(id.clone());
                network.lanes.push(Lane {
                    id,
                    road: road_id,
                    index: 0,
                    direction,
                    width: section.bike_lane_width,
                    offset,
                    use_kind: LaneUse::Bike,
                    path: Path::flat(points),
                    from_node: if direction > 0 { from_node } else { to_node },
                    to_node: if direction > 0 { to_node } else { from_node },
                    allowed: vec![Movement::Straight],
                    successors: Vec::new(),
                    predecessors: Vec::new(),
                });
            }
        }
        network.roads[road_index].lane_ids = ids;
    }

    // --- connectors ----------------------------------------------------------
    for node_index in 0..local.len() {
        let node = node_index as u32;
        let incoming: Vec<usize> = network
            .lanes
            .iter()
            .enumerate()
            .filter(|(_, lane)| lane.to_node == node)
            .map(|(index, _)| index)
            .collect();
        if incoming.is_empty() {
            continue;
        }
        let outgoing: Vec<usize> = network
            .lanes
            .iter()
            .enumerate()
            .filter(|(_, lane)| lane.from_node == node)
            .map(|(index, _)| index)
            .collect();
        if outgoing.is_empty() {
            continue;
        }
        for &lane_index in &incoming {
            let lane = network.lanes[lane_index].clone();
            let dir = lane.path.tangent_at(lane.path.length() - 0.01);
            if dir.length_squared() < 0.5 {
                continue;
            }
            // Every candidate is a lane on a *different* road, so a U-turn back
            // down the same carriageway is never generated.
            let mut candidates: Vec<(usize, Movement, f32, Vec2)> = Vec::new();
            for &target_index in &outgoing {
                let target = &network.lanes[target_index];
                if target.road == lane.road || target.use_kind != LaneUse::Through
                    && target.use_kind != LaneUse::LeftTurn
                    && target.use_kind != LaneUse::RightTurn
                {
                    continue;
                }
                let out = target.path.tangent_at(0.01);
                if out.length_squared() < 0.5 {
                    continue;
                }
                let (movement, degrees) = classify_movement(dir, out);
                if movement == Movement::UTurn {
                    continue;
                }
                candidates.push((target_index, movement, degrees, out));
            }
            if candidates.is_empty() {
                continue;
            }
            let available: Vec<Movement> = {
                let mut list: Vec<Movement> = candidates.iter().map(|c| c.1).collect();
                list.sort();
                list.dedup();
                list
            };
            let inbound_on_road = network
                .lanes
                .iter()
                .filter(|other| other.road == lane.road && other.to_node == node)
                .count();
            // Reserved turn bays, decided **per approach** and not per lane.
            //
            // `armLaneMovementSets` in the source kernel hands the whole arm one
            // pair of counts, and `laneMovementSets` then distributes them.  The
            // port first derived them from whichever lane happened to be
            // iterating, so a three-lane approach produced three different
            // distributions and the curb lane came out right-turn-only — a lane
            // that cannot go straight, on a road whose curb lane is the through
            // lane for most of its length.
            //
            // A protected left bay needs three lanes or more (`leftTurnCapacity`),
            // and this network has no channelised right-turn bay, so
            // `dedicated_right` is always zero and the curb lane doubles as the
            // right-turn lane, which is what an undivided Chinese street does.
            let dedicated_left = if available.contains(&Movement::Left) && inbound_on_road >= 3 {
                1_usize
            } else {
                0
            };
            let dedicated_right = 0_usize;
            let allowed = lane_movement_sets(
                inbound_on_road,
                &available,
                dedicated_left,
                dedicated_right,
            );
            let Some(allowed_set) = allowed.get(lane.index as usize) else {
                continue;
            };

            // Group candidates by target road so each exit road produces one
            // connector, exactly as the source kernel does.
            let mut target_roads: Vec<u32> = candidates.iter().map(|c| network.lanes[c.0].road).collect();
            target_roads.sort_unstable();
            target_roads.dedup();
            for target_road in target_roads {
                let mut choices: Vec<&(usize, Movement, f32, Vec2)> = candidates
                    .iter()
                    .filter(|c| network.lanes[c.0].road == target_road)
                    .collect();
                choices.sort_by_key(|c| network.lanes[c.0].index);
                let Some(first) = choices.first().copied() else {
                    continue;
                };
                if !allowed_set.contains(&first.1) {
                    continue;
                }
                // A right turn always feeds the curb-most exit lane; every other
                // movement mirrors the entry index, so a left turn from the
                // median lane lands in the exit road's median lane — which is
                // the one capable of turning left again, and is the only
                // assignment that makes two consecutive left turns legal.
                let target_index = if first.1 == Movement::Right {
                    choices.len() - 1
                } else {
                    (lane.index as usize).min(choices.len() - 1)
                };
                let Some(choice) = choices.get(target_index).copied() else {
                    continue;
                };
                let (target_lane_index, movement, _degrees, out) = *choice;
                let a = lane.path.end();
                let b = network.lanes[target_lane_index].path.start();
                let gap = Vec2::new(b.x - a.x, b.z - a.z);
                let distance = gap.length();
                if distance < 0.5 {
                    continue;
                }
                // Fillet handles: bend the connector where the two lane tangents
                // actually meet, so the path hugs the corner instead of bulging
                // across the box.  A straight movement uses a symmetric handle
                // and stays straight.
                let cross = dir.cross(out);
                let (handle, handle_out) = if movement == Movement::Straight || cross.abs() < 0.08 {
                    let h = distance * if movement == Movement::Straight { 0.33 } else { 0.45 };
                    (h, h)
                } else {
                    let along_in = gap.cross(out) / cross;
                    let along_out = (-gap).cross(dir) / cross;
                    (
                        (distance * 0.8).min((along_in.abs() * 0.55).max(1.5)),
                        (distance * 0.8).min((along_out.abs() * 0.55).max(1.5)),
                    )
                };
                // Twenty samples, exactly as the source kernel's connector
                // derivation: fewer bends the shoulder of the fillet flat and
                // the traffic kernel then cuts a corner no car could take.
                let plan = cubic_points(
                    Vec2::new(a.x, a.z),
                    Vec2::new(a.x, a.z) + dir * handle,
                    Vec2::new(b.x, b.z) - out * handle_out,
                    Vec2::new(b.x, b.z),
                    20,
                );
                let y_from = a.y;
                let y_to = b.y;
                let steps = plan.len() - 1;
                let points: Vec<Vec3> = plan
                    .iter()
                    .enumerate()
                    .map(|(index, point)| {
                        let t = index as f32 / steps.max(1) as f32;
                        Vec3::from_plan(*point, lerp_height(y_from, y_to, smoothstep(t)))
                    })
                    .collect();
                let id = format!(
                    "{}>{}",
                    network.lanes[lane_index].id, network.lanes[target_lane_index].id
                );
                let from_id = network.lanes[lane_index].id.clone();
                let to_id = network.lanes[target_lane_index].id.clone();
                if network.connectors.iter().any(|c| c.id == id) {
                    continue;
                }
                network.lanes[lane_index].successors.push(to_id.clone());
                network.lanes[target_lane_index].predecessors.push(from_id.clone());
                network.connectors.push(Connector {
                    id,
                    node,
                    from_lane: from_id,
                    to_lane: to_id,
                    movement,
                    turn_degrees: classify_movement(dir, out).1,
                    path: Path::new(points),
                    width: lane.width,
                });
            }
        }
    }

    // Fill in each lane's allowed movement set from the connectors that leave
    // it, so arrows, the signal programme and traffic all read one answer.
    for index in 0..network.lanes.len() {
        if network.lanes[index].use_kind != LaneUse::Through
            && network.lanes[index].use_kind != LaneUse::LeftTurn
            && network.lanes[index].use_kind != LaneUse::RightTurn
        {
            continue;
        }
        let mut movements: Vec<Movement> = network
            .connectors
            .iter()
            .filter(|c| c.from_lane == network.lanes[index].id)
            .map(|c| c.movement)
            .collect();
        movements.sort();
        movements.dedup();
        network.lanes[index].allowed = movements;
    }

    for (id, lane) in network.lanes.iter().enumerate() {
        network.lane_by_id.insert(lane.id.clone(), id);
    }

    // --- junctions -----------------------------------------------------------
    let mut rng = Rng::new(seed ^ 0x6a75_6e63);
    for node_index in 0..local.len() {
        let node = node_index as u32;
        let centre = local[node_index];
        let mut ports: Vec<Port> = Vec::new();
        for road in &network.roads {
            if road.from_node != node && road.to_node != node {
                continue;
            }
            if road.layer != 0 {
                // An elevated road passes over; it has no kerb face at this
                // level and must not grow a second box.
                continue;
            }
            let at_start = road.from_node == node;
            let station = if at_start {
                road.trim_start
            } else {
                road.centreline.length() - road.trim_end
            };
            let inward = if at_start {
                road.centreline
                    .tangent_at((road.trim_start + 2.0).min(road.centreline.length()))
            } else {
                -road.centreline.tangent_at(
                    (road.centreline.length() - road.trim_end - 2.0).max(0.0),
                )
            };
            let outward = -inward;
            let side = if at_start { -1.0 } else { 1.0 };
            let half = road.half_width();
            let outer = half + road.section.sidewalk_metres;
            let left = road.centreline.offset_at(station, side * half, 0.0);
            let right = road.centreline.offset_at(station, -side * half, 0.0);
            let outer_left = road.centreline.offset_at(station, side * outer, 0.0);
            let outer_right = road.centreline.offset_at(station, -side * outer, 0.0);
            ports.push(Port {
                road: road.id,
                at_start,
                dir: outward,
                station,
                left: Vec2::new(left.x, left.z),
                right: Vec2::new(right.x, right.z),
                outer_left: Vec2::new(outer_left.x, outer_left.z),
                outer_right: Vec2::new(outer_right.x, outer_right.z),
                angle: Vec2::new(left.x - centre.x, left.z - centre.y).angle(),
            });
        }
        if ports.is_empty() {
            continue;
        }
        ports.sort_by(|a, b| a.angle.total_cmp(&b.angle));

        let roundabout = ports.len() >= 4 && radius[node_index] > 14.0 && {
            rng.fork(node);
            rng.chance(0.20)
        };
        let mut ring: Vec<Vec2> = Vec::with_capacity(ports.len() * 9);
        let mut walk_ring: Vec<Vec2> = Vec::with_capacity(ports.len() * 9);
        for index in 0..ports.len() {
            let port = ports[index];
            ring.push(port.left);
            ring.push(port.right);
            walk_ring.push(port.outer_left);
            walk_ring.push(port.outer_right);
            let next = ports[(index + 1) % ports.len()];
            let chord = port.right.distance(next.left);
            let handle = chord * 0.45;
            let corner = if roundabout {
                // A roundabout's kerb is a circle: sweep between the two mouth
                // corners on an arc about the centre, radius blended between them,
                // so the carriageway is round instead of a square box.
                let (d0, d1) = (port.right - centre, next.left - centre);
                let (r0, r1) = (d0.length().max(0.5), d1.length().max(0.5));
                let a0 = d0.y.atan2(d0.x);
                let mut delta = d1.y.atan2(d1.x) - a0;
                while delta > std::f32::consts::PI {
                    delta -= std::f32::consts::TAU;
                }
                while delta < -std::f32::consts::PI {
                    delta += std::f32::consts::TAU;
                }
                let r_mid = (r0 + r1) * 0.5;
                (0..=7)
                    .map(|i| {
                        let t = i as f32 / 7.0;
                        // Bulge to the mean radius in the middle of the arc.
                        let bulge = (std::f32::consts::PI * t).sin();
                        let r = r0 + (r1 - r0) * t + (r_mid.max(r0.max(r1)) - (r0 + (r1 - r0) * t)) * bulge * 0.5;
                        let angle = a0 + delta * t;
                        centre + Vec2::new(angle.cos(), angle.sin()) * r
                    })
                    .collect::<Vec<Vec2>>()
            } else {
            cubic_points(
                port.right,
                // `dir` points from the road *towards* the junction (it is the
                // negated into-the-road tangent), so the handles run along each
                // kerb line into the box.  With the opposite sign the corner
                // bows away from the junction and reads as a convex blob rather
                // than a kerb return.
                port.right + port.dir * handle,
                next.left + next.dir * handle,
                next.left,
                8,
            )
            };
            // Drop the two endpoints; they are already the port corners.
            for point in corner.iter().skip(1).take(7) {
                ring.push(*point);
                // The walk ring is the same fillet pushed radially outward by
                // the approaching road's own sidewalk width plus 300 mm — the
                // source kernel's `sidewalkWidth + 0.3`.  Refitting a fresh
                // cubic out here folds the control-point sag back into the box
                // and floods the junction with pavement, so the radial push is
                // load-bearing, not cosmetic.  A fixed width would leave the
                // collar of a wide arterial footway floating short of the
                // block corner it has to meet.
                let walk = network
                    .road(port.road)
                    .map(|road| road.section.sidewalk_metres)
                    .unwrap_or(2.4);
                let delta = *point - centre;
                let distance = delta.length().max(0.01);
                let scale = (distance + walk + 0.3) / distance;
                walk_ring.push(centre + delta * scale);
            }
        }
        let kind = if roundabout {
            JunctionKind::Roundabout
        } else if ports.len() >= 4
            && network
                .incident_roads(node)
                .iter()
                .filter_map(|id| network.road(*id))
                .any(|road| road.layer != 0)
        {
            JunctionKind::GradeSeparated
        } else if ports.len() >= 3 {
            let has_major = network
                .incident_roads(node)
                .iter()
                .filter_map(|id| network.road(*id))
                .any(|road| {
                    matches!(road.class, ModernRoadClass::Arterial | ModernRoadClass::Expressway)
                });
            if has_major {
                JunctionKind::Signalized
            } else {
                JunctionKind::Channelized
            }
        } else {
            JunctionKind::Channelized
        };
        network.junctions.push(Junction {
            node,
            kind,
            roundabout,
            centre,
            y: JUNCTION_Y,
            radius: radius[node_index],
            ports,
            ring,
            walk_ring,
            warnings: Vec::new(),
        });
    }

    network
}

impl Network {
    fn warn(&mut self, warning: String) {
        if !self.warnings.contains(&warning) {
            self.warnings.push(warning);
        }
    }
}

fn lerp_height(from: f32, to: f32, t: f32) -> f32 {
    from + (to - from) * t
}

/// Stations where `id`'s segment crosses a road on a different layer, with the
/// crossed road's width.  Used to place a bridge's plateau.
fn crossing_stations(
    sd_roads: &[SdRoad],
    sections: &HashMap<u32, RoadCrossSection>,
    local: &[Vec2],
    id: u32,
) -> Vec<(f32, f32)> {
    let Some(road) = sd_roads.iter().find(|road| road.id == id) else {
        return Vec::new();
    };
    let (Some(a), Some(b)) = (
        local.get(road.from as usize).copied(),
        local.get(road.to as usize).copied(),
    ) else {
        return Vec::new();
    };
    let length = a.distance(b);
    if length < 1.0e-3 {
        return Vec::new();
    }
    let mut result = Vec::new();
    for other in sd_roads {
        if other.id == id {
            continue;
        }
        let (Some(c), Some(d)) = (
            local.get(other.from as usize).copied(),
            local.get(other.to as usize).copied(),
        ) else {
            continue;
        };
        // Only a road on a *different* level creates a vertical conflict; a
        // same-level crossing is a junction, not a bridge constraint.
        if crossing_layers_are_compatible(sd_roads, id, other.id) {
            continue;
        }
        let Some((t, _)) = segment_intersection(a, b, c, d) else {
            continue;
        };
        if !(0.01..0.99).contains(&t) {
            continue;
        }
        let width = sections
            .get(&other.id)
            .map(|section| section.width_metres)
            .unwrap_or(20.0);
        result.push((t * length, width));
    }
    result
}

/// A crossing needs vertical separation only when one of the two roads is
/// elevated.  `urban` already split same-level crossings into shared nodes, so
/// at this point any remaining intersection is either a genuine bridge or a
/// genuine at-grade junction.
fn crossing_layers_are_compatible(sd_roads: &[SdRoad], a: u32, b: u32) -> bool {
    let layer = |id: u32| {
        sd_roads
            .iter()
            .find(|road| road.id == id)
            .map(|road| road.bridge)
            .unwrap_or(false)
    };
    layer(a) == layer(b)
}

pub fn segment_intersection(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> Option<(f32, f32)> {
    let ab = b - a;
    let cd = d - c;
    let denominator = ab.cross(cd);
    if denominator.abs() < 1.0e-9 {
        return None;
    }
    let ac = c - a;
    let t = ac.cross(cd) / denominator;
    let u = ac.cross(ab) / denominator;
    if !(0.0..=1.0).contains(&t) || !(0.0..=1.0).contains(&u) {
        return None;
    }
    Some((t, u))
}

/// Compact projection of the network for the render payload and the traffic
/// kernel.  Paths are emitted as flat metre triples relative to the city origin
/// so the renderer needs no further coordinate conversion.
pub mod export {
    use super::{Connector, Junction, Lane, Network, Road};
    use crate::math::Path;
    use serde::Serialize;

    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LaneRecord {
        pub id: String,
        pub road: u32,
        pub index: u8,
        pub direction: i8,
        pub width: f32,
        pub offset: f32,
        pub kind: &'static str,
        pub length: f32,
        pub from_node: u32,
        pub to_node: u32,
        pub allowed: Vec<&'static str>,
        pub successors: Vec<String>,
        pub predecessors: Vec<String>,
        pub points: Vec<[f32; 3]>,
    }

    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ConnectorRecord {
        pub id: String,
        pub node: u32,
        pub from_lane: String,
        pub to_lane: String,
        pub movement: &'static str,
        pub width: f32,
        pub points: Vec<[f32; 3]>,
    }

    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct JunctionRecord {
        pub node: u32,
        pub kind: &'static str,
        pub roundabout: bool,
        pub centre: [f32; 2],
        pub radius: f32,
        pub ring: Vec<[f32; 2]>,
        pub walk_ring: Vec<[f32; 2]>,
        pub ports: Vec<PortRecord>,
    }

    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PortRecord {
        pub road: u32,
        pub at_start: bool,
        pub station: f32,
        pub left: [f32; 2],
        pub right: [f32; 2],
        pub outer_left: [f32; 2],
        pub outer_right: [f32; 2],
    }

    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RoadRecord {
        pub id: u32,
        pub class: &'static str,
        pub width: f32,
        pub median: f32,
        pub sidewalk: f32,
        pub layer: i8,
        pub bridge: bool,
        pub length: f32,
        pub trim_start: f32,
        pub trim_end: f32,
        pub deck: f32,
        pub max_grade: f32,
        pub crossing_start: bool,
        pub crossing_end: bool,
        pub has_sidewalk: bool,
        pub centreline: Vec<[f32; 3]>,
    }

    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NetworkRecord {
        pub roads: Vec<RoadRecord>,
        pub lanes: Vec<LaneRecord>,
        pub connectors: Vec<ConnectorRecord>,
        pub junctions: Vec<JunctionRecord>,
        pub warnings: Vec<String>,
    }

    fn points(path: &Path) -> Vec<[f32; 3]> {
        path.points()
            .iter()
            .map(|point| [point.x, point.y, point.z])
            .collect()
    }

    fn plan(point: crate::math::Vec2) -> [f32; 2] {
        [point.x, point.y]
    }

    pub fn lane_kind(lane: &Lane) -> &'static str {
        match lane.use_kind {
            super::LaneUse::Through => "through",
            super::LaneUse::LeftTurn => "leftTurn",
            super::LaneUse::RightTurn => "rightTurn",
            super::LaneUse::Bike => "bike",
            super::LaneUse::Shoulder => "shoulder",
        }
    }

    pub fn junction_kind(junction: &Junction) -> &'static str {
        match junction.kind {
            urban::JunctionKind::Signalized => "signalized",
            urban::JunctionKind::Channelized => "channelized",
            urban::JunctionKind::Roundabout => "roundabout",
            urban::JunctionKind::GradeSeparated => "gradeSeparated",
        }
    }

    pub fn road_class(class: urban::ModernRoadClass) -> &'static str {
        match class {
            urban::ModernRoadClass::Expressway => "expressway",
            urban::ModernRoadClass::Arterial => "arterial",
            urban::ModernRoadClass::Collector => "collector",
            urban::ModernRoadClass::Local => "local",
        }
    }

    fn lane_record(lane: &Lane) -> LaneRecord {
        LaneRecord {
            id: lane.id.clone(),
            road: lane.road,
            index: lane.index,
            direction: lane.direction,
            width: lane.width,
            offset: lane.offset,
            kind: lane_kind(lane),
            length: lane.path.length(),
            from_node: lane.from_node,
            to_node: lane.to_node,
            allowed: lane.allowed.iter().map(|m| m.as_str()).collect(),
            successors: lane.successors.clone(),
            predecessors: lane.predecessors.clone(),
            points: points(&lane.path),
        }
    }

    fn connector_record(connector: &Connector) -> ConnectorRecord {
        ConnectorRecord {
            id: connector.id.clone(),
            node: connector.node,
            from_lane: connector.from_lane.clone(),
            to_lane: connector.to_lane.clone(),
            movement: connector.movement.as_str(),
            width: connector.width,
            points: points(&connector.path),
        }
    }

    fn junction_record(junction: &Junction) -> JunctionRecord {
        JunctionRecord {
            node: junction.node,
            kind: junction_kind(junction),
            roundabout: junction.roundabout,
            centre: plan(junction.centre),
            radius: junction.radius,
            ring: junction.ring.iter().copied().map(plan).collect(),
            walk_ring: junction.walk_ring.iter().copied().map(plan).collect(),
            ports: junction
                .ports
                .iter()
                .map(|port| PortRecord {
                    road: port.road,
                    at_start: port.at_start,
                    station: port.station,
                    left: plan(port.left),
                    right: plan(port.right),
                    outer_left: plan(port.outer_left),
                    outer_right: plan(port.outer_right),
                })
                .collect(),
        }
    }

    fn road_record(road: &Road) -> RoadRecord {
        RoadRecord {
            id: road.id,
            class: road_class(road.class),
            width: road.section.width_metres,
            median: road.section.median_metres,
            sidewalk: road.section.sidewalk_metres,
            layer: road.layer,
            bridge: road.bridge,
            length: road.centreline.length(),
            trim_start: road.trim_start,
            trim_end: road.trim_end,
            deck: road.deck_thickness,
            max_grade: road.max_grade,
            crossing_start: road.crossing_start,
            crossing_end: road.crossing_end,
            has_sidewalk: road.has_sidewalk(),
            centreline: points(&road.centreline),
        }
    }

    impl Network {
        pub fn record(&self) -> NetworkRecord {
            NetworkRecord {
                roads: self.roads.iter().map(road_record).collect(),
                lanes: self.lanes.iter().map(lane_record).collect(),
                connectors: self.connectors.iter().map(connector_record).collect(),
                junctions: self.junctions.iter().map(junction_record).collect(),
                warnings: self.warnings.clone(),
            }
        }
    }
}
