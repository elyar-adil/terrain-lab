//! Species presets grown through the shared L-System, plus the prototype
//! builder that turns a grammar into renderable geometry.  Proportions follow
//! the real trees: a 樟树 camphor carries a broad dense crown on a 4 m clear
//! trunk, a ginkgo grows narrowly when young, willow shoots droop, and bamboo
//! is nearly all culm.

use crate::lsystem::{LSystem, TurtleParams, interpret};
use crate::{Lod, Species, SplitMix64, TreeGeometry, TreePrototype};

/// Resolve a species by its payload name; the renderer uses this to map
/// instance payloads to the matching prototype group.
pub fn species_from_name(name: &str) -> Option<Species> {
    match name.trim().to_ascii_lowercase().as_str() {
        "camphor" | "chineseplane" | "chinese_plane" => Some(Species::Camphor),
        "ginkgo" => Some(Species::Ginkgo),
        "willow" => Some(Species::Willow),
        "cedar" => Some(Species::Cedar),
        "bamboo" => Some(Species::Bamboo),
        "londonplane" | "london_plane" | "plane" => Some(Species::LondonPlane),
        _ => None,
    }
}

struct SpeciesModel {
    grammar: LSystem,
    iterations_near: usize,
    iterations_far: usize,
    params: TurtleParams,
    jitter_deg: f32,
}

impl SpeciesModel {
    /// The same grammar, less expanded: the far LOD keeps the primary
    /// skeleton and enlarges the foliage blobs so silhouettes still match.
    /// Segment and blob counts are capped so the prototype payload stays a
    /// few dozen kilobytes regardless of grammar depth.
    fn grow(&self, lod: Lod, seed: u64) -> TreeGeometry {
        let iterations = match lod {
            Lod::Near => self.iterations_near,
            Lod::Far => self.iterations_far,
        };
        let symbols = self.grammar.expand(iterations);
        let mut rng = SplitMix64::new(seed);
        let (all_segments, mut foliage) =
            interpret(&symbols, &self.params, self.jitter_deg, &mut rng);
        // Drop hair-thin twigs first, then hard-cap the count; parents are
        // emitted before children, so truncation keeps the main skeleton.
        let (max_segments, max_foliage) = match lod {
            // Instance budgets: with tens of thousands of trees per world the
            // far LOD must stay near a few hundred triangles per tree, and the
            // near LOD only ever shows inside a couple of city blocks.
            Lod::Near => (240, 32),
            Lod::Far => (90, 14),
        };
        let mut segments: Vec<crate::Segment> = all_segments
            .into_iter()
            .filter(|segment| segment.radius_end >= 0.012)
            .collect();
        // When the grammar still overflows the cap, keep the thickest
        // segments (the structural skeleton) rather than the first ones in
        // string order, then restore emission order for the renderer.
        if segments.len() > max_segments {
            let mut ranked: Vec<(usize, crate::Segment)> =
                segments.into_iter().enumerate().collect();
            ranked.sort_by(|a, b| {
                b.1.radius_start
                    .total_cmp(&a.1.radius_start)
                    .then(b.1.end[1].total_cmp(&a.1.end[1]))
            });
            ranked.truncate(max_segments);
            ranked.sort_by_key(|(index, _)| *index);
            segments = ranked.into_iter().map(|(_, segment)| segment).collect();
        }
        segments.truncate(max_segments);
        foliage.truncate(max_foliage);
        let mut height = 0.0_f32;
        let mut crown = 0.0_f32;
        for segment in &segments {
            height = height.max(segment.end[1]);
            crown = crown
                .max((segment.end[0] * segment.end[0] + segment.end[2] * segment.end[2]).sqrt());
        }
        for blob in &foliage {
            height = height.max(blob.centre[1] + blob.radius * 0.5);
            crown = crown.max(
                (blob.centre[0] * blob.centre[0] + blob.centre[2] * blob.centre[2]).sqrt()
                    + blob.radius,
            );
        }
        if segments.is_empty() {
            // Degenerate grammar guard: one bare culm keeps instances sane.
            segments.push(crate::Segment {
                start: [0.0, 0.0, 0.0],
                end: [0.0, self.params.step, 0.0],
                radius_start: self.params.base_radius,
                radius_end: self.params.base_radius * 0.8,
            });
            height = self.params.step;
        }
        if foliage.is_empty() {
            foliage.push(crate::FoliageBlob {
                centre: [0.0, height * 0.85, 0.0],
                radius: self.params.foliage_radius,
                density: 0.8,
            });
        }
        if matches!(lod, Lod::Far) {
            // Enlarge the remaining lobes so the cheap pass still draws a
            // solid crown silhouette from several hundred metres.
            for blob in &mut foliage {
                blob.radius *= 1.7;
                blob.density = (blob.density + 0.2).min(1.0);
            }
        }
        let crown_total = (crown + self.params.foliage_radius * 2.0).max(0.5);
        TreeGeometry {
            height_metres: (height + self.params.foliage_radius * 0.5).max(0.5),
            crown_radius_metres: crown_total,
            segments,
            foliage,
        }
    }
}

