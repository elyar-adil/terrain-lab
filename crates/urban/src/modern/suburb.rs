//! The edge of a town, and the country beyond it.
//!
//! A real town does not stop. Its blocks of flats give way to low-rise
//! housing, then to detached villas in gardens, then to farmhouses standing
//! alone beside the road, and then to fields. The rows of buildings do not fill
//! the land: they follow the streets, one lot deep, and what lies behind them is
//! garden, orchard or field.
//!
//! So out here lots are not made by cutting a block into pieces. They are laid
//! along the streets that bound a face, one after another, each as wide as a
//! house of that kind needs, each kept or left empty by the same hash that makes
//! the rest of the city stable. How built-up the place is (the settlement field's
//! `urbanness`) decides what kind of house a lot gets and how likely it is to
//! have one at all.

use super::CityFrame;
use super::blocks::Face;
use super::geom::{V, centroid, point_in, point_seg_dist, signed_area};
use super::graph::modern_hash;
use super::parcels::right_of_way;
use crate::{
    BuildingFacade, CityStyle, CropKind, Field, ModernBuilding, ModernRoadClass, Parcel, ParcelUse,
    RoofStyle,
};
use worldgen_core::smoothstep;

/// Built-up level (the settlement field's `urbanness`) at and above which a lot is
/// town: a walk-up block fronting the street.
pub(super) const TOWN_BUILT: f32 = 0.42;
/// Below this, and above `OPEN_BUILT`, a lot is a lone farmhouse, not a villa.
pub(super) const VILLA_BUILT: f32 = 0.15;
/// Below this the land is open country: nothing is built.
pub(super) const OPEN_BUILT: f32 = 0.025;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Zone {
    Town,
    Villa,
    Farm,
    Open,
}

pub(super) fn zone(built: f32) -> Zone {
    if built >= TOWN_BUILT {
        Zone::Town
    } else if built >= VILLA_BUILT {
        Zone::Villa
    } else if built >= OPEN_BUILT {
        Zone::Farm
    } else {
        Zone::Open
    }
}

/// A volume of a house, in the lot's own frame: `s` along the street, `d` away
/// from it.
#[derive(Clone, Copy)]
struct Part {
    s0: f32,
    s1: f32,
    d0: f32,
    d1: f32,
    floors: u16,
    roof: RoofStyle,
}

/// What one lot is going to hold, before it is placed.
struct Plan {
    width: f32,
    depth: f32,
    parts: Vec<Part>,
}

/// A convex quadrilateral's overlap with another, with `gap` metres of room: the
/// separating-axis test on the two rectangles.
fn overlap(a: &[V], b: &[V], gap: f32) -> bool {
    for poly in [a, b] {
        for i in 0..poly.len() {
            let (p, q) = (poly[i], poly[(i + 1) % poly.len()]);
            let len = (q.0 - p.0).hypot(q.1 - p.1);
            if len < 1.0e-4 {
                continue;
            }
            let axis = (-(q.1 - p.1) / len, (q.0 - p.0) / len);
            let project = |r: &[V]| {
                r.iter().fold((f32::MAX, f32::MIN), |(lo, hi), v| {
                    let d = v.0 * axis.0 + v.1 * axis.1;
                    (lo.min(d), hi.max(d))
                })
            };
            let (a0, a1) = project(a);
            let (b0, b1) = project(b);
            if a1 + gap < b0 || b1 + gap < a0 {
                return false;
            }
        }
    }
    true
}

/// Everything a face's frontage needs from the plan around it.
pub(super) struct Frontage<'a> {
    pub frame: &'a CityFrame,
    pub face: &'a Face,
    pub face_index: i32,
    /// The buildable ground of the face (after street setbacks), one polygon per bank.
    pub envelopes: &'a [Vec<V>],
    pub spurs: &'a [(V, V, ModernRoadClass)],
    pub seed: u32,
    pub block_base: u32,
}

pub(super) struct Sink<'a> {
    pub parcels: &'a mut Vec<Parcel>,
    pub buildings: &'a mut Vec<ModernBuilding>,
    pub next_parcel: &'a mut u32,
    pub next_building: &'a mut u32,
    pub fields: &'a mut Vec<Field>,
    pub next_field: &'a mut u32,
}

