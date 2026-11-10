//! Planting zones: which trees a town gets, from the climate and landform it
//! sits on.
//!
//! The tables are data. A zone is a region of climate space (annual mean
//! temperature, wetness, elevation, distance to the sea) with a planting
//! allocation for each role a tree can have. The town's [`SiteClimate`] — read
//! from the terrain simulation's temperature, moisture, height, snow and sea
//! fields — is scored against every zone with trapezoid memberships and the best
//! fit plants the town. Nothing here is a conditional on a species.
//!
//! # Where the numbers come from
//!
//! The ranges follow the mean annual temperature and rainfall of the Chinese
//! cities whose street-tree lists are published, read against the vegetation
//! regions of 《中国植被》 (Wu Zhengyi, 1980) and Köppen-Geiger as mapped for
//! China (Peel et al. 2007):
//!
//! | zone | stand-in cities (mean annual °C) | street and park trees |
//! |---|---|---|
//! | cold northeast | Harbin 4, Changchun 6 | elm, poplar, willow, Mongolian pine, spruce, birch |
//! | temperate north | Beijing 13, Taiyuan 10, Shenyang 8 | scholar tree, poplar, willow, ash, ginkgo, goldenrain, Chinese pine |
//! | warm temperate | Nanjing 16, Zhengzhou 15, Xi'an 14 | plane, scholar tree, camphor, ginkgo, ash |
//! | humid subtropical | Chengdu 16, Wuhan 17, Fuzhou 20 | camphor, banyan, magnolia, osmanthus, silk tree |
//! | coast, warm | Xiamen 21, Guangzhou 22, Haikou 24 | banyan, camphor, osmanthus, crape myrtle, pine |
//! | coast, cool | Qingdao 13, Dalian 11 | cedar, black pine, scholar tree, poplar, plane |
//! | arid northwest | Urumqi 7, Lanzhou 10, Yinchuan 9 | Xinjiang poplar, Euphrates poplar, Russian olive, elm, tamarisk |
//! | plateau and alpine | Xining 6, Lhasa 8, Lanzhou hills | Qinghai spruce, Qilian juniper, Tibetan poplar, willow |
//!
//! Tree *species not yet modelled* are mapped to the nearest form in the palette
//! and listed in `docs/vegetation-climate.md`: palms (棕榈, 大王椰子, 蒲葵) and
//! casuarina (木麻黄) of the warm coast, bamboo (竹), and the litchi and mango of
//! southern gardens, have no canopy form here yet.
//!
//! # What else a zone changes
//!
//! Besides the allocation, a zone says how many of the planted trees survive
//! (`keep`: a dry town waters only its road strips and parks, so most of the
//! parcel and verge planting is simply absent) and how big they grow (`size`:
//! the table's mature height is for a watered temperate site; a drought-limited
//! or cold-limited tree is shorter). Both are further scaled continuously by the
//! site's moisture, so two towns in one zone still differ.

use urban::SiteClimate;

/// A planting allocation: species key and relative share.
pub(crate) type Share = &'static [(&'static str, u32)];

/// One allocation per role a tree can have.
#[derive(Clone, Copy)]
pub(crate) struct ZonePlanting {
    pub street: Share,
    pub motorway: Share,
    pub median: Share,
    pub park: Share,
    pub compound: Share,
    pub waterfront: Share,
    pub farm: Share,
    pub orchard: Share,
    pub island: Share,
}

/// A trapezoid on one climate axis: zero outside `(lo0, hi0)`, one inside
/// `(lo1, hi1)`, linear between.
#[derive(Clone, Copy)]
pub(crate) struct Band {
    pub lo0: f32,
    pub lo1: f32,
    pub hi1: f32,
    pub hi0: f32,
}

impl Band {
    pub const ANY: Band = Band {
        lo0: f32::MIN,
        lo1: f32::MIN,
        hi1: f32::MAX,
        hi0: f32::MAX,
    };

    const fn new(lo0: f32, lo1: f32, hi1: f32, hi0: f32) -> Band {
        Band { lo0, lo1, hi1, hi0 }
    }

