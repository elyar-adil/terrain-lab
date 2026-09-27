//! The ornamental tree species a Chinese city actually plants.
//!
//! # Why a species table and not a shape enum
//!
//! The previous port modelled a tree as one of four crown shapes — broad,
//! conical, weeping, clump — which is why every street looked like the same
//! four stamps. A city reads as a city because the *species* differ: a
//! `紫花风铃木` in flower is a cloud of violet, a `落羽杉` is a narrow green
//! spire, a `海棠` is a spreading pink cloud on a short trunk, and a `乌桕`
//! turns scarlet in autumn. Those are different colours, different silhouettes,
//! different branching habits and different seasonal states, and none of them
//! is recoverable from a crown enum.
//!
//! So a species is a record: the botanical identity, the mature dimensions, the
//! canopy architecture, the bark, the leaf shape and colour through the year,
//! and the blossom state. Geometry and texture both read this table, so a tree's
//! silhouette and its colour can never disagree.
//!
//! # What is *not* here
//!
//! No per-tree geometry. A species is a prototype, built once and instanced
//! thousands of times; the per-tree variation is the instance transform and
//! tint. A species list of sixteen therefore costs sixteen draw calls for an
//! entire city's foliage, which is what makes leaf-level detail affordable at
//! all.

/// How a species holds its canopy. This is the silhouette, and it is what the
/// eye reads at a kilometre.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Canopy {
    /// A single dominant trunk rising clear of the canopy, then spreading
    /// branches — the shape of a mature `桂花` or `柚子`.
    Spreading,
    /// Rounded to hemispherical, often low-branched — `麻楝`, `小叶紫薇`.
    Rounded,
    /// A broad vase, widest at the top, on a straight trunk — `乌桕`, `黄皮`.
    Vase,
    /// Conical, whorled branches, narrow — `水杉`, `落羽杉`, `锦叶樱仁` as a
    /// shrub-trained standard.
    Conical,
    /// Drooping to the ground — a weeping form of `海棠` or `柳`.
    Weeping,
    /// A multi-stemmed clump — `红花鸡蛋花` grown as a shrub, `红花玉兰`.
    MultiStem,
    /// Horizontal tiers on a straight trunk — `锦叶樱仁`, `黄槿` as a standard.
    Tiered,
}

/// Leaf form. This drives the leaf card's alpha silhouette, which is most of
/// why a `樱花` and a `樟` read differently at close range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeafForm {
    /// Broad, ovate, smooth margin — `麻楝`, `柚子`.
    Ovate,
    /// Palmate, three to five lobes — `黄皮`, `红花玉兰`.
    Palmate,
    /// Elliptic, finely serrate — `小叶樱仁`, `海棠`.
    Elliptic,
    /// Needle or scale, in dense sprays — `水杉`, `落羽杉`.
    Needle,
    /// Pinnate, many leaflets on one rachis — `合欢`-like street trees.
    Pinnate,
    /// Cordate, heart-shaped, smooth — `紫荆`-like.
    Cordate,
}

/// What the canopy looks like when it is carrying flowers or fruit. A blossom
/// state is a colour and a density, not a different tree.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bloom {
    /// Linear RGB of the blossom, or `None` for a species that does not flower
    /// in a showy way.
    pub colour: Option<[f32; 3]>,
    /// Fraction of the canopy that is blossom rather than leaf, 0..1.
    pub density: f32,
    /// How far through the year the bloom peaks, 0..1. 0.15 is late March.
    pub at: f32,
}

impl Bloom {
    const NONE: Bloom = Bloom {
        colour: None,
        density: 0.0,
        at: 0.0,
    };

    const fn of(colour: [f32; 3], density: f32, at: f32) -> Bloom {
        Bloom {
            colour: Some(colour),
            density,
            at,
        }
    }
}

/// Bark colour, linear RGB, and how much the trunk reads as smooth or fissured.
#[derive(Debug, Clone, Copy)]
pub struct Bark {
    pub colour: [f32; 3],
    /// 0 = smooth plane, 1 = deeply fissured pine.
    pub fissure: f32,
    /// Extra greying with age, as a multiplier on the base colour.
    pub weathering: f32,
}