/// Lay out the houses along the streets of one face.
pub(super) fn place_frontage(f: &Frontage<'_>, sink: &mut Sink<'_>) {
    let debug = std::env::var("SUBURB_DEBUG").is_ok();
    let mut stats = [0_u32; 8];
    let ring = &f.face.ring;
    let mut placed: Vec<Vec<V>> = Vec::new();
    for i in 0..ring.len() {
        if f.face.open[i] {
            continue;
        }
        let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
        let len = f.face.edge_len[i];
        if len < 70.0 {
            stats[0] += 1;
            continue;
        }
        let t = ((b.0 - a.0) / len, (b.1 - a.1) / len);
        let n = (-t.1, t.0);
        let row = right_of_way(f.face.classes[i]);
        let at = |s: f32, d: f32| -> V { (a.0 + t.0 * s + n.0 * d, a.1 + t.1 * s + n.1 * d) };
        let end_clear = 24.0;
        let mut s = end_clear + 12.0 * modern_hash(f.seed, f.face_index, i as i32, 1101);
        let mut k = 0;
        while s < len - end_clear && k < 400 {
            k += 1;
            let key = f.face_index * 4096 + (i as i32) * 256 + k;
            let noise = |salt: i32| modern_hash(f.seed, key, 5, salt);
            // How built-up the ground is a little way in from the street here.
            let probe = at(s + 14.0, row + 18.0);
            let built = f.frame.urbanness(probe.0, probe.1);
            let z = zone(built);
            if z == Zone::Open {
                stats[1] += 1;
                s += 40.0;
                continue;
            }
            let plan = plan_lot(z, built, f.frame.core_weight(probe.0, probe.1), &noise);
            let (w, depth) = (plan.width, plan.depth);
            if s + w > len - end_clear {
                break;
            }
            // A little inside the buildable ground, so the lot is not on its very edge.
            let front = row + 0.5;
            let lot: Vec<V> = vec![
                at(s, front),
                at(s + w, front),
                at(s + w, front + depth),
                at(s, front + depth),
            ];
            let advance = w + 1.5 + 3.0 * noise(1109);
            // Whether this lot has a house on it at all: certain beside the town,
            // rare in the country.
            let keep = match z {
                Zone::Town => 0.96,
                Zone::Villa => 0.50 + 0.42 * smoothstep(VILLA_BUILT, TOWN_BUILT, built),
                _ => 0.10 + 0.55 * smoothstep(OPEN_BUILT, VILLA_BUILT, built),
            };
            if noise(1103) > keep {
                stats[2] += 1;
                s += advance;
                continue;
            }
            let inside = f
                .envelopes
                .iter()
                .any(|env| lot.iter().all(|p| point_in(env, *p)));
            let near_river = f.frame.near_water(&lot, 8.0);
            let clear_of_spurs = f.spurs.iter().all(|(p, q, class)| {
                let clear = right_of_way(*class);
                lot.iter().all(|c| point_seg_dist(*c, *p, *q) >= clear)
            });
            if !inside
                || near_river
                || !clear_of_spurs
                || placed.iter().any(|o| overlap(&lot, o, 1.0))
            {
                stats[3] += (!inside) as u32;
                stats[4] += near_river as u32;
                stats[5] += (!clear_of_spurs) as u32;
                s += advance * 0.5;
                continue;
            }
            stats[6] += 1;
            stats[7] += (z == Zone::Villa) as u32;
            // Orient the ring counter-clockwise (the street side is edge 0).
            let mut ring_out = lot.clone();
            if signed_area(&ring_out) < 0.0 {
                ring_out.reverse();
            }
            let use_type = match z {
                Zone::Town => ParcelUse::Residential,
                Zone::Villa => ParcelUse::Villa,
                _ => ParcelUse::Farmstead,
            };
            let parcel_id = *sink.next_parcel;
            *sink.next_parcel += 1;
            sink.parcels.push(Parcel {
                id: parcel_id,
                block_id: f.block_base,
                ring: ring_out
                    .iter()
                    .map(|p| f.frame.to_world(p.0, p.1))
                    .collect(),
                use_type,
                compound: false,
                gate_edge: 0,
            });
            for part in &plan.parts {
                let rect = vec![
                    at(s + part.s0, front + part.d0),
                    at(s + part.s1, front + part.d0),
                    at(s + part.s1, front + part.d1),
                    at(s + part.s0, front + part.d1),
                ];
                push_building(f.frame, sink, parcel_id, use_type, &rect, part);
            }
            placed.push(lot);
            s += advance;
        }
    }
    place_fields(f, &placed, sink);
    if debug {
        eprintln!(
            "frontage face {}: edges {} short {}, open {}, thinned {}, outside {}, river {}, spur {}, placed {} (villa {})",
            f.face_index,
            ring.len(),
            stats[0],
            stats[1],
            stats[2],
            stats[3],
            stats[4],
            stats[5],
            stats[6],
            stats[7]
        );
    }
}

