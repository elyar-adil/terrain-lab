//! The river: a water surface in a valley of its own, and banks that follow the
//! terrain simulation and the town around them.
//!
//! The water lies `drop_depth` below the ground (a few metres, set by the
//! river's discharge, the climate's wetness and the relief round the town), so
//! the banks have somewhere to go: down from the footway to the water. Each
//! 48 m reach, on each side, is one of
//!
//! * a stone **quay wall** with a promenade, coping and railing (dense, built-up
//!   bank, as in the Shanghai and Suzhou riverfronts);
//! * a **terraced** wall, a lower walkway a metre above the water under the
//!   upper wall (the hydrophilic platform of a Chinese riverfront park);
//! * a **paved slope**, a stone revetment at 1:2 with a promenade on top
//!   (under every bridge, where the abutment slope meets the water, and in parks);
//! * a **natural bank**: grass slope, a mud and sand bar at the toe, reeds in a
//!   humid climate;
//! * a **gravel bank**: cobble and gravel with a wide exposed bar, where the
//!   climate is dry or the channel steep.
//!
//! Which one a reach is, how wide its bar is and how steep its slope, all come
//! from [`RiverSetting`] (moisture, discharge, channel slope, relief, sediment) and
//! from how built-up the bank is, never from a fixed value.

use urban::{CityFrameInfo, Point, RiverSetting};

use crate::math::{Vec2, Vec3, point_in_ring, signed_area};
use crate::mesh::{GroupStyle, MeshBuilder};
use worldgen_core::hash::cell01;
use worldgen_core::smooth01;

use super::ring_of;

const QUAY: &str = "quay.stone";
const FOAM: &str = "water.foam";
const BANK: &str = "bank.earth";
const WATER: &str = "water";
/// Promenade level: a hand above the sidewalk datum.
const PROM: f32 = 0.19;
/// Natural ground level at the top of a bank.
const GROUND: f32 = 0.12;
/// Station spacing along the river and stations per reach.
const SPACING: f32 = 3.0;
const REACH: usize = 16;

/// How far the water lies below the ground, metres.
pub fn drop_depth(setting: &RiverSetting, width: f32) -> f32 {
    let d = 1.9
        + 1.6 * setting.discharge.clamp(0.0, 1.0)
        + 0.8 * (1.0 - setting.moisture.clamp(0.0, 1.0))
        + 0.012 * setting.relief_m.clamp(0.0, 100.0)
        + 0.006 * (width - 40.0).clamp(-30.0, 80.0);
    // Never less than the drop a low girder bridge needs to clear the water.
    d.clamp(3.0, 5.4)
}

