//! Shop signs, hanging signs and rooftop company lettering: the catalogue.
//!
//! A Chinese street is read by its signs, and the signs are mostly *names*:
//! chain brands in their house colours (蜜雪冰城, 麦当劳, 中国银行 ...) and a
//! long tail of local shops called 老王面馆, 张记烧烤, 巴蜀火锅.  This module
//! owns that vocabulary and nothing else about how it is drawn:
//!
//! * the **catalogue** — every sign design, its text, colour scheme, style,
//!   font class and mark — generated deterministically here, so Rust is the
//!   single authority on *what* a barber shop or a bank is;
//! * the **manifest** — the catalogue as UTF-8 text carried in a baked texture
//!   (`sign/manifest`), which the renderer reads and rasterises with canvas
//!   text into three sheets (board, vertical, crown) whose cell layout is also
//!   stated in the manifest, so the two sides cannot disagree about where a
//!   design sits;
//! * the **pick** functions the geometry layer uses: which design stands on the
//!   shop at a given place.
//!
//! Text is drawn by the browser rather than baked here because correct,
//! sharp CJK lettering at street level needs a real font; no logo imagery is
//! used anywhere.  Brands are plain text in a colour scheme with a simple
//! geometric mark drawn in code.

use std::fmt::Write as _;
use std::sync::OnceLock;

use worldgen_core::hash::avalanche01;

use crate::textures::BakedTexture;

pub const MANIFEST: &str = "sign/manifest";
pub const BOARD: &str = "sign/board";
pub const VERT: &str = "sign/vert";
pub const CROWN: &str = "sign/crown";
pub const POLE: &str = "sign/pole";

/// Board sheet: cells of 320 x 80 px (one 4.1 m x 1.04 m shop sign), four across.
pub const BOARD_CELL: (usize, usize) = (320, 80);
pub const BOARD_COLS: usize = 4;
/// Vertical hanging signs: 0.7 m x 1.8 m, five across.
pub const VERT_CELL: (usize, usize) = (112, 288);
pub const VERT_COLS: usize = 5;
/// Rooftop lettering: one name per row.
pub const CROWN_CELL: (usize, usize) = (896, 112);
/// Pixels per lettering character and per logo in a crown cell.
pub const CROWN_PITCH: usize = 100;
pub const CROWN_LOGO: usize = 120;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Brand,
    Generic,
    /// 理发 / 美发: carries a rotating barber pole at the door.
    Barber,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub cell: usize,
    pub bays: u8,
    pub kind: Kind,
    /// May stand on the second storey too (restaurants, gyms, game rooms).
    pub upper: bool,
    pub text: String,
    pub sub: String,
    pub style: u8,
    pub font: u8,
    pub colours: [u32; 3],
    pub mark: u8,
}

#[derive(Debug, Clone)]
pub struct Vert {
    pub cell: usize,
    pub text: String,
    pub style: u8,
    pub font: u8,
    pub colours: [u32; 3],
}

#[derive(Debug, Clone)]
pub struct Crown {
    pub cell: usize,
    pub name: String,
    pub style: u8,
    pub font: u8,
    pub colour: u32,
    pub mark: u8,
}

impl Crown {
    /// Width of the drawn block in cell pixels (it is centred in the cell).
    pub fn used_px(&self) -> usize {
        let logo = if self.mark > 0 { CROWN_LOGO } else { 0 };
        logo + self.name.chars().count() * CROWN_PITCH
    }
}

pub struct Catalogue {
    pub boards: Vec<Board>,
    pub verts: Vec<Vert>,
    pub crowns: Vec<Crown>,
    pub board_rows: usize,
    pub vert_rows: usize,
}

const fn rgb(r: u32, g: u32, b: u32) -> u32 {
    (r << 16) | (g << 8) | b
}

struct BrandDef {
    name: &'static str,
    sub: &'static str,
    /// 1, 2, or 3 for both widths.
    bays: u8,
    bg: u32,
    fg: u32,
    acc: u32,
    mark: u8,
    style: u8,
    font: u8,
    upper: bool,
}

const fn brand(
    name: &'static str,
    sub: &'static str,
    bays: u8,
    bg: u32,
    fg: u32,
    acc: u32,
    mark: u8,
    style: u8,
    font: u8,
) -> BrandDef {
    BrandDef {
        name,
        sub,
        bays,
        bg,
        fg,
        acc,
        mark,
        style,
        font,
        upper: false,
    }
}

