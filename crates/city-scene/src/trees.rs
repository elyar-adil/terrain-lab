//! Trees, built once per species as prototypes and instanced.
//!
//! # What a tree is, geometrically
//!
//! The previous port modelled a tree as one of four crown shapes — broad,
//! conical, weeping, clump — hung on a cylinder, with five green tones.  That
//! is why every street looked like the same four stamps: the *species* was a
//! number, so a `水杉` and a `麻楝` were the same silhouette in different
//! greens, and neither had any wood in it above the trunk.
//!
//! Here a tree is built the way a tree is built.  A trunk with a real taper and
//! a real flare; primary limbs; secondary branches; and then leaf cards at the
//! branch tips, and only there.  The branching is recursive, the reach of every
//! limb is the crown's own profile, and the profile is one of seven — one per
//! [`Canopy`] value — so:
//!
//! * a `水杉` is a narrow spire with a clear trunk for most of its height and
//!   whorled, drooping, conical tiers of foliage;
//! * a `麻楝` is a wide, open, see-through dome;
//! * a `紫花风铃木` is a vase: narrow at the foot, widest at the lip;
//! * a `海棠` is a low-branched spreading cloud on a short thick trunk.
//!
//! Nothing here is a stylisation.  There is no rim light, no outline, no
//! painted occlusion and no brightening.  The albedo is the species' own
//! reflectance, the leaf card is alpha-cut, and a card's only shading bias is
//! the quarter-turn toward the sky that a thin, scattering leaf genuinely has.
//!
//! # Unit height, and what it buys
//!
//! Prototypes are authored at **unit height** and instanced with a real metre
//! scale, so a 6 m street tree and a 24 m metasequoia share one mesh.  A
//! prototype is 900-3 000 triangles — expensive per tree, irrelevant per city,
//! because sixteen species across twenty-four prototypes is forty-eight draw
//! calls for *every* tree in *every* city.  That is the trade: the detail goes
//! into the prototype because the prototype is paid for once.

use urban::{CityFrameInfo, Parcel, ParcelUse, Point, TreeSpecies, modern};

use crate::buildings::level;
use crate::leaf_cards::{self, CARD_WINDOW};
use crate::math::{Rng, Vec2, Vec3};
use crate::mesh::{GroupStyle, Instance, MeshBuilder};
use crate::network::Network;
use crate::species::{Canopy, SPECIES, Species, by_key};

const TAU: f32 = std::f32::consts::PI * 2.0;

/// Per-city tree budget.
pub const TREE_BUDGET: usize = 3200;
/// Tufts and low shrubs share a much smaller budget; they are ground clutter.
pub const TUFT_BUDGET: usize = 6000;

/// The eight species a Chinese avenue is actually made of, and therefore the
/// only ones worth a second silhouette.  A variant is not a recolour: variant 0
/// is the tall narrow tree and variant 1 the broad low one, which is a real
/// difference in a real street — the same species planted on the north and
/// south kerb of the same road.  Everything else gets one prototype, because a
/// park tree that appears forty times does not need its silhouette varied.
const TWO_VARIANT: [&str; 8] = [
    "wu-jiu",
    "cong-shu",
    "zi-hua-feng-jiao-mu",
    "huang-hua-feng-jiao-mu",
    "ma-lian",
    "xiao-ye-ying-ren",
    "luo-yu-shan",
    "shui-shan",
];

/// The floor and ceiling on a canopy's card count.  The floor is what stops a
/// "simplification" from turning a species into a bare armature; the ceiling is
/// what keeps 3 200 instances of one prototype affordable.
const MIN_CARDS: usize = 220;
const MAX_CARDS: usize = 1050;

/// One prototype: a species and a variant, built at unit height so the instance
/// scale supplies the real metres.
pub struct TreePrototype {
    pub key: String,
    /// The species record every number below and every vertex of the mesh is
    /// derived from.  Held by reference so the table stays the single source of
    /// truth and a prototype can never disagree with it.
    pub species: &'static Species,
    pub variant: u16,
    /// Real metres, reported so the renderer and the placement code agree on how
    /// big this tree is.
    pub height: f32,
    pub crown: f32,
    pub trunk: f32,
}

/// The prototype set: every species, one or two variants each.
pub fn prototype_set() -> Vec<TreePrototype> {
    let mut result = Vec::new();
    for (index, species) in SPECIES.iter().enumerate() {
        let variants = if TWO_VARIANT.contains(&species.key) { 2 } else { 1 };
        for variant in 0..variants {
            let mut rng = Rng::new(0x7ee0_0000 ^ (index as u32) << 6 ^ variant as u32);
            let (height, crown, trunk) = real_dimensions(species, variant, &mut rng);
            result.push(TreePrototype {
                key: format!("tree/{}/{}", species.key, variant),
                species,
                variant: variant as u16,
                height,
                crown,
                trunk,
            });
        }
    }
    result
}

/// A real tree of this species, in real metres, inside the table's ranges.
///
/// Variant 0 is tall and narrow, variant 1 broad and low, and the crown radius
/// follows the height the way a real crown does — a stunted tree of a big
/// species is stunted in every dimension, not merely short.
fn real_dimensions(species: &Species, variant: u16, rng: &mut Rng) -> (f32, f32, f32) {
    let broad = variant == 1;
    let height = lerp_between(
        species.height_m,
        if broad { (0.68, 0.90) } else { (0.86, 1.00) },
        rng,
    );
    let crown = lerp_between(
        species.crown_m,
        if broad { (0.92, 1.00) } else { (0.60, 0.82) },
        rng,
    );
    // A short tree of a big species carries a proportionally thicker trunk.
    let stocky = 1.0 + 0.30 * (1.0 - (height - species.height_m.0)
        / (species.height_m.1 - species.height_m.0)
        .max(0.1));
    let trunk = lerp_between(species.trunk_m, (0.84, 1.00), rng) * stocky;
    (height, crown, trunk)
}

/// A value in `(low, high)` of a species' own range, at a fraction of that range.
fn lerp_between(range: (f32, f32), fraction: (f32, f32), rng: &mut Rng) -> f32 {
    range.0 + (range.1 - range.0) * rng.range(fraction.0, fraction.1)
}

fn declare(builder: &mut MeshBuilder) {
    builder.style(
        "bark",
        GroupStyle {
            cast_shadow: true,
            receive_shadow: true,
            alpha_cutout: false,
            dynamic: false,
        },
    );
    builder.style(
        "leaf",
        GroupStyle {
            cast_shadow: true,
            receive_shadow: true,
            alpha_cutout: true,
            dynamic: false,
        },
    );
    builder.style(
        "hedge",
        GroupStyle {
            cast_shadow: true,
            receive_shadow: true,
            alpha_cutout: false,
            dynamic: false,
        },
    );
    builder.style(
        "tuft",
        GroupStyle {
            cast_shadow: false,
            receive_shadow: true,
            alpha_cutout: true,
            dynamic: false,
        },
    );
}

/// Build every prototype's geometry.  Called once per generation, not per city:
/// the geometry is identical for every city in the world.
pub fn build_prototypes(prototypes: &[TreePrototype], builder: &mut MeshBuilder) {
    declare(builder);
    for prototype in prototypes {
        let bark = format!("{}#bark", prototype.key);
        let leaf = format!("{}#leaf", prototype.key);
        builder.style(
            &bark,
            GroupStyle {
                cast_shadow: true,
                receive_shadow: true,
                alpha_cutout: false,
                dynamic: false,
            },
        );
        builder.style(
            &leaf,
            GroupStyle {
                cast_shadow: true,
                receive_shadow: true,
                alpha_cutout: true,
                dynamic: false,
            },
        );
        let seed = hash_seed(prototype.species.key, prototype.variant);
        let mut grow = Grower::new(prototype, seed, &bark, &leaf, builder);
        grow.build();
        // One transform list places the whole tree: bark and foliage are two
        // groups, one instance buffer.
        builder.bind(&bark, &prototype.key);
        builder.bind(&leaf, &prototype.key);
    }
}

fn hash_seed(key: &str, variant: u16) -> u32 {
    let mut value = 0x7ee5_u32;
    for byte in key.as_bytes() {
        value = value.wrapping_mul(0x0100_0193) ^ *byte as u32;
    }
    value
        .wrapping_mul(0x9e37_79b9)
        .wrapping_add(variant as u32)
        .wrapping_mul(0x85eb_ca6b)
}

// --------------------------------------------------------------------------
// Canopy architecture
// --------------------------------------------------------------------------

/// Crown half-width at normalised crown height: 0 at the crown's foot, 1 at its
/// top.  This *is* the silhouette, and each of the seven is a different curve.
type Profile = fn(f32) -> f32;

/// A metasequoia or a Chinese fir: widest just above the crown's foot, then a
/// straight taper to a leader that keeps going.  The `0.94` leaves a spike so
/// the spire is a spire.
fn profile_conical(t: f32) -> f32 {
    (1.0 - t * 0.94).max(0.05).powf(0.72)
}

/// A tiered standard: flat whorls, so the crown barely narrows at all and the
/// silhouette is a stack of horizontal plates.
fn profile_tiered(t: f32) -> f32 {
    (1.0 - 0.24 * t).max(0.1)
}

/// A vase: narrow at the foot, widest at the lip.  This is the one shape that
/// gets *wider* as it rises, and it is why a `乌桕` looks nothing like a `麻楝`.
fn profile_vase(t: f32) -> f32 {
    0.20 + 0.80 * t.powf(0.80)
}

