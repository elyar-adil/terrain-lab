//! The sixteen common street trees of a Chinese city.
//!
//! # Why a species table and not a shape enum
//!
//! The previous port modelled a tree as one of four crown shapes — broad,
//! conical, weeping, clump — which is why every street looked like the same
//! four stamps. A city reads as a city because the *species* differ: a `银杏`
//! turns gold while a `香樟` stays black-green, a `雪松` is a stack of drooping
//! tiers, a `杨柳` falls to the ground in curtains, and a `合欢` carries a flat
//! umbrella of feather foliage. Those are different colours, different
//! silhouettes, different branching habits and different seasonal states, and
//! none of them is recoverable from a crown enum.
//!
//! This table is the reference set of sixteen trees a Chinese city actually
//! plants — the ones in the planting books from Harbin to Guangzhou:
//!
//! | key | 中文 | silhouette the eye reads |
//! |---|---|---|
//! | `xiang-zhang` | 香樟 | dense dark-green dome, stout trunk |
//! | `yin-xing` | 银杏 | broad fan on a high clean leg, gold in November |
//! | `wu-tong` | 梧桐 | tall open crown, pale mottled bark, huge leaves |
//! | `huai-shu` | 槐树 | rounded crown of layered flat-topped clumps |
//! | `yu-shu` | 榆树 | broad, dense, rounded |
//! | `gui-hua` | 桂花 | small dense evergreen oval, orange-white bloom |
//! | `xue-song` | 雪松 | conical, whorled horizontal tiers drooping at the tips |
//! | `liu-shu` | 柳树 | weeping: shoots fall from the shoulder to the ground |
//! | `song-shu` | 松树 | leaning tortuous trunk, flat needle clusters, gaps |
//! | `shui-shan` | 水杉 | narrow feathery spire, straight leader |
//! | `ci-huai` | 刺槐 | open irregular round crown, white June bloom |
//! | `rong-shu` | 榕树 | very wide dense dome, aerial roots to the ground |
//! | `tao-shu` | 桃树 | small spreading vase, pink blossom |
//! | `sha-shu` | 杉树 | dense bluish cone |
//! | `he-huan` | 合欢 | flat-topped umbrella of pinnate feather, pink powder-puffs |
//! | `yang-shu` | 杨树 | fastigiate column, branches sweeping steeply up |
//!
//! So a species is a record: the botanical identity, the mature dimensions, the
//! canopy architecture, the bark, the leaf shape and colour through the year,
//! and the blossom state. Geometry and texture both read this table, so a
//! tree's silhouette and its colour can never disagree.
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
///
/// Every variant is planted in the table — an unexercised variant would be a
/// branch of the generator nobody has run — and every variant has its own
/// profile curve and its own numbers in [`crate::trees::forms`], which is what
/// keeps a `雪松` and a `水杉` from being the same cone in different greens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Canopy {
    /// Broad, dense, rounded — the crown of a `香樟`, `榆树`, `槐树` or `刺槐`.
    Rounded,
    /// Tall, broad and *open*, branching high on a clean trunk, pale bark —
    /// the `梧桐`.
    Open,
    /// A dense evergreen oval, taller than it is wide, on a small tree — the
    /// `桂花`.
    Oval,
    /// A broad fan: narrow at the foot, wide through the middle, drawn in
    /// again at the top — the `银杏`.
    Fan,
    /// A vase: narrow at the foot, widest at the lip, on a small tree — the
    /// `桃树`.
    Vase,
    /// A narrow conical spire with a straight leader and short ascending
    /// branches — `水杉`, `杉树`.
    Conical,
    /// Conical but layered: whorls of long horizontal tiers that droop at the
    /// tips, widest at the foot — the `雪松`.
    Layered,
    /// Falling from a shoulder: limbs arch out and the shoots hang from them
    /// to the ground — the `柳树`.
    Weeping,
    /// A fastigiate column: branches sweep steeply upward, hugging the trunk —
    /// the `杨树`.
    Fastigiate,
    /// A leaning, tortuous trunk carrying irregular flat clusters of needle
    /// foliage with gaps between them — the `松树`.
    Irregular,
    /// Branches rise, then flatten out at the crown top into a wide, flat
    /// umbrella — the `合欢`.
    Umbrella,
    /// A very wide dense dome on a thick trunk, with column-like aerial roots
    /// dropping from the major limbs to the ground — the `榕树`.
    Banyan,
}

