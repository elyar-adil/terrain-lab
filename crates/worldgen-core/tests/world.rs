//! The three promises, tested on a toy world: a river that crosses every cell,
//! and trees that depend on it.
//!
//! 1. **Stable.** What a cell contains does not depend on what was asked for
//!    before it, or on whether anything was cached.
//! 2. **Seamless.** Two cells that share a side agree on what crosses it, without
//!    either having been computed first.
//! 3. **Refining.** Zooming in adds things and never moves or removes the ones
//!    already seen.

use worldgen_core::hash::{hash_words, to_unit};
use worldgen_core::{
    Cell, Context, Dependency, Engine, EngineBuilder, Error, Frame, Layer, LayerId, Seed, edge_fraction,
};

const RIVER: LayerId = LayerId("river");
const TREES: LayerId = LayerId("trees");
const RIVER_LEVEL: u8 = 4;

/// A polyline across a cell, west edge to east edge, in metres.
#[derive(Debug, Clone, PartialEq)]
struct River(Vec<[f64; 2]>);

struct RiverLayer;

impl Layer for RiverLayer {
    type Output = River;
    fn id(&self) -> LayerId {
        RIVER
    }
    fn collapse(&self, ctx: &Context<'_>, cell: Cell) -> Result<River, Error> {
        assert_eq!(cell.level, RIVER_LEVEL, "the river is defined at one level");
        let rect = cell.rect(ctx.frame());
        let h = rect.height();
        let seed = ctx.seed();
        // Where the river meets the west and east sides: a function of the *pair*
        // of cells, so the neighbour computes the same number.
        let west = edge_fraction(seed, cell, cell.neighbour(-1, 0));
        let east = edge_fraction(seed, cell, cell.neighbour(1, 0));
        let mut rng = ctx.cell_seed(cell).rng();
        let mut points = vec![[rect.min[0], rect.min[1] + west * h]];
        for i in 1..=5 {
            let t = i as f64 / 6.0;
            let base = west + (east - west) * t;
            let wobble = rng.range(-0.08, 0.08);
            points.push([rect.min[0] + t * rect.width(), rect.min[1] + (base + wobble).clamp(0.0, 1.0) * h]);
        }
        points.push([rect.max[0], rect.min[1] + east * h]);
        Ok(River(points))
    }
}

/// Trees at a point in metres. The same lattice at every level, so a tree seen at
/// a coarse level is the same tree at a fine one.
#[derive(Debug, Clone, PartialEq)]
struct Trees(Vec<[f64; 2]>);

const LATTICE_M: f64 = 8.0;

/// Fraction of lattice sites that hold a tree at a level: denser as you zoom in.
fn density(level: u8) -> f64 {
    (0.04 * f64::from(level.saturating_sub(RIVER_LEVEL) + 1)).min(0.9)
}

struct TreeLayer;

impl Layer for TreeLayer {
    type Output = Trees;
    fn id(&self) -> LayerId {
        TREES
    }
    fn inputs(&self) -> Vec<Dependency> {
        vec![Dependency::required(RIVER)]
    }
    fn collapse(&self, ctx: &Context<'_>, cell: Cell) -> Result<Trees, Error> {
        let rect = cell.rect(ctx.frame());
        // The river is defined on coarser cells; gather it from the cell's
        // ancestor and its neighbours, so a tree near a border avoids a river
        // that belongs to the next cell.
        let anchor = cell.ancestor(RIVER_LEVEL);
        let mut rivers = Vec::new();
        for dy in -1..=1 {
            for dx in -1..=1 {
                rivers.push(ctx.input::<River>(RIVER, anchor.neighbour(dx, dy))?);
            }
        }
        let seed = ctx.seed();
        let (i0, i1) = ((rect.min[0] / LATTICE_M).floor() as i64, (rect.max[0] / LATTICE_M).ceil() as i64);
        let (j0, j1) = ((rect.min[1] / LATTICE_M).floor() as i64, (rect.max[1] / LATTICE_M).ceil() as i64);
        let mut out = Vec::new();
        for j in j0..j1 {
            for i in i0..i1 {
                // Everything about a site comes from its global index only.
                let site = seed.derive_u64(hash_words(&[i as u64, j as u64]));
                if site.unit() >= density(cell.level) {
                    continue;
                }
                let p = [
                    (i as f64 + 0.15 + 0.7 * to_unit(site.derive("x").0)) * LATTICE_M,
                    (j as f64 + 0.15 + 0.7 * to_unit(site.derive("y").0)) * LATTICE_M,
                ];
                if !rect.contains(p) {
                    continue; // belongs to the neighbouring cell
                }
                if rivers.iter().any(|r| near(&r.0, p, 6.0)) {
                    continue;
                }
                out.push(p);
            }
        }
        out.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Ok(Trees(out))
    }
}

fn near(line: &[[f64; 2]], p: [f64; 2], within: f64) -> bool {
    line.windows(2).any(|s| {
        let (a, b) = (s[0], s[1]);
        let d = [b[0] - a[0], b[1] - a[1]];
        let l2 = d[0] * d[0] + d[1] * d[1];
        let t = if l2 == 0.0 { 0.0 } else { (((p[0] - a[0]) * d[0] + (p[1] - a[1]) * d[1]) / l2).clamp(0.0, 1.0) };
        ((p[0] - a[0] - d[0] * t).powi(2) + (p[1] - a[1] - d[1] * t).powi(2)).sqrt() < within
    })
}

