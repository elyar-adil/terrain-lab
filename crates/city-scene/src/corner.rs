//! The geometry of a junction corner: how far back each street stops, and the
//! kerb return that joins one street's kerb to the next.
//!
//! A junction box is only seamless when it is built from the streets' own
//! edges.  Each street's kerb line runs on into the box in a straight line and
//! the corner is a circular arc tangent to both kerb lines (the kerb return,
//! 缘石转弯半径).  The street therefore has to stop where its kerb meets the
//! arc, which depends on the angle it meets its neighbour at: an acute corner
//! needs a long trim and an obtuse one a short trim.  The previous version cut
//! every street at one radius per node, so at a skewed junction the kerbs of
//! two streets crossed before either reached the box and their footways were
//! drawn on top of each other.
//!
//! The footway's back edge is the same construction one sidewalk further out,
//! so the corner paving meets both footways, and the block behind them, along
//! their own lines.

use crate::math::Vec2;

/// Below this gap between two arms (radians short of a straight line) the
/// corner between them is open: the kerbs meet at a reflex angle or run on in
/// one line, and there is no kerb return to fit.
const OPEN_CORNER: f32 = 0.03;
/// Points per radian along a kerb return: a point every 4.5 degrees.
const ARC_POINTS_PER_RADIAN: f32 = 12.7;
/// Longest trim a corner may ask for.  Only an arm meeting its neighbour at
/// under about 15 degrees needs more, and the planner does not draw those.
pub const MAX_TRIM: f32 = 60.0;

/// One arm of a junction, in the junction's frame (the node at the origin).
#[derive(Clone, Copy, Debug)]
pub struct Arm {
    /// Unit direction out of the junction along the street.
    pub dir: Vec2,
    /// Centreline to kerb face.
    pub kerb: f32,
    /// Centreline to the back of the footway.
    pub walk: f32,
    /// Kerb return radius this street asks for at its corners.
    pub radius: f32,
    /// How much further out the kerb and footway on the clockwise side stand:
    /// the side of the arriving traffic, widened by a queue lane.
    pub widen: f32,
}

/// Where `p + s * d` meets `q + t * e`, as `(s, t)`.
pub fn intersect(p: Vec2, d: Vec2, q: Vec2, e: Vec2) -> Option<(f32, f32)> {
    let denom = d.cross(e);
    if denom.abs() < 1.0e-6 {
        return None;
    }
    let w = q - p;
    Some((w.cross(e) / denom, w.cross(d) / denom))
}

/// The angle from `a` round to `b`, counter-clockwise in `(0, 2π]`.
pub fn gap(a: Vec2, b: Vec2) -> f32 {
    let mut g = b.y.atan2(b.x) - a.y.atan2(a.x);
    while g <= 0.0 {
        g += std::f32::consts::TAU;
    }
    while g > std::f32::consts::TAU {
        g -= std::f32::consts::TAU;
    }
    g
}

/// How far along each arm its corners need it trimmed.  `arms` are in
/// counter-clockwise order; the corner after arm `i` is between its
/// counter-clockwise kerb and arm `i + 1`'s clockwise kerb.
///
/// An arm must stop at or beyond both the point where its kerb meets the kerb
/// return and the point where its footway's back edge meets the next footway's,
/// or the two footways overlap outside the box.
pub fn trims(arms: &[Arm]) -> Vec<f32> {
    let n = arms.len();
    let mut trim = vec![0.0_f32; n];
    if n < 2 {
        return trim;
    }
    for i in 0..n {
        // Two arms have two corners, one each side; the loop visits both.
        let j = (i + 1) % n;
        let (a, b) = (arms[i], arms[j]);
        let g = gap(a.dir, b.dir);
        if g >= std::f32::consts::PI - OPEN_CORNER {
            continue;
        }
        let radius = bend_radius(g, a.radius.min(b.radius));
        let tangent = radius / (g * 0.5).tan();
        let (na, nb) = (a.dir.left_normal(), b.dir.left_normal());
        for (offset_a, offset_b, extra) in [
            (a.kerb, b.kerb + b.widen, tangent),
            (a.walk, b.walk + b.widen, 0.0),
        ] {
            let Some((s, t)) = intersect(na * offset_a, a.dir, nb * -offset_b, b.dir) else {
                continue;
            };
            trim[i] = trim[i].max(s + extra);
            trim[j] = trim[j].max(t + extra);
        }
    }
    for value in &mut trim {
        *value = value.clamp(0.0, MAX_TRIM);
    }
    trim
}

