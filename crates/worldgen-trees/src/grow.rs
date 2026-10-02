//! The grower: (species, seed, conditions) in, a branch skeleton and individual
//! leaves out.
//!
//! A tree is made the way a tree is made. A trunk with taper and root flare; limbs
//! at the crown's foot, spiralling up it (or in whorls, for a conifer); branches
//! off the limbs; twigs off the branches; and leaves on the twigs. Each level draws
//! its own counts, angles, lengths and bends from a seed derived from *that branch's*
//! address, so changing one branch changes nothing else, and no two trees (or two
//! branches, or two leaves) share anything but the species' means.
//!
//! The crown envelope of the species' habit keeps the silhouette: a limb is only
//! as long as the envelope allows at the height it leaves the trunk.

use worldgen_core::Seed;
use worldgen_core::seed::Rng;

use crate::math::{GOLDEN_ANGLE, V3, smoothstep, v3};
use crate::params::{Architecture, architecture, profile};
use crate::species::{Habit, SPECIES, Species};

/// Everything that distinguishes this tree from another of the same species.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TreeSpec {
    pub species: usize,
    /// The tree's own identity: every branch and leaf is derived from it.
    pub seed: u64,
    pub height_m: f32,
    /// 0 = grown in a stand (narrow, self-pruned), 1 = open-grown (broad, low branches).
    pub openness: f32,
    /// 0 = a sapling's proportions, 1 = old.
    pub age: f32,
    /// Day of the year, 0..1 (0.5 is the height of summer in the north).
    pub season: f32,
    /// 0 = dying, 1 = healthy: sparse crown, dead twigs.
    pub health: f32,
    /// Lowest limb, metres: a street tree is lifted so a bus can pass.
    pub lift_m: f32,
}

impl TreeSpec {
    /// A typical healthy mature tree of the species, in summer.
    pub fn typical(species: usize, seed: u64) -> TreeSpec {
        let s = &SPECIES[species];
        TreeSpec {
            species,
            seed,
            height_m: 0.5 * (s.height_m.0 + s.height_m.1),
            openness: 0.7,
            age: 0.7,
            season: 0.5,
            health: 1.0,
            lift_m: 0.0,
        }
    }
}

/// A straight tapered piece of wood.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    pub a: V3,
    pub b: V3,
    pub ra: f32,
    pub rb: f32,
    /// 0 trunk, 1 limb, 2 branch, 3 twig.
    pub level: u8,
}

/// One leaf (or needle spray, or compound leaf). Every number is this leaf's own.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Leaf {
    /// Where the blade starts.
    pub pos: V3,
    /// Unit vector from the blade's base to its tip.
    pub dir: V3,
    /// Unit normal of the blade's upper face.
    pub normal: V3,
    /// Blade length, metres.
    pub length: f32,
    /// Outline parameters, 0..255: lobing or teeth, tip sharpness, asymmetry, curl.
    pub shape: [u8; 4],
    /// Hue shift, shade (inside the crown or out), age or damage, and when in the
    /// season this leaf turns.
    pub tint: [u8; 4],
}

#[derive(Debug, Clone)]
pub struct Tree {
    pub spec: TreeSpec,
    pub lod: u8,
    pub segments: Vec<Segment>,
    pub leaves: Vec<Leaf>,
    pub height: f32,
    pub crown_radius: f32,
    pub crown_base: f32,
    pub trunk_radius: f32,
    pub bark: [f32; 3],
    /// Share of the crown in leaf (0 in winter for a deciduous tree), how far the
    /// leaves have turned, how fresh they still are, how much is in bloom.
    pub cover: f32,
    pub autumn: f32,
    pub flush: f32,
    pub bloom: f32,
}

/// How many leaves each level of detail draws, at most.
pub const LEAF_BUDGET: [usize; 4] = [30_000, 6_500, 1_100, 170];
/// The deepest level of wood each level of detail draws.
pub const WOOD_LEVEL: [u8; 4] = [3, 3, 2, 1];

struct R(Rng);

impl R {
    fn of(seed: Seed) -> R {
        R(seed.rng())
    }
    fn u(&mut self) -> f32 {
        self.0.unit() as f32
    }
    fn range(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.u()
    }
    /// Roughly normal, mean 0, sd 1.
    fn normal(&mut self) -> f32 {
        (self.u() + self.u() + self.u() + self.u() - 2.0) * 1.732
    }
    fn unit_vec(&mut self) -> V3 {
        let z = self.range(-1.0, 1.0);
        let a = self.range(0.0, std::f32::consts::TAU);
        let r = (1.0 - z * z).max(0.0).sqrt();
        v3(r * a.cos(), z, r * a.sin())
    }
}

