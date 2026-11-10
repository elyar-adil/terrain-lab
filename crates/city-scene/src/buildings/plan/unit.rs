//! One dwelling drawn in its own frame: `u` along the primary (living) face,
//! `v` inward from it.  Rooms are rectangles on the 300 mm module; doors follow
//! connectivity; openings follow function.  The assembler maps the result into
//! the plate (`residential.rs`).

use super::geom::Rect;
use super::{Climate, DoorKind, Func, OpeningKind, PlanEra};
use crate::math::Rng;

pub(super) const NONE: usize = usize::MAX;

/// Design rules that change with the building era.
pub(super) struct EraRules {
    pub floor_height: f32,
    pub living: (f32, f32),
    pub master: (f32, f32),
    pub bed: (f32, f32),
    pub kitchen: (f32, f32),
    pub bath: (f32, f32),
    pub kitchen_area: (f32, f32),
    pub bath_area: (f32, f32),
    pub balcony_depth: (f32, f32),
    pub living_balcony: f32,
    pub master_balcony: f32,
    pub service_balcony: f32,
    pub dining: bool,
    pub bay_window: f32,
    pub ac_ledge: f32,
    pub gable_window: f32,
    pub head: f32,
    pub kitchen_sill: f32,
    pub kitchen_win: (f32, f32),
    pub bed_win: (f32, f32),
}

pub(super) fn era_rules(era: PlanEra) -> EraRules {
    match era {
        PlanEra::Soviet => EraRules {
            floor_height: 3.3,
            living: (3.3, 4.5),
            master: (3.6, 4.5),
            bed: (2.7, 3.9),
            kitchen: (1.8, 2.7),
            bath: (1.5, 2.1),
            kitchen_area: (5.0, 8.0),
            bath_area: (2.6, 4.0),
            balcony_depth: (1.2, 1.5),
            living_balcony: 0.35,
            master_balcony: 0.0,
            service_balcony: 0.0,
            dining: false,
            bay_window: 0.0,
            ac_ledge: 0.0,
            gable_window: 0.5,
            head: 2.7,
            kitchen_sill: 1.1,
            kitchen_win: (0.9, 1.2),
            bed_win: (1.5, 1.8),
        },
        PlanEra::Estate => EraRules {
            floor_height: 2.8,
            living: (3.0, 4.2),
            master: (3.3, 3.9),
            bed: (2.4, 3.3),
            kitchen: (1.8, 2.4),
            bath: (1.5, 2.1),
            kitchen_area: (4.2, 6.5),
            bath_area: (2.6, 4.0),
            balcony_depth: (1.2, 1.5),
            living_balcony: 0.7,
            master_balcony: 0.2,
            service_balcony: 0.0,
            dining: false,
            bay_window: 0.0,
            ac_ledge: 0.25,
            gable_window: 0.4,
            head: 2.3,
            kitchen_sill: 1.1,
            kitchen_win: (0.9, 1.2),
            bed_win: (1.5, 1.8),
        },
        PlanEra::Millennium => EraRules {
            floor_height: 2.9,
            living: (3.6, 5.1),
            master: (3.3, 4.2),
            bed: (3.0, 3.9),
            kitchen: (2.1, 3.0),
            bath: (1.8, 2.7),
            kitchen_area: (5.5, 8.0),
            bath_area: (3.2, 5.0),
            balcony_depth: (1.4, 1.8),
            living_balcony: 0.95,
            master_balcony: 0.35,
            service_balcony: 0.2,
            dining: true,
            bay_window: 0.1,
            ac_ledge: 0.6,
            gable_window: 0.6,
            head: 2.4,
            kitchen_sill: 1.0,
            kitchen_win: (1.2, 1.5),
            bed_win: (1.5, 2.1),
        },
        PlanEra::Contemporary => EraRules {
            floor_height: 3.0,
            living: (3.9, 5.4),
            master: (3.6, 4.5),
            bed: (3.0, 3.9),
            kitchen: (2.1, 3.0),
            bath: (1.8, 2.7),
            kitchen_area: (5.5, 9.0),
            bath_area: (3.4, 5.4),
            balcony_depth: (1.5, 2.1),
            living_balcony: 1.0,
            master_balcony: 0.5,
            service_balcony: 0.4,
            dining: true,
            bay_window: 0.2,
            ac_ledge: 0.8,
            gable_window: 0.7,
            head: 2.5,
            kitchen_sill: 1.0,
            kitchen_win: (1.2, 1.8),
            bed_win: (1.5, 2.4),
        },
    }
}

