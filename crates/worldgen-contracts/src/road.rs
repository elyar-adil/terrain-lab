//! Road classes and their physical cross-sections: the one definition.
//!
//! Before this module the same facts lived in three places that disagreed (a
//! 32 m arterial in the street planner, a 37 m one in the street scene, a 13 m one
//! in the regional network). A road is *one* thing whose width depends on its
//! functional class and on whether it runs through town or open country, so that
//! is what this module models. Everything that draws, simulates, trims or reserves
//! land for a road reads these numbers.
//!
//! Values follow mainland-Chinese practice (CJJ 37 for urban roads, JTG B01 for
//! highways). Other jurisdictions are a different table, not different code.

/// Functional class, **ascending importance**: the derived ordering is the
/// ordering, so `a > b` means "a is the bigger road". (An earlier enum listed the
/// widest class first and every "keep the higher class" comparison had to
/// remember to invert it.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RoadClass {
    /// Field track, alley, footpath-width lane. Never an arterial of anything.
    Track,
    /// Village road, estate or yard road, service lane.
    Service,
    /// 支路 / 县乡道.
    Local,
    /// 次干路 / 二级公路.
    Collector,
    /// 主干路 / 一级公路 (国道).
    Arterial,
    /// 快速路 / 高速公路: grade-separated, no frontage, no pedestrians.
    Motorway,
}

impl RoadClass {
    pub const ALL: [RoadClass; 6] = [
        RoadClass::Track,
        RoadClass::Service,
        RoadClass::Local,
        RoadClass::Collector,
        RoadClass::Arterial,
        RoadClass::Motorway,
    ];

    /// 0 for a track up to 5 for a motorway.
    pub fn rank(self) -> u8 {
        self as u8
    }

    pub fn from_rank(rank: u8) -> RoadClass {
        Self::ALL[(rank as usize).min(Self::ALL.len() - 1)]
    }

    /// One class smaller, stopping at the smallest.
    pub fn demoted(self) -> RoadClass {
        Self::from_rank(self.rank().saturating_sub(1))
    }

    pub fn name(self) -> &'static str {
        match self {
            RoadClass::Track => "track",
            RoadClass::Service => "service",
            RoadClass::Local => "local",
            RoadClass::Collector => "collector",
            RoadClass::Arterial => "arterial",
            RoadClass::Motorway => "motorway",
        }
    }

    pub fn cross_section(self, setting: Setting) -> CrossSection {
        cross_section(self, setting)
    }
}

/// Whether a road runs through built-up land or open country.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Setting {
    Rural,
    Urban,
}

/// Physical cross-section, all metres. Symmetric about the centreline.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrossSection {
    /// Central median (中央分隔带); zero when undivided.
    pub median: f64,
    /// Lanes in each direction.
    pub lanes_per_direction: u8,
    pub lane_width: f64,
    /// Buffer between the median or centreline and the first lane.
    pub inner_shoulder: f64,
    /// Hard shoulder outside the lanes.
    pub shoulder: f64,
    /// Separate non-motorised lane outside the lanes.
    pub bike_lane: f64,
    /// Pavement outside the carriageway.
    pub sidewalk: f64,
    /// Land held beside the pavement edge (embankment, drainage, planting): the
    /// right of way is wider than the road by twice this.
    pub verge: f64,
    /// One lane serves both directions (a track or alley): the two halves of the
    /// symmetric cross-section are then one lane, not two.
    pub shared_lane: bool,
    pub paved: bool,
    pub design_speed_kph: f64,
}

impl CrossSection {
    /// Kerb face to kerb face, counting every component on both sides.
    pub fn width(&self) -> f64 {
        2.0 * (self.median * 0.5
            + self.inner_shoulder
            + f64::from(self.lanes_per_direction) * self.lane_width
            + self.shoulder
            + self.bike_lane
            + self.sidewalk)
    }
    pub fn half_width(&self) -> f64 {
        self.width() * 0.5
    }
    /// Right of way: the road plus its verges.
    pub fn right_of_way(&self) -> f64 {
        self.width() + 2.0 * self.verge
    }
    /// Distance from the centreline to the outer edge of the motor lanes.
    pub fn half_carriageway(&self) -> f64 {
        self.median * 0.5 + self.inner_shoulder + f64::from(self.lanes_per_direction) * self.lane_width
    }
    /// Lanes in total, both directions.
    pub fn lanes(&self) -> u8 {
        if self.shared_lane { 1 } else { self.lanes_per_direction * 2 }
    }
    pub fn has_median(&self) -> bool {
        self.median > 0.3
    }
    /// Offset of motor lane `index` (0 is nearest the kerb) for a carriageway
    /// travelling in `direction` (+1: the carriageway lies on the left normal,
    /// which is right-hand traffic).
    pub fn lane_offset(&self, direction: i8, index_from_kerb: u8) -> f64 {
        let from_centre = self.median * 0.5
            + self.inner_shoulder
            + f64::from(self.lanes_per_direction.saturating_sub(index_from_kerb).max(1)) * self.lane_width
            - self.lane_width * 0.5;
        f64::from(direction) * from_centre
    }
    pub fn bike_offset(&self, direction: i8) -> Option<f64> {
        (self.bike_lane > 0.0).then(|| f64::from(direction) * (self.half_carriageway() + self.shoulder + self.bike_lane * 0.5))
    }
}