    pub fn membership(&self, v: f32) -> f32 {
        if v <= self.lo0 || v >= self.hi0 {
            0.0
        } else if v < self.lo1 {
            (v - self.lo0) / (self.lo1 - self.lo0).max(1.0e-6)
        } else if v > self.hi1 {
            (self.hi0 - v) / (self.hi0 - self.hi1).max(1.0e-6)
        } else {
            1.0
        }
    }
}

/// One planting zone.
#[allow(dead_code)] // `name_zh` is documentation and the audit report's label
pub(crate) struct Zone {
    pub key: &'static str,
    pub name_zh: &'static str,
    pub temperature_c: Band,
    pub moisture: Band,
    pub elevation_m: Band,
    pub coast_km: Band,
    /// Tie-break and specificity: a narrow zone (alpine, coast) outbids the
    /// broad zone it sits inside.
    pub priority: f32,
    pub planting: ZonePlanting,
    /// Share of planted trees that survive, at the zone's own moisture.
    pub keep: f32,
    /// Height multiplier on the table's mature height.
    pub size: f32,
}

/// What a town's planting looks like once the zone and the site are folded
/// together.
#[derive(Clone, Copy)]
pub(crate) struct Environment {
    pub zone: &'static Zone,
    pub keep: f32,
    pub size: f32,
    /// 0 (watered) to 1 (arid): how straw-coloured and bare the ground cover is.
    pub dry: f32,
    /// A high-mountain site: the ground cover is alpine meadow, kept (not
    /// thinned) and cool-toned.
    pub alpine: bool,
}

// ---------------------------------------------------------------------------
// The allocations. Weights are shares of an allocation, not a ranking.
// ---------------------------------------------------------------------------

/// The warm-temperate allocation is the planting the ChineseModern style has
/// always used (plane and scholar tree carry a Yangtze-to-Yellow-River avenue),
/// kept verbatim so a plan made without terrain does not change a tree.
const WARM_TEMPERATE: ZonePlanting = ZonePlanting {
    street: &[
        ("huai-shu", 32),
        ("wu-tong", 28),
        ("xiang-zhang", 14),
        ("yin-xing", 9),
        ("bai-la", 9),
        ("yang-shu", 8),
    ],
    motorway: &[("yang-shu", 4), ("huai-shu", 3), ("bai-la", 3)],
    median: &[
        ("huai-shu", 32),
        ("wu-tong", 28),
        ("xiang-zhang", 14),
        ("yin-xing", 9),
        ("bai-la", 9),
        ("yang-shu", 8),
    ],
    park: &[("huai-shu", 3), ("yin-xing", 3), ("xiang-zhang", 4)],
    compound: &[
        ("xue-song", 3),
        ("long-bai", 3),
        ("xiang-zhang", 3),
        ("gui-hua", 4),
        ("zi-wei", 2),
        ("tao-shu", 2),
    ],
    waterfront: &[
        ("liu-shu", 5),
        ("shui-shan", 2),
        ("yang-shu", 2),
        ("yu-shu", 1),
    ],
    farm: &[
        ("yang-shu", 4),
        ("liu-shu", 3),
        ("huai-shu", 2),
        ("yu-shu", 2),
        ("tao-shu", 1),
        ("ci-huai", 1),
    ],
    orchard: &[("tao-shu", 1)],
    island: &[
        ("gui-hua", 3),
        ("tao-shu", 2),
        ("yin-xing", 2),
        ("zi-wei", 2),
    ],
};

