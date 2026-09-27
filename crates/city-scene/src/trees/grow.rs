//! The grower: turns a form record into wood and leaf cards.
//!
//! One [`Grower`] builds one prototype at unit height. Everything it draws is
//! read from the species table and the canopy form's
//! [`Architecture`](crate::trees::forms::Architecture), so a tree's silhouette,
//! its colour and its size cannot disagree. The form-specific behaviours — the
//! willow's hanging shoots, the pine's lean and flat needle plates, the
//! banyan's aerial roots — are fields on the form record interpreted here, so
//! adding a form never means editing the grower.

use super::cards::{CARD_WINDOW, card_tile_m, luma, target_coverage};
use super::forms::{self, Architecture, Layout};
use super::{MAX_CARDS, MIN_CARDS, PRESENTED_FRACTION, TAU, TreePrototype};
use crate::math::{Rng, Vec2, Vec3};
use crate::mesh::MeshBuilder;
use crate::species::Species;

/// Everything one prototype needs, derived from the species table alone.
pub(super) struct Plan {
    /// Crown radius in unit-height space. The crown *is* this envelope, so a
    /// canopy's silhouette is a consequence of the table's `crown_m` and
    /// `clear_stem` and cannot drift away from them.
    radius: f32,
    /// Unit height of the crown's foot.
    base: f32,
    /// Unit height of the crown's top.
    top: f32,
    /// Trunk radius in unit-height space.
    trunk_r: f32,
    /// One card's physical size, in metres, and the same in unit space.
    tile_m: f32,
    card: f32,
    /// Cards the canopy is built from.
    cards: usize,
    /// Total weight of every foliage cluster, so the card budget can be divided
    /// between clusters in proportion to how much each one carries.
    cluster_weight: f32,
    /// Vertical flutes around a trunk of this species, from its `fissure`.
    flutes: f32,
    phase: f32,
}

impl Plan {
    pub(super) fn new(species: &Species, prototype: &TreePrototype, rng: &mut Rng) -> Self {
        // Unit height is the invariant, so every linear dimension is a real
        // dimension divided by the tree's real height. Nothing is authored in
        // scene units.
        let height = prototype.height.max(0.1);
        let radius = (prototype.crown / height) * 0.94;
        // The crown starts below the first branch: a `水杉`'s `clear_stem` is
        // 0.62, meaning foliage does not begin until 62% of its height, and the
        // 0.8 lets a whorl sit a little lower than the first true branch without
        // the crown touching the ground.
        let base = (species.clear_stem * 0.80).clamp(0.10, 0.86);
        let tile_m = card_tile_m(species);
        let card = tile_m / height;
        let arch = forms::architecture(species.canopy);
        // Card count from the crown's *silhouette*, because that is what a viewer
        // has to see through. A foliage card's normal is biased outward — the
        // face the world sees is the face the card shows — so a card presents
        // about three quarters of its area to any given view; and only
        // `window * coverage` of that area is opaque leaf. A canopy needs
        // several layers of cards before the sky stops showing through, and
        // `density` is the species' own statement about how many: a `梧桐` at
        // 0.54 is built to be see-through and a `榕树` at 0.86 is built to be a
        // solid glossy dome.
        let silhouette = std::f32::consts::PI * radius * radius;
        let presented = PRESENTED_FRACTION
            * card
            * card
            * CARD_WINDOW
            * CARD_WINDOW
            * target_coverage(species);
        let target_layer = 0.6 + 3.3 * species.density;
        let cards = (target_layer * silhouette / presented.max(1.0e-9))
            .round()
            .clamp(MIN_CARDS as f32, MAX_CARDS as f32) as usize;
        // Clusters: one at every secondary branch's tip, one partway along it,
        // and one at the primary limb's own tip. Counted up front so the card
        // budget can be divided between them in one pass.
        let primaries = match arch.layout {
            Layout::Single => arch.primary,
            Layout::Whorled => arch.tiers * arch.per_tier,
        };
        let cluster_weight = primaries as f32 * (0.55 + arch.fork as f32 * 1.45);
        Self {
            radius,
            base,
            top: 1.0,
            trunk_r: (prototype.trunk / height).max(0.0015),
            tile_m,
            card,
            cards,
            cluster_weight,
            // Bark's `fissure` is the number of ridges and how deep they are.
            // A pine at 0.90 has eight deep flutes; a `梧桐` at 0.18 has three
            // that are barely there. An *integer*, because a fractional flute
            // count leaves a seam where the ring closes on itself.
            flutes: (2.0 + 7.0 * species.bark.fissure).round(),
            phase: rng.range(0.0, TAU),
        }
    }