struct Path {
    pts: Vec<V3>,
    rad: Vec<f32>,
    cum: Vec<f32>,
}

impl Path {
    fn len(&self) -> f32 {
        *self.cum.last().unwrap_or(&0.0)
    }
    /// Position, direction and radius at fraction `t` of the arc length.
    fn at(&self, t: f32) -> (V3, V3, f32) {
        let s = t.clamp(0.0, 1.0) * self.len();
        let k = self.cum.partition_point(|&c| c < s).clamp(1, self.pts.len() - 1);
        let (c0, c1) = (self.cum[k - 1], self.cum[k]);
        let f = if c1 > c0 { (s - c0) / (c1 - c0) } else { 0.0 };
        let dir = (self.pts[k] - self.pts[k - 1]).norm();
        (self.pts[k - 1].lerp(self.pts[k], f), dir, self.rad[k - 1] + (self.rad[k] - self.rad[k - 1]) * f)
    }
}

struct Crown {
    z0: f32,
    height: f32,
    radius: f32,
    habit: Habit,
    lobes: [f32; 4], // amplitude and phase of the first two harmonics
    lobing: f32,
}

impl Crown {
    /// How far from the axis the crown reaches at height `y`, in direction `azimuth`.
    fn reach(&self, y: f32, azimuth: f32) -> f32 {
        let h = ((y - self.z0) / (self.height - self.z0).max(0.1)).clamp(0.0, 1.0);
        let lobe = 1.0
            + self.lobing
                * (self.lobes[0] * (azimuth - self.lobes[1]).cos() + self.lobes[2] * (2.0 * (azimuth - self.lobes[3])).cos());
        self.radius * profile(self.habit, h) * lobe.max(0.4)
    }
}

struct Grower {
    arch: Architecture,
    crown: Crown,
    seed: Seed,
    segments: Vec<Segment>,
    /// (start, end, start radius, end radius, seed) of every twig.
    twigs: Vec<(V3, V3, f32, f32, u64)>,
    lean_axis: V3,
}

impl Grower {
    /// Trace a branch: a bent, drooping, tapering chain of short segments.
    #[allow(clippy::too_many_arguments)]
    fn trace(
        &mut self,
        start: V3,
        dir: V3,
        len: f32,
        r0: f32,
        r_tip: f32,
        taper: f32,
        seg_m: f32,
        bend: f32,
        sag: f32,
        up: f32,
        droop: f32,
        level: u8,
        rng: &mut R,
        emit: bool,
    ) -> Path {
        let n = ((len / seg_m).ceil() as usize).clamp(2, 60);
        let step = len / n as f32;
        let mut pts = vec![start];
        let mut rad = vec![r0];
        let mut cum = vec![0.0];
        let (mut p, mut d) = (start, dir.norm());
        // A limb does not run straight: it turns, and keeps turning the same way for a
        // while before turning back (correlated curvature), so it makes long S-bends and
        // the occasional sharp kink, as a branch that has reached for the light does.
        let amp = match level {
            1 => 0.60,
            2 => 0.55,
            _ => 0.3,
        };
        let mut curv = rng.unit_vec() * amp;
        let mut kink = 0.0_f32;
        for i in 0..n {
            let t = (i + 1) as f32 / n as f32;
            let wobble = rng.unit_vec();
            curv = curv * 0.90 + rng.unit_vec() * (amp * 0.45);
            // Now and then the leader dies back and a side shoot takes over: a sudden bend.
            if rng.u() < 0.035 * level as f32 {
                kink = 0.5 + rng.u();
                curv = curv + rng.unit_vec() * (amp * 2.2);
            }
            kink *= 0.8;
            d = d + curv * (step * (1.0 + kink));
            // Reaching for the light: a branch that has dipped below level turns back up.
            if level >= 1 {
                d.y += (0.30 + 0.5 * (-d.y).max(0.0)) * step;
            }
            // The crown holds the branch in: lean back inside the envelope.
            // A limb never dips below the bare stem: below it, it can only rise or run level.
            if level >= 1 && p.y < self.crown.z0 * 1.05 {
                d.y = d.y.max(0.15);
            }
            let radial = (p.x * p.x + p.z * p.z).sqrt();
            let reach = self.crown.reach(p.y, p.z.atan2(p.x));
            if radial > reach * 1.02 {
                let inward = v3(-p.x, 0.0, -p.z).norm();
                d = d + inward * (0.5 * step);
            }
            d = d + wobble * (bend * step);
            d.y -= 0.45 * sag * len * (0.4 + t) * step;
            d.y += up * 0.5 * step * (1.0 - t);
            d.y -= 0.5 * droop * t * t * step * 2.5;
            // Nothing grows above the crown's top.
            if p.y + d.y * step > self.crown.height * 1.01 {
                d.y = d.y.min(0.0) - 0.05;
            }
            d = d.norm();
            p += d * step;
            let r = r_tip + (r0 - r_tip) * (1.0 - t).max(0.0).powf(taper);
            pts.push(p);
            rad.push(r);
            cum.push(cum[i] + step);
        }
        if emit {
            for i in 1..pts.len() {
                self.segments.push(Segment { a: pts[i - 1], b: pts[i], ra: rad[i - 1], rb: rad[i], level });
            }
        }
        Path { pts, rad, cum }
    }