const TEMPERATE_NORTH: ZonePlanting = ZonePlanting {
    street: &[
        ("huai-shu", 34),
        ("yang-shu", 14),
        ("bai-la", 14),
        ("luan-shu", 12),
        ("yin-xing", 8),
        ("yu-shu", 8),
        ("liu-shu", 5),
        ("song-shu", 5),
    ],
    motorway: &[
        ("yang-shu", 5),
        ("huai-shu", 3),
        ("bai-la", 2),
        ("song-shu", 2),
    ],
    median: &[
        ("huai-shu", 4),
        ("luan-shu", 3),
        ("bai-la", 3),
        ("tao-shu", 1),
    ],
    park: &[
        ("huai-shu", 4),
        ("song-shu", 3),
        ("yin-xing", 2),
        ("luan-shu", 2),
        ("yu-lan", 2),
        ("liu-shu", 2),
        ("yang-shu", 2),
        ("ci-huai", 1),
    ],
    compound: &[
        ("huai-shu", 4),
        ("song-shu", 3),
        ("yu-lan", 2),
        ("tao-shu", 2),
        ("long-bai", 2),
        ("luan-shu", 1),
    ],
    waterfront: &[("liu-shu", 6), ("yang-shu", 2), ("yu-shu", 2)],
    farm: &[
        ("yang-shu", 5),
        ("yu-shu", 3),
        ("huai-shu", 2),
        ("ci-huai", 2),
        ("liu-shu", 1),
    ],
    orchard: &[("tao-shu", 1)],
    island: &[
        ("song-shu", 3),
        ("yu-lan", 2),
        ("tao-shu", 2),
        ("yin-xing", 1),
    ],
};

const COLD_NORTHEAST: ZonePlanting = ZonePlanting {
    street: &[
        ("yu-shu", 34),
        ("yang-shu", 26),
        ("liu-shu", 12),
        ("bai-hua", 10),
        ("song-shu", 8),
        ("sha-shu", 6),
        ("ci-huai", 4),
    ],
    motorway: &[("yang-shu", 5), ("yu-shu", 3), ("sha-shu", 2)],
    median: &[("yu-shu", 4), ("bai-hua", 3), ("sha-shu", 2)],
    park: &[
        ("bai-hua", 4),
        ("song-shu", 4),
        ("sha-shu", 3),
        ("yu-shu", 3),
        ("yang-shu", 2),
        ("liu-shu", 1),
    ],
    compound: &[
        ("sha-shu", 4),
        ("yu-shu", 3),
        ("bai-hua", 3),
        ("song-shu", 2),
    ],
    waterfront: &[
        ("liu-shu", 5),
        ("yang-shu", 3),
        ("bai-hua", 2),
        ("lu-wei", 3),
    ],
    farm: &[
        ("yang-shu", 6),
        ("yu-shu", 3),
        ("liu-shu", 2),
        ("bai-hua", 1),
    ],
    orchard: &[("tao-shu", 1)],
    island: &[("sha-shu", 3), ("bai-hua", 2), ("song-shu", 2)],
};

const HUMID_SUBTROPICAL: ZonePlanting = ZonePlanting {
    street: &[
        ("xiang-zhang", 38),
        ("rong-shu", 12),
        ("wu-tong", 14),
        ("yin-xing", 6),
        ("luan-shu", 8),
        ("he-huan", 8),
        ("gui-hua", 8),
        ("yu-lan", 6),
    ],
    motorway: &[("xiang-zhang", 4), ("rong-shu", 2), ("shui-shan", 2)],
    median: &[
        ("gui-hua", 3),
        ("zi-wei", 3),
        ("he-huan", 2),
        ("tao-shu", 1),
    ],
    park: &[
        ("xiang-zhang", 4),
        ("rong-shu", 3),
        ("yu-lan", 2),
        ("gui-hua", 2),
        ("shui-shan", 2),
        ("liu-shu", 2),
        ("he-huan", 1),
        ("yin-xing", 1),
    ],
    compound: &[
        ("xiang-zhang", 4),
        ("gui-hua", 5),
        ("yu-lan", 3),
        ("zhu", 3),
        ("zi-wei", 2),
        ("tao-shu", 2),
        ("long-bai", 1),
    ],
    waterfront: &[
        ("liu-shu", 4),
        ("shui-shan", 3),
        ("lu-wei", 3),
        ("zhu", 2),
        ("rong-shu", 2),
        ("xiang-zhang", 1),
    ],
    farm: &[
        ("zhu", 4),
        ("xiang-zhang", 3),
        ("liu-shu", 2),
        ("tao-shu", 2),
        ("yu-shu", 1),
        ("gui-hua", 1),
    ],
    orchard: &[("tao-shu", 1)],
    island: &[("gui-hua", 3), ("zi-wei", 2), ("he-huan", 2), ("yu-lan", 2)],
};

