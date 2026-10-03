//! The species catalogue: data, not code.
//!
//! A species is a record of what is true of the plant: how big it gets, how it
//! holds its crown (its *habit*), what its leaves are like and what colour they are
//! through the year, what its bark is like. Nothing here says what any particular
//! tree looks like; that is decided per tree, from the tree's own seed, inside the
//! ranges the record allows. Adding a species is adding a row.
//!
//! Colours are linear RGB reflectance. Dimensions are metres.

/// How a species holds its crown. This is the silhouette the eye reads at a
/// distance, and it selects the growth template (see `params`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Habit {
    /// Broad, dense, rounded: camphor, elm, scholar tree, locust, persimmon.
    Rounded,
    /// Tall, broad, open, branching high on a clean trunk: plane tree.
    Open,
    /// A dense oval, taller than wide: osmanthus, magnolia.
    Oval,
    /// A fan: narrow at the foot, wide through the middle, drawn in at the top: ginkgo.
    Fan,
    /// A vase: narrow at the foot, widest at the lip: peach, cherry, apple.
    Vase,
    /// A narrow cone with a straight leader and short ascending branches: dawn redwood, fir.
    Conical,
    /// A cone built of long horizontal tiers that droop at the tips: deodar.
    Layered,
    /// Limbs arch out and the shoots hang from them to the ground: willow.
    Weeping,
    /// A column: branches sweep steeply upward: poplar.
    Fastigiate,
    /// A leaning, tortuous trunk carrying irregular flat clusters: pine.
    Irregular,
    /// Branches rise, then flatten into a wide flat umbrella: silk tree.
    Umbrella,
    /// A very wide dense dome on a thick trunk: banyan.
    Banyan,
}

/// The shape of one leaf (or leaf-like unit: a needle spray, a compound leaf).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeafForm {
    /// Broad, ovate, smooth margin.
    Ovate,
    /// Palmately lobed: plane, maple.
    Palmate,
    /// Elliptic, finely toothed.
    Elliptic,
    /// Needle sprays.
    Needle,
    /// Pinnate or bipinnate: many leaflets on one rachis.
    Pinnate,
    /// Long, narrow, fine-tipped.
    Lanceolate,
    /// A fan: narrow petiole opening into a broad blade.
    Fan,
}

