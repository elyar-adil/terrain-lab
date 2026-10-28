//! Which junctions of a town are roundabouts, decided once on the whole plan.
//!
//! `docs/city-visual-target.md` §8: about one junction in a hundred in Kuitun,
//! Urumqi, Chongqing and Beijing; CJJ 152 §4.6 keeps them off arterial
//! crossings and ramps. The choice has to be made here, on the whole town, and
//! not where a scene is built: a streamed scene is built from one clipped
//! tile of the plan, and a share of a tile's few junctions rounds to none, and
//! to a different set on every tile. A chosen node carries the role
//! `ROUNDABOUT_ROLE`; the street layer builds what is marked.

use std::collections::HashMap;

use crate::{CityFrameInfo, HdRoad, ModernRoadClass, RoadCrossSection, SdNode, SdRoad};

/// The role a roundabout's node carries.
pub const ROUNDABOUT_ROLE: &str = "roundabout";
/// Share of a town's junctions that are roundabouts.
const ROUNDABOUT_SHARE: f32 = 0.01;
/// No two roundabouts nearer than this.
const ROUNDABOUT_APART_M: f32 = 500.0;
/// Two to three lanes round the island.
pub const CIRCULATING_M: f32 = 10.0;
/// Straight kerb between an arm's end and its entry curve.
pub const ROUNDABOUT_LEAD_M: f32 = 4.0;

/// Centreline to kerb face.
pub fn kerb_half(section: &RoadCrossSection) -> f32 {
    section.half_width() - section.sidewalk_metres
}

/// A roundabout's size: the outer kerb a true circle, the central island the
/// circulating road's width inside it, and the entry curves' radius.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RoundaboutSize {
    /// Centre to the outer kerb.
    pub outer: f32,
    pub island: f32,
    /// The entry and exit curves' radius: no more than the island's (CJJ 152
    /// §4.6.4).
    pub fillet: f32,
}

impl RoundaboutSize {
    /// Sized to its widest arm: centreline radius 25–40 m as the OSM
    /// roundabouts have it.
    pub fn for_widest_kerb(widest: f32) -> Self {
        let outer = (widest + 25.0).clamp(30.0, 42.0);
        RoundaboutSize {
            outer,
            island: outer - CIRCULATING_M,
            fillet: 15.0_f32.min(outer - CIRCULATING_M),
        }
    }

    /// How far from the centre a straight arm's kerb, `kerb` from its
    /// centreline, leaves the entry curve: where the arm stops.
    pub fn trim(&self, kerb: f32) -> f32 {
        let (r, f) = (self.outer + self.fillet, kerb + self.fillet);
        // A few metres more, so a street that bends on its way in still
        // reaches its curve on a straight kerb; it also sets the crossing back
        // the 7 to 11 m from the circle the OSM roundabouts have it.
        (r * r - f * f).max(0.0).sqrt() + ROUNDABOUT_LEAD_M
    }
}

/// The size a junction would have as a roundabout, if it may be one: three
/// to five arms at grade, none a ramp, bridge or expressway, one arterial at
/// most and at least two streets above a local, every arm long enough for its
/// entry curve and a street beyond, no two arms nearer than 50 degrees.
/// `arms` are each street's centreline in city-local metres, out of the node.
pub fn size_if_eligible(arms: &[Arm]) -> Option<RoundaboutSize> {
    if !(3..=5).contains(&arms.len())
        || arms.iter().any(|(r, _)| {
            r.layer != 0 || r.oneway || r.bridge || r.class == ModernRoadClass::Expressway
        })
        || arms
            .iter()
            .filter(|(r, _)| r.class == ModernRoadClass::Arterial)
            .count()
            > 1
        || arms
            .iter()
            .filter(|(r, _)| r.class != ModernRoadClass::Local)
            .count()
            < 2
    {
        return None;
    }
    let widest = arms
        .iter()
        .map(|(r, _)| kerb_half(&r.cross_section()))
        .fold(0.0_f32, f32::max);
    let size = RoundaboutSize::for_widest_kerb(widest);
    let mut angles = Vec::new();
    for (road, line) in arms {
        let (length, toward) = along(line, 20.0);
        if length <= size.trim(kerb_half(&road.cross_section())) + 40.0 {
            return None;
        }
        angles.push((toward[1] - line[0][1]).atan2(toward[0] - line[0][0]));
    }
    angles.sort_by(f32::total_cmp);
    let narrowest = (0..angles.len())
        .map(|k| (angles[(k + 1) % angles.len()] - angles[k]).rem_euclid(std::f32::consts::TAU))
        .fold(std::f32::consts::TAU, f32::min);
    (narrowest > 50.0_f32.to_radians()).then_some(size)
}