/// Decide what one lot holds: its size, and the volumes of the house on it.
fn plan_lot(zone: Zone, built: f32, centrality: f32, noise: &dyn Fn(i32) -> f32) -> Plan {
    match zone {
        Zone::Town => {
            // A low-rise street front: one block, flush with the lot.
            let width = 22.0 + 12.0 * noise(1111);
            let depth = 20.0 + 6.0 * noise(1113);
            let floors = (3.0 + 3.0 * noise(1115) + 3.0 * centrality)
                .round()
                .clamp(3.0, 7.0) as u16;
            Plan {
                width,
                depth,
                parts: vec![Part {
                    s0: 1.0,
                    s1: width - 1.0,
                    d0: 1.0,
                    d1: depth - 1.0,
                    floors,
                    roof: if noise(1117) < 0.4 {
                        RoofStyle::Hip
                    } else {
                        RoofStyle::Flat
                    },
                }],
            }
        }
        Zone::Villa => villa(built, noise),
        _ => farmhouse(noise),
    }
}

fn roof_for(roll: f32, hip: f32, gable: f32) -> RoofStyle {
    if roll < hip {
        RoofStyle::Hip
    } else if roll < hip + gable {
        RoofStyle::Gable
    } else {
        RoofStyle::Flat
    }
}

/// A detached house in its garden: a main block, often a lower wing, sometimes a
/// garage, set back from the street behind a front garden.
fn villa(built: f32, noise: &dyn Fn(i32) -> f32) -> Plan {
    // Plots are larger the further out: the town edge is tight, the country club
    // end is generous.
    let roomy = 1.0 - smoothstep(VILLA_BUILT, TOWN_BUILT, built);
    let width = 24.0 + 14.0 * noise(1121) + 8.0 * roomy;
    let depth = 32.0 + 12.0 * noise(1123) + 8.0 * roomy;
    let mw = (11.0 + 6.0 * noise(1125)).min(width - 9.0);
    let md = 9.0 + 4.0 * noise(1127);
    let floors = if noise(1129) < 0.55 { 2 } else { 3 };
    let roof = roof_for(noise(1131), 0.48, 0.38);
    let front = 4.0 + 5.0 * noise(1133);
    let wing_side_right = noise(1135) < 0.5;
    let room_right = width - 0.5 * (width - mw) - mw;
    // The main block sits off-centre toward the side without the wing.
    let slack = width - mw;
    let (s0, wing) = if noise(1137) < 0.52 {
        // A wing on one side takes up that side's slack.
        let ww = (4.5 + 3.5 * noise(1139)).min(slack - 2.0).max(0.0);
        if ww >= 4.0 {
            let s0 = if wing_side_right {
                1.0 + 0.2 * (slack - ww)
            } else {
                width - 1.0 - mw - 0.2 * (slack - ww)
            };
            let _ = room_right;
            (s0.clamp(1.0, width - mw - 1.0), Some((wing_side_right, ww)))
        } else {
            (0.5 * slack, None)
        }
    } else {
        (0.5 * slack + (noise(1141) - 0.5) * 0.4 * slack, None)
    };
    let main = Part {
        s0,
        s1: s0 + mw,
        d0: front,
        d1: front + md,
        floors,
        roof,
    };
    let mut parts = vec![main];
    if let Some((right, ww)) = wing {
        let wd = (md - 1.0 - 2.0 * noise(1143)).max(5.0);
        let (ws0, ws1) = if right {
            (main.s1, (main.s1 + ww).min(width - 0.8))
        } else {
            ((main.s0 - ww).max(0.8), main.s0)
        };
        parts.push(Part {
            s0: ws0,
            s1: ws1,
            d0: front + 0.5 + 1.5 * noise(1145),
            d1: front + 0.5 + 1.5 * noise(1145) + wd,
            floors: floors.saturating_sub(1).max(1),
            roof: if roof == RoofStyle::Flat {
                RoofStyle::Flat
            } else {
                RoofStyle::Gable
            },
        });
    } else if noise(1147) < 0.4 {
        // A garage beside the house, flat-roofed and forward of it.
        let gw = 3.4;
        let on_right = main.s1 + gw + 0.8 < width;
        let (gs0, gs1) = if on_right {
            (main.s1 + 0.2, main.s1 + 0.2 + gw)
        } else {
            (main.s0 - 0.2 - gw, main.s0 - 0.2)
        };
        if gs0 > 0.6 && gs1 < width - 0.6 {
            parts.push(Part {
                s0: gs0,
                s1: gs1,
                d0: front - 0.5,
                d1: front + 6.0,
                floors: 1,
                roof: RoofStyle::Flat,
            });
        }
    }
    Plan {
        width,
        depth,
        parts,
    }
}

