//! The tree tests, measured on built geometry.
//!
//! These are the assertions that keep the species table honest: every species
//! builds, every prototype is unit height, every canopy form is a distinct
//! silhouette *on the vertices*, cards stay inside the crown envelope, and the
//! costs stay inside the budgets.

use super::cards::{CARD_WINDOW, card_tile_m, leaf_card_textures};
use super::grow::Grower;
use super::{
    MIN_CARDS, PRESENTED_FRACTION, TWO_VARIANT, TreePrototype, build_prototypes, declare,
    prototype_set, real_dimensions,
};
use crate::math::{Rng, Vec2, Vec3};
use crate::mesh::{GroupStyle, MeshBuilder, MeshGroup};
use crate::network::derive;
use crate::species::{Bark, Bloom, Canopy, LeafForm, SPECIES, Species};
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
        let card = card_tile_m(prototype.species) / prototype.height;
        assert!(
            (0.975..=1.0 + card).contains(&top),
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
/// mirror image: it opens upward.  The fastigiate poplar is as narrow as a
/// conifer without tapering to a point.  Measured on the built leaves, one
/// height band at a time, against the species' own form.
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
            Canopy::Layered => {
                // A deodar is not a spire — it is as wide as a broad-crowned
                // tree — but its tiers still narrow upward, less steeply than
                // the metasequoia's.
                assert!(ratio > 0.20, "{}'s crown is {ratio:.2}; a cedar's tiers are broad", species.key);
                let (foot, crown) = (reach(prototype, 0.10, 0.30), reach(prototype, 0.55, 0.75));
                assert!(
                    crown < foot * 0.92,
                    "{}'s tiers reach {crown:.4} in the upper band against {foot:.4} at \
                     the foot; a cedar narrows upward",
                    species.key
                );
            }
            Canopy::Fastigiate => {
                assert!(
                    ratio < 0.20,
                    "{}'s crown is {ratio:.2} of its height; a fastigiate column \
                     is not",
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
            Canopy::Banyan | Canopy::Rounded | Canopy::Open | Canopy::Oval | Canopy::Fan => {
                assert!(
                    ratio > 0.28,
                    "{}'s crown is only {ratio:.2} of its height; a broad-crowned \
                     street tree is far wider than tall",
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
/// difference between a `梧桐` you can see the sky through and a `榕树` you
/// cannot, measured on the built cards rather than asserted in a comment.
#[test]
fn canopy_opacity_follows_the_species_density() {
    let (prototypes, meshes) = built();
    // The opaque coverage of each species' own card, at the resolution the
    // payload actually ships, so the measure is physical: a card's geometry
    // times the fraction of its texture that is actually leaf.  A coarse bake
    // would understate it — a 64-pixel compound leaf loses its leaflets — and
    // the number has to be the one the renderer will see.
    let coverage: Vec<(String, f32)> = leaf_card_textures(256)
        .iter()
        .map(|texture| {
            let opaque = texture.rgba.chunks_exact(4).filter(|p| p[3] > 0).count();
            (
                texture.name.clone(),
                opaque as f32 / (texture.width * texture.height) as f32,
            )
        })
        .collect();
    let window = CARD_WINDOW;
    let mut dense: Vec<(f32, f32)> = Vec::new();
    let mut sparse: Vec<(f32, f32)> = Vec::new();
    let mut layers: Vec<(String, f32, f32)> = Vec::new();
    for prototype in &prototypes {
        let opacity = window * window * coverage_of(&coverage, prototype.species.key);
        let reach = crown_radius(&meshes, prototype);
        let silhouette = std::f32::consts::PI * reach * reach;
        let mut presented = 0.0_f32;
        let mut cards = 0.0_f32;
        for mesh in groups_of(&meshes, prototype, "leaf") {
            for card in mesh.positions.chunks_exact(12) {
                // A quad's corners run a, b, c, d, so a to c is its diagonal
                // and the card's own area is half the square of that.
                let dx = card[6] - card[0];
                let dy = card[7] - card[1];
                let dz = card[8] - card[2];
                let diagonal = (dx * dx + dy * dy + dz * dz).sqrt();
                let area = diagonal * diagonal * 0.5;
                // 0.75 is the mean projected area of a foliage card whose
                // normal is biased outward, and it is the same constant the
                // card count uses.
                presented += PRESENTED_FRACTION * area * opacity;
                cards += 1.0;
            }
        }
        let layer = presented / silhouette.max(1.0e-6);
        assert!(
            (1.2..=5.0).contains(&layer),
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
    let mean_cards = |values: &[(f32, f32)]| {
        values.iter().map(|v| v.1).sum::<f32>() / values.len() as f32
    };
    assert!(
        mean_cards(&dense) > mean_cards(&sparse) * 1.8,
        "the densest species need {} cards per unit of crown and the sparsest \
         {}; `density` is not reaching the geometry",
        mean_cards(&dense),
        mean_cards(&sparse)
    );
    // And the headline contrast, measured on the layers themselves: the
    // camphor is a solid evergreen mass and the parasol an open, thin crown.
    let layer_of = |key: &str| {
        layers
            .iter()
            .find(|(name, _, _)| name.ends_with(key))
            .map(|(_, layer, _)| *layer)
            .unwrap_or(0.0)
    };
    let camphor = layer_of("xiang-zhang/0");
    let parasol = layer_of("wu-tong/0");
    assert!(
        camphor > parasol * 1.20,
        "a `香樟` is {camphor:.2} layers deep and a `梧桐` {parasol:.2}; the \
         reference sheet has one almost opaque and the other see-through"
    );
    // A conifer's tiers are dense even though its card is a fine spray — and
    // they are still dense, because a 30 m metasequoia is read from a hundred
    // metres away and a thousand sub-metre cards is a lot of texture at that
    // range.
    for species in SPECIES.iter().filter(|s| s.leaf == LeafForm::Needle) {
        let layer = layer_of(&format!("{}/0", species.key));
        assert!(
            layer > 1.3,
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
            (0.02..=0.62).contains(&fraction),
            "{} has {:.0}% of its cards inside its own crown's half \
             radius; a shell has almost none and a solid ball has all of them",
            prototype.key,
            fraction * 100.0
        );
    }
}

/// Every one of the twelve canopy forms, each a distinct silhouette.
///
/// The measurement is the crown's outer reach at a quarter, a half and three
/// quarters of the *tree's* height — not of the crown's, so the clear stem
/// does real work: a `梧桐`'s empty lower half *is* its silhouette.  That is
/// the whole difference between a spire, a vase, a dome, a column, a stack of
/// drooping tiers and a curtain, and it is what a viewer reads at a kilometre.
#[test]
fn all_twelve_canopies_are_a_distinct_silhouette() {
    let base: Species = Species {
        key: "test-canopy",
        name_zh: "测试",
        name_en: "test",
        canopy: Canopy::Rounded,
        leaf: LeafForm::Elliptic,
        bark: Bark {
            colour: [0.20, 0.19, 0.18],
            fissure: 0.4,
            weathering: 1.0,
        },
        foliage: [0.14, 0.19, 0.12],
        autumn: None,
        bloom: Bloom {
            colour: None,
            density: 0.0,
            at: 0.0,
        },
        height_m: (9.0, 12.0),
        crown_m: (3.2, 4.6),
        trunk_m: (0.16, 0.24),
        clear_stem: 0.32,
        density: 0.70,
        leaf_scale: 0.30,
        evergreen: false,
        street_tolerant: true,
    };
    let clear_stem_of = |canopy: Canopy| match canopy {
        // A conifer carries a clear trunk most of its height; a cedar's lowest
        // tier hangs near the ground; a poplar's upward sweep starts low.
        Canopy::Conical => 0.58,
        Canopy::Layered => 0.18,
        Canopy::Fastigiate => 0.18,
        Canopy::Umbrella => 0.42,
        Canopy::Open => 0.45,
        Canopy::Fan => 0.44,
        Canopy::Weeping => 0.28,
        Canopy::Irregular => 0.35,
        Canopy::Banyan => 0.24,
        Canopy::Oval => 0.26,
        Canopy::Vase => 0.30,
        Canopy::Rounded => 0.30,
    };
    let canopies = [
        Canopy::Rounded,
        Canopy::Open,
        Canopy::Oval,
        Canopy::Fan,
        Canopy::Vase,
        Canopy::Conical,
        Canopy::Layered,
        Canopy::Weeping,
        Canopy::Fastigiate,
        Canopy::Irregular,
        Canopy::Umbrella,
        Canopy::Banyan,
    ];
    let mut signatures: Vec<(Canopy, [f32; 4], usize)> = Vec::new();
    for canopy in canopies {
        let species: &'static Species = Box::leak(Box::new(Species {
            key: match canopy {
                Canopy::Rounded => "test-rounded",
                Canopy::Open => "test-open",
                Canopy::Oval => "test-oval",
                Canopy::Fan => "test-fan",
                Canopy::Vase => "test-vase",
                Canopy::Conical => "test-conical",
                Canopy::Layered => "test-layered",
                Canopy::Weeping => "test-weeping",
                Canopy::Fastigiate => "test-fastigiate",
                Canopy::Irregular => "test-irregular",
                Canopy::Umbrella => "test-umbrella",
                Canopy::Banyan => "test-banyan",
            },
            canopy,
            clear_stem: clear_stem_of(canopy),
            ..base
        }));
        let mut rng = Rng::new(0x7ee0_0000 ^ (canopy as u32) << 5);
        let (height, crown, trunk) = real_dimensions(species, 0, &mut rng);
        let prototype = TreePrototype {
            key: format!("tree/{}/0", species.key),
            species,
            variant: 0,
            height,
            crown,
            trunk,
        };
        let bark = format!("{}#bark", prototype.key);
        let leaf = format!("{}#leaf", prototype.key);
        let mut builder = MeshBuilder::new();
        declare(&mut builder);
        builder.style(&bark, GroupStyle::default());
        builder.style(
            &leaf,
            GroupStyle {
                alpha_cutout: true,
                ..GroupStyle::default()
            },
        );
        let mut grower = Grower::new(&prototype, 0x1234_5678, &bark, &leaf, &mut builder);
        grower.build();
        let trunk_sides = grower.trunk_sides() as usize;
        let meshes = builder.build().meshes;
        let bark_group = meshes
            .iter()
            .find(|mesh| mesh.material == bark)
            .unwrap_or_else(|| panic!("{canopy:?} built no bark"));
        let leaf_group = meshes
            .iter()
            .find(|mesh| mesh.material == leaf)
            .unwrap_or_else(|| panic!("{canopy:?} built no leaves"));
        assert!(
            bark_group.indices.len() > 12,
            "{canopy:?} is a stick with no wood in it"
        );
        let cards = leaf_group.positions.len() / 3 / 4;
        assert!(cards >= MIN_CARDS, "{canopy:?} has only {cards} cards");
        let ground = bark_group
            .positions
            .chunks_exact(3)
            .filter(|v| v[1] < 0.01)
            .count();
        assert!(ground > 0, "{canopy:?} has no wood at the ground");
        if canopy == Canopy::Banyan {
            // A banyan's aerial roots are the whole point: several columns of
            // wood touch the ground besides the trunk.
            assert!(
                ground >= 16,
                "{canopy:?} has {ground} vertices at the ground, which is not a \
                 trunk plus aerial roots"
            );
        } else {
            assert!(
                ground < 2 * trunk_sides + 4,
                "{canopy:?} has {ground} vertices at the ground, which is more than \
                 one trunk of {trunk_sides} sides"
            );
        }
        let profile = crown_profile(&leaf_group.positions);
        let reach = profile.iter().copied().fold(0.0_f32, f32::max);
        // The signature is the crown's reach at a quarter, a half and three
        // quarters of the tree's height, plus the reach itself — the profile
        // tells the forms apart and the reach is the proportion a viewer reads
        // as "wide" or "tall".
        let widest = reach.max(1.0e-6);
        signatures.push((
            canopy,
            [
                profile[0] / widest,
                profile[1] / widest,
                profile[2] / widest,
                reach,
            ],
            cards,
        ));
    }
    // Every pair has to be a different shape, not a different seed.  With the
    // clear stem measured too, no two of the twelve are excused: a `雪松` and
    // a `榕树` both fill their lower crown, and the taper of the cedar's tiers
    // is what has to tell them apart.
    for (index, (a_canopy, a, _)) in signatures.iter().enumerate() {
        for (b_canopy, b, _) in signatures.iter().skip(index + 1) {
            let distance: f32 = (0..4).map(|i| (a[i] - b[i]).abs()).sum::<f32>() / 4.0;
            assert!(
                distance > 0.025,
                "{a_canopy:?} and {b_canopy:?} are the same silhouette: \
                 {a:?} against {b:?}"
            );
        }
    }
}

/// The crown's outer reach at a quarter, a half and three quarters of the
/// *tree's* height, in the crown's own units — so a high clear stem reads as
/// an empty band, which is what it is.
fn crown_profile(positions: &[f32]) -> [f32; 3] {
    let mut band = [0.0_f32; 3];
    for card in positions.chunks_exact(12) {
        let y = (card[1] + card[4] + card[7] + card[10]) / 4.0;
        let x = (card[0] + card[3] + card[6] + card[9]) / 4.0;
        let z = (card[2] + card[5] + card[8] + card[11]) / 4.0;
        let radius = x.hypot(z);
        for (index, centre) in [0.25_f32, 0.5, 0.75].iter().enumerate() {
            if (y - centre).abs() < 0.08 {
                band[index] = band[index].max(radius);
            }
        }
    }
    band
}

/// Cost.  The floor is what stops anyone "optimising" the detail away
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
                    let tile = card_tile_m(prototype.species);
                    let window = CARD_WINDOW * tile;
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
            triangles >= 800,
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
    let output = super::plant(
        &network,
        &city.parcels,
        city.river.as_deref(),
        city.frame,
        &prototypes,
        &mut builder,
        city.seed,
    );
    assert!(output.instances > 500, "only {} trees in a whole city", output.instances);
    assert!(output.instances <= super::TREE_BUDGET);
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
    let output = super::plant(
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
        let bark = super::cards::luma(prototype.species.bark.colour);
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

/// The willow actually weeps: below the crown's shoulder there are cards, and
/// the hanging shoots' cards are strung *vertically* — a card's long axis on
/// a hanging shoot points down, not out.
#[test]
fn the_willow_hangs_and_the_pine_leans() {
    let (prototypes, meshes) = built();
    let willow = prototypes
        .iter()
        .find(|p| p.species.key == "liu-shu")
        .expect("the willow is in the palette");
    let base = (willow.species.clear_stem * 0.80).clamp(0.10, 0.86);
    // Cards below the shoulder: the curtain.
    let mut low = 0;
    for mesh in groups_of(&meshes, willow, "leaf") {
        for card in mesh.positions.chunks_exact(12) {
            let y = (card[1] + card[4] + card[7] + card[10]) / 4.0;
            if y < base + 0.35 * (1.0 - base) {
                low += 1;
            }
        }
    }
    assert!(low > 0, "the willow grew no curtain below its shoulder");
    // The pine leans: its trunk's top is displaced from its foot by a
    // discernible fraction of the tree's height, and no other single-trunk
    // species leans like that.
    let pine = prototypes
        .iter()
        .find(|p| p.species.key == "song-shu")
        .expect("the pine is in the palette");
    let bark = groups_of(&meshes, pine, "bark")[0];
    let mut top_x = 0.0_f32;
    for vertex in bark.positions.chunks_exact(3) {
        if vertex[1] > 0.30 {
            top_x = top_x.max(vertex[0].abs());
        }
    }
    assert!(
        top_x > 0.03,
        "the pine's trunk above 30% height is displaced only {top_x:.3}; a \
         reference-sheet pine leans"
    );
}