#[derive(Debug, Clone)]
pub(super) struct LRoom {
    pub func: Func,
    pub rect: Rect,
    pub dark: bool,
    pub enclosed: bool,
}

#[derive(Debug, Clone)]
pub(super) struct LDoor {
    pub a: usize,
    pub b: usize,
    pub at: [f32; 2],
    pub width: f32,
    pub vertical_wall: bool,
    pub kind: DoorKind,
}

#[derive(Debug, Clone)]
pub(super) struct LOpening {
    pub at: [f32; 2],
    pub normal: [f32; 2],
    pub width: f32,
    pub sill: f32,
    pub head: f32,
    pub kind: OpeningKind,
    pub room: usize,
}

pub(super) struct Layout {
    pub rooms: Vec<LRoom>,
    pub doors: Vec<LDoor>,
    pub openings: Vec<LOpening>,
}

pub(super) struct Item {
    pub func: Func,
    pub min: f32,
    pub max: f32,
    pub absorb: bool,
}

fn item(func: Func, range: (f32, f32)) -> Item {
    Item { func, min: range.0, max: range.1, absorb: false }
}

fn ceil_grid(v: f32) -> f32 {
    ((v / 0.3) - 1e-3).ceil() * 0.3
}

/// Split a row of width `w` among `items` on the 300 mm module.  Slack goes to
/// random items that have room, and to the absorbing item last.
pub(super) fn fill_row(rng: &mut Rng, w: f32, items: &[Item]) -> Option<Vec<f32>> {
    let mut width: Vec<f32> = items.iter().map(|i| ceil_grid(i.min)).collect();
    let mut slack = w - width.iter().sum::<f32>();
    if slack < -1e-3 {
        return None;
    }
    let absorber = items.iter().position(|i| i.absorb);
    while slack >= 0.3 - 1e-3 {
        let open: Vec<usize> = (0..items.len())
            .filter(|&k| !items[k].absorb && width[k] + 0.3 <= items[k].max + 1e-3)
            .collect();
        let to_absorber = absorber.is_some() && (open.is_empty() || rng.unit() < 0.35);
        let k = if to_absorber {
            absorber.unwrap_or(0)
        } else if open.is_empty() {
            return None;
        } else {
            open[rng.int(open.len() as u32) as usize]
        };
        width[k] += 0.3;
        slack -= 0.3;
    }
    // The sub-module remainder.
    let k = absorber.unwrap_or_else(|| {
        (0..items.len()).max_by(|&a, &b| width[a].total_cmp(&width[b])).unwrap_or(0)
    });
    width[k] += slack;
    if let Some(a) = absorber {
        if width[a] > items[a].max + 1.8 {
            return None;
        }
    }
    Some(width)
}

fn shuffle<T>(rng: &mut Rng, v: &mut [T]) {
    for i in (1..v.len()).rev() {
        let j = rng.int(i as u32 + 1) as usize;
        v.swap(i, j);
    }
}

fn quant(v: f32) -> f32 {
    (v * 10.0).round() / 10.0
}

pub(super) struct Through {
    pub bedrooms: u8,
    pub ws: f32,
    pub wn: f32,
    pub d: f32,
    pub ds: f32,
    pub hall: f32,
    pub west_gable: bool,
}