const COAST_WARM: ZonePlanting = ZonePlanting {
    street: &[
        ("rong-shu", 20),
        ("xiang-zhang", 14),
        ("da-wang-ye", 12),
        ("pu-kui", 10),
        ("ye-zi", 8),
        ("mu-ma-huang", 8),
        ("gui-hua", 8),
        ("he-huan", 6),
        ("zi-wei", 4),
        ("zong-lu", 6),
    ],
    motorway: &[
        ("mu-ma-huang", 4),
        ("rong-shu", 3),
        ("da-wang-ye", 2),
        ("song-shu", 2),
    ],
    median: &[("zong-lu", 3), ("gui-hua", 3), ("zi-wei", 3), ("pu-kui", 2)],
    park: &[
        ("ye-zi", 4),
        ("rong-shu", 4),
        ("pu-kui", 3),
        ("da-wang-ye", 3),
        ("mu-ma-huang", 3),
        ("zhu", 3),
        ("xiang-zhang", 2),
        ("he-huan", 2),
        ("zong-lu", 2),
    ],
    compound: &[
        ("gui-hua", 4),
        ("zong-lu", 3),
        ("zhu", 3),
        ("xiang-zhang", 3),
        ("zi-wei", 3),
        ("rong-shu", 2),
        ("pu-kui", 2),
    ],
    waterfront: &[
        ("hong-shu", 5),
        ("lu-wei", 4),
        ("mu-ma-huang", 3),
        ("ye-zi", 3),
        ("rong-shu", 3),
        ("liu-shu", 2),
    ],
    farm: &[
        ("rong-shu", 3),
        ("mu-ma-huang", 3),
        ("xiang-zhang", 2),
        ("zhu", 2),
        ("ye-zi", 1),
    ],
    orchard: &[("tao-shu", 1)],
    island: &[
        ("ye-zi", 3),
        ("da-wang-ye", 3),
        ("zi-wei", 2),
        ("gui-hua", 2),
    ],
};

const COAST_COOL: ZonePlanting = ZonePlanting {
    street: &[
        ("huai-shu", 26),
        ("wu-tong", 22),
        ("xue-song", 14),
        ("song-shu", 14),
        ("yang-shu", 8),
        ("bai-la", 8),
        ("yin-xing", 8),
    ],
    motorway: &[("song-shu", 4), ("yang-shu", 3), ("xue-song", 2)],
    median: &[("tao-shu", 3), ("huai-shu", 3), ("song-shu", 2)],
    park: &[
        ("song-shu", 5),
        ("xue-song", 3),
        ("huai-shu", 3),
        ("yin-xing", 2),
        ("yu-lan", 1),
        ("liu-shu", 1),
    ],
    compound: &[
        ("xue-song", 4),
        ("song-shu", 3),
        ("tao-shu", 3),
        ("long-bai", 2),
        ("yu-lan", 1),
    ],
    waterfront: &[("song-shu", 4), ("liu-shu", 3), ("yang-shu", 2)],
    farm: &[
        ("yang-shu", 4),
        ("song-shu", 3),
        ("ci-huai", 2),
        ("liu-shu", 1),
    ],
    orchard: &[("tao-shu", 1)],
    island: &[("song-shu", 3), ("tao-shu", 2), ("xue-song", 2)],
};

const ARID_NORTHWEST: ZonePlanting = ZonePlanting {
    street: &[
        ("yang-shu", 36),
        ("yu-shu", 22),
        ("hu-yang", 14),
        ("sha-zao", 10),
        ("huai-shu", 8),
        ("bai-la", 4),
    ],
    motorway: &[("yang-shu", 6), ("sha-zao", 3)],
    median: &[
        ("sha-zao", 4),
        ("sha-ji", 4),
        ("suo-suo", 3),
        ("yang-shu", 2),
    ],
    park: &[
        ("hu-yang", 4),
        ("yang-shu", 4),
        ("yu-shu", 3),
        ("sha-zao", 3),
        ("suo-suo", 3),
        ("sha-ji", 3),
        ("song-shu", 2),
        ("liu-shu", 2),
    ],
    compound: &[
        ("yu-shu", 3),
        ("sha-zao", 3),
        ("yang-shu", 3),
        ("sha-ji", 2),
        ("long-bai", 2),
    ],
    waterfront: &[
        ("hu-yang", 4),
        ("liu-shu", 4),
        ("lu-wei", 4),
        ("sha-ji", 2),
        ("yang-shu", 2),
    ],
    farm: &[
        ("yang-shu", 6),
        ("yu-shu", 3),
        ("sha-zao", 3),
        ("suo-suo", 2),
        ("liu-shu", 1),
    ],
    orchard: &[("tao-shu", 1)],
    island: &[
        ("sha-zao", 3),
        ("sha-ji", 3),
        ("suo-suo", 2),
        ("long-bai", 2),
    ],
};