// Marks (drawn by the renderer): 0 none, 1 snowman, 2 arches, 3 striped bucket,
// 4 siren ring, 5 antlers, 6 arcs, 7 twin loops, 8 open ring, 9 coin, 10 工,
// 11 square ring, 12 cross, 13 hexagon, 14 chevron, 15 check swoosh, 16 bottle,
// 17 bowl, 18 hat, 19 leaf, 20 7-stripe, 21 family stripe, 22 cup, 23 gold bar.
// Styles: 0 flat, 1 light box, 2 3D characters over a band, 3 mark + name,
// 5 neon on dark, 6 striped fascia.
#[rustfmt::skip]
const BRANDS: &[BrandDef] = &[
    brand("蜜雪冰城", "冰淇淋 · 茶饮", 3, rgb(0xd9, 0x0f, 0x1f), rgb(0xff, 0xff, 0xff), rgb(0xff, 0xd2, 0x3f), 1, 3, 0),
    brand("麦当劳", "", 3, rgb(0xd8, 0x1a, 0x1a), rgb(0xff, 0xc7, 0x2c), rgb(0xff, 0xff, 0xff), 2, 3, 0),
    brand("肯德基", "KFC", 3, rgb(0xe0, 0x1b, 0x2f), rgb(0xff, 0xff, 0xff), rgb(0xe0, 0x1b, 0x2f), 3, 3, 0),
    brand("德克士", "", 2, rgb(0xd0, 0x1f, 0x26), rgb(0xff, 0xff, 0xff), rgb(0xff, 0xcb, 0x05), 0, 0, 1),
    brand("必胜客", "PIZZA HUT", 2, rgb(0xc8, 0x16, 0x1d), rgb(0xff, 0xff, 0xff), rgb(0x2b, 0x2b, 0x2b), 18, 3, 0),
    brand("华莱士", "全鸡汉堡", 3, rgb(0xe3, 0x1e, 0x25), rgb(0xff, 0xdd, 0x00), rgb(0xff, 0xff, 0xff), 0, 0, 1),
    brand("正新鸡排", "", 1, rgb(0xc8, 0x16, 0x1d), rgb(0xff, 0xe1, 0x00), rgb(0xff, 0xff, 0xff), 0, 0, 1),
    brand("瑞幸咖啡", "luckin coffee", 3, rgb(0x1e, 0x3a, 0x9c), rgb(0xff, 0xff, 0xff), rgb(0x8f, 0xb6, 0xff), 5, 3, 0),
    brand("星巴克", "STARBUCKS", 2, rgb(0x0b, 0x6b, 0x4a), rgb(0xff, 0xff, 0xff), rgb(0x0b, 0x4a, 0x34), 4, 3, 0),
    brand("中国移动", "China Mobile", 2, rgb(0x00, 0x85, 0xd0), rgb(0xff, 0xff, 0xff), rgb(0xbd, 0xe4, 0xf7), 6, 3, 0),
    brand("中国联通", "China Unicom", 2, rgb(0xe6, 0x00, 0x27), rgb(0xff, 0xff, 0xff), rgb(0xff, 0xff, 0xff), 7, 3, 0),
    brand("中国电信", "China Telecom", 2, rgb(0x00, 0x66, 0xb3), rgb(0xff, 0xff, 0xff), rgb(0xff, 0xdd, 0x33), 8, 3, 0),
    brand("中国邮政储蓄银行", "", 2, rgb(0x00, 0x86, 0x3d), rgb(0xff, 0xff, 0xff), rgb(0xf4, 0x4b, 0x3a), 9, 3, 0),
    brand("中国工商银行", "", 2, rgb(0xc7, 0x00, 0x0b), rgb(0xff, 0xff, 0xff), rgb(0xc7, 0x00, 0x0b), 10, 3, 0),
    brand("中国建设银行", "", 2, rgb(0x00, 0x62, 0xb0), rgb(0xff, 0xff, 0xff), rgb(0x00, 0x62, 0xb0), 11, 3, 0),
    brand("中国农业银行", "", 2, rgb(0x00, 0x91, 0x62), rgb(0xff, 0xff, 0xff), rgb(0x00, 0x91, 0x62), 9, 3, 0),
    brand("中国银行", "", 2, rgb(0xb5, 0x12, 0x1b), rgb(0xff, 0xff, 0xff), rgb(0xb5, 0x12, 0x1b), 9, 3, 0),
    brand("交通银行", "", 2, rgb(0x00, 0x3b, 0x8e), rgb(0xff, 0xff, 0xff), rgb(0x00, 0x3b, 0x8e), 11, 3, 0),
    brand("美宜佳", "24小时便利店", 3, rgb(0xd9, 0x21, 0x1c), rgb(0xff, 0xff, 0xff), rgb(0xff, 0xc8, 0x00), 0, 0, 1),
    brand("全家", "FamilyMart", 3, rgb(0xff, 0xff, 0xff), rgb(0x16, 0x8a, 0x45), rgb(0x00, 0x9f, 0xe8), 21, 6, 0),
    brand("罗森", "LAWSON", 3, rgb(0x1a, 0x6f, 0xb7), rgb(0xff, 0xff, 0xff), rgb(0xff, 0xff, 0xff), 16, 3, 0),
    brand("7-Eleven", "", 3, rgb(0xff, 0xff, 0xff), rgb(0x00, 0x7a, 0x3d), rgb(0xf0, 0x7a, 0x1a), 20, 6, 0),
    brand("便利蜂", "", 1, rgb(0xff, 0xcc, 0x00), rgb(0x22, 0x22, 0x22), rgb(0x22, 0x22, 0x22), 13, 3, 0),
    brand("沙县小吃", "", 3, rgb(0xd8, 0x20, 0x1f), rgb(0xff, 0xe0, 0x4a), rgb(0xff, 0xff, 0xff), 0, 1, 1),
    brand("兰州拉面", "清真", 3, rgb(0x0c, 0x6b, 0x3a), rgb(0xff, 0xff, 0xff), rgb(0xf2, 0xc1, 0x4e), 17, 3, 0),
    brand("重庆麻辣烫", "", 3, rgb(0xc8, 0x16, 0x1d), rgb(0xff, 0xdd, 0x44), rgb(0xff, 0xff, 0xff), 0, 1, 1),
    brand("杨国福麻辣烫", "", 2, rgb(0xe8, 0x3a, 0x12), rgb(0xff, 0xff, 0xff), rgb(0xff, 0xdd, 0x44), 0, 0, 1),
    brand("黄焖鸡米饭", "", 3, rgb(0xff, 0xcf, 0x1a), rgb(0xc8, 0x16, 0x1d), rgb(0x2b, 0x2b, 0x2b), 17, 3, 1),
    brand("重庆小面", "", 1, rgb(0x2a, 0x2a, 0x2a), rgb(0xf2, 0xc1, 0x4e), rgb(0xc8, 0x16, 0x1d), 17, 3, 0),
    brand("老乡鸡", "", 3, rgb(0x1e, 0x4f, 0xa0), rgb(0xff, 0xff, 0xff), rgb(0xff, 0xd2, 0x3f), 0, 0, 1),
    brand("益禾堂", "", 1, rgb(0x8a, 0x1c, 0x1c), rgb(0xf3, 0xe5, 0xc8), rgb(0xf3, 0xe5, 0xc8), 22, 3, 3),
    brand("古茗", "", 1, rgb(0x2f, 0x8f, 0x4e), rgb(0xff, 0xff, 0xff), rgb(0xff, 0xff, 0xff), 22, 3, 0),
    brand("茶百道", "", 1, rgb(0xf4, 0xf1, 0xe6), rgb(0x1d, 0x1d, 0x1b), rgb(0x2f, 0x8f, 0x4e), 19, 3, 0),
    brand("老百姓大药房", "", 3, rgb(0x00, 0x9a, 0x4a), rgb(0xff, 0xff, 0xff), rgb(0xff, 0xff, 0xff), 12, 3, 0),
    brand("一心堂", "药房", 3, rgb(0x0a, 0x8f, 0x4f), rgb(0xff, 0xff, 0xff), rgb(0xe0, 0x30, 0x30), 12, 3, 0),
    brand("海澜之家", "HLA", 2, rgb(0x0d, 0x2a, 0x5b), rgb(0xff, 0xff, 0xff), rgb(0xc9, 0xa2, 0x5c), 23, 1, 2),
    brand("李宁", "LI-NING", 2, rgb(0xd7, 0x1a, 0x21), rgb(0xff, 0xff, 0xff), rgb(0xff, 0xff, 0xff), 15, 3, 0),
    brand("安踏", "ANTA", 2, rgb(0xc8, 0x10, 0x2e), rgb(0xff, 0xff, 0xff), rgb(0x20, 0x20, 0x20), 14, 3, 0),
    brand("周大福", "", 2, rgb(0x7a, 0x0f, 0x1b), rgb(0xe6, 0xc2, 0x6a), rgb(0xe6, 0xc2, 0x6a), 23, 1, 2),
    brand("顺丰速运", "", 2, rgb(0xf4, 0xf4, 0xf0), rgb(0x1a, 0x1a, 0x1a), rgb(0xd0, 0x1f, 0x26), 13, 3, 0),
    brand("永辉超市", "", 2, rgb(0x2f, 0x9a, 0x3a), rgb(0xff, 0xff, 0xff), rgb(0xff, 0xdd, 0x00), 19, 3, 0),
    brand("屈臣氏", "", 2, rgb(0x00, 0xa6, 0xa6), rgb(0xff, 0xff, 0xff), rgb(0xff, 0xff, 0xff), 0, 1, 0),
    brand("中国体育彩票", "", 1, rgb(0x1b, 0x5f, 0xb0), rgb(0xff, 0xff, 0xff), rgb(0xff, 0xd2, 0x3f), 0, 0, 1),
    brand("中国福利彩票", "", 1, rgb(0xd2, 0x1f, 0x26), rgb(0xff, 0xf0, 0xb0), rgb(0xff, 0xf0, 0xb0), 0, 0, 1),
];