/// A through-unit (南北通透): living side to the south, service side to the
/// north, a hall band between, entry on the east edge of the hall.
pub(super) fn through_unit(
    rng: &mut Rng,
    er: &EraRules,
    cl: Climate,
    p: &Through,
) -> Option<Layout> {
    let nb = p.bedrooms;
    let old = !er.dining;
    let s_beds = match nb {
        1 => 1,
        2 => {
            if old || rng.unit() < 0.4 { 1 } else { 2 }
        }
        3 => {
            if p.ws > 11.4 && rng.unit() < 0.35 { 3 } else { 2 }
        }
        _ => {
            if rng.unit() < 0.6 { 3 } else { 2 }
        }
    };
    let n_beds = nb - s_beds;
    let hall_y0 = p.ds;
    let north_y0 = p.ds + p.hall;
    let dn = p.d - north_y0;

    // South row.
    let mut south: Vec<Item> = vec![Item {
        func: Func::Living,
        min: er.living.0,
        max: er.living.1,
        absorb: true,
    }];
    for b in 0..s_beds {
        south.push(item(if b == 0 { Func::MasterBed } else { Func::Bed }, if b == 0 { er.master } else { er.bed }));
    }
    shuffle(rng, &mut south);
    let sw = fill_row(rng, p.ws, &south)?;

    // North row.
    let mut north: Vec<Item> = vec![item(Func::Kitchen, er.kitchen), item(Func::Bath, er.bath)];
    for _ in 0..n_beds {
        north.push(item(Func::Bed, er.bed));
    }
    let mut opt: Vec<(Func, (f32, f32))> = Vec::new();
    if er.dining {
        opt.push((Func::Dining, (2.7, 3.6)));
    }
    opt.push((Func::Wc, (1.5, 2.1)));
    opt.push((Func::Store, (1.2, 2.1)));
    shuffle(rng, &mut opt);
    let mut extra = 0;
    while north.iter().map(|i| i.max).sum::<f32>() < p.wn - 0.01 && extra < opt.len() {
        north.push(item(opt[extra].0, opt[extra].1));
        extra += 1;
    }
    if (north.iter().map(|i| i.max).sum::<f32>() + 0.3) < p.wn {
        return None;
    }
    // Kitchen and bath keep a wall to the north; the end beside the core is
    // never a bedroom (a lift shaft may be there).
    shuffle(rng, &mut north);
    let last = north.len() - 1;
    if north[last].func.is_bedroom() {
        let swap = (0..north.len()).filter(|&k| !north[k].func.is_bedroom()).collect::<Vec<_>>();
        let k = swap[rng.int(swap.len() as u32) as usize];
        north.swap(last, k);
    }
    let nw = fill_row(rng, p.wn, &north)?;

    let mut rooms: Vec<LRoom> = Vec::new();
    let mut x = 0.0;
    for (it, w) in south.iter().zip(&sw) {
        rooms.push(LRoom {
            func: it.func,
            rect: Rect::new(x, 0.0, x + w, p.ds),
            dark: false,
            enclosed: false,
        });
        x += w;
    }
    let hall = rooms.len();
    rooms.push(LRoom {
        func: Func::Hall,
        rect: Rect::new(0.0, hall_y0, p.wn, north_y0),
        dark: true,
        enclosed: false,
    });
    let mut x = 0.0;
    for (it, w) in north.iter().zip(&nw) {
        let full = Rect::new(x, north_y0, x + w, p.d);
        x += w;
        let target = match it.func {
            Func::Kitchen => Some(rng.range(er.kitchen_area.0, er.kitchen_area.1)),
            Func::Bath => Some(rng.range(er.bath_area.0, er.bath_area.1)),
            Func::Wc => Some(rng.range(1.6, 2.4)),
            _ => None,
        };
        let mut rect = full;
        if let Some(t) = target {
            let depth = (t / w).clamp(2.1, dn);
            if dn - depth >= 1.2 {
                rect = Rect::new(full.x0, p.d - depth, full.x1, p.d);
                rooms.push(LRoom {
                    func: Func::Store,
                    rect: Rect::new(full.x0, north_y0, full.x1, p.d - depth),
                    dark: true,
                    enclosed: false,
                });
            }
        }
        let dark = matches!(it.func, Func::Store);
        rooms.push(LRoom { func: it.func, rect, dark, enclosed: false });
    }
    finish(rng, er, cl, rooms, hall, p.ws, p.d, [true, false, true, p.west_gable], Entry::Hall(p.wn))
}

pub(super) struct Corner {
    pub bedrooms: u8,
    pub w: f32,
    pub d: f32,
    pub ds: f32,
    /// `u` of the entry door on the back edge.
    pub entry_u: f32,
}