/// Qinghai spruce (青海云杉) is `sha-shu`, the blue-green cone; Qilian juniper
/// (祁连圆柏) is `long-bai`, the dense column; Tibetan poplar (藏川杨) is the
/// fastigiate poplar.
const PLATEAU: ZonePlanting = ZonePlanting {
    street: &[
        ("yang-shu", 36),
        ("sha-shu", 22),
        ("long-bai", 14),
        ("liu-shu", 10),
        ("sha-zao", 10),
    ],
    motorway: &[("yang-shu", 5), ("sha-shu", 3), ("long-bai", 2)],
    median: &[
        ("sha-ji", 4),
        ("dian-zhuang", 3),
        ("sha-zao", 3),
        ("long-bai", 2),
    ],
    park: &[
        ("sha-shu", 5),
        ("long-bai", 3),
        ("yang-shu", 3),
        ("dian-zhuang", 4),
        ("sha-ji", 3),
        ("liu-shu", 2),
        ("sha-zao", 2),
    ],
    compound: &[
        ("sha-shu", 3),
        ("long-bai", 3),
        ("yang-shu", 3),
        ("sha-zao", 2),
    ],
    waterfront: &[
        ("liu-shu", 5),
        ("yang-shu", 4),
        ("sha-ji", 3),
        ("lu-wei", 2),
    ],
    farm: &[("yang-shu", 6), ("liu-shu", 3), ("sha-zao", 2)],
    orchard: &[("tao-shu", 1)],
    island: &[
        ("long-bai", 3),
        ("dian-zhuang", 3),
        ("sha-ji", 2),
        ("sha-shu", 2),
    ],
};

// ---------------------------------------------------------------------------
// The table. Axis order: temperature (°C), moisture (0..1), elevation (m),
// distance to the sea (km).
//
// The terrain model's temperature is `21 - 10 * |latitude| - 0.0062 * height`,
// so its lowlands run 11-21 °C and anything colder is high ground; the bands
// are written against that scale and against the real cities above.
// ---------------------------------------------------------------------------