    /// A broadleaf tree divides into spreading limbs; a conifer keeps one leader to the top.
    fn decurrent(&self) -> bool {
        !matches!(self.crown.habit, Habit::Layered | Habit::Conical | Habit::Fastigiate | Habit::Fan)
    }

    fn trunk(&mut self, spec: &TreeSpec, r_dbh: f32, rng: &mut R) -> Path {
        let h = self.crown.height;
        let seg_m = (h / 38.0).clamp(0.3, 0.8);
        let n = ((h * 1.02 / seg_m).ceil() as usize).clamp(8, 90);
        let step = h * 1.02 / n as f32;
        let mut pts = vec![v3(0.0, 0.0, 0.0)];
        let mut rad = vec![0.0];
        let mut cum = vec![0.0];
        let mut d = (V3::UP + self.lean_axis).norm();
        let mut p = pts[0];
        for i in 0..n {
            let wobble = rng.unit_vec();
            d = d + v3(wobble.x, 0.0, wobble.z) * (self.arch.trunk_wobble * step * 6.0);
            // A trunk rights itself: it grows toward the light.
            d = (d + V3::UP * (0.10 * step)).norm();
            p += d * step;
            pts.push(p);
            cum.push(cum[i] + step);
            rad.push(0.0);
        }
        // Radius by height: taper to the tip, flare at the foot.
        let flare = 0.75 + 0.5 * spec.age;
        let norm = (1.0 - (1.3 / h).min(0.5)).max(0.2);
        for (k, pt) in pts.iter().enumerate() {
            let rel = (pt.y / h).clamp(0.0, 1.0);
            let base = r_dbh * ((1.0 - rel) / norm).max(0.0).powf(self.arch.trunk_taper);
            let foot = 1.0 + flare * 0.55 * (-pt.y / 0.45).exp();
            let mut r = (base * foot).max(0.012);
            if self.decurrent() {
                // The trunk gives way to its limbs: above the first third of the crown it thins
                // fast, and the limbs carry on upward.
                let u = ((rel - 0.50) / 0.45).clamp(0.0, 1.0);
                r *= 1.0 - 0.62 * u * u * (3.0 - 2.0 * u);
            }
            rad[k] = r;
        }
        for i in 1..pts.len() {
            self.segments.push(Segment { a: pts[i - 1], b: pts[i], ra: rad[i - 1], rb: rad[i], level: 0 });
        }
        Path { pts, rad, cum }
    }

    /// How far a straight limb from `start` along `dir` can go before it leaves the
    /// crown envelope (scaled by `margin`), at most `len`.
    fn fit(&self, start: V3, dir: V3, len: f32, margin: f32, floor: f32) -> f32 {
        const STEPS: usize = 14;
        let top = self.crown.height * 1.0;
        for i in 1..=STEPS {
            let s = len * i as f32 / STEPS as f32;
            let p = start + dir * s;
            let radial = (p.x * p.x + p.z * p.z).sqrt();
            if p.y > top || radial > self.crown.reach(p.y, p.z.atan2(p.x)) * margin {
                return (len * (i as f32 - 1.0) / STEPS as f32).max(floor.min(len));
            }
        }
        len
    }

    /// Frame (u, v) perpendicular to `axis`, for phyllotaxis about it.
    fn frame(axis: V3) -> (V3, V3) {
        let u = axis.any_perp();
        (u, axis.cross(u))
    }