/// A farmhouse and a shed or two, in a big yard.
fn farmhouse(noise: &dyn Fn(i32) -> f32) -> Plan {
    let width = 46.0 + 40.0 * noise(1151);
    let depth = 38.0 + 22.0 * noise(1153);
    let mw = 11.0 + 4.0 * noise(1155);
    let md = 7.0 + 2.5 * noise(1157);
    let floors = if noise(1159) < 0.55 { 1 } else { 2 };
    let front = 7.0 + 8.0 * noise(1161);
    let s0 = 4.0 + (width - mw - 8.0).max(0.0) * noise(1163) * 0.5;
    let roof = if noise(1165) < 0.8 {
        RoofStyle::Gable
    } else {
        RoofStyle::Hip
    };
    let main = Part {
        s0,
        s1: s0 + mw,
        d0: front,
        d1: front + md,
        floors,
        roof,
    };
    let mut parts = vec![main];
    // Sheds: a store house to the side and, sometimes, a pen behind.
    let sw = 6.0 + 4.0 * noise(1167);
    let sd = 4.0 + 2.0 * noise(1169);
    let gap = 5.0 + 4.0 * noise(1171);
    if main.s1 + gap + sw < width - 1.0 {
        parts.push(Part {
            s0: main.s1 + gap,
            s1: main.s1 + gap + sw,
            d0: front + 1.0 + 3.0 * noise(1173),
            d1: front + 1.0 + 3.0 * noise(1173) + sd,
            floors: 1,
            roof: if noise(1175) < 0.7 {
                RoofStyle::Gable
            } else {
                RoofStyle::Flat
            },
        });
    }
    if noise(1177) < 0.45 && front + md + 8.0 + 5.0 < depth {
        let bs0 = main.s0 + 2.0 * noise(1179);
        parts.push(Part {
            s0: bs0,
            s1: bs0 + 7.0,
            d0: front + md + 6.0,
            d1: front + md + 6.0 + 4.5,
            floors: 1,
            roof: RoofStyle::Gable,
        });
    }
    Plan {
        width,
        depth,
        parts,
    }
}

/// The facade of a suburban house. A Chinese dwelling is rendered and painted, not
/// brick; a farmhouse keeps the brick it was built of.  `WesternMixed` stays brick.
fn suburb_facade(frame: &CityFrame, use_type: ParcelUse) -> BuildingFacade {
    let plaster = frame.spec.style == CityStyle::ChineseModern
        && matches!(use_type, ParcelUse::Residential | ParcelUse::Villa);
    if plaster {
        BuildingFacade::PlasterResidential
    } else {
        BuildingFacade::BrickResidential
    }
}

/// Add one volume as a building.
fn push_building(
    frame: &CityFrame,
    sink: &mut Sink<'_>,
    parcel_id: u32,
    use_type: ParcelUse,
    rect: &[V],
    part: &Part,
) {
    let (w, d) = (
        ((rect[1].0 - rect[0].0).hypot(rect[1].1 - rect[0].1)),
        ((rect[3].0 - rect[0].0).hypot(rect[3].1 - rect[0].1)),
    );
    let storey = 3.0;
    let floors = part.floors.max(1);
    let id = *sink.next_building;
    *sink.next_building += 1;
    let long = w.max(d);
    sink.buildings.push(ModernBuilding {
        id,
        parcel_id,
        footprint: rect.iter().map(|p| frame.to_world(p.0, p.1)).collect(),
        height_metres: floors as f32 * storey + 0.6,
        floors,
        use_type,
        roof: part.roof,
        facade: suburb_facade(frame, use_type),
        podium_height_metres: 0.0,
        window_bays: ((long / 3.4).round() as u16).clamp(2, 12),
        balcony_bays: if floors >= 2 && use_type == ParcelUse::Villa {
            1
        } else {
            0
        },
        entrance_count: 1,
    });
    let _ = centroid;
}

/// The crop of one plot: the neighbourhood's own (`preferred`) when `follows`, else
/// a household's choice `own`. Rape is grown as a field system, its flowers a sheet
/// of yellow across a village's land; a household alone does not sow it, so a lone
/// choice of rape is a plot of vegetables instead.
fn household_crop(choices: &[CropKind], preferred: f32, follows: bool, own: f32) -> CropKind {
    let at = |t: f32| choices[((t * choices.len() as f32) as usize).min(choices.len() - 1)];
    let crop = at(if follows { preferred } else { own });
    if !follows && crop == CropKind::Rapeseed && at(preferred) != CropKind::Rapeseed {
        CropKind::Vegetables
    } else {
        crop
    }
}

/// The crops a place grows, by how far out it is. At the edge of town: market
/// gardens, orchards, a little rape and wheat; further out the plain's own mix.
fn crop_choices(zone: Zone) -> &'static [CropKind] {
    match zone {
        Zone::Villa | Zone::Town => &[
            CropKind::Vegetables,
            CropKind::Orchard,
            CropKind::Fallow,
            CropKind::Vegetables,
            CropKind::Wheat,
            CropKind::Rapeseed,
        ],
        _ => &[
            CropKind::Wheat,
            CropKind::Rice,
            CropKind::Corn,
            CropKind::Rapeseed,
            CropKind::Wheat,
            CropKind::Vegetables,
            CropKind::Rice,
            CropKind::Fallow,
        ],
    }
}