/// A corner unit of a pinwheel: living face to the south, a gable to the west,
/// the back and the east edge shared with the core and the next unit.
pub(super) fn corner_unit(rng: &mut Rng, er: &EraRules, cl: Climate, p: &Corner) -> Option<Layout> {
    let nb = p.bedrooms;
    let mut front: Vec<Item> = vec![Item {
        func: Func::Living,
        min: er.living.0,
        max: er.living.1,
        absorb: true,
    }];
    for b in 0..nb {
        front.push(item(if b == 0 { Func::MasterBed } else { Func::Bed }, if b == 0 { er.master } else { er.bed }));
    }
    shuffle(rng, &mut front);
    let fw = fill_row(rng, p.w, &front)?;

    let dr = p.d - p.ds;
    let mut back: Vec<Item> = vec![item(Func::Kitchen, er.kitchen), item(Func::Bath, er.bath)];
    if er.dining {
        back.push(item(Func::Dining, (2.7, 3.9)));
    } else {
        back.push(item(Func::Store, (1.2, 1.8)));
    }
    back.push(Item { func: Func::Hall, min: 2.4, max: 99.0, absorb: true });
    // Kitchen first (it holds the gable window); bath and the rest follow.
    let bw = fill_row(rng, p.w, &back)?;

    let mut rooms = Vec::new();
    let mut x = 0.0;
    for (it, w) in front.iter().zip(&fw) {
        rooms.push(LRoom { func: it.func, rect: Rect::new(x, 0.0, x + w, p.ds), dark: false, enclosed: false });
        x += w;
    }
    let mut x = 0.0;
    let mut hall = NONE;
    for (it, w) in back.iter().zip(&bw) {
        let dark = !matches!(it.func, Func::Kitchen);
        if it.func == Func::Hall {
            hall = rooms.len();
        }
        rooms.push(LRoom { func: it.func, rect: Rect::new(x, p.ds, x + w, p.d), dark, enclosed: false });
        x += w;
    }
    let _ = dr;
    let h = rooms[hall].rect;
    if p.entry_u < h.x0 + 0.5 || p.entry_u > h.x1 - 0.5 {
        return None;
    }
    finish(rng, er, cl, rooms, hall, p.w, p.d, [true, false, false, true], Entry::Back(p.entry_u))
}

enum Entry {
    /// On the east edge of the hall.
    Hall(f32),
    /// On the back edge at `u`.
    Back(f32),
}

/// Balconies, doors and openings of a laid-out dwelling.
fn finish(
    rng: &mut Rng,
    er: &EraRules,
    cl: Climate,
    mut rooms: Vec<LRoom>,
    hall: usize,
    bw: f32,
    bd: f32,
    ext: [bool; 4],
    entry: Entry,
) -> Option<Layout> {
    let interior = rooms.len();
    // Balconies on the south (primary) face.
    let enclose_p = match cl {
        Climate::Cold => 0.5,
        Climate::Transitional => 0.25,
        Climate::Warm => 0.08,
    } + if er.dining { 0.0 } else { 0.2 };
    let mut balcony_of: Vec<Option<usize>> = vec![None; interior];
    for i in 0..interior {
        let r = rooms[i].rect;
        if r.y0.abs() > 1e-3 {
            continue;
        }
        let p = match rooms[i].func {
            Func::Living => er.living_balcony,
            Func::MasterBed => er.master_balcony,
            _ => 0.0,
        };
        if rng.unit() < p {
            let depth = quant(rng.range(er.balcony_depth.0, er.balcony_depth.1));
            balcony_of[i] = Some(rooms.len());
            rooms.push(LRoom {
                func: Func::Balcony,
                rect: Rect::new(r.x0, -depth, r.x1, 0.0),
                dark: false,
                enclosed: rng.unit() < enclose_p,
            });
        }
    }
    // Service balconies behind kitchens on the north face.
    for i in 0..interior {
        let r = rooms[i].rect;
        if rooms[i].func == Func::Kitchen
            && ext[2]
            && (r.y1 - bd).abs() < 1e-3
            && r.w() >= 2.7
            && rng.unit() < er.service_balcony
        {
            balcony_of[i] = Some(rooms.len());
            rooms.push(LRoom {
                func: Func::ServiceBalcony,
                rect: Rect::new(r.x0, bd, r.x1, bd + 1.2),
                dark: false,
                enclosed: rng.unit() < enclose_p,
            });
        }
    }

    let doors = connect(rng, &rooms, interior, hall, &balcony_of)?;
    let mut openings = Vec::new();
    for i in 0..interior {
        windows(rng, er, &rooms, i, balcony_of[i], bw, bd, ext, &mut openings);
    }
    // Entry door, left for the assembler to attach to the lobby.
    let mut doors = doors;
    let h = rooms[hall].rect;
    let (at, vertical_wall) = match entry {
        Entry::Hall(wn) => ([wn, (h.y0 + h.y1) * 0.5], true),
        Entry::Back(u) => ([u, bd], false),
    };
    doors.push(LDoor { a: hall, b: NONE, at, width: 1.0, vertical_wall, kind: DoorKind::Entry });
    Some(Layout { rooms, doors, openings })
}