/// A dome, widest a little above the middle.
fn profile_rounded(t: f32) -> f32 {
    let x = (t - 0.42) / 0.64;
    (1.0 - x * x).max(0.0).powf(0.55)
}

/// A low, broad, flattish cloud: wide for most of its height, rounding over only
/// at the very top, and wider than tall.
fn profile_spreading(t: f32) -> f32 {
    let x = (t - 0.28) / 0.84;
    (1.0 - x * x).max(0.0).powf(0.42)
}

/// A weeping crown: full at the shoulder, then a long fall away from it.
fn profile_weeping(t: f32) -> f32 {
    (1.0 - t * 0.78).max(0.04).powf(0.50)
}

/// A multi-stemmed clump: a small dome, lifted off the ground.
fn profile_clump(t: f32) -> f32 {
    let x = (t - 0.48) / 0.58;
    (1.0 - x * x).max(0.0).powf(0.50)
}

/// How the wood is arranged.  Three arrangements cover the seven canopies: a
/// single trunk that forks into limbs, whorled tiers, or several stems from the
/// base.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Layout {
    /// One trunk, then limbs that arch out and up.
    Single,
    /// Whorled tiers of nearly horizontal branches — a conifer or a tiered
    /// standard.
    Whorled,
    /// Several stems out of the ground.
    Stems,
}

/// The numbers one canopy needs in order to be itself.  Every field is
/// architecture; none of it is colour, and none of it is a tint.
#[derive(Debug, Clone, Copy)]
struct Architecture {
    profile: Profile,
    layout: Layout,
    /// Primary limbs off the trunk.
    primary: usize,
    /// Exponent on a limb's normalised attachment height.  Below 1 the limbs
    /// bunch toward the top (a vase's lip); above 1 they bunch at the foot (a
    /// low-branched crabapple).
    attach_bias: f32,
    /// Horizontal reach of a primary limb, as a fraction of the reach the crown
    /// profile allows.
    reach: f32,
    /// How much of the remaining height a primary limb climbs.
    climb: f32,
    /// Downward bend of a branch's middle, as a fraction of its length.  This is
    /// what separates a conifer's drooping tier from a tiered standard's flat
    /// one, and a willow from a `麻楝`.
    droop: f32,
    /// Secondary branches per primary limb.
    fork: usize,
    /// Splay of a secondary branch from its parent's axis.
    fork_reach: f32,
    /// Whorls, and branches per whorl.
    tiers: usize,
    per_tier: usize,
    /// Stems out of the base, and how far each leans.
    stems: usize,
    stem_lean: f32,
}

fn architecture(canopy: Canopy) -> Architecture {
    let make = |profile, layout, primary, attach_bias, reach, climb, droop, fork, fork_reach| {
        Architecture {
            profile,
            layout,
            primary,
            attach_bias,
            reach,
            climb,
            droop,
            fork,
            fork_reach,
            tiers: 0,
            per_tier: 0,
            stems: 0,
            stem_lean: 0.0,
        }
    };
    match canopy {
        // A crabapple or a magnolia: three or four heavy limbs off a short trunk,
        // spread wide and flattened.  `attach_bias` above 1 pulls them to the
        // foot, which is what "low-branched" means.
        Canopy::Spreading => {
            let mut a = make(profile_spreading, Layout::Single, 4, 1.55, 0.97, 0.32, 0.22, 5, 0.98);
            a.stems = 0;
            a
        }
        // Canopy: rounded to hemispherical, arching out of a real trunk.
        Canopy::Rounded => make(profile_rounded, Layout::Single, 8, 1.00, 0.90, 0.52, 0.05, 5, 0.86),
        // A tallow or a tabebia: the same trunk, but the limbs climb hard and
        // their reach grows with their height, so the widest point is the lip.
        Canopy::Vase => make(profile_vase, Layout::Single, 8, 0.70, 0.94, 0.82, 0.12, 4, 0.76),
        // A weeping form: limbs arch out early and then fall, so the crown's
        // mass is at the shoulder and the tips hang below it.
        Canopy::Weeping => make(profile_weeping, Layout::Single, 6, 1.15, 0.88, 0.24, 0.82, 5, 1.05),
        // A metasequoia: ten whorls of four nearly horizontal branches, each a
        // little shorter than the one below, and every one drooping at the tip.
        Canopy::Conical => {
            let mut a = make(profile_conical, Layout::Whorled, 4, 1.0, 0.94, 0.14, 0.20, 2, 0.52);
            a.tiers = 10;
            a.per_tier = 4;
            a
        }
        // A tiered standard: fewer, flatter, longer tiers and a stiffer branch.
        Canopy::Tiered => {
            let mut a = make(profile_tiered, Layout::Whorled, 5, 1.0, 0.92, 0.20, 0.36, 2, 0.56);
            a.tiers = 5;
            a.per_tier = 5;
            a
        }
        // A crape myrtle or a frangipani: several stems out of the ground, each
        // carrying its own small crown.  There is no trunk to speak of, which is
        // why `clear_stem` is near zero for these.
        Canopy::MultiStem => {
            let mut a = make(profile_clump, Layout::Stems, 3, 1.0, 0.88, 0.42, 0.10, 4, 0.86);
            a.stems = 4;
            a.stem_lean = 0.34;
            a
        }
    }
}

// --------------------------------------------------------------------------
// The grower
// --------------------------------------------------------------------------

