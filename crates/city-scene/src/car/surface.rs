//! A surface that can be cut along curves.
//!
//! A car's lamps, glass, wheel arches and shut lines are not objects laid on
//! its body: they are regions *of* the body, bounded by curves that have
//! nothing to do with how the body happens to be meshed. So the shell is held
//! here as triangles carrying a label, and a region is cut out of it by a
//! scalar field over space: negative inside. Triangles the boundary crosses are
//! split along it, and both sides keep the very same vertices there, so a lens
//! and the wing it is set in meet edge to edge with nothing between them.
//!
//! The same split gives the curve itself (`contour`), which is what a chrome
//! strip, a light guide or a row of LEDs is strung along, and the open edge of
//! a region (`boundary`), which is what the wall of a lamp's housing or the
//! thickness of a wheel's spoke is raised from.
use std::collections::HashMap;

use crate::math::Vec3;
use crate::mesh::MeshBuilder;

/// A point of a surface and the surface's normal there.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Vert {
    pub p: Vec3,
    pub n: Vec3,
}

impl Vert {
    pub fn new(p: Vec3, n: Vec3) -> Self {
        Self { p, n }
    }
    fn mix(a: Vert, b: Vert, t: f32) -> Vert {
        Vert {
            p: a.p + (b.p - a.p) * t,
            n: (a.n + (b.n - a.n) * t).normalized_or_up(),
        }
    }
    /// The point `depth` under the surface.
    pub fn sunk(self, depth: f32) -> Vert {
        Vert {
            p: self.p - self.n * depth,
            n: self.n,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Tri {
    pub v: [Vert; 3],
    pub label: u8,
}

impl Tri {
    pub fn centre(&self) -> Vec3 {
        (self.v[0].p + self.v[1].p + self.v[2].p) / 3.0
    }
}

#[derive(Clone, Default)]
pub(crate) struct Soup {
    pub tris: Vec<Tri>,
}

type Key = (i32, i32, i32);

fn key_of(p: Vec3) -> Key {
    // A fiftieth of a millimetre: vertices made by the same split are the same
    // floats, so this only has to keep distinct vertices apart.
    let q = |v: f32| (v * 50_000.0).round() as i32;
    (q(p.x), q(p.y), q(p.z))
}

fn before(a: Vec3, b: Vec3) -> bool {
    (a.x, a.y, a.z) < (b.x, b.y, b.z)
}

/// Where a field crosses zero on the edge between two vertices of opposite
/// sign. The edge is taken in one order whichever triangle asks, so the two
/// triangles that share it are given the same point to the last bit.
fn crossing(field: &dyn Fn(Vec3) -> f32, a: Vert, fa: f32, b: Vert, fb: f32) -> Vert {
    let (a, fa, b, fb) = if before(a.p, b.p) {
        (a, fa, b, fb)
    } else {
        (b, fb, a, fa)
    };
    let (mut lo, mut flo, mut hi, mut fhi) = (0.0_f32, fa, 1.0_f32, fb);
    let mut t = fa / (fa - fb);
    // The field is not linear along the edge (a circle, a rounded corner):
    // close in on the root by false position, kept inside its bracket.
    for _ in 0..4 {
        let f = field(a.p + (b.p - a.p) * t);
        if f.abs() < 2.0e-6 {
            break;
        }
        if (f < 0.0) == (flo < 0.0) {
            lo = t;
            flo = f;
        } else {
            hi = t;
            fhi = f;
        }
        let next = lo + (hi - lo) * flo / (flo - fhi);
        t = if next.is_finite() && next > lo && next < hi {
            next
        } else {
            (lo + hi) * 0.5
        };
    }
    Vert::mix(a, b, t.clamp(0.0, 1.0))
}

fn area(v: &[Vert; 3]) -> f32 {
    (v[1].p - v[0].p).cross(v[2].p - v[0].p).length()
}

/// A triangle parted by a field: the pieces inside (negative) and outside, and
/// the segment of the boundary across it, if it is crossed.
struct Parted {
    /// Not crossed: the whole triangle is inside (`Some(true)`) or outside.
    whole: Option<bool>,
    inside: Vec<[Vert; 3]>,
    outside: Vec<[Vert; 3]>,
    segment: Option<(Vert, Vert)>,
}

impl Parted {
    fn whole(inside: bool) -> Self {
        Self {
            whole: Some(inside),
            inside: Vec::new(),
            outside: Vec::new(),
            segment: None,
        }
    }
}

/// How much faster than distance a field may change and still be trusted to
/// keep one sign over a triangle whose middle is far from its zero.
const FIELD_SLOPE: f32 = 3.0;

fn part(v: &[Vert; 3], field: &dyn Fn(Vec3) -> f32) -> Parted {
    // Most triangles are nowhere near the curve: one look at the middle
    // settles those. The fields here are distances, or near enough.
    let middle = (v[0].p + v[1].p + v[2].p) / 3.0;
    let reach = v
        .iter()
        .map(|c| (c.p - middle).length())
        .fold(0.0_f32, f32::max);
    let at_middle = field(middle);
    if at_middle.abs() > reach * FIELD_SLOPE + 1.0e-4 {
        return Parted::whole(at_middle < 0.0);
    }
    let f = [field(v[0].p), field(v[1].p), field(v[2].p)];
    let inside = [f[0] < 0.0, f[1] < 0.0, f[2] < 0.0];
    let count = inside.iter().filter(|x| **x).count();
    if count == 3 || count == 0 {
        return Parted::whole(count == 3);
    }
    // The odd vertex out, and the two edges from it that the boundary crosses.
    let lone_inside = count == 1;
    let i = (0..3).find(|i| inside[*i] == lone_inside).unwrap_or(0);
    let (a, b, c) = (v[i], v[(i + 1) % 3], v[(i + 2) % 3]);
    let (fa, fb, fc) = (f[i], f[(i + 1) % 3], f[(i + 2) % 3]);
    let ab = crossing(field, a, fa, b, fb);
    let ac = crossing(field, a, fa, c, fc);
    let tip = vec![[a, ab, ac]];
    let rest = vec![[ab, b, c], [ab, c, ac]];
    let keep = |list: Vec<[Vert; 3]>| -> Vec<[Vert; 3]> {
        list.into_iter().filter(|t| area(t) > 1.0e-11).collect()
    };
    let (inside, outside) = if lone_inside {
        (keep(tip), keep(rest))
    } else {
        (keep(rest), keep(tip))
    };
    Parted {
        whole: None,
        inside,
        outside,
        segment: Some((ab, ac)),
    }
}

impl Soup {
    /// The surface through a stack of closed rings, as `model::skin` draws it.
    pub fn from_rings(rings: &[Vec<Vec3>], label: u8) -> Self {
        let mut soup = Soup::default();
        if rings.len() < 2 || rings[0].len() < 3 {
            return soup;
        }
        let normals = crate::model::skin_normals(rings);
        let around = rings[0].len();
        for i in 0..rings.len() - 1 {
            for k in 0..around {
                let k1 = (k + 1) % around;
                let at = |i: usize, k: usize| Vert::new(rings[i][k], normals[i][k]);
                let (a, b, c, d) = (at(i, k), at(i + 1, k), at(i + 1, k1), at(i, k1));
                let face = (b.p - a.p).cross(c.p - a.p) + (c.p - a.p).cross(d.p - a.p);
                if face.length() < 1.0e-12 {
                    continue;
                }
                let quad = if face.dot(a.n + b.n + c.n + d.n) >= 0.0 {
                    [a, b, c, d]
                } else {
                    [d, c, b, a]
                };
                for v in [[quad[0], quad[1], quad[2]], [quad[0], quad[2], quad[3]]] {
                    if area(&v) > 1.0e-11 {
                        soup.tris.push(Tri { v, label });
                    }
                }
            }
        }
        soup
    }

    /// A grid of vertices that carry their own normals, each quad wound to
    /// face the way they do. `closed` joins every row's end to its start.
    pub fn from_grid(rows: &[Vec<Vert>], closed: bool, label: u8) -> Self {
        let mut soup = Soup::default();
        if rows.len() < 2 || rows[0].len() < 2 {
            return soup;
        }
        let around = rows[0].len();
        let columns = if closed { around } else { around - 1 };
        for i in 0..rows.len() - 1 {
            for k in 0..columns {
                let k1 = (k + 1) % around;
                let (a, b, c, d) = (rows[i][k], rows[i + 1][k], rows[i + 1][k1], rows[i][k1]);
                let face = (b.p - a.p).cross(c.p - a.p) + (c.p - a.p).cross(d.p - a.p);
                let quad = if face.dot(a.n + b.n + c.n + d.n) >= 0.0 {
                    [a, b, c, d]
                } else {
                    [d, c, b, a]
                };
                for v in [[quad[0], quad[1], quad[2]], [quad[0], quad[2], quad[3]]] {
                    if area(&v) > 1.0e-11 {
                        soup.tris.push(Tri { v, label });
                    }
                }
            }
        }
        soup
    }

    /// Press a relief into the triangles of `of`: every vertex moves along its
    /// normal by `height` there, and its normal leans by the relief's slope, so
    /// the relief is lit as what it is. Where the height is zero (the rim of a
    /// groove) a vertex stays exactly where it was, and the surface stays
    /// closed.
    pub fn emboss(&mut self, of: &dyn Fn(u8) -> bool, height: &dyn Fn(Vec3) -> f32) {
        const STEP: f32 = 8.0e-4;
        for tri in &mut self.tris {
            if !of(tri.label) {
                continue;
            }
            for v in &mut tri.v {
                let rise = height(v.p);
                if rise == 0.0 {
                    // Flat here, or the rim of the relief: an edge either way.
                    continue;
                }
                let d = |axis: Vec3| {
                    (height(v.p + axis * STEP) - height(v.p - axis * STEP)) / (2.0 * STEP)
                };
                let gradient = Vec3::new(
                    d(Vec3::new(1.0, 0.0, 0.0)),
                    d(Vec3::new(0.0, 1.0, 0.0)),
                    d(Vec3::new(0.0, 0.0, 1.0)),
                );
                let along = gradient - v.n * gradient.dot(v.n);
                v.p = v.p + v.n * rise;
                v.n = (v.n - along).normalized_or_up();
            }
        }
    }

    /// Relabel.
    pub fn rename(&mut self, from: u8, to: u8) {
        for tri in &mut self.tris {
            if tri.label == from {
                tri.label = to;
            }
        }
    }

    /// Give the part of the surface where `field` is negative the label `to`,
    /// among the triangles `from` allows, splitting those the boundary crosses.
    pub fn cut(&mut self, field: &dyn Fn(Vec3) -> f32, from: &dyn Fn(u8) -> bool, to: u8) {
        self.split(field, &|t| from(t.label), Some(to));
    }

    /// Split the triangles of `from` along the zero of `field` and leave their
    /// labels alone: the curve becomes edges of the mesh, with vertices on it
    /// to move (the bottom of a groove) or to follow.
    pub fn score(&mut self, field: &dyn Fn(Vec3) -> f32, from: &dyn Fn(u8) -> bool) {
        self.split(field, &|t| from(t.label), None);
    }

    /// `score`, among the triangles `pick` chooses.
    pub fn score_in(&mut self, field: &dyn Fn(Vec3) -> f32, pick: &dyn Fn(&Tri) -> bool) {
        self.split(field, pick, None);
    }

    /// Part the picked triangles along the zero of `field`, in place: a
    /// triangle the curve does not cross is at most relabelled.
    fn split(&mut self, field: &dyn Fn(Vec3) -> f32, pick: &dyn Fn(&Tri) -> bool, to: Option<u8>) {
        for index in 0..self.tris.len() {
            let tri = self.tris[index];
            if !pick(&tri) {
                continue;
            }
            let parted = part(&tri.v, field);
            if let Some(inside) = parted.whole {
                if let (true, Some(to)) = (inside, to) {
                    self.tris[index].label = to;
                }
                continue;
            }
            let inner = to.unwrap_or(tri.label);
            let mut pieces = parted
                .inside
                .into_iter()
                .map(|v| Tri { v, label: inner })
                .chain(parted.outside.into_iter().map(|v| Tri {
                    v,
                    label: tri.label,
                }));
            match pieces.next() {
                Some(first) => self.tris[index] = first,
                // Nothing left of it but slivers.
                None => continue,
            }
            self.tris.extend(pieces);
        }
    }

    /// The curves along which `field` is zero on the triangles of `from`,
    /// each in order from one end to the other (a closed one returns to its
    /// first point).
    pub fn contour(
        &self,
        field: &dyn Fn(Vec3) -> f32,
        from: &dyn Fn(u8) -> bool,
    ) -> Vec<Vec<Vert>> {
        let mut segments = Vec::new();
        for tri in &self.tris {
            if !from(tri.label) {
                continue;
            }
            if let Some((a, b)) = part(&tri.v, field).segment
                && key_of(a.p) != key_of(b.p)
            {
                segments.push((a, b));
            }
        }
        chain(&segments)
    }

    /// The edges of the region `of` that no second triangle of the region
    /// shares, chained into loops, each running with the region on its left
    /// as seen from outside the surface.
    pub fn boundary(&self, of: &dyn Fn(u8) -> bool) -> Vec<Vec<Vert>> {
        let mut edges: HashMap<(Key, Key), (Vert, Vert)> = HashMap::new();
        for tri in &self.tris {
            if !of(tri.label) {
                continue;
            }
            for i in 0..3 {
                let (a, b) = (tri.v[i], tri.v[(i + 1) % 3]);
                let (ka, kb) = (key_of(a.p), key_of(b.p));
                if ka == kb {
                    continue;
                }
                if edges.remove(&(kb, ka)).is_none() {
                    edges.insert((ka, kb), (a, b));
                }
            }
        }
        let mut list: Vec<((Key, Key), (Vert, Vert))> = edges.into_iter().collect();
        // A hash map's order is not the same from run to run; the model must be.
        list.sort_by_key(|(keys, _)| *keys);
        let segments: Vec<(Vert, Vert)> = list.into_iter().map(|(_, edge)| edge).collect();
        chain_directed(&segments)
    }

    /// The triangles `of` picks, as a surface of their own.
    pub fn take(&self, of: &dyn Fn(&Tri) -> bool) -> Soup {
        Soup {
            tris: self.tris.iter().filter(|t| of(t)).copied().collect(),
        }
    }

    /// The same surface, every vertex sunk `depth` along its normal.
    pub fn sunk(&self, depth: f32) -> Soup {
        Soup {
            tris: self
                .tris
                .iter()
                .map(|t| Tri {
                    v: [t.v[0].sunk(depth), t.v[1].sunk(depth), t.v[2].sunk(depth)],
                    label: t.label,
                })
                .collect(),
        }
    }

    /// The point of the surface that `map` sends to `target`, if there is one:
    /// how a part designed on a drawing (a lamp's elevation) finds its place on
    /// the body.
    pub fn locate(
        &self,
        of: &dyn Fn(u8) -> bool,
        map: &dyn Fn(Vec3) -> (f32, f32),
        target: (f32, f32),
    ) -> Option<Vert> {
        for tri in &self.tris {
            if !of(tri.label) {
                continue;
            }
            let [a, b, c] = [map(tri.v[0].p), map(tri.v[1].p), map(tri.v[2].p)];
            let det = (b.1 - c.1) * (a.0 - c.0) + (c.0 - b.0) * (a.1 - c.1);
            if det.abs() < 1.0e-12 {
                continue;
            }
            let u = ((b.1 - c.1) * (target.0 - c.0) + (c.0 - b.0) * (target.1 - c.1)) / det;
            let v = ((c.1 - a.1) * (target.0 - c.0) + (a.0 - c.0) * (target.1 - c.1)) / det;
            let w = 1.0 - u - v;
            if u >= -1.0e-4 && v >= -1.0e-4 && w >= -1.0e-4 {
                return Some(Vert {
                    p: tri.v[0].p * u + tri.v[1].p * v + tri.v[2].p * w,
                    n: (tri.v[0].n * u + tri.v[1].n * v + tri.v[2].n * w).normalized_or_up(),
                });
            }
        }
        None
    }

    /// Draw the triangles `of` picks, `lift` off the surface along its normals,
    /// welded: one vertex where triangles meet.
    pub fn emit(&self, builder: &mut MeshBuilder, key: &str, lift: f32, of: &dyn Fn(&Tri) -> bool) {
        let mut index: HashMap<(Key, Key), u32> = HashMap::new();
        let mut vertices: Vec<(Vec3, Vec3)> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        for tri in &self.tris {
            if !of(tri) {
                continue;
            }
            for v in &tri.v {
                let p = v.p + v.n * lift;
                let slot = (key_of(p), key_of(v.n * 0.002));
                let next = vertices.len() as u32;
                let at = *index.entry(slot).or_insert_with(|| {
                    vertices.push((p, v.n));
                    next
                });
                indices.push(at);
            }
        }
        builder.indexed(key, &vertices, &indices);
    }
}

/// Join segments end to end into curves, whichever way round each was given.
fn chain(segments: &[(Vert, Vert)]) -> Vec<Vec<Vert>> {
    let mut ends: HashMap<Key, Vec<usize>> = HashMap::new();
    for (index, (a, b)) in segments.iter().enumerate() {
        ends.entry(key_of(a.p)).or_default().push(index);
        ends.entry(key_of(b.p)).or_default().push(index);
    }
    let mut used = vec![false; segments.len()];
    let mut curves = Vec::new();
    // Open curves first, from their loose ends; what is left is loops.
    let mut starts: Vec<usize> = (0..segments.len())
        .filter(|i| {
            let (a, b) = segments[*i];
            ends[&key_of(a.p)].len() == 1 || ends[&key_of(b.p)].len() == 1
        })
        .collect();
    starts.extend(0..segments.len());
    for start in starts {
        if used[start] {
            continue;
        }
        used[start] = true;
        let (a, b) = segments[start];
        let (first, mut tip) = if ends[&key_of(a.p)].len() == 1 || ends[&key_of(b.p)].len() != 1 {
            (a, b)
        } else {
            (b, a)
        };
        let mut curve = vec![first, tip];
        loop {
            let next = ends[&key_of(tip.p)].iter().copied().find(|i| !used[*i]);
            let Some(next) = next else { break };
            used[next] = true;
            let (a, b) = segments[next];
            tip = if key_of(a.p) == key_of(tip.p) { b } else { a };
            curve.push(tip);
        }
        curves.push(curve);
    }
    curves
}

/// Join directed edges head to tail into loops, keeping their direction.
fn chain_directed(segments: &[(Vert, Vert)]) -> Vec<Vec<Vert>> {
    let mut from: HashMap<Key, Vec<usize>> = HashMap::new();
    for (index, (a, _)) in segments.iter().enumerate() {
        from.entry(key_of(a.p)).or_default().push(index);
    }
    let mut used = vec![false; segments.len()];
    let mut loops = Vec::new();
    for start in 0..segments.len() {
        if used[start] {
            continue;
        }
        used[start] = true;
        let mut curve = vec![segments[start].0, segments[start].1];
        loop {
            let tip = key_of(curve[curve.len() - 1].p);
            let next = from
                .get(&tip)
                .and_then(|list| list.iter().copied().find(|i| !used[*i]));
            let Some(next) = next else { break };
            used[next] = true;
            curve.push(segments[next].1);
        }
        loops.push(curve);
    }
    loops
}

/// The length of a curve.
pub(crate) fn length_of(curve: &[Vert]) -> f32 {
    curve.windows(2).map(|w| (w[1].p - w[0].p).length()).sum()
}

/// Points along a curve every `pitch`, centred on its length.
pub(crate) fn spaced(curve: &[Vert], pitch: f32) -> Vec<Vert> {
    let total = length_of(curve);
    if total < pitch * 0.5 || pitch <= 0.0 {
        return Vec::new();
    }
    let count = (total / pitch).floor() as usize;
    let mut want = (total - count as f32 * pitch) * 0.5;
    let mut out = Vec::with_capacity(count + 1);
    let mut walked = 0.0;
    for w in curve.windows(2) {
        let step = (w[1].p - w[0].p).length();
        while want <= walked + step && step > 1.0e-9 {
            out.push(Vert::mix(w[0], w[1], (want - walked) / step));
            want += pitch;
        }
        walked += step;
    }
    out
}

/// The part of a curve that `keep` accepts, as the runs it leaves.
pub(crate) fn runs(curve: &[Vert], keep: &dyn Fn(Vert) -> bool) -> Vec<Vec<Vert>> {
    let mut out = Vec::new();
    let mut run: Vec<Vert> = Vec::new();
    for v in curve {
        if keep(*v) {
            run.push(*v);
        } else if run.len() > 1 {
            out.push(std::mem::take(&mut run));
        } else {
            run.clear();
        }
    }
    if run.len() > 1 {
        out.push(run);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sheet() -> Soup {
        // A flat sheet of 8 x 8 quads, one metre square, facing up.
        let rings: Vec<Vec<Vec3>> = (0..=8)
            .map(|i| {
                let x = i as f32 / 8.0;
                let mut ring: Vec<Vec3> =
                    (0..=8).map(|k| Vec3::new(x, 0.0, k as f32 / 8.0)).collect();
                // Closed by a far point so the ring is a ring; the test looks
                // only at the sheet.
                ring.push(Vec3::new(x, -1.0, 0.5));
                ring
            })
            .collect();
        let mut soup = Soup::from_rings(&rings, 0);
        soup.tris.retain(|t| t.v.iter().all(|v| v.p.y > -0.5));
        soup
    }

    fn area_of(soup: &Soup, label: u8) -> f32 {
        soup.tris
            .iter()
            .filter(|t| t.label == label)
            .map(|t| area(&t.v) * 0.5)
            .sum()
    }

    #[test]
    fn a_cut_follows_its_curve_and_loses_no_surface() {
        let mut soup = sheet();
        let whole = area_of(&soup, 0);
        let disc = |p: Vec3| ((p.x - 0.5).powi(2) + (p.z - 0.5).powi(2)).sqrt() - 0.3;
        soup.cut(&disc, &|l| l == 0, 1);
        let inside = area_of(&soup, 1);
        let outside = area_of(&soup, 0);
        assert!((inside + outside - whole).abs() < 1.0e-4, "surface lost");
        let want = std::f32::consts::PI * 0.09;
        assert!(
            (inside - want).abs() < want * 0.02,
            "disc of {inside}, wanted {want}"
        );
        // Every vertex on the edge of the disc is on the circle itself.
        let edge = soup.boundary(&|l| l == 1);
        assert_eq!(edge.len(), 1, "one opening");
        for v in &edge[0] {
            assert!(disc(v.p).abs() < 1.0e-4, "edge vertex off the circle");
        }
        // And the curve asked for directly is the same circle, closed.
        let mut plain = sheet();
        plain.score(&|p: Vec3| p.x - 0.31, &|_| true);
        let curves = plain.contour(&disc, &|_| true);
        assert_eq!(curves.len(), 1);
        let ring = &curves[0];
        assert!((ring[0].p - ring[ring.len() - 1].p).length() < 1.0e-5);
        assert!((length_of(ring) - std::f32::consts::TAU * 0.3).abs() < 0.02);
    }
}