impl LeafForm {
    /// The number the renderer switches on.
    pub const fn code(self) -> u8 {
        match self {
            LeafForm::Ovate => 0,
            LeafForm::Palmate => 1,
            LeafForm::Elliptic => 2,
            LeafForm::Needle => 3,
            LeafForm::Pinnate => 4,
            LeafForm::Lanceolate => 5,
            LeafForm::Fan => 6,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bark {
    pub colour: [f32; 3],
    /// 0 = smooth, 1 = deeply fissured.
    pub fissure: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Bloom {
    pub colour: [f32; 3],
    /// Share of the crown that is blossom at its peak.
    pub density: f32,
    /// Where in the year it peaks, 0..1.
    pub at: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Species {
    pub key: &'static str,
    pub name_zh: &'static str,
    pub name_en: &'static str,
    pub habit: Habit,
    pub leaf: LeafForm,
    pub bark: Bark,
    pub foliage: [f32; 3],
    pub autumn: Option<[f32; 3]>,
    pub bloom: Option<Bloom>,
    /// Mature height, `(small, large)`.
    pub height_m: (f32, f32),
    /// Mature crown radius of an open-grown tree at those heights.
    pub crown_m: (f32, f32),
    /// Trunk radius at breast height.
    pub trunk_m: (f32, f32),
    /// Fraction of the height below the first limb.
    pub clear_stem: f32,
    /// Leaf area index of a healthy crown: leaf area per metre² of ground under it.
    pub leaf_cover: f32,
    /// Typical blade length (or spray, or compound leaf), metres.
    pub leaf_len_m: f32,
    /// Blade width over length.
    pub leaf_aspect: f32,
    pub evergreen: bool,
    /// Day of the year (0..1) leaves are out by, and when they start to go.
    pub leaf_out: f32,
    pub leaf_fall: f32,
    pub street_tolerant: bool,
}

const fn bark(colour: [f32; 3], fissure: f32) -> Bark {
    Bark { colour, fissure }
}

macro_rules! species {
    ($($body:tt)*) => { Species { $($body)* } };
}

pub const SPECIES: &[Species] = &[
    species! {
        key: "xiang-zhang", name_zh: "香樟", name_en: "Camphor", habit: Habit::Rounded, leaf: LeafForm::Ovate,
        bark: bark([0.160, 0.152, 0.142], 0.55), foliage: [0.034, 0.133, 0.019], autumn: None, bloom: None,
        height_m: (10.0, 18.0), crown_m: (5.5, 8.0), trunk_m: (0.28, 0.42), clear_stem: 0.30, leaf_cover: 4.6,
        leaf_len_m: 0.085, leaf_aspect: 0.50, evergreen: true, leaf_out: 0.30, leaf_fall: 0.95, street_tolerant: true,
    },
    species! {
        key: "yin-xing", name_zh: "银杏", name_en: "Ginkgo", habit: Habit::Fan, leaf: LeafForm::Fan,
        bark: bark([0.200, 0.190, 0.175], 0.30), foliage: [0.162, 0.305, 0.021], autumn: Some([0.320, 0.225, 0.030]), bloom: None,
        height_m: (12.0, 20.0), crown_m: (4.5, 6.5), trunk_m: (0.24, 0.38), clear_stem: 0.44, leaf_cover: 3.0,
        leaf_len_m: 0.07, leaf_aspect: 1.25, evergreen: false, leaf_out: 0.30, leaf_fall: 0.84, street_tolerant: true,
    },
    species! {
        key: "wu-tong", name_zh: "梧桐", name_en: "London plane", habit: Habit::Open, leaf: LeafForm::Palmate,
        bark: bark([0.255, 0.245, 0.228], 0.18), foliage: [0.122, 0.223, 0.034], autumn: Some([0.200, 0.160, 0.085]), bloom: None,
        height_m: (12.0, 20.0), crown_m: (6.0, 9.0), trunk_m: (0.30, 0.45), clear_stem: 0.45, leaf_cover: 3.2,
        leaf_len_m: 0.20, leaf_aspect: 1.05, evergreen: false, leaf_out: 0.31, leaf_fall: 0.86, street_tolerant: true,
    },
    species! {
        key: "huai-shu", name_zh: "槐树", name_en: "Scholar tree", habit: Habit::Rounded, leaf: LeafForm::Pinnate,
        bark: bark([0.185, 0.175, 0.160], 0.60), foliage: [0.117, 0.262, 0.034], autumn: Some([0.260, 0.190, 0.070]), bloom: None,
        height_m: (10.0, 16.0), crown_m: (5.5, 8.0), trunk_m: (0.28, 0.40), clear_stem: 0.34, leaf_cover: 3.6,
        leaf_len_m: 0.22, leaf_aspect: 0.32, evergreen: false, leaf_out: 0.32, leaf_fall: 0.87, street_tolerant: true,
    },
    species! {
        key: "yu-shu", name_zh: "榆树", name_en: "Elm", habit: Habit::Rounded, leaf: LeafForm::Ovate,
        bark: bark([0.165, 0.150, 0.132], 0.75), foliage: [0.089, 0.202, 0.025], autumn: Some([0.285, 0.205, 0.058]), bloom: None,
        height_m: (12.0, 20.0), crown_m: (6.0, 8.5), trunk_m: (0.28, 0.42), clear_stem: 0.36, leaf_cover: 4.2,
        leaf_len_m: 0.07, leaf_aspect: 0.48, evergreen: false, leaf_out: 0.30, leaf_fall: 0.88, street_tolerant: true,
    },
    species! {
        key: "gui-hua", name_zh: "桂花", name_en: "Osmanthus", habit: Habit::Oval, leaf: LeafForm::Elliptic,
        bark: bark([0.155, 0.148, 0.138], 0.35), foliage: [0.021, 0.098, 0.025], autumn: None,
        bloom: Some(Bloom { colour: [0.550, 0.440, 0.280], density: 0.32, at: 0.70 }),
        height_m: (4.0, 8.0), crown_m: (2.2, 3.4), trunk_m: (0.12, 0.20), clear_stem: 0.26, leaf_cover: 5.0,
        leaf_len_m: 0.09, leaf_aspect: 0.38, evergreen: true, leaf_out: 0.28, leaf_fall: 0.97, street_tolerant: true,
    },
    species! {
        key: "xue-song", name_zh: "雪松", name_en: "Deodar cedar", habit: Habit::Layered, leaf: LeafForm::Needle,
        bark: bark([0.145, 0.130, 0.115], 0.65), foliage: [0.042, 0.107, 0.076], autumn: None, bloom: None,
        height_m: (12.0, 20.0), crown_m: (4.5, 6.5), trunk_m: (0.26, 0.40), clear_stem: 0.18, leaf_cover: 4.0,
        leaf_len_m: 0.11, leaf_aspect: 0.55, evergreen: true, leaf_out: 0.30, leaf_fall: 0.97, street_tolerant: true,
    },
    species! {
        key: "liu-shu", name_zh: "柳树", name_en: "Willow", habit: Habit::Weeping, leaf: LeafForm::Lanceolate,
        bark: bark([0.140, 0.130, 0.112], 0.50), foliage: [0.223, 0.352, 0.045], autumn: Some([0.245, 0.205, 0.080]), bloom: None,
        height_m: (8.0, 14.0), crown_m: (5.0, 7.5), trunk_m: (0.22, 0.34), clear_stem: 0.28, leaf_cover: 3.4,
        leaf_len_m: 0.11, leaf_aspect: 0.13, evergreen: false, leaf_out: 0.24, leaf_fall: 0.90, street_tolerant: false,
    },
    species! {
        key: "song-shu", name_zh: "松树", name_en: "Pine", habit: Habit::Irregular, leaf: LeafForm::Needle,
        bark: bark([0.150, 0.128, 0.105], 0.90), foliage: [0.034, 0.089, 0.025], autumn: None, bloom: None,
        height_m: (9.0, 16.0), crown_m: (4.0, 6.5), trunk_m: (0.26, 0.42), clear_stem: 0.35, leaf_cover: 2.8,
        leaf_len_m: 0.14, leaf_aspect: 0.50, evergreen: true, leaf_out: 0.30, leaf_fall: 0.97, street_tolerant: false,
    },
    species! {
        key: "shui-shan", name_zh: "水杉", name_en: "Dawn redwood", habit: Habit::Conical, leaf: LeafForm::Needle,
        bark: bark([0.175, 0.153, 0.140], 0.60), foliage: [0.156, 0.305, 0.045], autumn: Some([0.200, 0.138, 0.076]), bloom: None,
        height_m: (18.0, 30.0), crown_m: (2.4, 3.6), trunk_m: (0.22, 0.34), clear_stem: 0.30, leaf_cover: 4.0,
        leaf_len_m: 0.07, leaf_aspect: 0.45, evergreen: false, leaf_out: 0.32, leaf_fall: 0.88, street_tolerant: true,
    },
    species! {
        key: "ci-huai", name_zh: "刺槐", name_en: "Black locust", habit: Habit::Rounded, leaf: LeafForm::Pinnate,
        bark: bark([0.175, 0.165, 0.150], 0.80), foliage: [0.138, 0.296, 0.040], autumn: Some([0.270, 0.210, 0.062]),
        bloom: Some(Bloom { colour: [0.550, 0.550, 0.520], density: 0.35, at: 0.40 }),
        height_m: (10.0, 16.0), crown_m: (4.5, 6.5), trunk_m: (0.24, 0.36), clear_stem: 0.40, leaf_cover: 2.8,
        leaf_len_m: 0.20, leaf_aspect: 0.34, evergreen: false, leaf_out: 0.34, leaf_fall: 0.87, street_tolerant: true,
    },
    species! {
        key: "rong-shu", name_zh: "榕树", name_en: "Banyan", habit: Habit::Banyan, leaf: LeafForm::Ovate,
        bark: bark([0.175, 0.162, 0.148], 0.45), foliage: [0.027, 0.122, 0.025], autumn: None, bloom: None,
        height_m: (10.0, 18.0), crown_m: (8.0, 12.0), trunk_m: (0.35, 0.55), clear_stem: 0.24, leaf_cover: 5.2,
        leaf_len_m: 0.10, leaf_aspect: 0.48, evergreen: true, leaf_out: 0.30, leaf_fall: 0.97, street_tolerant: true,
    },
    species! {
        key: "tao-shu", name_zh: "桃树", name_en: "Peach", habit: Habit::Vase, leaf: LeafForm::Lanceolate,
        bark: bark([0.190, 0.170, 0.150], 0.45), foliage: [0.127, 0.296, 0.042], autumn: Some([0.235, 0.115, 0.075]),
        bloom: Some(Bloom { colour: [0.620, 0.295, 0.360], density: 0.88, at: 0.16 }),
        height_m: (3.5, 6.5), crown_m: (2.8, 4.2), trunk_m: (0.12, 0.20), clear_stem: 0.30, leaf_cover: 3.2,
        leaf_len_m: 0.11, leaf_aspect: 0.22, evergreen: false, leaf_out: 0.27, leaf_fall: 0.88, street_tolerant: false,
    },
    species! {
        key: "sha-shu", name_zh: "杉树", name_en: "Chinese fir", habit: Habit::Conical, leaf: LeafForm::Needle,
        bark: bark([0.130, 0.118, 0.108], 0.55), foliage: [0.025, 0.107, 0.061], autumn: None, bloom: None,
        height_m: (14.0, 22.0), crown_m: (2.4, 3.4), trunk_m: (0.20, 0.30), clear_stem: 0.40, leaf_cover: 4.4,
        leaf_len_m: 0.06, leaf_aspect: 0.50, evergreen: true, leaf_out: 0.30, leaf_fall: 0.97, street_tolerant: true,
    },
    species! {
        key: "he-huan", name_zh: "合欢", name_en: "Silk tree", habit: Habit::Umbrella, leaf: LeafForm::Pinnate,
        bark: bark([0.165, 0.152, 0.135], 0.40), foliage: [0.156, 0.296, 0.061], autumn: None,
        bloom: Some(Bloom { colour: [0.600, 0.270, 0.360], density: 0.55, at: 0.44 }),
        height_m: (8.0, 14.0), crown_m: (5.0, 7.5), trunk_m: (0.20, 0.30), clear_stem: 0.42, leaf_cover: 2.6,
        leaf_len_m: 0.30, leaf_aspect: 0.45, evergreen: false, leaf_out: 0.36, leaf_fall: 0.82, street_tolerant: true,
    },
    species! {
        key: "yang-shu", name_zh: "杨树", name_en: "Poplar", habit: Habit::Fastigiate, leaf: LeafForm::Ovate,
        bark: bark([0.185, 0.175, 0.158], 0.50), foliage: [0.117, 0.262, 0.030], autumn: Some([0.300, 0.225, 0.055]), bloom: None,
        height_m: (16.0, 26.0), crown_m: (2.6, 4.0), trunk_m: (0.24, 0.38), clear_stem: 0.18, leaf_cover: 3.6,
        leaf_len_m: 0.085, leaf_aspect: 0.70, evergreen: false, leaf_out: 0.30, leaf_fall: 0.88, street_tolerant: true,
    },
    // --- Gardens, orchards and the countryside -----------------------------------
    species! {
        key: "ping-guo", name_zh: "苹果", name_en: "Apple", habit: Habit::Vase, leaf: LeafForm::Elliptic,
        bark: bark([0.170, 0.150, 0.130], 0.50), foliage: [0.089, 0.223, 0.030], autumn: Some([0.230, 0.180, 0.060]),
        bloom: Some(Bloom { colour: [0.640, 0.540, 0.540], density: 0.7, at: 0.30 }),
        height_m: (3.0, 5.5), crown_m: (2.4, 3.6), trunk_m: (0.09, 0.16), clear_stem: 0.32, leaf_cover: 3.6,
        leaf_len_m: 0.075, leaf_aspect: 0.58, evergreen: false, leaf_out: 0.30, leaf_fall: 0.85, street_tolerant: false,
    },
    species! {
        key: "shi-shu", name_zh: "柿树", name_en: "Persimmon", habit: Habit::Rounded, leaf: LeafForm::Ovate,
        bark: bark([0.140, 0.125, 0.110], 0.85), foliage: [0.065, 0.181, 0.021], autumn: Some([0.360, 0.130, 0.040]), bloom: None,
        height_m: (6.0, 11.0), crown_m: (3.5, 5.5), trunk_m: (0.16, 0.28), clear_stem: 0.30, leaf_cover: 3.4,
        leaf_len_m: 0.13, leaf_aspect: 0.55, evergreen: false, leaf_out: 0.33, leaf_fall: 0.86, street_tolerant: false,
    },
    species! {
        key: "feng-shu", name_zh: "枫树", name_en: "Maple", habit: Habit::Rounded, leaf: LeafForm::Palmate,
        bark: bark([0.185, 0.170, 0.150], 0.40), foliage: [0.076, 0.223, 0.021], autumn: Some([0.420, 0.080, 0.030]), bloom: None,
        height_m: (8.0, 15.0), crown_m: (4.0, 6.5), trunk_m: (0.20, 0.34), clear_stem: 0.32, leaf_cover: 3.8,
        leaf_len_m: 0.09, leaf_aspect: 1.0, evergreen: false, leaf_out: 0.30, leaf_fall: 0.88, street_tolerant: true,
    },
    species! {
        key: "ying-hua", name_zh: "樱花", name_en: "Cherry", habit: Habit::Vase, leaf: LeafForm::Elliptic,
        bark: bark([0.200, 0.160, 0.140], 0.35), foliage: [0.107, 0.246, 0.034], autumn: Some([0.330, 0.150, 0.050]),
        bloom: Some(Bloom { colour: [0.680, 0.420, 0.480], density: 0.90, at: 0.22 }),
        height_m: (5.0, 9.0), crown_m: (3.2, 5.0), trunk_m: (0.14, 0.24), clear_stem: 0.30, leaf_cover: 3.0,
        leaf_len_m: 0.10, leaf_aspect: 0.50, evergreen: false, leaf_out: 0.30, leaf_fall: 0.86, street_tolerant: true,
    },
    species! {
        key: "yu-lan", name_zh: "玉兰", name_en: "Magnolia", habit: Habit::Oval, leaf: LeafForm::Ovate,
        bark: bark([0.210, 0.200, 0.185], 0.20), foliage: [0.032, 0.138, 0.027], autumn: None,
        bloom: Some(Bloom { colour: [0.700, 0.660, 0.640], density: 0.6, at: 0.19 }),
        height_m: (6.0, 12.0), crown_m: (3.0, 5.0), trunk_m: (0.16, 0.30), clear_stem: 0.26, leaf_cover: 4.4,
        leaf_len_m: 0.14, leaf_aspect: 0.50, evergreen: true, leaf_out: 0.30, leaf_fall: 0.97, street_tolerant: true,
    },
    species! {
        key: "sang-shu", name_zh: "桑树", name_en: "Mulberry", habit: Habit::Rounded, leaf: LeafForm::Ovate,
        bark: bark([0.170, 0.150, 0.125], 0.70), foliage: [0.112, 0.262, 0.032], autumn: Some([0.280, 0.230, 0.070]), bloom: None,
        height_m: (5.0, 10.0), crown_m: (3.0, 5.0), trunk_m: (0.14, 0.26), clear_stem: 0.28, leaf_cover: 4.0,
        leaf_len_m: 0.12, leaf_aspect: 0.75, evergreen: false, leaf_out: 0.28, leaf_fall: 0.88, street_tolerant: false,
    },
    species! {
        key: "zao-shu", name_zh: "枣树", name_en: "Jujube", habit: Habit::Irregular, leaf: LeafForm::Elliptic,
        bark: bark([0.150, 0.130, 0.110], 0.80), foliage: [0.107, 0.246, 0.027], autumn: Some([0.300, 0.250, 0.060]), bloom: None,
        height_m: (4.0, 8.0), crown_m: (2.4, 4.0), trunk_m: (0.10, 0.20), clear_stem: 0.30, leaf_cover: 2.6,
        leaf_len_m: 0.05, leaf_aspect: 0.50, evergreen: false, leaf_out: 0.36, leaf_fall: 0.86, street_tolerant: false,
    },
];

pub fn by_key(key: &str) -> Option<usize> {
    SPECIES.iter().position(|s| s.key == key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_catalogue_is_consistent_and_every_habit_and_leaf_form_is_used() {
        let mut keys: Vec<_> = SPECIES.iter().map(|s| s.key).collect();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), SPECIES.len(), "keys are unique");
        for s in SPECIES {
            assert!(
                s.height_m.0 > 0.5 && s.height_m.0 < s.height_m.1,
                "{}",
                s.key
            );
            assert!(s.crown_m.0 > 0.5 && s.crown_m.0 < s.crown_m.1, "{}", s.key);
            assert!(s.trunk_m.0 > 0.02 && s.trunk_m.0 < s.trunk_m.1, "{}", s.key);
            // A trunk is not thicker than the crown is wide, nor a crown wider than three times the height.
            assert!(
                s.trunk_m.1 < s.crown_m.1 && s.crown_m.1 < s.height_m.1,
                "{}",
                s.key
            );
            assert!((0.05..0.7).contains(&s.clear_stem));
            assert!((1.0..7.0).contains(&s.leaf_cover));
            assert!(s.leaf_len_m > 0.02 && s.leaf_aspect > 0.08);
            assert!(s.leaf_out < s.leaf_fall);
        }
        for habit in [
            Habit::Rounded,
            Habit::Open,
            Habit::Oval,
            Habit::Fan,
            Habit::Vase,
            Habit::Conical,
            Habit::Layered,
            Habit::Weeping,
            Habit::Fastigiate,
            Habit::Irregular,
            Habit::Umbrella,
            Habit::Banyan,
        ] {
            assert!(
                SPECIES.iter().any(|s| s.habit == habit),
                "{habit:?} is never grown"
            );
        }
        for form in [
            LeafForm::Ovate,
            LeafForm::Palmate,
            LeafForm::Elliptic,
            LeafForm::Needle,
            LeafForm::Pinnate,
            LeafForm::Lanceolate,
            LeafForm::Fan,
        ] {
            assert!(
                SPECIES.iter().any(|s| s.leaf == form),
                "{form:?} is on no tree"
            );
        }
    }
}