    /// A direction `theta` off `axis`, rotated `phi` about it.
    fn spawn_dir(axis: V3, theta: f32, phi: f32) -> V3 {
        let (u, v) = Self::frame(axis);
        (axis * theta.cos() + (u * phi.cos() + v * phi.sin()) * theta.sin()).norm()
    }

    fn limbs(&mut self, trunk: &Path, spec: &TreeSpec, rng: &mut R) {
        let a = self.arch;
        let (z0, h) = (self.crown.z0, self.crown.height);
        let stem = (h - z0).max(0.5);
        let count = ((a.limb_per_m * stem * (0.9 + 0.2 * spec.age)).round() as usize).clamp(3, 140);
        let whorl = a.whorl.max(1) as usize;
        let groups = if a.whorl > 0 { count.div_ceil(whorl) } else { count };
        let phi0 = rng.range(0.0, std::f32::consts::TAU);
        let r_ref = trunk.at((z0 / (h * 1.02)).clamp(0.0, 0.9)).2;
        for g in 0..groups {
            // Even spacing up the crown with jitter; a whorl is all at one height.
            let hrel = (g as f32 + 0.5 + 0.35 * rng.normal().clamp(-1.5, 1.5)) / groups as f32;
            let hrel = hrel.clamp(0.0, 0.985);
            let y = z0 + hrel * stem;
            let t_trunk = (y / (h * 1.02)).clamp(0.0, 0.97);
            let (pos, axis, r_trunk) = trunk.at(t_trunk);
            for w in 0..(if a.whorl > 0 { whorl } else { 1 }) {
                let seed = self.seed.derive_u64(0x11 + (g * 16 + w) as u64);
                let mut lr = R::of(seed);
                let phi = if a.whorl > 0 {
                    phi0 + g as f32 * 0.9 + w as f32 * std::f32::consts::TAU / whorl as f32 + lr.range(-0.2, 0.2)
                } else {
                    phi0 + g as f32 * GOLDEN_ANGLE + lr.range(-0.25, 0.25)
                };
                let theta = a.limb_down.0 + (a.limb_down.1 - a.limb_down.0) * hrel + lr.normal() * 0.10;
                let theta = theta.clamp(0.12, 1.05);
                let dir = Self::spawn_dir(axis, theta, phi);
                // A limb reaches as far as the crown envelope allows in its direction.
                let ask = (self.crown.radius * 3.0).max(2.0);
                let mut len = self.fit(pos, dir, ask, 1.0, 0.25) * a.limb_len * lr.range(0.80, 1.05);
                len = len.clamp(0.25, 16.0);
                let mut r0 = (r_trunk * a.limb_radius).min(r_trunk * 0.9).max(0.010);
                if self.decurrent() {
                    // Big limbs: each is a good fraction of the trunk at the crown's base.
                    let big = r_ref * (0.62 - 0.30 * hrel) * lr.range(0.8, 1.1);
                    r0 = big.min(r_ref * 0.8).max(r0);
                }
                let sag = a.limb_sag * lr.range(0.6, 1.5);
                let path = self.trace(pos, dir, len, r0, 0.012, 0.9, 0.35, 0.10, sag, a.limb_up, a.tip_droop * 0.6, 1, &mut lr, true);
                self.branches(&path, spec, seed, &mut lr);
                if self.decurrent() {
                    self.forks(&path, 2, spec, seed, &mut lr);
                }
                if a.limb_twigs > 0.0 {
                    // Clothed along its length, as a conifer's limb is.
                    self.twigs_on_density(&path, seed.derive("limbtwigs"), a.limb_twigs, &mut lr);
                }
            }
        }
    }