    /// Crown half-width at a unit height, which is the species' own profile.
    fn radius_at(&self, arch: &Architecture, y: f32) -> f32 {
        let t = ((y - self.base) / (self.top - self.base)).clamp(0.0, 1.0);
        self.radius * (arch.profile)(t)
    }
}

/// One prototype under construction. Everything it writes is read from the
/// species record, so a tree's silhouette, its colour and its size cannot
/// disagree.
pub(super) struct Grower<'a, 'b> {
    pub(super) species: &'a Species,
    plan: Plan,
    arch: Architecture,
    rng: Rng,
    bark: &'b str,
    leaf: &'b str,
    builder: &'b mut MeshBuilder,
    /// Points partway along the primary limbs, recorded while the crown is
    /// grown so the banyan's aerial roots can drop from real limbs.
    limb_points: Vec<Vec3>,
}

impl<'a, 'b> Grower<'a, 'b> {
    pub(super) fn new(
        prototype: &'b TreePrototype,
        seed: u32,
        bark: &'b str,
        leaf: &'b str,
        builder: &'b mut MeshBuilder,
    ) -> Self {
        let species = prototype.species;
        let mut rng = Rng::new(seed);
        let plan = Plan::new(species, prototype, &mut rng);
        let arch = forms::architecture(species.canopy);
        Self {
            species,
            plan,
            arch,
            rng,
            bark,
            leaf,
            builder,
            limb_points: Vec::new(),
        }
    }

    pub(super) fn build(&mut self) {
        match self.arch.layout {
            Layout::Single => self.single_trunk(),
            Layout::Whorled => self.whorled_trunk(),
        }
        // A banyan drops its aerial roots after the crown exists, from points
        // recorded on the major limbs.
        if self.arch.aerial > 0 {
            self.aerial_roots();
        }
        // A tree is drawn to its canopy, so the top of the last card is the top
        // of the tree. The geometry is unit height and the instance scale is the
        // real metres; that invariant is what lets one mesh be any tree.
    }

    // -- the wood ----------------------------------------------------------

    /// A tapered, slightly bowed, fluted tube. The branch primitive for the
    /// whole tree: trunk, limbs, secondaries and aerial roots are all this.
    #[allow(clippy::too_many_arguments)]
    fn limb(
        &mut self,
        from: Vec3,
        to: Vec3,
        r0: f32,
        r1: f32,
        sides: u8,
        rings: u8,
        bow: f32,
        flare: f32,
    ) {
        let axis = to - from;
        let length = axis.length();
        if !length.is_finite() || length < 1.0e-4 {
            return;
        }
        let axis_n = axis / length;
        let reference = if axis_n.y.abs() < 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let side_a = reference.cross(axis_n).normalized_or_up();
        let side_b = axis_n.cross(side_a);
        let sides = sides.max(3) as usize;
        let rings = rings.max(2) as usize;
        let depth = 0.04 + 0.20 * self.species.bark.fissure;
        let mut previous: Vec<(Vec3, Vec3)> = Vec::with_capacity(sides);
        for ring in 0..rings {
            let t = ring as f32 / (rings - 1) as f32;
            let centre = from
                + axis * t
                // The bow is a parabola, so a limb that leaves the trunk steeply
                // and flattens at the tip is one segment and not two.
                + Vec3::new(0.0, bow * 4.0 * t * (1.0 - t), 0.0);
            let radius = (r0 + (r1 - r0) * t) * (1.0 + flare * (1.0 - t).powi(3));
            let mut current = Vec::with_capacity(sides);
            for side in 0..sides {
                let a = side as f32 / sides as f32 * TAU;
                // A trunk's ridges, and a limb's: a fluted tube is not a smooth
                // cylinder, and the number and depth of the flutes are the
                // species' own `fissure`.
                let flute = 1.0 + depth * (self.plan.flutes * a + 1.9 * t).cos();
                let radial = side_a * a.cos() + side_b * a.sin();
                current.push((centre + radial * (radius * flute), radial));
            }
            if ring > 0 {
                for side in 0..sides {
                    let next = (side + 1) % sides;
                    let a = previous[side];
                    let b = current[side];
                    let c = current[next];
                    let d = previous[next];
                    // Per-facet value from the bark's weathering, so a trunk is
                    // not one flat grey cylinder.
                    let facet = 0.90
                        + 0.10 * ((side * 7 + ring * 13) as f32 * 0.618).sin();
                    let colour = self.bark_colour(facet);
                    self.builder.quad_shaded(
                        self.bark,
                        (a.0, a.1),
                        (b.0, b.1),
                        (c.0, c.1),
                        (d.0, d.1),
                        Some(colour),
                    );
                }
            }
            previous = current;
        }
    }

