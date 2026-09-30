//! Trees, built once per species as prototypes and instanced.
//!
//! # What a tree is, geometrically
//!
//! The previous port modelled a tree as one of four crown shapes — broad,
//! conical, weeping, clump — hung on a cylinder, with five green tones. That
//! is why every street looked like the same four stamps: the *species* was a
//! number, so a `水杉` and a `麻楝` were the same silhouette in different
//! greens, and neither had any wood in it above the trunk.
//!
//! Here a tree is built the way a tree is built. A trunk with a real taper and
//! a real flare; primary limbs; secondary branches; and then leaf cards at the
//! branch tips, and only there. The branching is recursive, the reach of every
//! limb is the crown's own profile, and the profile is one of the twelve
//! canopy forms in [`forms`] — one per [`Canopy`] value — so:
//!
//! * a `水杉` is a narrow spire with a clear trunk for most of its height and
//!   whorled, ascending, conical tiers of foliage;
//! * a `雪松` is a stack of long horizontal tiers drooping at the tips;
//! * a `柳树` pours off its shoulders in hanging curtains of shoot;
//! * a `杨树` is a column whose branches sweep steeply up;
//! * a `松树` leans, and carries flat needle plates with gaps between them;
//! * a `榕树` drops aerial roots from its limbs to the ground.
//!
//! Nothing here is a stylisation. There is no rim light, no outline, no
//! painted occlusion and no brightening. The albedo is the species' own
//! reflectance, the leaf card is alpha-cut, and a card's only shading bias is
//! the quarter-turn toward the sky that a thin, scattering leaf genuinely has.
//!
//! # Unit height, and what it buys
//!
//! Prototypes are authored at **unit height** and instanced with a real metre
//! scale, so a 6 m street tree and a 24 m metasequoia share one mesh. A
//! prototype is 900-3 000 triangles — expensive per tree, irrelevant per city,
//! because sixteen species across twenty-four prototypes is forty-eight draw
//! calls for *every* tree in *every* city. That is the trade: the detail goes
//! into the prototype because the prototype is paid for once.
//!
//! # Module map
//!
//! * [`forms`] — one module per canopy form: the profile curve and the branch
//!   numbers that make each silhouette.
//! * [`grow`] — the grower that interprets a form record into wood and cards.
//! * [`cards`] — the per-species leaf-card geometry and textures.
//! * [`plant`] — where the city's trees go, and which species a street, park,
//!   median, courtyard, riverbank or roundabout gets.
//! * [`shrub`] — the clipped shrub and grass-tuft prototypes.

pub(crate) mod cards;
/// Publicly nameable but entirely `pub(crate)` inside: a documentation anchor,
/// so [`crate::species`] can point at the canopy records by path.
pub mod forms;
pub mod far;
mod grow;
mod plant;
mod shrub;

pub use cards::{
    CARD_WINDOW, card_tile_m, chromaticity, colour_distance, leaf_card_textures, luma,
    mean_albedo_rgb, opaque_albedo, target_coverage, tuft_texture,
};
pub use far::{FAR_SPECIES, FarTree, FarTreePayload, far_tree_payload, far_tree_set};
pub use plant::{TreeOutput, TreeRole, plant, species_for_parcel};
pub use shrub::{build_shrub_prototype, build_tuft_prototype, plant_median_shrubs};

use crate::math::Rng;
use crate::mesh::MeshBuilder;
use crate::species::{SPECIES, Species};

/// Two radians of pi, shared by every sub-module's trig.
pub(crate) const TAU: f32 = std::f32::consts::PI * 2.0;

/// Per-city tree budget.
pub const TREE_BUDGET: usize = 3200;
/// Tufts and low shrubs share a much smaller budget; they are ground clutter.
pub const TUFT_BUDGET: usize = 6000;

/// How many prototypes a species is built as. One for a park tree that appears
/// forty times, two for an avenue tree that appears four hundred.
pub(crate) fn two_variant_count(species: &Species) -> usize {
    if TWO_VARIANT.contains(&species.key) { 2 } else { 1 }
}

