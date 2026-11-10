//! Residential plates: the slab with two units to a core (also the old walk-up)
//! and the four-unit pinwheel tower.

use super::geom::{Rect, Side, Xf};
use super::unit::{self, Layout, NONE, era_rules};
use super::{
    Climate, CoreInfo, Door, DoorKind, Func, Opening, OpeningKind, Plan, PlanEra, PlanSpec, Room,
    Typology, UnitInfo, grid, pick,
};
use crate::math::Rng;

struct Builder {
    rooms: Vec<Room>,
    doors: Vec<Door>,
    openings: Vec<Opening>,
    units: Vec<UnitInfo>,
}

impl Builder {
    fn new() -> Self {
        Builder {
            rooms: Vec::new(),
            doors: Vec::new(),
            openings: Vec::new(),
            units: Vec::new(),
        }
    }

    fn room(&mut self, func: Func, rect: Rect, dark: bool) -> usize {
        self.rooms.push(Room {
            func,
            rect,
            unit: None,
            dark,
            enclosed: false,
        });
        self.rooms.len() - 1
    }

    fn door(
        &mut self,
        a: usize,
        b: usize,
        at: [f32; 2],
        width: f32,
        vertical_wall: bool,
        kind: DoorKind,
    ) -> usize {
        self.doors.push(Door {
            a,
            b,
            at,
            width,
            vertical_wall,
            kind,
        });
        self.doors.len() - 1
    }

    /// Put a unit drawn in its own frame into the plate.  `lobby` is the room
    /// its entry door opens onto.
    fn place(&mut self, layout: &Layout, xf: Xf, lobby: usize, bedrooms: u8) {
        let id = self.units.len();
        let base = self.rooms.len();
        let mut mine = Vec::new();
        let mut use_area = 0.0;
        for r in &layout.rooms {
            mine.push(self.rooms.len());
            if !r.func.is_balcony() {
                use_area += (r.rect.w() - 0.2).max(0.0) * (r.rect.d() - 0.2).max(0.0);
            }
            self.rooms.push(Room {
                func: r.func,
                rect: xf.rect(&r.rect),
                unit: Some(id),
                dark: r.dark,
                enclosed: r.enclosed,
            });
        }
        let mut entry = 0;
        for d in &layout.doors {
            let b = if d.b == NONE { lobby } else { base + d.b };
            let vertical = if xf.swaps_axes() {
                !d.vertical_wall
            } else {
                d.vertical_wall
            };
            let idx = self.door(base + d.a, b, xf.point(d.at), d.width, vertical, d.kind);
            if d.kind == DoorKind::Entry {
                entry = idx;
            }
        }
        for o in &layout.openings {
            let p = xf.point(o.at);
            let side = Side::from_normal(xf.dir(o.normal));
            let along = if matches!(side, Side::South | Side::North) {
                p[0]
            } else {
                p[1]
            };
            self.openings.push(Opening {
                side,
                along,
                width: o.width,
                sill: o.sill,
                head: o.head,
                kind: o.kind,
                room: base + o.room,
            });
        }
        self.units.push(UnitInfo {
            bedrooms,
            use_area,
            rooms: mine,
            entry_door: entry,
        });
    }
}

fn evac_max(b: &Builder, stair_doors: &[[f32; 2]]) -> f32 {
    b.units
        .iter()
        .map(|u| {
            let p = b.doors[u.entry_door].at;
            stair_doors
                .iter()
                .map(|s| ((p[0] - s[0]).powi(2) + (p[1] - s[1]).powi(2)).sqrt())
                .fold(f32::MAX, f32::min)
        })
        .fold(0.0, f32::max)
}

fn bedroom_count(rng: &mut Rng, era: PlanEra) -> u8 {
    match era {
        PlanEra::Soviet => pick(rng, &[(2u8, 0.65), (3, 0.35)]),
        PlanEra::Estate => pick(rng, &[(2u8, 0.6), (3, 0.4)]),
        PlanEra::Millennium => pick(rng, &[(2u8, 0.4), (3, 0.5), (4, 0.1)]),
        PlanEra::Contemporary => pick(rng, &[(2u8, 0.3), (3, 0.5), (4, 0.2)]),
    }
}