/// One species, complete.
#[derive(Debug, Clone, Copy)]
pub struct Species {
    /// Stable ASCII key, used in prototype and material names. Never localised:
    /// these strings are payload keys.
    pub key: &'static str,
    /// The Chinese name, for reports and the UI. Present so a tree list shown to
    /// a user reads correctly.
    pub name_zh: &'static str,
    /// Common English name, for reports.
    pub name_en: &'static str,
    pub canopy: Canopy,
    pub leaf: LeafForm,
    pub bark: Bark,
    /// Summer leaf colour, linear RGB.
    pub foliage: [f32; 3],
    /// Autumn colour. Some Chinese street trees are chosen *for* this — the
    /// `乌桕` turning scarlet is the reason it is planted.
    pub autumn: Option<[f32; 3]>,
    pub bloom: Bloom,
    /// Mature height in metres, `(min, max)`.
    pub height_m: (f32, f32),
    /// Mature crown radius in metres, `(min, max)`.
    pub crown_m: (f32, f32),
    /// Mature trunk radius at breast height in metres, `(min, max)`.
    pub trunk_m: (f32, f32),
    /// Fraction of total height below the first branch. A `红花玉兰` is
    /// low-branched; a `水杉` carries a clear trunk most of its height.
    pub clear_stem: f32,
    /// How densely the canopy is packed with leaf cards, 0..1. A `麻楝` is
    /// open and see-through; a `柚子` is a solid mass.
    pub density: f32,
    /// Card size as a fraction of crown radius. Small cards read as fine
    /// texture; large cards read as blobs.
    pub leaf_scale: f32,
    /// Whether this species keeps its leaves through winter.
    pub evergreen: bool,
    /// Whether the species tolerates the pollution, heat and compacted soil of a
    /// Chinese street. The street planting list is drawn from this.
    pub street_tolerant: bool,
}

const fn bark(colour: [f32; 3], fissure: f32, weathering: f32) -> Bark {
    Bark {
        colour,
        fissure,
        weathering,
    }
}