fn model(species: Species) -> SpeciesModel {
    let params =
        |step: f32, angle: f32, radius: f32, decay: f32, tropism: f32, foliage: f32| TurtleParams {
            step,
            angle_deg: angle,
            base_radius: radius,
            radius_decay: decay,
            step_decay: 0.96,
            tropism: crate::Tropism { up: tropism },
            foliage_radius: foliage,
        };
    match species {
        // 樟树: strong central leader through 2/3 of the height, then a wide,
        // dense evergreen crown.  12-20 m street specimens.
        Species::Camphor => SpeciesModel {
            grammar: LSystem::new("A").with_rule('A', "F[&FLA]////[&FLA]////[&FLA]F"),
            iterations_near: 5,
            iterations_far: 3,
            params: params(1.65, 42.0, 0.26, 0.82, 0.30, 1.9),
            jitter_deg: 6.0,
        },
        // 银杏: narrow crown, steep branches, sparse distinctive foliage.
        Species::Ginkgo => SpeciesModel {
            grammar: LSystem::new("A").with_rule('A', "F[^^FLA]//[^^FLA]//[^^FLA]FA"),
            iterations_near: 5,
            iterations_far: 3,
            params: params(1.25, 34.0, 0.22, 0.80, 0.5, 1.35),
            jitter_deg: 5.0,
        },
        // 垂柳: short trunk, shoots arch outward then droop under gravity.
        Species::Willow => SpeciesModel {
            grammar: LSystem::new("A").with_rule('A', "FF[&&FLA]////[&&FLA]////[&&FLA]FA"),
            iterations_near: 5,
            iterations_far: 3,
            params: params(1.95, 48.0, 0.24, 0.82, -0.45, 1.5),
            jitter_deg: 7.0,
        },
        // 雪松: whorled conifer, branches sweep down then flatten out.
        Species::Cedar => SpeciesModel {
            grammar: LSystem::new("A").with_rule('A', "F[&&&FLA]////[&&&FLA]////[&&&FLA]FA"),
            iterations_near: 6,
            iterations_far: 3,
            params: params(1.35, 55.0, 0.30, 0.86, 0.62, 1.1),
            jitter_deg: 3.0,
        },
        // 竹: thin culms, tiny foliage high up.
        Species::Bamboo => SpeciesModel {
            grammar: LSystem::new("A").with_rule('A', "FFFFFFFF[&FLA]F[&FLA]F"),
            iterations_near: 2,
            iterations_far: 1,
            params: params(0.72, 26.0, 0.045, 0.98, 0.02, 0.5),
            jitter_deg: 4.0,
        },
        // 法国梧桐: mottled bark, chunky branches, huge plane-like crown.
        Species::LondonPlane => SpeciesModel {
            grammar: LSystem::new("A").with_rule('A', "F[&FLA]///[&FLA]///[&FLA]///[&FLA]FA"),
            iterations_near: 5,
            iterations_far: 3,
            params: params(1.2, 46.0, 0.3, 0.75, 0.3, 2.2),
            jitter_deg: 8.0,
        },
    }
}

