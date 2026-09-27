use serde::{Deserialize, Serialize};

use super::core::{ModernRoadClass, Point};

/// Physical cross-section of a modern Chinese street, per CJJ 37 / GB 50647.
///
/// This lives in the model, not in a generator, because it is a *contract*:
/// the plan generator uses it to lay out lanes and the street-scene layer uses
/// it to lay out the carriageway, kerbs, markings and sidewalks.  Two
/// definitions would let the drawn road and the simulated road disagree about
/// where the median ends.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RoadCrossSection {
    /// Total ribbon width, kerb face to kerb face.
    pub width_metres: f32,
    /// Physical central median (中央隔离带); zero when undivided.
    pub median_metres: f32,
    pub motor_lanes_per_direction: u8,
    pub motor_lane_width: f32,
    /// Hard shoulder inside the carriageway (expressways only).
    pub shoulder_width: f32,
    /// Non-motorized lane outside the motor carriageway.
    pub bike_lane_width: f32,
    /// Inner buffer between the median/centreline and the first motor lane.
    pub inner_shoulder: f32,
    /// Sidewalk width outside the carriageway.  Kerbs and the pedestrian realm
    /// are sized from this, so it belongs with the cross-section.
    pub sidewalk_metres: f32,
}

impl RoadCrossSection {
    /// Outer edge of the motor carriageway, measured from the geometric centre.
    pub fn half_carriageway(&self) -> f32 {
        self.median_metres * 0.5
            + self.inner_shoulder
            + self.motor_lanes_per_direction as f32 * self.motor_lane_width
    }

    /// Distance from the geometric centre to the kerb face.
    pub fn half_width(&self) -> f32 {
        self.width_metres * 0.5
    }

    /// Offset of motor lane `index` (0 = median side) for a carriageway
    /// travelling in `direction` (`+1` means the carriageway lies on the left
    /// normal, which is right-hand traffic).
    pub fn lane_offset(&self, direction: i8, index: u8) -> f32 {
        let distance = self.median_metres * 0.5
            + self.inner_shoulder
            + (self.motor_lanes_per_direction.saturating_sub(index).max(1)) as f32
                * self.motor_lane_width
            - self.motor_lane_width * 0.5;
        direction as f32 * distance
    }

    /// Offset of the curb-side non-motorized lane, if the class has one.
    pub fn bike_offset(&self, direction: i8) -> Option<f32> {
        if self.bike_lane_width <= 0.0 {
            return None;
        }
        Some(direction as f32 * (self.half_carriageway() + self.bike_lane_width * 0.5))
    }

    pub fn has_median(&self) -> bool {
        self.median_metres > 0.3
    }
}

