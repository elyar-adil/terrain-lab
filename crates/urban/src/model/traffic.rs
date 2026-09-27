//! Ported from the source city kernel: deterministic signal synthesis for
//! junctions.  Lane semantics are built before any mesh is drawn, following
//! mainland-Chinese practice (GB 14886): a four-arm arterial junction runs a
//! four-phase programme — two through phases plus two protected-left phases —
//! while minor junctions keep a two-phase programme.

use serde::{Deserialize, Serialize};

use crate::model::jurisdiction::TrafficRules;
use crate::model::roads::{JunctionPhase, LaneMarking, MarkingKind};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproachSpec {
    pub id: String,
    /// Approach label used in phase movements, e.g. "north".
    pub label: String,
    pub lanes_in: u8,
    pub lanes_out: u8,
    pub protected_left: bool,
    pub right_turn_lane: bool,
    pub pedestrian: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JunctionPlan {
    pub phases: Vec<JunctionPhase>,
    pub markings: Vec<LaneMarking>,
    pub conflict_pairs: Vec<(String, String)>,
    pub signalized: bool,
}

/// Build the phase programme and junction-level markings.  `approaches` must
/// carry the real lane counts so green splits match street capacity.
pub fn synthesize_junction(rules: &TrafficRules, approaches: &[ApproachSpec]) -> JunctionPlan {
    let major = approaches
        .iter()
        .any(|a| a.protected_left || a.right_turn_lane && a.lanes_in >= 3);
    let signalized = approaches.len() >= 3 && major;

    let mut phases = Vec::new();
    if signalized && approaches.len() >= 4 {
        // GB practice: opposite arms release together.  Through phases carry
        // the right-turn streams (China permits right on red, so right lanes
        // keep moving); left turns get their own protected stage.
        let axes: [(usize, usize); 2] = [(0, 2), (1, 3)];
        for (stage, (a, b)) in axes.iter().enumerate() {
            for turn in [false, true] {
                let mut movements = Vec::new();
                for index in [a, b] {
                    if let Some(approach) = approaches.get(*index) {
                        if turn {
                            if approach.protected_left {
                                movements.push(format!("{}/left", approach.label));
                            }
                        } else {
                            movements.push(format!("{}/through", approach.label));
                            if approach.right_turn_lane {
                                movements.push(format!("{}/right", approach.label));
                            }
                        }
                    }
                }
                if movements.is_empty() {
                    continue;
                }
                phases.push(JunctionPhase {
                    id: format!("phase/{stage}/{}", if turn { "left" } else { "through" }),
                    duration_s: if turn { 18.0 } else { 32.0 },
                    movements,
                    pedestrian: !turn,
                });
            }
        }
    } else if signalized {
        // T junctions alternate the through axis against the stem.
        phases.push(JunctionPhase {
            id: "phase/0/through".into(),
            duration_s: 30.0,
            movements: approaches
                .iter()
                .flat_map(|a| {
                    let mut list = vec![format!("{}/through", a.label)];
                    if a.right_turn_lane {
                        list.push(format!("{}/right", a.label));
                    }
                    list
                })
                .collect(),
            pedestrian: true,
        });
        for approach in approaches.iter().filter(|a| a.protected_left) {
            phases.push(JunctionPhase {
                id: format!("phase/{}/left", approach.label),
                duration_s: 20.0,
                movements: vec![format!("{}/left", approach.label)],
                pedestrian: false,
            });
        }
    } else {
        // Minor junctions stay un-signalized in the data model; the renderer
        // shows yield markings instead of signal heads.
        phases.push(JunctionPhase {
            id: "phase/yield".into(),
            duration_s: rules.yellow_duration_s,
            movements: Vec::new(),
            pedestrian: false,
        });
    }

    let conflict_pairs = phases
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            phases
                .iter()
                .skip(i + 1)
                .flat_map(move |b| {
                    a.movements
                        .iter()
                        .flat_map(move |x| b.movements.iter().map(move |y| (x.clone(), y.clone())))
                })
        })
        .collect();

    let markings = if signalized {
        vec![
            LaneMarking {
                kind: MarkingKind::StopLine,
                arrow: None,
                offset_metres: rules.stop_line_offset_m,
                length_metres: 0.30,
                dash_gap_metres: None,
            },
            LaneMarking {
                kind: MarkingKind::Crosswalk,
                arrow: None,
                offset_metres: rules.stop_line_offset_m + rules.crosswalk_width_m * 0.5 + 1.0,
                length_metres: rules.crosswalk_width_m,
                dash_gap_metres: Some(rules.crosswalk_stripe_m),
            },
        ]
    } else {
        vec![LaneMarking {
            kind: MarkingKind::YieldTriangle,
            arrow: None,
            offset_metres: rules.stop_line_offset_m,
            length_metres: 2.0,
            dash_gap_metres: None,
        }]
    };

    JunctionPlan {
        phases,
        markings,
        conflict_pairs,
        signalized,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::jurisdiction::{JurisdictionId, TrafficRules};

    fn four_arm_arterial() -> Vec<ApproachSpec> {
        ["north", "east", "south", "west"]
            .iter()
            .map(|label| ApproachSpec {
                id: format!("approach/{label}"),
                label: (*label).into(),
                lanes_in: 5,
                lanes_out: 4,
                protected_left: true,
                right_turn_lane: true,
                pedestrian: true,
            })
            .collect()
    }

    #[test]
    fn china_arterial_junction_gets_four_phase_programme() {
        let rules = TrafficRules::for_jurisdiction(JurisdictionId::ChinaMainland);
        let plan = synthesize_junction(&rules, &four_arm_arterial());
        assert!(plan.signalized);
        assert_eq!(plan.phases.len(), 4);
        assert!(plan.phases.iter().any(|p| p.id.ends_with("/left")));
        assert!(plan.phases.iter().all(|p| p.duration_s > 0.0));
        assert!(!plan.conflict_pairs.is_empty());
        let through: Vec<_> = plan
            .phases
            .iter()
            .flat_map(|p| &p.movements)
            .filter(|m| m.ends_with("/through"))
            .collect();
        assert_eq!(through.len(), 4, "each arm releases once per cycle");
    }

    #[test]
    fn minor_junction_stays_unsignalized_with_yield_markings() {
        let rules = TrafficRules::for_jurisdiction(JurisdictionId::ChinaMainland);
        let approaches = vec![ApproachSpec {
            id: "approach/a".into(),
            label: "a".into(),
            lanes_in: 1,
            lanes_out: 1,
            protected_left: false,
            right_turn_lane: false,
            pedestrian: true,
        }];
        let plan = synthesize_junction(&rules, &approaches);
        assert!(!plan.signalized);
        assert!(plan
            .markings
            .iter()
            .any(|m| m.kind == MarkingKind::YieldTriangle));
    }
}
