//! Plan geometry: axis-aligned rectangles, the four faces, and the 2D affine
//! maps that put a unit drawn in its own frame into the plate.

/// A face of the plate.  The plate frame has x east and y north, so the south
/// face is `y = 0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Side {
    South,
    East,
    North,
    West,
}

impl Side {
    pub const ALL: [Side; 4] = [Side::South, Side::East, Side::North, Side::West];

    /// Outward normal in the plate frame.
    pub fn normal(self) -> [f32; 2] {
        match self {
            Side::South => [0.0, -1.0],
            Side::East => [1.0, 0.0],
            Side::North => [0.0, 1.0],
            Side::West => [-1.0, 0.0],
        }
    }

    pub fn from_normal(n: [f32; 2]) -> Side {
        if n[0].abs() > n[1].abs() {
            if n[0] > 0.0 { Side::East } else { Side::West }
        } else if n[1] > 0.0 {
            Side::North
        } else {
            Side::South
        }
    }
}

const EPS: f32 = 1e-3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

/// A wall shared by two rectangles.
#[derive(Debug, Clone, Copy)]
pub struct Shared {
    /// The wall runs along y (a vertical wall in the drawing) when true.
    pub vertical: bool,
    /// Coordinate of the wall across its length (x for a vertical wall).
    pub at: f32,
    pub lo: f32,
    pub hi: f32,
}

impl Rect {
    pub fn new(xa: f32, ya: f32, xb: f32, yb: f32) -> Rect {
        Rect {
            x0: xa.min(xb),
            y0: ya.min(yb),
            x1: xa.max(xb),
            y1: ya.max(yb),
        }
    }

    pub fn w(&self) -> f32 {
        self.x1 - self.x0
    }

    pub fn d(&self) -> f32 {
        self.y1 - self.y0
    }

    pub fn area(&self) -> f32 {
        self.w() * self.d()
    }

    pub fn centre(&self) -> [f32; 2] {
        [(self.x0 + self.x1) * 0.5, (self.y0 + self.y1) * 0.5]
    }

    pub fn overlap_area(&self, o: &Rect) -> f32 {
        let w = self.x1.min(o.x1) - self.x0.max(o.x0);
        let d = self.y1.min(o.y1) - self.y0.max(o.y0);
        if w > 0.0 && d > 0.0 { w * d } else { 0.0 }
    }

    pub fn contains_rect(&self, o: &Rect) -> bool {
        o.x0 >= self.x0 - EPS && o.x1 <= self.x1 + EPS && o.y0 >= self.y0 - EPS && o.y1 <= self.y1 + EPS
    }

    /// The wall the two rectangles share, if it is longer than a hair.
    pub fn shared(&self, o: &Rect) -> Option<Shared> {
        let ylo = self.y0.max(o.y0);
        let yhi = self.y1.min(o.y1);
        let xlo = self.x0.max(o.x0);
        let xhi = self.x1.min(o.x1);
        if yhi - ylo > EPS && ((self.x1 - o.x0).abs() < EPS || (self.x0 - o.x1).abs() < EPS) {
            let at = if (self.x1 - o.x0).abs() < EPS { self.x1 } else { self.x0 };
            return Some(Shared { vertical: true, at, lo: ylo, hi: yhi });
        }
        if xhi - xlo > EPS && ((self.y1 - o.y0).abs() < EPS || (self.y0 - o.y1).abs() < EPS) {
            let at = if (self.y1 - o.y0).abs() < EPS { self.y1 } else { self.y0 };
            return Some(Shared { vertical: false, at, lo: xlo, hi: xhi });
        }
        None
    }
}

/// An affine map `p' = M p + t` restricted to what plans need: quarter turns and
/// mirrors.
#[derive(Debug, Clone, Copy)]
pub struct Xf {
    m: [f32; 4],
    t: [f32; 2],
}

impl Xf {
    pub fn identity() -> Xf {
        Xf { m: [1.0, 0.0, 0.0, 1.0], t: [0.0, 0.0] }
    }

    /// Mirror about the vertical line `x = cx`.
    pub fn mirror_x(cx: f32) -> Xf {
        Xf { m: [-1.0, 0.0, 0.0, 1.0], t: [2.0 * cx, 0.0] }
    }

    /// A quarter turn counter-clockwise about `c`, `k` times.
    pub fn quarter_turns(c: [f32; 2], k: u32) -> Xf {
        let mut out = Xf::identity();
        for _ in 0..(k % 4) {
            let step = Xf {
                m: [0.0, -1.0, 1.0, 0.0],
                t: [c[0] + c[1], c[1] - c[0]],
            };
            out = out.then(step);
        }
        out
    }

    /// The map that applies `self` first, then `next`.
    pub fn then(self, next: Xf) -> Xf {
        let a = next.m;
        let b = self.m;
        Xf {
            m: [
                a[0] * b[0] + a[1] * b[2],
                a[0] * b[1] + a[1] * b[3],
                a[2] * b[0] + a[3] * b[2],
                a[2] * b[1] + a[3] * b[3],
            ],
            t: [
                a[0] * self.t[0] + a[1] * self.t[1] + next.t[0],
                a[2] * self.t[0] + a[3] * self.t[1] + next.t[1],
            ],
        }
    }

    pub fn point(&self, p: [f32; 2]) -> [f32; 2] {
        [
            self.m[0] * p[0] + self.m[1] * p[1] + self.t[0],
            self.m[2] * p[0] + self.m[3] * p[1] + self.t[1],
        ]
    }

    /// A direction (no translation).
    pub fn dir(&self, v: [f32; 2]) -> [f32; 2] {
        [self.m[0] * v[0] + self.m[1] * v[1], self.m[2] * v[0] + self.m[3] * v[1]]
    }

    pub fn rect(&self, r: &Rect) -> Rect {
        let a = self.point([r.x0, r.y0]);
        let b = self.point([r.x1, r.y1]);
        Rect::new(a[0], a[1], b[0], b[1])
    }

    /// Whether the map swaps the x and y axes.
    pub fn swaps_axes(&self) -> bool {
        self.m[0].abs() < 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quarter_turn_about_the_plate_centre_maps_south_to_east() {
        let xf = Xf::quarter_turns([12.0, 12.0], 1);
        let p = xf.point([0.0, 0.0]);
        assert!((p[0] - 24.0).abs() < 1e-4 && p[1].abs() < 1e-4, "{p:?}");
        assert_eq!(Side::from_normal(xf.dir(Side::South.normal())), Side::East);
    }

    #[test]
    fn mirror_keeps_rectangles_normalised() {
        let r = Xf::mirror_x(5.0).rect(&Rect::new(1.0, 0.0, 3.0, 2.0));
        assert!((r.x0 - 7.0).abs() < 1e-4 && (r.x1 - 9.0).abs() < 1e-4);
    }

    #[test]
    fn shared_wall_is_found() {
        let a = Rect::new(0.0, 0.0, 3.0, 3.0);
        let b = Rect::new(3.0, 1.0, 5.0, 5.0);
        let s = a.shared(&b).unwrap();
        assert!(s.vertical && (s.lo - 1.0).abs() < 1e-4 && (s.hi - 3.0).abs() < 1e-4);
    }
}