    /// Fill the voids a crown is left with: a few limbs, cut back to the envelope, leave
    /// whole sectors bare. Sample the envelope; where no twig is near, grow a branch from
    /// the nearest wood toward the spot, so the crown is a mass and not a handful of arms.
    fn fill_gaps(&mut self, spec: &TreeSpec) {
        if matches!(self.crown.habit, Habit::Layered | Habit::Conical | Habit::Fastigiate) {
            return;
        }
        let a = self.arch;
        let (z0, h) = (self.crown.z0, self.crown.height);
        let stem = (h - z0).max(0.5);
        let mut rng = R::of(self.seed.derive("fill"));
        let want = (((self.crown.radius * stem).sqrt() * 22.0) as usize).clamp(60, 360);
        let gap = (self.crown.radius * 0.20).max(0.35);
        let mut wood: Vec<(V3, V3, f32)> =
            self.segments.iter().filter(|s| s.level == 1 || s.level == 2).map(|s| (s.a, s.b, s.ra)).collect();
        for i in 0..want {
            let y = z0 + stem * (0.06 + 0.92 * rng.u());
            let az = std::f32::consts::TAU * rng.u();
            let rr = self.crown.reach(y, az) * (0.40 + 0.55 * rng.u());
            let p = v3(rr * az.cos(), y, rr * az.sin());
            let near = self.twigs.iter().map(|t| t.1.dist(p).min(t.0.dist(p))).fold(f32::INFINITY, f32::min);
            if near < gap {
                continue;
            }
            let Some(&(wa, wb, wr)) = wood
                .iter()
                .filter(|w| w.2 > 0.004)
                .min_by(|x, y2| ((x.0 + x.1) * 0.5).dist(p).total_cmp(&((y2.0 + y2.1) * 0.5).dist(p)))
            else {
                continue;
            };
            let start = wa.lerp(wb, 0.5);
            let dist = start.dist(p);
            if dist > 6.5 {
                continue;
            }
            let dir = (p - start).norm();
            let bseed = self.seed.derive_u64(0x9000 + i as u64);
            let mut br = R::of(bseed);
            let blen = self.fit(start, dir, dist.clamp(0.5, 5.5) * 1.1, 1.08, 0.3);
            let r0 = (wr * 0.6).clamp(0.006, 0.07);
            let before = self.segments.len();
            let path = self.trace(start, dir, blen, r0, 0.006, 0.9, 0.22, 0.12, a.branch_sag * 0.7, a.branch_up, a.tip_droop * 0.4, 2, &mut br, true);
            self.twigs_on(&path, bseed, spec, &mut br);
            for seg in &self.segments[before..] {
                if seg.level == 2 {
                    wood.push((seg.a, seg.b, seg.ra));
                }
            }
        }
    }

    /// A limb divides: partway along, one or two stout forks leave it at a wide angle and
    /// carry on bending toward the light, and each may divide again. This is what makes
    /// the framework of an old broadleaf tree: a few great arms, each splitting.
    fn forks(&mut self, limb: &Path, depth: u8, spec: &TreeSpec, seed: Seed, rng: &mut R) {
        if depth == 0 || limb.len() < 1.2 {
            return;
        }
        let a = self.arch;
        let n = if depth == 2 { 2 } else { 1 };
        for i in 0..n {
            let t = rng.range(0.30, 0.72);
            let (pos, axis, r_par) = limb.at(t);
            let theta = rng.range(0.32, 0.72);
            let phi = rng.range(0.0, std::f32::consts::TAU);
            let mut dir = Self::spawn_dir(axis, theta, phi);
            dir.y += 0.35;
            let dir = dir.norm();
            let remaining = limb.len() * (1.0 - t);
            let ask = (remaining * rng.range(0.8, 1.3) + 1.0).max(1.0);
            let len = self.fit(pos, dir, ask, 1.0, 0.4).clamp(0.4, 12.0);
            let r0 = (r_par * 0.72).clamp(0.01, 0.6);
            let fseed = seed.derive_u64(0x7000 + u64::from(depth) * 8 + i as u64);
            let mut fr = R::of(fseed);
            let path = self.trace(pos, dir, len, r0, 0.012, 0.9, 0.35, 0.10, a.limb_sag * 0.8, a.limb_up, a.tip_droop * 0.6, 1, &mut fr, true);
            self.branches(&path, spec, fseed, &mut fr);
            self.forks(&path, depth - 1, spec, fseed, &mut fr);
        }
    }

