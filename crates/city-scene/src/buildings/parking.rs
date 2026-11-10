//! Car parks laid out as a designer would: bays on the lot's own grid, in
//! double-loaded modules, with the paint as geometry and the cars as real
//! instanced models.
//!
//! Dimensions are JGJ 100 for a small car: a 2.5 x 5.3 m bay on a 6.0 m aisle,
//! so a module (bay, aisle, bay) is 16.6 m.  The grid is aligned to the lot's
//! longest edge, which is the street frontage, so bays run square to the kerb
//! and the aisles run parallel to it.  A bay is kept only where all of it lies
//! inside the open ground, clear of buildings, so no car stands in a wall or
//! across the footway; a lot with too little room for a handful of bays is left
//! as plain asphalt rather than given a token row.

use std::collections::HashSet;

use crate::furniture::{add_parked_car, yaw_along_x};
use crate::math::{Rng, Vec2, inset_ring, point_in_ring, signed_area};
use crate::mesh::MeshBuilder;

use super::{cut_away, level, normalise};

const BAY_W: f32 = 2.5;
const BAY_D: f32 = 5.3;
const AISLE: f32 = 6.0;
const LINE_W: f32 = 0.10;
/// Open ground kept free along the lot edge: the fence, the kerb apron.
const EDGE_CLEAR: f32 = 0.8;
/// Open ground kept free round a building: the footway and the door apron.
const BUILDING_CLEAR: f32 = 2.0;
const MIN_BAYS: usize = 8;
const OCCUPANCY: f32 = 0.55;
/// Paint sits a few millimetres over the lot, as lane paint sits over a road.
const PAINT_LIFT: f32 = 0.015;

struct Grid {
    origin: Vec2,
    u: Vec2,
    v: Vec2,
}

impl Grid {
    fn at(&self, u: f32, v: f32) -> Vec2 {
        self.origin + self.u * u + self.v * v
    }
}

/// A bay: its module row (`2 * module + side`), index along the row, and the
/// direction from the aisle to the back of the bay.
struct Bay {
    row: i32,
    index: i32,
    u0: f32,
    v_back: f32,
    v_open: f32,
    toward_back: f32,
}

pub(super) fn lay(
    pieces: &[Vec<Vec2>],
    buildings: &[Vec<Vec2>],
    id: u32,
    builder: &mut MeshBuilder,
) {
    let mut rng = Rng::new(id.wrapping_mul(2_654_435_761) ^ 0x5ca1);
    let clear: Vec<Vec<Vec2>> = buildings
        .iter()
        .filter(|ring| ring.len() >= 3)
        .map(|ring| {
            let mut ring = ring.clone();
            if signed_area(&ring) < 0.0 {
                ring.reverse();
            }
            inset_ring(&ring, -BUILDING_CLEAR)
        })
        .collect();
    for piece in pieces {
        if piece.len() < 3 || signed_area(piece).abs() < 400.0 {
            continue;
        }
        let mut ring = piece.clone();
        if signed_area(&ring) < 0.0 {
            ring.reverse();
        }
        let inner = inset_ring(&ring, EDGE_CLEAR);
        if inner.len() < 3 || signed_area(&inner) <= 0.0 {
            continue;
        }
        for free in cut_away(inner, &clear).iter().flat_map(|p| normalise(p)) {
            if free.len() >= 3 && signed_area(&free).abs() > 200.0 {
                lay_free(&free, builder, &mut rng);
            }
        }
    }
}

