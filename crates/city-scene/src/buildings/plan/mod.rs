//! The floor plan: the one data structure a building's facade, interior and
//! massing are read from.  See `docs/building-design.md`.
//!
//! A [`Plan`] is a typical floor drawn in the plate frame (x east, y north, the
//! south face on `y = 0`) from seeded design rules: a typology picks the plate
//! and the core, the units are subdivided into rooms on the structural bay
//! module, doors follow connectivity, and openings follow room function.  The
//! facade of a residential block is then `plan.openings_on(side)`.
//!
//! Generation is generate-and-test: a draw that breaks a code rule (room area,
//! daylight, reachability) is discarded and redrawn from the next sub-seed, so a
//! plan that comes out always satisfies [`Rules`].  How often a draw fails is a
//! reported statistic (`Plan::attempts`).

mod geom;
mod office;
mod residential;
mod unit;

pub use geom::{Rect, Side, Xf};

use crate::math::Rng;

/// What kind of floor this is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Typology {
    /// 板式 一梯两户: a slab, two mirrored units each side of a central core.
    SlabTwoUnit,
    /// 点式塔楼 一梯四户: four corner units in a pinwheel round a central core.
    TowerFourUnit,
    /// 单元楼: a low walk-up, two flats off one stair, no lift.
    WalkUp,
    /// An office floor: core, curtain wall, open plan.
    Office,
}

/// The building era of a development (the design culture, not a style).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlanEra {
    /// 苏式, 1950s-60s.
    Soviet,
    /// 80s-90s brick-and-concrete 单元楼.
    Estate,
    /// 1996-2010 commercial housing.
    Millennium,
    /// 2010 onwards.
    Contemporary,
}

/// Climate class from latitude: it decides window caps and balcony glazing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Climate {
    /// Severe cold and cold zones (north of about 36 N).
    Cold,
    /// Hot summer, cold winter (about 28-36 N).
    Transitional,
    /// Hot summer, warm winter (south of 28 N).
    Warm,
}

impl Climate {
    pub fn of_latitude(lat: f32) -> Climate {
        if lat >= 36.0 {
            Climate::Cold
        } else if lat >= 28.0 {
            Climate::Transitional
        } else {
            Climate::Warm
        }
    }
}

/// What a room is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Func {
    Living,
    Dining,
    MasterBed,
    Bed,
    Study,
    Kitchen,
    Bath,
    Wc,
    Hall,
    Store,
    Balcony,
    ServiceBalcony,
    Stair,
    Lift,
    Lobby,
    Shaft,
    /// Open-plan lettable office floor.
    Open,
    /// Office toilets.
    Toilets,
}

impl Func {
    /// Rooms the code requires direct daylight for (GB 50096 7.1.3).
    pub fn needs_daylight(self) -> bool {
        matches!(
            self,
            Func::Living | Func::MasterBed | Func::Bed | Func::Study | Func::Kitchen
        )
    }

    pub fn is_bedroom(self) -> bool {
        matches!(self, Func::MasterBed | Func::Bed | Func::Study)
    }

    pub fn is_balcony(self) -> bool {
        matches!(self, Func::Balcony | Func::ServiceBalcony)
    }

    fn code(self) -> u32 {
        self as u32
    }
}

/// Half a wall (centre line to finished face) on every side: partitions are
/// 100-120 mm, structural walls 240 mm.
const HALF_WALL: f32 = 0.1;

#[derive(Debug, Clone)]
pub struct Room {
    pub func: Func,
    pub rect: Rect,
    /// The dwelling this room belongs to; `None` for the core and for offices.
    pub unit: Option<usize>,
    /// No window: a bathroom or hall lit from elsewhere.
    pub dark: bool,
    /// Balconies only: glazed in by the owner or the design.
    pub enclosed: bool,
}

