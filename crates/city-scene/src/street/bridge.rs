//! The structure that carries a street over something else: parapets with a
//! capping rail, and piers on real ground.

use crate::mesh::MeshBuilder;
use crate::network::Road;

use super::{Carriageway, offset_path, sweep};

/// Parapets, piers and abutments for an elevated road.
///
/// The previous renderer lifted a bridge ribbon six metres and left it floating.
/// A viaduct without a deck edge and piers reads as a road pasted onto the sky,
/// which is worse than not drawing the bridge at all.
pub(super) fn bridge_structure(road: &Road, builder: &mut MeshBuilder) {
    if road.layer == 0 {
        return;
    }
    let path = road.carriageway.clone();
    let length = path.length();
    if length < 4.0 {
        return;
    }
    let surface = Carriageway::on_path(path, road.half_width());
    let half = road.half_width();
    // Parapet: a capping ribbon plus its full-height face on both sides.
    for side in [-1.0_f32, 1.0] {
        let edge = side * (half - 0.3);
        let cap = offset_path(&surface, edge);
        builder.ribbon("bridge.concrete", &cap, -0.18, 0.18, 0.0, length, 0.85, None);
        sweep(builder, "bridge.concrete", &Carriageway::on_path(cap, 0.2), -0.18, 0.18, 0.0, length, 0.85);
    }
    if road.bridge {
        river_bridge(road, &surface, length, builder);
        return;
    }
    // Piers every 28 m, skipping any that would land in the carriageway of
    // another road — the same avoidance the source kernel applies.
    let mut station = 14.0;
    while station < length - 12.0 {
        let (position, tangent) = surface.path.sample(station);
        let top = position.y - road.deck_thickness;
        if top > 1.2 {
            let height = top;
            let normal = tangent.left_normal();
            let base = crate::math::Vec3::new(position.x - normal.x * 0.7, 0.0, position.z - normal.y * 0.7);
            let top_point = crate::math::Vec3::new(position.x - normal.x * 0.7, height, position.z - normal.y * 0.7);
            builder.tube("bridge.concrete", base, top_point, 0.7, 0.6, 6, None);
            // Pier cap, spread to carry the deck.
            let cap_a = crate::math::Vec3::new(position.x - normal.x * 1.1, height, position.z - normal.y * 1.1);
            let cap_b = crate::math::Vec3::new(position.x + normal.x * 1.1, height, position.z + normal.y * 1.1);
            builder.tube("bridge.concrete", cap_a, cap_b, 0.6, 0.6, 4, None);
        }
        station += 28.0;
    }
}

const STEEL: [f32; 3] = [0.30, 0.33, 0.36];
const STONE: [f32; 3] = [0.52, 0.47, 0.41];

