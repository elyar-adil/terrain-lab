//! Ported from the source city kernel: traffic and streetscape conventions are
//! data, not hard-coded geometry.  The mainland-China profile follows the real
//! design dimensions of CJJ 37 (urban road engineering) and GB 5768 (markings),
//! so a junction built from these numbers has the same cross-section a Chinese
//! municipal drawing would.

use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum JurisdictionId {
    ChinaMainland,
    UnitedStates,
    Germany,
    Japan,
}

impl fmt::Display for JurisdictionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ChinaMainland => "china",
            Self::UnitedStates => "usa",
            Self::Germany => "germany",
            Self::Japan => "japan",
        })
    }
}

impl FromStr for JurisdictionId {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "cn" | "china" | "china-mainland" => Ok(Self::ChinaMainland),
            "us" | "usa" | "united-states" => Ok(Self::UnitedStates),
            "de" | "germany" => Ok(Self::Germany),
            "jp" | "japan" => Ok(Self::Japan),
            other => Err(format!("unknown jurisdiction: {other}")),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DrivingSide {
    Right,
    Left,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SignalStyle {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ArrowStyle {
    ChinaGb,
    Vienna,
    NorthAmerica,
    Japan,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrafficRules {
    pub jurisdiction: JurisdictionId,
    pub driving_side: DrivingSide,
    pub signal_style: SignalStyle,
    pub arrow_style: ArrowStyle,
    /// Motor-lane width.  China designs 3.25-3.5 m motor lanes (CJJ 37); the
    /// profile picks 3.5 m because the generator's arterials carry buses.
    pub lane_width_m: f32,
    /// Arterial/expressway lane width, wider than the local default.
    pub arterial_lane_width_m: f32,
    /// Non-motorized (bicycle) lane: GB 50647 keeps 3.5 m for two-way flow.
    pub bike_lane_width_m: f32,
    pub sidewalk_width_m: f32,
    /// Central median on arterials and expressways (护栏/绿化带).
    pub median_width_m: f32,
    pub stop_line_offset_m: f32,
    pub crosswalk_width_m: f32,
    pub crosswalk_stripe_m: f32,
    pub crosswalk_gap_m: f32,
    pub default_right_turn_yield: bool,
    pub right_turn_on_red: bool,
    pub yellow_duration_s: f32,
}

impl TrafficRules {
    pub const fn for_jurisdiction(jurisdiction: JurisdictionId) -> Self {
        match jurisdiction {
            JurisdictionId::ChinaMainland => Self {
                jurisdiction,
                driving_side: DrivingSide::Right,
                signal_style: SignalStyle::Vertical,
                arrow_style: ArrowStyle::ChinaGb,
                lane_width_m: 3.25,
                arterial_lane_width_m: 3.5,
                bike_lane_width_m: 3.5,
                sidewalk_width_m: 4.0,
                median_width_m: 2.0,
                stop_line_offset_m: 3.0,
                crosswalk_width_m: 5.0,
                crosswalk_stripe_m: 0.45,
                crosswalk_gap_m: 0.55,
                default_right_turn_yield: true,
                right_turn_on_red: true,
                yellow_duration_s: 3.0,
            },
            JurisdictionId::UnitedStates => Self {
                jurisdiction,
                driving_side: DrivingSide::Right,
                signal_style: SignalStyle::Horizontal,
                arrow_style: ArrowStyle::NorthAmerica,
                lane_width_m: 3.6,
                arterial_lane_width_m: 3.6,
                bike_lane_width_m: 1.8,
                sidewalk_width_m: 2.4,
                median_width_m: 3.0,
                stop_line_offset_m: 3.6,
                crosswalk_width_m: 3.0,
                crosswalk_stripe_m: 0.6,
                crosswalk_gap_m: 0.6,
                default_right_turn_yield: true,
                right_turn_on_red: true,
                yellow_duration_s: 4.0,
            },
            JurisdictionId::Germany => Self {
                jurisdiction,
                driving_side: DrivingSide::Right,
                signal_style: SignalStyle::Vertical,
                arrow_style: ArrowStyle::Vienna,
                lane_width_m: 3.25,
                arterial_lane_width_m: 3.5,
                bike_lane_width_m: 2.0,
                sidewalk_width_m: 2.5,
                median_width_m: 1.6,
                stop_line_offset_m: 3.0,
                crosswalk_width_m: 3.0,
                crosswalk_stripe_m: 0.5,
                crosswalk_gap_m: 0.5,
                default_right_turn_yield: false,
                right_turn_on_red: false,
                yellow_duration_s: 3.0,
            },
            JurisdictionId::Japan => Self {
                jurisdiction,
                driving_side: DrivingSide::Left,
                signal_style: SignalStyle::Vertical,
                arrow_style: ArrowStyle::Japan,
                lane_width_m: 3.0,
                arterial_lane_width_m: 3.25,
                bike_lane_width_m: 2.0,
                sidewalk_width_m: 3.0,
                median_width_m: 1.2,
                stop_line_offset_m: 2.5,
                crosswalk_width_m: 4.0,
                crosswalk_stripe_m: 0.45,
                crosswalk_gap_m: 0.45,
                default_right_turn_yield: false,
                right_turn_on_red: false,
                yellow_duration_s: 3.0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aliases_are_stable_and_profiles_differ() {
        assert_eq!(
            "cn".parse::<JurisdictionId>().unwrap(),
            JurisdictionId::ChinaMainland
        );
        assert_eq!(
            "japan".parse::<JurisdictionId>().unwrap().to_string(),
            "japan"
        );
        let china = TrafficRules::for_jurisdiction(JurisdictionId::ChinaMainland);
        assert_eq!(china.arrow_style, ArrowStyle::ChinaGb);
        assert_eq!(china.arterial_lane_width_m, 3.5);
        assert!(china.right_turn_on_red);
        assert_ne!(
            china.arrow_style,
            TrafficRules::for_jurisdiction(JurisdictionId::UnitedStates).arrow_style
        );
    }
}