fn door_cost(from: Func, to: Func) -> u32 {
    match (from, to) {
        (_, Func::Hall) => 0,
        (Func::Kitchen | Func::Dining, Func::Living) => 1,
        (Func::Bed | Func::MasterBed | Func::Study, Func::Living) => 2,
        (_, Func::Living) => 2,
        (Func::Bed | Func::MasterBed, Func::Dining) => 9,
        (_, Func::Dining) => 3,
        (Func::Dining, _) => 4,
        _ => 6,
    }
}

fn connect(
    rng: &mut Rng,
    rooms: &[LRoom],
    interior: usize,
    hall: usize,
    balcony_of: &[Option<usize>],
) -> Option<Vec<LDoor>> {
    let mut attached = vec![false; interior];
    attached[hall] = true;
    let mut doors = Vec::new();
    loop {
        let mut best: Option<(u32, usize, usize, super::geom::Shared)> = None;
        for i in 0..interior {
            if attached[i] {
                continue;
            }
            for j in 0..interior {
                if !attached[j] {
                    continue;
                }
                // Storage and baths open to circulation or a bedroom, never a
                // kitchen.
                if matches!(rooms[i].func, Func::Bath | Func::Wc | Func::Store)
                    && matches!(rooms[j].func, Func::Kitchen | Func::Dining)
                {
                    continue;
                }
                if let Some(s) = rooms[i].rect.shared(&rooms[j].rect) {
                    if s.hi - s.lo >= 1.0 {
                        let c = door_cost(rooms[i].func, rooms[j].func);
                        if best.as_ref().is_none_or(|b| c < b.0) {
                            best = Some((c, i, j, s));
                        }
                    }
                }
            }
        }
        match best {
            Some((_, i, j, s)) => {
                attached[i] = true;
                let mid = (s.lo + s.hi) * 0.5;
                let slack = ((s.hi - s.lo) - 1.2).max(0.0) * 0.5;
                let c = mid + (rng.unit() - 0.5) * slack;
                let at = if s.vertical { [s.at, c] } else { [c, s.at] };
                let width = if matches!(rooms[i].func, Func::Bath | Func::Wc | Func::Store) { 0.8 } else { 0.9 };
                doors.push(LDoor { a: i, b: j, at, width, vertical_wall: s.vertical, kind: DoorKind::Internal });
            }
            None => break,
        }
    }
    if attached.iter().any(|a| !a) {
        return None;
    }
    // Balcony doors.
    for (i, b) in balcony_of.iter().enumerate() {
        if let Some(b) = *b {
            let s = rooms[i].rect.shared(&rooms[b].rect)?;
            let w = (s.hi - s.lo).min(1.8).max(0.9);
            let mid = (s.lo + s.hi) * 0.5;
            let at = if s.vertical { [s.at, mid] } else { [mid, s.at] };
            doors.push(LDoor { a: i, b, at, width: w, vertical_wall: s.vertical, kind: DoorKind::Balcony });
        }
    }
    Some(doors)
}

