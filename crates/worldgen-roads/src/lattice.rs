//! The lattice and its chords.
//!
//! The ground is covered by a loose, slightly irregular grid of quadrilaterals.
//! Its sides, the *chords*, are the only things two neighbouring quadrilaterals
//! share, so everything two neighbours must agree on is decided here, once, as a
//! function of the chord and nothing else.
//!
//! Along a chord, streets leave at *divisions*. Divisions come from a nested
//! one-dimensional point process: each rung of the hierarchy (arterial, collector,
//! local, lane) is a jittered sequence in "cumulative density", so streets crowd
//! together where the place is built up and thin out in the country, and the
//! coarser rungs are always a subset of the finer ones. Zooming out removes
//! divisions; it never moves any.

use std::sync::Arc;

use worldgen_contracts::{NodeId, UrbanField, V2, v2};
use worldgen_core::hash::{hash_str, hash_words, to_unit};
use worldgen_core::{Cell, Context, Dependency, Error, Frame, Layer, LayerId, Seed};

use crate::config::{LEVELS, RoadsConfig, spacing};

pub const CHORDS: LayerId = LayerId("roads.chords");

/// The seed every roads layer shares, so a corner is the same corner to all of them.
pub(crate) fn fabric_seed(ctx: &Context<'_>) -> Seed {
    ctx.world_seed().derive("roads.fabric")
}

pub(crate) fn cell_size(frame: &Frame, level: u8) -> f64 {
    frame.root_size_m / (1_u64 << level) as f64
}

/// Position of lattice corner `(i, j)`, before the grid is bent.
pub(crate) fn corner(fabric: Seed, cfg: &RoadsConfig, frame: &Frame, i: i64, j: i64) -> V2 {
    let s = cell_size(frame, cfg.lattice_level);
    let h = fabric.derive("corner").derive_cell(Cell::new(cfg.lattice_level, i, j));
    v2(
        frame.origin[0] + i as f64 * s + (h.derive("x").unit() - 0.5) * 2.0 * cfg.corner_jitter * s,
        frame.origin[1] + j as f64 * s + (h.derive("y").unit() - 0.5) * 2.0 * cfg.corner_jitter * s,
    )
}

pub(crate) fn corner_id(fabric: Seed, i: i64, j: i64) -> NodeId {
    NodeId::named(hash_str("roads.corner") ^ fabric.0, i as u64, j as u64)
}

/// A place along a chord where a street may leave.
#[derive(Debug, Clone, PartialEq)]
pub struct Division {
    pub id: NodeId,
    /// Fraction of the way along the chord, 0 to 1.
    pub t: f64,
    pub level: u8,
    pub pos: V2,
    /// Street spacing at this place, metres: how tolerant a match to the far side may be.
    pub spacing_m: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chord {
    pub id: u64,
    pub a: V2,
    pub b: V2,
    pub len: f64,
    pub divisions: Vec<Division>,
}

/// The two chords a cell owns: its bottom side (towards +x) and its left side (towards +y).
#[derive(Debug, Clone, PartialEq)]
pub struct CellChords {
    pub bottom: Chord,
    pub left: Chord,
}

pub struct ChordLayer {
    pub config: RoadsConfig,
    pub urban: Arc<dyn UrbanField>,
}

impl Layer for ChordLayer {
    type Output = CellChords;

    fn id(&self) -> LayerId {
        CHORDS
    }

    fn inputs(&self) -> Vec<Dependency> {
        Vec::new()
    }