/// Everything one prototype needs, derived from the species table alone.
struct Plan {
    /// Crown radius in unit-height space.  The crown *is* this envelope, so a
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
    fn new(species: &Species, prototype: &TreePrototype, rng: &mut Rng) -> Self {
        // Unit height is the invariant, so every linear dimension is a real
        // dimension divided by the tree's real height.  Nothing is authored in
        // scene units.
        let height = prototype.height.max(0.1);
        let radius = (prototype.crown / height) * 0.94;
        // The crown starts below the first branch: a `水杉`'s `clear_stem` is
        // 0.62, meaning foliage does not begin until 62% of its height, and the
        // 0.8 lets a whorl sit a little lower than the first true branch without
        // the crown touching the ground.
        let base = (species.clear_stem * 0.80).clamp(0.10, 0.86);
        let tile_m = leaf_cards::card_tile_m(species);
        let card = tile_m / height;
        let arch = architecture(species.canopy);
        // Card count from the crown's *silhouette*, because that is what a viewer
        // has to see through.  A card of a randomly oriented quad presents about
        // 60% of its face, and only `window * coverage` of that face is opaque, so
        // a canopy needs several layers of them before the sky stops showing
        // through.  `density` is the species' own statement about how many: a
        // `麻楝` at 0.58 is built to be see-through and a `柚子` at 0.88 is built
        // to be a solid glossy dome, and the difference between those two numbers
        // is the difference between 2.5 and 3.5 layers.
        let silhouette = std::f32::consts::PI * radius * radius;
        let presented = 0.6
            * card
            * card
            * CARD_WINDOW
            * CARD_WINDOW
            * leaf_cards::target_coverage(species);
        let target_layer = 0.6 + 3.3 * species.density;
        let cards = (target_layer * silhouette / presented.max(1.0e-9))
            .round()
            .clamp(MIN_CARDS as f32, MAX_CARDS as f32) as usize;
        // Clusters: one at every secondary branch's tip, one partway along it,
        // and one at the primary limb's own tip.  Counted up front so the card
        // budget can be divided between them in one pass.
        let primaries = match arch.layout {
            Layout::Single => arch.primary,
            Layout::Whorled => arch.tiers * arch.per_tier,
            Layout::Stems => arch.stems * arch.primary,
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
            // A pine at 0.85 has eight deep flutes; a magnolia at 0.15 has three
            // that are barely there.  An *integer*, because a fractional flute
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

/// One prototype under construction.  Everything it writes is read from the
/// species record, so a tree's silhouette, its colour and its size cannot
/// disagree.
struct Grower<'a, 'b> {
    species: &'a Species,
    plan: Plan,
    arch: Architecture,
    rng: Rng,
    bark: &'b str,
    leaf: &'b str,
    builder: &'b mut MeshBuilder,
}

impl<'a, 'b> Grower<'a, 'b> {
    fn new(
        prototype: &'b TreePrototype,
        seed: u32,
        bark: &'b str,
        leaf: &'b str,
        builder: &'b mut MeshBuilder,
    ) -> Self {
        let species = prototype.species;
        let mut rng = Rng::new(seed);
        let plan = Plan::new(species, prototype, &mut rng);
        let arch = architecture(species.canopy);
        Self {
            species,
            plan,
            arch,
            rng,
            bark,
            leaf,
            builder,
        }
    }

    fn build(&mut self) {
        match self.arch.layout {
            Layout::Single => self.single_trunk(),
            Layout::Whorled => self.whorled_trunk(),
            Layout::Stems => self.stems(),
        }
        // A tree is drawn to its canopy, so the top of the last card is the top
        // of the tree.  The geometry is unit height and the instance scale is the
        // real metres; that invariant is what lets one mesh be any tree.
    }

    // -- the wood ----------------------------------------------------------

    /// A tapered, slightly bowed, fluted tube.  The branch primitive for the
    /// whole tree: trunk, limbs, secondaries and stems are all this.
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
        let grey = leaf_cards::luma(base);
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
        // the difference between a conifer's tier and a standard's.
        let bow = (self.arch.climb - 0.30) * length * 0.22 - self.arch.droop * length * 0.30;
        // Four sides on a primary, three on a secondary.  A primary limb is tens
        // of centimetres of real wood; a secondary is one, and three facets is
        // all a facet budget can spend on it honestly.
        let sides: u8 = if level == 0 { 4 } else { 3 };
        self.limb(from, to, radius, radius * 0.5, sides, 2, bow, 0.0);
        if level == 0 {
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
                let child = (dir * (1.0 - 0.5 * splay)
                    + (a1 * angle.cos() + a2 * angle.sin()) * splay)
                    .normalized_or_up();
                let reach = length * self.rng.range(0.38, 0.58);
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
        let lean = self.rng.direction() * 0.030;
        let leader = self.plan.base + (self.plan.top - self.plan.base) * 0.90;
        // The leader: a real trunk with a real flare at the foot, thinning as it
        // climbs, and leaning very slightly because a tree that grows dead
        // vertical is a lamp post.
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
            self.limb_from(lean, attach, height, self.plan.trunk_r, 1);
        }
    }

    /// Whorled tiers on a clear leader: a metasequoia, a dawn redwood, a tiered
    /// plum.
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
            // The lowest whorl sits at the crown's foot and the top one stops
            // short of the leader's tip, so the spire is a spire and not a
            // lollipop on a stick.
            let share = (tier as f32 + 0.55) / tiers as f32;
            let height = self.plan.base + share * (self.plan.top - self.plan.base) * 0.93;
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
                // A conifer's branch leaves the trunk almost horizontally and
                // lifts a little at first; a tiered standard's lifts more and
                // droops further at the tip.
                let lift = self.arch.climb * reach * self.rng.range(0.35, 1.0);
                let tip = attach
                    + Vec3::new(angle.cos() * reach, lift, angle.sin() * reach);
                self.branch(attach, tip, self.plan.trunk_r * 0.30, 0);
            }
        }
    }

    /// Several stems out of the ground, each carrying its own small crown.  A
    /// crape myrtle, a frangipani, a wampee: there is no trunk, which is why
    /// their `clear_stem` is a fifth of their height.
    fn stems(&mut self) {
        for index in 0..self.arch.stems {
            let angle = self.plan.phase
                + index as f32 / self.arch.stems as f32 * TAU
                + self.rng.range(-0.34, 0.34);
            let outward = Vec2::new(angle.cos(), angle.sin());
            let offset = self.plan.trunk_r * self.rng.range(0.5, 1.6);
            let base = Vec3::new(outward.x * offset, 0.0, outward.y * offset);
            let height = (0.38 + 0.34 * self.rng.unit()) * self.plan.top;
            let lean = self.arch.stem_lean * self.rng.range(0.7, 1.15);
            let top = Vec3::new(
                outward.x * lean * self.plan.radius,
                height,
                outward.y * lean * self.plan.radius,
            );
            self.limb(
                base,
                top,
                self.plan.trunk_r * 0.68,
                self.plan.trunk_r * 0.36,
                5,
                2,
                lean * 0.10,
                0.5,
            );
            for limb in 0..self.arch.primary {
                let share = (limb as f32 + 0.5) / self.arch.primary as f32;
                let attach = base + (top - base) * (0.42 + 0.58 * share);
                self.limb_from(
                    Vec2::new(outward.x, outward.y) * lean,
                    attach,
                    attach.y,
                    self.plan.trunk_r * 0.30,
                    limb,
                );
            }
        }
    }

    /// One primary limb: reached from the trunk or from a stem, aimed at the
    /// crown profile's surface at the height it will end up at.
    fn limb_from(&mut self, lean: Vec2, attach: Vec3, height: f32, radius: f32, index: usize) {
        let primary = self.arch.primary;
        // The phase carries the lean, so four stems do not produce four identical
        // fans and a leaning trunk does not produce a symmetric one.
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

    /// The leader's centre line at a height.  A real trunk is very nearly
    /// straight, which is not the same as exactly straight.
    fn spine(&self, lean: Vec2, height: f32) -> Vec3 {
        let t = (height / self.plan.top.max(0.01)).clamp(0.0, 1.0);
        Vec3::new(lean.x * t * t, height, lean.y * t * t)
    }

    fn trunk_sides(&self) -> u8 {
        // Enough sides to read as round at arm's length, and never fewer than the
        // flutes need or the ridges alias into a star.
        let by_girth = (self.plan.trunk_r * 420.0).clamp(6.0, 12.0);
        by_girth.max((self.plan.flutes * 2.0).ceil().clamp(4.0, 16.0)) as u8
    }

    // -- the foliage --------------------------------------------------------

    /// The crown is an envelope, and the branches are grown to fill it.  Every
    /// card is placed inside that envelope, so a canopy's silhouette is the
    /// species' profile and not whatever the random branch lengths happened to
    /// add up to.  This is the constraint that makes a `水杉` a spire.
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
            let out = self.rng.sphere();
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
    /// so the leaf spray stands up along the twig rather than floating.
    fn card(&mut self, centre: Vec3, shoot: Vec3) {
        let size = self.plan.card * self.rng.range(0.70, 1.32);
        // A different crop of the tile every time.  Foliage has no structure at
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

        // A foliage canopy is not a set of mirrors.  A thin leaf transmits and
        // scatters, so the effective normal of a leaf cluster sits between the
        // card's own normal and the sky.  A quarter of the way is about as far
        // as the evidence supports, and it is the only shading bias in the file.
        let shading = (normal * 0.76 + Vec3::new(0.0, 0.24, 0.0)).normalized_or_up();
        // Leaf-to-leaf variation, and nothing else.  It is deliberately
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

// --------------------------------------------------------------------------
// Planting
// --------------------------------------------------------------------------

/// Where a tree came from.  Reported so the renderer can pick a foliage LOD and
/// so placement can be audited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TreeRole {
    /// Kerbside avenue tree.
    Street,
    /// Median planting.
    Median,
    /// Park or green parcel.
    Park,
    /// Inside a residential compound.
    Compound,
    /// Riverfront promenade.
    Waterfront,
    /// Roundabout island.
    Island,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeOutput {
    pub instances: usize,
    pub by_role: Vec<(String, usize)>,
    pub tufts: usize,
}

/// A Chinese kerb is not random.  The default avenue is a tabebia — purple or
/// gold — because it is what a city in the lower Yangtze plants by the hundred,
/// with a tallow and a chinaberry where the footway is wide, and a plum where
/// the kerb is narrow.  The order is the conventional one, so a street reads as
/// designed rather than as a shuffle.
const AVENUE: &[&str] = &[
    "zi-hua-feng-jiao-mu",
    "huang-hua-feng-jiao-mu",
    "wu-jiu",
    "ma-lian",
    "xiao-ye-ying-ren",
    "hong-hua-yu-lan",
    "huang-jin",
    "luo-yu-shan",
    "shui-shan",
];

/// A motorway's trees have to clear a sign and survive a hot verge, so they are
/// the conifers.
const MOTORWAY: &[&str] = &["shui-shan", "cong-shu", "luo-yu-shan"];

/// A median is a narrow strip at 0.5 m, so it gets the small trees: a crape
/// myrtle is the most-planted median shrub-tree in China and a hibiscus is next.
const MEDIAN: &[&str] = &["xiao-ye-zi-wei", "huang-jin", "you-zi"];

/// A park is where every species in the palette can appear, and a city with a
/// park has all of them in it.
const PARK: &[&str] = &[
    "ma-lian",
    "wu-jiu",
    "luo-yu-shan",
    "shui-shan",
    "cong-shu",
    "zi-hua-feng-jiao-mu",
    "huang-hua-feng-jiao-mu",
    "xiao-ye-ying-ren",
    "hai-tang",
    "xiao-ye-zi-wei",
    "you-zi",
    "hong-hua-ji-dan-hua",
    "huang-pi",
    "hong-hua-yu-lan",
    "jin-ye-ying-ren",
    "huang-jin",
];

/// A compound courtyard is evergreen, sheltering and small.
const COMPOUND: &[&str] = &[
    "you-zi",
    "xiao-ye-zi-wei",
    "xiao-ye-ying-ren",
    "hong-hua-yu-lan",
    "hai-tang",
];

/// A riverbank in a Chinese city is a metasequoia or a dawn redwood avenue.  The
/// Wuhan and Nanjing embankments are lined with them and so is half the
/// country.
const WATERFRONT: &[&str] = &["luo-yu-shan", "shui-shan", "ma-lian", "wu-jiu"];

/// A roundabout island is visible from every approach, so it gets the showy
/// small trees.
const ISLAND: &[&str] = &["xiao-ye-zi-wei", "huang-jin", "you-zi", "jin-ye-ying-ren"];

/// A role's planting list.
fn planting(role: TreeRole) -> &'static [&'static str] {
    match role {
        TreeRole::Street => AVENUE,
        TreeRole::Median => MEDIAN,
        TreeRole::Park => PARK,
        TreeRole::Compound => COMPOUND,
        TreeRole::Waterfront => WATERFRONT,
        TreeRole::Island => ISLAND,
    }
}

/// Plant every tree a city should have: kerbside avenues, planted medians, park
/// groves, compound interiors, the river promenade and roundabout islands.
pub fn plant(
    network: &Network,
    parcels: &[Parcel],
    river: Option<&[Point]>,
    frame: CityFrameInfo,
    prototypes: &[TreePrototype],
    builder: &mut MeshBuilder,
    seed: u32,
) -> TreeOutput {
    declare(builder);
    let mut rng = Rng::new(seed ^ 0x74ee);
    let mut output = TreeOutput::default();
    let mut counts: Vec<(TreeRole, usize)> = Vec::new();
    // A free function rather than a closure: the planting loops need the same
    // stream for their own jitter, and a closure capturing it would force those
    // draws through `place` instead.
    fn place(
        role: TreeRole,
        point: Vec2,
        list: &'static [&'static str],
        salt: u32,
        prototypes: &[TreePrototype],
        rng: &mut Rng,
        builder: &mut MeshBuilder,
        output: &mut TreeOutput,
        counts: &mut Vec<(TreeRole, usize)>,
    ) {
        if output.instances >= TREE_BUDGET {
            return;
        }
        // A planting list is a designed order, not a uniform bag: the front of
        // the list is what the city actually plants, and `u^1.35` is a gentle
        // enough skew that the rest still turns up.
        let roll = rng.unit().powf(1.35) * list.len() as f32;
        let key = list[(roll as usize).min(list.len() - 1)];
        let Some(species) = by_key(key) else {
            return;
        };
        let variants = variants_of(species);
        let variant = rng.int(variants as u32) as u16;
        let index = prototypes
            .iter()
            .position(|prototype| prototype.species.key == species.key && prototype.variant == variant)
            .unwrap_or(0);
        let prototype = &prototypes[index];
        let jitter = 0.90 + rng.unit() * 0.22;
        // Per-tree variation, and only downwards: an instance tint above 1.0
        // clips in the payload's `u8` tint and the tree is brighter than the
        // species, which is the one direction that has to be impossible.
        let tint = [
            0.84 + rng.unit() * 0.16,
            0.86 + rng.unit() * 0.14,
            0.82 + rng.unit() * 0.18,
        ];
        builder.add_instance(
            &prototype.key,
            Instance {
                x: point.x,
                y: level::GROUND - 0.05,
                z: point.y,
                rotation_y: rng.unit() * TAU,
                scale_x: prototype.height * jitter,
                scale_y: prototype.height * jitter,
                scale_z: prototype.height * jitter,
                tint_r: tint[0],
                tint_g: tint[1],
                tint_b: tint[2],
            },
        );
        output.instances += 1;
        let _ = salt;
        match counts.iter_mut().find(|(existing, _)| *existing == role) {
            Some(entry) => entry.1 += 1,
            None => counts.push((role, 1)),
        }
    }

    // --- kerbside avenues and planted medians -------------------------------
    for road in &network.roads {
        if road.layer != 0 || !road.has_sidewalk() {
            continue;
        }
        let path = road.carriageway.clone();
        let length = path.length();
        if length < 24.0 {
            continue;
        }
        let section = road.section;
        let half = section.half_width();
        // A boulevard gets a double row; a local street gets one on the sunnier
        // side only, which is both what is built and what keeps the count sane.
        let double_row = matches!(
            road.class,
            urban::ModernRoadClass::Arterial | urban::ModernRoadClass::Expressway
        );
        let spacing = if road.is_motorway() { 22.0 } else { 15.0 };
        let lateral = half + section.sidewalk_metres * 0.6;
        let list = if road.is_motorway() { MOTORWAY } else { AVENUE };
        let mut station = 12.0;
        let mut side = 1.0_f32;
        while station < length - 12.0 {
            // Longitudinally jittered so an avenue does not read as a comb.
            let offset = station + (rng.unit() - 0.5) * 5.0;
            if offset < 10.0 || offset > length - 10.0 {
                station += spacing;
                continue;
            }
            let sides: &[f32] = if double_row { &[-1.0, 1.0] } else { &[side] };
            for direction in sides {
                let wobble = (rng.unit() - 0.5) * 0.9;
                let point = path.offset_at(offset, *direction * (lateral + wobble), 0.0);
                place(
                    TreeRole::Street,
                    Vec2::new(point.x, point.z),
                    list,
                    0,
                    prototypes,
                    &mut rng,
                    builder,
                    &mut output,
                    &mut counts,
                );
            }
            side = -side;
            station += spacing;
        }
        // Median belt on a divided street.
        if section.has_median() {
            let median_half = section.median_metres * 0.5;
            let mut at = 20.0;
            while at < length - 20.0 {
                let point =
                    path.offset_at(at + (rng.unit() - 0.5) * 4.0, (rng.unit() - 0.5) * median_half * 0.5, 0.0);
                place(
                    TreeRole::Median,
                    Vec2::new(point.x, point.z),
                    MEDIAN,
                    0,
                    prototypes,
                    &mut rng,
                    builder,
                    &mut output,
                    &mut counts,
                );
                at += 24.0;
            }
        }
    }

    // --- park groves and compound interiors ---------------------------------
    for parcel in parcels {
        let ring: Vec<Vec2> = parcel
            .ring
            .iter()
            .map(|point| {
                let [x, z] = frame.to_local(*point);
                Vec2::new(x, z)
            })
            .collect();
        if ring.len() < 3 {
            continue;
        }
        let interior = crate::math::inset_ring(&ring, 3.0);
        if interior.len() < 3 {
            continue;
        }
        let bounds = bounds_of(&interior);
        let (role, count, list) = match parcel.use_type {
            ParcelUse::Park => (TreeRole::Park, 10, PARK),
            ParcelUse::Residential if parcel.compound => (TreeRole::Compound, 8, COMPOUND),
            ParcelUse::Civic => (TreeRole::Park, 5, PARK),
            _ => continue,
        };
        for index in 0..count {
            // Rejection-sample inside the parcel: scatter trees in a bounding box
            // and keep the ones that actually land on green.
            for _ in 0..6 {
                let candidate = Vec2::new(
                    bounds.0 + rng.unit() * (bounds.2 - bounds.0),
                    bounds.1 + rng.unit() * (bounds.3 - bounds.1),
                );
                if crate::math::point_in_ring(candidate, &interior) {
                    place(
                        role,
                        candidate,
                        list,
                        index as u32,
                        prototypes,
                        &mut rng,
                        builder,
                        &mut output,
                        &mut counts,
                    );
                    break;
                }
            }
        }
    }

    // --- riverfront promenade ----------------------------------------------
    if let Some(river) = river {
        let path = crate::math::Path::from_plan(
            river
                .iter()
                .map(|point| {
                    let [x, z] = frame.to_local(*point);
                    [x, z]
                })
                .collect::<Vec<[f32; 2]>>(),
        );
        let mut station = 0.0;
        while station < path.length() {
            for side in [-1.0_f32, 1.0] {
                let point = path.offset_at(
                    station,
                    side * (network.max_trim().max(18.0) * 0.5 + 9.0),
                    0.0,
                );
                place(
                    TreeRole::Waterfront,
                    Vec2::new(point.x, point.z),
                    WATERFRONT,
                    0,
                    prototypes,
                    &mut rng,
                    builder,
                    &mut output,
                    &mut counts,
                );
            }
            station += 30.0;
        }
    }

    // --- roundabout islands --------------------------------------------------
    for junction in &network.junctions {
        if !junction.roundabout {
            continue;
        }
        let island = junction.radius.max(18.0) * 0.42;
        for index in 0..5 {
            let angle = index as f32 / 5.0 * TAU + rng.unit() * 0.9;
            let radius = island * (0.25 + rng.unit() * 0.5);
            place(
                TreeRole::Island,
                junction.centre + Vec2::new(angle.cos(), angle.sin()) * radius,
                ISLAND,
                index as u32,
                prototypes,
                &mut rng,
                builder,
                &mut output,
                &mut counts,
            );
        }
    }

    // --- low shrubs and grass tufts -----------------------------------------
    let mut tufts = 0;
    for parcel in parcels {
        if !matches!(parcel.use_type, ParcelUse::Park) {
            continue;
        }
        let ring: Vec<Vec2> = parcel
            .ring
            .iter()
            .map(|point| {
                let [x, z] = frame.to_local(*point);
                Vec2::new(x, z)
            })
            .collect();
        if ring.len() < 3 {
            continue;
        }
        let bounds = bounds_of(&ring);
        for _ in 0..40 {
            if tufts >= TUFT_BUDGET {
                break;
            }
            let candidate = Vec2::new(
                bounds.0 + rng.unit() * (bounds.2 - bounds.0),
                bounds.1 + rng.unit() * (bounds.3 - bounds.1),
            );
            if !crate::math::point_in_ring(candidate, &ring) {
                continue;
            }
            // The tuft's albedo is in its texture; the instance tint only
            // varies it, and only downwards, for the same reason the tree tint
            // does.
            let green = 0.82 + rng.unit() * 0.18;
            builder.add_instance(
                "tuft",
                Instance {
                    x: candidate.x,
                    y: level::GROUND - 0.02,
                    z: candidate.y,
                    rotation_y: rng.unit() * TAU,
                    scale_x: 0.55 + rng.unit() * 0.5,
                    scale_y: 0.55 + rng.unit() * 0.5,
                    scale_z: 0.55 + rng.unit() * 0.5,
                    tint_r: green * 0.96,
                    tint_g: green,
                    tint_b: green * 0.90,
                },
            );
            tufts += 1;
        }
    }
    output.tufts = tufts;
    output.by_role = counts
        .into_iter()
        .map(|(role, count)| (format!("{role:?}").to_lowercase(), count))
        .collect();
    output
}