pub(crate) const ZONES: &[Zone] = &[
    Zone {
        key: "plateau",
        name_zh: "高原与高山",
        temperature_c: Band::ANY,
        moisture: Band::ANY,
        elevation_m: Band::new(1900.0, 2700.0, 9000.0, 9001.0),
        coast_km: Band::ANY,
        priority: 1.6,
        planting: PLATEAU,
        keep: 0.55,
        size: 0.62,
    },
    Zone {
        key: "arid-northwest",
        name_zh: "西北干旱区",
        temperature_c: Band::ANY,
        moisture: Band::new(-1.0, -0.5, 0.26, 0.38),
        elevation_m: Band::new(-9000.0, -8999.0, 2000.0, 2700.0),
        coast_km: Band::ANY,
        priority: 1.4,
        planting: ARID_NORTHWEST,
        keep: 0.60,
        size: 0.72,
    },
    Zone {
        key: "cold-northeast",
        name_zh: "东北寒温带",
        temperature_c: Band::new(-60.0, -50.0, 5.5, 8.5),
        moisture: Band::new(0.24, 0.36, 2.0, 3.0),
        elevation_m: Band::new(-9000.0, -8999.0, 1500.0, 2400.0),
        coast_km: Band::ANY,
        priority: 1.0,
        planting: COLD_NORTHEAST,
        keep: 0.85,
        size: 0.88,
    },
    Zone {
        key: "temperate-north",
        name_zh: "华北暖温带",
        temperature_c: Band::new(4.0, 7.5, 13.5, 15.5),
        moisture: Band::new(0.26, 0.36, 2.0, 3.0),
        elevation_m: Band::new(-9000.0, -8999.0, 1500.0, 2400.0),
        coast_km: Band::ANY,
        priority: 1.0,
        planting: TEMPERATE_NORTH,
        keep: 0.92,
        size: 0.96,
    },
    Zone {
        key: "warm-temperate",
        name_zh: "黄淮江淮过渡带",
        temperature_c: Band::new(12.5, 14.5, 17.0, 18.5),
        moisture: Band::new(0.26, 0.36, 2.0, 3.0),
        elevation_m: Band::new(-9000.0, -8999.0, 1500.0, 2400.0),
        coast_km: Band::ANY,
        priority: 1.0,
        planting: WARM_TEMPERATE,
        keep: 1.0,
        size: 1.0,
    },
    Zone {
        key: "humid-subtropical",
        name_zh: "亚热带湿润区",
        temperature_c: Band::new(15.5, 17.5, 40.0, 41.0),
        moisture: Band::new(0.40, 0.52, 2.0, 3.0),
        elevation_m: Band::new(-9000.0, -8999.0, 1500.0, 2400.0),
        coast_km: Band::ANY,
        priority: 1.05,
        planting: HUMID_SUBTROPICAL,
        keep: 1.0,
        size: 1.04,
    },
    Zone {
        key: "coast-warm",
        name_zh: "南方滨海",
        temperature_c: Band::new(15.5, 17.5, 40.0, 41.0),
        moisture: Band::new(0.26, 0.36, 2.0, 3.0),
        elevation_m: Band::new(-9000.0, -8999.0, 600.0, 1200.0),
        coast_km: Band::new(-1.0, -0.5, 18.0, 40.0),
        priority: 1.35,
        planting: COAST_WARM,
        keep: 0.95,
        size: 0.88,
    },
    Zone {
        key: "coast-cool",
        name_zh: "北方滨海",
        temperature_c: Band::new(0.0, 5.0, 15.5, 17.0),
        moisture: Band::new(0.26, 0.36, 2.0, 3.0),
        elevation_m: Band::new(-9000.0, -8999.0, 600.0, 1200.0),
        coast_km: Band::new(-1.0, -0.5, 18.0, 40.0),
        priority: 1.35,
        planting: COAST_COOL,
        keep: 0.9,
        size: 0.86,
    },
];

/// The zone whose climate region best contains the site.
pub(crate) fn zone_for(site: &SiteClimate) -> &'static Zone {
    let mut best = &ZONES[3];
    let mut best_score = -1.0_f32;
    for zone in ZONES {
        let fit = zone.temperature_c.membership(site.temperature_c)
            * zone.moisture.membership(site.moisture)
            * zone.elevation_m.membership(site.elevation_m)
            * zone.coast_km.membership(site.coast_km);
        let score = fit * zone.priority;
        if score > best_score {
            best_score = score;
            best = zone;
        }
    }
    best
}

/// The zone, with the site's own wetness and snow folded into how many trees
/// survive and how tall they grow.
pub(crate) fn environment_for(site: &SiteClimate) -> Environment {
    let zone = zone_for(site);
    // Within a zone, wetter sites are lusher. 0.5 is the reference wetness the
    // tables are written for; the swing is bounded so a dry town keeps its
    // avenue and a wet one does not double it.
    let wet = (site.moisture - 0.5).clamp(-0.4, 0.4);
    let keep = (zone.keep * (1.0 + wet * 0.5)).clamp(0.12, 1.0);
    // Persistent snow cover is a growing-season cut: trees are shorter near it.
    let snow = (1.0 - site.snow * 0.35).clamp(0.6, 1.0);
    let size = (zone.size * (1.0 + wet * 0.30) * snow).clamp(0.35, 1.25);
    let dry = ((0.45 - site.moisture) / 0.33).clamp(0.0, 1.0);
    Environment {
        zone,
        keep,
        size,
        dry,
        alpine: zone.key == "plateau",
    }
}