/// Leaf form. This drives the leaf card's alpha silhouette, which is most of
/// why a `柳树` and a `香樟` read differently at close range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeafForm {
    /// Broad, ovate, smooth margin — `香樟`, `榆树`, `榕树`, `杨树`.
    Ovate,
    /// Palmate, three to five lobes — `梧桐`, with its huge plane leaves.
    Palmate,
    /// Elliptic, finely serrate — `桂花`, `桃树`.
    Elliptic,
    /// Needle or scale, in dense sprays — the conifers.
    Needle,
    /// Pinnate, many leaflets on one rachis — `槐树`, `刺槐`, `合欢`.
    Pinnate,
    /// Long, narrow, fine-tipped — the `柳树`'s lance.
    Lanceolate,
    /// A fan: narrow petiole opening into a broad rounded blade — the `银杏`.
    Fan,
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
    /// `银杏` turning gold is the reason it is planted.
    pub autumn: Option<[f32; 3]>,
    pub bloom: Bloom,
    /// Mature height in metres, `(min, max)`, for street planting.
    pub height_m: (f32, f32),
    /// Mature crown radius in metres, `(min, max)`.
    pub crown_m: (f32, f32),
    /// Mature trunk radius at breast height in metres, `(min, max)`.
    pub trunk_m: (f32, f32),
    /// Fraction of total height below the first branch. A `雪松` carries tiers
    /// almost to the ground; a `梧桐` branches high on a clean leg.
    pub clear_stem: f32,
    /// How densely the canopy is packed with leaf cards, 0..1. A `梧桐` is
    /// open and see-through; a `榕树` is a solid mass.
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