    /// A point partway along a bowed limb, used to hang secondaries off it.
    fn point_on(&self, from: Vec3, axis: Vec3, bow: f32, t: f32) -> Vec3 {
        from + axis * t + Vec3::new(0.0, bow * 4.0 * t * (1.0 - t), 0.0)
    }

    /// Bark colour: the species' own reflectance, greying with age.
    fn bark_colour(&self, facet: f32) -> [f32; 3] {
        let base = self.species.bark.colour;
        let grey = luma(base);
        // Weathering greys a bark: the colour a bark stops being is the colour it
        // goes, and the table says how far this species has gone.
        let toward = self.species.bark.weathering * 0.45;
        [
            (base[0] + (grey - base[0]) * toward) * facet,
            (base[1] + (grey - base[1]) * toward) * facet,
            (base[2] + (grey - base[2]) * toward) * facet,
        ]
    }

    /// A primary limb and its secondaries, and the foliage on both.
    fn branch(&mut self, from: Vec3, to: Vec3, radius: f32, level: u8) {
        let axis = to - from;
        let length = axis.length();
        if !length.is_finite() || length < 1.0e-4 {
            return;
        }
        let dir = axis / length;
        // The bend: upward for a climbing limb, downward for a drooping one, and
        // the difference between a cedar's tier and a metasequoia's.
        let bow = (self.arch.climb - 0.30) * length * 0.22 - self.arch.droop * length * 0.30;
        // Four sides on a primary, three on a secondary. A primary limb is tens
        // of centimetres of real wood; a secondary is one, and three facets is
        // all a facet budget can spend on it honestly.
        let sides: u8 = if level == 0 { 4 } else { 3 };
        self.limb(from, to, radius, radius * 0.5, sides, 2, bow, 0.0);
        if level == 0 {
            if self.arch.aerial > 0 {
                // A point partway out along the major limb, for a banyan's
                // aerial root to drop from.
                self.limb_points.push(self.point_on(from, axis, bow, 0.55));
            }
            for index in 0..self.arch.fork {
                let share = (index as f32 + 0.5) / self.arch.fork as f32;
                let t = 0.38 + 0.60 * share + self.rng.range(-0.07, 0.07);
                let origin = self.point_on(from, axis, bow, t);
                let reference = if dir.y.abs() < 0.9 {
                    Vec3::new(0.0, 1.0, 0.0)
                } else {
                    Vec3::new(1.0, 0.0, 0.0)
                };
                let a1 = reference.cross(dir).normalized_or_up();
                let a2 = dir.cross(a1);
                let angle = self.rng.range(0.0, TAU);
                // A branch forks in a plane, and the plane rotates as it climbs.
                let splay = self.arch.fork_reach * self.rng.range(0.55, 1.0);
                let mut child = (dir * (1.0 - 0.5 * splay)
                    + (a1 * angle.cos() + a2 * angle.sin()) * splay)
                    .normalized_or_up();
                let reach = length * self.rng.range(0.38, 0.58)
                    * (1.0 + 0.30 * self.arch.hang);
                if self.arch.hang > 0.0 {
                    // The willow's curtain: every second-order shoot turns hard
                    // downward from its parent's axis, and the fine lanceolate
                    // cards hang along the fall.
                    child = (child * (1.0 - 0.55 * self.arch.hang)
                        + Vec3::new(0.0, -0.85 * self.arch.hang, 0.0))
                        .normalized_or_up();
                }
                self.branch(origin, origin + child * reach, radius * 0.5, 1);
            }
            self.foliage(to, dir, 0.55);
        } else {
            // A secondary carries foliage at its tip and partway back, which is
            // what stops a canopy from being a shell of cards on the outside with
            // a hole through the middle of it.
            self.foliage(to, dir, 1.0);
            let origin = self.point_on(from, axis, bow, 0.42);
            self.foliage(origin, dir, 0.45);
        }
    }