    fn branches(&mut self, limb: &Path, spec: &TreeSpec, seed: Seed, rng: &mut R) {
        let a = self.arch;
        let len = limb.len();
        let n = ((a.branch_per_m * len * rng.range(0.85, 1.15)).round() as usize).clamp(2, 60);
        let phi0 = rng.range(0.0, std::f32::consts::TAU);
        for i in 0..n {
            let bseed = seed.derive_u64(0x200 + i as u64);
            let mut br = R::of(bseed);
            // Branches start a little way out along the limb and crowd toward the tip.
            let t = (0.14 + 0.86 * ((i as f32 + br.range(0.1, 0.9)) / n as f32)).clamp(0.1, 0.99);
            let (pos, axis, r_parent) = limb.at(t);
            let phi = phi0 + i as f32 * GOLDEN_ANGLE + br.range(-0.3, 0.3);
            let theta = (a.branch_down.0 + (a.branch_down.1 - a.branch_down.0) * t + br.normal() * 0.12).clamp(0.25, 1.0);
            let dir = Self::spawn_dir(axis, theta, phi);
            let mut blen = a.branch_len * limb.len() * (1.0 - t).max(0.12).powf(0.75) * br.range(0.7, 1.2);
            blen = blen.clamp(0.25, 7.0);
            // Keep the tip inside the crown.
            blen = self.fit(pos, dir, blen, 1.06, 0.12);
            let r0 = (r_parent * 0.55).clamp(0.006, 0.25);
            let path = self.trace(pos, dir, blen, r0, 0.006, 0.9, 0.22, 0.14, a.branch_sag * br.range(0.5, 1.4), a.branch_up, a.tip_droop * 0.4, 2, &mut br, true);
            self.twigs_on(&path, bseed, spec, &mut br);
        }
    }

    fn twigs_on(&mut self, branch: &Path, seed: Seed, _spec: &TreeSpec, rng: &mut R) {
        let per_m = self.arch.twig_per_m;
        self.twigs_on_density(branch, seed, per_m, rng);
    }

    fn twigs_on_density(&mut self, branch: &Path, seed: Seed, per_m: f32, rng: &mut R) {
        let a = self.arch;
        let len = branch.len();
        let n = ((per_m * len * rng.range(0.85, 1.15)).round() as usize).clamp(2, 60) + 1;
        let phi0 = rng.range(0.0, std::f32::consts::TAU);
        for i in 0..n {
            let tseed = seed.derive_u64(0x4000 + i as u64);
            let mut tr = R::of(tseed);
            // The last twig is the branch's own tip.
            let tip = i + 1 == n;
            let t = if tip { 1.0 } else { (0.10 + 0.90 * ((i as f32 + tr.range(0.1, 0.9)) / (n - 1) as f32).powf(0.85)).clamp(0.08, 0.98) };
            let (pos, axis, r_parent) = branch.at(t);
            let dir = if tip {
                (axis + tr.unit_vec() * 0.25).norm()
            } else {
                let phi = phi0 + i as f32 * GOLDEN_ANGLE + tr.range(-0.4, 0.4);
                let theta = tr.range(0.40, 0.85);
                Self::spawn_dir(axis, theta, phi)
            };
            let dir = if a.tip_droop >= 1.0 { (dir + v3(0.0, -1.1, 0.0)).norm() } else { dir };
            let tlen = tr.range(a.twig_len.0, a.twig_len.1) * (1.0 - 0.45 * t);
            // A twig outside the crown is cut back to it.
            let tlen = self.fit(pos, dir, tlen, 1.12, 0.05).max(0.05);
            let mid = (pos + dir * (tlen * 0.5) + tr.unit_vec() * (tlen * 0.07) + v3(0.0, -a.tip_droop * 0.05 * tlen, 0.0)).clone();
            let mut end = pos + dir * tlen + v3(0.0, -a.tip_droop * 0.12 * tlen, 0.0);
            let ymax = self.crown.height * 1.01;
            let (mut mid, end_y) = (mid, end.y.min(ymax));
            end.y = end_y;
            mid.y = mid.y.min(ymax);
            // Nothing hangs below the bare stem: a twig that would is simply not grown.
            if end.y < self.crown.z0 * 0.97 || mid.y < self.crown.z0 * 0.97 {
                continue;
            }
            let r0 = (r_parent * 0.6).clamp(0.003, 0.012);
            self.segments.push(Segment { a: pos, b: mid, ra: r0, rb: r0 * 0.7, level: 3 });
            self.segments.push(Segment { a: mid, b: end, ra: r0 * 0.7, rb: 0.0015, level: 3 });
            self.twigs.push((pos, end, r0, 0.0015, tseed.0));
        }
    }
}