fn unit_width(rng: &mut Rng, era: PlanEra, bedrooms: u8) -> f32 {
    let (lo, hi) = match (era, bedrooms) {
        (PlanEra::Soviet, 2) => (6.0, 7.2),
        (PlanEra::Soviet, _) => (7.8, 9.0),
        (PlanEra::Estate, 2) => (5.7, 6.9),
        (PlanEra::Estate, _) => (6.9, 8.1),
        (PlanEra::Millennium, 2) => (7.2, 8.7),
        (PlanEra::Millennium, 3) => (8.7, 10.5),
        (PlanEra::Millennium, _) => (10.5, 12.0),
        (PlanEra::Contemporary, 2) => (7.5, 9.0),
        (PlanEra::Contemporary, 3) => (9.0, 10.8),
        (PlanEra::Contemporary, _) => (10.8, 12.6),
    };
    grid(rng.range(lo, hi))
}

fn depth_range(era: PlanEra) -> (f32, f32) {
    match era {
        PlanEra::Soviet => (10.8, 12.0),
        PlanEra::Estate => (9.9, 11.1),
        PlanEra::Millennium => (11.1, 12.9),
        PlanEra::Contemporary => (11.7, 13.5),
    }
}

pub(super) fn slab(spec: PlanSpec, climate: Climate, rng: &mut Rng) -> Option<Plan> {
    let er = era_rules(spec.era);
    let walk = spec.typology == Typology::WalkUp;
    let lifts = if walk {
        0
    } else if spec.storeys >= 12 {
        2
    } else {
        1
    };
    let d = grid({
        let (lo, hi) = depth_range(spec.era);
        rng.range(lo, hi)
    });
    let stair_w = if spec.era == PlanEra::Soviet {
        3.0
    } else {
        2.7
    };
    let lift_w = 2.1;
    let wc = stair_w + lift_w * lifts as f32;
    let dst = pick(rng, &[(4.8f32, 0.5), (5.1, 0.35), (5.4, 0.15)]);
    let hall = if lifts > 0 {
        pick(rng, &[(1.5f32, 0.6), (1.8, 0.4)])
    } else {
        pick(rng, &[(1.2f32, 0.5), (1.5, 0.5)])
    };
    let ds = d - dst - hall;
    if !(4.2..=6.6).contains(&ds) {
        return None;
    }
    let (nb_a, nb_b) = {
        let a = bedroom_count(rng, spec.era);
        let b = if rng.unit() < 0.7 {
            a
        } else {
            bedroom_count(rng, spec.era)
        };
        (a, b)
    };
    let wa = unit_width(rng, spec.era, nb_a);
    let wb = unit_width(rng, spec.era, nb_b);
    let w = wa + wc + wb;
    let hc = wc * 0.5;
    let yl1 = d - dst;
    let yl0 = ds;

    let la = unit::through_unit(
        rng,
        &er,
        climate,
        &unit::Through {
            bedrooms: nb_a,
            ws: wa + hc,
            wn: wa,
            d,
            ds,
            hall,
            west_gable: true,
        },
    )?;
    let lb = unit::through_unit(
        rng,
        &er,
        climate,
        &unit::Through {
            bedrooms: nb_b,
            ws: wb + hc,
            wn: wb,
            d,
            ds,
            hall,
            west_gable: true,
        },
    )?;

    let mut b = Builder::new();
    // Core.
    let lobby = b.room(Func::Lobby, Rect::new(wa, yl0, wa + wc, yl1), true);
    let stair = b.room(Func::Stair, Rect::new(wa, yl1, wa + stair_w, d), true);
    let stair_door = [wa + stair_w * 0.5, yl1];
    b.door(stair, lobby, stair_door, 1.0, false, DoorKind::Core);
    let mut core = CoreInfo {
        stairs: vec![b.rooms[stair].rect],
        lifts: Vec::new(),
        stair_doors: vec![stair_door],
    };
    for k in 0..lifts {
        let x = wa + stair_w + lift_w * k as f32;
        let car = b.room(Func::Lift, Rect::new(x, yl1, x + lift_w, yl1 + 2.4), true);
        b.door(
            car,
            lobby,
            [x + lift_w * 0.5, yl1],
            1.1,
            false,
            DoorKind::Core,
        );
        core.lifts.push(b.rooms[car].rect);
        if dst - 2.4 > 0.3 {
            b.room(Func::Shaft, Rect::new(x, yl1 + 2.4, x + lift_w, d), true);
        }
    }
    let ua = b.units.len();
    b.place(&la, Xf::identity(), lobby, nb_a);
    b.place(&lb, Xf::mirror_x(w * 0.5), lobby, nb_b);
    let _ = ua;
    // The stair window: at the half landing, north face, centred on the stair.
    let fh = er.floor_height;
    b.openings.push(Opening {
        side: Side::North,
        along: wa + stair_w * 0.5,
        width: 1.2,
        sill: fh * 0.5 + 0.9,
        head: fh * 0.5 + 2.3,
        kind: OpeningKind::StairWindow,
        room: stair,
    });
    let evac = evac_max(&b, &core.stair_doors);
    Some(Plan {
        spec,
        climate,
        plate: Rect::new(0.0, 0.0, w, d),
        floor_height: fh,
        rooms: b.rooms,
        doors: b.doors,
        openings: b.openings,
        units: b.units,
        core,
        attempts: 0,
        evac_max: evac,
        azimuth_deg: azimuth(rng, climate),
    })
}