/// Palettes of local shops: ground, lettering, accent, style.
#[rustfmt::skip]
const PALETTES: &[(u32, u32, u32, u8)] = &[
    (rgb(0xc8, 0x16, 0x1d), rgb(0xf8, 0xf2, 0xe0), rgb(0xf2, 0xc1, 0x4e), 1),
    (rgb(0xf4, 0xf1, 0xe8), rgb(0xc8, 0x16, 0x1d), rgb(0x1f, 0x3c, 0x88), 1),
    (rgb(0x1b, 0x3c, 0x8c), rgb(0xff, 0xd2, 0x3f), rgb(0xf4, 0xf4, 0xf4), 0),
    (rgb(0x0e, 0x6b, 0x3c), rgb(0xf4, 0xf4, 0xec), rgb(0xf2, 0xc1, 0x4e), 1),
    (rgb(0xf6, 0xc2, 0x1c), rgb(0xb3, 0x16, 0x1b), rgb(0x22, 0x22, 0x22), 0),
    (rgb(0x1c, 0x1c, 0x1e), rgb(0xd8, 0xb4, 0x5a), rgb(0x7a, 0x5a, 0x18), 5),
    (rgb(0xe8, 0x65, 0x0f), rgb(0xff, 0xff, 0xff), rgb(0xff, 0xe0, 0x8a), 0),
    (rgb(0x22, 0x30, 0x4a), rgb(0xff, 0xff, 0xff), rgb(0xe0, 0x4a, 0x3a), 2),
    (rgb(0xf2, 0xf4, 0xf6), rgb(0x1b, 0x4f, 0x9c), rgb(0xc8, 0x16, 0x1d), 1),
    (rgb(0xec, 0xe8, 0xdc), rgb(0xd4, 0x20, 0x2a), rgb(0x2b, 0x2b, 0x2b), 2),
    (rgb(0x5b, 0x2e, 0x7c), rgb(0xf5, 0xeb, 0xd0), rgb(0xf2, 0xc1, 0x4e), 0),
    (rgb(0x0f, 0x7c, 0x80), rgb(0xff, 0xff, 0xff), rgb(0xff, 0xd2, 0x3f), 1),
    (rgb(0x2b, 0x2b, 0x2b), rgb(0xff, 0xf2, 0x9a), rgb(0xe0, 0x4a, 0x3a), 2),
    (rgb(0x8a, 0x1c, 0x1c), rgb(0xf3, 0xe5, 0xc8), rgb(0xd8, 0xb4, 0x5a), 1),
];