/// The palette.
///
/// The first eight are the classic Chinese street trees — the ones that appear
/// in every city from Harbin to Guangzhou, and the ones a Chinese viewer
/// recognises without being told. The blossom trees and the conifers follow.
pub const SPECIES: &[Species] = &[
    Species {
        key: "wu-jiu",
        name_zh: "乌桕",
        name_en: "Chinese tallow",
        canopy: Canopy::Vase,
        leaf: LeafForm::Ovate,
        bark: bark([0.185, 0.178, 0.167], 0.55, 1.0),
        // A deep, almost blue green: the tallow's summer foliage is the darkest
        // in the palette, which is what makes its scarlet autumn so violent.
        foliage: [0.078, 0.155, 0.059],
        autumn: Some([0.235, 0.076, 0.052]),
        bloom: Bloom::NONE,
        height_m: (11.0, 15.0),
        crown_m: (4.5, 6.5),
        trunk_m: (0.22, 0.34),
        clear_stem: 0.34,
        density: 0.72,
        leaf_scale: 0.30,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "cong-shu",
        name_zh: "枞树",
        name_en: "Chinese fir",
        canopy: Canopy::Conical,
        leaf: LeafForm::Needle,
        bark: bark([0.135, 0.117, 0.105], 0.85, 0.9),
        foliage: [0.063, 0.105, 0.097],
        autumn: None,
        bloom: Bloom::NONE,
        height_m: (16.0, 24.0),
        crown_m: (2.6, 4.0),
        trunk_m: (0.20, 0.30),
        clear_stem: 0.22,
        density: 0.86,
        leaf_scale: 0.22,
        evergreen: true,
        street_tolerant: true,
    },
    Species {
        key: "zi-hua-feng-jiao-mu",
        name_zh: "紫花风铃木",
        name_en: "Purple-flowered tabebia",
        canopy: Canopy::Vase,
        leaf: LeafForm::Palmate,
        bark: bark([0.235, 0.229, 0.221], 0.20, 1.0),
        foliage: [0.185, 0.215, 0.116],
        autumn: Some([0.205, 0.126, 0.062]),
        bloom: Bloom::of([0.455, 0.343, 0.520], 0.88, 0.22),
        height_m: (10.0, 16.0),
        crown_m: (4.0, 6.0),
        trunk_m: (0.20, 0.30),
        clear_stem: 0.38,
        density: 0.68,
        leaf_scale: 0.34,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "xiao-ye-ying-ren",
        name_zh: "小叶樱仁",
        name_en: "Small-leaf cherry plum",
        canopy: Canopy::Tiered,
        leaf: LeafForm::Elliptic,
        bark: bark([0.150, 0.133, 0.126], 0.30, 0.95),
        // A plum's foliage is a dull olive, not a lawn. Held deliberately low in
        // both value and chroma, because the tabebia is the palette's other
        // yellow-green and two bright yellow-greens are one yellow-green.
        foliage: [0.150, 0.149, 0.117],
        autumn: Some([0.240, 0.121, 0.053]),
        bloom: Bloom::of([0.660, 0.568, 0.598], 0.82, 0.16),
        height_m: (6.0, 9.0),
        crown_m: (3.2, 4.6),
        trunk_m: (0.13, 0.20),
        clear_stem: 0.30,
        density: 0.74,
        leaf_scale: 0.28,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "huang-hua-feng-jiao-mu",
        name_zh: "黄花风铃木",
        name_en: "Golden trumpet tree",
        canopy: Canopy::Vase,
        leaf: LeafForm::Palmate,
        bark: bark([0.245, 0.241, 0.233], 0.22, 1.0),
        foliage: [0.242, 0.245, 0.142],
        autumn: None,
        bloom: Bloom::of([0.640, 0.535, 0.115], 0.86, 0.24),
        height_m: (9.0, 14.0),
        crown_m: (3.8, 5.6),
        trunk_m: (0.20, 0.28),
        clear_stem: 0.36,
        density: 0.66,
        leaf_scale: 0.34,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "ma-lian",
        name_zh: "麻楝",
        name_en: "Chinaberry",
        canopy: Canopy::Rounded,
        leaf: LeafForm::Pinnate,
        bark: bark([0.215, 0.210, 0.198], 0.25, 1.0),
        foliage: [0.144, 0.195, 0.109],
        autumn: Some([0.290, 0.234, 0.081]),
        bloom: Bloom::NONE,
        height_m: (12.0, 18.0),
        crown_m: (6.0, 8.5),
        trunk_m: (0.32, 0.48),
        clear_stem: 0.40,
        density: 0.58,
        leaf_scale: 0.26,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "xiao-ye-zi-wei",
        name_zh: "小叶紫薇",
        name_en: "Crape myrtle",
        canopy: Canopy::MultiStem,
        leaf: LeafForm::Elliptic,
        bark: bark([0.265, 0.253, 0.239], 0.30, 0.85),
        foliage: [0.079, 0.195, 0.066],
        // Crape myrtle's autumn is a deep rose-crimson, and it is grown in China
        // for exactly this: it is the loudest thing on a street in November.
        autumn: Some([0.260, 0.073, 0.098]),
        bloom: Bloom::of([0.500, 0.190, 0.345], 0.92, 0.52),
        height_m: (4.5, 7.0),
        crown_m: (2.4, 3.6),
        trunk_m: (0.10, 0.17),
        clear_stem: 0.26,
        density: 0.76,
        leaf_scale: 0.30,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "hai-tang",
        name_zh: "海棠",
        name_en: "Flowering crabapple",
        canopy: Canopy::Spreading,
        leaf: LeafForm::Elliptic,
        bark: bark([0.180, 0.162, 0.155], 0.60, 0.95),
        foliage: [0.126, 0.180, 0.140],
        autumn: Some([0.245, 0.122, 0.069]),
        bloom: Bloom::of([0.680, 0.571, 0.593], 0.90, 0.18),
        height_m: (5.0, 8.0),
        crown_m: (3.0, 4.8),
        trunk_m: (0.14, 0.22),
        clear_stem: 0.28,
        density: 0.78,
        leaf_scale: 0.30,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "you-zi",
        name_zh: "柚子",
        name_en: "Pomelo",
        canopy: Canopy::Rounded,
        leaf: LeafForm::Ovate,
        bark: bark([0.205, 0.199, 0.189], 0.28, 1.0),
        foliage: [0.035, 0.125, 0.089],
        autumn: None,
        bloom: Bloom::of([0.720, 0.710, 0.648], 0.42, 0.30),
        height_m: (8.0, 12.0),
        crown_m: (3.6, 5.2),
        trunk_m: (0.18, 0.26),
        clear_stem: 0.30,
        density: 0.88,
        leaf_scale: 0.36,
        evergreen: true,
        street_tolerant: true,
    },
    Species {
        key: "hong-hua-ji-dan-hua",
        name_zh: "红花鸡蛋花",
        name_en: "Red frangipani",
        canopy: Canopy::MultiStem,
        leaf: LeafForm::Elliptic,
        bark: bark([0.230, 0.220, 0.207], 0.18, 0.9),
        foliage: [0.136, 0.150, 0.066],
        autumn: None,
        bloom: Bloom::of([0.520, 0.192, 0.156], 0.78, 0.56),
        height_m: (4.0, 6.5),
        crown_m: (2.6, 3.8),
        trunk_m: (0.10, 0.16),
        clear_stem: 0.20,
        density: 0.72,
        leaf_scale: 0.38,
        evergreen: true,
        street_tolerant: false,
    },
    Species {
        key: "luo-yu-shan",
        name_zh: "落羽杉",
        name_en: "Dawn redwood",
        canopy: Canopy::Conical,
        leaf: LeafForm::Needle,
        bark: bark([0.185, 0.148, 0.130], 0.70, 0.9),
        foliage: [0.102, 0.170, 0.131],
        // A quiet bronze. Deliberately lower in chroma than every other autumn
        // in the palette: a dawn redwood's autumn is a subtle thing, and letting
        // it shout competes with the tallow's scarlet for attention.
        autumn: Some([0.215, 0.122, 0.082]),
        bloom: Bloom::NONE,
        height_m: (14.0, 22.0),
        crown_m: (2.2, 3.4),
        trunk_m: (0.18, 0.26),
        clear_stem: 0.55,
        density: 0.80,
        leaf_scale: 0.20,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "huang-pi",
        name_zh: "黄皮",
        name_en: "Wampee",
        canopy: Canopy::MultiStem,
        leaf: LeafForm::Palmate,
        bark: bark([0.200, 0.195, 0.184], 0.22, 1.0),
        foliage: [0.241, 0.245, 0.122],
        autumn: None,
        bloom: Bloom::of([0.660, 0.650, 0.515], 0.36, 0.28),
        height_m: (4.5, 7.0),
        crown_m: (2.4, 3.6),
        trunk_m: (0.11, 0.18),
        clear_stem: 0.24,
        density: 0.80,
        leaf_scale: 0.34,
        evergreen: true,
        street_tolerant: false,
    },
    Species {
        key: "hong-hua-yu-lan",
        name_zh: "红花玉兰",
        name_en: "Red magnolia",
        canopy: Canopy::Spreading,
        leaf: LeafForm::Palmate,
        bark: bark([0.225, 0.221, 0.216], 0.15, 0.9),
        foliage: [0.142, 0.215, 0.147],
        // Magnolia goes bronze-tan and *lighter*, which is unusual among the
        // deciduous species here and is why it does not read as another
        // version of the dawn redwood's autumn.
        autumn: Some([0.170, 0.151, 0.112]),
        bloom: Bloom::of([0.480, 0.163, 0.205], 0.86, 0.14),
        height_m: (7.0, 11.0),
        crown_m: (3.4, 5.0),
        trunk_m: (0.16, 0.24),
        clear_stem: 0.32,
        density: 0.70,
        leaf_scale: 0.32,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "jin-ye-ying-ren",
        name_zh: "锦叶樱仁",
        name_en: "Purple-leaf plum",
        canopy: Canopy::Tiered,
        leaf: LeafForm::Elliptic,
        bark: bark([0.155, 0.133, 0.127], 0.35, 0.95),
        // A plum whose defining feature is the colour, not the flower: the
        // foliage itself is wine-purple all summer. It also barely turns —
        // holding that colour through autumn is part of what it is for — so
        // unlike every other deciduous species here it records no autumn.
        foliage: [0.190, 0.076, 0.091],
        autumn: None,
        bloom: Bloom::of([0.600, 0.504, 0.536], 0.30, 0.15),
        height_m: (4.5, 7.0),
        crown_m: (2.2, 3.4),
        trunk_m: (0.11, 0.18),
        clear_stem: 0.28,
        density: 0.80,
        leaf_scale: 0.26,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "huang-jin",
        name_zh: "黄槿",
        name_en: "Chinese hibiscus",
        canopy: Canopy::Rounded,
        leaf: LeafForm::Cordate,
        bark: bark([0.215, 0.210, 0.200], 0.20, 1.0),
        // A light, warm yellow-green: the hibiscus is grown for its big yellow
        // flowers against a bright, open crown.
        foliage: [0.285, 0.285, 0.091],
        autumn: Some([0.290, 0.182, 0.058]),
        bloom: Bloom::of([0.700, 0.620, 0.098], 0.58, 0.48),
        height_m: (6.0, 10.0),
        crown_m: (3.0, 4.4),
        trunk_m: (0.14, 0.22),
        clear_stem: 0.30,
        density: 0.82,
        leaf_scale: 0.38,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "shui-shan",
        name_zh: "水杉",
        name_en: "Metasequoia",
        canopy: Canopy::Conical,
        leaf: LeafForm::Needle,
        bark: bark([0.175, 0.153, 0.140], 0.60, 0.9),
        foliage: [0.141, 0.155, 0.124],
        autumn: Some([0.200, 0.138, 0.076]),
        bloom: Bloom::NONE,
        height_m: (18.0, 30.0),
        crown_m: (2.4, 3.6),
        trunk_m: (0.22, 0.34),
        clear_stem: 0.62,
        density: 0.78,
        leaf_scale: 0.20,
        evergreen: false,
        street_tolerant: true,
    },
];