/// The kerb-tree allocation of the zone a site falls in, as `(key, share)`, for
/// the far-form crowns the terrain view draws from the plan alone.
pub(crate) fn street_mix(site: &SiteClimate) -> Vec<(String, u32)> {
    zone_for(site)
        .planting
        .street
        .iter()
        .map(|&(key, weight)| (key.to_string(), weight))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::species::by_key;

    fn site(t: f32, m: f32, e: f32, c: f32) -> SiteClimate {
        SiteClimate {
            temperature_c: t,
            moisture: m,
            elevation_m: e,
            snow: 0.0,
            coast_km: c,
        }
    }

    #[test]
    fn every_allocation_names_a_real_species() {
        for zone in ZONES {
            let p = zone.planting;
            for list in [
                p.street,
                p.motorway,
                p.median,
                p.park,
                p.compound,
                p.waterfront,
                p.farm,
                p.orchard,
                p.island,
            ] {
                assert!(!list.is_empty(), "{} has an empty list", zone.key);
                for (key, weight) in list {
                    assert!(by_key(key).is_some(), "{}: unknown species {key}", zone.key);
                    assert!(*weight > 0);
                }
            }
        }
    }

    #[test]
    fn the_default_site_is_the_original_planting() {
        assert_eq!(zone_for(&SiteClimate::default()).key, "warm-temperate");
    }

    #[test]
    fn climate_picks_the_zone_a_gardener_would() {
        assert_eq!(
            zone_for(&site(4.0, 0.5, 120.0, 400.0)).key,
            "cold-northeast"
        );
        assert_eq!(
            zone_for(&site(12.0, 0.45, 80.0, 300.0)).key,
            "temperate-north"
        );
        assert_eq!(
            zone_for(&site(15.5, 0.5, 40.0, 300.0)).key,
            "warm-temperate"
        );
        assert_eq!(
            zone_for(&site(20.0, 0.7, 40.0, 300.0)).key,
            "humid-subtropical"
        );
        assert_eq!(zone_for(&site(21.0, 0.7, 20.0, 5.0)).key, "coast-warm");
        assert_eq!(zone_for(&site(13.0, 0.5, 20.0, 5.0)).key, "coast-cool");
        assert_eq!(
            zone_for(&site(13.0, 0.15, 900.0, 400.0)).key,
            "arid-northwest"
        );
        assert_eq!(zone_for(&site(4.0, 0.4, 3300.0, 400.0)).key, "plateau");
    }

    #[test]
    fn the_coast_plants_palms_and_the_dry_zones_plant_their_shrubs() {
        let has = |z: &Zone, key: &str| {
            let p = z.planting;
            [
                p.street,
                p.park,
                p.waterfront,
                p.compound,
                p.median,
                p.island,
            ]
            .iter()
            .any(|l| l.iter().any(|(k, _)| *k == key))
        };
        let coast = zone_for(&site(21.0, 0.7, 20.0, 5.0));
        for key in [
            "da-wang-ye",
            "ye-zi",
            "pu-kui",
            "zong-lu",
            "mu-ma-huang",
            "hong-shu",
            "lu-wei",
            "zhu",
        ] {
            assert!(has(coast, key), "coast-warm lacks {key}");
        }
        let arid = zone_for(&site(13.0, 0.15, 900.0, 400.0));
        for key in ["sha-ji", "suo-suo", "hu-yang"] {
            assert!(has(arid, key), "arid lacks {key}");
        }
        let alpine = zone_for(&site(4.0, 0.4, 3300.0, 400.0));
        assert!(has(alpine, "dian-zhuang") && has(alpine, "sha-ji"));
    }

    #[test]
    fn dry_sites_keep_fewer_and_smaller_trees() {
        let wet = environment_for(&site(18.0, 0.8, 50.0, 300.0));
        let dry = environment_for(&site(13.0, 0.12, 900.0, 400.0));
        assert!(dry.keep < wet.keep * 0.8, "{} vs {}", dry.keep, wet.keep);
        assert!(dry.size < wet.size * 0.8);
    }
}