struct Category {
    name: &'static str,
    kind: Kind,
    upper: bool,
    /// Palette indices that suit the trade.
    palettes: &'static [usize],
    subs: &'static [&'static str],
}

#[rustfmt::skip]
const CATEGORIES: &[Category] = &[
    Category { name: "重庆火锅", kind: Kind::Generic, upper: true, palettes: &[0, 4, 5, 12], subs: &["麻辣鲜香 地道川味", "毛肚 鸭血 脑花", "聚会宴请首选"] },
    Category { name: "火锅", kind: Kind::Generic, upper: true, palettes: &[0, 5, 13], subs: &["鲜切牛羊肉", "自助小火锅"] },
    Category { name: "烧烤", kind: Kind::Generic, upper: true, palettes: &[5, 12, 0, 6], subs: &["夜宵 啤酒 烤串", "炭火现烤"] },
    Category { name: "串串香", kind: Kind::Generic, upper: false, palettes: &[0, 4, 6], subs: &["成都风味"] },
    Category { name: "水果超市", kind: Kind::Generic, upper: false, palettes: &[3, 4, 11], subs: &["新鲜水果 当季直供", "果切 果篮"] },
    Category { name: "鲜果", kind: Kind::Generic, upper: false, palettes: &[3, 6, 11], subs: &["每日新鲜"] },
    Category { name: "手机维修", kind: Kind::Generic, upper: false, palettes: &[2, 7, 8], subs: &["换屏 换电池 贴膜", "苹果 华为 小米"] },
    Category { name: "打印复印", kind: Kind::Generic, upper: false, palettes: &[2, 8, 7], subs: &["打印 复印 扫描 装订", "名片 彩印"] },
    Category { name: "五金建材", kind: Kind::Generic, upper: false, palettes: &[2, 4, 12], subs: &["水电 五金 工具", "各类建材批发"] },
    Category { name: "药房", kind: Kind::Generic, upper: false, palettes: &[3, 8, 11], subs: &["医保定点", "24小时营业"] },
    Category { name: "大药房", kind: Kind::Generic, upper: false, palettes: &[3, 8], subs: &["医保定点 连锁经营"] },
    Category { name: "小吃", kind: Kind::Generic, upper: false, palettes: &[0, 4, 6, 9], subs: &["特色风味", "早点 夜宵"] },
    Category { name: "面馆", kind: Kind::Generic, upper: false, palettes: &[0, 4, 6, 9, 13], subs: &["手工面 现做现煮", "牛肉面 炸酱面"] },
    Category { name: "米线", kind: Kind::Generic, upper: false, palettes: &[0, 6, 9], subs: &["过桥米线"] },
    Category { name: "包子铺", kind: Kind::Generic, upper: false, palettes: &[0, 4, 9], subs: &["现蒸包子 豆浆", "早餐"] },
    Category { name: "饺子馆", kind: Kind::Generic, upper: false, palettes: &[0, 9, 13], subs: &["手工水饺"] },
    Category { name: "烟酒", kind: Kind::Generic, upper: false, palettes: &[5, 13, 7], subs: &["名烟名酒 礼品", "中国福利彩票"] },
    Category { name: "便利店", kind: Kind::Generic, upper: false, palettes: &[2, 6, 3, 11], subs: &["24小时营业", "饮料 零食 日用"] },
    Category { name: "超市", kind: Kind::Generic, upper: false, palettes: &[0, 2, 3], subs: &["日用百货 粮油副食"] },
    Category { name: "干洗店", kind: Kind::Generic, upper: false, palettes: &[2, 8, 11], subs: &["干洗 水洗 熨烫"] },
    Category { name: "眼镜", kind: Kind::Generic, upper: false, palettes: &[2, 8, 7], subs: &["验光配镜"] },
    Category { name: "服饰", kind: Kind::Generic, upper: false, palettes: &[7, 9, 10, 5], subs: &["时尚女装", "男女装 童装"] },
    Category { name: "鲜花", kind: Kind::Generic, upper: false, palettes: &[10, 11, 3], subs: &["鲜花 绿植 花束"] },
    Category { name: "宠物", kind: Kind::Generic, upper: false, palettes: &[11, 6, 2], subs: &["宠物美容 寄养"] },
    Category { name: "足浴", kind: Kind::Generic, upper: true, palettes: &[5, 12, 10], subs: &["足疗 推拿 按摩"] },
    Category { name: "棋牌", kind: Kind::Generic, upper: true, palettes: &[5, 12, 13], subs: &["麻将 扑克 茶水"] },
    Category { name: "网咖", kind: Kind::Generic, upper: true, palettes: &[12, 7, 5], subs: &["电竞 包间"] },
    Category { name: "健身", kind: Kind::Generic, upper: true, palettes: &[7, 12, 6], subs: &["私教 团课 器械"] },
    Category { name: "奶茶", kind: Kind::Generic, upper: false, palettes: &[11, 6, 3, 10], subs: &["鲜果茶 奶盖"] },
    Category { name: "蛋糕", kind: Kind::Generic, upper: false, palettes: &[10, 11, 9], subs: &["生日蛋糕 面包"] },
    Category { name: "家电", kind: Kind::Generic, upper: false, palettes: &[2, 0, 7], subs: &["家电维修 销售"] },
    Category { name: "房产", kind: Kind::Generic, upper: false, palettes: &[0, 2, 4], subs: &["买卖 租赁 中介"] },
    Category { name: "粮油", kind: Kind::Generic, upper: false, palettes: &[4, 3, 0], subs: &["米面粮油 调味"] },
    Category { name: "理发店", kind: Kind::Barber, upper: false, palettes: &[1, 8, 2], subs: &["剪发 烫染 护理", "洗剪吹 15元起"] },
    Category { name: "美发", kind: Kind::Barber, upper: false, palettes: &[1, 8, 7, 5], subs: &["烫发 染发 造型", "专业护理"] },
    Category { name: "发艺", kind: Kind::Barber, upper: false, palettes: &[8, 7, 5], subs: &["洗剪吹 烫染护"] },
    Category { name: "造型", kind: Kind::Barber, upper: false, palettes: &[7, 5, 12], subs: &["Hair Salon"] },
];