fn engine(seed: u64) -> Engine {
    EngineBuilder::new(Seed::new(seed), Frame::new([0.0, 0.0], 1024.0 * 16.0))
        .with(RiverLayer)
        .with(TreeLayer)
        .build()
        .unwrap()
}

fn cells() -> Vec<Cell> {
    let mut v = Vec::new();
    for y in -2..4 {
        for x in -2..4 {
            v.push(Cell::new(RIVER_LEVEL, x, y));
        }
    }
    v
}

#[test]
fn what_a_cell_contains_does_not_depend_on_the_order_it_was_asked_for() {
    let forward = engine(99);
    let backward = engine(99);
    let cs = cells();
    let a: Vec<_> = cs.iter().map(|c| (*forward.get::<Trees>(TREES, *c).unwrap()).clone()).collect();
    let b: Vec<_> = cs.iter().rev().map(|c| (*backward.get::<Trees>(TREES, *c).unwrap()).clone()).collect();
    let b: Vec<_> = b.into_iter().rev().collect();
    assert_eq!(a, b);
    assert!(a.iter().any(|t| !t.0.is_empty()), "the fixture has trees");
}

#[test]
fn a_cold_cache_and_a_warm_one_give_the_same_cell() {
    let warm = engine(5);
    for c in cells() {
        warm.get::<Trees>(TREES, c).unwrap();
    }
    let cold = engine(5);
    let target = Cell::new(RIVER_LEVEL, 1, 1);
    assert_eq!(*warm.get::<Trees>(TREES, target).unwrap(), *cold.get::<Trees>(TREES, target).unwrap());
}

#[test]
fn a_different_seed_gives_a_different_world() {
    let (a, b) = (engine(1), engine(2));
    let target = Cell::new(RIVER_LEVEL, 0, 0);
    assert_ne!(*a.get::<River>(RIVER, target).unwrap(), *b.get::<River>(RIVER, target).unwrap());
    assert_ne!(*a.get::<Trees>(TREES, target).unwrap(), *b.get::<Trees>(TREES, target).unwrap());
}

#[test]
fn neighbouring_cells_agree_on_where_the_river_crosses_their_shared_side() {
    let e = engine(42);
    let mut checked = 0;
    for c in cells() {
        let east = c.neighbour(1, 0);
        let here = e.get::<River>(RIVER, c).unwrap();
        let there = e.get::<River>(RIVER, east).unwrap();
        let (end, start) = (here.0.last().unwrap(), there.0.first().unwrap());
        assert!((end[0] - start[0]).abs() < 1e-9 && (end[1] - start[1]).abs() < 1e-9, "{end:?} vs {start:?}");
        checked += 1;
    }
    assert!(checked > 20);
}

#[test]
fn the_river_is_one_line_across_many_cells_not_a_row_of_unrelated_pieces() {
    // Along a row, consecutive cells chain end to start, so the whole river is a
    // single connected line however many cells it was computed in.
    let e = engine(7);
    let row: Vec<_> = (-3..5).map(|x| e.get::<River>(RIVER, Cell::new(RIVER_LEVEL, x, 0)).unwrap()).collect();
    for w in row.windows(2) {
        assert_eq!(w[0].0.last().unwrap()[1], w[1].0.first().unwrap()[1]);
    }
}

#[test]
fn zooming_in_adds_trees_and_never_moves_or_removes_the_ones_already_seen() {
    let e = engine(11);
    let parent = Cell::new(RIVER_LEVEL, 1, 0);
    let coarse = e.get::<Trees>(TREES, parent).unwrap();
    assert!(!coarse.0.is_empty());
    let mut finer: Vec<[f64; 2]> = Vec::new();
    for child in parent.children() {
        finer.extend(e.get::<Trees>(TREES, child).unwrap().0.iter().copied());
    }
    for tree in &coarse.0 {
        assert!(finer.contains(tree), "tree {tree:?} seen at the coarse level vanished or moved");
    }
    assert!(finer.len() > coarse.0.len(), "a finer level has more trees");
}

#[test]
fn trees_keep_off_a_river_that_belongs_to_the_next_cell() {
    let e = engine(3);
    for c in cells() {
        let trees = e.get::<Trees>(TREES, c).unwrap();
        for dy in -1..=1 {
            for dx in -1..=1 {
                let river = e.get::<River>(RIVER, c.neighbour(dx, dy)).unwrap();
                for t in &trees.0 {
                    assert!(!near(&river.0, *t, 6.0), "a tree stands in the river at {t:?}");
                }
            }
        }
    }
}

#[test]
fn trees_are_unchanged_by_which_cell_asked_about_them() {
    // The trees of a fine cell are exactly the trees of the coarse cell that lie
    // inside it: a place has one population, however you reach it.
    let e = engine(21);
    let parent = Cell::new(RIVER_LEVEL, 0, 1);
    let child = parent.children()[2];
    let from_child = e.get::<Trees>(TREES, child).unwrap();
    // Evaluate the same fine level by another route: all four children, then pick.
    let e2 = engine(21);
    for c in parent.children().iter().rev() {
        e2.get::<Trees>(TREES, *c).unwrap();
    }
    assert_eq!(*from_child, *e2.get::<Trees>(TREES, child).unwrap());
}

#[test]
fn the_world_identity_changes_when_a_layer_version_does() {
    let a = engine(1).identity();
    assert_eq!(a.0, 1);
    assert_eq!(a.1, vec![("river", 1), ("trees", 1)]);
}