/// A line's length, and its point `at` metres along (or its end).
fn along(line: &[[f32; 2]], at: f32) -> (f32, [f32; 2]) {
    let mut total = 0.0;
    let mut point = None;
    for w in line.windows(2) {
        let step = (w[1][0] - w[0][0]).hypot(w[1][1] - w[0][1]);
        if point.is_none() && total + step >= at && step > 0.0 {
            let t = (at - total) / step;
            point = Some([
                w[0][0] + (w[1][0] - w[0][0]) * t,
                w[0][1] + (w[1][1] - w[0][1]) * t,
            ]);
        }
        total += step;
    }
    (total, point.unwrap_or(*line.last().unwrap_or(&[0.0, 0.0])))
}

/// One street out of a node: the road, and its centreline in city-local
/// metres starting at the node.
pub type Arm<'a> = (&'a HdRoad, Vec<[f32; 2]>);

/// Each node's streets, as centrelines out of it in city-local metres.
pub fn arms_at<'a>(
    frame: CityFrameInfo,
    sd: &[SdRoad],
    hd: &'a [HdRoad],
) -> HashMap<u32, Vec<Arm<'a>>> {
    let hd_by_id: HashMap<u32, &HdRoad> = hd.iter().map(|road| (road.id, road)).collect();
    let mut out: HashMap<u32, Vec<Arm>> = HashMap::new();
    for road in sd.iter().filter(|r| r.from != r.to) {
        let Some(h) = hd_by_id.get(&road.id).copied() else {
            continue;
        };
        let line: Vec<[f32; 2]> = h.centreline.iter().map(|p| frame.to_local(*p)).collect();
        if line.len() < 2 {
            continue;
        }
        let mut back = line.clone();
        back.reverse();
        out.entry(road.from).or_default().push((h, line));
        out.entry(road.to).or_default().push((h, back));
    }
    out
}

/// Mark a town's roundabouts: of the junctions that may be one, those nearest
/// the town centre (as the town-centre roundabouts of the photographs are),
/// each at least `ROUNDABOUT_APART_M` from the next, about one in a hundred.
pub fn mark(frame: CityFrameInfo, nodes: &mut [SdNode], sd: &[SdRoad], hd: &[HdRoad]) {
    let arms = arms_at(frame, sd, hd);
    let junctions = arms.values().filter(|list| list.len() >= 3).count();
    let wanted = (junctions as f32 * ROUNDABOUT_SHARE).round() as usize;
    let mut candidates: Vec<(f32, usize, [f32; 2])> = Vec::new();
    for (index, node) in nodes.iter().enumerate() {
        let Some(list) = arms.get(&node.id) else {
            continue;
        };
        if size_if_eligible(list).is_some() {
            let at = frame.to_local(node.point);
            candidates.push((at[0].hypot(at[1]), index, at));
        }
    }
    candidates.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut chosen: Vec<[f32; 2]> = Vec::new();
    for (_, index, at) in candidates {
        if chosen.len() >= wanted {
            break;
        }
        if chosen
            .iter()
            .all(|other| (other[0] - at[0]).hypot(other[1] - at[1]) >= ROUNDABOUT_APART_M)
        {
            chosen.push(at);
            nodes[index].role = ROUNDABOUT_ROLE.into();
        }
    }
}