/// Farmland on what the houses leave of a face: long strips laid parallel to its
/// longest street, as the field systems of the plain are, each a few tens of metres
/// wide and up to a hundred and fifty long, a ditch-width apart. A neighbourhood
/// tends to grow one crop (a face has its own preference), with plots of something
/// else among them.
pub(super) fn place_fields(f: &Frontage<'_>, lots: &[Vec<V>], sink: &mut Sink<'_>) {
    let ring = &f.face.ring;
    let face_key = f.face_index;
    let face = |salt: i32| modern_hash(f.seed, face_key, 7, salt);
    // The strips run along the longest street.
    let (mut best, mut dir) = (0.0_f32, (1.0_f32, 0.0_f32));
    for i in 0..ring.len() {
        if !f.face.open[i] && f.face.edge_len[i] > best {
            best = f.face.edge_len[i];
            let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
            dir = ((b.0 - a.0) / best, (b.1 - a.1) / best);
        }
    }
    if face(1201) < 0.3 {
        dir = (-dir.1, dir.0);
    }
    let u = dir;
    let v = (-dir.1, dir.0);
    let angle = u.1.atan2(u.0);
    let width = 20.0 + 26.0 * face(1203);
    let long = 80.0 + 80.0 * face(1205);
    let preferred = face(1207);
    let mut made = 0;
    for env in f.envelopes {
        let (mut u0, mut u1, mut v0, mut v1) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
        for p in env {
            let (pu, pv) = (p.0 * u.0 + p.1 * u.1, p.0 * v.0 + p.1 * v.1);
            u0 = u0.min(pu);
            u1 = u1.max(pu);
            v0 = v0.min(pv);
            v1 = v1.max(pv);
        }
        let world = |pu: f32, pv: f32| -> V { (u.0 * pu + v.0 * pv, u.1 * pu + v.1 * pv) };
        let mut row = 0;
        let mut pv = v0 + 2.0;
        while pv + width * 0.5 <= v1 && made < 90 {
            row += 1;
            let mut pu = u0 + 2.0 + long * 0.7 * modern_hash(f.seed, face_key, row, 1209);
            let mut col = 0;
            while pu + 24.0 < u1 && made < 90 {
                col += 1;
                let key = face_key * 8192 + row * 64 + col;
                let noise = |salt: i32| modern_hash(f.seed, key, 9, salt);
                let mut placed = false;
                for frac in [1.0_f32, 0.62, 0.38] {
                    let len = (long * frac * (0.75 + 0.5 * noise(1211))).min(u1 - pu - 1.0);
                    if len < 22.0 {
                        continue;
                    }
                    let w = width * (0.8 + 0.4 * noise(1213));
                    let corners = vec![
                        world(pu, pv),
                        world(pu + len, pv),
                        world(pu + len, pv + w),
                        world(pu, pv + w),
                    ];
                    if !f
                        .envelopes
                        .iter()
                        .any(|e| corners.iter().all(|c| point_in(e, *c)))
                    {
                        continue;
                    }
                    if lots.iter().any(|l| overlap(&corners, l, 4.0)) {
                        continue;
                    }
                    if f.frame.near_water(&corners, 6.0) {
                        continue;
                    }
                    let c = centroid(&corners);
                    let built = f.frame.urbanness(c.0, c.1);
                    if built >= TOWN_BUILT {
                        continue;
                    }
                    let choices = crop_choices(zone(built));
                    let crop = household_crop(choices, preferred, noise(1215) < 0.66, noise(1217));
                    sink.fields.push(Field {
                        id: *sink.next_field,
                        ring: corners.iter().map(|p| f.frame.to_world(p.0, p.1)).collect(),
                        crop,
                        row_angle: angle,
                        variant: (noise(1219) * 3.0) as u8,
                    });
                    *sink.next_field += 1;
                    made += 1;
                    pu += len + 1.6;
                    placed = true;
                    break;
                }
                if !placed {
                    pu += 18.0;
                }
            }
            pv += width + 1.6;
        }
    }
}

