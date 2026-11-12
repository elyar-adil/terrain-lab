//! Rail and air transport: the shared vocabulary of the world planner
//! (`infrastructure`) and the scene builder (`city-scene`), and the rules that
//! make a town accept them. See `docs/transport-infrastructure.md` for the
//! standards each number comes from.

use serde::{Deserialize, Serialize};

use super::core::Point;
use super::scene::ModernCity;

// --- railways ---------------------------------------------------------------

/// The class of a line, which fixes its alignment limits and section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RailKind {
    HighSpeed,
    Mainline,
    Branch,
}

/// Alignment and section limits of a line class (TB 10098, TB 10621, TB 10009).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RailLimits {
    pub design_speed_kmh: f32,
    pub min_radius_m: f32,
    /// Ruling gradient of the design profile, metres per metre.
    pub max_grade: f32,
    /// Steepest gradient of a mountain section (a pass the ruling grade cannot
    /// reach without kilometres of deep tunnel), worked with banking or
    /// double heading on conventional lines.
    pub mountain_grade: f32,
    pub track_spacing_m: f32,
    pub platform_length_m: f32,
    pub platform_height_m: f32,
    pub contact_wire_m: f32,
    pub mast_spacing_m: f32,
    pub ballast_depth_m: f32,
    pub ballast_bed_width_m: f32,
    /// Formation (roadbed) width of a double-track line.
    pub formation_width_m: f32,
    /// Fenced throughout: no level crossings.
    pub closed: bool,
}

impl RailKind {
    pub const fn limits(self) -> RailLimits {
        match self {
            Self::HighSpeed => RailLimits {
                design_speed_kmh: 300.0,
                min_radius_m: 5000.0,
                max_grade: 0.020,
                mountain_grade: 0.030,
                track_spacing_m: 5.0,
                platform_length_m: 450.0,
                platform_height_m: 1.25,
                contact_wire_m: 5.3,
                mast_spacing_m: 50.0,
                ballast_depth_m: 0.35,
                ballast_bed_width_m: 3.6,
                formation_width_m: 13.6,
                closed: true,
            },
            Self::Mainline => RailLimits {
                design_speed_kmh: 160.0,
                min_radius_m: 1200.0,
                max_grade: 0.012,
                mountain_grade: 0.025,
                track_spacing_m: 4.0,
                platform_length_m: 650.0,
                platform_height_m: 1.10,
                contact_wire_m: 5.8,
                mast_spacing_m: 50.0,
                ballast_depth_m: 0.45,
                ballast_bed_width_m: 3.4,
                formation_width_m: 11.6,
                closed: false,
            },
            Self::Branch => RailLimits {
                design_speed_kmh: 120.0,
                min_radius_m: 800.0,
                max_grade: 0.012,
                mountain_grade: 0.025,
                track_spacing_m: 4.0,
                platform_length_m: 650.0,
                platform_height_m: 1.10,
                contact_wire_m: 5.8,
                mast_spacing_m: 50.0,
                ballast_depth_m: 0.45,
                ballast_bed_width_m: 3.4,
                formation_width_m: 11.6,
                closed: false,
            },
        }
    }