const SURNAMES: &[&str] = &[
    "王", "李", "张", "刘", "陈", "杨", "黄", "赵", "周", "吴", "徐", "孙", "马", "胡", "郭", "林",
    "何", "高", "梁", "郑", "谢", "韩", "唐", "冯", "曾", "彭", "罗", "肖",
];
const PLACES: &[&str] = &[
    "巴蜀", "江南", "老街", "天府", "秦川", "金陵", "岭南", "鹭岛", "汉口", "洛阳", "东北", "湘西",
    "闽南", "关中", "塞北", "西域", "蓉城", "山城", "滨江", "新桥",
];
const LUCKY: &[&str] = &[
    "福", "鑫", "康", "乐", "旺", "顺", "佳", "美", "华", "新", "盛", "宏", "嘉", "祥", "瑞", "兴",
    "达",
];
const ROADS: &[&str] = &[
    "解放路",
    "人民路",
    "建设路",
    "中山路",
    "和平路",
    "新华路",
    "团结路",
    "文化路",
    "光明路",
];

const COMPANY_HEADS: &[&str] = &[
    "华信", "鼎盛", "云帆", "中联", "星河", "恒远", "瑞博", "创美", "汇智", "天成", "嘉禾", "腾飞",
    "锐科", "宏图", "东方", "长江", "华夏", "滨海", "蓉城", "申江", "泰和", "明珠", "海川", "博瑞",
];
const COMPANY_TAILS: &[(&str, bool)] = &[
    ("科技", true),
    ("软件", true),
    ("资本", false),
    ("集团", false),
    ("国际", false),
    ("传媒", false),
    ("控股", false),
    ("置业", false),
    ("证券", false),
    ("能源", false),
    ("医药", false),
    ("银行大厦", false),
    ("金融中心", false),
    ("广场", false),
];

const KEY: u32 = 0x51_67_4e_d3;

fn pick_index(key: u32, a: i32, b: i32, salt: u32, len: usize) -> usize {
    ((avalanche01(key, a, b, salt) * len as f32) as usize).min(len - 1)
}

