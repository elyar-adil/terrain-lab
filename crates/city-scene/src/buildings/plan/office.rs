//! An office floor: the plan is driven by the core, the lettable depth and the
//! column grid.  No room layout is exposed; the facade reads the grid and the
//! curtain-wall module.

use super::geom::{Rect, Side};
use super::{
    Climate, CoreInfo, Door, DoorKind, Func, Opening, OpeningKind, Plan, PlanEra, PlanSpec, Room,
    pick,
};
use crate::math::Rng;

pub(super) fn office(spec: PlanSpec, climate: Climate, rng: &mut Rng) -> Option<Plan> {
    let g = pick(rng, &[(8.4f32, 0.4), (9.0, 0.6)]);
    let nx = 4 + rng.int(5) as usize;
    let cb_options: Vec<usize> = (2..=nx - 2).filter(|c| (nx - c) % 2 == 0 && *c <= 5).collect();
    if cb_options.is_empty() {
        return None;
    }
    let cb = cb_options[rng.int(cb_options.len() as u32) as usize];
    let depth = g * pick(rng, &[(1.0f32, 0.35), (1.25, 0.3), (1.5, 0.35)]);
    let cd = g;
    let edge = rng.unit() < 0.3;
    let w = nx as f32 * g;
    let d = if edge { depth + cd } else { 2.0 * depth + cd };
    let cx0 = (nx - cb) as f32 * 0.5 * g;
    let cx1 = cx0 + cb as f32 * g;
    let cy0 = if edge { d - cd } else { depth };
    let cy1 = cy0 + cd;

    let mut rooms: Vec<Room> = Vec::new();
    let mut doors: Vec<Door> = Vec::new();
    let add = |rooms: &mut Vec<Room>, func: Func, rect: Rect, dark: bool| {
        rooms.push(Room { func, rect, unit: None, dark, enclosed: false });
        rooms.len() - 1
    };

    // Open plan bands.
    let south = add(&mut rooms, Func::Open, Rect::new(0.0, 0.0, w, cy0), false);
    let mut bands = vec![south];
    let west = add(&mut rooms, Func::Open, Rect::new(0.0, cy0, cx0, cy1), false);
    let east = add(&mut rooms, Func::Open, Rect::new(cx1, cy0, w, cy1), false);
    bands.push(west);
    bands.push(east);
    if !edge {
        bands.push(add(&mut rooms, Func::Open, Rect::new(0.0, cy1, w, d), false));
    }
    // The core: stairs at both ends, lobby on the lettable south side, a row
    // of lifts behind it, toilets and shafts at the back.
    let stair_w = pick(rng, &[(3.0f32, 0.4), (3.3, 0.3), (3.6, 0.3)]);
    let lobby_d = pick(rng, &[(3.0f32, 0.4), (3.3, 0.3), (3.6, 0.3)]);
    let stair_a = add(&mut rooms, Func::Stair, Rect::new(cx0, cy0, cx0 + stair_w, cy1), true);
    let stair_b = add(&mut rooms, Func::Stair, Rect::new(cx1 - stair_w, cy0, cx1, cy1), true);
    let mx0 = cx0 + stair_w;
    let mx1 = cx1 - stair_w;
    let lobby = add(&mut rooms, Func::Lobby, Rect::new(mx0, cy0, mx1, cy0 + lobby_d), true);
    let lift_rows = cy0 + lobby_d + 2.4;
    let per_row = (((mx1 - mx0) / 2.1).floor() as usize).min(8);
    let want = (2 + spec.storeys as usize / 3).clamp(2, 12);
    let n_lifts = (want - rng.int(3) as usize).max(2).min(per_row);
    let mut core = CoreInfo::default();
    for k in 0..n_lifts {
        let x = mx0 + 2.1 * k as f32;
        let r = Rect::new(x, cy0 + lobby_d, x + 2.1, lift_rows);
        let id = add(&mut rooms, Func::Lift, r, true);
        doors.push(Door { a: id, b: lobby, at: [x + 1.05, cy0 + lobby_d], width: 1.4, vertical_wall: false, kind: DoorKind::Core });
        core.lifts.push(r);
    }
    let used = mx0 + 2.1 * n_lifts as f32;
    if mx1 - used > 0.3 {
        add(&mut rooms, Func::Shaft, Rect::new(used, cy0 + lobby_d, mx1, lift_rows), true);
    }
    let half = (mx1 - mx0) * pick(rng, &[(0.4f32, 0.3), (0.5, 0.4), (0.6, 0.3)]);
    let wc_m = add(&mut rooms, Func::Toilets, Rect::new(mx0, lift_rows, mx0 + half, cy1), true);
    let wc_f = add(&mut rooms, Func::Toilets, Rect::new(mx0 + half, lift_rows, mx1, cy1), true);
    // Toilets open through the lift lobby wall via the lobby edge: the lifts
    // stand between, so give them their door onto the nearest stair lobby edge.
    doors.push(Door { a: wc_m, b: stair_a, at: [mx0, (lift_rows + cy1) * 0.5], width: 0.9, vertical_wall: true, kind: DoorKind::Core });
    doors.push(Door { a: wc_f, b: stair_b, at: [mx1, (lift_rows + cy1) * 0.5], width: 0.9, vertical_wall: true, kind: DoorKind::Core });
    // Lobby and stairs onto the open floor.
    doors.push(Door { a: lobby, b: south, at: [(mx0 + mx1) * 0.5, cy0], width: 3.0, vertical_wall: false, kind: DoorKind::Core });
    doors.push(Door { a: lobby, b: stair_a, at: [mx0, cy0 + 1.5], width: 1.1, vertical_wall: true, kind: DoorKind::Core });
    doors.push(Door { a: lobby, b: stair_b, at: [mx1, cy0 + 1.5], width: 1.1, vertical_wall: true, kind: DoorKind::Core });
    let sa = [cx0, (cy0 + cy1) * 0.5];
    let sb = [cx1, (cy0 + cy1) * 0.5];
    doors.push(Door { a: stair_a, b: west, at: sa, width: 1.2, vertical_wall: true, kind: DoorKind::Core });
    doors.push(Door { a: stair_b, b: east, at: sb, width: 1.2, vertical_wall: true, kind: DoorKind::Core });
    core.stairs.push(rooms[stair_a].rect);
    core.stairs.push(rooms[stair_b].rect);
    core.stair_doors.push(sa);
    core.stair_doors.push(sb);
    // The open bands are one floor: open connections between neighbours.
    for i in 0..bands.len() {
        for j in (i + 1)..bands.len() {
            if let Some(sh) = rooms[bands[i]].rect.shared(&rooms[bands[j]].rect) {
                let mid = (sh.lo + sh.hi) * 0.5;
                let at = if sh.vertical { [sh.at, mid] } else { [mid, sh.at] };
                doors.push(Door { a: bands[i], b: bands[j], at, width: sh.hi - sh.lo, vertical_wall: sh.vertical, kind: DoorKind::Core });
            }
        }
    }

    // Floor height, skin and module by era.
    let (floor_height, module): (f32, Option<f32>) = match spec.era {
        PlanEra::Soviet => (3.6, None),
        PlanEra::Estate => (3.3, None),
        PlanEra::Millennium => (3.9, Some(1.5)),
        PlanEra::Contemporary => (4.2, Some(pick(rng, &[(1.2f32, 0.3), (1.35, 0.3), (1.5, 0.4)]))),
    };
    let mut openings = Vec::new();
    for side in Side::ALL {
        let (n, origin_len) = match side {
            Side::South | Side::North => (nx, w),
            _ => (((d / g).round() as usize).max(1), d),
        };
        let step = origin_len / n as f32;
        for k in 0..n {
            let along = step * (k as f32 + 0.5);
            // The room just inside the face at this point.
            let probe = match side {
                Side::South => [along, 0.2],
                Side::North => [along, d - 0.2],
                Side::East => [w - 0.2, along],
                Side::West => [0.2, along],
            };
            let Some(room) = rooms.iter().position(|r| {
                r.func == Func::Open
                    && probe[0] >= r.rect.x0
                    && probe[0] <= r.rect.x1
                    && probe[1] >= r.rect.y0
                    && probe[1] <= r.rect.y1
            }) else {
                continue;
            };
            openings.push(match module {
                Some(m) => Opening { side, along, width: step, sill: 0.15, head: floor_height - 0.6, kind: OpeningKind::Curtain { module: m }, room },
                None => Opening { side, along, width: step - 0.9, sill: 0.9, head: 2.7, kind: OpeningKind::Ribbon, room },
            });
        }
    }

    // Evacuation: the farthest plate corner from its nearest stair door.
    let corners = [[0.0, 0.0], [w, 0.0], [0.0, d], [w, d]];
    let evac = corners
        .iter()
        .map(|c: &[f32; 2]| {
            core.stair_doors
                .iter()
                .map(|s| ((c[0] - s[0]).powi(2) + (c[1] - s[1]).powi(2)).sqrt())
                .fold(f32::MAX, f32::min)
        })
        .fold(0.0, f32::max);
    Some(Plan {
        spec,
        climate,
        plate: Rect::new(0.0, 0.0, w, d),
        floor_height,
        rooms,
        doors,
        openings,
        units: Vec::new(),
        core,
        attempts: 0,
        evac_max: evac,
        azimuth_deg: 0.0,
    })
}