/// Farmland along every country street, on both sides, up to three strips deep. Open
/// country has no closed blocks to cut up (the streets thin out and stop), but it has
/// its lanes, and farmland is exactly what lies along them. Every strip is checked
/// against the houses and fields already placed and against all other streets.
pub(super) fn roadside_fields(
    frame: &CityFrame,
    pts: &[V],
    edges: &[(usize, usize, ModernRoadClass)],
    _river: &[V],
    _river_half: f32,
    seed: u32,
    occupied: &mut Vec<Vec<V>>,
    fields: &mut Vec<Field>,
    next_field: &mut u32,
) {
    let mut count = 0;
    for (ei, &(ia, ib, class)) in edges.iter().enumerate() {
        let (a, b) = (pts[ia], pts[ib]);
        let len = (b.0 - a.0).hypot(b.1 - a.1);
        if len < 60.0 {
            continue;
        }
        let mid = ((a.0 + b.0) * 0.5, (a.1 + b.1) * 0.5);
        if frame.urbanness(mid.0, mid.1) >= TOWN_BUILT {
            continue;
        }
        let t = ((b.0 - a.0) / len, (b.1 - a.1) / len);
        let angle = t.1.atan2(t.0);
        let row = right_of_way(class);
        for side in [-1.0_f32, 1.0] {
            let n = (-t.1 * side, t.0 * side);
            let key = ei as i32 * 2 + (side > 0.0) as i32;
            let preferred = modern_hash(seed, key, 11, 1301);
            let width = 20.0 + 24.0 * modern_hash(seed, key, 11, 1303);
            let mut off = row + 7.0;
            for depth in 0..3 {
                let at =
                    |s: f32, d: f32| -> V { (a.0 + t.0 * s + n.0 * d, a.1 + t.1 * s + n.1 * d) };
                let mut s = 8.0 + 20.0 * modern_hash(seed, key, depth, 1305);
                let mut col = 0;
                while s + 24.0 < len - 8.0 && count < 700 {
                    col += 1;
                    let k2 = key * 1024 + depth * 64 + col;
                    let noise = |salt: i32| modern_hash(seed, k2, 13, salt);
                    let l = (55.0 + 70.0 * noise(1307)).min(len - 8.0 - s);
                    let w = width * (0.8 + 0.4 * noise(1309));
                    let corners = vec![
                        at(s, off),
                        at(s + l, off),
                        at(s + l, off + w),
                        at(s, off + w),
                    ];
                    s += l + 1.6;
                    let c = centroid(&corners);
                    if frame.urbanness(c.0, c.1) >= TOWN_BUILT {
                        continue;
                    }
                    // Clear of every street (this one included, since the road bends).
                    let clear = edges.iter().all(|&(ja, jb, cl)| {
                        let ro = right_of_way(cl) + 2.0;
                        corners
                            .iter()
                            .all(|p| point_seg_dist(*p, pts[ja], pts[jb]) >= ro)
                            && super::geom::seg_seg_dist(corners[0], corners[2], pts[ja], pts[jb])
                                >= ro
                            && super::geom::seg_seg_dist(corners[1], corners[3], pts[ja], pts[jb])
                                >= ro
                    });
                    if !clear {
                        continue;
                    }
                    if frame.near_water(&corners, 6.0) {
                        continue;
                    }
                    if occupied.iter().any(|o| overlap(&corners, o, 2.0)) {
                        continue;
                    }
                    let choices = crop_choices(zone(frame.urbanness(c.0, c.1)));
                    let crop = household_crop(choices, preferred, noise(1311) < 0.66, noise(1313));
                    fields.push(Field {
                        id: *next_field,
                        ring: corners.iter().map(|p| frame.to_world(p.0, p.1)).collect(),
                        crop,
                        row_angle: angle,
                        variant: (noise(1315) * 3.0) as u8,
                    });
                    *next_field += 1;
                    occupied.push(corners);
                    count += 1;
                }
                off += width + 1.6;
            }
        }
    }
}

