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

use crate::math::{Path, Rng, Vec2, Vec3, cubic_points};
use crate::spec::{JunctionSpec, Movement, classify_movement, lane_movement_sets};
use worldgen_core::smootherstep;

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
/// Vertical clearance an elevated road keeps above the road it crosses.  Only a
/// grade-separated crossing needs it; a bridge over water does not.
pub const STRUCTURE_HEIGHT: f32 = 6.4;
/// Deck surface of an expressway viaduct above the street it runs over: 5 m
/// of clearance for a bus and the signal heads under it (CJJ 37 gives 5.0 m
/// for motor traffic), a 2 m box girder and its surfacing. The reference
/// photographs of Shenzhen and Shanghai put the soffit at 7 - 8 m.
pub const VIADUCT_Y: f32 = 9.0;
/// Grade a ramp falls at between its vertical curves (CJJ 129 allows 5 - 6 %
/// on a 40 km/h ramp; 4.5 % leaves a margin under the limit).
const RAMP_GRADE: f32 = 0.045;
/// Length of each vertical curve at the top and foot of a ramp.
const VERTICAL_CURVE_M: f32 = 40.0;
/// A ramp is level with the deck for this long past the end of its taper, so
/// the nose where it parts from the deck is flat.
const RAMP_HOLD_M: f32 = 30.0;
/// The gap left between two decks that meet in the air.
const AIR_GAP_M: f32 = 0.5;
/// A ramp reaches the ground this far before the junction it lands at.
const RAMP_LANDING_M: f32 = 15.0;
/// Steepest longitudinal grade an elevated road may reach: 5 %, the design limit
/// this crate assumes for urban elevated roads.  A ramp steeper than this is
/// reported as a warning.
const MAX_GRADE: f32 = 0.05;
/// A deck lower than this is a causeway at quay level, not a bridge: 0.3 m.
pub const MIN_DECK: f32 = 0.3;
/// Below this height a structure is a girder bridge with no towers or cables.
pub const LOW_DECK: f32 = 3.0;
/// Clearance a deck keeps above a road it passes over.
const CROSSING_CLEARANCE: f32 = 4.5;
/// Peak slope of `smootherstep`, relative to its average slope.  A ramp that
/// rises `h` over a run `r` has a steepest point of `SMOOTHSTEP_PEAK * h / r`.
const SMOOTHSTEP_PEAK: f32 = 1.875;

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
    /// Direction out of the road into the junction: toward the node.
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
    /// One way, from `from_node` to `to_node`: an expressway ramp.
    pub oneway: bool,
    /// Full centreline, kerb face to kerb face, including the junction boxes.
    pub centreline: Path,
    /// `centreline.trim(trim_start, length - trim_end)`.
    pub carriageway: Path,
    pub trim_start: f32,
    pub trim_end: f32,
    pub deck_thickness: f32,
    /// Height of the deck above roadbed at its highest point, in metres.  Zero for
    /// an at-grade road.  The deck is sized to the ramp the span can carry, so a
    /// short bridge is a low one.
    pub deck_height: f32,
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
    /// Nodes in the air, by node index: where decks meet with no junction box.
    pub air: Vec<bool>,
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

    /// Is `node` in the air, where decks meet with no junction box?
    pub fn is_air(&self, node: u32) -> bool {
        self.air.get(node as usize).copied().unwrap_or(false)
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
    // A street the plan drew as a chain of short pieces, one per bend, is one
    // street: built piece by piece it was trimmed at every bend, so its median,
    // markings and kerbs broke every 80 - 100 m.
    let (sd_merged, hd_merged) = merge_continuations(nodes.len(), sd_roads, hd_roads);
    let (sd_roads, hd_roads) = (sd_merged.as_slice(), hd_merged.as_slice());
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

    // --- elevation of each node ---------------------------------------------
    // A node every road of which is on a structure is in the air: where an
    // expressway's ramps leave and join it, or where two pieces of a viaduct
    // meet. It has no junction box; the decks meet there at viaduct height.
    let layer_of_road: HashMap<u32, i8> =
        hd_roads.iter().map(|road| (road.id, road.layer)).collect();
    let air: Vec<bool> = incidence
        .iter()
        .map(|list| {
            list.len() >= 2
                && list
                    .iter()
                    .all(|id| layer_of_road.get(id).copied().unwrap_or(0) != 0)
                && list
                    .iter()
                    .all(|id| sd_by_id.get(id).is_some_and(|road| !road.bridge))
        })
        .collect();
    let node_y = |node: usize| if air[node] { VIADUCT_Y } else { ROADBED_Y };
    network.air = air.clone();

    // --- junction control -----------------------------------------------------
    // Decided from the streets meeting at a node, before anything is drawn, so a
    // street end knows whether it stops, gives way or runs straight through.
    let layer_of: HashMap<u32, i8> = hd_roads.iter().map(|road| (road.id, road.layer)).collect();
    let ramp_ids: std::collections::HashSet<u32> = hd_roads
        .iter()
        .filter(|road| road.oneway)
        .map(|road| road.id)
        .collect();
    let control: Vec<Control> = incidence
        .iter()
        .map(|list| {
            let classes: Vec<ModernRoadClass> = list
                .iter()
                .filter_map(|id| sd_by_id.get(id))
                .filter(|road| layer_of.get(&road.id).copied().unwrap_or(0) == 0)
                .map(|road| road.class)
                .collect();
            // Where ramps come down to a street, the terminal is signalised (a
            // diamond's two terminals run as one coordinated pair) and the
            // street is the major road through it.
            if list.iter().any(|id| ramp_ids.contains(id)) {
                return Control {
                    kind: JunctionKind::Signalized,
                    major: classes
                        .iter()
                        .map(|class| class_rank(*class))
                        .min()
                        .unwrap_or(1),
                    terminal: true,
                };
            }
            control_for(&classes)
        })
        .collect();
    let control = space_median_openings(control, &incidence, &sd_by_id, hd_roads, frame);

    let sections: HashMap<u32, RoadCrossSection> = hd_roads
        .iter()
        .map(|road| (road.id, road.cross_section()))
        .collect();

    // --- trim radii ----------------------------------------------------------
    // A degree-2 node is a continuation or a bend, not an intersection.  Sizing
    // its trim like a crossing is what digs a broken box out of every corner.
    let mut radius = vec![0.0_f32; nodes.len()];
    for (index, list) in incidence.iter().enumerate() {
        if air[index] {
            // Decks meet end to end in the air: a hair of a gap, so the lane
            // joining them has a length.
            radius[index] = AIR_GAP_M;
            continue;
        }
        if list.len() < 2 {
            continue;
        }
        let mut widths: Vec<f32> = list
            .iter()
            .filter_map(|id| sections.get(id))
            .map(|section| section.width_metres)
            .collect();
        widths.sort_by(|a, b| b.total_cmp(a));
        let widest = widths.first().copied().unwrap_or(0.0);
        let second = widths.get(1).copied().unwrap_or(0.0);
        // The same number the street planner sizes its links by: the widest
        // street's half width plus the kerb-return radius.
        let _ = second;
        let mut value = urban::junction_trim_m(widest, list.len());
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
        let mut crosses_road = false;
        if elevated {
            let crossings = crossing_stations(sd_roads, &sections, &local, hd.id);
            crosses_road = !crossings.is_empty();
            if crossings.is_empty() {
                // A pure river crossing has nothing to attach to, so the plateau
                // sits at mid-span.  That is the best place for it, but it cannot
                // rescue a span too short for its deck: no plateau position does,
                // see the bound in the grade warning below.
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
        let rise_len = (rise_end - trim_start - 10.0).max(1.0);
        let fall_len = (full_length - trim_end - 10.0 - fall_start).max(1.0);
        // The tallest deck whose ramps stay within the grade limit.  A ramp that
        // rises `h` over `r` peaks at `SMOOTHSTEP_PEAK * h / r`, so the limit gives
        // `h <= MAX_GRADE * r / SMOOTHSTEP_PEAK` on the shorter of the two ramps.
        // A span too short for a ramp takes the causeway floor and reports below.
        // An expressway flies over at viaduct height; any other structure at the
        // height its clearance needs.
        let ceiling = if hd.class == ModernRoadClass::Expressway {
            VIADUCT_Y
        } else {
            STRUCTURE_HEIGHT
        };
        let (y_from, y_to) = (node_y(from_index), node_y(to_index));
        // A deck that meets another in the air: level between two air nodes, or
        // a ramp from the deck to the ground (`RampProfile`).
        let ramp = (elevated && (y_from > 0.0 || y_to > 0.0))
            .then(|| RampProfile::new(full_length, trim_start, trim_end, y_from, y_to));
        let deck = if let Some(ramp) = &ramp {
            ramp.top
        } else if elevated {
            (MAX_GRADE * rise_len.min(fall_len) / SMOOTHSTEP_PEAK).clamp(MIN_DECK, ceiling)
        } else {
            0.0
        };
        // Elevate along each centreline vertex's own station so a bridge ramps
        // up away from the junction, holds a plateau across whatever it
        // crosses, and comes back down before the next one.
        let mut vertices: Vec<Vec3> = Vec::with_capacity(path_points.len());
        let mut station = 0.0_f32;
        for index in 0..path_points.len() {
            if index > 0 {
                station += path_points[index - 1].distance(path_points[index]);
            }
            let elevation = if let Some(ramp) = &ramp {
                ramp.height(station)
            } else if elevated {
                let rise = smootherstep((station - trim_start - 10.0) / rise_len);
                let fall = smootherstep((full_length - trim_end - 10.0 - station) / fall_len);
                let t = station / full_length.max(1.0);
                start_y + (end_y - start_y) * t + deck * rise.min(fall)
            } else {
                ROADBED_Y
            };
            vertices.push(Vec3::from_plan(path_points[index], elevation));
        }

        let centreline = Path::new(vertices);
        let carriageway = centreline.trim(trim_start, full_length - trim_end);
        // An end is a controlled approach (a crosswalk and a stop or give-way
        // line) at a signal, and at an unsignalised junction only when it is a
        // minor arm: the major street runs through without stopping.
        let controlled = |node: usize| {
            incidence[node].len() > 2 && {
                let c = control[node];
                c.kind == JunctionKind::Signalized || class_rank(hd.class) > c.major
            }
        };
        let crossing_start = controlled(from_index);
        let crossing_end = controlled(to_index);
        // A bridge over another road must clear it.  A deck the span cannot lift
        // to that height would drive through the road below, so say so.
        if crosses_road && deck < CROSSING_CLEARANCE {
            network.warn(format!(
                "road {} crosses a road on a {:.1} m deck, below the {:.1} m clearance it needs; the planner should lengthen the span",
                hd.id, deck, CROSSING_CLEARANCE
            ));
        }
        let max_grade = centreline.max_grade();
        if max_grade > MAX_GRADE {
            // Reported rather than silently accepted.  A span this short cannot
            // carry even a causeway at the grade limit: the planner should lengthen
            // the span or not cross the water here.
            network.warn(format!(
                "road {} reaches {:.1}% grade on a {:.1} m deck; its {:.0} m span is too short for a ramp at {:.1}%",
                hd.id,
                max_grade * 100.0,
                deck,
                carriageway.length(),
                MAX_GRADE * 100.0
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
            oneway: hd.oneway,
            centreline,
            carriageway,
            trim_start,
            trim_end,
            deck_thickness: if elevated { BRIDGE_DECK } else { ROAD_PAVEMENT },
            deck_height: deck,
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
        let oneway = network.roads[road_index].oneway;
        let mut ids = Vec::with_capacity((count as usize + 1) * 2);
        // A one-way ramp has lanes in its own direction only, centred on its deck.
        let directions: &[i8] = if oneway { &[1] } else { &[1, -1] };
        for &direction in directions {
            for index in 0..count {
                // `RoadCrossSection::lane_offset` counts from the *kerb* inward,
                // which is the opposite of this crate's `Lane::index` contract
                // and of the source kernel's lane order.  Converting here, once,
                // is what keeps `index == 0` meaning "the median lane" for the
                // arrow stencils, the waiting box, the guide lanes and the
                // exported payload alike.
                let offset = if oneway {
                    // Index 0 on the driver's left, as on a carriageway with a
                    // median: the kerb side is the centreline's left normal.
                    (index as f32 - (count as f32 - 1.0) * 0.5) * section.motor_lane_width
                } else {
                    section.lane_offset(direction, count - 1 - index)
                };
                // A lane path starts and ends exactly at the kerb face of the
                // junction box, so its arc length is the distance a vehicle
                // really travels between two decision points.
                // On the deck's own surface, so a lane on a viaduct is driven
                // at the viaduct's height and a ramp's lane falls with it.
                let steps = ((length / 10.0).ceil() as usize).max(1);
                let mut points: Vec<Vec3> = Vec::with_capacity(steps + 1);
                for step in 0..=steps {
                    let station = length * step as f32 / steps as f32;
                    let (at, tangent) = carriageway.sample(station);
                    let side = tangent.left_normal() * offset;
                    points.push(Vec3::new(at.x + side.x, at.y, at.z + side.y));
                }
                if direction < 0 {
                    // Right-hand traffic: the `+1` carriageway runs forward along
                    // the centreline, the `-1` one runs back.
                    points.reverse();
                }
                let path = Path::new(points);
                let (use_kind, _) = lane_use_for(class, index);
                let id = format!(
                    "road/{road_id}/lane/{}/{index}",
                    if direction > 0 { "f" } else { "b" }
                );
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
                let id = format!(
                    "road/{road_id}/lane/{}/bike",
                    if direction > 0 { "f" } else { "b" }
                );
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
        if air[node_index] {
            free_flow_connectors(&mut network, node);
            continue;
        }
        if incidence[node_index].len() == 1 {
            dead_end_turns(&mut network, node);
            continue;
        }
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
            let here = control[node_index];
            let rank_of = |road: u32| network.road(road).map(|r| class_rank(r.class)).unwrap_or(3);
            for &target_index in &outgoing {
                let target = &network.lanes[target_index];
                // Across an unbroken median only right turns are made: out of the
                // side street, and into it.
                if here.kind == JunctionKind::RightInRightOut && local.len() > node_index {
                    let into_minor = rank_of(target.road) > here.major;
                    let from_minor = rank_of(lane.road) > here.major;
                    if into_minor || from_minor {
                        let out = target.path.tangent_at(0.01);
                        if classify_movement(dir, out).0 != Movement::Right {
                            continue;
                        }
                    }
                }
                if target.road == lane.road
                    || target.use_kind != LaneUse::Through
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
            let allowed =
                lane_movement_sets(inbound_on_road, &available, dedicated_left, dedicated_right);
            let Some(allowed_set) = allowed.get(lane.index as usize) else {
                continue;
            };

            // Group candidates by target road so each exit road produces one
            // connector, exactly as the source kernel does.
            let mut target_roads: Vec<u32> =
                candidates.iter().map(|c| network.lanes[c.0].road).collect();
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
                    let h = distance
                        * if movement == Movement::Straight {
                            0.33
                        } else {
                            0.45
                        };
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
                        Vec3::from_plan(*point, lerp_height(y_from, y_to, smootherstep(t)))
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
                network.lanes[target_lane_index]
                    .predecessors
                    .push(from_id.clone());
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
        // Decks meeting in the air have no junction box.
        if air[node_index] {
            continue;
        }
        let centre = local[node_index];
        let mut ports: Vec<Port> = Vec::new();
        for road in &network.roads {
            if road.from_node != node && road.to_node != node {
                continue;
            }
            // A river bridge *ends* at this node and lands in its box like any
            // other arm, ramping down to road level; a flyover that merely passes
            // over has no kerb face here and must not grow a second box.
            // A ramp lands here and is an arm like any other.
            if road.layer != 0 && !road.bridge && !road.oneway {
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
                -road
                    .centreline
                    .tangent_at((road.centreline.length() - road.trim_end - 2.0).max(0.0))
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
                        let r = r0
                            + (r1 - r0) * t
                            + (r_mid.max(r0.max(r1)) - (r0 + (r1 - r0) * t)) * bulge * 0.5;
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
                .any(|road| road.layer != 0 && !road.oneway)
        {
            JunctionKind::GradeSeparated
        } else if ports.len() >= 3 {
            control[node_index].kind
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

/// How a junction is controlled, and the rank of the street that has priority.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Control {
    /// A ramp terminal: its spacing from the next signal is the interchange's.
    terminal: bool,
    kind: JunctionKind,
    /// `class_rank` of the major street; an arm of a lower rank is a minor arm.
    major: u8,
}

pub(crate) fn class_rank(class: ModernRoadClass) -> u8 {
    match class {
        ModernRoadClass::Expressway => 0,
        ModernRoadClass::Arterial => 1,
        ModernRoadClass::Collector => 2,
        ModernRoadClass::Local => 3,
    }
}

/// The control of an at-grade junction from the classes of the streets meeting
/// at it, as Chinese streets are controlled (CJJ 37, CJJ 152):
///
/// - wherever an arterial (or expressway) meets anything but local streets, and
///   where four or more collector arms meet, there is a signal;
/// - a local street on an arterial that runs through is right-in, right-out
///   across an unbroken median;
/// - everything else gives way to its major street (让行).
///
/// The major street is the best class present; an arm of a lower class is a
/// minor arm.
fn control_for(classes: &[ModernRoadClass]) -> Control {
    let ranks: Vec<u8> = classes.iter().map(|c| class_rank(*c)).collect();
    let Some(&major) = ranks.iter().min() else {
        return Control {
            terminal: false,
            kind: JunctionKind::Channelized,
            major: 3,
        };
    };
    let count = |r: u8| ranks.iter().filter(|x| **x == r).count();
    let minors_all_local = ranks.iter().all(|r| *r == major || *r == 3);
    let kind = if major <= 1 {
        if count(major) >= 2 && minors_all_local && count(3) > 0 {
            JunctionKind::RightInRightOut
        } else {
            JunctionKind::Signalized
        }
    } else if major == 2 && count(2) >= 4 {
        JunctionKind::Signalized
    } else {
        JunctionKind::Priority
    };
    Control {
        terminal: false,
        kind,
        major,
    }
}

/// An arterial's median opens for a signal no closer than 300 m to the next
/// opening (CJJ 37 §6.2.8, 断口间距). Where two signalised junctions on one
/// arterial are closer, the one whose cross street is the lesser becomes
/// right-in, right-out; a crossing of two arterials is never closed.
fn space_median_openings(
    mut control: Vec<Control>,
    incidence: &[Vec<u32>],
    sd_by_id: &HashMap<u32, &SdRoad>,
    hd_roads: &[HdRoad],
    frame: CityFrameInfo,
) -> Vec<Control> {
    const OPENING_M: f32 = 300.0;
    let length: HashMap<u32, f32> = hd_roads
        .iter()
        .map(|road| {
            let pts: Vec<Vec2> = road
                .centreline
                .iter()
                .map(|p| {
                    let [x, z] = frame.to_local(*p);
                    Vec2::new(x, z)
                })
                .collect();
            (road.id, pts.windows(2).map(|w| w[0].distance(w[1])).sum())
        })
        .collect();
    // The best cross street at a node: the lowest rank among arms that are not
    // the major street.
    let cross = |node: usize, major: u8| {
        incidence[node]
            .iter()
            .filter_map(|id| sd_by_id.get(id))
            .map(|road| class_rank(road.class))
            .filter(|r| *r > major)
            .min()
            .unwrap_or(major)
    };
    let opening = |c: &Control| c.kind == JunctionKind::Signalized && c.major <= 1 && !c.terminal;
    for node in 0..incidence.len() {
        if !opening(&control[node]) {
            continue;
        }
        let major = control[node].major;
        for &first in &incidence[node] {
            let Some(road) = sd_by_id.get(&first) else {
                continue;
            };
            if class_rank(road.class) != major {
                continue;
            }
            // Walk along the arterial to the next opening.
            let (mut at, mut via, mut walked) = (node, first, 0.0_f32);
            let next = loop {
                let Some(road) = sd_by_id.get(&via) else {
                    break None;
                };
                walked += length.get(&via).copied().unwrap_or(0.0);
                let far = if road.from as usize == at {
                    road.to as usize
                } else {
                    road.from as usize
                };
                if walked >= OPENING_M || far >= control.len() {
                    break None;
                }
                if opening(&control[far]) {
                    break Some(far);
                }
                let onward = incidence[far].iter().copied().find(|id| {
                    *id != via
                        && sd_by_id
                            .get(id)
                            .is_some_and(|r| class_rank(r.class) == major)
                });
                match onward {
                    Some(id) => {
                        at = far;
                        via = id;
                    }
                    None => break None,
                }
            };
            let Some(other) = next else {
                continue;
            };
            let (mine, theirs) = (cross(node, major), cross(other, control[other].major));
            // The lesser cross street closes; a tie closes the one with fewer arms.
            let close_mine =
                mine > theirs || (mine == theirs && incidence[node].len() < incidence[other].len());
            let target = if close_mine { node } else { other };
            if cross(target, control[target].major) > 1 {
                control[target].kind = JunctionKind::RightInRightOut;
            }
            if close_mine {
                break;
            }
        }
    }
    control
}

/// Join every chain of streets that meet two at a time, end to end, at nodes
/// where nothing else meets them, into one street with one centreline.
///
/// Only like is joined to like: the same class, the same layer, neither a
/// bridge, both one-way and end to start or neither. A node where a third
/// street meets, or where the class or layer changes, stays a junction. The
/// joined street keeps the id, lanes and cross section of its first piece. A
/// chain that closes on itself (a ring road with nothing on it) is joined as
/// two halves, so no street starts and ends at one node.
fn merge_continuations(
    node_count: usize,
    sd_roads: &[SdRoad],
    hd_roads: &[HdRoad],
) -> (Vec<SdRoad>, Vec<HdRoad>) {
    let hd_by_id: HashMap<u32, &HdRoad> = hd_roads.iter().map(|road| (road.id, road)).collect();
    let mut incidence: Vec<Vec<usize>> = vec![Vec::new(); node_count];
    for (index, road) in sd_roads.iter().enumerate() {
        for node in [road.from as usize, road.to as usize] {
            if node < node_count {
                incidence[node].push(index);
            }
        }
    }
    let joinable = |a: usize, b: usize, node: usize| {
        let (ra, rb) = (&sd_roads[a], &sd_roads[b]);
        let (Some(ha), Some(hb)) = (hd_by_id.get(&ra.id), hd_by_id.get(&rb.id)) else {
            return false;
        };
        let end_to_start =
            |x: &SdRoad, y: &SdRoad| x.to as usize == node && y.from as usize == node;
        a != b
            && ra.class == rb.class
            && !ra.bridge
            && !rb.bridge
            && ha.layer == hb.layer
            && ha.oneway == hb.oneway
            && (!ha.oneway || end_to_start(ra, rb) || end_to_start(rb, ra))
    };
    // A node a chain passes through.
    let through = |node: usize| {
        let list = &incidence[node];
        list.len() == 2 && joinable(list[0], list[1], node)
    };
    let mut used = vec![false; sd_roads.len()];
    let mut sd_out = Vec::with_capacity(sd_roads.len());
    let mut hd_out = Vec::with_capacity(hd_roads.len());
    for start in 0..sd_roads.len() {
        if used[start] || !hd_by_id.contains_key(&sd_roads[start].id) {
            continue;
        }
        // Walk back to the chain's first piece, so a chain is joined once and in
        // order. A closed ring has no first piece; it starts here.
        let mut first = start;
        let mut first_from = sd_roads[start].from as usize;
        let mut guard = 0;
        loop {
            guard += 1;
            if guard > sd_roads.len() || !through(first_from) {
                break;
            }
            let list = &incidence[first_from];
            let prev = if list[0] == first { list[1] } else { list[0] };
            if prev == start || used[prev] {
                break;
            }
            let road = &sd_roads[prev];
            first_from = if road.to as usize == first_from {
                road.from as usize
            } else {
                road.to as usize
            };
            first = prev;
        }
        // Walk forward, gathering the centreline.
        let mut pieces: Vec<(usize, bool)> = Vec::new();
        let mut current = first;
        let mut at = first_from;
        loop {
            used[current] = true;
            let road = &sd_roads[current];
            let forward = road.from as usize == at;
            pieces.push((current, forward));
            let next_node = if forward {
                road.to as usize
            } else {
                road.from as usize
            };
            if !through(next_node) {
                at = next_node;
                break;
            }
            let list = &incidence[next_node];
            let next = if list[0] == current { list[1] } else { list[0] };
            if used[next] {
                at = next_node;
                break;
            }
            current = next;
            at = next_node;
        }
        // A closed chain is joined as two halves, meeting at the node where the
        // second half starts.
        let halves: Vec<&[(usize, bool)]> = if at == first_from && pieces.len() >= 2 {
            let (a, b) = pieces.split_at(pieces.len() / 2);
            vec![a, b]
        } else {
            vec![&pieces[..]]
        };
        for half in halves {
            let head = &sd_roads[half[0].0];
            let head_hd = hd_by_id[&head.id];
            let mut centreline: Vec<urban::Point> = Vec::new();
            for (index, forward) in half {
                let hd = hd_by_id[&sd_roads[*index].id];
                let mut line = hd.centreline.clone();
                if !forward {
                    line.reverse();
                }
                if !centreline.is_empty() && !line.is_empty() {
                    line.remove(0);
                }
                centreline.extend(line);
            }
            let end = |piece: (usize, bool), last: bool| {
                let road = &sd_roads[piece.0];
                if piece.1 == last { road.to } else { road.from }
            };
            sd_out.push(SdRoad {
                id: head.id,
                from: end(half[0], false),
                to: end(half[half.len() - 1], true),
                class: head.class,
                bridge: head.bridge,
            });
            // The first piece may run against the chain; its lanes are kept as
            // they are, and they are symmetric about the centreline for every
            // class but a one-way ramp, whose pieces only ever join end to start.
            hd_out.push(HdRoad {
                id: head.id,
                sd_road: head.id,
                centreline,
                ..head_hd.clone()
            });
        }
    }
    (sd_out, hd_out)
}

/// Lane connections where decks meet in the air, with no junction: each main
/// lane runs on into the lane of the same index; a ramp that leaves takes the
/// kerb-side lane of the carriageway running its way, and one that joins runs
/// into it (exits and entries are on the right, CJJ 129).
///
/// A ramp's deck end is beside the main deck, a taper's length from the node,
/// so its connector is the lane change a driver makes along the taper.
fn free_flow_connectors(network: &mut Network, node: u32) {
    let motor = |lane: &Lane| lane.use_kind != LaneUse::Bike && lane.use_kind != LaneUse::Shoulder;
    let incoming: Vec<usize> = (0..network.lanes.len())
        .filter(|&i| network.lanes[i].to_node == node && motor(&network.lanes[i]))
        .collect();
    let outgoing: Vec<usize> = (0..network.lanes.len())
        .filter(|&i| network.lanes[i].from_node == node && motor(&network.lanes[i]))
        .collect();
    let oneway = |lane: usize| {
        network
            .road(network.lanes[lane].road)
            .is_some_and(|road| road.oneway)
    };
    let end_dir = |lane: &Lane| lane.path.tangent_at(lane.path.length() - 0.01);
    let start_dir = |lane: &Lane| lane.path.tangent_at(0.01);
    let mut links: Vec<(usize, usize, Movement)> = Vec::new();
    for &a in &incoming {
        let lane = &network.lanes[a];
        let dir = end_dir(lane);
        // Lanes leaving the node the same way, by road.
        let mut roads: Vec<u32> = outgoing
            .iter()
            .map(|&b| network.lanes[b].road)
            .filter(|&road| road != lane.road)
            .collect();
        roads.sort_unstable();
        roads.dedup();
        for road in roads {
            let mut targets: Vec<usize> = outgoing
                .iter()
                .copied()
                .filter(|&b| network.lanes[b].road == road)
                .collect();
            targets.sort_by_key(|&b| network.lanes[b].index);
            let Some(&first) = targets.first() else {
                continue;
            };
            if start_dir(&network.lanes[first]).dot(dir) < 0.3 {
                continue;
            }
            let kerb_in = incoming
                .iter()
                .filter(|&&x| network.lanes[x].road == lane.road)
                .map(|&x| network.lanes[x].index)
                .max()
                .unwrap_or(0);
            match (oneway(a), oneway(first)) {
                // Main line on into main line, lane for lane.
                (false, false) => {
                    if let Some(&b) = targets.get(lane.index as usize) {
                        links.push((a, b, Movement::Straight));
                    }
                }
                // Off the main line: from its kerb-side lane only.
                (false, true) => {
                    if lane.index == kerb_in {
                        for &b in &targets {
                            links.push((a, b, Movement::Right));
                        }
                    }
                }
                // Onto the main line: into its kerb-side lane.
                (true, false) => {
                    if let Some(&b) = targets.last() {
                        links.push((a, b, Movement::Straight));
                    }
                }
                // A ramp on into a ramp.
                (true, true) => {
                    if let Some(&b) = targets.get(lane.index as usize) {
                        links.push((a, b, Movement::Straight));
                    }
                }
            }
        }
    }
    for (a, b, movement) in links {
        let (from, to) = (network.lanes[a].clone(), network.lanes[b].clone());
        let (p, q) = (from.path.end(), to.path.start());
        let (dir, out) = (end_dir(&from), start_dir(&to));
        let gap = Vec2::new(q.x - p.x, q.z - p.z).length();
        let handle = (gap * 0.4).max(0.2);
        let plan = cubic_points(
            Vec2::new(p.x, p.z),
            Vec2::new(p.x, p.z) + dir * handle,
            Vec2::new(q.x, q.z) - out * handle,
            Vec2::new(q.x, q.z),
            20,
        );
        let steps = plan.len() - 1;
        let points: Vec<Vec3> = plan
            .iter()
            .enumerate()
            .map(|(index, point)| {
                let t = index as f32 / steps.max(1) as f32;
                Vec3::from_plan(*point, lerp_height(p.y, q.y, t))
            })
            .collect();
        let id = format!("{}>{}", from.id, to.id);
        if network.connectors.iter().any(|c| c.id == id) {
            continue;
        }
        network.lanes[a].successors.push(to.id.clone());
        network.lanes[b].predecessors.push(from.id.clone());
        network.connectors.push(Connector {
            id,
            node,
            from_lane: from.id.clone(),
            to_lane: to.id.clone(),
            movement,
            turn_degrees: classify_movement(dir, out).1,
            path: Path::new(points),
            width: from.width,
        });
    }
}

/// A street that ends goes back the way it came: each lane arriving at the
/// end turns into the lane of the same index leaving it, round a turning head.
/// Without it a vehicle that reached a dead end had nowhere to go.
fn dead_end_turns(network: &mut Network, node: u32) {
    let motor = |lane: &Lane| lane.use_kind != LaneUse::Bike && lane.use_kind != LaneUse::Shoulder;
    let pairs: Vec<(usize, usize)> = (0..network.lanes.len())
        .filter(|&a| network.lanes[a].to_node == node && motor(&network.lanes[a]))
        .filter_map(|a| {
            let lane = &network.lanes[a];
            (0..network.lanes.len())
                .find(|&b| {
                    let other = &network.lanes[b];
                    other.from_node == node
                        && other.road == lane.road
                        && other.index == lane.index
                        && motor(other)
                })
                .map(|b| (a, b))
        })
        .collect();
    for (a, b) in pairs {
        let (from, to) = (network.lanes[a].clone(), network.lanes[b].clone());
        let (p, q) = (from.path.end(), to.path.start());
        let dir = from.path.tangent_at(from.path.length() - 0.01);
        let out = to.path.tangent_at(0.01);
        let across = Vec2::new(q.x - p.x, q.z - p.z).length();
        // Out past the end and round: the handles reach as far as the turn is wide.
        let reach = across * 0.9 + 3.0;
        let plan = cubic_points(
            Vec2::new(p.x, p.z),
            Vec2::new(p.x, p.z) + dir * reach,
            Vec2::new(q.x, q.z) - out * reach,
            Vec2::new(q.x, q.z),
            20,
        );
        let points: Vec<Vec3> = plan
            .iter()
            .enumerate()
            .map(|(index, point)| {
                let t = index as f32 / 20.0;
                Vec3::from_plan(*point, lerp_height(p.y, q.y, t))
            })
            .collect();
        let id = format!("{}>{}", from.id, to.id);
        network.lanes[a].successors.push(to.id.clone());
        network.lanes[b].predecessors.push(from.id.clone());
        network.connectors.push(Connector {
            id,
            node,
            from_lane: from.id.clone(),
            to_lane: to.id.clone(),
            movement: Movement::UTurn,
            turn_degrees: 180.0,
            path: Path::new(points),
            width: from.width,
        });
    }
}

/// The long section of a deck that meets another in the air.
///
/// Between two air nodes it is level at viaduct height. From an air node to the
/// ground it is a ramp as a road engineer draws one: level beside the deck for
/// the taper, then a parabolic vertical curve into a constant grade and another
/// out of it, landing short of the junction. A short ramp steepens rather than
/// leaves the deck in the air, and the grade check reports it.
struct RampProfile {
    top: f32,
    length: f32,
    /// Station of the ground end's kerb face, and which end that is.
    ground_at: f32,
    ground_is_end: bool,
    /// Grade and vertical-curve length of the fall.
    grade: f32,
    curve: f32,
    level: Option<(f32, f32)>,
}

impl RampProfile {
    fn new(length: f32, trim_start: f32, trim_end: f32, y_from: f32, y_to: f32) -> Self {
        let top = y_from.max(y_to);
        if y_from > 0.0 && y_to > 0.0 {
            return Self {
                top,
                length,
                ground_at: 0.0,
                ground_is_end: true,
                grade: 0.0,
                curve: 1.0,
                level: Some((y_from, y_to)),
            };
        }
        let ground_is_end = y_to <= 0.0;
        let ground_at = if ground_is_end {
            length - trim_end
        } else {
            trim_start
        };
        // The run the fall has: from the end of the level hold beside the deck
        // to the landing short of the ground end's junction.
        let run = (length - trim_start - trim_end - RAMP_HOLD_M - RAMP_LANDING_M).max(20.0);
        let curve = VERTICAL_CURVE_M.min(run / 3.0);
        let wanted = top / RAMP_GRADE + curve;
        let grade = if run >= wanted {
            RAMP_GRADE
        } else {
            top / (run - curve)
        };
        Self {
            top,
            length,
            ground_at,
            ground_is_end,
            grade,
            curve,
            level: None,
        }
    }

    fn height(&self, station: f32) -> f32 {
        if let Some((a, b)) = self.level {
            return a + (b - a) * (station / self.length.max(1.0));
        }
        // Distance up the ramp from its landing.
        let from_ground = if self.ground_is_end {
            self.ground_at - station
        } else {
            station - self.ground_at
        } - RAMP_LANDING_M;
        let (g, lv, h) = (self.grade, self.curve, self.top);
        let run = h / g + lv;
        if from_ground <= 0.0 {
            ROADBED_Y
        } else if from_ground >= run {
            h
        } else if from_ground < lv {
            g * from_ground * from_ground / (2.0 * lv)
        } else if from_ground < run - lv {
            g * lv * 0.5 + g * (from_ground - lv)
        } else {
            h - g * (run - from_ground).powi(2) / (2.0 * lv)
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
            urban::JunctionKind::Priority => "priority",
            urban::JunctionKind::RightInRightOut => "rightInRightOut",
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::JunctionSpec;
    use urban::{ModernChinaSpec, generate_modern_chinese_city};

    /// The default-radius cities the grade report is quoted for.
    const SEEDS: [u32; 3] = [42, 1, 7];

    fn derived(seed: u32) -> Network {
        let city = generate_modern_chinese_city(ModernChinaSpec {
            seed,
            radius_km: 0.5,
            block_size_metres: 110.0,
            ..ModernChinaSpec::default()
        });
        derive(
            &city.nodes,
            &city.sd_roads,
            &city.hd_roads,
            city.frame,
            JunctionSpec::default(),
            city.seed,
        )
    }

    #[test]
    fn junctions_are_controlled_by_the_streets_that_meet() {
        use ModernRoadClass::{Arterial as A, Collector as C, Local as L};
        let kind = |classes: &[ModernRoadClass]| control_for(classes).kind;
        // A local street on a through arterial turns right in and right out.
        assert_eq!(kind(&[A, A, L]), JunctionKind::RightInRightOut);
        assert_eq!(kind(&[A, A, L, L]), JunctionKind::RightInRightOut);
        // Major streets meeting, or a collector arm on an arterial, are signalised.
        assert_eq!(kind(&[A, A, A, A]), JunctionKind::Signalized);
        assert_eq!(kind(&[A, A, C]), JunctionKind::Signalized);
        assert_eq!(kind(&[A, C, C]), JunctionKind::Signalized);
        assert_eq!(kind(&[C, C, C, C]), JunctionKind::Signalized);
        // Everything else gives way.
        assert_eq!(kind(&[C, C, L]), JunctionKind::Priority);
        assert_eq!(kind(&[C, C, C]), JunctionKind::Priority);
        assert_eq!(kind(&[L, L, L, L]), JunctionKind::Priority);
    }

    #[test]
    fn arterial_median_openings_are_three_hundred_metres_apart() {
        // CJJ 37 §6.2.8: no two signals on one arterial closer than 300 m,
        // except where two arterials cross.
        let mut closed = 0;
        for seed in SEEDS {
            let network = derived(seed);
            let arterial = |road: u32| {
                network
                    .road(road)
                    .is_some_and(|r| r.class == ModernRoadClass::Arterial)
            };
            let ramp = |road: u32| network.road(road).is_some_and(|r| r.oneway);
            let openings: Vec<&Junction> = network
                .junctions
                .iter()
                .filter(|j| j.kind == JunctionKind::Signalized)
                // A diamond's terminals are spaced by the interchange.
                .filter(|j| !j.ports.iter().any(|p| ramp(p.road)))
                .filter(|j| j.ports.iter().filter(|p| arterial(p.road)).count() == 2)
                .filter(|j| j.ports.len() >= 3)
                .collect();
            closed += network
                .junctions
                .iter()
                .filter(|j| j.kind == JunctionKind::RightInRightOut)
                .count();
            for road in network
                .roads
                .iter()
                .filter(|r| r.class == ModernRoadClass::Arterial)
            {
                let at = |node: u32| openings.iter().any(|j| j.node == node);
                let both = at(road.from_node) && at(road.to_node);
                assert!(
                    !both || road.centreline.length() >= 300.0,
                    "seed {seed}: arterial {} has signals {} m apart",
                    road.id,
                    road.centreline.length()
                );
            }
        }
        assert!(closed > 0, "no opening was ever closed");
    }

    #[test]
    fn a_right_in_right_out_junction_only_turns_right_into_or_out_of_its_side_street() {
        let mut checked = 0;
        for seed in SEEDS {
            let network = derived(seed);
            let rank = |lane: &str| {
                network
                    .lane(lane)
                    .and_then(|l| network.road(l.road))
                    .map(|r| class_rank(r.class))
                    .unwrap_or(3)
            };
            for junction in network
                .junctions
                .iter()
                .filter(|j| j.kind == JunctionKind::RightInRightOut)
            {
                let major = junction
                    .ports
                    .iter()
                    .filter_map(|p| network.road(p.road))
                    .map(|r| class_rank(r.class))
                    .min()
                    .unwrap_or(3);
                for connector in network
                    .connectors
                    .iter()
                    .filter(|c| c.node == junction.node)
                {
                    let minor =
                        rank(&connector.from_lane) > major || rank(&connector.to_lane) > major;
                    if minor {
                        assert_eq!(
                            connector.movement,
                            Movement::Right,
                            "seed {seed}: {} across the median at node {}",
                            connector.id,
                            junction.node
                        );
                        checked += 1;
                    }
                }
            }
        }
        assert!(
            checked > 0,
            "no right-in, right-out junction in the test cities"
        );
    }

    #[test]
    fn a_street_is_one_road_from_junction_to_junction() {
        // Built piece by piece, a street was trimmed at every bend and its median
        // and paint broke there. No node may join exactly two at-grade, non-bridge
        // roads of one class: that is a bend, and the two are one road.
        for seed in SEEDS {
            let network = derived(seed);
            let mut at: HashMap<u32, Vec<&Road>> = HashMap::new();
            for road in &network.roads {
                at.entry(road.from_node).or_default().push(road);
                at.entry(road.to_node).or_default().push(road);
            }
            let bends = at
                .values()
                .filter(|list| {
                    list.len() == 2
                        && list[0].id != list[1].id
                        && list[0].class == list[1].class
                        && list.iter().all(|r| r.layer == 0 && !r.bridge)
                })
                .count();
            assert_eq!(bends, 0, "seed {seed}: {bends} bends still split a street");
        }
    }

    #[test]
    fn elevated_roads_meet_the_roadbed_at_both_junction_edges() {
        // Where an elevated road meets its junction box it must already be at
        // roadbed level, and where it meets another deck in the air (no box,
        // two or more decks) it must be at viaduct height, the same as the deck
        // it meets.  The tolerance is 1 mm, well under anything visible.
        for seed in SEEDS {
            let network = derived(seed);
            let degree = |node: u32| network.incident_roads(node).len();
            for road in network.roads.iter().filter(|road| road.layer != 0) {
                let length = road.centreline.length();
                for (station, node) in [
                    (road.trim_start, road.from_node),
                    (length - road.trim_end, road.to_node),
                ] {
                    let (point, _) = road.centreline.sample(station);
                    let in_air = degree(node) >= 2 && network.junction(node).is_none();
                    let expected = if in_air { VIADUCT_Y } else { ROADBED_Y };
                    assert!(
                        (point.y - expected).abs() < 1.0e-3,
                        "seed {seed}: road {} stands {:.3} m above roadbed where it meets \
                         its {} (station {station:.1} m)",
                        road.id,
                        point.y,
                        if in_air { "deck" } else { "junction box" }
                    );
                }
            }
        }
    }

    #[test]
    fn an_expressway_meets_no_street_and_is_reached_by_ramps() {
        // CJJ 129: an expressway is fully access-controlled. Its main line meets
        // no street at grade, every ramp leaves or joins it on the kerb side in
        // the air, and lands at a signalised terminal on a street.
        let mut ramps = 0;
        for seed in SEEDS {
            let network = derived(seed);
            // A town whose expressway has no interchange keeps it at grade:
            // lifted, it would be an island nobody can reach.
            if !network.roads.iter().any(|road| road.oneway) {
                continue;
            }
            for junction in &network.junctions {
                let main_line = junction.ports.iter().any(|port| {
                    network
                        .road(port.road)
                        .is_some_and(|road| road.is_motorway() && !road.oneway)
                });
                assert!(
                    !main_line,
                    "seed {seed}: the expressway meets a street at grade at node {}",
                    junction.node
                );
            }
            for road in network.roads.iter().filter(|road| road.oneway) {
                ramps += 1;
                let deck = [road.from_node, road.to_node]
                    .into_iter()
                    .filter(|node| network.junction(*node).is_none())
                    .count();
                let terminal = [road.from_node, road.to_node]
                    .into_iter()
                    .filter_map(|node| network.junction(node))
                    .find(|junction| junction.kind == JunctionKind::Signalized);
                assert!(
                    deck == 1 && terminal.is_some(),
                    "seed {seed}: ramp {} does not run from the deck to a signalised terminal",
                    road.id
                );
                // Every lane of a ramp is reached from the network and leads on.
                for id in &road.lane_ids {
                    let lane = network.lane(id).expect("a ramp's lane exists");
                    assert!(
                        !lane.successors.is_empty() && !lane.predecessors.is_empty(),
                        "seed {seed}: ramp lane {id} is a dead end"
                    );
                }
            }
        }
        assert!(ramps >= 4, "no interchange was built in any test city");
    }

    #[test]
    fn every_grade_over_the_limit_is_reported() {
        // Nothing may exceed the limit silently: each over-limit road gets exactly
        // one grade warning, and no warning refers to a road that is within it.
        for seed in SEEDS {
            let network = derived(seed);
            let over: Vec<u32> = network
                .roads
                .iter()
                .filter(|road| road.max_grade > MAX_GRADE)
                .map(|road| road.id)
                .collect();
            for id in &over {
                let prefix = format!("road {id} reaches ");
                assert!(
                    network.warnings.iter().any(|w| w.starts_with(&prefix)),
                    "seed {seed}: road {id} is over the grade limit with no warning"
                );
            }
            let grade_warnings = network
                .warnings
                .iter()
                .filter(|w| w.contains("% grade on a"))
                .count();
            assert_eq!(
                grade_warnings,
                over.len(),
                "seed {seed}: grade warnings and over-limit roads disagree"
            );
        }
    }

    #[test]
    fn elevated_roads_stay_within_the_grade_limit() {
        // Every deck is sized to the ramp its span can carry, so no elevated road
        // exceeds the grade limit.  A span too short even for a causeway is
        // reported by `derive` and fails here.
        for seed in SEEDS {
            let network = derived(seed);
            for road in network.roads.iter().filter(|road| road.layer != 0) {
                assert!(
                    road.max_grade <= MAX_GRADE,
                    "seed {seed}: road {} reaches {:.1}% grade",
                    road.id,
                    road.max_grade * 100.0
                );
            }
        }
    }
}