    // -- layouts ------------------------------------------------------------

    /// One trunk, then limbs that arch out of it.
    fn single_trunk(&mut self) {
        // A pine leans 5-15 degrees and the crown carries the lean; every other
        // form leans only enough to not be a lamp post.
        let lean = if self.arch.lean > 0.0 {
            self.rng.direction() * self.arch.lean * self.rng.range(0.45, 1.50)
        } else {
            self.rng.direction() * 0.030
        };
        let leader = self.plan.base + (self.plan.top - self.plan.base) * 0.90;
        // The leader: a real trunk with a real flare at the foot, thinning as it
        // climbs.
        self.limb(
            Vec3::new(0.0, 0.0, 0.0),
            self.spine(lean, leader),
            self.plan.trunk_r,
            self.plan.trunk_r * 0.26,
            self.trunk_sides(),
            3,
            0.0,
            0.9,
        );
        for index in 0..self.arch.primary {
            let share = ((index as f32 + 0.5 + self.rng.range(-0.28, 0.28))
                / self.arch.primary as f32)
                .clamp(0.0, 1.0);
            let height = self.plan.base + share.powf(self.arch.attach_bias)
                * (self.plan.top - self.plan.base)
                * 0.90;
            let attach = self.spine(lean, height);
            self.limb_from(lean, attach, height, self.plan.trunk_r, index);
        }
    }

    /// Whorled tiers on a clear leader: a dawn redwood, a fir, a deodar.
    fn whorled_trunk(&mut self) {
        let lean = self.rng.direction() * 0.022;
        let spine_top = self.plan.top * 0.995;
        self.limb(
            Vec3::new(0.0, 0.0, 0.0),
            self.spine(lean, spine_top),
            self.plan.trunk_r,
            self.plan.trunk_r * 0.20,
            self.trunk_sides(),
            4,
            0.0,
            1.1,
        );
        let tiers = self.arch.tiers;
        let per_tier = self.arch.per_tier;
        for tier in 0..tiers {
            // The lowest whorl sits just above the crown's foot and the top one
            // reaches the leader's tip, so the spire is a spire and not a
            // lollipop on a stick.
            let share = (tier as f32 + 0.90) / tiers as f32;
            let height = self.plan.base + share * (self.plan.top - self.plan.base);
            // Each whorl is rotated off the one below by the golden angle, so the
            // tiers never line up into a cross and a tree never reads as a
            // compass rose.
            let phase = self.plan.phase + tier as f32 * 2.399_963;
            for branch in 0..per_tier {
                let angle = phase
                    + branch as f32 / per_tier as f32 * TAU
                    + self.rng.range(-0.16, 0.16);
                let attach = self.spine(lean, height);
                let reach = self.plan.radius_at(&self.arch, height) * self.arch.reach
                    * self.rng.range(0.82, 1.0);
                // A metasequoia's branch leaves the trunk nearly horizontally and
                // lifts a little; a deodar's runs out flat and drops at the tip.
                let lift = self.arch.climb * reach * self.rng.range(0.35, 1.0);
                let tip = attach
                    + Vec3::new(angle.cos() * reach, lift, angle.sin() * reach);
                self.branch(attach, tip, self.plan.trunk_r * 0.30, 0);
            }
        }
    }