/// The table. Urban values are CJJ 37; rural are JTG B01 practice.
pub fn cross_section(class: RoadClass, setting: Setting) -> CrossSection {
    use RoadClass::*;
    use Setting::*;
    let s = |median, lanes, lane_width, inner_shoulder, shoulder, bike_lane, sidewalk, verge, paved, speed| CrossSection {
        median,
        lanes_per_direction: lanes,
        lane_width,
        inner_shoulder,
        shoulder,
        bike_lane,
        sidewalk,
        verge,
        shared_lane: lanes == 1 && lane_width < 2.0,
        paved,
        design_speed_kph: speed,
    };
    match (setting, class) {
        // 快速路: 双向八车道 + 中央隔离带 + 硬路肩, no non-motorised traffic.
        (Urban, Motorway) => s(2.5, 4, 3.5, 0.75, 1.0, 0.0, 0.0, 6.0, true, 80.0),
        // 主干路: 双向六车道 + 中央分隔带 + 机非分隔的非机动车道 + 人行道.
        (Urban, Arterial) => s(2.0, 3, 3.5, 0.5, 0.0, 3.5, 3.0, 0.0, true, 60.0),
        // 次干路: 双向四车道 + 非机动车道, no median.
        (Urban, Collector) => s(0.0, 2, 3.25, 0.4, 0.0, 3.0, 2.5, 0.0, true, 50.0),
        // 支路: 双向两车道, shared non-motorised use.
        (Urban, Local) => s(0.0, 1, 3.25, 0.4, 0.0, 0.0, 2.0, 0.0, true, 30.0),
        // 小区路 / 内部道路.
        (Urban, Service) => s(0.0, 1, 2.75, 0.25, 0.0, 0.0, 1.0, 0.0, true, 20.0),
        // 巷 / 弄.
        (Urban, Track) => s(0.0, 1, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0, true, 10.0),

        // 高速公路: 双向四车道 + 中央分隔带 + 硬路肩.
        (Rural, Motorway) => s(3.0, 2, 3.75, 0.75, 2.5, 0.0, 0.0, 6.75, true, 100.0),
        // 一级公路 / 国道: 双向四车道, narrow median.
        (Rural, Arterial) => s(1.5, 2, 3.5, 0.5, 1.5, 0.0, 0.0, 3.0, true, 80.0),
        // 二级公路: 双向两车道 + 硬路肩.
        (Rural, Collector) => s(0.0, 1, 3.5, 0.0, 1.25, 0.0, 0.0, 2.5, true, 60.0),
        // 县乡道.
        (Rural, Local) => s(0.0, 1, 3.0, 0.0, 0.5, 0.0, 0.0, 2.0, true, 40.0),
        // 村道: concrete, single carriageway.
        (Rural, Service) => s(0.0, 1, 2.5, 0.0, 0.25, 0.0, 0.0, 1.5, true, 30.0),
        // 田间路: earth or gravel.
        (Rural, Track) => s(0.0, 1, 1.75, 0.0, 0.0, 0.0, 0.0, 1.0, false, 15.0),
    }
}