impl Room {
    /// Floor area inside the finished faces.
    pub fn net_area(&self) -> f32 {
        ((self.rect.w() - 2.0 * HALF_WALL).max(0.0)) * ((self.rect.d() - 2.0 * HALF_WALL).max(0.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoorKind {
    Entry,
    Internal,
    Balcony,
    Core,
}

#[derive(Debug, Clone)]
pub struct Door {
    pub a: usize,
    pub b: usize,
    pub at: [f32; 2],
    pub width: f32,
    /// The wall the door is in runs along y.
    pub vertical_wall: bool,
    pub kind: DoorKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpeningKind {
    Window,
    /// A full-height glazed door onto a balcony.
    BalconyDoor,
    /// A projecting bay window (sill low, glazed sides).
    Bay,
    /// A stair-landing window.  Sill and head are above the storey's floor and
    /// may pass the next floor line: the stair has no floor slab.
    StairWindow,
    /// A louvred air-conditioner platform (空调位) beside a window.
    AcLedge,
    /// A curtain-wall bay of `module`-wide panes.
    Curtain {
        module: f32,
    },
    /// A strip window between spandrels.
    Ribbon,
}

/// A hole or fitting in a face, in the plate frame.  `along` is the x (south,
/// north faces) or y (east, west faces) coordinate of the centre; `sill` and
/// `head` are metres above the storey's floor.
#[derive(Debug, Clone)]
pub struct Opening {
    pub side: Side,
    pub along: f32,
    pub width: f32,
    pub sill: f32,
    pub head: f32,
    pub kind: OpeningKind,
    pub room: usize,
}

impl Opening {
    /// Glazed area for daylight: nothing below 0.5 m counts (GB 50096 7.1.7).
    pub fn glazed_area(&self) -> f32 {
        match self.kind {
            OpeningKind::AcLedge => 0.0,
            _ => self.width * (self.head - self.sill.max(0.5)).max(0.0),
        }
    }

    fn counts_for_daylight(&self) -> bool {
        matches!(
            self.kind,
            OpeningKind::Window | OpeningKind::BalconyDoor | OpeningKind::Bay
        )
    }
}

#[derive(Debug, Clone)]
pub struct UnitInfo {
    pub bedrooms: u8,
    /// Sum of the rooms inside the plate, balconies excluded (套内使用面积).
    pub use_area: f32,
    pub rooms: Vec<usize>,
    /// Index into `Plan::doors`.
    pub entry_door: usize,
}

#[derive(Debug, Clone, Default)]
pub struct CoreInfo {
    /// Stair rectangles (a scissor pair counts as two).
    pub stairs: Vec<Rect>,
    pub lifts: Vec<Rect>,
    /// Points where a stair opens onto the circulation: the evacuation targets.
    pub stair_doors: Vec<[f32; 2]>,
}

/// What a plan is drawn from.
#[derive(Debug, Clone, Copy)]
pub struct PlanSpec {
    pub typology: Typology,
    pub era: PlanEra,
    pub latitude: f32,
    pub storeys: u16,
    pub seed: u32,
}

#[derive(Debug, Clone)]
pub struct Plan {
    pub spec: PlanSpec,
    pub climate: Climate,
    pub plate: Rect,
    pub floor_height: f32,
    pub rooms: Vec<Room>,
    pub doors: Vec<Door>,
    pub openings: Vec<Opening>,
    pub units: Vec<UnitInfo>,
    pub core: CoreInfo,
    /// Redraws that were needed before this draw satisfied the rules.
    pub attempts: u32,
    /// Largest straight-line distance from a dwelling door (or any office
    /// point) to the nearest stair door.
    pub evac_max: f32,
    /// Deviation of the building axis from true south, degrees.
    pub azimuth_deg: f32,
}

/// The numeric limits a plan is checked against.  Each is a clause of the cited
/// standard; values marked [R] in `docs/building-design.md` are to be verified.
#[derive(Debug, Clone)]
pub struct Rules {
    pub min_double_bed: f32,
    pub min_single_bed: f32,
    pub min_living: f32,
    pub min_kitchen: f32,
    pub min_bath: f32,
    pub min_wc: f32,
    /// Window-to-floor ratio for habitable rooms.
    pub min_daylight_ratio: f32,
    /// Unit door to nearest stair door, residential (stricter of the two).
    pub max_evac_unit_door: f32,
    /// Any point of an open office floor to the nearest stair door.
    pub max_evac_office: f32,
    pub min_stair_flight: f32,
    pub min_entry_door: f32,
    /// Largest window-to-wall ratio by face: `[south, east, north, west]`.
    pub wwr_cap: [f32; 4],
}

impl Rules {
    pub fn for_climate(climate: Climate) -> Rules {
        let wwr_cap = match climate {
            Climate::Cold => [0.50, 0.35, 0.30, 0.35],
            Climate::Transitional => [0.50, 0.35, 0.40, 0.35],
            Climate::Warm => [0.55, 0.35, 0.40, 0.35],
        };
        Rules {
            min_double_bed: 9.0,
            min_single_bed: 5.0,
            min_living: 10.0,
            min_kitchen: 4.0,
            min_bath: 2.5,
            min_wc: 1.1,
            min_daylight_ratio: 1.0 / 7.0,
            max_evac_unit_door: 20.0,
            max_evac_office: 30.0,
            min_stair_flight: 1.1,
            min_entry_door: 0.9,
            wwr_cap,
        }
    }
}

/// Use-area window of a dwelling by bedroom count and era (`docs`, section 5).
pub fn unit_area_range(era: PlanEra, bedrooms: u8) -> (f32, f32) {
    match (era, bedrooms) {
        (PlanEra::Soviet, 1) => (28.0, 45.0),
        (PlanEra::Soviet, 2) => (45.0, 78.0),
        (PlanEra::Soviet, _) => (65.0, 100.0),
        (PlanEra::Estate, 1) => (30.0, 46.0),
        (PlanEra::Estate, 2) => (42.0, 68.0),
        (PlanEra::Estate, _) => (60.0, 85.0),
        (PlanEra::Millennium, 1) => (40.0, 58.0),
        (PlanEra::Millennium, 2) => (60.0, 95.0),
        (PlanEra::Millennium, 3) => (88.0, 132.0),
        (PlanEra::Millennium, _) => (110.0, 155.0),
        (PlanEra::Contemporary, 1) => (40.0, 62.0),
        (PlanEra::Contemporary, 2) => (65.0, 100.0),
        (PlanEra::Contemporary, 3) => (90.0, 142.0),
        (PlanEra::Contemporary, _) => (115.0, 172.0),
    }
}

impl Plan {
    /// Draw a plan.  Panics only if no draw within the attempt budget satisfies
    /// the rules, which the tests make a measured impossibility.
    pub fn generate(spec: PlanSpec) -> Plan {
        Plan::try_generate(spec).unwrap_or_else(|why| panic!("no valid plan for {spec:?}: {why:?}"))
    }

    /// Like [`Plan::generate`], returning the last draw's rule violations when
    /// the attempt budget is spent.
    pub fn try_generate(spec: PlanSpec) -> Result<Plan, Vec<String>> {
        let climate = Climate::of_latitude(spec.latitude);
        let rules = Rules::for_climate(climate);
        let mut last = Vec::new();
        for attempt in 0..400u32 {
            let mut rng = Rng::new(spec.seed);
            rng.fork(0x51ab_0000 ^ attempt.wrapping_mul(0x9e37));
            let drawn = match spec.typology {
                Typology::SlabTwoUnit | Typology::WalkUp => {
                    residential::slab(spec, climate, &mut rng)
                }
                Typology::TowerFourUnit => residential::tower(spec, climate, &mut rng),
                Typology::Office => office::office(spec, climate, &mut rng),
            };
            if let Some(mut plan) = drawn {
                plan.attempts = attempt;
                let issues = plan.check(&rules);
                if issues.is_empty() {
                    return Ok(plan);
                }
                last = issues;
            }
        }
        Err(last)
    }

    pub fn side_length(&self, side: Side) -> f32 {
        match side {
            Side::South | Side::North => self.plate.w(),
            Side::East | Side::West => self.plate.d(),
        }
    }

    /// Everything the facade of `side` is made of.
    pub fn openings_on(&self, side: Side) -> impl Iterator<Item = &Opening> {
        self.openings.iter().filter(move |o| o.side == side)
    }

    /// Balconies on a face: `(span along the face, depth)`.
    pub fn balconies_on(&self, side: Side) -> Vec<(f32, f32, f32, bool)> {
        let mut out = Vec::new();
        for r in self.rooms.iter().filter(|r| r.func.is_balcony()) {
            let (lo, hi, depth, s) = if r.rect.y1 <= self.plate.y0 + 1e-3 {
                (r.rect.x0, r.rect.x1, r.rect.d(), Side::South)
            } else if r.rect.y0 >= self.plate.y1 - 1e-3 {
                (r.rect.x0, r.rect.x1, r.rect.d(), Side::North)
            } else if r.rect.x0 >= self.plate.x1 - 1e-3 {
                (r.rect.y0, r.rect.y1, r.rect.w(), Side::East)
            } else {
                (r.rect.y0, r.rect.y1, r.rect.w(), Side::West)
            };
            if s == side {
                out.push((lo, hi, depth, r.enclosed));
            }
        }
        out
    }

    /// Glazed area over wall area on one face (balcony doors included).
    pub fn wwr(&self, side: Side) -> f32 {
        let glazed: f32 = self
            .openings_on(side)
            .filter(|o| !matches!(o.kind, OpeningKind::AcLedge))
            .map(|o| o.width * (o.head.min(self.floor_height) - o.sill.max(0.0)).max(0.0))
            .sum();
        glazed / (self.side_length(side) * self.floor_height)
    }

    /// A hash of the whole drawing, 5 cm resolution.
    pub fn signature(&self) -> u64 {
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        let mut mix = |v: i64| {
            h ^= v as u64;
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
            h ^= h >> 29;
        };
        let q = |v: f32| (v * 20.0).round() as i64;
        mix(self.spec.typology as i64);
        mix(self.spec.era as i64);
        mix(q(self.plate.w()));
        mix(q(self.plate.d()));
        for r in &self.rooms {
            mix(r.func.code() as i64);
            mix(q(r.rect.x0));
            mix(q(r.rect.y0));
            mix(q(r.rect.x1));
            mix(q(r.rect.y1));
        }
        for o in &self.openings {
            mix(o.side as i64);
            mix(q(o.along));
            mix(q(o.width));
            mix(q(o.sill));
            mix(q(o.head));
        }
        for d in &self.doors {
            mix(q(d.at[0]));
            mix(q(d.at[1]));
        }
        h
    }

    /// Every rule the plan breaks, as text; empty when it satisfies them.
    pub fn check(&self, rules: &Rules) -> Vec<String> {
        let mut bad = Vec::new();
        // Geometry: rooms tile without overlap, inside the plate unless balcony.
        for (i, a) in self.rooms.iter().enumerate() {
            if !a.func.is_balcony() && !self.plate.contains_rect(&a.rect) {
                bad.push(format!("room {i} {:?} leaves the plate", a.func));
            }
            for (j, b) in self.rooms.iter().enumerate().skip(i + 1) {
                if a.rect.overlap_area(&b.rect) > 0.01 {
                    bad.push(format!(
                        "rooms {i} {:?} and {j} {:?} overlap",
                        a.func, b.func
                    ));
                }
            }
        }
        // Room minima (GB 50096 5.2-5.4).
        for (i, r) in self.rooms.iter().enumerate() {
            let a = r.net_area();
            let min = match r.func {
                Func::MasterBed => rules.min_double_bed,
                Func::Bed | Func::Study => rules.min_single_bed,
                Func::Living => rules.min_living,
                Func::Kitchen => rules.min_kitchen,
                Func::Bath => rules.min_bath,
                Func::Wc => rules.min_wc,
                _ => 0.0,
            };
            if a < min {
                bad.push(format!("room {i} {:?} is {a:.1} m2, under {min}", r.func));
            }
            if r.func.needs_daylight() {
                let ratio = r.rect.w().max(r.rect.d()) / r.rect.w().min(r.rect.d());
                if ratio > 2.6 {
                    bad.push(format!("room {i} {:?} is {ratio:.1}:1", r.func));
                }
                let glass: f32 = self
                    .openings
                    .iter()
                    .filter(|o| o.room == i && o.counts_for_daylight())
                    .map(Opening::glazed_area)
                    .sum();
                if glass < a * rules.min_daylight_ratio {
                    bad.push(format!(
                        "room {i} {:?} has {glass:.2} m2 of glass for {a:.1} m2 floor",
                        r.func
                    ));
                }
            }
        }
        // Units: area window, entry door width.
        for (k, u) in self.units.iter().enumerate() {
            let (lo, hi) = unit_area_range(self.spec.era, u.bedrooms);
            if u.use_area < lo || u.use_area > hi {
                bad.push(format!(
                    "unit {k} ({} bed) use area {:.1} outside {lo}-{hi}",
                    u.bedrooms, u.use_area
                ));
            }
            if self.doors[u.entry_door].width < rules.min_entry_door - 1e-3 {
                bad.push(format!("unit {k} entry door too narrow"));
            }
        }
        // Reachability: every room from the stair by door graph.
        if !self.all_reachable() {
            bad.push("a room is not reachable from the stair".into());
        }
        // Evacuation.
        let limit = if self.spec.typology == Typology::Office {
            rules.max_evac_office
        } else {
            rules.max_evac_unit_door
        };
        if self.evac_max > limit {
            bad.push(format!(
                "evacuation distance {:.1} m over {limit}",
                self.evac_max
            ));
        }
        // Stair.
        for s in &self.core.stairs {
            if s.w().min(s.d()) < rules.min_stair_flight * 2.0 + 0.1 {
                bad.push("stair too narrow for two flights".into());
            }
        }
        // Lifts never beside a bedroom (GB 50096 6.4.7).
        for l in &self.core.lifts {
            for r in self.rooms.iter().filter(|r| r.func.is_bedroom()) {
                if l.shared(&r.rect).is_some() {
                    bad.push("lift adjoins a bedroom".into());
                }
            }
        }
        // Window-to-wall ratio caps.
        for (n, side) in Side::ALL.iter().enumerate() {
            let w = self.wwr(*side);
            if self.spec.typology != Typology::Office && w > rules.wwr_cap[n] {
                bad.push(format!(
                    "{side:?} window-to-wall ratio {w:.2} over {}",
                    rules.wwr_cap[n]
                ));
            }
        }
        // Openings stay on their face.
        for o in &self.openings {
            let len = self.side_length(o.side);
            let origin = match o.side {
                Side::South | Side::North => self.plate.x0,
                _ => self.plate.y0,
            };
            if o.along - o.width * 0.5 < origin - 1e-3
                || o.along + o.width * 0.5 > origin + len + 1e-3
            {
                bad.push(format!(
                    "opening {:?} runs off the {:?} face",
                    o.kind, o.side
                ));
            }
        }
        bad
    }

    fn all_reachable(&self) -> bool {
        let n = self.rooms.len();
        let mut seen = vec![false; n];
        let mut stack: Vec<usize> = (0..n)
            .filter(|&i| matches!(self.rooms[i].func, Func::Stair | Func::Lobby))
            .collect();
        if stack.is_empty() {
            return false;
        }
        for &s in &stack {
            seen[s] = true;
        }
        while let Some(i) = stack.pop() {
            for d in &self.doors {
                let other = if d.a == i {
                    d.b
                } else if d.b == i {
                    d.a
                } else {
                    continue;
                };
                if !seen[other] {
                    seen[other] = true;
                    stack.push(other);
                }
            }
        }
        // Machine and pipe shafts have no door by design.
        (0..n).all(|i| seen[i] || self.rooms[i].func == Func::Shaft)
    }
}

/// Pick one entry of a weighted table.
pub(crate) fn pick<T: Copy>(rng: &mut Rng, table: &[(T, f32)]) -> T {
    let total: f32 = table.iter().map(|(_, w)| w).sum();
    let mut r = rng.unit() * total;
    for (v, w) in table {
        if r < *w {
            return *v;
        }
        r -= w;
    }
    table[table.len() - 1].0
}

/// Round to the 300 mm structural module.
pub(crate) fn grid(v: f32) -> f32 {
    (v / 0.3).round() * 0.3
}

#[cfg(test)]
mod tests;