    /// The banyan's aerial roots: thin, slightly flared columns dropping
    /// straight from the major limbs to the ground, evenly spread over the
    /// recorded limb points so they ring the crown rather than bunching.
    fn aerial_roots(&mut self) {
        let wanted = self.arch.aerial.min(self.limb_points.len());
        if wanted == 0 {
            return;
        }
        let stride = self.limb_points.len() as f32 / wanted as f32;
        for root in 0..wanted {
            let index = (((root as f32 + 0.5) * stride) as usize).min(self.limb_points.len() - 1);
            let from = self.limb_points[index];
            let r = self.plan.trunk_r * self.rng.range(0.14, 0.24);
            // A root column reads as a root because it flares at the foot.
            self.limb(
                from,
                Vec3::new(from.x, 0.0, from.z),
                r,
                r * 1.6,
                3,
                3,
                0.0,
                0.6,
            );
        }
    }

    /// One primary limb: reached from the trunk, aimed at the crown profile's
    /// surface at the height it will end up at.
    fn limb_from(&mut self, lean: Vec2, attach: Vec3, height: f32, radius: f32, index: usize) {
        let primary = self.arch.primary;
        // The phase carries the lean, so a leaning trunk does not produce a
        // symmetric crown.
        let angle = self.plan.phase
            + lean.angle() * 0.55
            + index as f32 / primary as f32 * TAU
            + self.rng.range(-0.26, 0.26);
        let tip_y = height + self.arch.climb * (self.plan.top - height) * self.rng.range(0.70, 1.0);
        let reach = self.plan.radius_at(&self.arch, tip_y) * self.arch.reach
            * self.rng.range(0.84, 1.0);
        let tip = attach
            + Vec3::new(angle.cos() * reach, tip_y - height, angle.sin() * reach);
        let thickness = radius * self.rng.range(0.72, 1.0);
        self.branch(attach, tip, thickness, 0);
    }

    /// The leader's centre line at a height. A real trunk is very nearly
    /// straight, which is not the same as exactly straight.
    fn spine(&self, lean: Vec2, height: f32) -> Vec3 {
        let t = (height / self.plan.top.max(0.01)).clamp(0.0, 1.0);
        Vec3::new(lean.x * t * t, height, lean.y * t * t)
    }

    pub(super) fn trunk_sides(&self) -> u8 {
        // Enough sides to read as round at arm's length, and never fewer than the
        // flutes need or the ridges alias into a star.
        let by_girth = (self.plan.trunk_r * 420.0).clamp(6.0, 12.0);
        by_girth.max((self.plan.flutes * 2.0).ceil().clamp(4.0, 16.0)) as u8
    }

    // -- the foliage --------------------------------------------------------

    /// The crown is an envelope, and the branches are grown to fill it. Every
    /// card is placed inside that envelope, so a canopy's silhouette is the
    /// species' profile and not whatever the random branch lengths happened to
    /// add up to. This is the constraint that makes a `水杉` a spire.
    fn clamp_into_crown(&self, point: &mut Vec3) {
        let limit = self.plan.radius_at(&self.arch, point.y);
        let flat = Vec2::new(point.x, point.z).length();
        if flat > limit && limit > 1.0e-4 {
            let scale = limit / flat;
            point.x *= scale;
            point.z *= scale;
        }
        point.y = point.y.clamp(self.plan.base, self.plan.top);
    }

    /// A cluster of leaf cards on a shoot, `weight` share of the canopy's card
    /// budget.
    ///
    /// The budget is *divided*, not spent: a cluster that arrives late in the
    /// build gets the same share as one that arrived first, because a canopy
    /// whose upper half is empty is a different silhouette and not a cheaper
    /// one.
    fn foliage(&mut self, tip: Vec3, shoot: Vec3, weight: f32) {
        let count = ((self.plan.cards as f32 * weight) / self.plan.cluster_weight.max(0.1))
            .round()
            .max(1.0) as usize;
        // The cluster is a little over a card across, so consecutive clusters on
        // a branch merge into one mass of foliage instead of reading as beads.
        let spread = self.plan.card * 1.35;
        let shoot = shoot.normalized_or_up();
        for _ in 0..count {
            // A leaf is arranged on a shoot, not on a sphere: the offset from the
            // tip is along the shoot and around it, not uniform in a ball.
            let along = self.rng.range(-0.55, 0.85);
            let mut out = self.rng.sphere();
            if self.arch.tuft > 0.0 {
                // The pine's needle plates: a cluster squashed vertically reads
                // at a distance as a flat layer of foliage, which is what a
                // pine's shoot tips actually are.
                out.y *= 1.0 - self.arch.tuft;
            }
            let mut centre = tip
                + shoot * (along * spread)
                + out * (spread * self.rng.range(0.10, 1.05));
            self.clamp_into_crown(&mut centre);
            self.card(centre, shoot);
        }
    }