/// Shortest straight-to-curve run a corner's kerb gets, metres.
const MIN_TANGENT: f32 = 6.0;

/// The radius of the corner between two arms `g` apart.  A street turning a
/// corner takes its kerb return; a street that only bends at the node (the
/// arms nearly in line) would get a return a metre or two long, which is a
/// kink, so its kerb is eased round on at least `MIN_TANGENT` each way.
fn bend_radius(g: f32, radius: f32) -> f32 {
    let deflection = (std::f32::consts::PI - g).max(1.0e-3);
    radius.max((MIN_TANGENT / (deflection * 0.5).tan()).min(400.0))
}

/// The kerb return from `a` to `b`: a straight run along each kerb line into a
/// circular arc tangent to both.  `da` and `db` are the kerb lines' directions
/// into the junction at `a` and `b`, and the arm at `b` is counter-clockwise of
/// the arm at `a`.  The first point is `a` and the last is `b`.
///
/// The radius shrinks when the ports are too close to the corner for it, so the
/// return always stays tangent to both kerbs at or inside the ports.  An open
/// corner (the arms at a straight line or more apart) gets a smooth curve
/// tangent to both kerbs instead.
pub fn kerb_return(a: Vec2, da: Vec2, b: Vec2, db: Vec2, radius: f32) -> Vec<Vec2> {
    let g = gap(-da, -db);
    let chord = a.distance(b);
    if chord < 1.0e-3 {
        return vec![a, b];
    }
    let convex = g < std::f32::consts::PI - OPEN_CORNER;
    if convex && let Some((s, t)) = intersect(a, da, b, db) {
        if s < -1.0e-3 || t < -1.0e-3 {
            // The kerbs already crossed short of the ports: the arms overlap,
            // and the best the box can do is close the gap straight.
            return vec![a, b];
        }
        let half = g * 0.5;
        let radius = bend_radius(g, radius);
        let fit = radius
            .min(s.max(0.0) * half.tan())
            .min(t.max(0.0) * half.tan());
        let p = a + da * s;
        if fit < 0.05 {
            return dedup(vec![a, p, b]);
        }
        let tangent = fit / half.tan();
        let ta = p - da * tangent;
        let tb = p - db * tangent;
        let bisector = (-da - db).normalize();
        let centre = p + bisector * (fit / half.sin());
        let mut points = vec![a];
        let (start, end) = (ta - centre, tb - centre);
        let a0 = start.y.atan2(start.x);
        let mut sweep = end.y.atan2(end.x) - a0;
        while sweep > std::f32::consts::PI {
            sweep -= std::f32::consts::TAU;
        }
        while sweep < -std::f32::consts::PI {
            sweep += std::f32::consts::TAU;
        }
        let steps = ((sweep.abs() * ARC_POINTS_PER_RADIAN).ceil() as usize).max(2);
        for k in 0..=steps {
            let angle = a0 + sweep * k as f32 / steps as f32;
            points.push(centre + Vec2::new(angle.cos(), angle.sin()) * fit);
        }
        points.push(b);
        return dedup(points);
    }
    // An open corner: a cubic leaving each port along its kerb, so the kerb
    // has no kink where the street ends.  In one line it is the straight chord.
    let handle = chord * 0.45;
    let steps = 8;
    let (c1, c2) = (a + da * handle, b + db * handle);
    let points = (0..=steps)
        .map(|k| {
            let t = k as f32 / steps as f32;
            let u = 1.0 - t;
            a * (u * u * u) + c1 * (3.0 * u * u * t) + c2 * (3.0 * u * t * t) + b * (t * t * t)
        })
        .collect();
    dedup(points)
}

