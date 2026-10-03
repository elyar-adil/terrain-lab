//! Three-component vectors, y up, metres. Single precision: this is geometry for
//! drawing, not for accumulating error.

use std::ops::{Add, AddAssign, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub const fn v3(x: f32, y: f32, z: f32) -> V3 {
    V3 { x, y, z }
}

impl V3 {
    pub const ZERO: V3 = v3(0.0, 0.0, 0.0);
    pub const UP: V3 = v3(0.0, 1.0, 0.0);

    pub fn dot(self, o: V3) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    pub fn cross(self, o: V3) -> V3 {
        v3(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }
    pub fn len(self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn dist(self, o: V3) -> f32 {
        (self - o).len()
    }
    pub fn norm(self) -> V3 {
        let l = self.len();
        if l < 1e-9 { V3::UP } else { self * (1.0 / l) }
    }
    pub fn lerp(self, o: V3, t: f32) -> V3 {
        self + (o - self) * t
    }
    pub fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
    /// Any unit vector perpendicular to `self` (which should be unit).
    pub fn any_perp(self) -> V3 {
        let helper = if self.y.abs() < 0.9 {
            V3::UP
        } else {
            v3(1.0, 0.0, 0.0)
        };
        self.cross(helper).norm()
    }
    /// Rotate `self` about the unit axis by `angle` radians (Rodrigues).
    pub fn rotate_about(self, axis: V3, angle: f32) -> V3 {
        let (s, c) = angle.sin_cos();
        self * c + axis.cross(self) * s + axis * (axis.dot(self) * (1.0 - c))
    }
}

impl Add for V3 {
    type Output = V3;
    fn add(self, o: V3) -> V3 {
        v3(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}
impl AddAssign for V3 {
    fn add_assign(&mut self, o: V3) {
        *self = *self + o;
    }
}
impl Sub for V3 {
    type Output = V3;
    fn sub(self, o: V3) -> V3 {
        v3(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}
impl Mul<f32> for V3 {
    type Output = V3;
    fn mul(self, k: f32) -> V3 {
        v3(self.x * k, self.y * k, self.z * k)
    }
}
impl Neg for V3 {
    type Output = V3;
    fn neg(self) -> V3 {
        v3(-self.x, -self.y, -self.z)
    }
}

pub fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub const GOLDEN_ANGLE: f32 = 2.399_963_2; // 137.507764 degrees