/// How far back from a junction node each street is trimmed so the kerbs can
/// round the corner: half the widest street plus a kerb-return allowance.
///
/// The earlier formula left a three-metre corner radius on an arterial, which
/// reads as a square corner. A node with two streets is a bend and gets a small
/// trim. Streets and street scenes share this function; two copies would let the
/// drawn kerb and the planned junction disagree.
pub fn junction_trim_m(widest_width_m: f64, arms: usize) -> f64 {
    if arms <= 2 {
        return (widest_width_m * 0.5 + 2.0).min(8.0);
    }
    let kerb_return = if widest_width_m >= 12.0 {
        12.0
    } else if widest_width_m >= 6.5 {
        8.0
    } else {
        5.0
    };
    widest_width_m * 0.5 + kerb_return + 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_ordering_is_the_importance() {
        let mut sorted = RoadClass::ALL;
        sorted.sort();
        assert_eq!(sorted, RoadClass::ALL);
        assert!(RoadClass::Arterial > RoadClass::Local);
        assert_eq!(RoadClass::Local.demoted(), RoadClass::Service);
        assert_eq!(RoadClass::Track.demoted(), RoadClass::Track);
        for c in RoadClass::ALL {
            assert_eq!(RoadClass::from_rank(c.rank()), c);
        }
    }

    #[test]
    fn a_bigger_road_never_carries_traffic_on_less_pavement_within_a_setting() {
        // The motor carriageway, not the whole ribbon: an urban arterial with cycle
        // lanes and pavements is wider kerb to kerb than an expressway, as in life.
        let motor = |c: CrossSection| 2.0 * (c.half_carriageway() + c.shoulder);
        for setting in [Setting::Rural, Setting::Urban] {
            for pair in RoadClass::ALL.windows(2) {
                let (small, big) = (pair[0].cross_section(setting), pair[1].cross_section(setting));
                assert!(motor(big) > motor(small), "{:?} {:?}: {} vs {}", setting, pair[1], motor(big), motor(small));
                assert!(big.design_speed_kph >= small.design_speed_kph);
                assert!(big.lanes() >= small.lanes());
            }
        }
    }

    #[test]
    fn the_urban_widths_are_the_ones_the_street_scene_has_always_drawn() {
        let w = |c| cross_section(c, Setting::Urban).width();
        assert!((w(RoadClass::Motorway) - 34.0).abs() < 1e-9);
        assert!((w(RoadClass::Arterial) - 37.0).abs() < 1e-9);
        assert!((w(RoadClass::Collector) - 24.8).abs() < 1e-9);
        assert!((w(RoadClass::Local) - 11.3).abs() < 1e-9);
    }

    #[test]
    fn open_country_roads_are_narrower_than_the_same_class_in_town() {
        for c in [RoadClass::Arterial, RoadClass::Collector, RoadClass::Local, RoadClass::Service] {
            assert!(
                c.cross_section(Setting::Rural).width() < c.cross_section(Setting::Urban).width(),
                "{c:?} should widen as it enters a town"
            );
        }
    }

    #[test]
    fn only_the_lowest_rural_class_is_unpaved_and_none_in_town_is() {
        for c in RoadClass::ALL {
            assert!(c.cross_section(Setting::Urban).paved);
            assert_eq!(c.cross_section(Setting::Rural).paved, c != RoadClass::Track);
        }
    }

    #[test]
    fn lanes_sit_inside_the_carriageway_and_mirror_about_the_centre() {
        let a = cross_section(RoadClass::Arterial, Setting::Urban);
        for dir in [-1i8, 1] {
            for i in 0..a.lanes_per_direction {
                let off = a.lane_offset(dir, i).abs();
                assert!(off - a.lane_width * 0.5 >= a.median * 0.5 - 1e-9);
                assert!(off + a.lane_width * 0.5 <= a.half_carriageway() + 1e-9);
            }
        }
        assert_eq!(a.lane_offset(1, 0), -a.lane_offset(-1, 0));
        assert!(a.bike_offset(1).unwrap() > a.half_carriageway());
        assert!(cross_section(RoadClass::Local, Setting::Urban).bike_offset(1).is_none());
        assert!(a.right_of_way() >= a.width());
    }

    #[test]
    fn a_track_is_one_lane_for_both_directions_and_a_local_road_is_two() {
        assert_eq!(cross_section(RoadClass::Track, Setting::Rural).lanes(), 1);
        assert_eq!(cross_section(RoadClass::Track, Setting::Urban).lanes(), 1);
        assert_eq!(cross_section(RoadClass::Local, Setting::Rural).lanes(), 2);
        assert_eq!(cross_section(RoadClass::Motorway, Setting::Rural).lanes(), 4);
        assert_eq!(cross_section(RoadClass::Arterial, Setting::Urban).lanes(), 6);
    }

    #[test]
    fn a_junction_trim_leaves_room_for_a_kerb_return() {
        // The old arterial trim was about 9 m, leaving a 3 m radius: a square corner.
        let w = cross_section(RoadClass::Arterial, Setting::Urban).width();
        assert!(junction_trim_m(w, 4) - w * 0.5 >= 12.0);
        assert!(junction_trim_m(w, 2) <= 8.0);
        assert!(junction_trim_m(11.3, 3) < junction_trim_m(w, 3));
    }
}