fn local_name(i: i32, cat: &Category, max_chars: usize) -> (String, String) {
    let k = KEY;
    let cname = cat.name;
    let chars = |s: &str| s.chars().count();
    // Place + trade, surname + 记, 老 + surname, lucky pair, number branch.
    for attempt in 0..8 {
        let pattern = pick_index(k, i, attempt, 701, 5);
        let sn = SURNAMES[pick_index(k, i, attempt, 703, SURNAMES.len())];
        let pl = PLACES[pick_index(k, i, attempt, 705, PLACES.len())];
        let l1 = LUCKY[pick_index(k, i, attempt, 707, LUCKY.len())];
        let l2 = LUCKY[pick_index(k, i, attempt, 709, LUCKY.len())];
        let name = match pattern {
            0 => format!("{pl}{cname}"),
            1 => format!("{sn}记{cname}"),
            2 => format!("老{sn}{cname}"),
            3 => format!("{l1}{l2}{cname}"),
            _ => format!("{sn}{l1}{cname}"),
        };
        if chars(&name) <= max_chars {
            let sub_pool = cat.subs;
            let mut sub = sub_pool[pick_index(k, i, attempt, 711, sub_pool.len())].to_owned();
            // Some shops state a branch number or a road instead of a trade line.
            let extra = avalanche01(k, i, attempt, 713);
            if extra < 0.15 {
                let road = ROADS[pick_index(k, i, attempt, 715, ROADS.len())];
                sub = format!("{road}{}号", 3 + pick_index(k, i, attempt, 717, 180));
            } else if extra < 0.28 {
                sub = format!("第{}分店", 2 + pick_index(k, i, attempt, 719, 18));
            } else if extra < 0.40 {
                let tail = pick_index(k, i, attempt, 721, 10_000);
                sub = format!(
                    "TEL 1{}{} {tail:04}",
                    3 + pick_index(k, i, attempt, 723, 6),
                    100 + pick_index(k, i, attempt, 725, 899)
                );
            }
            return (name, sub);
        }
    }
    (cname.to_owned(), cat.subs[0].to_owned())
}

fn build() -> Catalogue {
    let mut boards: Vec<Board> = Vec::new();
    // (bays, board) pushed in two passes so wide boards sit on even cells.
    let mut singles: Vec<Board> = Vec::new();
    let mut wides: Vec<Board> = Vec::new();
    let mut push = |board: Board| {
        if board.bays == 1 {
            singles.push(board);
        } else {
            wides.push(board);
        }
    };
    for def in BRANDS {
        let widths: &[u8] = match def.bays {
            1 => &[1],
            2 => &[2],
            _ => &[1, 2],
        };
        for &bays in widths {
            // A bank or a big chain is a two-bay unit; their long names are
            // set condensed on a single bay when that is all there is.
            push(Board {
                cell: 0,
                bays,
                kind: Kind::Brand,
                upper: def.upper,
                text: def.name.to_owned(),
                sub: def.sub.to_owned(),
                style: def.style,
                font: def.font,
                colours: [def.bg, def.fg, def.acc],
                mark: def.mark,
            });
        }
    }
    // Local shops: 40 single-bay and 16 two-bay designs.
    for (count, bays, max_chars) in [(40_i32, 1_u8, 6_usize), (16, 2, 9)] {
        for n in 0..count {
            let i = n + if bays == 2 { 1000 } else { 0 };
            // Barbers are rare: two single-bay and one two-bay design, the
            // rest of the local shops draw from the other trades.
            let barbers = CATEGORIES.len() - 4;
            let cat = if (bays == 1 && (n == 3 || n == 21)) || (bays == 2 && n == 5) {
                &CATEGORIES[barbers + pick_index(KEY, i, 0, 601, 4)]
            } else {
                &CATEGORIES[pick_index(KEY, i, 0, 601, barbers)]
            };
            let (text, sub) = local_name(i, cat, max_chars);
            let pal = PALETTES[cat.palettes[pick_index(KEY, i, 0, 603, cat.palettes.len())]];
            let font = [0_u8, 0, 1, 2, 3, 4][pick_index(KEY, i, 0, 605, 6)];
            // Local shops: light box and 3D-character fascias dominate.
            let mut style = pal.3;
            if matches!(cat.kind, Kind::Barber) && style > 2 {
                style = 1;
            }
            let with_sub = avalanche01(KEY, i, 0, 607) < 0.7 || style == 2;
            push(Board {
                cell: 0,
                bays,
                kind: cat.kind,
                upper: cat.upper,
                text,
                sub: if with_sub { sub } else { String::new() },
                style,
                font,
                colours: [pal.0, pal.1, pal.2],
                mark: 0,
            });
        }
    }
    // Layout: singles first, then the wides on even cells.
    let mut cell = 0;
    for mut b in singles {
        b.cell = cell;
        cell += 1;
        boards.push(b);
    }
    if cell % 2 == 1 {
        cell += 1;
    }
    for mut b in wides {
        b.cell = cell;
        cell += 2;
        boards.push(b);
    }
    let board_rows = cell.div_ceil(BOARD_COLS);

    // Hanging vertical signs: names of up to four characters on a ground colour.
    let mut verts = Vec::new();
    for n in 0..20_i32 {
        let cat = &CATEGORIES[pick_index(KEY, n, 1, 801, CATEGORIES.len())];
        let (text, _) = local_name(5000 + n, cat, 4);
        let pal = PALETTES[cat.palettes[pick_index(KEY, n, 1, 803, cat.palettes.len())]];
        verts.push(Vert {
            cell: n as usize,
            text,
            style: pal.3.min(2),
            font: [0_u8, 1, 2, 3][pick_index(KEY, n, 1, 805, 4)],
            colours: [pal.0, pal.1, pal.2],
        });
    }
    let vert_rows = verts.len().div_ceil(VERT_COLS);

    // Company lettering for towers.
    let mut crowns = Vec::new();
    for n in 0..28_i32 {
        let head = COMPANY_HEADS[pick_index(KEY, n, 2, 901, COMPANY_HEADS.len())];
        let (tail, logo_ok) = COMPANY_TAILS[pick_index(KEY, n, 2, 903, COMPANY_TAILS.len())];
        let name = format!("{head}{tail}");
        let colour = [
            rgb(0xf2, 0xf2, 0xee), // white
            rgb(0xe3, 0xbf, 0x6a), // gold
            rgb(0xb4, 0xbd, 0xc6), // steel
            rgb(0xf2, 0xf2, 0xee),
            rgb(0xe3, 0xbf, 0x6a),
        ][pick_index(KEY, n, 2, 905, 5)];
        let logo = if logo_ok || avalanche01(KEY, n, 2, 907) < 0.35 {
            1 + pick_index(KEY, n, 2, 909, 6) as u8
        } else {
            0
        };
        crowns.push(Crown {
            cell: n as usize,
            name,
            style: pick_index(KEY, n, 2, 911, 2) as u8,
            font: [0_u8, 1, 4][pick_index(KEY, n, 2, 913, 3)],
            colour,
            mark: logo,
        });
    }
    Catalogue {
        boards,
        verts,
        crowns,
        board_rows,
        vert_rows,
    }
}