/// The eight species a Chinese avenue is actually made of, and therefore the
/// only ones worth a second silhouette. A variant is not a recolour: variant 0
/// is the tall narrow tree and variant 1 the broad low one, which is a real
/// difference in a real street — the same species planted on the north and
/// south kerb of the same road. Everything else gets one prototype, because a
/// park tree that appears forty times does not need its silhouette varied.
const TWO_VARIANT: [&str; 8] = [
    "xiang-zhang",
    "yin-xing",
    "wu-tong",
    "yu-shu",
    "xue-song",
    "liu-shu",
    "shui-shan",
    "yang-shu",
];

/// The floor and ceiling on a canopy's card count. The floor is what stops a
/// "simplification" from turning a species into a bare armature; the ceiling is
/// what keeps 3 200 instances of one prototype affordable.
pub(crate) const MIN_CARDS: usize = 220;
pub(crate) const MAX_CARDS: usize = 2200;

/// The share of a card's own area a card presents to any given view.
///
/// A flat card at a random angle presents half its area, but these cards are not
/// at random angles: their normal is mostly the crown's outward radial, because
/// the face the world sees is the face the card shows. Three quarters is the
/// mean, and it is used in exactly two places — the card count and the test that
/// checks the card count — so the two cannot drift apart.
pub(crate) const PRESENTED_FRACTION: f32 = 0.75;

/// One prototype: a species and a variant, built at unit height so the instance
/// scale supplies the real metres.
pub struct TreePrototype {
    pub key: String,
    /// The species record every number below and every vertex of the mesh is
    /// derived from. Held by reference so the table stays the single source of
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
        let variants = two_variant_count(species);
        for variant in 0..variants {
            let variant = variant as u16;
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
    // A conifer's or a poplar's crown is set by its age and girth rather than
    // by its height, so a stunted `水杉` is a narrow thing and not a squat wide
    // one: its two variants differ in height, not in habit.
    let crown_fraction = match species.canopy {
        crate::species::Canopy::Conical
        | crate::species::Canopy::Layered
        | crate::species::Canopy::Fastigiate => {
            if broad {
                (0.74, 0.88)
            } else {
                (0.58, 0.80)
            }
        }
        _ => {
            if broad {
                (0.92, 1.00)
            } else {
                (0.60, 0.82)
            }
        }
    };
    let crown = lerp_between(species.crown_m, crown_fraction, rng);
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
        crate::mesh::GroupStyle {
            cast_shadow: true,
            receive_shadow: true,
            alpha_cutout: false,
            dynamic: false,
        },
    );
    builder.style(
        "leaf",
        crate::mesh::GroupStyle {
            cast_shadow: true,
            receive_shadow: true,
            alpha_cutout: true,
            dynamic: false,
        },
    );
    builder.style(
        "hedge",
        crate::mesh::GroupStyle {
            cast_shadow: true,
            receive_shadow: true,
            alpha_cutout: false,
            dynamic: false,
        },
    );
    builder.style(
        "tuft",
        crate::mesh::GroupStyle {
            cast_shadow: false,
            receive_shadow: true,
            alpha_cutout: true,
            dynamic: false,
        },
    );
}

/// Build every prototype's geometry. Called once per generation, not per city:
/// the geometry is identical for every city in the world.
pub fn build_prototypes(prototypes: &[TreePrototype], builder: &mut MeshBuilder) {
    declare(builder);
    for prototype in prototypes {
        let bark = format!("{}#bark", prototype.key);
        let leaf = format!("{}#leaf", prototype.key);
        builder.style(
            &bark,
            crate::mesh::GroupStyle {
                cast_shadow: true,
                receive_shadow: true,
                alpha_cutout: false,
                dynamic: false,
            },
        );
        builder.style(
            &leaf,
            crate::mesh::GroupStyle {
                cast_shadow: true,
                receive_shadow: true,
                alpha_cutout: true,
                dynamic: false,
            },
        );
        let mass = format!("{}#mass", prototype.key);
        builder.style(
            &mass,
            crate::mesh::GroupStyle {
                cast_shadow: true,
                receive_shadow: true,
                alpha_cutout: false,
                dynamic: false,
            },
        );
        let seed = hash_seed(prototype.species.key, prototype.variant);
        let mut grower = grow::Grower::new(prototype, seed, &bark, &leaf, builder);
        grower.build();
        // One transform list places the whole tree: bark and foliage are two
        // groups, one instance buffer.
        builder.bind(&bark, &prototype.key);
        builder.bind(&leaf, &prototype.key);
        builder.bind(&mass, &prototype.key);
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

#[cfg(test)]
mod tests;