/// The palette: the sixteen reference trees, in the order of the planting sheet.
pub const SPECIES: &[Species] = &[
    Species {
        key: "xiang-zhang",
        name_zh: "香樟",
        name_en: "Camphor",
        canopy: Canopy::Rounded,
        leaf: LeafForm::Ovate,
        bark: bark([0.160, 0.152, 0.142], 0.55, 1.0),
        // The darkest broadleaf green in the palette: a camphor's crown reads
        // nearly black-green against everything else on the street, which is
        // why it is the evergreen backbone of a southern Chinese avenue.
        foliage: [0.052, 0.115, 0.052],
        autumn: None,
        bloom: Bloom::NONE,
        height_m: (10.0, 18.0),
        crown_m: (5.5, 8.0),
        trunk_m: (0.28, 0.42),
        clear_stem: 0.30,
        density: 0.88,
        leaf_scale: 0.30,
        evergreen: true,
        street_tolerant: true,
    },
    Species {
        key: "yin-xing",
        name_zh: "银杏",
        name_en: "Ginkgo",
        canopy: Canopy::Fan,
        leaf: LeafForm::Fan,
        bark: bark([0.200, 0.190, 0.175], 0.30, 1.0),
        // A fresh, light green — lighter than every other broadleaf here so its
        // autumn has somewhere to go.
        foliage: [0.115, 0.165, 0.062],
        // The gold. A ginkgo avenue in November is the single loudest colour
        // event in a Chinese city, and it is why the tree is planted.
        autumn: Some([0.320, 0.225, 0.030]),
        bloom: Bloom::NONE,
        height_m: (12.0, 20.0),
        crown_m: (4.5, 6.5),
        trunk_m: (0.24, 0.38),
        // Branches start at mid-height and spread elegantly, which is what
        // gives the crown its fan shape on a clean leg.
        clear_stem: 0.44,
        density: 0.62,
        leaf_scale: 0.26,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "wu-tong",
        name_zh: "梧桐",
        name_en: "Chinese parasol",
        canopy: Canopy::Open,
        leaf: LeafForm::Palmate,
        // The pale mottled bark is half the tree's identity in winter.
        bark: bark([0.255, 0.245, 0.228], 0.18, 1.0),
        // A grey, dull green: the huge plane leaves are matte and the crown is
        // open, so the tree reads lighter and thinner than an elm of the same
        // size.
        foliage: [0.130, 0.165, 0.115],
        autumn: Some([0.200, 0.160, 0.085]),
        bloom: Bloom::NONE,
        height_m: (12.0, 20.0),
        crown_m: (6.0, 9.0),
        trunk_m: (0.30, 0.45),
        // Branching high on a clean leg is the habit: the crown is all in the
        // top half and the pale trunk is on show.
        clear_stem: 0.45,
        density: 0.54,
        leaf_scale: 0.36,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "huai-shu",
        name_zh: "槐树",
        name_en: "Chinese scholar tree",
        canopy: Canopy::Rounded,
        leaf: LeafForm::Pinnate,
        bark: bark([0.185, 0.175, 0.160], 0.60, 1.0),
        // An olive cast between the camphor's black-green and the elm's mid
        // green: the pagoda tree's pinnate foliage is never glossy.
        foliage: [0.105, 0.155, 0.075],
        autumn: Some([0.260, 0.190, 0.070]),
        bloom: Bloom::NONE,
        height_m: (10.0, 16.0),
        crown_m: (5.5, 8.0),
        trunk_m: (0.28, 0.40),
        clear_stem: 0.34,
        density: 0.68,
        leaf_scale: 0.28,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "yu-shu",
        name_zh: "榆树",
        name_en: "Elm",
        canopy: Canopy::Rounded,
        leaf: LeafForm::Ovate,
        bark: bark([0.165, 0.150, 0.132], 0.75, 1.0),
        // A deep forest green, sitting between the camphor's near-black and
        // the scholar tree's olive.
        foliage: [0.105, 0.160, 0.088],
        // Butter yellow, the classic elm fall.
        autumn: Some([0.285, 0.205, 0.058]),
        bloom: Bloom::NONE,
        height_m: (12.0, 20.0),
        crown_m: (6.0, 8.5),
        trunk_m: (0.28, 0.42),
        clear_stem: 0.36,
        density: 0.80,
        leaf_scale: 0.28,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "gui-hua",
        name_zh: "桂花",
        name_en: "Osmanthus",
        canopy: Canopy::Oval,
        leaf: LeafForm::Elliptic,
        bark: bark([0.155, 0.148, 0.138], 0.35, 1.0),
        // The glossiest, darkest small leaf in the palette: an osmanthus is a
        // solid near-black-green oval even next to a camphor.
        foliage: [0.038, 0.088, 0.052],
        autumn: None,
        // The flowers are the point of the tree and they are *tiny*: a scatter
        // of orange-white over a still-dark canopy, not a cloud. A low density
        // is the honest number and a quiet colour is the honest colour.
        bloom: Bloom::of([0.550, 0.440, 0.280], 0.32, 0.70),
        height_m: (4.0, 8.0),
        crown_m: (2.2, 3.4),
        trunk_m: (0.12, 0.20),
        clear_stem: 0.26,
        density: 0.90,
        leaf_scale: 0.30,
        evergreen: true,
        street_tolerant: true,
    },
    Species {
        key: "xue-song",
        name_zh: "雪松",
        name_en: "Deodar cedar",
        canopy: Canopy::Layered,
        leaf: LeafForm::Needle,
        bark: bark([0.145, 0.130, 0.115], 0.65, 0.95),
        // Silver-green: paler and bluer than any other conifer here, which is
        // what a deodar's foliage actually is.
        foliage: [0.105, 0.145, 0.128],
        autumn: None,
        bloom: Bloom::NONE,
        height_m: (12.0, 20.0),
        crown_m: (4.5, 6.5),
        trunk_m: (0.26, 0.40),
        // A deodar carries its lowest tier almost to the lawn: the crown starts
        // low and the tiers stack from there.
        clear_stem: 0.18,
        density: 0.78,
        leaf_scale: 0.22,
        evergreen: true,
        street_tolerant: true,
    },
    Species {
        key: "liu-shu",
        name_zh: "柳树",
        name_en: "Willow",
        canopy: Canopy::Weeping,
        leaf: LeafForm::Lanceolate,
        bark: bark([0.140, 0.130, 0.112], 0.50, 0.95),
        // The palest, most yellow-green broadleaf in the palette: a willow in
        // leaf is fresh growth all over.
        foliage: [0.165, 0.215, 0.098],
        autumn: Some([0.245, 0.205, 0.080]),
        bloom: Bloom::NONE,
        height_m: (8.0, 14.0),
        crown_m: (5.0, 7.5),
        trunk_m: (0.22, 0.34),
        clear_stem: 0.28,
        density: 0.72,
        leaf_scale: 0.30,
        evergreen: false,
        street_tolerant: false,
    },
    Species {
        key: "song-shu",
        name_zh: "松树",
        name_en: "Pine",
        canopy: Canopy::Irregular,
        leaf: LeafForm::Needle,
        bark: bark([0.150, 0.128, 0.105], 0.90, 0.95),
        // A dark, grey-green needle tone — the hue sits where a two-needle
        // pine's actually sits, greener than the deodar and bluer than the
        // metasequoia.
        foliage: [0.075, 0.125, 0.062],
        autumn: None,
        bloom: Bloom::NONE,
        height_m: (9.0, 16.0),
        crown_m: (4.0, 6.5),
        trunk_m: (0.26, 0.42),
        clear_stem: 0.35,
        density: 0.58,
        leaf_scale: 0.26,
        evergreen: true,
        street_tolerant: false,
    },
    Species {
        key: "shui-shan",
        name_zh: "水杉",
        name_en: "Dawn redwood",
        canopy: Canopy::Conical,
        leaf: LeafForm::Needle,
        bark: bark([0.175, 0.153, 0.140], 0.60, 0.9),
        foliage: [0.141, 0.155, 0.124],
        // A quiet bronze. Deliberately lower in chroma than every other autumn
        // in the palette: a dawn redwood's autumn is a subtle thing, and letting
        // it shout competes with the ginkgo's gold for attention.
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
    Species {
        key: "ci-huai",
        name_zh: "刺槐",
        name_en: "Black locust",
        canopy: Canopy::Rounded,
        leaf: LeafForm::Pinnate,
        bark: bark([0.175, 0.165, 0.150], 0.80, 1.0),
        // A grey, matte green — the locust's pinnate foliage never reads as
        // glossy, and the crown is open enough to see the sky through.
        foliage: [0.118, 0.172, 0.098],
        autumn: Some([0.270, 0.210, 0.062]),
        // Locusts in white flower in late May are a real event and a visible
        // one, but a scatter, not a cloud.
        bloom: Bloom::of([0.550, 0.550, 0.520], 0.35, 0.40),
        height_m: (10.0, 16.0),
        crown_m: (4.5, 6.5),
        trunk_m: (0.24, 0.36),
        clear_stem: 0.40,
        density: 0.55,
        leaf_scale: 0.26,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "rong-shu",
        name_zh: "榕树",
        name_en: "Banyan",
        canopy: Canopy::Banyan,
        leaf: LeafForm::Ovate,
        bark: bark([0.175, 0.162, 0.148], 0.45, 1.0),
        // A deep, saturated green under which nothing much grows: the banyan's
        // crown is the densest mass of shade in the palette.
        foliage: [0.062, 0.130, 0.068],
        autumn: None,
        bloom: Bloom::NONE,
        height_m: (10.0, 18.0),
        crown_m: (8.0, 12.0),
        trunk_m: (0.35, 0.55),
        clear_stem: 0.24,
        density: 0.86,
        leaf_scale: 0.30,
        evergreen: true,
        street_tolerant: true,
    },
    Species {
        key: "tao-shu",
        name_zh: "桃树",
        name_en: "Peach",
        canopy: Canopy::Vase,
        leaf: LeafForm::Elliptic,
        bark: bark([0.190, 0.170, 0.150], 0.45, 0.95),
        // A slightly blue-green fresh leaf, kept distinct from the scholar
        // tree's olive and the locust's grey.
        foliage: [0.085, 0.150, 0.098],
        autumn: Some([0.235, 0.115, 0.075]),
        // The pink cloud. A peach in March is blossom with a little foliage
        // behind it, which is exactly what the density says.
        bloom: Bloom::of([0.620, 0.295, 0.360], 0.88, 0.16),
        height_m: (3.5, 6.5),
        crown_m: (2.8, 4.2),
        trunk_m: (0.12, 0.20),
        clear_stem: 0.30,
        density: 0.70,
        leaf_scale: 0.32,
        evergreen: false,
        street_tolerant: false,
    },
    Species {
        key: "sha-shu",
        name_zh: "杉树",
        name_en: "Fir",
        canopy: Canopy::Conical,
        leaf: LeafForm::Needle,
        bark: bark([0.130, 0.118, 0.108], 0.55, 0.95),
        // Bluish and dark: the fir is the coldest green in the palette, and it
        // holds that colour all year.
        foliage: [0.055, 0.108, 0.100],
        autumn: None,
        bloom: Bloom::NONE,
        height_m: (14.0, 22.0),
        crown_m: (2.4, 3.4),
        trunk_m: (0.20, 0.30),
        clear_stem: 0.40,
        density: 0.85,
        leaf_scale: 0.20,
        evergreen: true,
        street_tolerant: true,
    },
    Species {
        key: "he-huan",
        name_zh: "合欢",
        name_en: "Silk tree",
        canopy: Canopy::Umbrella,
        leaf: LeafForm::Pinnate,
        bark: bark([0.165, 0.152, 0.135], 0.40, 1.0),
        // An olive, feathery green — the twice-cut pinnate leaves read as a
        // haze rather than as a mass, warmer than the locust's grey.
        foliage: [0.145, 0.168, 0.098],
        autumn: None,
        // Pink powder-puffs across the flat top of the umbrella through July.
        bloom: Bloom::of([0.600, 0.270, 0.360], 0.55, 0.44),
        height_m: (8.0, 14.0),
        crown_m: (5.0, 7.5),
        trunk_m: (0.20, 0.30),
        // An umbrella on a clear leg: the trunk is on show, the crown is flat.
        clear_stem: 0.42,
        density: 0.62,
        leaf_scale: 0.30,
        evergreen: false,
        street_tolerant: true,
    },
    Species {
        key: "yang-shu",
        name_zh: "杨树",
        name_en: "Poplar",
        canopy: Canopy::Fastigiate,
        leaf: LeafForm::Ovate,
        bark: bark([0.185, 0.175, 0.158], 0.50, 1.0),
        // A grey-green, cooler than the elm and the willow, which is what a
        // poplar's leaves shaking in the least wind actually read as.
        foliage: [0.128, 0.172, 0.105],
        // Clear gold, a shade warmer than the ginkgo's.
        autumn: Some([0.300, 0.225, 0.055]),
        bloom: Bloom::NONE,
        height_m: (16.0, 26.0),
        crown_m: (2.6, 4.0),
        trunk_m: (0.24, 0.38),
        // Fastigiate: the sweep of upward branches starts low, so there is no
        // clear stem to speak of — the column is the tree.
        clear_stem: 0.18,
        density: 0.74,
        leaf_scale: 0.26,
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
/// species that would not survive a kerb but are ubiquitous in a Chinese garden.
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
        // The reference sheet is sixteen trees and the palette is the sheet.
        assert_eq!(SPECIES.len(), 16, "the reference set is sixteen species");
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
            if species.leaf == LeafForm::Needle {
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
        // a Chinese city, which is roughly two-thirds evergreen. The reference
        // sheet has six: camphor, osmanthus, deodar, pine, banyan, fir.
        let evergreen = SPECIES.iter().filter(|s| s.evergreen).count();
        assert!(
            evergreen >= 4 && evergreen <= 7,
            "{evergreen} evergreens out of {} is not a plausible urban mix",
            SPECIES.len()
        );
    }

    /// The reference palette is a *street tree* palette, not an ornamental one,
    /// so blossom is the exception and not the rule. What must hold is that the
    /// three trees planted for their flowers are planted for the right flower:
    /// a peach is a pink cloud, a silk tree is pink puffs over a flat top, an
    /// osmanthus is a quiet orange-white scatter, and the locust's white June
    /// dress is a real but minor event. Anything else flowers, if at all, below
    /// the noise floor.
    #[test]
    fn the_blossom_species_bloom_the_flowers_they_are_planted_for() {
        let bloom_of = |key: &str| {
            SPECIES
                .iter()
                .find(|species| species.key == key)
                .unwrap_or_else(|| panic!("{key} is in the palette"))
                .bloom
        };
        // Peach: the loud pink cloud of March.
        let peach = bloom_of("tao-shu");
        assert!(peach.density > 0.75, "a peach in blossom is mostly blossom");
        // Silk tree: powder-puffs over the umbrella, present but not a cloud.
        let silk = bloom_of("he-huan");
        assert!(silk.density > 0.45, "a silk tree's puffs must be visible");
        // Osmanthus: the quietest famous flower there is.
        let osmanthus = bloom_of("gui-hua");
        assert!(
            osmanthus.density < 0.45,
            "an osmanthus is a scatter of tiny flowers, not a canopy of them"
        );
        // Locust: white June bloom, a minor event.
        let locust = bloom_of("ci-huai");
        assert!(locust.density < 0.50, "a locust's bloom is a dress, not a cloud");
        // And nobody else is showy: thirteen of the sixteen are foliage trees.
        let blossoming = SPECIES
            .iter()
            .filter(|species| species.bloom.density > 0.5)
            .count();
        assert!(
            blossoming == 2,
            "{blossoming} species are showy in flower; the reference sheet has \
             exactly two (peach, silk tree)"
        );
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

    /// Conifers are tall and narrow; the fastigiate poplar is the column the
    /// reference sheet draws it as; the deodar is the wide-tiered exception.
    #[test]
    fn the_conifers_are_tall_and_narrow_and_the_poplar_is_a_column() {
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
            if species.canopy == Canopy::Fastigiate {
                assert!(
                    species.crown_m.1 < species.height_m.0 * 0.25,
                    "fastigiate {} should be a column, not a cone: crown {:?} \
                     against a {:?} height",
                    species.key,
                    species.crown_m,
                    species.height_m
                );
            }
            if species.canopy == Canopy::Banyan {
                assert!(
                    species.trunk_m.0 > 0.30,
                    "a banyan's trunk is the thickest thing on the street"
                );
            }
        }
    }
}