    /// One alpha-cut leaf card.
    ///
    /// Three things stop a canopy of 900 quads from reading as wallpaper: the
    /// card is scaled, spun and tinted per card; it samples a different random
    /// window of the texture; and its plane contains the shoot it is painted on,
    /// so the leaf spray stands up along the twig — or, on a hanging shoot,
    /// pours down it.
    fn card(&mut self, centre: Vec3, shoot: Vec3) {
        let size = self.plan.card * self.rng.range(0.70, 1.32);
        // A different crop of the tile every time. Foliage has no structure at
        // any scale, so any window of a leaf scatter is a valid leaf scatter, and
        // the wrap makes the crop's edges invisible.
        let window = self.plan.tile_m * CARD_WINDOW;
        let slack = (self.plan.tile_m - window).max(0.0);
        let origin_u = self.rng.range(0.0, slack);
        let origin_v = self.rng.range(0.0, slack);

        // The card's face is mostly outward, because that is the face the world
        // sees, with a real tilt either way.
        let outward = Vec3::new(centre.x, 0.0, centre.z);
        let outward = if outward.length() > 1.0e-4 {
            outward.normalized_or_up()
        } else {
            self.rng.sphere()
        };
        let mut normal = (outward * 0.72 + self.rng.sphere() * 0.58).normalized_or_up();
        // A card lying exactly flat is edge-on to every viewer: geometry that
        // costs a quad and shows nothing.
        if normal.y.abs() < 0.16 {
            normal = (normal + Vec3::new(0.0, 0.40, 0.0)).normalized_or_up();
        }
        // Orthonormalise: `axis_v` is the shoot, in the card's plane.
        let mut axis_v = shoot - normal * shoot.dot(normal);
        if axis_v.length() < 0.20 {
            axis_v = normal.cross(Vec3::new(0.0, 1.0, 0.0));
            if axis_v.length() < 1.0e-4 {
                axis_v = normal.cross(Vec3::new(1.0, 0.0, 0.0));
            }
        }
        let axis_v = axis_v.normalized_or_up();
        let axis_u = axis_v.cross(normal).normalized_or_up();
        let normal = axis_u.cross(axis_v).normalized_or_up();

        // A foliage canopy is not a set of mirrors. A thin leaf transmits and
        // scatters, so the effective normal of a leaf cluster sits between the
        // card's own normal and the sky. A quarter of the way is about as far
        // as the evidence supports, and it is the only shading bias in the file.
        let shading = (normal * 0.76 + Vec3::new(0.0, 0.24, 0.0)).normalized_or_up();
        // Leaf-to-leaf variation, and nothing else. It is deliberately
        // uncorrelated with where the card is: a canopy that is darker deep
        // inside is painted occlusion, and this renderer has real shadows.
        let value = self.rng.range(0.78, 1.0);
        let warm = self.rng.range(-0.035, 0.035);
        let tint = [value * (1.0 + warm), value, value * (1.0 - warm)];
        let half = size * 0.5;
        let uv = |a: f32, b: f32| (origin_u + a * window, origin_v + b * window);
        self.builder.quad_uv_shaded(
            self.leaf,
            [
                (centre - axis_u * half - axis_v * half, shading),
                (centre + axis_u * half - axis_v * half, shading),
                (centre + axis_u * half + axis_v * half, shading),
                (centre - axis_u * half + axis_v * half, shading),
            ],
            [
                uv(0.0, 0.0),
                uv(1.0, 0.0),
                uv(1.0, 1.0),
                uv(0.0, 1.0),
            ],
            Some(tint),
        );
    }
}