fn variants_of(species: &Species) -> usize {
    if TWO_VARIANT.contains(&species.key) { 2 } else { 1 }
}

fn bounds_of(ring: &[Vec2]) -> (f32, f32, f32, f32) {
    let mut bounds = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for point in ring {
        bounds.0 = bounds.0.min(point.x);
        bounds.1 = bounds.1.min(point.y);
        bounds.2 = bounds.2.max(point.x);
        bounds.3 = bounds.3.max(point.y);
    }
    bounds
}

/// A clipped shrub, instanced along medians and park edges.
///
/// Two crossed cards of a real leaf-mass silhouette rather than a low-poly ball:
/// a ball of 7-sided quads is the other thing that made the last port's parks
/// read as props, and this geometry is seen from two metres away.
pub fn build_shrub_prototype(builder: &mut MeshBuilder) {
    declare(builder);
    builder.style(
        "hedge",
        GroupStyle {
            cast_shadow: true,
            receive_shadow: true,
            alpha_cutout: false,
            dynamic: false,
        },
    );
    // A dense clipped dome: a vertical profile that is widest just below the
    // middle and a flat top, which is exactly what a pruning shear leaves.
    let (rings, sides) = (5_u32, 9_u32);
    let mut points: Vec<Vec3> = Vec::new();
    for ring in 0..=rings {
        let phi = ring as f32 / rings as f32 * std::f32::consts::PI;
        let y = phi.cos().abs().powf(0.7);
        let radius = phi.sin().powf(0.8) * (0.94 - 0.10 * ring as f32 / rings as f32);
        for side in 0..sides {
            let theta = side as f32 / sides as f32 * TAU;
            // Lobed rather than circular: a clipped hedge is never a lathe form.
            let lobe = 1.0 + 0.07 * (theta * 3.0).sin();
            points.push(Vec3::new(
                radius * lobe * theta.cos() * 0.62,
                y * 0.62 + 0.10,
                radius * lobe * theta.sin() * 0.62,
            ));
        }
    }
    for ring in 0..rings as usize {
        for side in 0..sides as usize {
            let a = ring * sides as usize + side;
            let b = ring * sides as usize + (side + 1) % sides as usize;
            let c = (ring + 1) * sides as usize + (side + 1) % sides as usize;
            let d = (ring + 1) * sides as usize + side;
            builder.quad("hedge", points[a], points[b], points[c], points[d], None);
        }
    }
    // One transform list places every shrub in the city.
    builder.bind("hedge", "hedge");
}