pub fn catalogue() -> &'static Catalogue {
    static CATALOGUE: OnceLock<Catalogue> = OnceLock::new();
    CATALOGUE.get_or_init(build)
}

fn hex(c: u32) -> String {
    format!("{c:06x}")
}

/// The catalogue as text for the renderer: sheet geometry first, then one
/// tab-separated line per design.
pub fn manifest_text() -> String {
    let cat = catalogue();
    let mut out = String::new();
    let _ = writeln!(
        out,
        "SHEET\tboard\t{}\t{}\t{}\t{}",
        BOARD_COLS, cat.board_rows, BOARD_CELL.0, BOARD_CELL.1
    );
    let _ = writeln!(
        out,
        "SHEET\tvert\t{}\t{}\t{}\t{}",
        VERT_COLS, cat.vert_rows, VERT_CELL.0, VERT_CELL.1
    );
    let _ = writeln!(
        out,
        "SHEET\tcrown\t1\t{}\t{}\t{}",
        cat.crowns.len(),
        CROWN_CELL.0,
        CROWN_CELL.1
    );
    for b in &cat.boards {
        let _ = writeln!(
            out,
            "B\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            b.cell,
            b.bays,
            b.style,
            b.font,
            hex(b.colours[0]),
            hex(b.colours[1]),
            hex(b.colours[2]),
            b.mark,
            b.text,
            b.sub
        );
    }
    for v in &cat.verts {
        let _ = writeln!(
            out,
            "V\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            v.cell,
            v.style,
            v.font,
            hex(v.colours[0]),
            hex(v.colours[1]),
            hex(v.colours[2]),
            v.text
        );
    }
    for c in &cat.crowns {
        let _ = writeln!(
            out,
            "C\t{}\t{}\t{}\t{}\t{}\t{}",
            c.cell,
            c.style,
            c.font,
            hex(c.colour),
            c.mark,
            c.name
        );
    }
    out
}

/// The manifest as a baked texture (UTF-8 bytes in the RGBA payload).
pub fn manifest_texture() -> BakedTexture {
    let mut bytes = manifest_text().into_bytes();
    let texels = bytes.len().div_ceil(4);
    bytes.resize(texels * 4, 0);
    BakedTexture {
        name: MANIFEST.to_owned(),
        width: texels,
        height: 1,
        tile_width_m: 1.0,
        tile_height_m: 1.0,
        has_normal_source: false,
        rgba: bytes,
    }
}

/// UV rectangle `(u0, v0, u1, v1)` of a board in the sheet, `v` counting down,
/// inset by two texels so filtering never reaches the neighbouring design.
pub fn board_uv(b: &Board) -> (f32, f32, f32, f32) {
    let rows = catalogue().board_rows as f32;
    let (cw, ch) = (BOARD_CELL.0 as f32, BOARD_CELL.1 as f32);
    let col = (b.cell % BOARD_COLS) as f32;
    let row = (b.cell / BOARD_COLS) as f32;
    let span = f32::from(b.bays);
    let sheet_w = BOARD_COLS as f32 * cw;
    let sheet_h = rows * ch;
    let inset = 2.0;
    (
        (col * cw + inset) / sheet_w,
        (row * ch + inset) / sheet_h,
        ((col + span) * cw - inset) / sheet_w,
        ((row + 1.0) * ch - inset) / sheet_h,
    )
}