/// The state of the foliage on a given day: how much of the crown is in leaf, how
/// far the leaves have turned, how fresh they still are, and how much is in bloom.
pub fn leaf_state(sp: &Species, season: f32) -> (f32, f32, f32, f32) {
    let bloom = sp.bloom.map_or(0.0, |b| {
        let d = (season - b.at).abs().min(1.0 - (season - b.at).abs());
        b.density * (-(d / 0.045).powi(2)).exp()
    });
    if sp.evergreen {
        // An evergreen sheds a little through the autumn and keeps the rest.
        let cover = 1.0 - 0.10 * smoothstep(sp.leaf_fall - 0.10, sp.leaf_fall, season);
        let flush = 1.0 - smoothstep(sp.leaf_out, sp.leaf_out + 0.12, season);
        return (cover, 0.0, flush * 0.5, bloom);
    }
    let up = smoothstep(sp.leaf_out - 0.05, sp.leaf_out + 0.04, season);
    let down = 1.0 - smoothstep(sp.leaf_fall + 0.03, sp.leaf_fall + 0.11, season);
    let autumn = smoothstep(sp.leaf_fall - 0.12, sp.leaf_fall + 0.03, season);
    let flush = 1.0 - smoothstep(sp.leaf_out, sp.leaf_out + 0.10, season);
    (up * down, autumn, flush, bloom)
}