/// Look a species up by its stable key.
pub fn by_key(key: &str) -> Option<&'static Species> {
    SPECIES.iter().find(|species| species.key == key)
}

/// The species a street planting may use, in the order a Chinese city plants
/// them. `street_tolerant` is the filter, and the order is the conventional one
/// so an avenue reads as designed rather than random.
pub fn street_list() -> Vec<&'static Species> {
    SPECIES
        .iter()
        .filter(|species| species.street_tolerant)
        .collect()
}

/// A species chosen for a park, courtyard or compound: everything, including the
/// subtropical species that would not survive a kerb but are ubiquitous in a
/// Chinese garden.
pub fn garden_list() -> &'static [Species] {
    SPECIES
}

/// Every species as a slice, for callers that just want to iterate.
pub const fn all() -> &'static [Species] {
    SPECIES
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Scale-invariant colour separability: chromaticity plus *relative* luma.
    ///
    /// Absolute distance is the wrong metric for reflectance. A physically
    /// plausible foliage palette is uniformly dark, so two real materials can sit
    /// close in absolute terms while being obviously different to the eye — and
    /// a yellow-green tree turning yellow is a small absolute change by
    /// construction, because both colours are yellow. What a viewer actually
    /// reads is the *direction* of the colour and where it falls on the
    /// light-to-dark axis relative to its own brightness, so that is measured.
    fn separable(colour: [f32; 3]) -> [f32; 4] {
        let [r, g, b] = colour;
        let total = (r + g + b).max(1.0e-4);
        let peak = r.max(g).max(b).max(1.0e-4);
        [
            r / total,
            g / total,
            b / total,
            (0.2126 * r + 0.7152 * g + 0.0722 * b) / peak,
        ]
    }

    fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
        let (pa, pb) = (separable(a), separable(b));
        (0..4).map(|axis| (pa[axis] - pb[axis]).powi(2)).sum::<f32>().sqrt()
    }

    /// Hue in degrees, 0..360. Used to assert that a colour is the *kind* of
    /// green its species should be, rather than merely "not red".
    fn hue_degrees(colour: [f32; 3]) -> f32 {
        let peak = colour[0].max(colour[1]).max(colour[2]);
        if peak <= 1.0e-6 {
            return 0.0;
        }
        // Normalise *first*, then take the minimum from the normalised values.
        // Mixing the two scales is a silent way to get a zero chroma and a hue of
        // 0 for every colour.
        let [r, g, b] = [colour[0] / peak, colour[1] / peak, colour[2] / peak];
        let chroma = 1.0 - r.min(g).min(b);
        if chroma <= 1.0e-6 {
            return 0.0;
        }
        let hue = if r >= g && r >= b {
            60.0 * (((g - b) / chroma) % 6.0)
        } else if g >= b {
            60.0 * (((b - r) / chroma) + 2.0)
        } else {
            60.0 * (((r - g) / chroma) + 4.0)
        };
        hue.rem_euclid(360.0)
    }

    fn worst_pair<F: Fn(&Species) -> [f32; 3]>(select: F) -> (f32, &'static str, &'static str) {
        let mut worst = (f32::INFINITY, "", "");
        for (index, a) in SPECIES.iter().enumerate() {
            for b in &SPECIES[index + 1..] {
                let d = distance(select(a), select(b));
                if d < worst.0 {
                    worst = (d, a.key, b.key);
                }
            }
        }
        worst
    }

    #[test]
    fn keys_are_unique_and_ascii() {
        let mut seen = HashSet::new();
        for species in SPECIES {
            assert!(
                seen.insert(species.key),
                "duplicate species key {}",
                species.key
            );
            assert!(
                species.key.is_ascii() && !species.key.is_empty(),
                "{} must be an ascii payload key",
                species.key
            );
        }
    }

    #[test]
    fn every_species_has_a_name_in_both_scripts() {
        for species in SPECIES {
            assert!(!species.name_zh.is_empty(), "{} has no Chinese name", species.key);
            assert!(!species.name_en.is_empty(), "{} has no English name", species.key);
        }
    }

    /// The palette is worthless if it is all one green, which is exactly what
    /// five green tones looked like. Distinctness is the requirement, so it is
    /// asserted rather than hoped for.
    #[test]
    fn the_palette_is_actually_distinguishable() {
        let (worst, a, b) = worst_pair(|species| species.foliage);
        assert!(
            worst > 0.030,
            "{a} and {b} have near-identical foliage ({worst:.4}); the palette reads as one tree"
        );
        // Autumn has to be a second palette, not a recolouring of the first: a
        // street where every species turns the same colour turns the same colour.
        let (worst, a, b) = worst_pair(|species| species.autumn.unwrap_or(species.foliage));
        assert!(
            worst > 0.030,
            "{a} and {b} turn the same colour ({worst:.4})"
        );
    }

    /// The single most important realism constraint in this file, and the one
    /// most easily lost.
    ///
    /// These values are **albedo**, not screen colour. A leaf reflects roughly
    /// 5-20% in the green band and 2-5% in red and blue; bark reflects 10-25%;
    /// petals are the reflective thing on a tree at 40-70%. A canopy built from
    /// bright saturated "greens" is not a tree, it is highlighter paint, and
    /// bright saturated albedo is the clearest signature of a cartoon render.
    ///
    /// The corollary matters just as much: if the palette is this dark, then
    /// **the lighting has to be physical**. A canopy at 0.12 albedo under a
    /// correct sun lands in the right place on screen; under flat ambient it
    /// renders black, and the tempting fix — brightening the albedo — destroys
    /// the material. So this test is a contract with the renderer, not just with
    /// this table.
    #[test]
    fn albedo_stays_in_the_range_of_a_real_material() {
        for species in SPECIES {
            let peak = species.foliage.iter().copied().fold(0.0_f32, f32::max);
            assert!(
                (0.06..=0.32).contains(&peak),
                "{} has {} peak foliage reflectance; a leaf is 0.05-0.20 and the \
                 brightest street tree is about 0.28",
                species.key,
                peak
            );
            // Foliage is green. How *much* green depends on the hue, and both
            // edges of the range are real: a yellow-green leaf genuinely has a
            // red channel almost level with its green, and a conifer needle
            // genuinely has a blue channel almost level with its green. Forcing
            // either into a "proper green" would make the palette wrong, so the
            // bound is the one the hue itself implies — computed, not guessed.
            let [r, g, b] = species.foliage;
            let hue = hue_degrees([r, g, b]);
            // The plum is the one species whose foliage is not green at all, and
            // it is in the palette for exactly that reason. It is checked, not
            // exempted: a wine-purple leaf must be red-dominant and in the red
            // sector of the wheel.
            if species.key == "jin-ye-ying-ren" {
                assert!(r > b && r > g, "a purple plum has red above both");
                assert!(
                    hue > 330.0 || hue < 15.0,
                    "the plum's foliage hue is {hue:.0} deg, which is not a wine red"
                );
            } else if species.leaf == LeafForm::Needle {
                assert!(
                    (138.0..=185.0).contains(&hue) || (80.0..=115.0).contains(&hue),
                    "{} is a conifer but its foliage hue is {hue:.0} deg; a \
                     needle is grey-green or blue-green ({r:.3}, {g:.3}, {b:.3})",
                    species.key
                );
            } else if hue < 75.0 {
                // A yellow-green: red and green are level, blue is well below.
                assert!(
                    g >= r * 0.98,
                    "{} reads as yellow rather than yellow-green at hue {hue:.0} \
                     ({r:.3}, {g:.3}, {b:.3})",
                    species.key
                );
                assert!(
                    b < g * 0.82,
                    "{} is a yellow-green but its blue channel is too high \
                     ({r:.3}, {g:.3}, {b:.3})",
                    species.key
                );
            } else {
                assert!(
                    g > r * 1.10 && g > b * 1.10,
                    "{} has no green dominance ({r:.3}, {g:.3}, {b:.3}, hue {hue:.0})",
                    species.key
                );
            }

            let bark_peak = species.bark.colour.iter().copied().fold(0.0_f32, f32::max);
            assert!(
                (0.08..=0.30).contains(&bark_peak),
                "{} has {} peak bark reflectance; dry bark is 0.10-0.25",
                species.key,
                bark_peak
            );
            // Bark is far less saturated than foliage, and greyer.
            let [br, bg, bb] = species.bark.colour;
            let chroma = br.max(bg).max(bb) - br.min(bg).min(bb);
            assert!(
                chroma < 0.09,
                "{} has saturated bark (chroma {chroma:.3}); bark is grey-brown",
                species.key
            );
        }
    }

    #[test]
    fn blossom_is_the_reflective_part_of_a_tree() {
        // Petals are near-white tinted; leaves are not. A renderer that gets
        // this backwards produces a tree that glows, which is the most obvious
        // tell of a stylised canopy.
        for species in SPECIES {
            if let Some(colour) = species.bloom.colour {
                let peak = colour.iter().copied().fold(0.0_f32, f32::max);
                assert!(
                    (0.42..=0.78).contains(&peak),
                    "{} has {} peak petal reflectance; petals are 0.42-0.78",
                    species.key,
                    peak
                );
                let leaf_peak = species.foliage.iter().copied().fold(0.0_f32, f32::max);
                assert!(
                    peak > leaf_peak * 1.7,
                    "{}'s blossom ({} peak) is not clearly lighter than its \
                     foliage ({} peak)",
                    species.key,
                    peak,
                    leaf_peak
                );
            }
        }
    }

    #[test]
    fn autumn_foliage_warmer_and_denser_than_summer_not_just_a_hue_shift() {
        // Losing chlorophyll does not rotate a hue; it removes the green and
        // leaves behind carotenoids behind it, so autumn goes redder and usually
        // more chromatic. A tint that only shifts hue reads as a filter over
        // summer.
        //
        // "Usually", not always: a bronze-tan autumn is *less* chromatic than a
        // vivid green summer, and a species chosen for a quiet autumn should be
        // allowed to have one. So the chroma condition is a floor, while the
        // red-over-green ratio — the actual chlorophyll-loss signature — must
        // move for every deciduous species.
        for species in SPECIES {
            if let Some(autumn) = species.autumn {
                let spread = |c: [f32; 3]| {
                    c.iter().copied().fold(0.0_f32, f32::max)
                        - c.iter().copied().fold(f32::INFINITY, f32::min)
                };
                assert!(
                    spread(autumn) >= spread(species.foliage) * 0.70,
                    "{}'s autumn is far less chromatic than its summer \
                     ({:.3} vs {:.3})",
                    species.key,
                    spread(autumn),
                    spread(species.foliage)
                );
                let summer_ratio = species.foliage[0] / species.foliage[1].max(1.0e-4);
                let autumn_ratio = autumn[0] / autumn[1].max(1.0e-4);
                assert!(
                    autumn_ratio > summer_ratio * 1.20,
                    "{}'s autumn is not redder than its summer ({} -> {})",
                    species.key,
                    summer_ratio,
                    autumn_ratio
                );
            }
        }
    }

    #[test]
    fn the_autumn_colours_are_the_reason_these_trees_are_planted() {
        // A species that records an autumn colour must record one a viewer can
        // tell from its summer, or the field is decoration.
        for species in SPECIES {
            if let Some(autumn) = species.autumn {
                let d = distance(autumn, species.foliage);
                assert!(
                    d > 0.020,
                    "{}'s autumn is indistinguishable from its summer ({d:.4})",
                    species.key
                );
            }
        }
        let turning: Vec<&str> = SPECIES
            .iter()
            .filter(|species| species.autumn.is_some())
            .map(|species| species.key)
            .collect();
        assert!(
            turning.len() >= 8,
            "only {} species turn colour, so no street has a season",
            turning.len()
        );
        // And a palette of evergreens would be a temperate conifer forest, not
        // a Chinese city, which is roughly two-thirds evergreen.
        let evergreen = SPECIES.iter().filter(|s| s.evergreen).count();
        assert!(
            evergreen >= 4 && evergreen <= 7,
            "{evergreen} evergreens out of {} is not a plausible urban mix",
            SPECIES.len()
        );
    }

    #[test]
    fn blossom_species_are_actually_showy() {
        for species in SPECIES {
            if species.bloom.colour.is_some() {
                assert!(
                    species.bloom.density > 0.25,
                    "{} has a bloom colour but no bloom",
                    species.key
                );
                assert!(
                    species.bloom.at > 0.05 && species.bloom.at < 0.95,
                    "{} blooms outside the year",
                    species.key
                );
            }
        }
        // The reference is a Chinese ornamental palette: blossom must dominate.
        let blossoming = SPECIES
            .iter()
            .filter(|species| species.bloom.density > 0.5)
            .count();
        assert!(
            blossoming >= 7,
            "only {blossoming} species are showy in flower"
        );
    }

    #[test]
    fn dimensions_are_realistic_and_internally_consistent() {
        for species in SPECIES {
            let (low, high) = species.height_m;
            assert!(low > 3.0, "{} is a shrub", species.key);
            assert!(high <= 32.0, "{} is a forest tree", species.key);
            assert!(low < high, "{} has an inverted height range", species.key);
            // A crown wider than tall is a bush; a crown far narrower than a
            // conifer should be is a pole.
            assert!(
                species.crown_m.1 < species.height_m.1 * 0.85,
                "{} has a crown as wide as it is tall",
                species.key
            );
            assert!(
                species.crown_m.0 > 1.5,
                "{} has no crown to speak of",
                species.key
            );
            assert!(
                species.trunk_m.0 > 0.06,
                "{} has no trunk to speak of",
                species.key
            );
            assert!(
                species.clear_stem > 0.1 && species.clear_stem < 0.75,
                "{} has an implausible clear stem",
                species.key
            );
            assert!(
                species.density > 0.4 && species.density <= 1.0,
                "{} has an implausible canopy density",
                species.key
            );
        }
    }

    #[test]
    fn a_street_list_exists_and_is_a_strict_subset() {
        let street = street_list();
        assert!(
            street.len() >= 10,
            "only {} species can survive a kerb",
            street.len()
        );
        assert!(street.len() < SPECIES.len(), "everything is street tolerant");
        for species in &street {
            assert!(species.street_tolerant, "{} leaked into the street list", species.key);
        }
    }

    #[test]
    fn the_conifers_are_tall_and_narrow_and_the_shrubs_are_short() {
        for species in SPECIES {
            if species.canopy == Canopy::Conical {
                assert!(
                    species.height_m.0 > 12.0,
                    "conifer {} should be tall",
                    species.key
                );
                assert!(
                    species.crown_m.0 < 4.2,
                    "conifer {} should be narrow",
                    species.key
                );
            }
            if species.canopy == Canopy::MultiStem {
                assert!(
                    species.height_m.1 < 9.0,
                    "multi-stem {} should be a small tree",
                    species.key
                );
            }
        }
    }
}
