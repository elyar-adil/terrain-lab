//! Railways in a town: formation or viaduct, track, catenary, fences, signals,
//! level crossings, stations with platforms, canopies and yards, and trains.
//!
//! Sizes follow `docs/transport-infrastructure.md` (TB 10098, TB 10621,
//! TB 10009, GB 50091). The line itself comes from `urban::RailCorridor`,
//! planned on the terrain by the world planner; this module only builds what
//! that plan implies and checks it against the street network it meets.

use urban::{ModernCity, Point, RailCorridor, RailGround, RailKind, RailLimits, RailStationPlan};

use crate::math::{Path, Rng, Vec2, Vec3};
use crate::mesh::{GroupStyle, MeshBuilder, box_at, face_normal};
use crate::network::Network;

/// What was built, and what was found wrong.
#[derive(Debug, Clone, Default)]
pub struct RailStats {
    pub corridors: usize,
    pub length_m: f32,
    pub sleepers: usize,
    pub masts: usize,
    pub piers: usize,
    pub level_crossings: usize,
    pub overpasses: usize,
    pub stations: usize,
    pub train_cars: usize,
    pub warnings: Vec<String>,
}

/// Rail head height above the surrounding ground on a line at grade.
const HEAD_ABOVE_GROUND: f32 = 0.30;
/// Rail head to the top of the ballast shoulder / slab.
const BED_BELOW_HEAD: f32 = 0.20;
const RAIL_HEIGHT: f32 = 0.176;
const GAUGE_CENTRES: f32 = 1.50;
/// Girder depth under a viaduct deck (box girder, 32.6 m span).
const GIRDER_DEPTH: f32 = 2.9;
const SPAN_M: f32 = 32.6;
/// Clearance a road keeps under a structure (CJJ 37: 4.5 m, plus allowance).
pub const ROAD_CLEARANCE: f32 = 5.0;

const CONCRETE: [f32; 3] = [0.70, 0.69, 0.66];
const CONCRETE_SHADE: [f32; 3] = [0.48, 0.47, 0.45];
const EARTH: [f32; 3] = [0.42, 0.37, 0.28];

fn declare(b: &mut MeshBuilder) {
    for material in [
        "viaduct.concrete",
        "station.wall",
        "station.roof",
        "station.glass",
        "rail.platform",
        "train.body",
        "train.glass",
        "train.stripe",
        "barrier.concrete",
        "steel",
        "wire",
        "rail.steel",
    ] {
        b.style(
            material,
            GroupStyle {
                cast_shadow: true,
                receive_shadow: true,
                alpha_cutout: false,
                dynamic: false,
            },
        );
    }
}

fn smooth(x: f32) -> f32 {
    let t = x.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// A quad wound so that it faces `toward`, whatever order the corners came in.
fn quad_facing(
    b: &mut MeshBuilder,
    material: &str,
    p: [Vec3; 4],
    toward: Vec3,
    color: Option<[f32; 3]>,
) {
    let n = face_normal(p[0], p[1], p[2]);
    let dot = n.x * toward.x + n.y * toward.y + n.z * toward.z;
    if dot >= 0.0 {
        b.quad(material, p[0], p[1], p[2], p[3], color);
    } else {
        b.quad(material, p[0], p[3], p[2], p[1], color);
    }
}

const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);

/// The corridor in city-local metres: a path whose `y` is the rail head level
/// above the ground, plus per-vertex ground kind.
struct Line {
    path: Path,
    kinds: Vec<RailGround>,
    s0: f32,
    /// Path station of each vertex.
    stations: Vec<f32>,
}

impl Line {
    fn new(city: &ModernCity, corridor: &RailCorridor) -> Option<Self> {
        if corridor.vertices.len() < 2 {
            return None;
        }
        let s0 = corridor.vertices[0].s_m;
        let pts: Vec<Vec3> = corridor
            .vertices
            .iter()
            .map(|v| {
                let [x, z] = city.frame.to_local(v.point);
                // Cuttings and tunnels are drawn at grade: the town's ground is
                // a flat pad, with nothing to cut into.
                let above = match v.ground {
                    RailGround::Cutting | RailGround::Tunnel => 0.0,
                    _ => v.above_ground_m.max(0.0),
                };
                Vec3::new(x, above + HEAD_ABOVE_GROUND, z)
            })
            .collect();
        let path = Path::new(pts);
        if path.is_empty() {
            return None;
        }
        let stations = path.stations().to_vec();
        Some(Self {
            path,
            kinds: corridor.vertices.iter().map(|v| v.ground).collect(),
            s0,
            stations,
        })
    }

    fn len(&self) -> f32 {
        self.path.length()
    }

    fn kind_at(&self, s: f32) -> RailGround {
        let i = self
            .stations
            .partition_point(|st| *st <= s)
            .saturating_sub(1)
            .min(self.kinds.len() - 1);
        self.kinds[i]
    }

    /// Height of the rail head above the ground at `s`.
    fn head_at(&self, s: f32) -> f32 {
        self.path.sample(s).0.y
    }

    fn elevated_at(&self, s: f32) -> bool {
        self.head_at(s) - HEAD_ABOVE_GROUND >= 4.0
    }

    /// A point `off` metres to the left of the line at station `s` and `lift`
    /// above its rail head. The direction is blended between the segments
    /// either side of a vertex, so offset edges (platforms, ballast) run
    /// smoothly round a curve instead of stepping at every vertex.
    fn point(&self, s: f32, off: f32, lift: f32) -> Vec3 {
        let (p, _) = self.path.sample(s);
        let pts = self.path.points();
        let i = self
            .stations
            .partition_point(|st| *st <= s)
            .saturating_sub(1)
            .min(pts.len() - 2);
        let seg =
            |k: usize| Vec2::new(pts[k + 1].x - pts[k].x, pts[k + 1].z - pts[k].z).normalize();
        let at_vertex = |k: usize| {
            let a = if k > 0 { seg(k - 1) } else { seg(0) };
            let b = if k + 1 < pts.len() {
                seg(k)
            } else {
                seg(pts.len() - 2)
            };
            (a + b).normalize()
        };
        let span = (self.stations[i + 1] - self.stations[i]).max(1.0e-3);
        let t = ((s - self.stations[i]) / span).clamp(0.0, 1.0);
        let d = (at_vertex(i) * (1.0 - t) + at_vertex(i + 1) * t).normalize();
        let n = d.left_normal();
        Vec3::new(p.x + n.x * off, p.y + lift, p.z + n.y * off)
    }
}

struct StationLayout {
    plan: RailStationPlan,
    /// Path station of the platform centre.
    s: f32,
}

