//! Junction design specification, movement classification and GB arrow stencils.
//!
//! This module is the single source of truth for every metric the street layer
//! draws, ported from the source kernel's `junction-spec.js`,
//! `road-movements.js` and `lane-derive.js`.  Nothing downstream may hard-code a
//! lane width, a stop-line offset or a crosswalk pitch: if two call sites could
//! disagree about a marking's size, the drawing and the traffic model would
//! drift apart, which is exactly the failure the source project documented.
//!
//! References: GB 5768.3 (道路交通标志和标线) and GB 50647 (城市道路交叉口
//! 设计规程).

use crate::math::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct CrosswalkSpec {
    /// Stripe width, 45 cm.
    pub bar_width: f32,
    /// Stripe centre pitch (45 cm stripe + 60 cm clear).
    pub pitch: f32,
    /// Crossing depth, 4 m.
    pub depth: f32,
    /// Kerb-to-crosswalk-band distance.
    pub gap: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct ArrowSpec {
    /// Arrow footprint, 5 m.
    pub footprint: f32,
    /// Tip setback from the junction edge, 13 m.
    pub tip_gap: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct WaitingBoxSpec {
    /// How far the left-turn waiting box reaches into the junction, 11 m.
    pub depth: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct JunctionSpec {
    pub lane_width_min: f32,
    pub lane_width_max: f32,
    pub yellow_line_width: f32,
    pub white_line_width: f32,
    pub edge_line_width: f32,
    pub stop_line_width: f32,
    /// Stop-line centre distance from the junction edge.
    pub stop_line_gap: f32,
    pub crosswalk: CrosswalkSpec,
    /// Guide (solid) lane-line segment length, 30 m.
    pub guide_zone: f32,
    /// Clear gap kept between the guide zone and the stop line.
    pub solid_zone_gap: f32,
    pub arrow: ArrowSpec,
    pub waiting_box: WaitingBoxSpec,
    pub sidewalk_min: f32,
    /// Approach-widening taper length, 58 m.
    pub taper_len: f32,
    /// Width of one widened lane, 3.4 m.
    pub taper_lane_width: f32,
    /// Kerb height.  Every surface elevation in the crate is expressed against
    /// this datum: roadbed `0`, kerb top `+kerb_height`, sidewalk above that.
    pub kerb_height: f32,
    /// Physical median widths by road class.
    pub median_arterial: f32,
    pub median_expressway: f32,
    pub median_local: f32,
    /// Width of the cast kerb's top surface, CJJ 37.  The kerb is a real object
    /// with a top and a face, not a painted line, and the 30 cm top is what a
    /// crossing's dropped kerb is actually sawn out of.
    pub kerb_top_width: f32,
    /// Transverse cross-fall of the carriageway, CJJ 37 allows 1-2%.  A crowned
    /// road is the single strongest cue that a surface drains, and a dead-flat
    /// plane reads as unbuilt no matter how good the asphalt is.
    pub crossfall: f32,
    /// Width of the tactile paving strip (盲道) behind a dropped kerb, GB 50763.
    pub tactile_width: f32,
    /// Length of the yellow-on-black hazard marking on a median nose.
    pub median_hazard_length: f32,
    /// Give-way line (让行线) triangle geometry: pitch, base width, depth.
    pub give_way_pitch: f32,
    pub give_way_width: f32,
    pub give_way_depth: f32,
    /// Yellow box (黄色网格线) pitch and line width, used in large boxes to mark
    /// the no-stopping area.  CJJ 37: 4-6 m grid, 40-50 cm lines.
    pub yellow_grid_pitch: f32,
    pub yellow_grid_width: f32,
    /// Radius multiplier for a roundabout's mountable truck apron.
    pub roundabout_apron: f32,
}

impl Default for JunctionSpec {
    fn default() -> Self {
        Self {
            lane_width_min: 3.25,
            lane_width_max: 3.5,
            yellow_line_width: 0.15,
            white_line_width: 0.15,
            edge_line_width: 0.15,
            stop_line_width: 0.35,
            stop_line_gap: 5.2,
            crosswalk: CrosswalkSpec {
                bar_width: 0.45,
                pitch: 1.05,
                depth: 4.0,
                gap: 2.6,
            },
            guide_zone: 30.0,
            solid_zone_gap: 5.6,
            arrow: ArrowSpec {
                footprint: 5.0,
                tip_gap: 13.0,
            },
            waiting_box: WaitingBoxSpec { depth: 11.0 },
            sidewalk_min: 2.0,
            taper_len: 58.0,
            taper_lane_width: 3.4,
            kerb_height: 0.15,
            median_arterial: 1.6,
            median_expressway: 2.0,
            median_local: 0.5,
            kerb_top_width: 0.30,
            crossfall: 0.02,
            tactile_width: 0.30,
            median_hazard_length: 1.5,
            give_way_pitch: 1.0,
            give_way_width: 0.6,
            give_way_depth: 0.75,
            yellow_grid_pitch: 5.0,
            yellow_grid_width: 0.45,
            roundabout_apron: 1.6,
        }
    }
}

/// Which way a movement turns relative to the approach.  `Right` is resolved
/// from the 2-D cross product under right-hand traffic, never from a hash.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Movement {
    Straight,
    Left,
    Right,
    UTurn,
}

impl Movement {
    pub fn as_str(self) -> &'static str {
        match self {
            Movement::Straight => "straight",
            Movement::Left => "left",
            Movement::Right => "right",
            Movement::UTurn => "uturn",
        }
    }
}

/// Classify the turn from an approach heading into an exit heading.
///
/// `approach` points *into* the junction and `exit` points away from it.  Both
/// are plan unit vectors in the crate's `(+X, +Z)` ground frame.  The magnitude
/// is returned so a caller can size a fillet from the real angle instead of
/// assuming 90°.
///
/// # Which way is "right" here
///
/// The ground frame is `+Y` up with the plan in `(+X, +Z)`.  In it, the
/// **right** of a direction `t` is `(-t.z, t.x)`, and `cross(t, right) = +1`.
/// That is the vector [`crate::math::Vec2::left_normal`] returns — the name
/// refers to the 2-D `(x, y)` convention, and in this crate it is the driver's
/// right, which is also why the lane graph puts a `+1` carriageway on a positive
/// offset.  So a **positive** cross product between approach and exit means the
/// exit is to the driver's right, and that is what [`Movement::Right`] means
/// everywhere in the crate.
///
/// The source kernel reaches the same classification with the *opposite* sign,
/// because its planning camera looks straight down and therefore mirrors the
/// world `X/Z` axes.  Everything downstream of the classification was
/// transcribed from that kernel, so the arrow outlines below are mirrored
/// relative to the tables printed in the source file.  The two tests
/// `a_right_arrow_head_lands_on_the_drivers_right` and
/// `the_stencils_carry_the_gb_proportions` are what keep the conventions from
/// drifting apart again.
pub fn classify_movement(approach: Vec2, exit: Vec2) -> (Movement, f32) {
    let cross = approach.cross(exit);
    let dot = approach.dot(exit);
    // Negated so a positive cross — an exit to the driver's right — reads as a
    // positive turn angle, which is what the arms below are written against.
    let turn = -cross.atan2(dot).to_degrees();
    if (-45.0..45.0).contains(&turn) {
        (Movement::Straight, turn)
    } else if (45.0..150.0).contains(&turn) {
        (Movement::Left, turn)
    } else if (-150.0..-45.0).contains(&turn) {
        (Movement::Right, turn)
    } else {
        (Movement::UTurn, turn)
    }
}

/// Assign allowed movements to each lane of an approach.
///
/// `lane_count` is the number of *inbound* motor lanes; `available` is the set
/// of movement types some target lane actually offers.  `dedicated_left` and
/// `dedicated_right` count lanes the cross-section reserves.  This is the
/// shared single source of truth: the drawn arrows, the connector graph and the
/// signal programme all read the result, so they cannot disagree.
pub fn lane_movement_sets(
    lane_count: usize,
    available: &[Movement],
    dedicated_left: usize,
    dedicated_right: usize,
) -> Vec<Vec<Movement>> {
    if lane_count == 0 {
        return Vec::new();
    }
    let has = |movement: Movement| available.contains(&movement);
    // Preference order for a lane that would otherwise be left with nothing.
    // The source kernel searches `['straight', 'right', 'left']` in that order,
    // and the order matters: an approach whose only reachable exit is a right
    // turn must not advertise a phantom straight, and an empty lane set must
    // resolve to the movement a driver would assume rather than to whichever
    // movement happened to be listed first by the caller.
    let fallback = if has(Movement::Straight) {
        Movement::Straight
    } else if has(Movement::Right) {
        Movement::Right
    } else if has(Movement::Left) {
        Movement::Left
    } else {
        Movement::Straight
    };
    let right_count = dedicated_right.min(lane_count);
    if lane_count == 1 {
        if right_count > 0 && has(Movement::Right) {
            return vec![vec![Movement::Right]];
        }
        return vec![available.to_vec()];
    }

    let mut lanes: Vec<Vec<Movement>> = vec![Vec::new(); lane_count];
    if has(Movement::Right) {
        for lane in (lane_count - right_count)..lane_count {
            lanes[lane].push(Movement::Right);
        }
        if right_count == 0 {
            // No dedicated bay: the curb lane doubles as the right-turn lane,
            // which is what an undivided Chinese street actually does.
            lanes[lane_count - 1].push(Movement::Right);
        }
    }
    let left_count = if has(Movement::Left) {
        (lane_count - right_count).min(dedicated_left.max(1))
    } else {
        0
    };
    for lane in 0..left_count {
        lanes[lane].push(Movement::Left);
    }
    if has(Movement::Straight) {
        let start = if dedicated_left > 0 { left_count } else { 0 };
        for lane in start..(lane_count - right_count) {
            lanes[lane].push(Movement::Straight);
        }
        if right_count == 0 {
            for lane in start..lane_count {
                if !lanes[lane].contains(&Movement::Straight) {
                    lanes[lane].push(Movement::Straight);
                }
            }
        }
    }
    for lane in lanes.iter_mut() {
        lane.retain(|movement| has(*movement));
        lane.sort();
        if lane.is_empty() {
            lane.push(fallback);
        }
    }
    lanes
}

/// The 24 facade tiles, ported verbatim from the source renderer's
/// `FACADE_SPECS`.  Keeping the exact table matters: a city's realism comes
/// from *diversity* of wall rhythm, and re-deriving these ad hoc produces the
/// uniform corrugation this port set out to remove.
#[derive(Debug, Clone, Copy)]
pub struct FacadeTile {
    pub base: [u8; 3],
    pub window: [u8; 3],
    pub cols: u8,
    pub sill: f32,
    pub brick: bool,
    pub band: bool,
    pub glass: bool,
}

pub const FACADE_TILES: [FacadeTile; 24] = [
    FacadeTile { base: [198, 188, 168], window: [96, 104, 112], cols: 2, sill: 0.34, brick: false, band: false, glass: false },
    FacadeTile { base: [176, 179, 176], window: [88, 97, 106], cols: 2, sill: 0.32, brick: false, band: false, glass: false },
    FacadeTile { base: [216, 211, 197], window: [112, 142, 154], cols: 3, sill: 0.28, brick: false, band: false, glass: false },
    FacadeTile { base: [156, 100, 80], window: [64, 56, 54], cols: 2, sill: 0.36, brick: true, band: false, glass: false },
    FacadeTile { base: [208, 196, 168], window: [98, 118, 128], cols: 3, sill: 0.30, brick: false, band: false, glass: false },
    FacadeTile { base: [150, 151, 149], window: [80, 92, 100], cols: 4, sill: 0.24, brick: false, band: true, glass: false },
    FacadeTile { base: [190, 148, 120], window: [84, 90, 96], cols: 2, sill: 0.32, brick: false, band: false, glass: false },
    FacadeTile { base: [132, 136, 142], window: [98, 130, 142], cols: 3, sill: 0.30, brick: false, band: false, glass: false },
    FacadeTile { base: [76, 90, 100], window: [134, 170, 186], cols: 4, sill: 0.30, brick: false, band: false, glass: true },
    FacadeTile { base: [66, 80, 88], window: [120, 160, 152], cols: 5, sill: 0.30, brick: false, band: false, glass: true },
    FacadeTile { base: [90, 94, 106], window: [150, 152, 160], cols: 4, sill: 0.30, brick: false, band: false, glass: true },
    FacadeTile { base: [112, 98, 90], window: [128, 150, 158], cols: 3, sill: 0.30, brick: false, band: false, glass: true },
    FacadeTile { base: [58, 62, 70], window: [110, 150, 168], cols: 6, sill: 0.30, brick: false, band: false, glass: true },
    FacadeTile { base: [168, 170, 172], window: [96, 118, 130], cols: 5, sill: 0.30, brick: false, band: false, glass: true },
    FacadeTile { base: [122, 96, 72], window: [150, 138, 116], cols: 4, sill: 0.30, brick: false, band: false, glass: true },
    FacadeTile { base: [70, 92, 96], window: [128, 164, 170], cols: 5, sill: 0.30, brick: false, band: false, glass: true },
    FacadeTile { base: [214, 212, 206], window: [104, 116, 126], cols: 4, sill: 0.26, brick: false, band: false, glass: false },
    FacadeTile { base: [142, 138, 130], window: [88, 96, 104], cols: 4, sill: 0.26, brick: false, band: true, glass: false },
    FacadeTile { base: [186, 172, 148], window: [96, 110, 118], cols: 3, sill: 0.30, brick: false, band: false, glass: false },
    FacadeTile { base: [84, 86, 90], window: [118, 128, 136], cols: 5, sill: 0.22, brick: false, band: false, glass: false },
    FacadeTile { base: [226, 222, 212], window: [100, 110, 118], cols: 2, sill: 0.40, brick: false, band: false, glass: false },
    FacadeTile { base: [164, 132, 96], window: [76, 64, 54], cols: 2, sill: 0.36, brick: false, band: false, glass: false },
    FacadeTile { base: [142, 74, 60], window: [70, 58, 52], cols: 2, sill: 0.36, brick: true, band: false, glass: false },
    FacadeTile { base: [172, 178, 164], window: [92, 104, 110], cols: 3, sill: 0.34, brick: false, band: false, glass: false },
];

/// Tile index range per land use, matching the source generator's mapping:
/// `tower → [8,20)`, `mixed → [4,8)`, `waterfront → [2,6)`, `residential → [0,4)`.
pub fn facade_tile_range(style: u8) -> std::ops::Range<usize> {
    match style {
        0 => 8..20,  // tower
        1 => 4..8,   // mixed-use
        2 => 2..6,   // waterfront
        _ => 0..4,   // residential
    }
}

/// GB 5768.3 arrow outlines in **millimetres**, local frame `[lateral, forward]`.
/// `y = 0` is the tail, the maximum `y` is the front tip.
///
/// # The lateral axis points to the driver's RIGHT
///
/// The draw code lays a stencil out with
/// `point = origin + tangent_left_normal() * lateral + tangent * forward`, and
/// in this crate [`crate::math::Vec2::left_normal`] — `(-t.z, t.x)` — is the
/// driver's **right**.  So positive `lateral` is the driver's right, and:
///
/// * a **right** arrow's head sits at *positive* `lateral`;
/// * a **left** arrow's head sits at *negative* `lateral`.
///
/// [`ARROW_TURN`] below is therefore the *left*-turn stencil and its mirror is
/// the right-turn one, which is the reverse of the source kernel's variable
/// names: the kernel's planning camera mirrors the ground frame, so its
/// `ARROW_LEFT_RAW` is this file's `mirror(ARROW_TURN)`.  The tables themselves
/// are byte-identical to the kernel's — `the_stencil_table_is_a_faithful_copy_of
/// _the_source_kernel` asserts that, and so does
/// `a_right_arrow_head_lands_on_the_drivers_right` on the built triangles.
///
/// # Why millimetres and not centimetres
///
/// The source kernel's `ARROW_LENGTH = 3050` is divided by *metres* to get a
/// scale, and the largest `y` in the tables is 3000, so one unit is 1 mm: a
/// 3.0 m arrow, a 450 mm head, a 150 mm shaft.  Reading the table as
/// centimetres gives a thirty-metre arrow, which is the single most visible way
/// to get this wrong and is why the unit is named here.
pub const ARROW_LENGTH_MM: f32 = 3050.0;
/// Millimetres to metres, the scale the draw code applies to a stencil.
pub const MM: f32 = 0.001;
/// Half-width of a straight arrow's head, 225 mm — 450 mm across, which is the
/// GB 5768.3 proportion at the 3.05 m arrow length.
pub const ARROW_HEAD_HALF_WIDTH_MM: f32 = 225.0;
/// Half-width of a straight arrow's shaft, 75 mm.
pub const ARROW_SHAFT_HALF_WIDTH_MM: f32 = 75.0;
/// Half-width of a turning arrow's tail, 375 mm.
pub const ARROW_TURN_TAIL_HALF_WIDTH_MM: f32 = 375.0;

type Outline = &'static [(f32, f32)];

const ARROW_STRAIGHT: Outline = &[
    (-75.0, 0.0),
    (75.0, 0.0),
    (75.0, 1800.0),
    (225.0, 1800.0),
    (0.0, 3000.0),
    (-225.0, 1800.0),
    (-75.0, 1800.0),
];

/// The turning-arrow stencil: shaft on the `+lateral` side, head on `-lateral`,
/// which is the driver's **left**.
const ARROW_TURN: Outline = &[
    (225.0, 0.0),
    (375.0, 0.0),
    (375.0, 1950.0),
    (-175.0, 2550.0),
    (-175.0, 3050.0),
    (-375.0, 2250.0),
    (-175.0, 1350.0),
    (-175.0, 1800.0),
    (225.0, 1350.0),
];

/// The through-and-turning stencil that pairs with [`ARROW_TURN`].
const ARROW_STRAIGHT_TURN: Outline = &[
    (150.0, 0.0),
    (300.0, 0.0),
    (300.0, 1800.0),
    (450.0, 1800.0),
    (225.0, 3000.0),
    (0.0, 1800.0),
    (150.0, 1800.0),
    (150.0, 800.0),
    (-250.0, 1250.0),
    (-250.0, 1750.0),
    (-450.0, 950.0),
    (-250.0, 200.0),
    (-250.0, 650.0),
    (150.0, 200.0),
];

const ARROW_LEFT_RIGHT_STRAIGHT: Outline = &[
    (-75.0, 0.0),
    (75.0, 0.0),
    (75.0, 200.0),
    (475.0, 650.0),
    (475.0, 200.0),
    (675.0, 950.0),
    (475.0, 1750.0),
    (475.0, 1260.0),
    (75.0, 650.0),
    (75.0, 1800.0),
    (225.0, 1800.0),
    (0.0, 3000.0),
    (-225.0, 1800.0),
    (-75.0, 1800.0),
    (-75.0, 800.0),
    (-475.0, 1250.0),
    (-475.0, 1750.0),
    (-675.0, 950.0),
    (-475.0, 200.0),
    (-475.0, 650.0),
    (-75.0, 200.0),
];

fn mirror(outline: Outline) -> Vec<(f32, f32)> {
    outline.iter().map(|(x, y)| (-x, *y)).collect()
}

/// Signed area of an `[lateral, forward]` outline, in mm².
///
/// The stencils above are not all wound the same way: mirroring a polygon
/// reverses it.  A renderer that picks one winding "by convention" therefore
/// lights half the arrows from below, which is why the draw code asks this
/// function instead of hard-coding a triangle order.  Positive is
/// counter-clockwise in the stencil's own 2-D frame.
pub fn outline_signed_area(outline: &[(f32, f32)]) -> f32 {
    let mut total = 0.0;
    for index in 0..outline.len() {
        let a = outline[index];
        let b = outline[(index + 1) % outline.len()];
        total += a.0 * b.1 - b.0 * a.1;
    }
    total * 0.5
}

/// The stencils for a lane's movement set.  A combined movement uses its own
/// dedicated outline; two combined movements are laid side by side, exactly as
/// the source kernel does, so a shared through-and-right lane still reads as
/// one marking instead of two overlapping arrows.
///
/// # Side by side
///
/// `HW = {left: 375, right: 375, straight: 225}` and a 50 cm gap between
/// stencils for a pair, 60 cm for a triple — the source kernel's numbers, and
/// the reason a shared through-and-right arrow does not simply look like a
/// through arrow with a smudge beside it.  Two stencils at 3.05 m long, 37.5 cm
/// half-width and a 50 cm gap span 2.0 m, which is what lets the pair fit one
/// 3.25-3.5 m lane.
///
/// `Movement` orders as `Straight < Left < Right < UTurn`, so the deduped slice
/// is canonical and the match arms below are exhaustive.
pub fn arrow_polygons(movements: &[Movement]) -> Vec<Vec<(f32, f32)>> {
    let mut unique: Vec<Movement> = movements.to_vec();
    unique.sort();
    unique.dedup();
    if unique.is_empty() {
        unique.push(Movement::Straight);
    }
    let mirrored_turn = mirror(ARROW_TURN);
    let mirrored_straight_turn = mirror(ARROW_STRAIGHT_TURN);
    // The stencil for a single movement.  A turn bends to the side the connector
    // actually bends to: the raw table hooks to `-lateral`, which is the driver's
    // left, so the *left* turn uses it and the mirror is the right turn.
    let outline_for = |movement: Movement| -> Vec<(f32, f32)> {
        match movement {
            Movement::Right => mirrored_turn.clone(),
            Movement::Left => ARROW_TURN.to_vec(),
            _ => ARROW_STRAIGHT.to_vec(),
        }
    };
    match unique.as_slice() {
        // Through-and-turn, mirrored to whichever side the turn is on.
        [Movement::Straight, Movement::Left] => vec![ARROW_STRAIGHT_TURN.to_vec()],
        [Movement::Straight, Movement::Right] => vec![mirrored_straight_turn],
        [Movement::Straight, Movement::Left, Movement::Right] => {
            vec![ARROW_LEFT_RIGHT_STRAIGHT.to_vec()]
        }
        [single] => vec![outline_for(*single)],
        _ => {
            let half_width = |movement: &Movement| match movement {
                Movement::Left | Movement::Right => 375.0,
                _ => 225.0,
            };
            let spacing = if unique.len() == 2 {
                half_width(&unique[0]) + half_width(&unique[1]) + 50.0
            } else {
                600.0
            };
            unique
                .iter()
                .enumerate()
                .map(|(index, movement)| {
                    let offset = (index as f32 - (unique.len() as f32 - 1.0) / 2.0) * spacing;
                    outline_for(*movement)
                        .into_iter()
                        .map(|(x, y)| (x + offset, y))
                        .collect()
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement_classification_resolves_the_side_from_the_cross_product() {
        let north = Vec2::new(0.0, 1.0);
        let (movement, degrees) = classify_movement(north, Vec2::new(0.0, 1.0));
        assert_eq!(movement, Movement::Straight);
        assert!(degrees.abs() < 0.01);
        // `Vec2::left_normal` is `(-z, x)`, and for a `+Z` heading that is `-X`.
        // So a driver heading north turns **west** — negative `X` — to go right.
        let (movement, degrees) = classify_movement(north, Vec2::new(-1.0, 0.0));
        assert_eq!(movement, Movement::Right, "north then west is a right turn");
        assert!((degrees.abs() - 90.0).abs() < 0.01, "turn was {degrees} degrees");
        // The returned angle is signed: negative to the right, positive to the
        // left, so a fillet radius can be sized from it without a second test.
        assert!(degrees < 0.0);
        let (movement, _) = classify_movement(north, Vec2::new(1.0, 0.0));
        assert_eq!(movement, Movement::Left, "north then east is a left turn");
        let (movement, _) = classify_movement(north, Vec2::new(0.0, -1.0));
        assert_eq!(movement, Movement::UTurn);
        // And the sign convention the whole crate is built on.
        assert!(north.cross(Vec2::new(-1.0, 0.0)) > 0.0, "right is a positive cross");
    }

    #[test]
    fn a_three_lane_approach_reserves_both_turns() {
        let available = [Movement::Straight, Movement::Left, Movement::Right];
        let lanes = lane_movement_sets(3, &available, 1, 1);
        assert_eq!(lanes.len(), 3);
        assert_eq!(lanes[0], vec![Movement::Left]);
        assert_eq!(lanes[1], vec![Movement::Straight]);
        assert_eq!(lanes[2], vec![Movement::Right]);
    }

    #[test]
    fn a_dedicated_left_lane_keeps_the_left_turn_off_the_through_lane() {
        // Chinese practice on an undivided two-lane approach: the median-side
        // lane turns left only, the curb lane carries through and right.
        let available = [Movement::Straight, Movement::Left, Movement::Right];
        let lanes = lane_movement_sets(2, &available, 1, 0);
        assert_eq!(lanes[0], vec![Movement::Left]);
        assert_eq!(lanes[1], vec![Movement::Straight, Movement::Right]);
    }

    #[test]
    fn a_two_lane_approach_with_lefts_shares_the_through_movement() {
        let available = [Movement::Straight, Movement::Right];
        let lanes = lane_movement_sets(2, &available, 0, 0);
        assert!(lanes.iter().all(|lane| lane.contains(&Movement::Straight)));
        assert!(lanes[1].contains(&Movement::Right));
    }

    #[test]
    fn lane_sets_never_offer_a_movement_the_targets_do_not_have() {
        let available = [Movement::Straight, Movement::Right];
        let lanes = lane_movement_sets(3, &available, 1, 1);
        for lane in &lanes {
            for movement in lane {
                assert!(available.contains(movement), "phantom {movement:?}");
            }
        }
    }

    #[test]
    fn a_single_lane_approach_falls_back_to_a_single_movement() {
        let lanes = lane_movement_sets(1, &[Movement::Straight, Movement::Left], 0, 0);
        assert_eq!(lanes, vec![vec![Movement::Straight, Movement::Left]]);
        assert!(lane_movement_sets(0, &[Movement::Straight], 0, 0).is_empty());
    }

    #[test]
    fn combined_movements_use_their_dedicated_stencil() {
        let left_straight = arrow_polygons(&[Movement::Left, Movement::Straight]);
        assert_eq!(left_straight.len(), 1);
        assert_eq!(left_straight[0].len(), ARROW_STRAIGHT_TURN.len());
        let triple = arrow_polygons(&[Movement::Left, Movement::Right, Movement::Straight]);
        assert_eq!(triple, vec![ARROW_LEFT_RIGHT_STRAIGHT.to_vec()]);
    }

    /// The stencils are transcribed from the source kernel's `lane-derive.js`,
    /// and the transcription has been wrong in both directions before: the
    /// turning stencil was labelled for the wrong hand, and the combined
    /// through-and-turn stencils were swapped.  This table is the reference the
    /// port is checked against, written out with the *kernel's* handedness and
    /// converted here rather than by eye.
    #[test]
    fn the_stencil_table_is_a_faithful_copy_of_the_source_kernel() {
        // lane-derive.js, verbatim, before its `.map(([x, y]) => [-x, y])`.
        const JS_LEFT: [(f32, f32); 9] = [
            (225.0, 0.0),
            (375.0, 0.0),
            (375.0, 1950.0),
            (-175.0, 2550.0),
            (-175.0, 3050.0),
            (-375.0, 2250.0),
            (-175.0, 1350.0),
            (-175.0, 1800.0),
            (225.0, 1350.0),
        ];
        const JS_STRAIGHT_LEFT: [(f32, f32); 14] = [
            (150.0, 0.0),
            (300.0, 0.0),
            (300.0, 1800.0),
            (450.0, 1800.0),
            (225.0, 3000.0),
            (0.0, 1800.0),
            (150.0, 1800.0),
            (150.0, 800.0),
            (-250.0, 1250.0),
            (-250.0, 1750.0),
            (-450.0, 950.0),
            (-250.0, 200.0),
            (-250.0, 650.0),
            (150.0, 200.0),
        ];
        const JS_STRAIGHT: [(f32, f32); 7] = [
            (-75.0, 0.0),
            (75.0, 0.0),
            (75.0, 1800.0),
            (225.0, 1800.0),
            (0.0, 3000.0),
            (-225.0, 1800.0),
            (-75.0, 1800.0),
        ];
        const JS_STRAIGHT_LEFT_RIGHT: [(f32, f32); 21] = [
            (-75.0, 0.0),
            (75.0, 0.0),
            (75.0, 200.0),
            (475.0, 650.0),
            (475.0, 200.0),
            (675.0, 950.0),
            (475.0, 1750.0),
            (475.0, 1260.0),
            (75.0, 650.0),
            (75.0, 1800.0),
            (225.0, 1800.0),
            (0.0, 3000.0),
            (-225.0, 1800.0),
            (-75.0, 1800.0),
            (-75.0, 800.0),
            (-475.0, 1250.0),
            (-475.0, 1750.0),
            (-675.0, 950.0),
            (-475.0, 200.0),
            (-475.0, 650.0),
            (-75.0, 200.0),
        ];
        // The kernel's `left` is this crate's `right`, because its planning
        // camera mirrors the ground frame.  See `classify_movement`.
        assert_eq!(ARROW_TURN, &JS_LEFT[..]);
        assert_eq!(ARROW_STRAIGHT_TURN, &JS_STRAIGHT_LEFT[..]);
        assert_eq!(ARROW_STRAIGHT, &JS_STRAIGHT[..]);
        assert_eq!(ARROW_LEFT_RIGHT_STRAIGHT, &JS_STRAIGHT_LEFT_RIGHT[..]);

        // And the derived stencils are the kernel's, with the same conversion:
        // the kernel's mirrored `left+straight` is this crate's `left+straight`,
        // because the kernel's own `lateral` axis is this crate's *left* while
        // the crate's is its *right*.
        let js_left_straight: Vec<(f32, f32)> = JS_STRAIGHT_LEFT
            .iter()
            .map(|(x, y)| (-x, *y))
            .collect();
        assert_eq!(arrow_polygons(&[Movement::Straight, Movement::Left])[0], JS_STRAIGHT_LEFT.to_vec());
        assert_eq!(arrow_polygons(&[Movement::Straight, Movement::Right])[0], js_left_straight);
        let js_left: Vec<(f32, f32)> = JS_LEFT.iter().map(|(x, y)| (-x, *y)).collect();
        assert_eq!(arrow_polygons(&[Movement::Right])[0], js_left);
        assert_eq!(arrow_polygons(&[Movement::Left])[0], JS_LEFT.to_vec());
    }

    /// The load-bearing geometric fact, asserted on the outline the draw code
    /// actually receives.  Positive `lateral` is the driver's right, because the
    /// draw code's lateral axis is `Vec2::left_normal` — which in this crate's
    /// `(+X, +Z, +Y)` ground frame is the right-hand normal.
    #[test]
    fn a_right_arrow_head_lands_on_the_drivers_right() {
        let tip_side = |movement: Movement| {
            let outline = &arrow_polygons(&[movement])[0];
            // The tip is the outline's most-forward vertex.
            let tip = outline
                .iter()
                .max_by(|a, b| a.1.total_cmp(&b.1))
                .copied()
                .unwrap();
            tip.0
        };
        assert!(
            tip_side(Movement::Right) > 0.0,
            "a right arrow hooks to the driver's left"
        );
        assert!(
            tip_side(Movement::Left) < 0.0,
            "a left arrow hooks to the driver's right"
        );
        assert_eq!(tip_side(Movement::Straight), 0.0);
        // The combined stencils bend the same way.  Their *turning lobe* is the
        // outboard pair of barbs, and the one with the least forward distance is
        // the lobe's own tip — the through head sits further forward, so this
        // cannot pick the wrong end of the outline.
        let lobe_tip = |movements: &[Movement]| {
            let outline = &arrow_polygons(movements)[0];
            *outline
                .iter()
                .filter(|p| p.0.abs() > 380.0)
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .expect("a combined stencil has a turning lobe")
        };
        assert!(
            lobe_tip(&[Movement::Straight, Movement::Right]).0 > 0.0,
            "a through-and-right stencil turns to the driver's left"
        );
        assert!(lobe_tip(&[Movement::Straight, Movement::Left]).0 < 0.0);
    }

    #[test]
    fn the_stencils_carry_the_gb_proportions() {
        let straight = &arrow_polygons(&[Movement::Straight])[0];
        let length = straight.iter().map(|p| p.1).fold(f32::MIN, f32::max);
        let span = straight
            .iter()
            .map(|p| p.0)
            .fold(f32::MIN, f32::max)
            - straight.iter().map(|p| p.0).fold(f32::MAX, f32::min);
        assert!((length - 3000.0).abs() < 1.0, "straight arrow is {length} mm");
        assert!(
            (span - 2.0 * ARROW_HEAD_HALF_WIDTH_MM).abs() < 1.0,
            "head is {span} mm across, expected {}",
            2.0 * ARROW_HEAD_HALF_WIDTH_MM
        );
        // 3.05 m of arrow carrying a 450 mm head, which is the proportion the
        // draw code's `length / ARROW_LENGTH_MM` scale exists to preserve.  A
        // millimetre table read as centimetres gives a thirty-metre arrow, so
        // the conversion is asserted rather than assumed.
        assert!(((length + ARROW_LENGTH_MM - 3000.0) * MM - 3.05).abs() < 0.01);
        assert!((span * MM - 0.45).abs() < 0.005, "head is {} m across", span * MM);
        // The turning arrow is the longest thing the design calls for.
        let turn = &arrow_polygons(&[Movement::Right])[0];
        let turn_length = turn.iter().map(|p| p.1).fold(f32::MIN, f32::max);
        assert!((turn_length - ARROW_LENGTH_MM).abs() < 1.0);
    }

    /// Two stencils side by side must clear each other by exactly the kernel's
    /// 50 cm.  `HW = {left: 375, right: 375, straight: 225}` with a `+50` gap is
    /// the source kernel's rule, and it is the reason a shared through-and-right
    /// lane reads as one marking rather than as two overlapping arrows.
    #[test]
    fn two_stencils_side_by_side_keep_the_kernel_spacing() {
        // `left + right` is the only two-movement set that reaches the
        // side-by-side branch with two *different* half widths, so it is the
        // one that exercises the rule; a driver never gets that lane, which is
        // exactly why the kernel only needs the rule to be arithmetically
        // right rather than tuned.
        let pair = arrow_polygons(&[Movement::Left, Movement::Right]);
        assert_eq!(pair.len(), 2);
        let span = |polygon: &Vec<(f32, f32)>| {
            let lo = polygon.iter().map(|p| p.0).fold(f32::MAX, f32::min);
            let hi = polygon.iter().map(|p| p.0).fold(f32::MIN, f32::max);
            (lo, hi)
        };
        let (left_lo, left_hi) = span(&pair[0]);
        let (right_lo, right_hi) = span(&pair[1]);
        // 375 + 375 + 50: the two stencils' centres are 800 mm apart, which puts
        // their nearest edges exactly 50 mm apart.
        let centre_gap = (right_lo + right_hi) * 0.5 - (left_lo + left_hi) * 0.5;
        assert!(
            (centre_gap - 800.0).abs() < 1.0,
            "stencil centres are {centre_gap} mm apart, expected 800"
        );
        assert!((right_lo - left_hi - 50.0).abs() < 1.0);
        // A straight stencil beside a turning one uses 225 + 375 + 50.
        let mixed = arrow_polygons(&[Movement::Straight, Movement::UTurn]);
        let (_a_lo, a_hi) = span(&mixed[0]);
        let (b_lo, _b_hi) = span(&mixed[1]);
        assert!((b_lo - a_hi - 50.0).abs() < 1.0, "pair gap is {} mm", b_lo - a_hi);
    }

    /// Mirroring a stencil reverses its winding, and the draw code does not care
    /// which way any individual table is wound because it ear-clips rather than
    /// fans.  The sign is still asserted here because it is the cheapest way to
    /// see a table accidentally transcribed inside-out.
    #[test]
    fn mirroring_reverses_the_winding() {
        assert!(outline_signed_area(ARROW_TURN) > 0.0);
        assert!(outline_signed_area(ARROW_STRAIGHT) > 0.0);
        assert!(outline_signed_area(ARROW_STRAIGHT_TURN) > 0.0);
        assert!(outline_signed_area(ARROW_LEFT_RIGHT_STRAIGHT) > 0.0);
        for (left, right) in [
            (Movement::Left, Movement::Right),
            (Movement::Straight, Movement::Right),
        ] {
            let a = outline_signed_area(&arrow_polygons(&[left])[0]);
            let b = outline_signed_area(&arrow_polygons(&[right])[0]);
            assert!(
                a * b < 0.0,
                "{left:?} and {right:?} are not mirror images: {a} vs {b}"
            );
        }
    }

    /// The GB arrow outlines are **not convex**: a shaft joins a head at a reflex
    /// vertex, and a combined straight-and-turn stencil carries a second lobe at
    /// another.  A triangle fan over such an outline emits triangles outside the
    /// polygon, which on retroreflective paint is the "white blob" an arrow used
    /// to render as, so convexity is asserted rather than assumed.
    #[test]
    fn the_stencils_are_concave_so_a_fan_would_be_wrong() {
        for movements in [
            vec![Movement::Straight],
            vec![Movement::Left],
            vec![Movement::Right],
            vec![Movement::Left, Movement::Straight],
            vec![Movement::Right, Movement::Straight],
            vec![Movement::Left, Movement::Right, Movement::Straight],
        ] {
            for outline in arrow_polygons(&movements) {
                let count = outline.len();
                let mut reflex = 0;
                for index in 0..count {
                    let a = outline[index];
                    let b = outline[(index + 1) % count];
                    let c = outline[(index + 2) % count];
                    let turn = (b.0 - a.0) * (c.1 - b.1) - (b.1 - a.1) * (c.0 - b.0);
                    if turn < 0.0 {
                        reflex += 1;
                    }
                }
                assert!(
                    reflex > 0,
                    "{movements:?} is convex, so this test no longer describes it"
                );
            }
        }
    }

    /// Ear-clipping a stencil must cover it exactly once.  This is the assertion
    /// the fan was failing: the fan's triangles sum to *more* than the outline's
    /// area, which is geometrically impossible and is what puts paint on the
    /// road where there is no arrow.
    #[test]
    fn an_ear_clipped_stencil_covers_its_outline_exactly_once() {
        let area = |outline: &[(f32, f32)]| outline_signed_area(outline).abs();
        for movements in [
            vec![Movement::Straight],
            vec![Movement::Left],
            vec![Movement::Right],
            vec![Movement::Left, Movement::Straight],
            vec![Movement::Right, Movement::Straight],
            vec![Movement::Left, Movement::Right, Movement::Straight],
        ] {
            for outline in arrow_polygons(&movements) {
                let ring: Vec<crate::math::Vec2> = outline
                    .iter()
                    .map(|(x, y)| crate::math::Vec2::new(*x, *y))
                    .collect();
                let triangles = crate::math::triangulate(&ring);
                let covered: f32 = triangles
                    .iter()
                    .map(|[a, b, c]| {
                        let (a, b, c) = (ring[*a], ring[*b], ring[*c]);
                        (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y)
                    })
                    .sum::<f32>()
                    .abs()
                    * 0.5;
                assert!(
                    (covered - area(&outline)).abs() < 1.0,
                    "{movements:?}: ear clipping covered {covered} of {}",
                    area(&outline)
                );
                // And every triangle's winding agrees, so the whole stencil faces
                // one way instead of half of it pointing at the sky.
                for [a, b, c] in &triangles {
                    let (a, b, c) = (ring[*a], ring[*b], ring[*c]);
                    let turn = (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y);
                    assert!(turn > 0.0, "{movements:?} has a reversed triangle");
                }
            }
        }
    }

    #[test]
    fn a_lone_arrow_is_centred_and_the_right_stencil_mirrors_the_left() {
        let straight = arrow_polygons(&[Movement::Straight]);
        assert_eq!(straight, vec![ARROW_STRAIGHT.to_vec()]);
        let left = arrow_polygons(&[Movement::Left])[0].clone();
        let right = arrow_polygons(&[Movement::Right])[0].clone();
        for (a, b) in left.iter().zip(right.iter()) {
            assert!((a.0 + b.0).abs() < 1.0e-3);
            assert!((a.1 - b.1).abs() < 1.0e-3);
        }
    }

    /// Every stencil the crate can emit must be a simple, positively-sized
    /// polygon.  A self-intersecting outline triangulates into overlapping
    /// garbage, and since the paint is emissive-bright that garbage reads as a
    /// white blob rather than as a broken arrow — which is exactly the failure
    /// that sent the drive arrows back for rework.
    #[test]
    fn every_stencil_is_a_simple_polygon() {
        for movement in [
            Movement::Straight,
            Movement::Left,
            Movement::Right,
            Movement::UTurn,
        ] {
            for polygon in arrow_polygons(&[movement]) {
                assert!(polygon.len() >= 3, "{movement:?} has too few vertices");
                assert!(
                    outline_signed_area(&polygon).abs() > 1.0e3,
                    "{movement:?} encloses no area"
                );
                for a in 0..polygon.len() {
                    for b in a + 1..polygon.len() {
                        if a == b || (a + 1) % polygon.len() == b || (b + 1) % polygon.len() == a
                        {
                            continue;
                        }
                        assert!(
                            !segments_cross(
                                polygon[a],
                                polygon[(a + 1) % polygon.len()],
                                polygon[b],
                                polygon[(b + 1) % polygon.len()]
                            ),
                            "{movement:?} is self-intersecting at edges {a} and {b}"
                        );
                    }
                }
            }
        }
        for pair in [
            vec![Movement::Straight, Movement::Left],
            vec![Movement::Straight, Movement::Right],
            vec![Movement::Left, Movement::Right],
            vec![Movement::Straight, Movement::Left, Movement::Right],
        ] {
            for polygon in arrow_polygons(&pair) {
                assert!(outline_signed_area(&polygon).abs() > 1.0e3);
            }
        }
    }

    fn segments_cross(a: (f32, f32), b: (f32, f32), c: (f32, f32), d: (f32, f32)) -> bool {
        let side = |p: (f32, f32), q: (f32, f32), r: (f32, f32)| {
            (q.0 - p.0) * (r.1 - p.1) - (q.1 - p.1) * (r.0 - p.0)
        };
        let d1 = side(a, b, c);
        let d2 = side(a, b, d);
        let d3 = side(c, d, a);
        let d4 = side(c, d, b);
        d1 * d2 < -1.0e-3 && d3 * d4 < -1.0e-3
    }

    #[test]
    fn every_facade_tile_has_a_readable_window_rhythm() {
        let luma = |c: [u8; 3]| 0.2126 * c[0] as f32 + 0.7152 * c[1] as f32 + 0.0722 * c[2] as f32;
        for (index, tile) in FACADE_TILES.iter().enumerate() {
            assert!((2..=6).contains(&tile.cols), "tile {index} cols");
            assert!((0.2..=0.45).contains(&tile.sill), "tile {index} sill");
            for channel in 0..3 {
                assert!(tile.base[channel] > 40, "tile {index} base too dark");
            }
            // What actually makes a facade read at city scale is the dark
            // inter-storey band between floors, not the window's own contrast:
            // the source generator paints it at `base * 0.55`.  Without it a
            // tile degenerates into a colour block, which is exactly the
            // failure this port exists to remove.  The bound is relative so a
            // dark granite tile is held to the same rhythm as a white slab.
            let base_luma = luma(tile.base);
            let band_luma = base_luma * 0.55;
            assert!(
                base_luma - band_luma > 0.30 * base_luma,
                "tile {index} has no readable storey line (base {base_luma:.0})"
            );
        }
        // Eight of the twenty-four are curtain wall, which is what gives a
        // skyline its glass-tower mix.
        assert_eq!(FACADE_TILES.iter().filter(|tile| tile.glass).count(), 8);
    }

    #[test]
    fn facade_ranges_tile_the_whole_palette_without_overlap_gaps() {
        for (style, range) in (0..4).map(|style| (style, facade_tile_range(style))) {
            assert!(range.start < range.end, "style {style}");
        }
    }
}