/// The plain's field system over all the open country the streets leave: farmland
/// in China is not plots dotted on grass but land worked edge to edge. It is laid
/// out in tracts (条田) a hundred and fifty to two hundred and fifty metres on a
/// side, bounded by farm tracks and ditches, all on one bearing (the bearing of the
/// town's streets, which were laid on the same survey). Each tract is split into
/// household strips ten to thirty metres wide that run its whole length, sown more
/// often than not with the tract's crop. Where a strip meets a street, a house, the
/// water or the town, it is shortened, not dropped, so the fields reach the road.
#[allow(clippy::too_many_arguments)]
pub(super) fn fill_farmland(
    frame: &CityFrame,
    pts: &[V],
    edges: &[(usize, usize, ModernRoadClass)],
    town: &[Vec<V>],
    seed: u32,
    occupied: &mut Vec<Vec<V>>,
    fields: &mut Vec<Field>,
    next_field: &mut u32,
) {
    const TRACT_U: f32 = 200.0;
    const TRACT_V: f32 = 160.0;
    const TRACK_M: f32 = 4.0;
    const BUND_M: f32 = 1.2;
    const BUCKET_M: f32 = 120.0;
    // The bearing of the streets, length-weighted, folded to a quarter turn.
    let (mut sx, mut sy) = (0.0_f32, 0.0_f32);
    for &(a, b, _) in edges {
        let (dx, dy) = (pts[b].0 - pts[a].0, pts[b].1 - pts[a].1);
        let len = dx.hypot(dy);
        if len > 1.0 {
            let t = 4.0 * dy.atan2(dx);
            sx += len * t.cos();
            sy += len * t.sin();
        }
    }
    let angle = sy.atan2(sx) * 0.25;
    let u = (angle.cos(), angle.sin());
    let v = (-u.1, u.0);
    let world = |pu: f32, pv: f32| -> V { (u.0 * pu + v.0 * pv, u.1 * pu + v.1 * pv) };

    // Streets, town blocks and what is already placed, bucketed so a strip is tested
    // only against what is near it.
    let key = |p: V| {
        (
            (p.0 / BUCKET_M).floor() as i32,
            (p.1 / BUCKET_M).floor() as i32,
        )
    };
    let mut street_cells: std::collections::HashMap<(i32, i32), Vec<usize>> = Default::default();
    for (ei, &(a, b, class)) in edges.iter().enumerate() {
        let reach = right_of_way(class) + 2.0;
        let (pa, pb) = (pts[a], pts[b]);
        let (k0, k1) = (
            key((pa.0.min(pb.0) - reach, pa.1.min(pb.1) - reach)),
            key((pa.0.max(pb.0) + reach, pa.1.max(pb.1) + reach)),
        );
        for x in k0.0..=k1.0 {
            for y in k0.1..=k1.1 {
                street_cells.entry((x, y)).or_default().push(ei);
            }
        }
    }
    let bounds = |r: &[V]| {
        r.iter()
            .fold((f32::MAX, f32::MAX, f32::MIN, f32::MIN), |b, p| {
                (b.0.min(p.0), b.1.min(p.1), b.2.max(p.0), b.3.max(p.1))
            })
    };
    let mut area_cells: std::collections::HashMap<(i32, i32), Vec<(bool, usize)>> =
        Default::default();
    let file = |cells: &mut std::collections::HashMap<(i32, i32), Vec<(bool, usize)>>,
                ring: &[V],
                entry: (bool, usize)| {
        let b = bounds(ring);
        let (k0, k1) = (key((b.0, b.1)), key((b.2, b.3)));
        for x in k0.0..=k1.0 {
            for y in k0.1..=k1.1 {
                cells.entry((x, y)).or_default().push(entry);
            }
        }
    };
    for (i, r) in town.iter().enumerate() {
        file(&mut area_cells, r, (true, i));
    }
    for (i, r) in occupied.iter().enumerate() {
        file(&mut area_cells, r, (false, i));
    }

    let radius = frame.radius_m * 0.97;
    let free = |corners: &Vec<V>, wet: bool| -> bool {
        // Land the town has zoned but not yet laid streets over is still farmed, as
        // the edge of a Chinese town is: only its blocks, lots and streets are not.
        if !corners.iter().all(|c| c.0.hypot(c.1) <= radius) {
            return false;
        }
        if wet && frame.near_water(corners, 6.0) {
            return false;
        }
        let bb = bounds(corners);
        let (k0, k1) = (key((bb.0, bb.1)), key((bb.2, bb.3)));
        for x in k0.0..=k1.0 {
            for y in k0.1..=k1.1 {
                for &ei in street_cells.get(&(x, y)).map_or(&[][..], |l| &l[..]) {
                    let (ja, jb, cl) = edges[ei];
                    let ro = right_of_way(cl) + 2.0;
                    if corners
                        .iter()
                        .any(|p| point_seg_dist(*p, pts[ja], pts[jb]) < ro)
                        || super::geom::seg_seg_dist(corners[0], corners[2], pts[ja], pts[jb]) < ro
                        || super::geom::seg_seg_dist(corners[1], corners[3], pts[ja], pts[jb]) < ro
                    {
                        return false;
                    }
                }
                for &(is_town, i) in area_cells.get(&(x, y)).map_or(&[][..], |l| &l[..]) {
                    let clash = if is_town {
                        corners.iter().any(|p| point_in(&town[i], *p))
                            || town[i].iter().any(|p| point_in(corners, *p))
                    } else {
                        overlap(corners, &occupied[i], 2.0)
                    };
                    if clash {
                        return false;
                    }
                }
            }
        }
        true
    };
    let span = (radius / TRACT_U).ceil() as i32 + 1;
    let span_v = (radius / TRACT_V).ceil() as i32 + 1;
    let mut placed = Vec::new();
    for tu in -span..=span {
        for tv in -span_v..=span_v {
            let tract = tu * 4096 + tv;
            let h = |salt: i32| modern_hash(seed, tract, 17, salt);
            let (u0, v0) = (tu as f32 * TRACT_U, tv as f32 * TRACT_V);
            let (u1, v1) = (u0 + TRACT_U - TRACK_M, v0 + TRACT_V - TRACK_M);
            let centre = world((u0 + u1) * 0.5, (v0 + v1) * 0.5);
            if centre.0.hypot(centre.1) > radius + TRACT_U {
                continue;
            }
            // The water is tested strip by strip only in a tract it comes near.
            let wet = frame.near_water(
                &[world(u0, v0), world(u1, v0), world(u1, v1), world(u0, v1)],
                6.0,
            );
            // Neighbouring tracts tend to the same crop, as a village plants its land.
            let region = modern_hash(seed, tu.div_euclid(3) * 977 + tv.div_euclid(3), 19, 1401);
            let preferred = if h(1403) < 0.6 { region } else { h(1405) };
            // And the land of one village is at one stage of the season.
            let stage = modern_hash(seed, tu.div_euclid(3) * 977 + tv.div_euclid(3), 19, 1419);
            let stage = if h(1421) < 0.7 { stage } else { h(1423) };
            // Strips across the tract's short side, so each runs its whole length.
            let along_u = h(1407) < 0.7;
            let (a0, a1, b0, b1) = if along_u {
                (u0, u1, v0, v1)
            } else {
                (v0, v1, u0, u1)
            };
            let at = |a: f32, b: f32| if along_u { world(a, b) } else { world(b, a) };
            let mut b = b0;
            let mut strip = 0;
            while b < b1 - 6.0 {
                strip += 1;
                let n = |salt: i32| modern_hash(seed, tract * 64 + strip, 21, salt);
                let w = (10.0 + 20.0 * n(1409) * n(1411).sqrt()).min(b1 - b);
                // A strip is tested in eighths; the runs of eighths that fit become
                // plots, so a strip that meets a road or a house is shortened, not lost.
                const PIECES: usize = 8;
                let step = (a1 - a0) / PIECES as f32;
                let mut run: Option<(f32, f32)> = None;
                for j in 0..=PIECES {
                    let fits = j < PIECES && {
                        let (s0, s1) = (a0 + step * j as f32, a0 + step * (j + 1) as f32);
                        let corners = vec![
                            at(s0, b),
                            at(s1, b),
                            at(s1, b + w - BUND_M),
                            at(s0, b + w - BUND_M),
                        ];
                        free(&corners, wet)
                    };
                    if fits {
                        let s1 = a0 + step * (j + 1) as f32;
                        run = Some((run.map_or(a0 + step * j as f32, |r| r.0), s1));
                    } else if let Some((r0, r1)) = run.take() {
                        // A plot under twenty metres long is not worth a farmer's while.
                        if r1 - r0 >= 20.0 {
                            let r1 = if r1 < a1 - 0.1 { r1 - BUND_M } else { r1 };
                            placed.push((
                                vec![
                                    at(r0, b),
                                    at(r1, b),
                                    at(r1, b + w - BUND_M),
                                    at(r0, b + w - BUND_M),
                                ],
                                (preferred, stage),
                            ));
                        }
                    }
                }
                b += w;
            }
        }
    }
    for (corners, (preferred, stage)) in placed {
        let c = centroid(&corners);
        let id = *next_field as i32;
        let noise = |salt: i32| modern_hash(seed, id, 23, salt);
        let choices = crop_choices(zone(frame.urbanness(c.0, c.1)));
        // Most strips are the tract's crop; the rest are a household's own choice.
        let crop = household_crop(choices, preferred, noise(1413) < 0.82, noise(1415));
        let (dx, dy) = (corners[1].0 - corners[0].0, corners[1].1 - corners[0].1);
        fields.push(Field {
            id: *next_field,
            ring: corners.iter().map(|p| frame.to_world(p.0, p.1)).collect(),
            crop,
            row_angle: dy.atan2(dx),
            variant: ((if noise(1417) < 0.8 {
                stage
            } else {
                noise(1425)
            }) * 3.0) as u8,
        });
        *next_field += 1;
        occupied.push(corners);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ModernChinaSpec;

    fn frame(style: CityStyle) -> CityFrame {
        CityFrame::new(ModernChinaSpec {
            style,
            ..ModernChinaSpec::default()
        })
    }

    #[test]
    fn suburban_houses_are_plaster_in_china_and_brick_in_the_western_style() {
        let china = frame(CityStyle::ChineseModern);
        assert_eq!(
            suburb_facade(&china, ParcelUse::Residential),
            BuildingFacade::PlasterResidential
        );
        assert_eq!(
            suburb_facade(&china, ParcelUse::Villa),
            BuildingFacade::PlasterResidential
        );
        assert_eq!(
            suburb_facade(&china, ParcelUse::Farmstead),
            BuildingFacade::BrickResidential
        );
        let western = frame(CityStyle::WesternMixed);
        assert_eq!(
            suburb_facade(&western, ParcelUse::Residential),
            BuildingFacade::BrickResidential
        );
    }
}