    /// Width of the safety protection zone beyond the formation (铁路安全管理条例
    /// art. 27): urban area 8 m (10 m for high speed), suburb 10 (12), village 12 (15),
    /// elsewhere 15 (20).
    pub fn protection_zone_m(self, setting: ProtectionSetting) -> f32 {
        let hsr = self == Self::HighSpeed;
        match (setting, hsr) {
            (ProtectionSetting::Urban, false) => 8.0,
            (ProtectionSetting::Urban, true) => 10.0,
            (ProtectionSetting::Suburb, false) => 10.0,
            (ProtectionSetting::Suburb, true) => 12.0,
            (ProtectionSetting::Village, false) => 12.0,
            (ProtectionSetting::Village, true) => 15.0,
            (ProtectionSetting::Open, false) => 15.0,
            (ProtectionSetting::Open, true) => 20.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectionSetting {
    Urban,
    Suburb,
    Village,
    Open,
}

/// How the track stands relative to the ground at a point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RailGround {
    AtGrade,
    Embankment,
    Cutting,
    Viaduct,
    Bridge,
    Tunnel,
}

impl RailGround {
    pub const fn elevated(self) -> bool {
        matches!(self, Self::Viaduct | Self::Bridge)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RailVertex {
    /// Track-pair centreline, world kilometres.
    pub point: Point,
    /// Chainage along the whole railway, metres.
    pub s_m: f32,
    /// Top of rail above the local ground, metres (negative in a cutting).
    pub above_ground_m: f32,
    pub ground: RailGround,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RailStationPlan {
    /// Chainage of the platform centre.
    pub s_m: f32,
    pub platform_length_m: f32,
    /// Running tracks at the station (2 at a halt, 4 with loops).
    pub tracks: u8,
    /// Side the station building and forecourt are on: +1 left of the line's
    /// direction of travel, -1 right.
    pub building_side: i8,
    /// The line ends here (buffer stops beyond the platforms).
    pub terminal: bool,
    /// A freight yard and depot lie on the far side.
    pub yard: bool,
}

/// One line's part inside a town's plan window.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RailCorridor {
    pub railway_id: u32,
    pub kind: RailKind,
    pub vertices: Vec<RailVertex>,
    pub stations: Vec<RailStationPlan>,
}

impl RailCorridor {
    /// Polyline in city-local metres.
    pub fn local_line(&self, city: &ModernCity) -> Vec<[f32; 2]> {
        self.vertices
            .iter()
            .map(|v| city.frame.to_local(v.point))
            .collect()
    }
}

fn seg_dist(p: [f32; 2], a: [f32; 2], b: [f32; 2]) -> f32 {
    let (vx, vy) = (b[0] - a[0], b[1] - a[1]);
    let l2 = vx * vx + vy * vy;
    let t = if l2 > 1.0e-9 {
        (((p[0] - a[0]) * vx + (p[1] - a[1]) * vy) / l2).clamp(0.0, 1.0)
    } else {
        0.0
    };
    (p[0] - a[0] - vx * t).hypot(p[1] - a[1] - vy * t)
}

fn line_dist(p: [f32; 2], line: &[[f32; 2]]) -> f32 {
    line.windows(2)
        .map(|w| seg_dist(p, w[0], w[1]))
        .fold(f32::INFINITY, f32::min)
}

fn in_ring(p: [f32; 2], ring: &[[f32; 2]]) -> bool {
    let mut inside = false;
    let mut j = ring.len().wrapping_sub(1);
    for i in 0..ring.len() {
        let (a, b) = (ring[i], ring[j]);
        if (a[1] > p[1]) != (b[1] > p[1])
            && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0]
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// The footprint of a station in the line's own frame: along the line from
/// the platform centre, and across it.
pub fn station_zone_half_extent(plan: &RailStationPlan) -> (f32, f32) {
    // Platforms and throat ladders either side; across, the tracks and
    // platforms on one side, a freight yard behind them when there is one.
    (
        plan.platform_length_m * 0.5 + 250.0,
        if plan.yard { 130.0 } else { 45.0 },
    )
}

/// Where a station's forecourt and building stand across the line: from the
/// outer track out to this depth on the building side.
pub const STATION_FORECOURT_DEPTH_M: f32 = 140.0;

impl ModernCity {
    /// Lay railway corridors through the plan: the line, then everything the
    /// formation and its protection zone occupy is cleared (buildings, lots,
    /// blocks, trees, fields), as a real line is built on acquired land.
    /// Returns the number of buildings removed.
    pub fn set_rail(&mut self, rail: Vec<RailCorridor>) -> usize {
        self.rail = rail;
        self.clear_rail_zone()
    }

    fn clear_rail_zone(&mut self) -> usize {
        let frame = self.frame;
        let local = |p: &Point| frame.to_local(*p);
        // The setting decides the protection width: the town's built-up area
        // is "urban"; the plan window's tail is suburb.
        let mut zones: Vec<(Vec<[f32; 2]>, f32)> = Vec::new();
        let mut station_boxes: Vec<[[f32; 2]; 4]> = Vec::new();
        for corridor in &self.rail {
            let limits = corridor.kind.limits();
            let line = corridor.local_line(self);
            if line.len() < 2 {
                continue;
            }
            // Viaducts need no embankment toe; the formation is the deck.
            let half = limits.formation_width_m * 0.5
                + corridor.kind.protection_zone_m(ProtectionSetting::Urban);
            zones.push((line.clone(), half));
            for plan in &corridor.stations {
                let (along, across) = station_zone_half_extent(plan);
                let Some((centre, dir)) = point_at(&corridor.vertices, &line, plan.s_m) else {
                    continue;
                };
                let n = [-dir[1], dir[0]];
                let far = STATION_FORECOURT_DEPTH_M;
                let (z0, z1) = if plan.building_side >= 0 {
                    (-across, far)
                } else {
                    (-far, across)
                };
                let corner = |a: f32, z: f32| {
                    [
                        centre[0] + dir[0] * a + n[0] * z,
                        centre[1] + dir[1] * a + n[1] * z,
                    ]
                };
                station_boxes.push([
                    corner(-along, z0),
                    corner(along, z0),
                    corner(along, z1),
                    corner(-along, z1),
                ]);
            }
        }
        if zones.is_empty() {
            return 0;
        }
        let occupied = |p: [f32; 2]| -> bool {
            zones.iter().any(|(line, half)| line_dist(p, line) < *half)
                || station_boxes.iter().any(|b| in_ring(p, b))
        };
        // A ring is in the way when a corner or the middle is inside a zone, or
        // when the line runs through it.
        let ring_hit = |ring: &[Point]| -> bool {
            if ring.is_empty() {
                return false;
            }
            let pts: Vec<[f32; 2]> = ring.iter().map(local).collect();
            let n = pts.len() as f32;
            let c = [
                pts.iter().map(|p| p[0]).sum::<f32>() / n,
                pts.iter().map(|p| p[1]).sum::<f32>() / n,
            ];
            if occupied(c) || pts.iter().any(|p| occupied(*p)) {
                return true;
            }
            // Edge midpoints catch a long lot with the line through its middle.
            let hit_edge = (0..pts.len()).any(|i| {
                let a = pts[i];
                let b = pts[(i + 1) % pts.len()];
                occupied([(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5])
            });
            hit_edge
                || zones
                    .iter()
                    .any(|(line, _)| line.iter().any(|q| in_ring(*q, &pts)))
        };
        let before = self.buildings.len();
        let gone: std::collections::HashSet<u32> = self
            .parcels
            .iter()
            .filter(|p| ring_hit(&p.ring))
            .map(|p| p.id)
            .collect();
        self.parcels.retain(|p| !gone.contains(&p.id));
        self.buildings
            .retain(|b| !gone.contains(&b.parcel_id) && !ring_hit(&b.footprint));
        self.compounds.retain(|c| !gone.contains(&c.parcel_id));
        self.blocks.retain(|b| !ring_hit(&b.boundary));
        self.trees.retain(|t| !occupied(local(&t.point)));
        self.shelterbelts.retain(|t| !occupied(local(&t.point)));
        self.fields.retain(|f| !ring_hit(&f.ring));
        before - self.buildings.len()
    }

    /// Cap building heights under an airfield's obstacle limitation surfaces.
    /// `ground_at` gives terrain elevation (metres) at a world point. Returns
    /// `(lowered, removed)`.
    pub fn cap_heights_for_airfield(
        &mut self,
        plan: &AirfieldPlan,
        ground_at: impl Fn(Point) -> f32,
    ) -> (usize, usize) {
        let (mut lowered, mut removed) = (0, 0);
        let mut keep = Vec::with_capacity(self.buildings.len());
        for mut b in std::mem::take(&mut self.buildings) {
            if b.footprint.is_empty() {
                keep.push(b);
                continue;
            }
            // The most restrictive point of the footprint decides.
            let mut allowed = f32::INFINITY;
            for p in &b.footprint {
                if let Some(limit) = plan.surface_height_limit(*p) {
                    allowed = allowed.min(plan.elevation_m + limit - ground_at(*p));
                }
            }
            if allowed.is_finite() && b.height_metres > allowed {
                if allowed < 3.0 {
                    removed += 1;
                    continue;
                }
                let per_floor = b.height_metres / f32::from(b.floors.max(1));
                b.floors = ((allowed / per_floor).floor() as u16).max(1);
                b.height_metres = per_floor * f32::from(b.floors);
                lowered += 1;
            }
            keep.push(b);
        }
        self.buildings = keep;
        (lowered, removed)
    }
}

/// Position and unit direction of the corridor line at chainage `s`.
pub fn point_at(
    vertices: &[RailVertex],
    local: &[[f32; 2]],
    s: f32,
) -> Option<([f32; 2], [f32; 2])> {
    if vertices.len() < 2 || vertices.len() != local.len() {
        return None;
    }
    let mut i = 0;
    while i + 2 < vertices.len() && vertices[i + 1].s_m < s {
        i += 1;
    }
    let (a, b) = (local[i], local[i + 1]);
    let span = (vertices[i + 1].s_m - vertices[i].s_m).max(1.0e-3);
    let t = ((s - vertices[i].s_m) / span).clamp(0.0, 1.0);
    let d = [b[0] - a[0], b[1] - a[1]];
    let l = d[0].hypot(d[1]).max(1.0e-6);
    Some(([a[0] + d[0] * t, a[1] + d[1] * t], [d[0] / l, d[1] / l]))
}

// --- airfields -------------------------------------------------------------

/// Aerodrome reference code (ICAO Annex 14 / MH 5001): number by reference
/// field length, letter by wingspan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AerodromeCode {
    C3,
    C4C,
    C4D,
    C4E,
}

impl AerodromeCode {
    pub const fn name(self) -> &'static str {
        match self {
            Self::C3 => "3C",
            Self::C4C => "4C",
            Self::C4D => "4D",
            Self::C4E => "4E",
        }
    }
    pub const fn runway_length_m(self) -> f32 {
        match self {
            Self::C3 => 1800.0,
            Self::C4C => 2800.0,
            Self::C4D => 3200.0,
            Self::C4E => 3400.0,
        }
    }
    pub const fn runway_width_m(self) -> f32 {
        match self {
            Self::C3 => 30.0,
            _ => 45.0,
        }
    }
    /// Paved width including shoulders.
    pub const fn paved_width_m(self) -> f32 {
        match self {
            Self::C3 => 30.0,
            Self::C4C | Self::C4D => 60.0,
            Self::C4E => 75.0,
        }
    }
    pub const fn taxiway_width_m(self) -> f32 {
        match self {
            Self::C3 | Self::C4C => 15.0,
            Self::C4D | Self::C4E => 23.0,
        }
    }
    /// Centre-line separation of the parallel taxiway from an instrument runway.
    pub const fn taxiway_separation_m(self) -> f32 {
        match self {
            Self::C3 => 152.0,
            Self::C4C | Self::C4D => 168.0,
            Self::C4E => 190.0,
        }
    }
    pub const fn strip_width_m(self) -> f32 {
        300.0
    }
    /// Crosswind component permitted for 95 % usability, m/s (20 kt = 37 km/h
    /// for a reference field length of 1 500 m or more).
    pub const fn crosswind_limit_ms(self) -> f32 {
        match self {
            Self::C3 => 13.0 * 0.514_444,
            _ => 20.0 * 0.514_444,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanRect {
    /// Along the runway axis, metres from the runway midpoint.
    pub x0: f32,
    pub x1: f32,
    /// Across, metres; positive is the terminal side.
    pub z0: f32,
    pub z1: f32,
    pub height_m: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxiLink {
    /// From the runway edge to the taxiway centreline, plan metres.
    pub from: [f32; 2],
    pub to: [f32; 2],
}

/// Everything about an airfield's layout, in a frame whose origin is the
/// runway midpoint, `x` along the runway and `z` across toward the terminal.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AirfieldPlan {
    pub code: AerodromeCode,
    pub seed: u32,
    /// Runway midpoint, world kilometres.
    pub centre: Point,
    /// Unit vectors of the local axes in world (kilometre-grid) coordinates.
    pub axis: [f32; 2],
    pub normal: [f32; 2],
    /// Aerodrome elevation: the graded platform level, metres.
    pub elevation_m: f32,
    pub runway_length_m: f32,
    pub runway_width_m: f32,
    pub paved_width_m: f32,
    /// Runway designators at the -x and +x ends (ICAO: bearing / 10).
    pub designator_lo: String,
    pub designator_hi: String,
    pub taxiway_z: f32,
    pub taxiway_width_m: f32,
    pub links: Vec<TaxiLink>,
    pub apron: PlanRect,
    pub stands: Vec<[f32; 2]>,
    pub terminal: PlanRect,
    pub tower: PlanRect,
    pub hangars: Vec<PlanRect>,
    pub car_park: PlanRect,
    pub fence: PlanRect,
    /// The graded platform: where the ground is levelled.
    pub platform: PlanRect,
}

fn designator(bearing_deg: f32) -> String {
    let mut n = (bearing_deg.rem_euclid(360.0) / 10.0).round() as i32;
    if n == 0 {
        n = 36;
    }
    format!("{n:02}")
}

impl AirfieldPlan {
    /// Lay an aerodrome out. `axis_angle` is the runway's axis in the world
    /// grid frame (radians, `x` east, `y` south); `terminal_sign` picks the side
    /// the terminal is on (+1 along the left normal, -1 right).
    pub fn layout(
        code: AerodromeCode,
        seed: u32,
        centre: Point,
        axis_angle: f32,
        terminal_sign: f32,
        elevation_m: f32,
    ) -> Self {
        let (sin, cos) = axis_angle.sin_cos();
        let axis = [cos, sin];
        let s = if terminal_sign < 0.0 { -1.0 } else { 1.0 };
        let normal = [-sin * s, cos * s];
        let l = code.runway_length_m();
        let sep = code.taxiway_separation_m();
        let tw = code.taxiway_width_m();
        // Hashes for the few free choices; the rest follows from the code.
        let h = |salt: u32| -> f32 {
            let mut x = seed ^ salt.wrapping_mul(0x9e37_79b9);
            x ^= x >> 15;
            x = x.wrapping_mul(0x2c1b_3c6d);
            x ^= x >> 12;
            x = x.wrapping_mul(0x297a_2d39);
            x ^= x >> 15;
            (x & 0xffff) as f32 / 65535.0
        };
        // Compass bearing of the +x direction with grid y south.
        let bearing = (90.0 + axis_angle.to_degrees()).rem_euclid(360.0);
        let designator_hi = designator(bearing);
        let designator_lo = designator(bearing + 180.0);
        // Links: end links at both runway ends and two 30-degree rapid exits
        // for each direction, 600-900 m beyond the touchdown zone.
        let end = l * 0.5 - 90.0;
        let run = sep / 30.0_f32.to_radians().tan();
        let mut links = vec![
            TaxiLink {
                from: [-end, 0.0],
                to: [-end, sep],
            },
            TaxiLink {
                from: [end, 0.0],
                to: [end, sep],
            },
        ];
        for k in 0..2 {
            let off = 1650.0 + 300.0 * h(11 + k) + 400.0 * k as f32;
            links.push(TaxiLink {
                from: [-l * 0.5 + off, 0.0],
                to: [-l * 0.5 + off + run, sep],
            });
            links.push(TaxiLink {
                from: [l * 0.5 - off, 0.0],
                to: [l * 0.5 - off - run, sep],
            });
        }
        // Apron: stands nose-in toward the terminal; pitch for code C is 45 m of
        // wingspan plus 9 m clearance.
        let stand_count = 6 + (h(3) * 5.0) as usize;
        let pitch = 54.0;
        let apron_half = stand_count as f32 * pitch * 0.5 + 40.0;
        let apron_cx = (h(4) - 0.5) * 300.0;
        let apron_z0 = sep + tw * 0.5 + 40.0;
        let apron_z1 = apron_z0 + 190.0;
        let apron = PlanRect {
            x0: apron_cx - apron_half,
            x1: apron_cx + apron_half,
            z0: sep + tw * 0.5,
            z1: apron_z1,
            height_m: 0.0,
        };
        let stands: Vec<[f32; 2]> = (0..stand_count)
            .map(|i| {
                [
                    apron_cx - apron_half + 40.0 + pitch * (i as f32 + 0.5),
                    apron_z1 - 45.0,
                ]
            })
            .collect();
        let term_half = stand_count as f32 * pitch * 0.5 * 0.9;
        let terminal = PlanRect {
            x0: apron_cx - term_half,
            x1: apron_cx + term_half,
            z0: apron_z1 + 12.0,
            z1: apron_z1 + 12.0 + 62.0,
            height_m: 18.0 + 4.0 * h(5),
        };
        let tower = PlanRect {
            x0: apron.x1 + 60.0,
            x1: apron.x1 + 60.0 + 14.0,
            z0: apron_z0 + 90.0,
            z1: apron_z0 + 90.0 + 14.0,
            height_m: 38.0,
        };
        let hangar_x = apron.x0 - 90.0;
        let hangars = (0..3)
            .map(|i| PlanRect {
                x0: hangar_x - 70.0 * (i as f32 + 1.0) - 12.0 * i as f32,
                x1: hangar_x - 12.0 * i as f32 - 70.0 * i as f32,
                z0: apron_z0 + 20.0,
                z1: apron_z0 + 20.0 + 60.0,
                height_m: 13.0 + 2.0 * h(20 + i),
            })
            .collect();
        let car_park = PlanRect {
            x0: terminal.x0 - 20.0,
            x1: terminal.x1 + 20.0,
            z0: terminal.z1 + 40.0,
            z1: terminal.z1 + 40.0 + 110.0,
            height_m: 0.0,
        };
        let fence = PlanRect {
            x0: -l * 0.5 - 240.0,
            x1: l * 0.5 + 240.0,
            z0: -190.0,
            z1: car_park.z1 + 30.0,
            height_m: 2.5,
        };
        let platform = PlanRect {
            x0: fence.x0 - 60.0,
            x1: fence.x1 + 60.0,
            z0: fence.z0 - 60.0,
            z1: fence.z1 + 60.0,
            height_m: 0.0,
        };
        Self {
            code,
            seed,
            centre,
            axis,
            normal,
            elevation_m,
            runway_length_m: l,
            runway_width_m: code.runway_width_m(),
            paved_width_m: code.paved_width_m(),
            designator_lo,
            designator_hi,
            taxiway_z: sep,
            taxiway_width_m: tw,
            links,
            apron,
            stands,
            terminal,
            tower,
            hangars,
            car_park,
            fence,
            platform,
        }
    }

    /// World kilometre point to the plan's (x along, z across) metres.
    pub fn to_local(&self, p: Point) -> [f32; 2] {
        let dx = (p.x_km - self.centre.x_km) * 1000.0;
        let dy = (p.y_km - self.centre.y_km) * 1000.0;
        [
            dx * self.axis[0] + dy * self.axis[1],
            dx * self.normal[0] + dy * self.normal[1],
        ]
    }

    pub fn to_world(&self, x: f32, z: f32) -> Point {
        Point {
            x_km: self.centre.x_km + (x * self.axis[0] + z * self.normal[0]) / 1000.0,
            y_km: self.centre.y_km + (x * self.axis[1] + z * self.normal[1]) / 1000.0,
        }
    }

    /// Height limit above aerodrome elevation (Annex 14 Table 4-1, precision
    /// approach category I, code 3/4): the lowest of the approach, transitional,
    /// inner horizontal and conical surfaces over a point. `None` where none
    /// applies.
    pub fn surface_height_limit(&self, p: Point) -> Option<f32> {
        let [x, z] = self.to_local(p);
        let strip_half_len = self.runway_length_m * 0.5 + 60.0;
        let strip_half_w = self.code.strip_width_m() * 0.5;
        // Distance from the strip rectangle.
        let ox = (x.abs() - strip_half_len).max(0.0);
        let oz = (z.abs() - strip_half_w).max(0.0);
        let d = ox.hypot(oz);
        let mut limit = f32::INFINITY;
        // Inner horizontal: 45 m within 4 000 m of the strip. Conical: 5 % up to 100 m above it.
        if d <= 4000.0 {
            limit = limit.min(45.0);
        } else if d <= 4000.0 + 55.0 / 0.05 {
            limit = limit.min(45.0 + (d - 4000.0) * 0.05);
        }
        // Transitional: 14.3 % from the strip edge up to the inner horizontal.
        if ox <= 0.0 && oz > 0.0 && oz * 0.143 <= 45.0 {
            limit = limit.min(oz * 0.143);
        }
        // Approach: from the strip end, 280 m wide, 15 % divergence, 2 % for
        // 3 000 m, 2.5 % for 3 600 m, then level at 150 m to 15 000 m.
        if ox > 0.0 && ox <= 15000.0 {
            let half = 140.0 + 0.15 * ox;
            if z.abs() <= half {
                let h = if ox <= 3000.0 {
                    0.02 * ox
                } else if ox <= 6600.0 {
                    60.0 + 0.025 * (ox - 3000.0)
                } else {
                    150.0
                };
                limit = limit.min(h);
            }
        }
        limit.is_finite().then_some(limit)
    }

    /// Bearing of the +x end of the runway, compass degrees (grid y south).
    pub fn bearing_deg(&self) -> f32 {
        (90.0 + self.axis[1].atan2(self.axis[0]).to_degrees()).rem_euclid(360.0)
    }
}