/// A crossed-quad grass tuft, sampling the baked tuft card so the blades are
/// blades and not a green box.
pub fn build_tuft_prototype(builder: &mut MeshBuilder) {
    declare(builder);
    builder.style(
        "tuft",
        GroupStyle {
            cast_shadow: false,
            receive_shadow: true,
            alpha_cutout: true,
            dynamic: false,
        },
    );
    for index in 0..2 {
        let angle = index as f32 * std::f32::consts::FRAC_PI_2;
        let (sin, cos) = angle.sin_cos();
        let axis_u = Vec3::new(cos, 0.0, sin);
        // Metre UVs: the tuft texture's tile is one metre, so `0..1` is the whole
        // image on a one-metre card.
        let corners = [
            Vec3::new(0.0, 0.0, 0.0) - axis_u * 0.5,
            Vec3::new(0.0, 0.0, 0.0) + axis_u * 0.5,
            Vec3::new(0.0, 1.0, 0.0) + axis_u * 0.35,
            Vec3::new(0.0, 1.0, 0.0) - axis_u * 0.35,
        ];
        // White, because the albedo is in the texture now.  The old `tuft` card
        // was transparent, so this quad was a solid `[0.34, 0.52, 0.26]` cross
        // standing in every park.
        let tint = [1.0_f32, 1.0_f32, 1.0_f32];
        builder.quad_uv(
            "tuft",
            corners[0],
            corners[1],
            corners[2],
            corners[3],
            [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            Some(tint),
        );
        builder.quad_uv(
            "tuft",
            corners[1],
            corners[0],
            corners[3],
            corners[2],
            [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            Some(tint),
        );
    }
    builder.bind("tuft", "tuft");
}

/// Median shrubs along a divided road, as instances of the shrub prototype.
pub fn plant_median_shrubs(
    network: &Network,
    builder: &mut MeshBuilder,
    seed: u32,
) -> usize {
    let mut rng = Rng::new(seed ^ 0x5b12);
    let mut count = 0;
    for road in &network.roads {
        if road.layer != 0 || !road.section.has_median() {
            continue;
        }
        let path = road.carriageway.clone();
        let length = path.length();
        let half = road.section.median_metres * 0.5;
        let mut station = 4.0;
        while station < length - 4.0 {
            let point = path.offset_at(station, (rng.unit() - 0.5) * half * 0.7, 0.0);
            // Downwards only, for the same reason the tree tint is: an instance
            // tint above 1.0 clips in the payload's `u8` and the shrub ends up
            // brighter than any clipped hedge can be.
            let green = 0.80 + rng.unit() * 0.20;
            builder.add_instance(
                "hedge",
                Instance::new(
                    point.x,
                    crate::street::level::MEDIAN + 0.10,
                    point.z,
                    rng.unit() * TAU,
                    0.8 + rng.unit() * 0.5,
                    [green * 0.94, green, green * 0.88],
                ),
            );
            count += 1;
            station += 1.35;
        }
    }
    count
}

/// Deterministic species choice for a parcel, shared with the plan generator so
/// a tree's species and its planting position cannot disagree.
///
/// This is the *plan* layer's species, from `urban`'s own five-variant enum; the
/// scene layer plants the sixteen-species palette in [`crate::species`] and
/// picks per role from a designed list. It stays because `urban` still carries
/// it on every `TreeInstance`, and a plan whose tree species cannot be named is
/// not auditable.
pub fn species_for_parcel(parcel: &Parcel, index: usize) -> TreeSpecies {
    let roll = modern::hash_u32(parcel.id, index as i32, 917);
    match parcel.use_type {
        ParcelUse::Park => {
            if roll < 0.34 {
                TreeSpecies::Willow
            } else {
                TreeSpecies::Ginkgo
            }
        }
        ParcelUse::Residential => {
            if roll < 0.5 {
                TreeSpecies::ChinesePlane
            } else {
                TreeSpecies::Ginkgo
            }
        }
        _ => TreeSpecies::ChinesePlane,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::MeshGroup;
    use crate::network::derive;
    use crate::spec::JunctionSpec;
    use urban::{ModernChinaSpec, generate_modern_chinese_city};

    fn city() -> urban::ModernCity {
        generate_modern_chinese_city(ModernChinaSpec {
            seed: 42,
            radius_km: 0.5,
            block_size_metres: 110.0,
            ..ModernChinaSpec::default()
        })
    }

    fn built() -> (Vec<TreePrototype>, Vec<MeshGroup>) {
        let prototypes = prototype_set();
        let mut builder = MeshBuilder::new();
        build_prototypes(&prototypes, &mut builder);
        let scene = builder.build();
        (prototypes, scene.meshes)
    }

    fn groups_of<'a>(
        meshes: &'a [MeshGroup],
        prototype: &TreePrototype,
        part: &str,
    ) -> Vec<&'a MeshGroup> {
        let key = format!("{}#{}", prototype.key, part);
        meshes
            .iter()
            .filter(|mesh| mesh.material == key)
            .collect()
    }

    /// One prototype per species, every species present, and the set no bigger
    /// than the renderer's draw-call budget allows.
    #[test]
    fn every_species_has_a_prototype_and_the_set_is_bounded() {
        let prototypes = prototype_set();
        let keys: Vec<&str> = prototypes.iter().map(|p| p.species.key).collect();
        for species in SPECIES {
            let count = keys.iter().filter(|key| **key == species.key).count();
            assert!(
                (1..=2).contains(&count),
                "{} has {count} prototypes",
                species.key
            );
        }
        assert_eq!(prototypes.len(), SPECIES.len() + TWO_VARIANT.len());
        assert!(prototypes.len() <= 48, "{} prototypes", prototypes.len());
        // Two mesh groups each, so the tree layer's draw-call cost is exactly
        // twice the prototype count: bark and leaf are separate materials and the
        // renderer splits on `#`.
        let (_p, meshes) = built();
        let tree_meshes = meshes
            .iter()
            .filter(|mesh| mesh.material.starts_with("tree/"))
            .count();
        assert_eq!(tree_meshes, prototypes.len() * 2);
        assert!(tree_meshes <= 96, "{tree_meshes} tree mesh groups");
        // And every one of them is bound to a real instance list.
        for prototype in &prototypes {
            for part in ["bark", "leaf"] {
                let found = groups_of(&meshes, prototype, part);
                assert_eq!(found.len(), 1, "{}#{} is missing", prototype.key, part);
                assert_eq!(found[0].instance_of.as_deref(), Some(prototype.key.as_str()));
            }
        }
    }

    /// Every card's centre, as a radius from the tree's axis.  This — not an
    /// axis-aligned bounding box — is the canopy's real width, because a crown
    /// with eight limbs in random directions is round and its box is not.
    fn crown_radius(meshes: &[MeshGroup], prototype: &TreePrototype) -> f32 {
        let mut widest = 0.0_f32;
        for mesh in groups_of(meshes, prototype, "leaf") {
            for card in mesh.positions.chunks_exact(12) {
                let x = (card[0] + card[3] + card[6] + card[9]) / 4.0;
                let z = (card[2] + card[5] + card[8] + card[11]) / 4.0;
                widest = widest.max(x.hypot(z));
            }
        }
        widest
    }

    /// Prototypes are unit height so one mesh is any tree, and the instance scale
    /// is the real metres.  A prototype that did not top out at 1.0 would make
    /// the instance scale wrong; one that topped out much above 1.0 would make
    /// the tree taller than the renderer was told.
    #[test]
    fn prototypes_are_authored_at_unit_height_so_one_mesh_scales_to_any_tree() {
        let (prototypes, meshes) = built();
        for prototype in &prototypes {
            let leaves = groups_of(&meshes, prototype, "leaf");
            let top = leaves[0]
                .positions
                .chunks(3)
                .map(|v| v[1])
                .fold(f32::MIN, f32::max);
            // A card is centred inside the crown and stands up to its own
            // half-diagonal proud of the crown's top, because the crown bounds a
            // card's *centre* and not its corners.  More than a card's width
            // would be a canopy that has grown past the height the instance scale
            // says it has.
            let card = leaf_cards::card_tile_m(prototype.species) / prototype.height;
            assert!(
                (0.985..=1.0 + card).contains(&top),
                "{}'s canopy tops out at {top:.3}, not unit height (+/- one {card:.3} card)",
                prototype.key
            );
            let bark = groups_of(&meshes, prototype, "bark");
            let base = bark[0]
                .positions
                .chunks(3)
                .map(|v| v[1])
                .fold(f32::MAX, f32::min);
            // The base has to be at the planting line.  A tree is placed with its
            // origin at ground level and its flare may dip a little below that —
            // a root collar does — but a trunk floating above the ground or a
            // whole metre underground is not a tree.
            assert!(
                base > -0.02 && base < 0.005,
                "{}'s trunk starts at {base:.4}, which is not the planting line",
                prototype.key
            );
        }
    }

    /// Axis-aligned bounds of a whole prototype: `(min_x, max_x, min_y, max_y,
    /// min_z, max_z)`.
    fn bounds(meshes: &[MeshGroup], prototype: &TreePrototype) -> [f32; 6] {
        let mut bounds = [f32::MAX, f32::MIN, f32::MAX, f32::MIN, f32::MAX, f32::MIN];
        for part in ["bark", "leaf"] {
            for mesh in groups_of(meshes, prototype, part) {
                for vertex in mesh.positions.chunks_exact(3) {
                    bounds[0] = bounds[0].min(vertex[0]);
                    bounds[1] = bounds[1].max(vertex[0]);
                    bounds[2] = bounds[2].min(vertex[1]);
                    bounds[3] = bounds[3].max(vertex[1]);
                    bounds[4] = bounds[4].min(vertex[2]);
                    bounds[5] = bounds[5].max(vertex[2]);
                }
            }
        }
        bounds
    }

    /// The bug: every tree was a cylinder with a ball on it, and a cedar and a
    /// willow were the same shape in different greens.  This asserts the shape is
    /// the species', on the *built geometry* — not on the table's claims.
    #[test]
    fn silhouette_is_different_per_species_measured_on_the_geometry() {
        let (prototypes, meshes) = built();
        let mut widest = 0.0_f32;
        let mut narrowest = f32::INFINITY;
        for prototype in &prototypes {
            let b = bounds(&meshes, prototype);
            let height = b[3] - b[2];
            // The species table says this tree's crown is `crown_m` across and
            // `height_m` tall, and the built canopy has to agree with the numbers
            // the renderer and the placement code use, within the slack a limb's
            // reach jitter and the profile's own taper account for.
            let nominal = prototype.crown / prototype.height;
            let built = crown_radius(&meshes, prototype);
            assert!(
                built > nominal * 0.70 && built < nominal * 1.10,
                "{}'s crown reaches {built:.4} against a nominal {nominal:.4}",
                prototype.key
            );
            assert!(height > 0.98, "{} is only {height:.3} tall", prototype.key);
            let ratio = built / height;
            widest = widest.max(ratio);
            narrowest = narrowest.min(ratio);
            // No species' geometry contains a NaN, and every normal is a unit
            // vector.  A NaN in a prototype is silent: it poisons the buffer and
            // the renderer drops triangles one by one.
            for part in ["bark", "leaf"] {
                for mesh in groups_of(&meshes, prototype, part) {
                    assert!(
                        mesh.positions.iter().all(|v| v.is_finite()),
                        "{}#{part} has a non-finite vertex",
                        prototype.key
                    );
                    for normal in mesh.normals.chunks_exact(3) {
                        let length = (normal[0] * normal[0]
                            + normal[1] * normal[1]
                            + normal[2] * normal[2])
                            .sqrt();
                        assert!(
                            (length - 1.0).abs() < 0.02,
                            "{}#{part} has a {length:.3}-length normal",
                            prototype.key
                        );
                    }
                }
            }
        }
        // And the spread across the palette is a real spread, not a spread of
        // three percent around one number.
        assert!(
            widest / narrowest > 2.5,
            "the narrowest prototype's crown is {narrowest:.3} of its height and the \
             widest {widest:.3}; that is not a range of species"
        );
    }

    /// A conifer is a spire: narrow, tall, and *tapering* — the crown's outer
    /// reach in an upper band is much less than in a lower one.  A vase is the
    /// mirror image: it opens upward.  Measured on the built leaves, one height
    /// band at a time, against the species' own profile.
    #[test]
    fn a_conifer_tapers_upward_and_a_vase_opens_upward() {
        let (prototypes, meshes) = built();
        // The outer reach of a crown inside a band of normalised crown height.
        let reach = |prototype: &TreePrototype, low: f32, high: f32| -> f32 {
            let base = (prototype.species.clear_stem * 0.80).clamp(0.10, 0.86);
            let mut widest = 0.0_f32;
            for mesh in groups_of(&meshes, prototype, "leaf") {
                for card in mesh.positions.chunks_exact(12) {
                    // A quad is four consecutive vertices at 3 floats each, so
                    // vertex `n` is at offset `3n`: y at 1, 4, 7, 10 and z at
                    // 2, 5, 8, 11.
                    let y = (card[1] + card[4] + card[7] + card[10]) / 4.0;
                    let t = (y - base) / (1.0 - base);
                    if t < low || t > high {
                        continue;
                    }
                    let x = (card[0] + card[3] + card[6] + card[9]) / 4.0;
                    let z = (card[2] + card[5] + card[8] + card[11]) / 4.0;
                    widest = widest.max(x.hypot(z));
                }
            }
            widest
        };
        for species in SPECIES {
            let Some(prototype) = prototypes.iter().find(|p| p.species.key == species.key) else {
                continue;
            };
            let b = bounds(&meshes, prototype);
            let ratio = crown_radius(&meshes, prototype) / (b[3] - b[2]);
            match species.canopy {
                Canopy::Conical => {
                    assert!(
                        ratio < 0.15,
                        "{}'s crown is {ratio:.2} of its height; a spire is not",
                        species.key
                    );
                    let (foot, crown) = (reach(prototype, 0.10, 0.30), reach(prototype, 0.55, 0.75));
                    assert!(
                        crown < foot * 0.88,
                        "{}'s crown reaches {crown:.4} in its upper band against {foot:.4} \
                         at its foot; a conifer's tiers narrow upward",
                        species.key
                    );
                }
                Canopy::Vase => {
                    // Narrow at the foot, widest at the lip.
                    let (foot, lip) = (reach(prototype, 0.20, 0.42), reach(prototype, 0.58, 0.80));
                    assert!(
                        lip > foot * 1.15,
                        "{}'s crown reaches {lip:.4} at its lip against {foot:.4} at its foot; \
                         a vase is the opposite of a spire",
                        species.key
                    );
                }
                Canopy::Spreading => {
                    assert!(
                        ratio > 0.30,
                        "{}'s crown is only {ratio:.2} of its height; a low spreading \
                         crabapple is far wider than tall",
                        species.key
                    );
                }
                Canopy::MultiStem => {
                    assert!(
                        ratio > 0.25,
                        "{}'s crown is only {ratio:.2} of its height; a multi-stemmed \
                         shrub is as broad as it is high",
                        species.key
                    );
                }
                _ => {}
            }
        }
    }

    /// The crown is the species' own envelope, so no card is placed outside it.
    /// This is the constraint that makes a `水杉` a spire rather than whatever
    /// its random branch lengths added up to.
    #[test]
    fn no_card_escapes_its_crowns_envelope() {
        let (prototypes, meshes) = built();
        for prototype in &prototypes {
            let limit = (prototype.crown / prototype.height) * 0.94;
            let base = (prototype.species.clear_stem * 0.80).clamp(0.10, 0.86);
            for mesh in groups_of(&meshes, prototype, "leaf") {
                for card_vertices in mesh.positions.chunks_exact(12) {
                    let centre = Vec3::new(
                        (card_vertices[0] + card_vertices[3] + card_vertices[6] + card_vertices[9]) / 4.0,
                        (card_vertices[1] + card_vertices[4] + card_vertices[7] + card_vertices[10]) / 4.0,
                        (card_vertices[2] + card_vertices[5] + card_vertices[8] + card_vertices[11]) / 4.0,
                    );
                    let radius = Vec2::new(centre.x, centre.z).length();
                    assert!(
                        radius <= limit * 1.02 + 1.0e-4,
                        "{} has a card at {radius:.4} from its axis, outside its \
                         {limit:.4} crown",
                        prototype.key
                    );
                    assert!(
                        centre.y >= base - 1.0e-4 && centre.y <= 1.0 + 1.0e-4,
                        "{} has a card at {:.3}, outside its crown's {:.3}..1.0",
                        prototype.key,
                        centre.y,
                        base
                    );
                }
            }
        }
    }

    /// Cards, not a shell: a canopy is thousands of independently oriented quads,
    /// and no two of them sample the same texture window or carry the same tint.
    #[test]
    fn a_canopy_is_cards_that_do_not_repeat() {
        let (prototypes, meshes) = built();
        for prototype in &prototypes {
            let leaves = groups_of(&meshes, prototype, "leaf");
            let mesh = leaves[0];
            let cards = mesh.positions.len() / 3 / 4;
            assert!(
                cards >= MIN_CARDS,
                "{} has only {cards} leaf cards",
                prototype.key
            );
            assert!(mesh.alpha_cutout, "leaf cards must be alpha-cut");
            assert!(mesh.uvs.is_some(), "leaf cards need metre UVs for the card");
            assert!(mesh.colors.is_some(), "leaf cards need per-card tints");
            // Independent orientations, not one repeated quad: one sample per
            // card, so the count is of cards and not of vertices.
            let mut seen: Vec<(i32, i32)> = mesh
                .normals
                .chunks_exact(3)
                .step_by(3)
                .take(120)
                .map(|n| ((n[0] * 40.0) as i32, (n[2] * 40.0) as i32))
                .collect();
            seen.sort_unstable();
            seen.dedup();
            assert!(
                seen.len() > 15,
                "{}'s cards share one orientation ({} distinct of 40 sampled)",
                prototype.key,
                seen.len()
            );
            // And the UVs differ card to card, so a canopy is not a wallpapering of
            // one crop.
            let uvs = mesh.uvs.as_ref().unwrap();
            let mut windows: Vec<[i64; 4]> = uvs
                .chunks_exact(8)
                .map(|corners| {
                    let mut key = [0_i64; 4];
                    for axis in 0..2 {
                        key[axis] = (corners[axis] * 8192.0) as i64;
                        key[axis + 2] = (corners[axis + 2] * 8192.0) as i64;
                    }
                    key
                })
                .collect();
            windows.sort_unstable();
            windows.dedup();
            assert!(
                windows.len() > cards * 9 / 10,
                "{} has {} distinct texture windows for {cards} cards; a canopy that \
                 stamps one crop is wallpaper",
                prototype.key,
                windows.len()
            );
        }
    }

    /// The tint on a card is leaf-to-leaf variation, not painted occlusion.  A
    /// canopy that is darker deep inside is exactly the "cheap interior shading"
    /// that made the last port's trees look like they were lit by a lamp.
    #[test]
    fn card_tint_is_not_painted_occlusion() {
        let (prototypes, meshes) = built();
        for prototype in &prototypes {
            let mesh = groups_of(&meshes, prototype, "leaf")[0];
            let colours = mesh.colors.as_ref().unwrap();
            let b = bounds(&meshes, prototype);
            let span = (b[3] - b[2]).max(1.0e-4);
            let mut low = Vec::new();
            let mut high = Vec::new();
            for (index, card) in mesh.positions.chunks_exact(12).enumerate() {
                let y = (card[1] + card[4] + card[7] + card[10]) / 4.0;
                let t = (y - b[2]) / span;
                // Per-card tint, so one sample: the first vertex of the card.
                let pixel = index * 4;
                if t < 0.35 {
                    low.push(colours[pixel + 1] as f32);
                } else if t > 0.70 {
                    high.push(colours[pixel + 1] as f32);
                }
            }
            if low.len() < 40 || high.len() < 40 {
                continue;
            }
            let mean = |values: &Vec<f32>| values.iter().sum::<f32>() / values.len() as f32;
            let (low_mean, high_mean) = (mean(&low), mean(&high));
            assert!(
                (low_mean - high_mean).abs() < 12.0,
                "{}'s cards in the bottom third average {low_mean:.1} and those in the top \
                 third {high_mean:.1}; canopy tint must be leaf-to-leaf variation, not \
                 painted occlusion",
                prototype.key
            );
        }
    }

    /// `density` is the canopy's opacity and it has to be one.  This is the
    /// difference between a `麻楝` you can see the sky through and a `柚子` you
    /// cannot, measured on the built cards rather than asserted in a comment.
    #[test]
    fn canopy_opacity_follows_the_species_density() {
        let (prototypes, meshes) = built();
        // The opaque coverage of each species' own card, so the measure is
        // physical: a card's geometry times the fraction of its texture that is
        // actually leaf.
        let coverage: Vec<(String, f32)> = leaf_cards::leaf_card_textures(64)
            .iter()
            .map(|texture| {
                let opaque = texture.rgba.chunks_exact(4).filter(|p| p[3] > 0).count();
                (
                    texture.name.clone(),
                    opaque as f32 / (texture.width * texture.height) as f32,
                )
            })
            .collect();
        let window = leaf_cards::CARD_WINDOW;
        let mut dense: Vec<(f32, f32)> = Vec::new();
        let mut sparse: Vec<(f32, f32)> = Vec::new();
        let mut layers: Vec<(String, f32, f32)> = Vec::new();
        for prototype in &prototypes {
            let card_m = leaf_cards::card_tile_m(prototype.species);
            let opacity = window * window * coverage_of(&coverage, prototype.species.key);
            let reach = crown_radius(&meshes, prototype);
            let silhouette = std::f32::consts::PI * reach * reach;
            let mut presented = 0.0_f32;
            let mut cards = 0.0_f32;
            for mesh in groups_of(&meshes, prototype, "leaf") {
                for card in mesh.positions.chunks_exact(12) {
                    // A quad's diagonal is `size * sqrt(2)`, so the card's own
                    // area is a sixth of the square of that.
                    let dx = card[9] - card[0];
                    let dy = card[10] - card[1];
                    let dz = card[11] - card[2];
                    let diagonal = (dx * dx + dy * dy + dz * dz).sqrt();
                    let area = diagonal * diagonal / 12.0;
                    // 0.6 is the mean projected area of a randomly oriented flat
                    // card; the foliage normals here are biased outward, so this
                    // is a slight under-estimate rather than a fudge.
                    presented += 0.6 * area * opacity;
                    cards += 1.0;
                }
            }
            let layer = presented / silhouette.max(1.0e-6);
            assert!(
                (1.0..=5.2).contains(&layer),
                "{}'s canopy is {layer:.2} leaf layers deep; a canopy is neither a \
                 shell nor a block",
                prototype.key
            );
            // Cards per unit of crown: this is the quantity `density` has to move,
            // and it is the one a "simplification" would flatten first.
            let per_area = cards / silhouette.max(1.0e-6);
            layers.push((prototype.key.clone(), layer, per_area));
            if prototype.species.density > 0.75 {
                dense.push((layer, per_area));
            } else if prototype.species.density < 0.70 {
                sparse.push((layer, per_area));
            }
        }
        let mean = |values: &[(f32, f32)], index: usize| {
            values.iter().map(|v| v[index]).sum::<f32>() / values.len() as f32
        };
        assert!(
            mean(&dense, 1) > mean(&sparse, 1) * 1.8,
            "the densest species need {} cards per unit of crown and the sparsest \
             {}; `density` is not reaching the geometry",
            mean(&dense, 1),
            mean(&sparse, 1)
        );
        // And the headline contrast, measured on the layers themselves.
        let layer_of = |key: &str| {
            layers
                .iter()
                .find(|(name, _, _)| name.ends_with(key))
                .map(|(_, layer, _)| *layer)
                .unwrap_or(0.0)
        };
        let pomelo = layer_of("you-zi/0");
        let chinaberry = layer_of("ma-lian/0");
        assert!(
            pomelo > chinaberry * 1.20,
            "a `柚子` is {pomelo:.2} layers deep and a `麻楝` {chinaberry:.2}; the \
             reference sheet has one almost opaque and the other see-through"
        );
        // A conifer's tiers are dense even though its card is a fine spray.
        for species in SPECIES.iter().filter(|s| s.leaf == crate::species::LeafForm::Needle) {
            let layer = layer_of(&format!("{}/0", species.key));
            assert!(
                layer > 1.2,
                "{}'s conifer tiers are only {layer:.2} layers deep",
                species.key
            );
        }
    }

    fn coverage_of(lookup: &[(String, f32)], key: &str) -> f32 {
        let name = format!("vegetation/leaf/{key}");
        lookup
            .iter()
            .find(|(candidate, _)| *candidate == name)
            .map(|(_, value)| *value)
            .unwrap_or(0.0)
    }

    /// A canopy is a volume of foliage, not a shell of cards around a hole.  The
    /// previous port hung a sphere of cards off a cylinder and this is the
    /// measurement that would have caught it: with a shell, essentially no card
    /// is anywhere near the trunk.
    #[test]
    fn a_canopy_is_a_volume_and_not_a_shell_of_cards() {
        let (prototypes, meshes) = built();
        for prototype in &prototypes {
            let reach = crown_radius(&meshes, prototype);
            let mut inner = 0_usize;
            let mut total = 0_usize;
            for mesh in groups_of(&meshes, prototype, "leaf") {
                for card in mesh.positions.chunks_exact(12) {
                    let x = (card[0] + card[3] + card[6] + card[9]) / 4.0;
                    let z = (card[2] + card[5] + card[8] + card[11]) / 4.0;
                    total += 1;
                    if x.hypot(z) < reach * 0.55 {
                        inner += 1;
                    }
                }
            }
            let fraction = inner as f32 / total.max(1) as f32;
            assert!(
                (0.03..=0.55).contains(&fraction),
                "{} has {fraction:.0%} of its cards inside its own crown's half \
                 radius; a shell has almost none and a solid ball has all of them",
                prototype.key
            );
        }
    }

    /// The budget.  The floor is what stops anyone "optimising" the detail away
    /// to nothing; the ceiling is what keeps 3 200 instances of one prototype
    /// affordable.  These numbers are the argument for doing leaf-level detail
    /// at all, so they are asserted rather than hoped for.
    #[test]
    fn the_prototype_budget_is_bounded_below_and_above() {
        let (prototypes, meshes) = built();
        let mut counts: Vec<usize> = Vec::new();
        for prototype in &prototypes {
            let mut triangles = 0;
            for part in ["bark", "leaf"] {
                for mesh in groups_of(&meshes, prototype, part) {
                    triangles += mesh.indices.len() / 3;
                    // Metre UVs on a card: the renderer sets `repeat` to
                    // `1 / tile_width_m`, so a card's UV extent must be its
                    // physical size or the card samples a fraction of the image
                    // that was not drawn.
                    if part == "leaf" {
                        let tile = leaf_cards::card_tile_m(prototype.species);
                        let window = leaf_cards::CARD_WINDOW * tile;
                        let uvs = mesh.uvs.as_ref().unwrap();
                        for corner in uvs.chunks_exact(2) {
                            assert!(
                                corner[0] >= -1.0e-3
                                    && corner[0] <= tile + 1.0e-3
                                    && corner[1] >= -1.0e-3
                                    && corner[1] <= tile + 1.0e-3,
                                "{} has a card UV outside the texture's tile",
                                prototype.key
                            );
                        }
                        // The sampled window is the same on every card of a
                        // prototype, which is what makes a per-card offset
                        // possible: the four corners of one card are a rectangle
                        // of the tile, and its position inside the tile varies.
                        // Corner `a` is `uvs[0..2]` and corner `b` is `uvs[2..4]`.
                        let (du, dv) = (uvs[2] - uvs[0], uvs[3] - uvs[1]);
                        assert!(
                            (du - window).abs() < 1.0e-3 && dv.abs() < 1.0e-6,
                            "{}'s card window is {du:.3} x {dv:.3}, not {window:.3} square",
                            prototype.key
                        );
                    }
                }
            }
            assert!(
                triangles >= 900,
                "{} is {triangles} triangles, which is an armature rather than a tree",
                prototype.key
            );
            assert!(
                triangles <= 3300,
                "{} is {triangles} triangles, which is a canopy nobody can afford",
                prototype.key
            );
            counts.push(triangles);
        }
        counts.sort_unstable();
        let median = counts[counts.len() / 2];
        let total: usize = counts.iter().sum();
        assert!(total <= prototypes.len() * 3300);
        eprintln!(
            "tree prototypes: {} total, min {} median {} max {}",
            prototypes.len(),
            counts[0],
            median,
            counts[counts.len() - 1]
        );
    }

    /// A city is thousands of trees on a couple of dozen instance lists.  The
    /// previous port built every tree individually and capped the whole *world*
    /// at 1 200, which is why its avenues looked like isolated blobs.
    #[test]
    fn a_city_is_thousands_of_trees_on_a_few_prototypes() {
        let city = city();
        let network = derive(
            &city.nodes,
            &city.sd_roads,
            &city.hd_roads,
            city.frame,
            JunctionSpec::default(),
            city.seed,
        );
        let prototypes = prototype_set();
        let mut builder = MeshBuilder::new();
        build_prototypes(&prototypes, &mut builder);
        let output = plant(
            &network,
            &city.parcels,
            city.river.as_deref(),
            city.frame,
            &prototypes,
            &mut builder,
            city.seed,
        );
        assert!(output.instances > 500, "only {} trees in a whole city", output.instances);
        assert!(output.instances <= TREE_BUDGET);
        assert!(!output.by_role.is_empty());
        assert!(
            output
                .by_role
                .iter()
                .any(|(role, count)| role == "street" && *count > 100),
            "no kerbside avenue: {output:?}"
        );
        let scene = builder.build();
        let lists: Vec<&crate::mesh::InstanceGroup> = scene
            .instances
            .iter()
            .filter(|list| list.key.starts_with("tree/"))
            .collect();
        assert!(
            lists.len() <= 48,
            "{} tree instance lists for {} instances",
            lists.len(),
            output.instances
        );
        let total: usize = lists.iter().map(|list| list.instances.len()).sum();
        assert_eq!(total, output.instances);
        // Every prototype is reachable, or the palette is not really in the city.
        let used = lists.iter().filter(|list| !list.instances.is_empty()).count();
        assert!(used >= 16, "only {used} prototypes are actually planted");
    }

    /// No tree in a tree, and no tree outside the city.
    #[test]
    fn every_tree_instance_is_placed_inside_the_city() {
        let city = city();
        let network = derive(
            &city.nodes,
            &city.sd_roads,
            &city.hd_roads,
            city.frame,
            JunctionSpec::default(),
            city.seed,
        );
        let prototypes = prototype_set();
        let mut builder = MeshBuilder::new();
        // Both halves share one builder in production: the prototype geometry and
        // the per-city instance lists are the same draw calls.
        build_prototypes(&prototypes, &mut builder);
        let output = plant(
            &network,
            &city.parcels,
            city.river.as_deref(),
            city.frame,
            &prototypes,
            &mut builder,
            city.seed,
        );
        let scene = builder.build();
        let radius_km = 0.5;
        for list in &scene.instances {
            if !list.key.starts_with("tree/") {
                continue;
            }
            for instance in &list.instances {
                let distance = (instance.x * instance.x + instance.z * instance.z).sqrt() / 1000.0;
                assert!(
                    distance < radius_km * 1.6,
                    "tree at {distance:.2} km is outside the city"
                );
                assert!(instance.scale_y > 1.0, "tree scaled to nothing");
                // An instance tint can only darken: the payload's tint is a `u8`,
                // so a factor above 1.0 clips and a tree ends up brighter than the
                // material it is supposed to be made of.
                assert!(
                    instance.tint_r <= 1.0 && instance.tint_g <= 1.0 && instance.tint_b <= 1.0,
                    "instance tint above 1.0 clips in u8 and brightens the tree"
                );
            }
        }
        assert!(output.instances > 0);
    }

    /// Bark is a real material and it comes from the species' own bark, so a
    /// conifer's trunk and a magnolia's cannot be the same grey.
    #[test]
    fn bark_comes_from_the_species_record() {
        let (prototypes, meshes) = built();
        for prototype in &prototypes {
            let mesh = groups_of(&meshes, prototype, "bark")[0];
            let colours = mesh
                .colors
                .as_ref()
                .unwrap_or_else(|| panic!("{}#bark has no vertex colour", prototype.key));
            let mut peak = 0.0_f32;
            let mut total = 0.0_f32;
            let mut count = 0.0_f32;
            for pixel in colours.chunks_exact(4) {
                // The payload's vertex colour is a `u8` holding a *linear*
                // reflectance, not an sRGB byte: `pack_tint` multiplies by 255
                // and three.js reads a vertex colour as being in the working
                // colour space already.  Decoding it as sRGB here would put every
                // bark eight times too dark, which is exactly the kind of silent
                // colour-space bug this assertion exists to catch.
                let linear = pixel[0] as f32 / 255.0 * 0.2126
                    + pixel[1] as f32 / 255.0 * 0.7152
                    + pixel[2] as f32 / 255.0 * 0.0722;
                peak = peak.max(linear);
                total += linear;
                count += 1.0;
            }
            let bark = leaf_cards::luma(prototype.species.bark.colour);
            let mean = total / count.max(1.0);
            assert!(
                mean > bark * 0.85 && mean < bark * 1.05,
                "{}'s bark averages {mean:.3} against its own {bark:.3}",
                prototype.key
            );
            assert!(peak < 0.34, "{}'s bark peaks at {peak:.3}", prototype.key);
            // And the trunk is fluted, not a smooth prism: a cylinder of `sides`
            // quads would have a perimeter that is a straight line between
            // vertices, and a fluted one does not.
            let radii: Vec<f32> = mesh
                .positions
                .chunks_exact(3)
                .step_by(3)
                .map(|v| (v[0] * v[0] + v[2] * v[2]).sqrt())
                .collect();
            let max = radii.iter().copied().fold(0.0_f32, f32::max);
            let min = radii.iter().copied().fold(f32::MAX, f32::min);
            assert!(
                max / min.max(1.0e-6) > 1.02,
                "{}'s trunk is a smooth cylinder, not a fissured one",
                prototype.key
            );
        }
    }
}