/// The offsets (from the line centre) of the tracks through a station.
#[derive(Clone, Copy)]
struct StationTracks {
    through: [f32; 2],
    loops: Option<[f32; 2]>,
    /// Platform strips `(from, to)` across the line.
    platforms: [(f32, f32); 2],
    platform_width: f32,
}

fn station_tracks(plan: &RailStationPlan, limits: &RailLimits) -> StationTracks {
    let sp = limits.track_spacing_m;
    if plan.tracks >= 4 {
        // Two through tracks in the middle, a platform track either side:
        // island platforms between each loop and its through track.
        let (t, l, w) = (sp * 0.5, 18.0, 12.0);
        let e = 1.75;
        StationTracks {
            through: [-t, t],
            loops: Some([-l, l]),
            platforms: [(-(l - e), -(t + e)), (t + e, l - e)],
            platform_width: w,
        }
    } else {
        let e = 1.75;
        let w = 8.0;
        let t = sp * 0.5;
        StationTracks {
            through: [-t, t],
            loops: None,
            platforms: [(-(t + e + w), -(t + e)), (t + e, t + e + w)],
            platform_width: w,
        }
    }
}

/// Weight 1 along the platforms, easing to 0 over the ladder either side.
fn station_weight(s: f32, centre: f32, half: f32) -> f32 {
    const RAMP: f32 = 260.0;
    let d = (s - centre).abs();
    if d <= half {
        1.0
    } else {
        1.0 - smooth((d - half) / RAMP)
    }
}

/// One track: its lateral offset as a function of station, and the range it exists over.
struct Track {
    from: f32,
    to: f32,
    offset: Box<dyn Fn(f32) -> f32>,
    siding: bool,
}

fn tracks_for(
    limits: &RailLimits,
    len: f32,
    stations: &[StationLayout],
    far_sign: impl Fn(&StationLayout) -> f32,
) -> Vec<Track> {
    let sp = limits.track_spacing_m;
    let mut tracks: Vec<Track> = Vec::new();
    // The two main tracks, easing to the through positions at stations.
    for side in [-1.0_f32, 1.0] {
        let layouts: Vec<(f32, f32, f32)> = stations
            .iter()
            .map(|st| {
                let t = station_tracks(&st.plan, limits);
                let target = if side < 0.0 {
                    t.through[0]
                } else {
                    t.through[1]
                };
                (st.s, st.plan.platform_length_m * 0.5, target)
            })
            .collect();
        tracks.push(Track {
            from: 0.0,
            to: len,
            siding: false,
            offset: Box::new(move |s| {
                let mut off = side * sp * 0.5;
                for (c, half, target) in &layouts {
                    let w = station_weight(s, *c, *half);
                    off += (target - side * sp * 0.5) * w;
                }
                off
            }),
        });
    }
    for st in stations {
        let t = station_tracks(&st.plan, limits);
        let half = st.plan.platform_length_m * 0.5;
        let reach = half + 260.0;
        if let Some(loops) = t.loops {
            for (k, side) in [-1.0_f32, 1.0].into_iter().enumerate() {
                let c = st.s;
                let target = loops[k];
                tracks.push(Track {
                    from: (c - reach).max(0.0),
                    to: (c + reach).min(len),
                    siding: false,
                    offset: Box::new(move |s| {
                        let w = station_weight(s, c, half);
                        let main = side * sp * 0.5;
                        main + (target - main) * w
                    }),
                });
            }
        }
        if st.plan.yard {
            // Freight yard behind the station, on the side away from the town.
            let far = far_sign(st);
            let c = st.s - 60.0;
            let yard_half = 280.0;
            for i in 0..5 {
                let target = far * (34.0 + 5.0 * i as f32);
                let main = far * sp * 0.5;
                tracks.push(Track {
                    from: (c - yard_half - 260.0).max(0.0),
                    to: (c + yard_half + 260.0).min(len),
                    siding: true,
                    offset: Box::new(move |s| {
                        let w = station_weight(s, c, yard_half);
                        main + (target - main) * w
                    }),
                });
            }
        }
    }
    tracks
}

pub fn build(city: &ModernCity, network: &Network, b: &mut MeshBuilder) -> RailStats {
    let mut stats = RailStats::default();
    if city.rail.is_empty() {
        return stats;
    }
    declare(b);
    for corridor in &city.rail {
        build_corridor(city, network, corridor, b, &mut stats);
    }
    stats
}

/// A crossing of the line with a street of the plan.
struct Crossing {
    s: f32,
    /// Direction of the street at the crossing (plan unit).
    street_dir: Vec2,
    half_width: f32,
    road_y: f32,
    centre: Vec2,
}