/// Drop consecutive points closer than a centimetre, keeping both ends.
fn dedup(points: Vec<Vec2>) -> Vec<Vec2> {
    let mut out: Vec<Vec2> = Vec::with_capacity(points.len());
    let last = points.len().saturating_sub(1);
    for (index, point) in points.into_iter().enumerate() {
        match out.last() {
            Some(previous) if previous.distance(point) < 0.01 => {
                if index == last {
                    out.pop();
                    out.push(point);
                }
            }
            _ => out.push(point),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arm(degrees: f32, kerb: f32, walk: f32) -> Arm {
        let r = degrees.to_radians();
        Arm {
            dir: Vec2::new(r.cos(), r.sin()),
            kerb,
            walk,
            radius: 12.0,
            widen: 0.0,
        }
    }

    /// Distance from `p` to the line through `a` along `d`.
    fn off_line(p: Vec2, a: Vec2, d: Vec2) -> f32 {
        (p - a).cross(d).abs()
    }

    #[test]
    fn a_square_crossing_trims_every_arm_by_the_other_half_width_and_the_radius() {
        let arms: Vec<Arm> = [0.0, 90.0, 180.0, 270.0]
            .iter()
            .map(|d| arm(*d, 8.0, 11.0))
            .collect();
        for value in trims(&arms) {
            // Kerb at 8 m across, then a 12 m return: 20 m.
            assert!((value - 20.0).abs() < 1.0e-3, "{value}");
        }
    }

    #[test]
    fn an_acute_corner_trims_further_than_an_obtuse_one() {
        // A skewed crossing: the 60 degree corners need more than the 120.
        let arms: Vec<Arm> = [0.0, 60.0, 180.0, 240.0]
            .iter()
            .map(|d| arm(*d, 8.0, 11.0))
            .collect();
        let t = trims(&arms);
        let square = 20.0;
        for value in t {
            assert!(value > square, "an acute corner trimmed only {value} m");
        }
    }

    #[test]
    fn a_tee_does_not_trim_its_through_street_for_the_straight_side() {
        let arms = [
            arm(0.0, 8.0, 11.0),
            arm(90.0, 5.0, 7.0),
            arm(180.0, 8.0, 11.0),
        ];
        let t = trims(&arms);
        // The through arms only need to clear the stem: 5 m and the return.
        assert!((t[0] - 17.0).abs() < 1.0e-3, "{t:?}");
        assert!((t[2] - 17.0).abs() < 1.0e-3, "{t:?}");
        // The stem clears the through street's kerb and the return.
        assert!((t[1] - 20.0).abs() < 1.0e-3, "{t:?}");
    }

    #[test]
    fn the_kerb_return_is_tangent_to_both_kerbs_and_round() {
        // Arm A along +x, its CCW kerb at y = 8; arm B along +y, its CW kerb at
        // x = 8.  Ports 20 m out.
        let (a, da) = (Vec2::new(20.0, 8.0), Vec2::new(-1.0, 0.0));
        let (b, db) = (Vec2::new(8.0, 20.0), Vec2::new(0.0, -1.0));
        let line = kerb_return(a, da, b, db, 12.0);
        assert_eq!(line.first().copied(), Some(a));
        assert_eq!(line.last().copied(), Some(b));
        // Every point is on the kerb lines or on the 12 m circle about (20, 20).
        let centre = Vec2::new(20.0, 20.0);
        for p in &line {
            let on_a = off_line(*p, a, da) < 1.0e-3 && p.x >= 20.0 - 1.0e-3;
            let on_b = off_line(*p, b, db) < 1.0e-3 && p.y >= 20.0 - 1.0e-3;
            let on_arc = (p.distance(centre) - 12.0).abs() < 1.0e-3;
            assert!(on_a || on_b || on_arc, "{p:?} is off the kerb return");
        }
        // And it leaves each port along its kerb: no kink where the street ends.
        let first = (line[1] - line[0]).normalize();
        assert!(first.dot(da) > 0.999, "{first:?}");
    }

    #[test]
    fn a_kerb_return_too_big_for_its_ports_shrinks_to_fit() {
        let (a, da) = (Vec2::new(12.0, 8.0), Vec2::new(-1.0, 0.0));
        let (b, db) = (Vec2::new(8.0, 12.0), Vec2::new(0.0, -1.0));
        let line = kerb_return(a, da, b, db, 12.0);
        // Only 4 m to the corner on each kerb: a 4 m return.
        let centre = Vec2::new(12.0, 12.0);
        for p in &line[1..line.len() - 1] {
            assert!((p.distance(centre) - 4.0).abs() < 1.0e-3, "{p:?}");
        }
    }

    #[test]
    fn an_open_corner_is_straight_when_the_kerbs_are_in_line() {
        let (a, da) = (Vec2::new(-10.0, 8.0), Vec2::new(1.0, 0.0));
        let (b, db) = (Vec2::new(10.0, 8.0), Vec2::new(-1.0, 0.0));
        for p in kerb_return(a, da, b, db, 12.0) {
            assert!((p.y - 8.0).abs() < 1.0e-3, "{p:?}");
        }
    }
}