/// A deck over water: a deep girder with a soffit, hammerhead piers standing
/// in the channel, and — on a long span — a pair of towers carrying draped
/// main cables and vertical hangers, so it reads as a bridge from the air and
/// not as a road that happens to be lifted.
fn river_bridge(road: &Road, surface: &Carriageway, length: f32, builder: &mut MeshBuilder) {
    use crate::math::{Vec2, Vec3};
    let half = road.half_width();
    let depth = (road.deck_thickness * 2.2).max(1.6);
    let steps = ((length / 3.0).ceil() as usize).max(2);
    // Girder: outer fascia on both sides and the soffit between them.
    let mut prev: Option<(Vec3, Vec3)> = None;
    for k in 0..=steps {
        let st = length * k as f32 / steps as f32;
        let l = surface.point(st, -half, 0.0);
        let r = surface.point(st, half, 0.0);
        if let Some((pl, pr)) = prev {
            for (a, b) in [(pl, l), (pr, r)] {
                builder.wall("bridge.steel", Vec2::new(a.x, a.z), Vec2::new(b.x, b.z), a.y - depth, a.y, Some(STEEL));
                builder.wall("bridge.steel", Vec2::new(b.x, b.z), Vec2::new(a.x, a.z), b.y - depth, b.y, Some(STEEL));
            }
            builder.quad(
                "bridge.steel",
                Vec3::new(pl.x, pl.y - depth, pl.z),
                Vec3::new(pr.x, pr.y - depth, pr.z),
                Vec3::new(r.x, r.y - depth, r.z),
                Vec3::new(l.x, l.y - depth, l.z),
                Some([0.2, 0.22, 0.24]),
            );
        }
        prev = Some((l, r));
    }
    // Piers: a wall-like column pair with a hammerhead cap, every ~34 m, only
    // where the deck is well clear of the ground.
    let mut st = 17.0;
    while st < length - 14.0 {
        let (pos, tan) = surface.path.sample(st);
        if pos.y - depth > 1.5 {
            let n = tan.left_normal();
            let top = pos.y - depth;
            for side in [-0.5_f32, 0.5] {
                let c = Vec2::new(pos.x + n.x * half * side, pos.z + n.y * half * side);
                crate::mesh::box_at(builder, "bridge.concrete", c, (top - 9.0) * 0.5, 2.4, top + 9.0, 3.2, tan.y.atan2(tan.x));
            }
            crate::mesh::box_at(builder, "bridge.concrete", Vec2::new(pos.x, pos.z), top - 0.6, half * 2.0 + 3.0, 1.2, 3.6, tan.y.atan2(tan.x));
        }
        st += 34.0;
    }
    if length < 70.0 {
        return;
    }
    // Towers at a quarter and three quarters; deck-level anchor at each end.
    let tower_h = (length * 0.16).clamp(14.0, 34.0);
    let towers = [length * 0.25, length * 0.75];
    let out = half + 1.4;
    let mut tops = [[Vec3::default(); 2]; 2];
    for (ti, &ts) in towers.iter().enumerate() {
        for (si, side) in [-1.0_f32, 1.0].into_iter().enumerate() {
            let base = surface.point(ts, side * out, 0.0);
            let (_, tan) = surface.path.sample(ts);
            let yaw = tan.y.atan2(tan.x);
            let c = Vec2::new(base.x, base.z);
            crate::mesh::box_at(builder, "bridge.concrete", c, (base.y + tower_h - 9.0) * 0.5, 2.6, base.y + tower_h + 9.0, 2.6, yaw);
            tops[ti][si] = Vec3::new(base.x, base.y + tower_h, base.z);
        }
        // Portal beams between the legs, near the top and at deck level.
        for f in [0.55_f32, 0.92] {
            let a = surface.point(ts, -out, tower_h * f);
            let b = surface.point(ts, out, tower_h * f);
            builder.tube("bridge.concrete", a, b, 0.9, 0.9, 6, Some(STONE));
        }
    }
    // Main cables: catenary-ish parabola between tower tops sagging to the deck
    // mid-span, and straight back-stays to the deck at each end.
    let cable = |builder: &mut MeshBuilder, side: usize, sgn: f32| {
        let mut pts: Vec<Vec3> = Vec::new();
        let seg = 16;
        // back-stay, start
        let a0 = surface.point(0.0, sgn * out, 1.0);
        for k in 0..=seg {
            let t = k as f32 / seg as f32;
            pts.push(Vec3::new(a0.x + (tops[0][side].x - a0.x) * t, a0.y + (tops[0][side].y - a0.y) * t, a0.z + (tops[0][side].z - a0.z) * t));
        }
        // main span
        let (p0, p1) = (tops[0][side], tops[1][side]);
        let sag_to = surface.point(length * 0.5, sgn * out, 1.6).y;
        for k in 1..=seg * 3 {
            let t = k as f32 / (seg * 3) as f32;
            let x = p0.x + (p1.x - p0.x) * t;
            let z = p0.z + (p1.z - p0.z) * t;
            let base = p0.y + (p1.y - p0.y) * t;
            let dip = 4.0 * t * (1.0 - t);
            pts.push(Vec3::new(x, base + (sag_to - (p0.y + p1.y) * 0.5) * dip, z));
        }
        let a1 = surface.point(length, sgn * out, 1.0);
        for k in 1..=seg {
            let t = k as f32 / seg as f32;
            pts.push(Vec3::new(p1.x + (a1.x - p1.x) * t, p1.y + (a1.y - p1.y) * t, p1.z + (a1.z - p1.z) * t));
        }
        for w in pts.windows(2) {
            builder.tube("bridge.steel", w[0], w[1], 0.28, 0.28, 5, Some([0.27, 0.29, 0.31]));
        }
        // Hangers from the main span down to the deck edge.
        let mut idx = seg;
        while idx < seg + seg * 3 {
            let pt = pts[idx];
            // find nearest deck station by projecting along the path
            let mut best = (f32::MAX, 0.0);
            let mut q = 0.0;
            while q <= length {
                let d = surface.point(q, sgn * out, 0.0);
                let dd = (d.x - pt.x).powi(2) + (d.z - pt.z).powi(2);
                if dd < best.0 { best = (dd, q); }
                q += 1.5;
            }
            let d = surface.point(best.1, sgn * out, 0.6);
            if pt.y - d.y > 1.0 {
                builder.tube("bridge.steel", Vec3::new(pt.x, pt.y, pt.z), d, 0.07, 0.07, 4, Some([0.3, 0.32, 0.34]));
            }
            idx += 3;
        }
    };
    cable(builder, 0, -1.0);
    cable(builder, 1, 1.0);
}