fn segment_hit(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> Option<(f32, f32)> {
    let r = b - a;
    let s = d - c;
    let den = r.x * s.y - r.y * s.x;
    if den.abs() < 1.0e-9 {
        return None;
    }
    let q = c - a;
    let t = (q.x * s.y - q.y * s.x) / den;
    let u = (q.x * r.y - q.y * r.x) / den;
    ((0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u)).then_some((t, u))
}

fn find_crossings(line: &Line, network: &Network) -> Vec<Crossing> {
    let pts = line.path.points();
    let mut out: Vec<Crossing> = Vec::new();
    for road in &network.roads {
        if road.bridge || road.layer != 0 {
            continue;
        }
        let rp = road.carriageway.points();
        let rs = road.carriageway.stations();
        for w in 0..rp.len().saturating_sub(1) {
            let (c, d) = (
                Vec2::new(rp[w].x, rp[w].z),
                Vec2::new(rp[w + 1].x, rp[w + 1].z),
            );
            for i in 0..pts.len() - 1 {
                let (a, bb) = (
                    Vec2::new(pts[i].x, pts[i].z),
                    Vec2::new(pts[i + 1].x, pts[i + 1].z),
                );
                if a.x.max(bb.x) < c.x.min(d.x) - 1.0
                    || a.x.min(bb.x) > c.x.max(d.x) + 1.0
                    || a.y.max(bb.y) < c.y.min(d.y) - 1.0
                    || a.y.min(bb.y) > c.y.max(d.y) + 1.0
                {
                    continue;
                }
                if let Some((t, u)) = segment_hit(a, bb, c, d) {
                    let s = line.stations[i] + (line.stations[i + 1] - line.stations[i]) * t;
                    let at = rs[w] + (rs[w + 1] - rs[w]) * u;
                    let centre = a + (bb - a) * t;
                    if out
                        .iter()
                        .any(|x| (x.s - s).abs() < 6.0 && x.centre.distance(centre) < 12.0)
                    {
                        continue;
                    }
                    out.push(Crossing {
                        s,
                        street_dir: (d - c).normalize(),
                        half_width: road.section.half_carriageway() + 3.0,
                        road_y: road.carriageway.sample(at).0.y,
                        centre,
                    });
                }
            }
        }
    }
    out.sort_by(|a, b| a.s.total_cmp(&b.s));
    out
}

fn along_half_extent(c: &Crossing, line: &Line) -> f32 {
    let tangent = line.path.tangent_at(c.s);
    let sin = (tangent.x * c.street_dir.y - tangent.y * c.street_dir.x)
        .abs()
        .max(0.35);
    (c.half_width + 1.5) / sin
}

#[allow(clippy::too_many_lines)]
fn build_corridor(
    city: &ModernCity,
    network: &Network,
    corridor: &RailCorridor,
    b: &mut MeshBuilder,
    stats: &mut RailStats,
) {
    let Some(line) = Line::new(city, corridor) else {
        return;
    };
    let limits = corridor.kind.limits();
    let len = line.len();
    stats.corridors += 1;
    stats.length_m += len;
    let mut rng = Rng::new(city.seed ^ corridor.railway_id.wrapping_mul(0x9e37_79b1));

    let stations: Vec<StationLayout> = corridor
        .stations
        .iter()
        .map(|p| StationLayout {
            plan: *p,
            s: p.s_m - line.s0,
        })
        .filter(|st| st.s > -200.0 && st.s < len + 200.0)
        .collect();
    stats.stations += stations.len();
    let tracks = tracks_for(&limits, len, &stations, |st| {
        -f32::from(st.plan.building_side)
    });

    // --- crossings with the streets ---
    let crossings = find_crossings(&line, network);
    let mut level: Vec<(f32, f32, usize)> = Vec::new(); // from, to, crossing index
    for (i, c) in crossings.iter().enumerate() {
        let head = line.head_at(c.s) - HEAD_ABOVE_GROUND - (c.road_y);
        let half = along_half_extent(c, &line);
        if head >= ROAD_CLEARANCE {
            stats.overpasses += 1;
        } else if head <= 0.9 {
            if limits.closed {
                stats.warnings.push(format!(
                    "rail: closed line meets a street at grade at {:.0} m",
                    c.s
                ));
            }
            level.push((c.s - half, c.s + half, i));
            stats.level_crossings += 1;
        } else {
            stats.warnings.push(format!(
                "rail: line {:.1} m over a street at {:.0} m, below the {:.1} m clearance",
                head, c.s, ROAD_CLEARANCE
            ));
        }
    }
    let in_crossing = |s: f32| level.iter().any(|(a, bb, _)| s >= *a && s <= *bb);
    let near_crossing = |s: f32, extra: f32| {
        crossings
            .iter()
            .any(|c| (s - c.s).abs() < along_half_extent(c, &line) + extra)
    };

    // --- formation, structure and track ---
    let step = 10.0;
    let n_steps = (len / step).ceil().max(1.0) as usize;
    let formation_half = limits.formation_width_m * 0.5;
    let bed_half = limits.ballast_bed_width_m * 0.5;
    let main_half = limits.track_spacing_m * 0.5 + bed_half + 0.35;
    for k in 0..n_steps {
        let s0 = k as f32 * step;
        let s1 = ((k + 1) as f32 * step).min(len);
        let mid = (s0 + s1) * 0.5;
        let ground = line.kind_at(mid);
        let elevated = line.elevated_at(mid);
        let head = line.head_at(mid) - HEAD_ABOVE_GROUND;
        let bed_lift = -BED_BELOW_HEAD;
        // Extent of the station here: tracks spread out.
        let station_here = stations
            .iter()
            .find(|st| station_weight(mid, st.s, st.plan.platform_length_m * 0.5) > 0.0);
        let spread = station_here.map_or(0.0, |st| {
            let t = station_tracks(&st.plan, &limits);
            let w = station_weight(mid, st.s, st.plan.platform_length_m * 0.5);
            let reach = t.loops.map_or(t.platforms[1].1, |l| l[1] + 3.0);
            w * (reach - (limits.track_spacing_m * 0.5 + 1.0)).max(0.0)
        });
        let bed_outer = main_half + spread;
        let skip_bed = in_crossing(mid);

        if elevated || matches!(ground, RailGround::Viaduct | RailGround::Bridge) && head > 2.0 {
            // Box girder viaduct.
            let deck_half = formation_half - 0.4 + spread * 0.5;
            let web_half = deck_half * 0.52;
            let deck_top = -0.50 + 0.0;
            let soffit = deck_top - GIRDER_DEPTH;
            // Ballast trough floor / slab.
            let (a0, a1) = (
                line.point(s0, -deck_half, deck_top),
                line.point(s0, deck_half, deck_top),
            );
            let (c0, c1) = (
                line.point(s1, -deck_half, deck_top),
                line.point(s1, deck_half, deck_top),
            );
            quad_facing(b, "viaduct.concrete", [a0, a1, c1, c0], UP, Some(CONCRETE));
            // Fascia, web and soffit on both sides.
            for side in [-1.0_f32, 1.0] {
                let out = Vec3::new(0.0, 0.0, 0.0);
                let _ = out;
                let p = |s: f32, off: f32, lift: f32| line.point(s, side * off, lift);
                let left = line.path.tangent_at(mid).left_normal();
                let outward = Vec3::new(left.x * side, 0.0, left.y * side);
                // Fascia (vertical 0.5 m deck edge).
                quad_facing(
                    b,
                    "viaduct.concrete",
                    [
                        p(s0, deck_half, deck_top),
                        p(s1, deck_half, deck_top),
                        p(s1, deck_half, deck_top - 0.55),
                        p(s0, deck_half, deck_top - 0.55),
                    ],
                    outward,
                    Some(CONCRETE),
                );
                // Web: slopes in to the soffit.
                quad_facing(
                    b,
                    "viaduct.concrete",
                    [
                        p(s0, deck_half, deck_top - 0.55),
                        p(s1, deck_half, deck_top - 0.55),
                        p(s1, web_half, soffit),
                        p(s0, web_half, soffit),
                    ],
                    Vec3::new(outward.x, -0.7, outward.z),
                    Some(CONCRETE_SHADE),
                );
                // Deck underside overhang.
                // Parapet / cable trough wall and noise barrier base.
                quad_facing(
                    b,
                    "barrier.concrete",
                    [
                        p(s0, deck_half - 0.15, deck_top),
                        p(s1, deck_half - 0.15, deck_top),
                        p(s1, deck_half - 0.15, deck_top + 1.15),
                        p(s0, deck_half - 0.15, deck_top + 1.15),
                    ],
                    outward,
                    None,
                );
                quad_facing(
                    b,
                    "barrier.concrete",
                    [
                        p(s0, deck_half - 0.15, deck_top + 1.15),
                        p(s1, deck_half - 0.15, deck_top + 1.15),
                        p(s1, deck_half - 0.55, deck_top + 1.15),
                        p(s0, deck_half - 0.55, deck_top + 1.15),
                    ],
                    UP,
                    None,
                );
                quad_facing(
                    b,
                    "barrier.concrete",
                    [
                        p(s0, deck_half - 0.55, deck_top),
                        p(s1, deck_half - 0.55, deck_top),
                        p(s1, deck_half - 0.55, deck_top + 1.15),
                        p(s0, deck_half - 0.55, deck_top + 1.15),
                    ],
                    Vec3::new(-outward.x, 0.0, -outward.z),
                    None,
                );
            }
            // Soffit.
            quad_facing(
                b,
                "viaduct.concrete",
                [
                    line.point(s0, -web_half, soffit),
                    line.point(s1, -web_half, soffit),
                    line.point(s1, web_half, soffit),
                    line.point(s0, web_half, soffit),
                ],
                Vec3::new(0.0, -1.0, 0.0),
                Some(CONCRETE_SHADE),
            );
            // Ballast on conventional viaducts, slab track on high speed.
            if !skip_bed && !limits.closed {
                strip(
                    b,
                    "rail.ballast",
                    &line,
                    s0,
                    s1,
                    -bed_outer,
                    bed_outer,
                    bed_lift,
                    None,
                );
            }
            // Piers at span centres where no street runs.
            let span = (mid / SPAN_M).floor();
            let pier_s = (span + 0.5) * SPAN_M;
            if pier_s >= s0 && pier_s < s1 && !near_crossing(pier_s, 4.0) && !in_crossing(pier_s) {
                let (pos, tan) = line.path.sample(pier_s);
                let angle = tan.y.atan2(tan.x);
                let top = pos.y + deck_top - GIRDER_DEPTH;
                if top > 1.0 {
                    box_at(
                        b,
                        "viaduct.concrete",
                        Vec2::new(pos.x, pos.z),
                        top * 0.5,
                        2.6,
                        top,
                        6.2,
                        angle,
                    );
                    box_at(
                        b,
                        "viaduct.concrete",
                        Vec2::new(pos.x, pos.z),
                        top - 0.4,
                        3.0,
                        0.8,
                        7.4,
                        angle,
                    );
                    stats.piers += 1;
                }
            }
        } else {
            // Embankment or at grade: formation, shoulders, ballast bed.
            let top = -0.74;
            let slope_drop = (head + HEAD_ABOVE_GROUND + top).max(0.0);
            if !skip_bed {
                // The earthwork: sides fall at 1:1.5 to the ground.
                for side in [-1.0_f32, 1.0] {
                    let w = slope_drop * 1.5;
                    let p = |s: f32, off: f32, lift: f32| line.point(s, side * off, lift);
                    quad_facing(
                        b,
                        "bank.earth",
                        [
                            p(s0, formation_half + spread, top + 0.75),
                            p(s1, formation_half + spread, top + 0.75),
                            p(
                                s1,
                                formation_half + spread + w + 0.6,
                                -(head + HEAD_ABOVE_GROUND) - 0.05,
                            ),
                            p(
                                s0,
                                formation_half + spread + w + 0.6,
                                -(head + HEAD_ABOVE_GROUND) - 0.05,
                            ),
                        ],
                        UP,
                        Some(EARTH),
                    );
                    // Ballast shoulder.
                    quad_facing(
                        b,
                        "rail.ballast",
                        [
                            p(s0, bed_outer, bed_lift),
                            p(s1, bed_outer, bed_lift),
                            p(s1, bed_outer + 0.9, bed_lift - 0.45),
                            p(s0, bed_outer + 0.9, bed_lift - 0.45),
                        ],
                        UP,
                        None,
                    );
                }
                // Formation top between the shoulders.
                strip(
                    b,
                    "bank.earth",
                    &line,
                    s0,
                    s1,
                    -(formation_half + spread),
                    formation_half + spread,
                    bed_lift - 0.5,
                    Some(EARTH),
                );
                strip(
                    b,
                    "rail.ballast",
                    &line,
                    s0,
                    s1,
                    -bed_outer,
                    bed_outer,
                    bed_lift,
                    None,
                );
            }
            let _ = ground;
        }
    }

    // --- sleepers, rails per track ---
    for track in &tracks {
        let (t0, t1) = (track.from.max(0.0), track.to.min(len));
        let mut s = t0;
        // Sleepers every 0.6 m (1 667 per km).
        let ballasted = !limits.closed
            || !matches!(
                line.kind_at((t0 + t1) * 0.5),
                RailGround::Viaduct | RailGround::Bridge
            );
        while s < t1 {
            if !in_crossing(s) {
                let on_slab = limits.closed && line.elevated_at(s);
                let off = (track.offset)(s);
                if on_slab {
                    // Continuous slab, laid in 5 m panels.
                    if ((s / 0.6) as i64) % 8 == 0 {
                        let e = (s + 4.8).min(t1);
                        slab(b, &line, s, e, off, track);
                    }
                } else if ballasted || !on_slab {
                    sleeper(b, &line, s, off);
                    stats.sleepers += 1;
                }
            }
            s += 0.6;
        }
        // Rails (two per track), in 8 m runs so curves follow.
        let rail_step = 8.0;
        let mut s = t0;
        while s < t1 {
            let e = (s + rail_step).min(t1);
            for side in [-1.0_f32, 1.0] {
                rail_run(b, &line, s, e, side * GAUGE_CENTRES * 0.5, track);
            }
            s = e;
        }
    }

    // --- catenary ---
    let mast_span = (limits.mast_spacing_m * 0.96).min(limits.mast_spacing_m);
    let n_masts = (len / mast_span).floor() as usize;
    for m in 0..=n_masts {
        let s = m as f32 * mast_span;
        if s > len {
            break;
        }
        if in_crossing(s) {
            continue;
        }
        let spread = stations
            .iter()
            .map(|st| {
                let w = station_weight(s, st.s, st.plan.platform_length_m * 0.5);
                let t = station_tracks(&st.plan, &limits);
                w * t.loops.map_or(t.platforms[1].1, |l| l[1] + 3.0)
            })
            .fold(0.0_f32, f32::max);
        let outer = (limits.track_spacing_m * 0.5 + 3.2).max(spread + 0.5);
        let elevated = line.elevated_at(s);
        for side in [-1.0_f32, 1.0] {
            let base = if elevated {
                -0.5 + 0.0
            } else {
                -(line.head_at(s)) + 0.12
            };
            let off = side
                * if elevated {
                    (limits.formation_width_m * 0.5 - 1.4).min(outer)
                } else {
                    outer
                };
            let pole_base = line.point(s, off, base);
            let pole_top = line.point(s, off, 8.1);
            b.tube("steel", pole_base, pole_top, 0.17, 0.12, 6, None);
            // Cantilever to the nearest main track.
            let arm_to = line.point(s, side * (limits.track_spacing_m * 0.5 + 0.2), 7.2);
            let arm_from = line.point(s, off, 7.2);
            b.tube("steel", arm_from, arm_to, 0.06, 0.05, 4, None);
            let brace_to = line.point(s, side * (limits.track_spacing_m * 0.5 + 1.2), 7.3);
            b.tube(
                "steel",
                line.point(s, off, 6.0),
                brace_to,
                0.04,
                0.035,
                3,
                None,
            );
        }
        stats.masts += 2;
    }
    // Contact and messenger wires over the two main tracks.
    for side in [-1.0_f32, 1.0] {
        let mut s = 0.0;
        while s < len {
            let e = (s + 12.5).min(len);
            if !in_crossing((s + e) * 0.5) {
                let zig = if ((s / 50.0) as i64) % 2 == 0 {
                    0.2
                } else {
                    -0.2
                };
                let o = |s: f32| {
                    let w = tracks[if side < 0.0 { 0 } else { 1 }].offset.as_ref()(s);
                    w + zig
                };
                let cw = limits.contact_wire_m;
                let sag = |s: f32| {
                    let phase = (s % 50.0) / 50.0;
                    0.9 + 1.4 * (1.0 - (2.0 * phase - 1.0).powi(2)) * 0.0
                        - 0.35 * (1.0 - (2.0 * phase - 1.0).powi(2))
                };
                b.tube(
                    "wire",
                    line.point(s, o(s), cw),
                    line.point(e, o(e), cw),
                    0.012,
                    0.012,
                    3,
                    None,
                );
                b.tube(
                    "wire",
                    line.point(s, o(s), cw + sag(s) + 0.3),
                    line.point(e, o(e), cw + sag(e) + 0.3),
                    0.009,
                    0.009,
                    3,
                    None,
                );
            }
            s = e;
        }
    }

    // --- fences along the open line, signals ---
    {
        let mut s = 0.0;
        while s < len {
            let kind = line.kind_at(s);
            let in_station = stations
                .iter()
                .any(|st| (s - st.s).abs() < st.plan.platform_length_m * 0.5 + 280.0);
            if !line.elevated_at(s)
                && !matches!(kind, RailGround::Viaduct | RailGround::Bridge)
                && !in_station
                && !in_crossing(s)
                && !near_crossing(s, 9.0)
            {
                for side in [-1.0_f32, 1.0] {
                    let off = side * (limits.formation_width_m * 0.5 + 2.2);
                    let foot = line.point(s, off, -(line.head_at(s)) + 0.0);
                    let post_top = line.point(s, off, -(line.head_at(s)) + 1.8);
                    b.tube("steel", foot, post_top, 0.04, 0.04, 4, None);
                    let e = (s + 8.0).min(len);
                    for h in [0.5_f32, 1.1, 1.75] {
                        b.tube(
                            "wire",
                            line.point(s, off, -(line.head_at(s)) + h),
                            line.point(e, off, -(line.head_at(e)) + h),
                            0.008,
                            0.008,
                            3,
                            None,
                        );
                    }
                }
            }
            s += 8.0;
        }
    }
    // Colour-light signals every 1.2 km, and a pair at each station throat.
    {
        let mut marks: Vec<f32> = (1..)
            .map(|k| k as f32 * 1200.0)
            .take_while(|s| *s < len)
            .collect();
        for st in &stations {
            marks.push(st.s - st.plan.platform_length_m * 0.5 - 330.0);
            marks.push(st.s + st.plan.platform_length_m * 0.5 + 330.0);
        }
        for s in marks {
            if s <= 0.0 || s >= len || in_crossing(s) {
                continue;
            }
            let side = 1.0;
            let off = side * (limits.track_spacing_m * 0.5 + 2.6);
            let base = if line.elevated_at(s) {
                -0.5 + 0.0
            } else {
                -line.head_at(s) + 0.1
            };
            let foot = line.point(s, off, base);
            let top = line.point(s, off, 4.5);
            b.tube("steel", foot, top, 0.07, 0.07, 5, None);
            let head = line.point(s, off - 0.3, 4.6);
            let tan = line.path.tangent_at(s);
            box_at(
                b,
                "signal.body",
                Vec2::new(head.x, head.z),
                head.y,
                0.32,
                0.95,
                0.32,
                tan.y.atan2(tan.x),
            );
        }
    }

    // --- level crossings ---
    for (from, to, ci) in &level {
        let c = &crossings[*ci];
        let (tm, tm_to) = (0.0_f32, 0.0_f32);
        let _ = (tm, tm_to);
        let off_lo = -(limits.track_spacing_m * 0.5 + 1.7);
        let off_hi = limits.track_spacing_m * 0.5 + 1.7;
        // Road panel flush with the rail heads between the rails.
        let (a, e) = (*from, *to);
        strip(b, "asphalt", &line, a, e, off_lo, off_hi, -0.02, None);
        // Gates, lights and bells on the approaches.
        let sd = c.street_dir;
        let left = sd.left_normal();
        for travel in [-1.0_f32, 1.0] {
            for lane_side in [-1.0_f32, 1.0] {
                let tangent = line.path.tangent_at(c.s);
                let base = c.centre
                    + Vec2::new(tangent.x, tangent.y) * 0.0
                    + sd * travel * (limits.track_spacing_m * 0.5 + 3.4)
                    + left * lane_side * (c.half_width - 1.4);
                if lane_side * travel < 0.0 {
                    continue;
                }
                let y = HEAD_ABOVE_GROUND * 0.0;
                box_at(b, "pole.concrete", base, y + 1.0, 0.25, 2.0, 0.25, 0.0);
                let mast_top = Vec3::new(base.x, 2.0, base.y);
                // Raised gate arm, striped red and white.
                for seg in 0..4 {
                    let t0 = seg as f32 / 4.0;
                    let t1 = (seg + 1) as f32 / 4.0;
                    let dir = -left * lane_side;
                    let arm_len = c.half_width - 0.6;
                    let p0 =
                        mast_top + Vec3::new(dir.x * arm_len * 0.0, 0.0, dir.y * arm_len * 0.0);
                    let tilt = 0.25; // raised: mostly vertical, leaning over the road
                    let a0 = Vec3::new(
                        p0.x + dir.x * arm_len * tilt * t0,
                        p0.y - 0.5 + arm_len * 0.9 * t0,
                        p0.z + dir.y * arm_len * tilt * t0,
                    );
                    let a1 = Vec3::new(
                        p0.x + dir.x * arm_len * tilt * t1,
                        p0.y - 0.5 + arm_len * 0.9 * t1,
                        p0.z + dir.y * arm_len * tilt * t1,
                    );
                    b.tube(
                        if seg % 2 == 0 {
                            "marking.white"
                        } else {
                            "furniture/lantern"
                        },
                        a0,
                        a1,
                        0.05,
                        0.04,
                        4,
                        None,
                    );
                }
                // Twin red lamps.
                box_at(
                    b,
                    "furniture/lantern",
                    base + Vec2::new(0.0, 0.0),
                    2.35,
                    0.34,
                    0.16,
                    0.16,
                    0.0,
                );
            }
        }
    }

    // --- stations ---
    for st in &stations {
        build_station(
            city, corridor, &line, &limits, st, &tracks, b, &mut rng, stats,
        );
    }
}

fn strip(
    b: &mut MeshBuilder,
    material: &str,
    line: &Line,
    s0: f32,
    s1: f32,
    off_a: f32,
    off_b: f32,
    lift: f32,
    color: Option<[f32; 3]>,
) {
    if s1 <= s0 {
        return;
    }
    // Break at every vertex so the strip follows the vertical profile.
    let mut cuts = vec![s0];
    for st in &line.stations {
        if *st > s0 + 0.01 && *st < s1 - 0.01 {
            cuts.push(*st);
        }
    }
    cuts.push(s1);
    for w in cuts.windows(2) {
        let p = [
            line.point(w[0], off_a, lift),
            line.point(w[0], off_b, lift),
            line.point(w[1], off_b, lift),
            line.point(w[1], off_a, lift),
        ];
        quad_facing(
            b,
            material,
            p,
            if off_a < off_b {
                UP
            } else {
                Vec3::new(0.0, -1.0, 0.0)
            },
            color,
        );
    }
}

fn sleeper(b: &mut MeshBuilder, line: &Line, s: f32, off: f32) {
    let top = -RAIL_HEIGHT - 0.01;
    let (hl, hw) = (1.3, 0.13);
    let p = [
        line.point(s - hw, off - hl, top),
        line.point(s - hw, off + hl, top),
        line.point(s + hw, off + hl, top),
        line.point(s + hw, off - hl, top),
    ];
    quad_facing(b, "rail.sleeper", p, UP, None);
}

fn slab(b: &mut MeshBuilder, line: &Line, s0: f32, s1: f32, off: f32, _track: &Track) {
    let top = -RAIL_HEIGHT - 0.04;
    let mut s = s0;
    while s < s1 {
        let e = (s + 4.8).min(s1);
        let p = [
            line.point(s, off - 1.2, top),
            line.point(s, off + 1.2, top),
            line.point(e, off + 1.2, top),
            line.point(e, off - 1.2, top),
        ];
        quad_facing(b, "rail.sleeper", p, UP, None);
        s = e + 0.0;
    }
}

fn rail_run(b: &mut MeshBuilder, line: &Line, s0: f32, s1: f32, rel: f32, track: &Track) {
    let o0 = (track.offset)(s0) + rel;
    let o1 = (track.offset)(s1) + rel;
    let hw = 0.036;
    let top = |s: f32, o: f32, d: f32| line.point(s, o + d, 0.0);
    let low = |s: f32, o: f32, d: f32| line.point(s, o + d, -RAIL_HEIGHT);
    // Head top and two sides.
    quad_facing(
        b,
        "rail.steel",
        [
            top(s0, o0, -hw),
            top(s0, o0, hw),
            top(s1, o1, hw),
            top(s1, o1, -hw),
        ],
        UP,
        None,
    );
    for d in [-hw, hw] {
        let n = line.path.tangent_at((s0 + s1) * 0.5).left_normal();
        let toward = Vec3::new(n.x * d.signum(), 0.0, n.y * d.signum());
        quad_facing(
            b,
            "rail.steel",
            [
                top(s0, o0, d),
                low(s0, o0, d),
                low(s1, o1, d),
                top(s1, o1, d),
            ],
            toward,
            None,
        );
    }
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn build_station(
    city: &ModernCity,
    corridor: &RailCorridor,
    line: &Line,
    limits: &RailLimits,
    st: &StationLayout,
    tracks: &[Track],
    b: &mut MeshBuilder,
    rng: &mut Rng,
    stats: &mut RailStats,
) {
    let _ = city;
    let t = station_tracks(&st.plan, limits);
    let half = st.plan.platform_length_m * 0.5;
    let (s0, s1) = (st.s - half, st.s + half);
    let platform_top = limits.platform_height_m;
    let elevated = line.elevated_at(st.s);
    let base = -line.head_at(st.s) + 0.0; // lift that reaches the ground from the rail head
    // Platforms.
    for (a, c) in t.platforms {
        let (lo, hi) = (a.min(c), a.max(c));
        strip(b, "rail.platform", line, s0, s1, lo, hi, platform_top, None);
        // Edge walls.
        for (edge, toward_sign) in [(lo, -1.0_f32), (hi, 1.0)] {
            let left = line.path.tangent_at(st.s).left_normal();
            let toward = Vec3::new(left.x * toward_sign, 0.0, left.y * toward_sign);
            quad_facing(
                b,
                "rail.platform",
                [
                    line.point(s0, edge, platform_top),
                    line.point(s1, edge, platform_top),
                    line.point(s1, edge, -0.6),
                    line.point(s0, edge, -0.6),
                ],
                toward,
                None,
            );
        }
        // Tactile safety strip 0.8 m from the track-side edge.
        let (e0, e1) = (lo + 0.05, lo + 0.65);
        let (f0, f1) = (hi - 0.65, hi - 0.05);
        strip(
            b,
            "marking.yellow",
            line,
            s0,
            s1,
            e0,
            e1,
            platform_top + 0.012,
            None,
        );
        strip(
            b,
            "marking.yellow",
            line,
            s0,
            s1,
            f0,
            f1,
            platform_top + 0.012,
            None,
        );
        // Canopy: columns every 14 m and a roof 70 % of the platform length.
        let c_mid = (lo + hi) * 0.5;
        let roof_half = half * 0.7;
        let roof_w = (hi - lo) * 0.5 + 1.0;
        let roof_lift = platform_top + 5.6;
        strip(
            b,
            "station.roof",
            line,
            st.s - roof_half,
            st.s + roof_half,
            c_mid - roof_w,
            c_mid + roof_w,
            roof_lift,
            None,
        );
        strip(
            b,
            "station.roof",
            line,
            st.s - roof_half,
            st.s + roof_half,
            c_mid + roof_w,
            c_mid - roof_w,
            roof_lift - 0.35,
            None,
        );
        let mut s = st.s - roof_half;
        while s <= st.s + roof_half {
            let foot = line.point(s, c_mid, platform_top);
            let top = line.point(s, c_mid, roof_lift - 0.2);
            b.tube("steel", foot, top, 0.22, 0.2, 6, None);
            s += 14.0;
        }
    }
    // Station building and forecourt on the town's side.
    let side = f32::from(st.plan.building_side);
    let outer = t
        .platforms
        .iter()
        .map(|p| p.0.abs().max(p.1.abs()))
        .fold(0.0_f32, f32::max);
    let (pos, tan) = line.path.sample(st.s);
    let angle = tan.y.atan2(tan.x);
    let left = tan.left_normal();
    let big = st.plan.tracks >= 4;
    let (len_b, depth, height) = if big {
        (rng.range(130.0, 170.0), 38.0, rng.range(15.0, 19.0))
    } else {
        (rng.range(50.0, 70.0), 20.0, rng.range(7.5, 9.5))
    };
    let across = outer + 20.0 + depth * 0.5;
    let centre = Vec2::new(
        pos.x + left.x * side * across,
        pos.z + left.y * side * across,
    );
    let ground_y = pos.y + base;
    box_at(
        b,
        "station.wall",
        centre,
        ground_y + height * 0.5,
        len_b,
        height,
        depth,
        angle,
    );
    // Roof slab overhanging, glass band wrapped round the middle storeys.
    box_at(
        b,
        "station.roof",
        centre,
        ground_y + height + 0.25,
        len_b + 3.0,
        0.5,
        depth + 3.0,
        angle,
    );
    box_at(
        b,
        "station.glass",
        centre,
        ground_y + height * 0.45,
        len_b + 0.3,
        height * 0.42,
        depth + 0.3,
        angle,
    );
    if elevated {
        // The hall under the tracks: the station is a bridge-type building.
        let hall_h = (pos.y - HEAD_ABOVE_GROUND - 3.8).max(4.0);
        let hall_centre = Vec2::new(pos.x, pos.z);
        box_at(
            b,
            "station.glass",
            hall_centre,
            hall_h * 0.5,
            (len_b * 0.8).min(half * 1.2),
            hall_h,
            outer * 2.0 + 6.0,
            angle,
        );
    }
    // Forecourt plaza: paved, with a car park beyond it.
    let plaza_depth = 70.0;
    let plaza_centre = across + depth * 0.5 + plaza_depth * 0.5;
    let corner = |a: f32, z: f32| {
        Vec2::new(
            pos.x + tan.x * a + left.x * side * z,
            pos.z + tan.y * a + left.y * side * z,
        )
    };
    let hl = len_b * 0.65;
    let ring = vec![
        corner(-hl, plaza_centre - plaza_depth * 0.5),
        corner(hl, plaza_centre - plaza_depth * 0.5),
        corner(hl, plaza_centre + plaza_depth * 0.5),
        corner(-hl, plaza_centre + plaza_depth * 0.5),
    ];
    let mut ring = ring;
    if crate::math::signed_area(&ring) < 0.0 {
        ring.reverse();
    }
    b.fill_ring("parcel.paving", &ring, ground_y + 0.03, None);

    // Freight yard: depot shed and a freight consist on a siding.
    if st.plan.yard {
        let far = -side;
        let shed_across = 34.0 + 5.0 * 5.0 + 22.0;
        let shed_centre = Vec2::new(
            pos.x + left.x * far * shed_across - tan.x * 160.0,
            pos.z + left.y * far * shed_across - tan.y * 160.0,
        );
        box_at(
            b,
            "station.wall",
            shed_centre,
            ground_y + 5.5,
            90.0,
            11.0,
            26.0,
            angle,
        );
        box_at(
            b,
            "station.roof",
            shed_centre,
            ground_y + 11.2,
            92.0,
            0.5,
            28.0,
            angle,
        );
        // A freight train standing on the first siding.
        if let Some(track) = tracks.iter().filter(|t| t.siding).nth(2) {
            freight_train(b, line, track, st.s - 60.0, rng, stats);
        }
    }
    // A passenger train at the platform: on the loop where there is one.
    let train_track = if big {
        tracks
            .iter()
            .filter(|t| !t.siding)
            .nth(2 + (rng.int(2) as usize))
    } else {
        tracks.iter().filter(|t| !t.siding).nth(rng.int(2) as usize)
    };
    if let Some(track) = train_track {
        passenger_train(b, line, corridor.kind, track, st.s, rng, stats);
    }
}

// --- trains -----------------------------------------------------------------

/// Sweep a closed cross-section along a track between two stations. `profile`
/// is the right half `(across, up)` from the underframe to the roof centre; it
/// is mirrored. `nose` shrinks the section over that many metres at each end
/// flagged.
#[allow(clippy::too_many_arguments)]
fn sweep(
    b: &mut MeshBuilder,
    material: &str,
    line: &Line,
    track: &Track,
    s0: f32,
    s1: f32,
    profile: &[(f32, f32)],
    color: [f32; 3],
    nose: (f32, f32),
) {
    let cut = 5.0;
    let n = (((s1 - s0) / cut).ceil() as usize).max(1);
    let section = |s: f32| -> Vec<(f32, f32)> {
        let mut f = 1.0_f32;
        if nose.0 > 0.0 {
            f = f.min(((s - s0) / nose.0).clamp(0.0, 1.0));
        }
        if nose.1 > 0.0 {
            f = f.min(((s1 - s) / nose.1).clamp(0.0, 1.0));
        }
        let shape = f.powf(0.55);
        let scale_x = 0.12 + 0.88 * shape;
        let scale_y = 0.62 + 0.38 * shape;
        let mut pts: Vec<(f32, f32)> = profile
            .iter()
            .map(|(x, y)| (-x * scale_x, y * scale_y))
            .collect();
        pts.reverse();
        pts.extend(profile.iter().map(|(x, y)| (x * scale_x, y * scale_y)));
        pts
    };
    let rows: Vec<Vec<Vec3>> = (0..=n)
        .map(|i| {
            let s = (s0 + (s1 - s0) * i as f32 / n as f32).clamp(0.0, line.len());
            let off = (track.offset)(s);
            section(s0 + (s1 - s0) * i as f32 / n as f32)
                .into_iter()
                .map(|(x, y)| line.point(s, off + x, y + 0.15))
                .collect()
        })
        .collect();
    for i in 0..n {
        for k in 0..rows[i].len() - 1 {
            let p = [
                rows[i][k],
                rows[i + 1][k],
                rows[i + 1][k + 1],
                rows[i][k + 1],
            ];
            // Outward: away from the centre of the section.
            let mid = Vec3::new(
                (p[0].x + p[2].x) * 0.5 - (rows[i][0].x + rows[i][rows[i].len() - 1].x) * 0.5,
                (p[0].y + p[2].y) * 0.5 - (rows[i][0].y + 1.8),
                (p[0].z + p[2].z) * 0.5 - (rows[i][0].z + rows[i][rows[i].len() - 1].z) * 0.5,
            );
            quad_facing(b, material, p, mid, Some(color));
        }
    }
    // End caps.
    for (row, sign) in [(&rows[0], -1.0_f32), (&rows[n], 1.0)] {
        let t = line.path.tangent_at(s0.max(0.0));
        let toward = Vec3::new(t.x * sign, 0.0, t.y * sign);
        let ring: Vec<Vec3> = row.clone();
        let centre = ring.iter().fold(Vec3::default(), |a, p| a + *p) / ring.len() as f32;
        for k in 0..ring.len() - 1 {
            quad_facing(
                b,
                material,
                [centre, ring[k], ring[k + 1], centre],
                toward,
                Some(color),
            );
        }
    }
}

fn side_band(
    b: &mut MeshBuilder,
    material: &str,
    line: &Line,
    track: &Track,
    s0: f32,
    s1: f32,
    x: f32,
    y0: f32,
    y1: f32,
    color: Option<[f32; 3]>,
) {
    for side in [-1.0_f32, 1.0] {
        let left = line.path.tangent_at((s0 + s1) * 0.5).left_normal();
        let toward = Vec3::new(left.x * side, 0.0, left.y * side);
        let p = |s: f32, y: f32| line.point(s, (track.offset)(s) + side * x, y + 0.15);
        quad_facing(
            b,
            material,
            [p(s0, y0), p(s1, y0), p(s1, y1), p(s0, y1)],
            toward,
            color,
        );
    }
}

fn passenger_train(
    b: &mut MeshBuilder,
    line: &Line,
    kind: RailKind,
    track: &Track,
    centre: f32,
    rng: &mut Rng,
    stats: &mut RailStats,
) {
    let hsr = kind == RailKind::HighSpeed;
    let (cars, length, width, roof) = if hsr {
        (8, 25.2, 1.68, 3.75)
    } else {
        (7, 25.5, 1.55, 4.3)
    };
    let total = cars as f32 * length;
    let start = centre - total * 0.5;
    let profile_hsr = [
        (1.2, 0.0),
        (1.68, 0.35),
        (1.68, 2.9),
        (1.45, 3.45),
        (0.85, 3.72),
        (0.0, 3.78),
    ];
    let profile_conv = [(1.4, 0.0), (1.55, 0.3), (1.55, 3.7), (1.3, 4.1), (0.0, 4.3)];
    let profile: Vec<(f32, f32)> = if hsr {
        profile_hsr.to_vec()
    } else {
        profile_conv.to_vec()
    };
    let _ = (width, roof);
    let body: [f32; 3] = if hsr {
        [0.93, 0.94, 0.95]
    } else if rng.chance(0.5) {
        [0.12, 0.40, 0.24]
    } else {
        [0.62, 0.12, 0.10]
    };
    for car in 0..cars {
        let a = start + car as f32 * length + 0.25;
        let e = a + length - 0.5;
        if a < track.from || e > track.to {
            continue;
        }
        let nose = (
            if car == 0 { 9.0 } else { 0.0 },
            if car == cars - 1 { 9.0 } else { 0.0 },
        );
        let colour = if !hsr && car == 0 {
            [0.20, 0.30, 0.55]
        } else {
            body
        };
        sweep(b, "train.body", line, track, a, e, &profile, colour, nose);
        let (wa, we) = (a + nose.0.max(1.0), e - nose.1.max(1.0));
        let wx = if hsr { 1.695 } else { 1.56 };
        if we > wa {
            side_band(b, "train.glass", line, track, wa, we, wx, 1.85, 2.75, None);
            side_band(
                b,
                "train.stripe",
                line,
                track,
                a + nose.0,
                e - nose.1,
                wx + 0.005,
                0.95,
                1.25,
                None,
            );
        }
        stats.train_cars += 1;
    }
}

fn freight_train(
    b: &mut MeshBuilder,
    line: &Line,
    track: &Track,
    centre: f32,
    rng: &mut Rng,
    stats: &mut RailStats,
) {
    let wagons = 12;
    let loco_len = 20.0;
    let wagon_len = 13.0;
    let total = loco_len + wagons as f32 * wagon_len;
    let start = centre - total * 0.5;
    let loco = [(1.5, 0.0), (1.55, 0.4), (1.55, 3.9), (1.2, 4.3), (0.0, 4.3)];
    let wagon = [(1.4, 0.0), (1.45, 0.55), (1.45, 3.4), (0.0, 3.4)];
    if start >= track.from && start + loco_len <= track.to {
        sweep(
            b,
            "train.body",
            line,
            track,
            start,
            start + loco_len - 0.4,
            &loco,
            [0.78, 0.35, 0.10],
            (0.0, 0.0),
        );
        stats.train_cars += 1;
    }
    let palette = [
        [0.38, 0.20, 0.14],
        [0.30, 0.34, 0.40],
        [0.22, 0.30, 0.22],
        [0.45, 0.43, 0.38],
    ];
    for w in 0..wagons {
        let a = start + loco_len + w as f32 * wagon_len + 0.3;
        let e = a + wagon_len - 0.6;
        if a < track.from || e > track.to {
            continue;
        }
        let colour = palette[rng.int(palette.len() as u32) as usize];
        sweep(
            b,
            "train.body",
            line,
            track,
            a,
            e,
            &wagon,
            colour,
            (0.0, 0.0),
        );
        stats.train_cars += 1;
    }
}

/// The station-side point of a world position, for tests.
pub fn corridor_point(city: &ModernCity, p: Point) -> [f32; 2] {
    city.frame.to_local(p)
}