/// The windows of room `i` on each exterior face it touches.
fn windows(
    rng: &mut Rng,
    er: &EraRules,
    rooms: &[LRoom],
    i: usize,
    balcony: Option<usize>,
    bw: f32,
    bd: f32,
    ext: [bool; 4],
    out: &mut Vec<LOpening>,
) {
    let room = &rooms[i];
    let r = room.rect;
    // (side index south, east, north, west), segment lo..hi along the face, the
    // coordinate of the face, outward normal.
    let faces: [(bool, f32, f32, [f32; 2], bool); 4] = [
        (r.y0.abs() < 1e-3 && ext[0], r.x0, r.x1, [0.0, -1.0], true),
        ((r.x1 - bw).abs() < 1e-3 && ext[1], r.y0, r.y1, [1.0, 0.0], false),
        ((r.y1 - bd).abs() < 1e-3 && ext[2], r.x0, r.x1, [0.0, 1.0], true),
        (r.x0.abs() < 1e-3 && ext[3], r.y0, r.y1, [-1.0, 0.0], false),
    ];
    let need = (r.w() - 0.2) * (r.d() - 0.2) / 7.0 * 1.12;
    for (n, (on, lo, hi, normal, along_x)) in faces.iter().enumerate() {
        if !*on {
            continue;
        }
        let len = hi - lo;
        let primary = n == 0;
        let coord = match n {
            0 => 0.0,
            1 => bw,
            2 => bd,
            _ => 0.0,
        };
        let mid = (lo + hi) * 0.5;
        let mut put = |rng: &mut Rng, centre: f32, width: f32, sill: f32, head: f32, kind: OpeningKind| {
            let at = if *along_x { [centre, coord] } else { [coord, centre] };
            let _ = rng;
            out.push(LOpening { at, normal: *normal, width, sill, head, kind, room: i });
        };
        let margin = 0.3;
        let max_w = (len - 2.0 * margin).max(0.0);
        let jitter = |rng: &mut Rng, w: f32| {
            let free = ((len - w) * 0.5 - margin).max(0.0).min(0.45);
            mid + (rng.unit() * 2.0 - 1.0) * free
        };
        let head = er.head;
        match room.func {
            Func::Living | Func::MasterBed if balcony.is_some() && primary => {
                let base = if room.func == Func::Living {
                    (len * 0.62).clamp(2.1, 3.3)
                } else {
                    (len * 0.5).clamp(1.5, 2.1)
                };
                let w = quant(base.max(need / (head - 0.5)).min(max_w));
                put(rng, mid, w, 0.0, head, OpeningKind::BalconyDoor);
                ac_ledge(rng, er, room.func, mid, w, *lo, *hi, &mut put);
            }
            Func::Living | Func::Dining => {
                let w = quant((2.1f32).max(need / (head - 0.9)).min(max_w));
                let w = if primary || room.func == Func::Dining { w } else { w.min(1.8) };
                let c = jitter(rng, w);
                put(rng, c, w, 0.9, head, OpeningKind::Window);
                if room.func == Func::Living {
                    ac_ledge(rng, er, room.func, c, w, *lo, *hi, &mut put);
                }
            }
            Func::MasterBed | Func::Bed | Func::Study => {
                if !primary && rng.unit() > er.gable_window.max(if n == 2 { 1.0 } else { 0.0 }) {
                    continue;
                }
                let base = rng.range(er.bed_win.0, er.bed_win.1);
                let bay = primary && rng.unit() < er.bay_window;
                let (sill, kind): (f32, OpeningKind) = if bay { (0.45, OpeningKind::Bay) } else { (0.9, OpeningKind::Window) };
                let hh = if bay { head + 0.1 } else { head };
                let w = quant(base.max(need / (hh - sill.max(0.5))).min(max_w));
                let c = jitter(rng, w);
                put(rng, c, w, sill, hh, kind);
                ac_ledge(rng, er, room.func, c, w, *lo, *hi, &mut put);
            }
            Func::Kitchen => {
                if let Some(_b) = balcony {
                    if n == 2 {
                        let w = quant((len * 0.7).clamp(1.2, 2.1).min(max_w));
                        put(rng, mid, w, 0.0, head, OpeningKind::BalconyDoor);
                        continue;
                    }
                }
                if !primary && n != 2 && rng.unit() > 0.8 {
                    continue;
                }
                let base = rng.range(er.kitchen_win.0, er.kitchen_win.1);
                let kh = (head - 0.2).max(2.1);
                let w = quant(base.max(need / (kh - er.kitchen_sill)).min(max_w));
                let c = jitter(rng, w);
                put(rng, c, w, er.kitchen_sill, kh, OpeningKind::Window);
            }
            Func::Bath | Func::Wc => {
                let w = quant(rng.range(0.6, 0.9).min(max_w));
                let c = jitter(rng, w);
                put(rng, c, w, 1.5, 2.2, OpeningKind::Window);
            }
            _ => {}
        }
    }
}

/// A louvred platform beside the main opening, on the side with more room.
fn ac_ledge(
    rng: &mut Rng,
    er: &EraRules,
    func: Func,
    centre: f32,
    width: f32,
    lo: f32,
    hi: f32,
    put: &mut impl FnMut(&mut Rng, f32, f32, f32, f32, OpeningKind),
) {
    if !matches!(func, Func::Living | Func::MasterBed | Func::Bed) || rng.unit() >= er.ac_ledge {
        return;
    }
    let w = 0.9;
    let right = hi - (centre + width * 0.5);
    let left = (centre - width * 0.5) - lo;
    let (room, dir) = if right >= left { (right, 1.0) } else { (left, -1.0) };
    if room < w + 0.2 {
        return;
    }
    put(rng, centre + dir * (width * 0.5 + 0.1 + w * 0.5), w, 0.0, 0.9, OpeningKind::AcLedge);
}