fn lay_free(ring: &[Vec2], builder: &mut MeshBuilder, rng: &mut Rng) {
    let mut ring = ring.to_vec();
    if signed_area(&ring) < 0.0 {
        ring.reverse();
    }
    // The longest edge sets the grid.
    let mut best = (0.0_f32, Vec2::new(1.0, 0.0));
    for i in 0..ring.len() {
        let edge = ring[(i + 1) % ring.len()] - ring[i];
        if edge.length() > best.0 {
            best = (edge.length(), edge.normalize());
        }
    }
    let u = best.1;
    let v = u.left_normal();
    let (mut u_lo, mut u_hi, mut v_lo, mut v_hi) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
    for p in &ring {
        let (pu, pv) = (p.dot(u), p.dot(v));
        u_lo = u_lo.min(pu);
        u_hi = u_hi.max(pu);
        v_lo = v_lo.min(pv);
        v_hi = v_hi.max(pv);
    }
    let grid = Grid {
        origin: Vec2::new(0.0, 0.0),
        u,
        v,
    };
    let at = |pu: f32, pv: f32| grid.at(pu, pv);
    let fits = |u0: f32, v0: f32, v1: f32| -> bool {
        let (a, b) = (v0.min(v1), v0.max(v1));
        let samples = [
            (u0, a),
            (u0 + BAY_W, a),
            (u0, b),
            (u0 + BAY_W, b),
            (u0 + BAY_W * 0.5, (a + b) * 0.5),
            (u0 + BAY_W * 0.5, a),
            (u0 + BAY_W * 0.5, b),
            (u0, (a + b) * 0.5),
            (u0 + BAY_W, (a + b) * 0.5),
        ];
        samples
            .iter()
            .all(|&(su, sv)| point_in_ring(at(su, sv), &ring))
    };

    let mut bays: Vec<Bay> = Vec::new();
    let module = BAY_D * 2.0 + AISLE;
    // Try a few grid phases along the lot so a module sits where most bays fit.
    let mut best_bays: Vec<Bay> = Vec::new();
    for phase in 0..4 {
        bays.clear();
        let v_start = v_lo + phase as f32 * 1.0;
        let mut m = 0;
        let mut v0 = v_start;
        while v0 + BAY_D <= v_hi {
            // Row A: back wall at v0, bays open towards +v.
            // Row B: back wall at v0 + module, bays open towards -v.
            let rows = [
                (2 * m, v0, v0 + BAY_D, 1.0_f32),
                (2 * m + 1, v0 + module, v0 + module - BAY_D, -1.0_f32),
            ];
            for (row, v_back, v_open, toward) in rows {
                let mut index = 0;
                let mut u0 = u_lo;
                while u0 + BAY_W <= u_hi {
                    if fits(u0, v_back, v_open) {
                        // `toward_back` is the unit direction from the aisle
                        // to the back wall, along v.
                        bays.push(Bay {
                            row,
                            index,
                            u0,
                            v_back,
                            v_open,
                            toward_back: -toward,
                        });
                    }
                    u0 += BAY_W;
                    index += 1;
                }
            }
            m += 1;
            v0 += module;
        }
        if bays.len() > best_bays.len() {
            std::mem::swap(&mut best_bays, &mut bays);
        }
    }
    let bays = best_bays;
    // A lot with room for a token few bays is plain asphalt.
    if bays.len() < MIN_BAYS {
        return;
    }
    let taken: HashSet<(i32, i32)> = bays.iter().map(|b| (b.row, b.index)).collect();
    let y = level::PARCEL + PAINT_LIFT;
    let paint = |builder: &mut MeshBuilder, u0: f32, u1: f32, v0: f32, v1: f32| {
        let mut quad = vec![at(u0, v0), at(u1, v0), at(u1, v1), at(u0, v1)];
        if signed_area(&quad) < 0.0 {
            quad.reverse();
        }
        builder.ground_uv("marking.white", &quad, y, None);
    };
    for bay in &bays {
        let (a, b) = (bay.v_back.min(bay.v_open), bay.v_back.max(bay.v_open));
        // The left side line, and the right one only where no bay beside it
        // shares it.
        paint(builder, bay.u0 - LINE_W * 0.5, bay.u0 + LINE_W * 0.5, a, b);
        if !taken.contains(&(bay.row, bay.index + 1)) {
            let r = bay.u0 + BAY_W;
            paint(builder, r - LINE_W * 0.5, r + LINE_W * 0.5, a, b);
        }
        // The back line closes the bay at the wall end.
        let (b0, b1) = if bay.toward_back > 0.0 {
            (bay.v_back - LINE_W, bay.v_back)
        } else {
            (bay.v_back, bay.v_back + LINE_W)
        };
        paint(builder, bay.u0, bay.u0 + BAY_W, b0, b1);
    }
    for bay in &bays {
        if !rng.chance(OCCUPANCY) {
            continue;
        }
        // Mostly nose in, as people drive into a bay; some back in.
        let nose = if rng.chance(0.8) {
            bay.toward_back
        } else {
            -bay.toward_back
        };
        let depth_mid = (bay.v_back + bay.v_open) * 0.5 + rng.range(-0.15, 0.15);
        let centre = at(bay.u0 + BAY_W * 0.5 + rng.range(-0.18, 0.18), depth_mid);
        let heading = (v * nose + u * rng.range(-0.04, 0.04)).normalize();
        add_parked_car(builder, centre, level::PARCEL, yaw_along_x(heading), rng);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x0: f32, z0: f32, x1: f32, z1: f32) -> Vec<Vec2> {
        vec![
            Vec2::new(x0, z0),
            Vec2::new(x1, z0),
            Vec2::new(x1, z1),
            Vec2::new(x0, z1),
        ]
    }

    #[test]
    fn a_lot_gets_bays_and_cars_inside_it() {
        let mut builder = MeshBuilder::new();
        lay(&[rect(0.0, 0.0, 60.0, 40.0)], &[], 7, &mut builder);
        assert!(builder.index_count("marking.white") > 0);
        let scene = builder.build();
        let cars = scene
            .instances
            .iter()
            .find(|g| g.key == "car/body")
            .expect("cars");
        assert!(cars.instances.len() >= 8, "{} cars", cars.instances.len());
        for car in &cars.instances {
            assert!(car.x > 0.0 && car.x < 60.0 && car.z > 0.0 && car.z < 40.0);
        }
    }

    #[test]
    fn a_building_clears_its_surroundings_and_a_sliver_stays_bare() {
        let mut builder = MeshBuilder::new();
        let whole = rect(0.0, 0.0, 60.0, 40.0);
        lay(&[whole], &[rect(0.0, 0.0, 60.0, 40.0)], 7, &mut builder);
        assert_eq!(builder.index_count("marking.white"), 0);
        lay(&[rect(0.0, 0.0, 60.0, 5.0)], &[], 7, &mut builder);
        assert_eq!(builder.index_count("marking.white"), 0);
    }
}