/// Axis deviation from south: practice leans south-east in the cold north and
/// south-west in the hot south, within about 15 degrees.
fn azimuth(rng: &mut Rng, climate: Climate) -> f32 {
    let lean = match climate {
        Climate::Cold => -4.0,
        Climate::Transitional => 0.0,
        Climate::Warm => 4.0,
    };
    ((lean + rng.range(-11.0, 11.0)) * 2.0).round() / 2.0
}

pub(super) fn tower(spec: PlanSpec, climate: Climate, rng: &mut Rng) -> Option<Plan> {
    let era = if matches!(spec.era, PlanEra::Soviet | PlanEra::Estate) {
        PlanEra::Millennium
    } else {
        spec.era
    };
    let er = era_rules(era);
    let c = 7.2;
    let s = grid(rng.range(21.6, 25.8));
    let p = (s - c) * 0.5;
    let q = p + c;
    let ds = pick(rng, &[(4.8f32, 0.6), (5.1, 0.4)]);
    let dr = p - ds;
    if dr < 2.4 {
        return None;
    }
    let tall = spec.storeys as f32 * er.floor_height > 27.0;
    let lifts = if spec.storeys >= 12 { 2 } else { 1 };
    let gross = q * p;
    let same = rng.unit() < 0.75;
    let mut nbs = [0u8; 4];
    let first = if gross > 125.0 {
        pick(rng, &[(3u8, 0.85), (2, 0.15)])
    } else {
        pick(rng, &[(2u8, 0.7), (3, 0.3)])
    };
    for n in &mut nbs {
        *n = if same {
            first
        } else if gross > 125.0 {
            pick(rng, &[(3u8, 0.8), (2, 0.2)])
        } else {
            pick(rng, &[(2u8, 0.7), (3, 0.3)])
        };
    }
    let mirror = rng.unit() < 0.5;
    let centre = [s * 0.5, s * 0.5];

    let mut b = Builder::new();
    let lobby = b.room(Func::Lobby, Rect::new(p, p, q, q), true);
    // Core contents are placed inside the lobby envelope; their rectangles are
    // annotations (the cluster is not partitioned).
    let flip = |r: Rect| {
        if mirror {
            Xf::mirror_x(s * 0.5).rect(&r)
        } else {
            r
        }
    };
    let stair_w = if tall { 3.0 } else { 2.7 };
    let stair = Rect::new(p, q - 5.1, p + stair_w, q);
    let mut core = CoreInfo::default();
    core.stairs.push(flip(stair));
    let sd = |pt: [f32; 2]| {
        if mirror {
            Xf::mirror_x(s * 0.5).point(pt)
        } else {
            pt
        }
    };
    core.stair_doors.push(sd([p + stair_w * 0.5, q - 5.1]));
    if tall {
        // A scissor pair: two stairs interleaved in one shaft.
        core.stairs.push(flip(stair));
        core.stair_doors.push(sd([p + stair_w, q - 2.5]));
    }
    for k in 0..lifts {
        let x = q - 2.1 * (lifts - k) as f32;
        core.lifts.push(flip(Rect::new(x, p, x + 2.1, p + 2.4)));
    }

    let mut layouts = Vec::new();
    for (k, &nb) in nbs.iter().enumerate() {
        let l = unit::corner_unit(
            rng,
            &er,
            climate,
            &unit::Corner {
                bedrooms: nb,
                w: q,
                d: p,
                ds,
                entry_u: s * 0.5,
            },
        )?;
        let mut xf = Xf::quarter_turns(centre, k as u32);
        if mirror {
            xf = xf.then(Xf::mirror_x(s * 0.5));
        }
        layouts.push((l, xf, nb));
    }
    for (l, xf, nb) in &layouts {
        b.place(l, *xf, lobby, *nb);
    }
    let evac = evac_max(&b, &core.stair_doors);
    Some(Plan {
        spec: PlanSpec { era, ..spec },
        climate,
        plate: Rect::new(0.0, 0.0, s, s),
        floor_height: er.floor_height,
        rooms: b.rooms,
        doors: b.doors,
        openings: b.openings,
        units: b.units,
        core,
        attempts: 0,
        evac_max: evac,
        azimuth_deg: azimuth(rng, climate),
    })
}