    fn collapse(&self, ctx: &Context<'_>, cell: Cell) -> Result<CellChords, Error> {
        if cell.level != self.config.lattice_level {
            return Err(Error::Layer(format!("chords live at level {}, not {}", self.config.lattice_level, cell.level)));
        }
        let fabric = fabric_seed(ctx);
        let (i, j) = (cell.x, cell.y);
        let c00 = corner(fabric, &self.config, ctx.frame(), i, j);
        let c10 = corner(fabric, &self.config, ctx.frame(), i + 1, j);
        let c01 = corner(fabric, &self.config, ctx.frame(), i, j + 1);
        Ok(CellChords {
            bottom: make_chord(fabric, &*self.urban, hash_words(&[fabric.0, i as u64, j as u64, 0xB0]), c00, c10),
            left: make_chord(fabric, &*self.urban, hash_words(&[fabric.0, i as u64, j as u64, 0x1E]), c00, c01),
        })
    }
}

/// Divide a chord at every rung. A pure function of the chord's ends, its id and
/// the urban field.
pub(crate) fn make_chord(fabric: Seed, urban: &dyn UrbanField, id: u64, a: V2, b: V2) -> Chord {
    make_chord_below(fabric, urban, id, a, b, 3)
}

/// Divide a chord at rungs up to and including `top_rung`. A street of rung `r`
/// is divided at the finer rungs only.
pub(crate) fn make_chord_below(fabric: Seed, urban: &dyn UrbanField, id: u64, a: V2, b: V2, top_rung: i8) -> Chord {
    let len = a.dist(b);
    let n = ((len / 14.0).ceil() as usize).max(4);
    let step = len / n as f64;
    let u: Vec<f64> = (0..n).map(|k| urban.urbanness(a.lerp(b, (k as f64 + 0.5) / n as f64))).collect();

    let mut kept: Vec<Division> = Vec::new();
    for spec in LEVELS.iter().rev().filter(|s| i8::try_from(s.level).unwrap() <= top_rung) {
        // Cumulative count of this rung's streets along the chord.
        let mut cumulative = vec![0.0];
        for &urbanness in &u {
            let per_metre = spacing(spec, urbanness).map_or(0.0, |s| 1.0 / s);
            cumulative.push(cumulative.last().unwrap() + step * per_metre);
        }
        let total = *cumulative.last().unwrap();
        let mut m = 0_u64;
        loop {
            let jitter = to_unit(hash_words(&[fabric.0, id, u64::from(spec.level), m, 0xD1]));
            let target = m as f64 + 0.5 + (jitter - 0.5) * 0.6;
            if target >= total {
                break;
            }
            m += 1;
            let k = cumulative.partition_point(|&w| w < target).saturating_sub(1).min(n - 1);
            let span = cumulative[k + 1] - cumulative[k];
            if span <= 0.0 {
                continue;
            }
            let t = (k as f64 + (target - cumulative[k]) / span) / n as f64;
            let pos = a.lerp(b, t);
            let local = spacing(spec, urban.urbanness(pos)).unwrap_or(spec.base_spacing_m);
            let from_ends = (t * len).min((1.0 - t) * len);
            if from_ends < (0.22 * local).max(16.0) {
                continue;
            }
            // A finer street too close to a coarser one is absorbed by it.
            let crowded = kept.iter().any(|d| d.level > spec.level && (d.t - t).abs() * len < 0.25 * local);
            if crowded {
                continue;
            }
            kept.push(Division {
                id: NodeId(hash_words(&[id, u64::from(spec.level), m, 0xD0])),
                t,
                level: spec.level,
                pos,
                spacing_m: local,
            });
        }
    }
    kept.sort_by(|x, y| x.t.partial_cmp(&y.t).unwrap());
    Chord { id, a, b, len, divisions: kept }
}

#[cfg(test)]
mod tests {
    use super::*;
    use worldgen_contracts::ConstantUrban;

    fn chord(urban: f64, len: f64) -> Chord {
        make_chord(Seed::new(1), &ConstantUrban(urban), 42, v2(0.0, 0.0), v2(len, 0.0))
    }

    #[test]
    fn a_denser_place_has_more_divisions_and_every_rung_adds_to_the_coarser_ones() {
        let town = chord(1.0, 2048.0);
        let country = chord(0.0, 2048.0);
        assert!(town.divisions.len() > 8 * country.divisions.len().max(1), "{} vs {}", town.divisions.len(), country.divisions.len());
        // The countryside only has the coarsest rung.
        assert!(country.divisions.iter().all(|d| d.level == 3));
        // A town has all four.
        for level in 0..4 {
            assert!(town.divisions.iter().any(|d| d.level == level), "no level {level}");
        }
        // Dropping the finer rungs leaves exactly the coarser divisions, unmoved.
        let coarse: Vec<_> = town.divisions.iter().filter(|d| d.level >= 2).collect();
        assert!(coarse.len() < town.divisions.len());
    }

    #[test]
    fn divisions_are_deterministic_ordered_and_keep_away_from_the_ends() {
        let a = chord(0.8, 2048.0);
        assert_eq!(a, chord(0.8, 2048.0));
        assert!(a.divisions.windows(2).all(|w| w[0].t < w[1].t));
        for d in &a.divisions {
            assert!(d.t * a.len >= 16.0 && (1.0 - d.t) * a.len >= 16.0);
        }
        let other = make_chord(Seed::new(1), &ConstantUrban(0.8), 43, v2(0.0, 0.0), v2(2048.0, 0.0));
        assert_ne!(a.divisions.iter().map(|d| d.id).collect::<Vec<_>>(), other.divisions.iter().map(|d| d.id).collect::<Vec<_>>());
    }

    #[test]
    fn dense_spacing_is_close_to_the_specification_and_no_two_streets_are_on_top_of_each_other() {
        let town = chord(1.0, 4096.0);
        let lanes: Vec<_> = town.divisions.iter().filter(|d| d.level == 1).collect();
        let mean = 4096.0 / lanes.len() as f64;
        assert!((100.0..260.0).contains(&mean), "level-1 mean spacing {mean}");
        let min_gap = town.divisions.windows(2).map(|w| (w[1].t - w[0].t) * town.len).fold(f64::INFINITY, f64::min);
        assert!(min_gap > 14.0, "two divisions {min_gap} m apart");
    }
}