/// Grow one prototype of a species at a given LOD.  `seed` varies the jitter
/// so a city can request several variants per species.
pub fn build_prototype(species: Species, lod: Lod, seed: u64) -> TreePrototype {
    let model = model(species);
    let geometry: TreeGeometry = model.grow(lod, seed);
    TreePrototype {
        species,
        lod,
        segments: geometry.segments_flat(),
        foliage: geometry.foliage_flat(),
        height_metres: geometry.height_metres,
        crown_radius_metres: geometry.crown_radius_metres,
    }
}

/// Grow the standard prototype pair (near + far) for a species.
pub fn build_prototype_pair(species: Species, seed: u64) -> [TreePrototype; 2] {
    [
        build_prototype(species, Lod::Near, seed),
        build_prototype(species, Lod::Far, seed ^ 0x5f37_59df),
    ]
}

/// The species order every emitter and renderer agrees on; instance payloads
/// reference `(species, variant)` and the client resolves the matching LOD.
pub const STANDARD_SPECIES: [Species; 6] = [
    Species::Camphor,
    Species::Ginkgo,
    Species::Willow,
    Species::Cedar,
    Species::Bamboo,
    Species::LondonPlane,
];

/// One deterministic prototype per (species, variant, LOD).  `variants` is
/// normally 2; the whole set is emitted once per generation request and
/// instanced across every city and forest in the world.
pub fn standard_prototype_set(variants: usize) -> Vec<TreePrototype> {
    let mut prototypes = Vec::with_capacity(STANDARD_SPECIES.len() * variants * 2);
    for (species_index, species) in STANDARD_SPECIES.iter().enumerate() {
        for variant in 0..variants {
            let seed = 0x51_7c_c1b7
                ^ (species_index as u64).wrapping_mul(0x9e37_79b9)
                ^ (variant as u64).wrapping_mul(0x85eb_ca6b);
            prototypes.push(build_prototype(*species, Lod::Near, seed));
            prototypes.push(build_prototype(*species, Lod::Far, seed ^ 0x5f37_59df));
        }
    }
    prototypes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prototypes_are_proportioned_like_real_trees() {
        for species in [
            Species::Camphor,
            Species::Ginkgo,
            Species::Willow,
            Species::Cedar,
            Species::LondonPlane,
        ] {
            let tree = build_prototype(species, Lod::Near, 42);
            // Street trees: 7-30 m tall, crown radius proportional but never
            // degenerate, trunk thicker than a broomstick.
            assert!(
                (7.0..=30.0).contains(&tree.height_metres),
                "{species:?} height {}",
                tree.height_metres
            );
            assert!(tree.crown_radius_metres >= 1.0);
            assert!(tree.segments.len() >= 8);
            // The flattened segment layout is [sx,sy,sz,ex,ey,ez,r0,r1]; the
            // trunk's base radius is the seventh float of the first segment.
            let trunk_base_radius = tree.segments[6];
            assert!(trunk_base_radius >= 0.15, "{species:?} trunk too thin");
            // Branch count grows with LOD detail.
            assert!(tree.segments.len() > 20);
        }
        let bamboo = build_prototype(Species::Bamboo, Lod::Near, 42);
        assert!((4.0..=12.0).contains(&bamboo.height_metres));
        assert!(bamboo.segments[6] < 0.08);
    }

    #[test]
    fn far_lod_is_cheaper_but_same_species() {
        let near = build_prototype(Species::Camphor, Lod::Near, 5);
        let far = build_prototype(Species::Camphor, Lod::Far, 5);
        assert_eq!(far.species, near.species);
        assert!(far.segments.len() <= near.segments.len());
        assert!(far.foliage.len() <= near.foliage.len());
    }

    #[test]
    fn species_names_resolve() {
        assert_eq!(species_from_name("camphor"), Some(Species::Camphor));
        assert_eq!(species_from_name("ginkgo"), Some(Species::Ginkgo));
        assert_eq!(species_from_name("nope"), None);
    }
}