fn byte(x: f32) -> u8 {
    (x.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// Grow a tree at a level of detail (0 nearest .. 3 farthest).
pub fn grow(spec: &TreeSpec, lod: u8) -> Tree {
    let lod = lod.min(3);
    let sp = &SPECIES[spec.species];
    let arch = architecture(sp.habit);
    let seed = Seed::new(spec.seed).derive("tree").derive_u64(spec.species as u64);
    let mut rng = R::of(seed.derive("dims"));

    // --- dimensions -----------------------------------------------------------
    let h = spec.height_m.max(1.5);
    let span = (sp.height_m.1 - sp.height_m.0).max(0.1);
    let f = ((h - sp.height_m.0) / span).clamp(-0.4, 1.4);
    let fc = f.clamp(0.0, 1.0);
    let open = 0.62 + 0.38 * spec.openness.clamp(0.0, 1.0);
    let mut crown_r = (sp.crown_m.0 + (sp.crown_m.1 - sp.crown_m.0) * fc) * open * rng.range(0.92, 1.08);
    crown_r *= 1.0 + 0.2 * (f - 1.0).max(0.0) + 0.25 * f.min(0.0);
    let trunk_r = ((sp.trunk_m.0 + (sp.trunk_m.1 - sp.trunk_m.0) * fc) * (0.62 + 0.38 * spec.age) * rng.range(0.9, 1.1)).max(0.03);
    let natural_base = (sp.clear_stem + (1.0 - spec.openness) * 0.16) * h;
    let z0 = natural_base.max(spec.lift_m).min(h * 0.75);
    let crown = Crown {
        z0,
        height: h,
        radius: crown_r.max(0.4),
        habit: sp.habit,
        lobes: [rng.range(0.4, 1.0), rng.range(0.0, 6.28), rng.range(0.2, 0.8), rng.range(0.0, 6.28)],
        lobing: arch.lobing * (0.6 + 0.8 * rng.u()),
    };
    let lean = {
        let az = rng.range(0.0, std::f32::consts::TAU);
        let amount = arch.lean * rng.u();
        v3(az.cos() * amount.tan(), 0.0, az.sin() * amount.tan())
    };
    let mut grower = Grower {
        arch,
        crown,
        seed,
        segments: Vec::with_capacity(4000),
        twigs: Vec::with_capacity(1800),
        lean_axis: lean,
    };

    // --- wood -------------------------------------------------------------------
    let mut trng = R::of(seed.derive("trunk"));
    let trunk = grower.trunk(spec, trunk_r, &mut trng);
    let mut lrng = R::of(seed.derive("limbs"));
    grower.limbs(&trunk, spec, &mut lrng);
    grower.fill_gaps(spec);

    // Health: a sick tree has lost some of its twigs.
    let keep_twigs = (0.35 + 0.65 * spec.health.clamp(0.0, 1.0)).clamp(0.0, 1.0);

    // --- leaves -------------------------------------------------------------------
    let (cover, autumn, flush, bloom) = leaf_state(sp, spec.season);
    let mut leaves = Vec::new();
    let total_twig: f32 = grower.twigs.iter().map(|t| t.0.dist(t.1)).sum::<f32>().max(0.1);
    let budget = (LEAF_BUDGET[lod as usize] as f32 * (0.35 + 0.65 * sp.leaf_cover.min(5.0) / 5.0)).max(40.0);
    let n_target = budget * cover * keep_twigs;
    if n_target >= 1.0 {
        let per_m = n_target / total_twig;
        // Leaf size: the crown's leaf area, shared out among the leaves drawn.
        let crown_area = std::f32::consts::PI * grower.crown.radius * grower.crown.radius;
        let area_total = 1.9 * sp.leaf_cover * cover * keep_twigs * (0.55 + 0.45 * spec.openness) * crown_area;
        let k = 0.70 * sp.leaf_aspect.min(1.1);
        // A farther level draws fewer, bigger leaves; the ceiling on their size rises with
        // how many fewer, so the crown keeps its mass.
        let stretch = (LEAF_BUDGET[0] as f32 / LEAF_BUDGET[lod as usize] as f32).sqrt();
        let size = (area_total / (n_target * k)).sqrt().clamp(sp.leaf_len_m * 0.6, sp.leaf_len_m * 5.0 * stretch);
        let crown_centre = v3(0.0, 0.5 * (grower.crown.z0 + h), 0.0);
        for (ti, &(a, b, _, _, tseed)) in grower.twigs.iter().enumerate() {
            let mut lr = R::of(Seed::new(tseed).derive("leaves"));
            if lr.u() > keep_twigs {
                continue;
            }
            let len = a.dist(b);
            let expected = len * per_m;
            let count = (expected + lr.u() - 0.5 + 0.5).floor().max(0.0) as usize;
            let axis = (b - a).norm();
            let phi0 = lr.range(0.0, std::f32::consts::TAU);
            for j in 0..count {
                let mut kr = R::of(Seed::new(tseed).derive_u64(0x1EAF + j as u64));
                let s = if count == 1 { 0.5 + 0.45 * kr.u() } else { (j as f32 + kr.range(0.2, 0.9)) / count as f32 };
                let base = a.lerp(b, s.clamp(0.0, 1.0));
                let phi = phi0 + (j + ti) as f32 * GOLDEN_ANGLE + kr.range(-0.3, 0.3);
                let petiole = Grower::spawn_dir(axis, kr.range(0.55, 1.2), phi);
                let outward = {
                    let o = base - crown_centre;
                    o.norm()
                };
                // The blade leans out of the crown, droops a little, and turns to the sky.
                let mut dir = (petiole * 0.8 + outward * 0.35 + v3(0.0, -0.18 * kr.u(), 0.0)).norm();
                if sp.habit == Habit::Weeping {
                    dir = (dir + v3(0.0, -1.2, 0.0)).norm();
                }
                let roll = kr.range(-0.7, 0.7);
                let up_perp = (V3::UP - dir * dir.dot(V3::UP)).norm();
                let side = dir.cross(up_perp).norm();
                let normal = (up_perp * roll.cos() + side * roll.sin()).norm();
                let length = size * (kr.normal() * 0.18).exp() * (0.92 + 0.14 * kr.u());
                let depth = (base.dist(crown_centre) / (grower.crown.radius.max(0.5) * 1.1)).clamp(0.0, 1.0);
                leaves.push(Leaf {
                    pos: {
                        let mut pos = base + dir * (length * 0.10);
                        pos.y = pos.y.max(grower.crown.z0 * 0.97);
                        pos
                    },
                    dir,
                    normal,
                    length,
                    shape: [kr.u8(), kr.u8(), kr.u8(), kr.u8()],
                    tint: [
                        byte(0.5 + 0.5 * kr.normal().clamp(-2.0, 2.0) * 0.5),
                        byte((0.35 + 0.65 * depth) * kr.range(0.8, 1.1)),
                        byte(kr.u().powi(2) * (1.2 - 0.6 * spec.health)),
                        kr.u8(),
                    ],
                });
            }
        }
    }

    // --- level of detail: the wood a far tree does not need ---------------------
    let wood = WOOD_LEVEL[lod as usize];
    let mut segments = grower.segments;
    segments.retain(|s| s.level <= wood);
    let bark = {
        let mut br = R::of(seed.derive("bark"));
        let j = br.range(0.85, 1.15);
        [sp.bark.colour[0] * j, sp.bark.colour[1] * j * br.range(0.96, 1.04), sp.bark.colour[2] * j]
    };
    Tree {
        spec: *spec,
        lod,
        segments,
        leaves,
        height: h,
        crown_radius: grower.crown.radius,
        crown_base: grower.crown.z0,
        trunk_radius: trunk_r,
        bark,
        cover,
        autumn,
        flush,
        bloom,
    }
}

impl R {
    fn u8(&mut self) -> u8 {
        (self.0.next_u64() >> 24) as u8
    }
}
