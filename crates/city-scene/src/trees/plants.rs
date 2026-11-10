//! The plants the branching grower does not make: palms, bamboo, reed, bushes
//! (sea buckthorn, saxaul, mangrove) and cushion plants.
//!
//! Each is a prototype at unit height like a tree, with the same two groups
//! (`#bark`, `#leaf`), so the renderer binds them with no change: the leaf group
//! takes the species' own card, `vegetation/leaf/<key>`.
//!
//! * **Palm** — a bare column (tapered, or swollen mid-way for the royal palm,
//!   leaning for the coconut) carrying a crown of fronds. A frond is one card
//!   bent into an arch from many quads, because a palm leaf is a single large
//!   object and not a spray: fan palms get a pleated fan blade, feather palms a
//!   long rachis with leaflets (the card textures are `cards::frond_card`).
//! * **Bamboo** — twenty-odd slender jointed culms leaving one point and
//!   arching outward, leaf sprays in the upper third.
//! * **Reed** — a tussock of tall, narrow, bending blades.
//! * **Shrub** — several stems from the ground, leaf sprays along them; the
//!   mangrove stands on arched prop roots.
//! * **Cushion** — a low dome of leaf cards.

use super::TAU;
use super::TreePrototype;
use super::cards::{CARD_WINDOW, card_tile_m};
use crate::math::{Rng, Vec3};
use crate::mesh::MeshBuilder;
use crate::species::{Canopy, Species};

pub(super) fn build(
    prototype: &TreePrototype,
    seed: u32,
    bark: &str,
    leaf: &str,
    builder: &mut MeshBuilder,
) {
    let mut p = Plant {
        species: prototype.species,
        height: prototype.height.max(0.1),
        crown: prototype.crown,
        trunk: prototype.trunk,
        rng: Rng::new(seed),
        bark,
        leaf,
        builder,
    };
    match p.species.canopy {
        Canopy::Palm => p.palm(),
        Canopy::Bamboo => p.bamboo(),
        Canopy::Reed => p.reed(),
        Canopy::Shrub => p.shrub(),
        Canopy::Cushion => p.cushion(),
        _ => {}
    }
}

struct Plant<'a> {
    species: &'static Species,
    height: f32,
    crown: f32,
    trunk: f32,
    rng: Rng,
    bark: &'a str,
    leaf: &'a str,
    builder: &'a mut MeshBuilder,
}

fn up() -> Vec3 {
    Vec3::new(0.0, 1.0, 0.0)
}