/// Cross-sections for the four Chinese road classes.
pub fn cross_section(class: ModernRoadClass) -> RoadCrossSection {
    match class {
        // 快速路: 双向八车道 + 中央隔离带 + 硬路肩, no non-motorized traffic.
        ModernRoadClass::Expressway => RoadCrossSection {
            width_metres: 36.0,
            median_metres: 2.5,
            motor_lanes_per_direction: 4,
            motor_lane_width: 3.5,
            shoulder_width: 1.0,
            bike_lane_width: 0.0,
            inner_shoulder: 0.75,
            sidewalk_metres: 0.0,
        },
        // 主干路: 双向六车道 + 中央分隔带 + 机非分隔的非机动车道.
        ModernRoadClass::Arterial => RoadCrossSection {
            width_metres: 32.0,
            median_metres: 2.0,
            motor_lanes_per_direction: 3,
            motor_lane_width: 3.5,
            shoulder_width: 0.0,
            bike_lane_width: 3.5,
            inner_shoulder: 0.5,
            sidewalk_metres: 3.0,
        },
        // 次干路: 双向四车道 + 非机动车道, no median.
        ModernRoadClass::Collector => RoadCrossSection {
            width_metres: 22.0,
            median_metres: 0.0,
            motor_lanes_per_direction: 2,
            motor_lane_width: 3.25,
            shoulder_width: 0.0,
            bike_lane_width: 3.0,
            inner_shoulder: 0.4,
            sidewalk_metres: 2.5,
        },
        // 支路: 单车道双向混行, parking and shared non-motorized use.
        ModernRoadClass::Local => RoadCrossSection {
            width_metres: 12.0,
            median_metres: 0.0,
            motor_lanes_per_direction: 1,
            motor_lane_width: 3.25,
            shoulder_width: 0.0,
            bike_lane_width: 0.0,
            inner_shoulder: 0.4,
            sidewalk_metres: 2.0,
        },
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdNode {
    pub id: u32,
    pub point: Point,
    /// Stable semantic role used by editors and renderers.  Keeping this on
    /// the SD graph means gateways, district centres and local junctions do
    /// not get flattened into anonymous grid vertices.
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdRoad {
    pub id: u32,
    pub from: u32,
    pub to: u32,
    pub class: ModernRoadClass,
    pub bridge: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HdRoad {
    pub id: u32,
    pub sd_road: u32,
    pub class: ModernRoadClass,
    pub width_metres: f32,
    /// Physical central median (中央隔离带) width in metres; zero when the
    /// street is undivided.  Arterials and expressways in a Chinese city are
    /// always divided, locals never are.
    pub median_metres: f32,
    pub centreline: Vec<Point>,
    pub bridge: bool,
    pub layer: i8,
    pub lanes: Vec<HdLane>,
    pub connectors: Vec<RoadConnector>,
}

impl HdRoad {
    pub fn cross_section(&self) -> RoadCrossSection {
        cross_section(self.class)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LaneUse {
    General,
    LeftTurn,
    Through,
    RightTurn,
    UTurn,
    Bus,
    Bike,
    Shoulder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Movement {
    Left,
    Through,
    Right,
    UTurn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TurnArrow {
    Straight,
    Left,
    Right,
    StraightLeft,
    StraightRight,
    UTurn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MarkingKind {
    StopLine,
    Crosswalk,
    SolidEdge,
    DashedDivider,
    Arrow,
    YieldTriangle,
    GuideLine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaneMarking {
    pub kind: MarkingKind,
    pub arrow: Option<TurnArrow>,
    pub offset_metres: f32,
    pub length_metres: f32,
    pub dash_gap_metres: Option<f32>,
}

/// A lane is a semantic render primitive, not just a width count.  The
/// renderer can place exact lane lines and Chinese GB arrows from this data at
/// any zoom level without guessing from a broad road ribbon.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HdLane {
    pub id: String,
    pub direction: i8,
    pub index_from_curb: u8,
    pub use_type: LaneUse,
    pub width_metres: f32,
    pub offset_metres: f32,
    pub allowed_movements: Vec<Movement>,
    pub markings: Vec<LaneMarking>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoadConnector {
    pub id: String,
    pub from_road: u32,
    pub to_road: u32,
    pub node: u32,
    pub from_lane: Option<String>,
    pub to_lane: Option<String>,
    pub movement: Movement,
    pub centreline: Vec<Point>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JunctionKind {
    Signalized,
    Channelized,
    Roundabout,
    GradeSeparated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JunctionPhase {
    pub id: String,
    pub duration_s: f32,
    pub movements: Vec<String>,
    pub pedestrian: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalHead {
    pub id: String,
    pub road_id: u32,
    pub position: Point,
    pub height_metres: f32,
    pub aspects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CityJunction {
    pub id: u32,
    pub node: u32,
    pub road_ids: Vec<u32>,
    pub kind: JunctionKind,
    pub phases: Vec<JunctionPhase>,
    pub signal_heads: Vec<SignalHead>,
    pub markings: Vec<LaneMarking>,
    pub crosswalks: Vec<Vec<Point>>,
    pub connectors: Vec<RoadConnector>,
    /// Movement pairs that would conflict if released simultaneously, ported
    /// from the source kernel's safety model.
    pub conflict_pairs: Vec<(String, String)>,
}
