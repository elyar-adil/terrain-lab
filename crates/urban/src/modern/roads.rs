use crate::{
    HdLane, LaneMarking, LaneUse, MarkingKind, ModernRoadClass, Movement, TurnArrow, cross_section,
    model::jurisdiction::{JurisdictionId, TrafficRules},
};

/// Physical cross-section of a modern Chinese street, per CJJ 37.  Widths are the
/// ribbon width the renderer draws; the lane offsets inside it place the median,
/// motor lanes, hard shoulders and non-motorized lanes so close zoom matches a
/// real municipal drawing.
///
/// The table itself lives in `model::roads` because the street-scene layer derives
/// junction geometry, kerbs, sidewalks and markings from the same numbers.  Two
/// copies would let the drawn road and the simulated road disagree about where
/// the median ends.
///
/// Expand a hierarchical road class into physically plausible two-way lanes.
/// This is deliberately kept in the city data layer: a renderer must not
/// infer lane counts from a road ribbon and then lose turning arrows at close
/// zoom.  Offsets are measured from the HD centreline in metres; right-hand
/// driving places the direction's carriageway on its right side.
pub(super) fn lanes_for_modern_road(
    road_id: u32,
    class: ModernRoadClass,
    _rules: &TrafficRules,
) -> Vec<HdLane> {
    let section = cross_section(class);
    let motor_lane_width = section.motor_lane_width;
    let count = section.motor_lanes_per_direction;
    let median_half = section.median_metres * 0.5;
    let mut lanes = Vec::with_capacity((count as usize + 2) * 2);
    for direction in [-1_i8, 1_i8] {
        for index in 0..count {
            let use_type = if count >= 2 && index == 0 {
                LaneUse::RightTurn
            } else if count >= 3 && index + 1 == count {
                LaneUse::LeftTurn
            } else {
                LaneUse::Through
            };
            let arrow = match use_type {
                LaneUse::RightTurn => TurnArrow::StraightRight,
                LaneUse::LeftTurn => TurnArrow::StraightLeft,
                _ => TurnArrow::Straight,
            };
            // index_from_curb 0 is the curb-side lane; the innermost lane sits
            // nearest the median/centreline.  The offset sign follows the
            // driving direction so both carriageways fill the real width.
            let distance =
                median_half + section.inner_shoulder + (count - index) as f32 * motor_lane_width
                    - motor_lane_width * 0.5;
            let offset_metres = direction as f32 * distance;
            lanes.push(HdLane {
                id: format!("road/{road_id}/lane/{direction}/{index}"),
                direction,
                index_from_curb: index,
                use_type,
                width_metres: motor_lane_width,
                offset_metres,
                allowed_movements: match use_type {
                    LaneUse::RightTurn => vec![Movement::Through, Movement::Right],
                    LaneUse::LeftTurn => vec![Movement::Left, Movement::Through],
                    _ => vec![Movement::Through],
                },
                markings: vec![
                    LaneMarking {
                        kind: MarkingKind::Arrow,
                        arrow: Some(arrow),
                        offset_metres: 0.0,
                        length_metres: 5.2,
                        dash_gap_metres: None,
                    },
                    LaneMarking {
                        kind: if index + 1 == count {
                            MarkingKind::SolidEdge
                        } else {
                            MarkingKind::DashedDivider
                        },
                        arrow: None,
                        offset_metres: 0.0,
                        length_metres: 18.0,
                        dash_gap_metres: (index + 1 != count).then_some(9.0),
                    },
                ],
            });
        }
        if section.bike_lane_width > 0.0 {
            let distance = median_half
                + section.inner_shoulder
                + count as f32 * motor_lane_width
                + section.bike_lane_width * 0.5;
            lanes.push(HdLane {
                id: format!("road/{road_id}/lane/{direction}/bike"),
                direction,
                index_from_curb: 0,
                use_type: LaneUse::Bike,
                width_metres: section.bike_lane_width,
                offset_metres: direction as f32 * distance,
                allowed_movements: vec![Movement::Through],
                markings: vec![LaneMarking {
                    kind: MarkingKind::SolidEdge,
                    arrow: None,
                    offset_metres: 0.0,
                    length_metres: 0.0,
                    dash_gap_metres: None,
                }],
            });
        }
        if section.shoulder_width > 0.0 {
            // Expressway hard shoulder: no arrows, marked as a full-width
            // solid edge so renderers keep it free of moving traffic.
            let distance = median_half
                + section.inner_shoulder
                + count as f32 * motor_lane_width
                + section.shoulder_width * 0.5;
            lanes.push(HdLane {
                id: format!("road/{road_id}/lane/{direction}/shoulder"),
                direction,
                index_from_curb: 0,
                use_type: LaneUse::Shoulder,
                width_metres: section.shoulder_width,
                offset_metres: direction as f32 * distance,
                allowed_movements: vec![],
                markings: vec![LaneMarking {
                    kind: MarkingKind::SolidEdge,
                    arrow: None,
                    offset_metres: 0.0,
                    length_metres: 0.0,
                    dash_gap_metres: None,
                }],
            });
        }
    }
    lanes
}

/// The mainland-Chinese profile every generator call site shares; keeping the
/// construction in one place makes the whole crate read as one jurisdiction.
pub(super) fn china_rules() -> TrafficRules {
    TrafficRules::for_jurisdiction(JurisdictionId::ChinaMainland)
}