impl Plant<'_> {
    /// Unit-space crown radius and trunk radius.
    fn r(&self) -> f32 {
        self.crown / self.height * 0.5
    }

    fn bark_colour(&mut self, lift: f32) -> [f32; 3] {
        let c = self.species.bark.colour;
        let v = lift * self.rng.range(0.92, 1.04);
        [c[0] * v, c[1] * v, c[2] * v]
    }

    /// A tube along a curve of points, tapering from `r0` to `r1`.
    fn limb(&mut self, points: &[Vec3], r0: f32, r1: f32, sides: u8, lift: f32) {
        let n = points.len().max(2) - 1;
        for i in 0..n {
            let t0 = i as f32 / n as f32;
            let t1 = (i + 1) as f32 / n as f32;
            let colour = self.bark_colour(lift);
            self.builder.tube(
                self.bark,
                points[i],
                points[i + 1],
                r0 + (r1 - r0) * t0,
                r0 + (r1 - r0) * t1,
                sides,
                Some(colour),
            );
        }
    }

    /// A crop-card of the species' leaf scatter, as the grower hangs on a shoot.
    fn spray(&mut self, centre: Vec3, outward: Vec3, shoot: Vec3, size: f32) {
        let tile_m = card_tile_m(self.species);
        let window = tile_m * CARD_WINDOW;
        let slack = (tile_m - window).max(0.0);
        let (ou, ov) = (self.rng.range(0.0, slack), self.rng.range(0.0, slack));
        let mut normal = (outward * 0.7 + self.rng.sphere() * 0.55).normalized_or_up();
        if normal.y.abs() < 0.16 {
            normal = (normal + Vec3::new(0.0, 0.4, 0.0)).normalized_or_up();
        }
        let mut axis_v = shoot - normal * shoot.dot(normal);
        if axis_v.length() < 0.2 {
            axis_v = normal.cross(up());
            if axis_v.length() < 1.0e-4 {
                axis_v = normal.cross(Vec3::new(1.0, 0.0, 0.0));
            }
        }
        let axis_v = axis_v.normalized_or_up();
        let axis_u = axis_v.cross(normal).normalized_or_up();
        let normal = axis_u.cross(axis_v).normalized_or_up();
        let shading = (normal * 0.76 + up() * 0.24).normalized_or_up();
        let value = self.rng.range(0.80, 1.0);
        let warm = self.rng.range(-0.04, 0.04);
        let tint = [value * (1.0 + warm), value, value * (1.0 - warm)];
        let half = size * 0.5;
        let uv = |a: f32, b: f32| (ou + a * window, ov + b * window);
        let c = [
            centre - axis_u * half - axis_v * half,
            centre + axis_u * half - axis_v * half,
            centre + axis_u * half + axis_v * half,
            centre - axis_u * half + axis_v * half,
        ];
        self.builder.quad_uv_shaded(
            self.leaf,
            [
                (c[0], shading),
                (c[1], shading),
                (c[2], shading),
                (c[3], shading),
            ],
            [uv(0.0, 0.0), uv(1.0, 0.0), uv(1.0, 1.0), uv(0.0, 1.0)],
            Some(tint),
        );
    }

    /// Card size in unit space, as the grower's `Plan::card`.
    fn card_size(&self) -> f32 {
        // Small plants carry proportionally larger sprays than trees do: a
        // 0.2 m card on a 3 m bush is a speck, so the card grows with the crown.
        let tile = card_tile_m(self.species);
        let boost = match self.species.canopy {
            Canopy::Shrub | Canopy::Bamboo => 2.1,
            Canopy::Cushion => 1.6,
            _ => 1.0,
        };
        tile * boost / self.height * 0.66
    }

    // -- palm -----------------------------------------------------------------

    /// One frond: a single card bent along an arch. `heading` is the horizontal
    /// direction, `rise` the initial climb, `droop` how far the tip falls.
    fn frond(&mut self, top: Vec3, heading: f32, length: f32, width: f32, rise: f32, droop: f32) {
        let segments = 6;
        let dir = Vec3::new(heading.cos(), 0.0, heading.sin());
        let side = Vec3::new(-heading.sin(), 0.0, heading.cos());
        let value = self.rng.range(0.82, 1.0);
        let warm = self.rng.range(-0.05, 0.05);
        let tint = [value * (1.0 + warm), value, value * (1.0 - warm)];
        let point =
            |t: f32| top + dir * (length * t) + up() * (length * (rise * t - droop * t * t));
        for i in 0..segments {
            let (t0, t1) = (i as f32 / segments as f32, (i + 1) as f32 / segments as f32);
            let (p0, p1) = (point(t0), point(t1));
            // The blade's width follows the frond's own outline: widest a third of
            // the way out, closing to a point.
            let w = |t: f32| width * (0.25 + 0.75 * (t * 3.0).min(1.0)) * (1.0 - t * t * 0.85);
            let (w0, w1) = (w(t0), w(t1));
            // Fan fronds lie flatter; the blade's normal is the cross of its two
            // edges, biased up for the sky-lit side.
            let tangent = (p1 - p0).normalized_or_up();
            let n = tangent.cross(side).normalized_or_up();
            let n = if n.y < 0.0 { -n } else { n };
            let n = (n * 0.8 + up() * 0.2).normalized_or_up();
            self.builder.quad_uv_shaded(
                self.leaf,
                [
                    (p0 - side * w0, n),
                    (p0 + side * w0, n),
                    (p1 + side * w1, n),
                    (p1 - side * w1, n),
                ],
                [(0.0, t0), (1.0, t0), (1.0, t1), (0.0, t1)],
                Some(tint),
            );
        }
    }

    fn palm(&mut self) {
        let key = self.species.key;
        let tr = (self.trunk / self.height).max(0.004);
        let crown_r = self.r() * 2.0; // full crown width in unit space
        // Per-species habit: trunk top height, lean, frond count, arch.
        let (top_y, lean, fronds, rise, droop, sides, swell, fan) = match key {
            "da-wang-ye" => (0.80, 0.0, 15, 0.55, 0.85, 12, 0.35, false),
            "ye-zi" => (0.80, 0.12, 17, 0.45, 1.05, 10, 0.10, false),
            "pu-kui" => (0.80, 0.03, 11, 0.28, 0.65, 10, 0.05, true),
            _ => (0.78, 0.02, 14, 0.22, 0.45, 8, 0.0, true),
        };
        let lean_dir = self.rng.range(0.0, TAU);
        let steps = 8;
        let mut points = Vec::new();
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let bow = lean * (t * t) * top_y;
            points.push(Vec3::new(
                lean_dir.cos() * bow,
                top_y * t,
                lean_dir.sin() * bow,
            ));
        }
        let top = *points.last().unwrap();
        // Trunk radius: flared foot, a swelling below the middle for the royal
        // palm, and a thin neck under the crown.
        for i in 0..steps {
            let t0 = i as f32 / steps as f32;
            let t1 = (i + 1) as f32 / steps as f32;
            let radius = |t: f32| {
                tr * (1.0
                    + 0.8 * (1.0 - t * 5.0).max(0.0)
                    + swell * (std::f32::consts::PI * t).sin())
                    * (1.0 - 0.18 * t)
            };
            let colour = self.bark_colour(0.9 + 0.1 * ((i * 3) as f32).sin());
            self.builder.tube(
                self.bark,
                points[i],
                points[i + 1],
                radius(t0),
                radius(t1),
                sides,
                Some(colour),
            );
        }
        // Crownshaft / leaf bases: a short fat collar under the fronds.
        let collar_top = top + up() * 0.035;
        let collar = self.bark_colour(0.85);
        self.builder.tube(
            self.bark,
            top,
            collar_top,
            tr * 1.1,
            tr * 1.5,
            sides,
            Some(collar),
        );
        let length = if fan { crown_r * 0.58 } else { crown_r * 0.60 };
        let width = if fan { length * 0.55 } else { length * 0.36 };
        let spin = self.rng.range(0.0, TAU);
        for k in 0..fronds {
            let heading = spin + k as f32 / fronds as f32 * TAU + self.rng.range(-0.18, 0.18);
            // Older fronds (outer ring) hang lower; the newest stand up.
            let age = (k % 3) as f32 / 2.0;
            let l = length * self.rng.range(0.85, 1.12);
            let climb = rise * (1.15 - 0.6 * age) + self.rng.range(-0.05, 0.05);
            self.frond(
                collar_top,
                heading,
                l,
                width,
                climb,
                droop * (0.7 + 0.5 * age),
            );
        }
    }

    // -- bamboo ---------------------------------------------------------------

    fn bamboo(&mut self) {
        let culms = 22 + self.rng.int(8) as usize;
        let spread = self.r();
        let card = self.card_size();
        for _ in 0..culms {
            let angle = self.rng.range(0.0, TAU);
            let foot = self.rng.range(0.0, 1.0).sqrt() * spread * 0.30;
            let base = Vec3::new(angle.cos() * foot, 0.0, angle.sin() * foot);
            let h = self.rng.range(0.70, 1.0);
            let arch = self.rng.range(0.10, 0.34) * spread * 1.8;
            let r = (self.trunk / self.height).max(0.003) * self.rng.range(0.7, 1.15);
            let steps = 7;
            let mut pts = Vec::new();
            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let out = arch * t * t;
                pts.push(base + Vec3::new(angle.cos() * out, h * t, angle.sin() * out));
            }
            self.limb(&pts, r, r * 0.45, 6, 1.0);
            // Joints: a darker ring every tenth of the height.
            for i in 1..steps {
                let at = pts[i];
                let c = self.bark_colour(0.7);
                self.builder.tube(
                    self.bark,
                    at - up() * 0.004,
                    at + up() * 0.004,
                    r * 1.35,
                    r * 1.35,
                    6,
                    Some(c),
                );
            }
            // Leaf sprays on the upper third, hanging off short side twigs.
            let shoot_dir = (pts[steps] - pts[steps - 2]).normalized_or_up();
            for j in 0..16 {
                let t = 0.50 + 0.50 * (j as f32 / 15.0);
                let idx = ((t * steps as f32) as usize).min(steps);
                let c = pts[idx]
                    + Vec3::new(
                        self.rng.range(-0.5, 0.5),
                        self.rng.range(-0.3, 0.3),
                        self.rng.range(-0.5, 0.5),
                    ) * card
                        * 0.7;
                let out = Vec3::new(angle.cos(), 0.3, angle.sin());
                let size = card * self.rng.range(0.8, 1.15);
                self.spray(c, out, shoot_dir, size);
            }
        }
    }

    // -- reed -----------------------------------------------------------------

    fn reed(&mut self) {
        let blades = 46;
        let spread = self.r();
        let tile = card_tile_m(self.species);
        let value_lo = 0.82;
        for _ in 0..blades {
            let angle = self.rng.range(0.0, TAU);
            let foot = self.rng.range(0.0, 1.0).sqrt() * spread * 0.6;
            let base = Vec3::new(angle.cos() * foot, 0.0, angle.sin() * foot);
            let h = self.rng.range(0.55, 1.0);
            let lean = self.rng.range(0.05, 0.45) * spread * 1.4;
            let width = 0.016 + 0.014 * self.rng.unit();
            let heading = angle + self.rng.range(-0.5, 0.5);
            let dir = Vec3::new(heading.cos(), 0.0, heading.sin());
            let side = Vec3::new(-heading.sin(), 0.0, heading.cos());
            let segs = 5;
            let value = self.rng.range(value_lo, 1.0);
            let warm = self.rng.range(-0.06, 0.10);
            let tint = [value * (1.0 + warm), value, value * (1.0 - warm * 1.6)];
            let (ou, ov) = (
                self.rng.range(0.0, tile * 0.2),
                self.rng.range(0.0, tile * 0.2),
            );
            let p = |t: f32| base + dir * (lean * t * t) + up() * (h * (t - 0.12 * t * t * t));
            for i in 0..segs {
                let (t0, t1) = (i as f32 / segs as f32, (i + 1) as f32 / segs as f32);
                let w = |t: f32| width * (1.0 - 0.8 * t * t);
                let (p0, p1) = (p(t0), p(t1));
                let n = side.cross(p1 - p0).normalized_or_up();
                let n = if n.y < 0.0 { -n } else { n };
                let n = (n * 0.7 + up() * 0.3).normalized_or_up();
                // A narrow strip of leaf scatter, stretched along the blade.
                self.builder.quad_uv_shaded(
                    self.leaf,
                    [
                        (p0 - side * w(t0), n),
                        (p0 + side * w(t0), n),
                        (p1 + side * w(t1), n),
                        (p1 - side * w(t1), n),
                    ],
                    [
                        (ou, ov + t0 * tile * 0.5),
                        (ou + tile * 0.05, ov + t0 * tile * 0.5),
                        (ou + tile * 0.05, ov + t1 * tile * 0.5),
                        (ou, ov + t1 * tile * 0.5),
                    ],
                    Some(tint),
                );
            }
        }
    }

    // -- shrub ----------------------------------------------------------------

    fn shrub(&mut self) {
        let key = self.species.key;
        let mangrove = key == "hong-shu";
        let stems = if key == "suo-suo" { 5 } else { 8 };
        let spread = self.r() * 1.7;
        let card = self.card_size();
        let cards_per_stem = match key {
            "suo-suo" => 9,
            "hong-shu" => 22,
            _ => 20,
        };
        let r0 = (self.trunk / self.height).max(0.004);
        let lift = if mangrove { 0.30 } else { 0.0 };
        for _ in 0..stems {
            let angle = self.rng.range(0.0, TAU);
            let out = self.rng.range(0.35, 1.0) * spread;
            let h = self.rng.range(0.62, 1.0);
            let base = Vec3::new(angle.cos(), 0.0, angle.sin()) * (spread * 0.12);
            let top = Vec3::new(angle.cos() * out, h, angle.sin() * out);
            let mut pts = Vec::new();
            for i in 0..=5 {
                let t = i as f32 / 5.0;
                let mid = base + (top - base) * t;
                pts.push(Vec3::new(
                    mid.x,
                    mid.y * (0.25 + 0.75 * t) + lift * t.min(0.2),
                    mid.z,
                ));
            }
            pts[0] = base + up() * lift;
            self.limb(&pts, r0, r0 * 0.35, 5, 1.0);
            let shoot = (pts[5] - pts[3]).normalized_or_up();
            for j in 0..cards_per_stem {
                let t = 0.35 + 0.65 * (j as f32 / cards_per_stem as f32);
                let at = pts[0]
                    + (pts[5] - pts[0]) * t
                    + Vec3::new(
                        self.rng.range(-1.0, 1.0),
                        self.rng.range(-0.4, 0.6),
                        self.rng.range(-1.0, 1.0),
                    ) * card
                        * 0.55;
                let outward = Vec3::new(angle.cos(), 0.2, angle.sin());
                let size = card * self.rng.range(0.85, 1.2);
                self.spray(at, outward, shoot, size);
            }
        }
        if mangrove {
            // Prop roots: arched tubes from the stems into the mud.
            for _ in 0..10 {
                let angle = self.rng.range(0.0, TAU);
                let from = Vec3::new(angle.cos(), 0.0, angle.sin()) * (spread * 0.1)
                    + up() * (0.30 + self.rng.range(0.0, 0.2));
                let to =
                    Vec3::new(angle.cos(), 0.0, angle.sin()) * (spread * self.rng.range(0.5, 0.9));
                let mid = (from + to) * 0.5 + up() * 0.06;
                self.limb(&[from, mid, to], r0 * 0.5, r0 * 0.3, 4, 0.9);
            }
        }
    }

    // -- cushion --------------------------------------------------------------

    fn cushion(&mut self) {
        let radius = self.r() * 1.0;
        let card = (card_tile_m(self.species) / self.height * 0.5).min(radius * 0.7);
        for _ in 0..46 {
            let a = self.rng.range(0.0, TAU);
            let polar = self.rng.range(0.0, 1.0).sqrt() * std::f32::consts::FRAC_PI_2 * 0.95;
            let dir = Vec3::new(a.cos() * polar.sin(), polar.cos(), a.sin() * polar.sin());
            // A dome: wider than tall.
            let at = Vec3::new(dir.x * radius, dir.y * 0.5 + 0.08, dir.z * radius);
            let shoot = Vec3::new(dir.x, 0.4, dir.z).normalized_or_up();
            let size = card * self.rng.range(0.8, 1.2);
            self.spray(at, dir, shoot, size);
        }
    }
}