pub fn vert_uv(v: &Vert) -> (f32, f32, f32, f32) {
    let rows = catalogue().vert_rows as f32;
    let (cw, ch) = (VERT_CELL.0 as f32, VERT_CELL.1 as f32);
    let col = (v.cell % VERT_COLS) as f32;
    let row = (v.cell / VERT_COLS) as f32;
    let sheet_w = VERT_COLS as f32 * cw;
    let sheet_h = rows * ch;
    let inset = 2.0;
    (
        (col * cw + inset) / sheet_w,
        (row * ch + inset) / sheet_h,
        ((col + 1.0) * cw - inset) / sheet_w,
        ((row + 1.0) * ch - inset) / sheet_h,
    )
}

/// UV rectangle of a company name's drawn block (the used part of its cell).
pub fn crown_uv(c: &Crown) -> (f32, f32, f32, f32) {
    let n = catalogue().crowns.len() as f32;
    let (cw, ch) = (CROWN_CELL.0 as f32, CROWN_CELL.1 as f32);
    let used = c.used_px() as f32;
    let x0 = (cw - used) * 0.5;
    let y0 = c.cell as f32 * ch;
    (
        x0 / cw,
        (y0 + 1.0) / (n * ch),
        (x0 + used) / cw,
        (y0 + ch - 1.0) / (n * ch),
    )
}

/// Physical width of a company name at letter-cell height `h` metres.
pub fn crown_width_m(c: &Crown, h: f32) -> f32 {
    c.used_px() as f32 / CROWN_CELL.1 as f32 * h
}

/// The design standing on a shop: `bays` wide (1 or 2), chosen by `key` (a hash
/// of the shop's position), and from the second-storey list when `upper`.
pub fn pick_board(bays: usize, key: u32, upper: bool) -> &'static Board {
    let cat = catalogue();
    let want = if bays >= 2 { 2 } else { 1 };
    let pool: Vec<&Board> = cat
        .boards
        .iter()
        .filter(|b| b.bays == want && (!upper || b.upper))
        .collect();
    let roll = avalanche01(key, 0, 0, 1);
    // Brands are about a third of the frontage on a main street.
    let brands: Vec<&&Board> = pool.iter().filter(|b| b.kind == Kind::Brand).collect();
    let locals: Vec<&&Board> = pool.iter().filter(|b| b.kind != Kind::Brand).collect();
    let pick = avalanche01(key, 1, 0, 2);
    if !upper && !brands.is_empty() && roll < 0.30 {
        return brands[((pick * brands.len() as f32) as usize).min(brands.len() - 1)];
    }
    let list = if locals.is_empty() { &brands } else { &locals };
    list[((pick * list.len() as f32) as usize).min(list.len() - 1)]
}

pub fn pick_vert(key: u32) -> &'static Vert {
    let v = &catalogue().verts;
    &v[((avalanche01(key, 2, 0, 3) * v.len() as f32) as usize).min(v.len() - 1)]
}

pub fn pick_crown(key: u32) -> &'static Crown {
    let v = &catalogue().crowns;
    &v[((avalanche01(key, 3, 0, 4) * v.len() as f32) as usize).min(v.len() - 1)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_catalogue_has_brands_locals_barbers_and_company_names() {
        let c = catalogue();
        assert!(
            c.boards
                .iter()
                .any(|b| b.kind == Kind::Brand && b.text == "蜜雪冰城")
        );
        assert!(
            c.boards
                .iter()
                .any(|b| b.kind == Kind::Barber && b.bays == 1)
        );
        assert!(c.boards.iter().any(|b| b.kind == Kind::Generic && b.upper));
        assert!(c.crowns.len() >= 20);
        for b in &c.boards {
            let max = if b.bays == 1 { 8 } else { 10 };
            assert!(b.text.chars().count() <= max, "{} too long", b.text);
        }
        for cr in &c.crowns {
            assert!(cr.used_px() <= CROWN_CELL.0, "{} too wide", cr.name);
        }
    }

    #[test]
    fn wide_boards_sit_on_even_cells_inside_one_row() {
        for b in &catalogue().boards {
            if b.bays == 2 {
                assert_eq!(b.cell % 2, 0);
                assert_eq!(b.cell / BOARD_COLS, (b.cell + 1) / BOARD_COLS);
            }
        }
    }

    #[test]
    fn the_manifest_survives_a_round_trip_through_the_texture() {
        let t = manifest_texture();
        let end = t.rgba.iter().rposition(|&b| b != 0).unwrap() + 1;
        assert_eq!(
            std::str::from_utf8(&t.rgba[..end]).unwrap(),
            manifest_text().trim_end_matches('\0')
        );
    }

    #[test]
    fn picks_are_deterministic_and_respect_width_and_floor() {
        for key in 0..200_u32 {
            assert_eq!(
                pick_board(1, key, false).cell,
                pick_board(1, key, false).cell
            );
            assert_eq!(pick_board(2, key, false).bays, 2);
            assert!(pick_board(1, key, true).upper);
        }
    }
}