/// Height of the water surface in the scene's metres.
pub fn water_level(setting: &RiverSetting, width: f32) -> f32 {
    -drop_depth(setting, width)
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

fn scale(a: [f32; 3], k: f32) -> [f32; 3] {
    [a[0] * k, a[1] * k, a[2] * k]
}

/// Smooth 1-D value noise in -1..1, one lattice point every `period` metres.
fn vnoise(s: f32, period: f32, seed: i32, lane: u32) -> f32 {
    let x = s / period;
    let i = x.floor();
    let f = smooth01(x - i);
    let a = cell01(0xF17, i as i32, seed, lane);
    let b = cell01(0xF17, i as i32 + 1, seed, lane);
    (a + (b - a) * f) * 2.0 - 1.0
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Wall,
    Terrace,
    Paved,
    Natural,
    Gravel,
}

#[derive(Clone, Copy, Debug)]
struct Pt {
    /// Metres outward from the plan's bank line (negative: into the channel).
    o: f32,
    y: f32,
    /// 1 stone, 0 earth: the material of the segment that ends here.
    stone: bool,
    col: [f32; 3],
}

fn pt(o: f32, y: f32, stone: bool, col: [f32; 3]) -> Pt {
    Pt { o, y, stone, col }
}

/// Reach parameters, constant over a reach and varying slowly within it.
#[derive(Clone, Copy)]
struct Reach {
    kind: Kind,
    /// Exposed bar between the toe and the waterline, metres.
    bar: f32,
    /// Horizontal run per metre of fall on a sloping bank.
    ratio: f32,
    /// Bank palette.
    sand: [f32; 3],
    grass: [f32; 3],
    cobble: [f32; 3],
    reeds: f32,
}

struct Palette {
    stone: [f32; 3],
    wet: [f32; 3],
    coping: [f32; 3],
    paving: [f32; 3],
}

const PALETTE: Palette = Palette {
    stone: [0.56, 0.54, 0.50],
    wet: [0.16, 0.19, 0.18],
    coping: [0.70, 0.68, 0.63],
    paving: [0.50, 0.49, 0.47],
};

/// The bank's section from the waterline outward. `room` is the width available
/// before the next street; `n` (-1..1) varies the toe a little along the reach;
/// `cap` is the most the water may narrow the channel on this side.
fn profile(r: &Reach, depth: f32, room: f32, n: f32, cap: f32) -> Vec<Pt> {
    let p = &PALETTE;
    let bed_y = -depth;
    let bar = r.bar.min(cap * 0.6);
    let mut pts: Vec<Pt> = Vec::with_capacity(12);
    let promenade = |pts: &mut Vec<Pt>, o_in: f32, yp: f32| {
        // Coping, then the walkway out to the street, then a kerb down.
        let width = (room - 0.3).clamp(0.7, 3.4);
        pts.push(pt(o_in, yp + 0.13, true, p.coping));
        pts.push(pt(o_in + 0.45, yp + 0.13, true, p.coping));
        pts.push(pt(o_in + 0.45, yp, true, p.paving));
        pts.push(pt(o_in + width, yp, true, p.paving));
        pts.push(pt(o_in + width + 0.04, GROUND - 0.02, true, p.stone));
    };
    match r.kind {
        Kind::Wall => {
            let o_t = -0.2;
            let o_w = (o_t - bar).max(-cap);
            pts.push(pt(o_w, bed_y - 0.05, false, r.sand));
            pts.push(pt(o_t, bed_y + 0.15, false, mix(r.sand, p.wet, 0.4)));
            pts.push(pt(o_t, bed_y + 0.9, true, p.wet));
            pts.push(pt(o_t, PROM, true, p.stone));
            promenade(&mut pts, o_t, PROM);
        }
        Kind::Terrace => {
            let o_u = -0.2;
            let o_f = o_u - 2.5;
            let yt = bed_y + 1.2;
            let o_w = (o_f - bar).max(-cap);
            pts.push(pt(o_w, bed_y - 0.05, false, r.sand));
            pts.push(pt(o_f, bed_y + 0.15, false, mix(r.sand, p.wet, 0.4)));
            pts.push(pt(o_f, bed_y + 0.8, true, p.wet));
            pts.push(pt(o_f, yt, true, p.stone));
            pts.push(pt(o_u, yt, true, p.paving));
            pts.push(pt(o_u, PROM, true, p.stone));
            promenade(&mut pts, o_u, PROM);
        }
        Kind::Paved => {
            let top = 0.7;
            let fall = PROM - bed_y - 0.1;
            let run = (r.ratio.min(2.2) * fall).min(top + cap - bar);
            let o_toe = top - run;
            let o_w = (o_toe - bar).max(-cap);
            pts.push(pt(o_w, bed_y - 0.05, false, r.sand));
            pts.push(pt(o_toe, bed_y + 0.1, false, mix(r.sand, p.wet, 0.5)));
            pts.push(pt(o_toe + run * 0.25, bed_y + 0.1 + fall * 0.25, true, mix(p.wet, p.stone, 0.45)));
            pts.push(pt(top, PROM, true, p.stone));
            promenade(&mut pts, top, PROM);
        }
        Kind::Natural | Kind::Gravel => {
            let gravel = r.kind == Kind::Gravel;
            let rim = (room - 0.4).clamp(1.2, 3.0);
            let fall = GROUND - bed_y - 0.28;
            let wobble = 1.0 + 0.22 * n;
            // The toe sits as far in as the slope needs, but the water keeps most
            // of the channel.
            let run = (r.ratio * wobble * fall).min(rim + cap - bar).max(1.0);
            let o_toe = rim - run;
            let o_w = (o_toe - bar).max(-cap);
            let low = if gravel { r.cobble } else { mix(r.sand, [0.30, 0.27, 0.19], 0.35) };
            let mid = if gravel { mix(r.cobble, r.grass, 0.25) } else { mix(r.grass, [0.30, 0.27, 0.19], 0.4) };
            let top = if gravel { mix(r.cobble, r.grass, 0.55) } else { r.grass };
            let berm_y = bed_y + 0.28;
            pts.push(pt(o_w, bed_y - 0.05, false, mix(r.sand, low, 0.3)));
            pts.push(pt(o_w + (o_toe - o_w) * 0.6, bed_y + 0.12 + 0.05 * n, false, r.sand));
            pts.push(pt(o_toe, berm_y, false, low));
            pts.push(pt(o_toe + run * 0.35, berm_y + fall * 0.33, false, mid));
            pts.push(pt(o_toe + run * 0.7, berm_y + fall * 0.7 + 0.08 * n, false, mix(mid, top, 0.5)));
            pts.push(pt(rim, GROUND, false, top));
            pts.push(pt(rim + 1.2, GROUND, false, top));
        }
    }
    pts
}

fn height_at(pts: &[Pt], o: f32) -> f32 {
    for w in pts.windows(2) {
        if o >= w[0].o && o <= w[1].o && w[1].o > w[0].o {
            let t = (o - w[0].o) / (w[1].o - w[0].o);
            return w[0].y + (w[1].y - w[0].y) * t;
        }
    }
    pts.last().map_or(0.0, |p| p.y)
}

fn differs(a: &[Pt], b: &[Pt]) -> bool {
    a.len() != b.len()
        || a.iter()
            .zip(b)
            .any(|(p, q)| (p.o - q.o).abs() > 0.05 || (p.y - q.y).abs() > 0.05)
}

/// A quad seen from both sides, as the double-sided bank and quay materials are.
fn both_sides(
    builder: &mut MeshBuilder,
    mat: &str,
    c: [Vec3; 4],
    uv: [(f32, f32); 4],
    col: [f32; 3],
) {
    builder.quad_uv(mat, c[0], c[1], c[2], c[3], uv, Some(col));
    builder.quad_uv(
        mat,
        c[3],
        c[2],
        c[1],
        c[0],
        [uv[3], uv[2], uv[1], uv[0]],
        Some(col),
    );
}

/// The river as a water surface plus its banks. `crossings` are (plan point,
/// radius) discs along every river bridge: the bank there is a paved abutment
/// slope. `built` are building centres and `streets` the ground the streets take,
/// so the bank knows how built-up it is and how much room it has.
#[allow(clippy::too_many_arguments)]
pub fn build_water(
    river: &[Point],
    width: f32,
    frame: CityFrameInfo,
    crossings: &[(Vec2, f32)],
    setting: &RiverSetting,
    built: &[Vec2],
    streets: &[Vec<Vec2>],
    builder: &mut MeshBuilder,
) {
    for (name, shadow) in [(WATER, false), (FOAM, false), (QUAY, true), (BANK, true)] {
        builder.style(
            name,
            GroupStyle {
                cast_shadow: false,
                receive_shadow: shadow || name == WATER,
                alpha_cutout: false,
                dynamic: false,
            },
        );
    }
    let mut line = ring_of(river, frame);
    if line.len() < 2 {
        return;
    }
    // Chaikin corner cutting: four passes round every bend into a smooth curve.
    for _ in 0..4 {
        let mut next = Vec::with_capacity(line.len() * 2);
        next.push(line[0]);
        for pair in line.windows(2) {
            next.push(pair[0] * 0.75 + pair[1] * 0.25);
            next.push(pair[0] * 0.25 + pair[1] * 0.75);
        }
        next.push(line[line.len() - 1]);
        line = next;
    }
    // Resample at an even spacing, so reaches are of one length.
    let mut cum = vec![0.0_f32; line.len()];
    for i in 1..line.len() {
        let d = line[i] - line[i - 1];
        cum[i] = cum[i - 1] + d.x.hypot(d.y);
    }
    let total = cum[line.len() - 1];
    if total < SPACING * 2.0 {
        return;
    }
    let count = (total / SPACING).round().max(2.0) as usize;
    let mut pos: Vec<Vec2> = Vec::with_capacity(count + 1);
    let mut seg = 0usize;
    for k in 0..=count {
        let s = total * k as f32 / count as f32;
        while seg + 2 < line.len() && cum[seg + 1] < s {
            seg += 1;
        }
        let span = (cum[seg + 1] - cum[seg]).max(1.0e-4);
        let t = ((s - cum[seg]) / span).clamp(0.0, 1.0);
        pos.push(line[seg] * (1.0 - t) + line[seg + 1] * t);
    }
    let n = pos.len();
    let arc = |i: usize| total * i as f32 / (n - 1) as f32;
    let half = width * 0.5;
    let depth = drop_depth(setting, width);
    let y_w = -depth;
    let mut normal = vec![Vec2::new(0.0, 1.0); n];
    for i in 0..n {
        let a = pos[i.saturating_sub(2)];
        let b = pos[(i + 2).min(n - 1)];
        let (dx, dz) = (b.x - a.x, b.y - a.y);
        let len = dx.hypot(dz).max(1.0e-4);
        normal[i] = Vec2::new(-dz / len, dx / len);
    }
    // Offset `off` to the left (+) of the centre line at station `i`.
    let at = |i: usize, off: f32| {
        Vec2::new(pos[i].x + normal[i].x * off, pos[i].y + normal[i].y * off)
    };
    let to3 = |i: usize, sg: f32, o: f32, y: f32| Vec3::from_plan(at(i, sg * (half + o)), y);

    // ---- the terrain's account of this river --------------------------------
    let moisture = setting.moisture.clamp(0.0, 1.0);
    let discharge = setting.discharge.clamp(0.0, 1.0);
    let low_flow = (1.0 - 0.75 * discharge - 0.35 * moisture).clamp(0.0, 1.0);
    let gravel_p = ((0.62 - moisture) * 2.2 + (setting.slope - 0.0015) * 250.0).clamp(0.0, 0.92);
    let reed_amount = ((moisture - 0.35) * 2.0).clamp(0.0, 1.0) * (1.0 - (setting.slope * 140.0).clamp(0.0, 0.8));
    let steep = (setting.slope / 0.004).clamp(0.0, 1.0);
    let turb = (0.3 + 1.2 * setting.sediment + 0.25 * (1.0 - moisture)).clamp(0.0, 1.0);
    // Banks: dry climates and fast water leave pale sand and cobble; wet ones
    // dark silt and green grass.
    let sand = mix([0.40, 0.37, 0.28], [0.66, 0.58, 0.42], (1.0 - moisture) * 0.8 + setting.sediment * 0.3);
    let grass = mix([0.17, 0.30, 0.10], [0.42, 0.40, 0.21], ((1.0 - moisture) * 1.3 - 0.1).clamp(0.0, 1.0));
    let cobble = mix([0.40, 0.38, 0.34], [0.58, 0.53, 0.45], (1.0 - moisture) * 0.7);

    // ---- room before the next street, per station and side ------------------
    let (mut lo, mut hi) = (Vec2::new(f32::MAX, f32::MAX), Vec2::new(f32::MIN, f32::MIN));
    for p in &pos {
        lo = Vec2::new(lo.x.min(p.x), lo.y.min(p.y));
        hi = Vec2::new(hi.x.max(p.x), hi.y.max(p.y));
    }
    let pad = half + 14.0;
    let near_rings: Vec<(&Vec<Vec2>, [f32; 4])> = streets
        .iter()
        .filter_map(|ring| {
            let (mut a, mut b) = ([f32::MAX; 2], [f32::MIN; 2]);
            for p in ring {
                a = [a[0].min(p.x), a[1].min(p.y)];
                b = [b[0].max(p.x), b[1].max(p.y)];
            }
            (b[0] >= lo.x - pad && a[0] <= hi.x + pad && b[1] >= lo.y - pad && a[1] <= hi.y + pad)
                .then_some((ring, [a[0], a[1], b[0], b[1]]))
        })
        .collect();
    let room_at = |i: usize, sg: f32| {
        let mut o = 0.0_f32;
        while o < 4.6 {
            let p = at(i, sg * (half + o));
            let hit = near_rings.iter().any(|(ring, b)| {
                p.x >= b[0] && p.x <= b[2] && p.y >= b[1] && p.y <= b[3] && point_in_ring(p, ring)
            });
            if hit {
                return (o - 0.1).max(0.0);
            }
            o += 0.3;
        }
        4.6
    };
    let near_crossing = |p: Vec2| crossings.iter().any(|(c, r)| (p.x - c.x).hypot(p.y - c.y) < *r);

    // ---- reaches ------------------------------------------------------------
    let cells = (n - 1).div_ceil(REACH);
    let mut banks: Vec<[Vec<Vec<Pt>>; 2]> = Vec::with_capacity(cells);
    let mut all_kinds: Vec<[Kind; 2]> = Vec::with_capacity(cells);
    let mut all_reeds: Vec<[f32; 2]> = Vec::with_capacity(cells);
    for cell in 0..cells {
        let (i0, i1) = (cell * REACH, ((cell + 1) * REACH).min(n - 1));
        let mut bank: [Vec<Vec<Pt>>; 2] = [Vec::new(), Vec::new()];
        let mut kinds = [Kind::Natural; 2];
        let mut reeds = [0.0_f32; 2];
        if i1 <= i0 {
            banks.push(bank);
            all_kinds.push(kinds);
            all_reeds.push(reeds);
            continue;
        }
        for (side, sg) in [(0usize, -1.0_f32), (1, 1.0)] {
            let side_id = side as i32 + 1;
            let probe = at((i0 + i1) / 2, sg * (half + 30.0));
            let near = built
                .iter()
                .filter(|b| (b.x - probe.x).hypot(b.y - probe.y) < 150.0)
                .count() as f32;
            let urban = (near / 10.0).clamp(0.0, 1.0);
            let crossing = (i0..=i1).any(|i| near_crossing(at(i, sg * half)) || near_crossing(pos[i]));
            let u = cell01(0xA11, cell as i32, side_id, 7);
            let u2 = cell01(0xA12, cell as i32, side_id, 9);
            let kind = if crossing {
                Kind::Paved
            } else if u < urban.powf(1.15) * 0.9 {
                if u2 < 0.5 {
                    Kind::Wall
                } else if u2 < 0.78 {
                    Kind::Terrace
                } else {
                    Kind::Paved
                }
            } else if cell01(0xA13, cell as i32, side_id, 5) < gravel_p {
                Kind::Gravel
            } else {
                Kind::Natural
            };
            kinds[side] = kind;
            if std::env::var("RIVER_DEBUG").is_ok() {
                eprintln!("reach {cell} side {side} at ({:.0},{:.0}) urban {urban:.2} -> {kind:?}", pos[(i0 + i1) / 2].x, pos[(i0 + i1) / 2].y);
            }
            let bar_noise = 0.6 + 0.8 * cell01(0xA14, cell as i32, side_id, 2);
            let bar = match kind {
                Kind::Wall | Kind::Terrace | Kind::Paved => low_flow * 3.0 * bar_noise,
                Kind::Natural => 0.8 + low_flow * low_flow * 9.0 * bar_noise,
                Kind::Gravel => 2.0 + low_flow * 13.0 * bar_noise,
            };
            let reach = Reach {
                kind,
                bar,
                ratio: (4.0 - 2.1 * steep) * (0.85 + 0.3 * cell01(0xA15, cell as i32, side_id, 4)),
                sand,
                grass,
                cobble,
                reeds: if kind == Kind::Natural {
                    reed_amount * (0.35 + 0.65 * cell01(0xA16, cell as i32, side_id, 6))
                } else {
                    0.0
                },
            };
            reeds[side] = reach.reeds;
            let cap = (half * 0.4).max(2.0);
            for i in i0..=i1 {
                let room = (i.saturating_sub(1)..=(i + 1).min(n - 1))
                    .map(|j| room_at(j, sg))
                    .fold(4.6_f32, f32::min);
                let noise = vnoise(arc(i), 17.0, side_id + 31 * cell as i32 % 7, 1);
                bank[side].push(profile(&reach, depth, room, noise, cap));
            }
        }
        banks.push(bank);
        all_kinds.push(kinds);
        all_reeds.push(reeds);
    }

    for cell in 0..cells {
        let (i0, i1) = (cell * REACH, ((cell + 1) * REACH).min(n - 1));
        if i1 <= i0 {
            continue;
        }
        let bank = &banks[cell];
        let kinds = all_kinds[cell];
        let reeds = all_reeds[cell];

        // -- water: between the two waterlines of each station -----------------
        let wl = |side: usize, k: usize| {
            let o = bank[side][k][0].o;
            half + o
        };
        let ts: Vec<f32> = (0..=30)
            .map(|k| 0.5 - 0.5 * (std::f32::consts::PI * k as f32 / 30.0).cos())
            .collect();
        let deep = [0.09, 0.17, 0.19];
        let clear = [0.13, 0.24, 0.25];
        let muddy = [0.27, 0.27, 0.17];
        let silt = [0.34, 0.31, 0.20];
        let colour_at = |t: f32, s: f32| {
            // Distance from the nearer bank, 0 at the edge, 1 mid-channel.
            let c = (1.0 - (2.0 * t - 1.0).abs()).clamp(0.0, 1.0);
            let patch = 0.5 + 0.5 * (s * 0.013 + 1.3).sin() * (s * 0.037).sin();
            let t_local = (turb + (patch - 0.5) * 0.25).clamp(0.0, 1.0);
            let body = mix(clear, muddy, t_local);
            let channel = mix(body, scale(body, 0.62), smooth01(c * 1.2));
            let shallow = mix(channel, silt, (1.0 - smooth01(c * 5.0)) * (0.55 + 0.4 * t_local));
            mix(shallow, mix(deep, body, 0.3), 0.0)
        };
        for i in i0..i1 {
            let s = (arc(i) + arc(i + 1)) * 0.5;
            for w in ts.windows(2) {
                let lat = |k: usize, t: f32| {
                    let l = -wl(0, k - i0);
                    let r = wl(1, k - i0);
                    at(k, l + (r - l) * t)
                };
                let mut quad = vec![lat(i, w[0]), lat(i + 1, w[0]), lat(i + 1, w[1]), lat(i, w[1])];
                if signed_area(&quad) < 0.0 {
                    quad.reverse();
                }
                builder.ground_uv(WATER, &quad, y_w, Some(colour_at((w[0] + w[1]) * 0.5, s)));
            }
        }

        // -- banks -------------------------------------------------------------
        for (side, sg) in [(0usize, -1.0_f32), (1, 1.0)] {
            let side_id = side as i32 + 1;
            for i in i0..i1 {
                let (pa, pb) = (&bank[side][i - i0], &bank[side][i - i0 + 1]);
                let (sa, sb) = (arc(i), arc(i + 1));
                let tone = 0.93 + 0.14 * cell01(0xC33, (sa / 4.0) as i32, side_id, 1);
                let mut u0 = 0.0_f32;
                for k in 0..pa.len().saturating_sub(1) {
                    let seg_len = ((pa[k + 1].o - pa[k].o).hypot(pa[k + 1].y - pa[k].y)).max(0.01);
                    let (p0, p1, q0, q1) = (pa[k], pa[k + 1], pb[k], pb[k + 1]);
                    let col = scale(mix(p1.col, p0.col, 0.35), tone);
                    let corners = [
                        to3(i, sg, p0.o, p0.y),
                        to3(i + 1, sg, q0.o, q0.y),
                        to3(i + 1, sg, q1.o, q1.y),
                        to3(i, sg, p1.o, p1.y),
                    ];
                    let mat = if p1.stone { QUAY } else { BANK };
                    both_sides(
                        builder,
                        mat,
                        corners,
                        [(u0, sa), (u0, sb), (u0 + seg_len, sb), (u0 + seg_len, sa)],
                        col,
                    );
                    u0 += seg_len;
                }
            }
            // A closing cheek where the next reach is different.
            for (i, nb) in [(i0, cell.checked_sub(1)), (i1, (cell + 1 < cells).then_some(cell + 1))] {
                let Some(nb) = nb else { continue };
                if banks[nb][side].is_empty() {
                    continue;
                }
                let prof = &bank[side][i - i0];
                let other = if nb < cell {
                    banks[nb][side].last().unwrap()
                } else {
                    &banks[nb][side][0]
                };
                if !differs(prof, other) {
                    continue;
                }
                let floor = y_w - 0.4;
                for w in prof.windows(2) {
                    let (a, b) = (w[0], w[1]);
                    let corners = [
                        to3(i, sg, a.o, a.y),
                        to3(i, sg, b.o, b.y),
                        to3(i, sg, b.o, floor),
                        to3(i, sg, a.o, floor),
                    ];
                    let mat = if b.stone { QUAY } else { BANK };
                    both_sides(
                        builder,
                        mat,
                        corners,
                        [(a.o, a.y), (b.o, b.y), (b.o, floor), (a.o, floor)],
                        scale(b.col, 0.85),
                    );
                }
            }

            // -- furniture and planting on the bank ----------------------------
            let stone_top = matches!(kinds[side], Kind::Wall | Kind::Terrace | Kind::Paved);
            let mut rail_acc = 0.0_f32;
            let mut bollard_acc = 6.0 + 20.0 * cell01(0xB01, cell as i32, side_id, 3);
            for i in i0..i1 {
                let prof = &bank[side][i - i0];
                let nxt = &bank[side][i - i0 + 1];
                let seg_len = arc(i + 1) - arc(i);
                if stone_top {
                    // Railing along the coping: posts every ~3 m, a top and a mid rail.
                    let o_rail = prof
                        .iter()
                        .find(|p| (p.y - (PROM + 0.13)).abs() < 1.0e-3)
                        .map_or(0.3, |p| p.o + 0.25);
                    let o_rail_n = nxt
                        .iter()
                        .find(|p| (p.y - (PROM + 0.13)).abs() < 1.0e-3)
                        .map_or(0.3, |p| p.o + 0.25);
                    let (p, q) = (
                        at(i, sg * (half + o_rail)),
                        at(i + 1, sg * (half + o_rail_n)),
                    );
                    let rail = [0.55, 0.57, 0.60];
                    let base = PROM + 0.13;
                    for (h, r) in [(base + 1.02, 0.035_f32), (base + 0.55, 0.025)] {
                        builder.tube(
                            "rail.steel",
                            Vec3::from_plan(p, h),
                            Vec3::from_plan(q, h),
                            r,
                            r,
                            5,
                            Some(rail),
                        );
                    }
                    rail_acc -= seg_len;
                    if rail_acc <= 0.0 {
                        builder.tube(
                            "rail.steel",
                            Vec3::from_plan(p, base),
                            Vec3::from_plan(p, base + 1.05),
                            0.045,
                            0.04,
                            5,
                            Some(rail),
                        );
                        rail_acc += 3.0;
                    }
                    bollard_acc -= seg_len;
                    if bollard_acc <= 0.0 && kinds[side] == Kind::Terrace {
                        // Mooring bollards on the lower walkway.
                        let o = prof.iter().find(|q| q.y > y_w + 1.0).map_or(-1.0, |q| q.o + 0.5);
                        let c = at(i, sg * (half + o));
                        let y = height_at(prof, o);
                        let dark = [0.08, 0.09, 0.10];
                        builder.tube(
                            "rail.steel",
                            Vec3::from_plan(c, y),
                            Vec3::from_plan(c, y + 0.5),
                            0.13,
                            0.10,
                            8,
                            Some(dark),
                        );
                        bollard_acc += 24.0;
                    }
                } else if reeds[side] > 0.05 {
                    // Reed beds on the bar and the lower slope: clumps of tall
                    // blade cards, thicker where the water is calm and the air humid.
                    let lo_o = prof[1].o;
                    let hi_o = prof[3].o;
                    let clumps = (reeds[side] * 3.6 + 0.4) as usize;
                    for c in 0..clumps {
                        let r1 = cell01(0xD44, i as i32 * 7 + c as i32, side_id, 1);
                        let r2 = cell01(0xD45, i as i32 * 7 + c as i32, side_id, 2);
                        let patch = 0.5 + 0.5 * vnoise(arc(i), 9.0, side_id, 5);
                        if r1 > 0.25 + 0.75 * patch {
                            continue;
                        }
                        let o = lo_o + (hi_o - lo_o) * r2;
                        let y = height_at(prof, o);
                        if y < y_w + 0.05 {
                            continue;
                        }
                        let along = r1 * SPACING;
                        let c0 = at(i, sg * (half + o));
                        let c1 = at(i + 1, sg * (half + o));
                        let (x, z) = (c0.x + (c1.x - c0.x) * along / SPACING, c0.y + (c1.y - c0.y) * along / SPACING);
                        // A clump of blades: narrow tapering leaves leaning outward,
                        // straw at the tip, olive at the root.
                        let n_blades = 5 + (r1 * 4.0) as usize;
                        for k in 0..n_blades {
                            let h = cell01(0xD46, i as i32 * 31 + c as i32 * 7 + k as i32, side_id, 3);
                            let a = cell01(0xD47, i as i32 * 31 + c as i32 * 7 + k as i32, side_id, 4) * 6.283;
                            let height = 1.1 + 1.1 * h;
                            let lean = Vec2::new(a.cos(), a.sin()) * (0.25 + 0.35 * h);
                            let root = Vec2::new(x + a.cos() * 0.12, z + a.sin() * 0.12);
                            let w = Vec2::new(-a.sin(), a.cos()) * 0.035;
                            let tip = Vec3::from_plan(root + lean, y + height);
                            let base_l = Vec3::from_plan(root - w, y - 0.04);
                            let base_r = Vec3::from_plan(root + w, y - 0.04);
                            let mid_l = Vec3::from_plan(root + lean * 0.35 - w * 0.7, y + height * 0.55);
                            let mid_r = Vec3::from_plan(root + lean * 0.35 + w * 0.7, y + height * 0.55);
                            let root_col = [0.22 * (0.8 + 0.4 * h), 0.30, 0.10];
                            let tip_col = [0.52, 0.50, 0.24];
                            builder.quad_uv(BANK, base_l, base_r, mid_r, mid_l, [(0.0, 0.0); 4], Some(root_col));
                            builder.quad_uv(BANK, mid_l, mid_r, tip, tip, [(0.0, 0.0); 4], Some(tip_col));
                            builder.quad_uv(BANK, base_r, base_l, mid_l, mid_r, [(0.0, 0.0); 4], Some(root_col));
                            builder.quad_uv(BANK, mid_r, mid_l, tip, tip, [(0.0, 0.0); 4], Some(tip_col));
                        }
                    }
                }
            }
        }
    }
}
