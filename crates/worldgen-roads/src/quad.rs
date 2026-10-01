//! A quadrilateral of the lattice and everything inside it.
//!
//! The quadrilateral is a *face*. At each rung of the hierarchy, from the
//! arterial grid down to the lanes, a face is cut by streets of that rung: streets
//! of the first family run from a division on its bottom side to one on its top
//! side, streets of the second from left to right. Divisions of the same rung are
//! matched in order, nearest first, never letting two streets of a family cross.
//! The streets cut the face into smaller faces, each bounded by pieces of streets
//! of this rung or of coarser ones, and the next rung cuts those in turn.
//!
//! A face only ever reads its own sides, and a side is a piece of a *chord*: a
//! straight line whose divisions are decided from the chord alone (see
//! `lattice`). The lattice chords are shared with the neighbouring quadrilateral,
//! the streets made here are not, so the neighbours never need to agree on
//! anything but what the shared chords already fix. A street matched on both sides
//! of a chord goes straight on across it; one matched on one side ends in a T.

use std::collections::BTreeSet;

use worldgen_contracts::{NodeId, UrbanField, V2, segment_intersection};
use worldgen_core::hash::{hash_words, to_unit};
use worldgen_core::{Cell, Context, Dependency, Error, Layer, LayerId, Seed};

use crate::config::{LEVELS, RoadsConfig, TOP_RUNG};
use crate::lattice::{CHORDS, CellChords, Chord, corner, corner_id, fabric_seed, make_chord_below};

pub const QUADS: LayerId = LayerId("roads.quads");

/// How likely a street of each rung is to be built where its divisions match.
/// The finest streets are the ones most often left out, which is what gives a
/// town blocks of different sizes instead of one uniform grid.
const PRESENT: [f64; 5] = [0.78, 0.88, 0.95, 0.99, 1.0];

/// One road between two nodes inside a quadrilateral, before the grid is bent.
#[derive(Debug, Clone, PartialEq)]
pub struct QEdge {
    pub a: NodeId,
    pub a_pos: V2,
    pub b: NodeId,
    pub b_pos: V2,
    /// Identifies the street this piece belongs to.
    pub slot: u64,
    pub level: u8,
}

/// A city block: a face no street of any rung cuts further.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub id: u64,
    /// Bottom-left, bottom-right, top-right, top-left; before the grid is bent.
    pub corners: [V2; 4],
    /// The finest rung of street that cuts it; a block with only arterials round it is 3.
    pub rung: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Quad {
    pub corners: [V2; 4], // (i, j), (i+1, j), (i+1, j+1), (i, j+1)
    pub edges: Vec<QEdge>,
    pub blocks: Vec<Block>,
    /// Which divisions of each lattice chord a street of this quadrilateral uses.
    pub used_bottom: Vec<usize>,
    pub used_top: Vec<usize>,
    pub used_left: Vec<usize>,
    pub used_right: Vec<usize>,
}

pub struct QuadLayer {
    pub config: RoadsConfig,
    pub urban: std::sync::Arc<dyn UrbanField>,
}

impl Layer for QuadLayer {
    type Output = Quad;

    fn id(&self) -> LayerId {
        QUADS
    }

    fn inputs(&self) -> Vec<Dependency> {
        vec![Dependency::required(CHORDS)]
    }

    fn collapse(&self, ctx: &Context<'_>, cell: Cell) -> Result<Quad, Error> {
        let fabric = fabric_seed(ctx);
        let (i, j) = (cell.x, cell.y);
        let own = ctx.input::<CellChords>(CHORDS, cell)?;
        let above = ctx.input::<CellChords>(CHORDS, cell.neighbour(0, 1))?;
        let right = ctx.input::<CellChords>(CHORDS, cell.neighbour(1, 0))?;
        let frame = ctx.frame();
        let corners = [
            corner(fabric, &self.config, frame, i, j),
            corner(fabric, &self.config, frame, i + 1, j),
            corner(fabric, &self.config, frame, i + 1, j + 1),
            corner(fabric, &self.config, frame, i, j + 1),
        ];
        let ids = [
            corner_id(fabric, i, j),
            corner_id(fabric, i + 1, j),
            corner_id(fabric, i + 1, j + 1),
            corner_id(fabric, i, j + 1),
        ];
        // Chord order: bottom, right, top, left; all run in the direction the face does.
        let lattice = [own.bottom.clone(), right.left.clone(), above.bottom.clone(), own.left.clone()];
        let mut build = Build { fabric, urban: &*self.urban, chords: Vec::new(), blocks: Vec::new() };
        for c in lattice {
            build.chords.push(ChordObj {
                chord: c,
                used: BTreeSet::new(),
                crossings: Vec::new(),
                rung: TOP_RUNG,
                start: NodeId(0),
                end: NodeId(0),
            });
        }
        let root = Face {
            corners,
            ids,
            bottom: FaceSide { chord: 0, t0: 0.0, t1: 1.0, forward: true },
            right: FaceSide { chord: 1, t0: 0.0, t1: 1.0, forward: true },
            top: FaceSide { chord: 2, t0: 0.0, t1: 1.0, forward: true },
            left: FaceSide { chord: 3, t0: 0.0, t1: 1.0, forward: true },
            finest: TOP_RUNG,
        };
        build.expand(root, TOP_RUNG as i8);
        let used = |k: usize| build.chords[k].used.iter().copied().collect::<Vec<_>>();
        let (used_bottom, used_right, used_top, used_left) = (used(0), used(1), used(2), used(3));
        let edges = build.edges();
        Ok(Quad { corners, edges, blocks: build.blocks, used_bottom, used_top, used_left, used_right })
    }
}

/// A street, or a lattice chord, with the divisions it offers and the ones taken.
struct ChordObj {
    chord: Chord,
    used: BTreeSet<usize>,
    /// Where streets of the same rung cross it: (parameter, node, position).
    crossings: Vec<(f64, NodeId, V2)>,
    rung: u8,
    /// The nodes a street starts and ends at (unused for lattice chords).
    start: NodeId,
    end: NodeId,
}

/// A piece of a chord forming one side of a face, `t0..t1` of the way along it.
/// `forward` says the face walks it in the chord's own direction.
#[derive(Clone, Copy)]
struct FaceSide {
    chord: usize,
    t0: f64,
    t1: f64,
    forward: bool,
}

#[derive(Clone, Copy)]
struct Face {
    corners: [V2; 4], // bottom-left, bottom-right, top-right, top-left
    ids: [NodeId; 4],
    bottom: FaceSide,
    right: FaceSide,
    top: FaceSide,
    left: FaceSide,
    /// The finest rung that has cut this face so far.
    finest: u8,
}

/// A line the grid inside a face is built from, as a straight segment.
struct Boundary {
    chord: usize,
    t0: f64,
    t1: f64,
    forward: bool,
}

impl Boundary {
    fn param(&self, s: f64) -> f64 {
        let (a, b) = if self.forward { (self.t0, self.t1) } else { (self.t1, self.t0) };
        a + (b - a) * s
    }
}

struct Build<'a> {
    fabric: Seed,
    urban: &'a dyn UrbanField,
    chords: Vec<ChordObj>,
    blocks: Vec<Block>,
}

struct Grid {
    /// `pos[c][r]`: the crossing of column boundary `c` and row boundary `r`.
    pos: Vec<Vec<V2>>,
    id: Vec<Vec<NodeId>>,
    /// Fraction along column boundary `c` at row `r`, and along row boundary `r` at column `c`.
    along_col: Vec<Vec<f64>>,
    along_row: Vec<Vec<f64>>,
}

impl Build<'_> {
    fn divisions_on(&self, side: &FaceSide, rung: u8) -> Vec<(f64, usize)> {
        let chord = &self.chords[side.chord].chord;
        let mut out: Vec<(f64, usize)> = chord
            .divisions
            .iter()
            .enumerate()
            .filter(|(_, d)| d.level == rung && d.t > side.t0 + 1e-9 && d.t < side.t1 - 1e-9)
            .map(|(k, d)| {
                let s = (d.t - side.t0) / (side.t1 - side.t0);
                (if side.forward { s } else { 1.0 - s }, k)
            })
            .collect();
        out.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        out
    }

    /// Cut a face with streets of `rung`, then hand the pieces to the next rung.
    fn expand(&mut self, face: Face, rung: i8) {
        if rung < 0 {
            self.finish(face);
            return;
        }
        let r = rung as u8;
        let len = |a: V2, b: V2| a.dist(b);
        let [bl, br, tr, tl] = face.corners;
        let extent = 0.25 * (len(bl, br) + len(br, tr) + len(tr, tl) + len(tl, bl));
        let a_pairs = self.match_sides(&face.bottom, &face.top, r, len(bl, br), len(tl, tr), extent);
        let b_pairs = self.match_sides(&face.left, &face.right, r, len(bl, tl), len(br, tr), extent);
        if a_pairs.is_empty() && b_pairs.is_empty() {
            return self.expand(face, rung - 1);
        }

        // Streets, as new chords, not yet part of the world.
        let mut a_lines: Vec<Street> = self.streets(&a_pairs, &face.bottom, &face.top, r, 0xA);
        let mut b_lines: Vec<Street> = self.streets(&b_pairs, &face.left, &face.right, r, 0xB);
        if a_lines.is_empty() && b_lines.is_empty() {
            return self.expand(face, rung - 1);
        }
        // Where each A street meets each B street. If the arrangement is not a
        // proper grid (a badly bent face), keep the face whole at this rung.
        let Some(grid) = grid(&face, &mut a_lines, &mut b_lines) else {
            return self.expand(face, rung - 1);
        };
        let a_index = self.commit(a_lines, r);
        let b_index = self.commit(b_lines, r);

        let (na, nb) = (a_index.len(), b_index.len());
        let whole = |chord: usize| Boundary { chord, t0: 0.0, t1: 1.0, forward: true };
        let row_bounds: Vec<Boundary> = std::iter::once(Boundary::of(&face.bottom))
            .chain(b_index.iter().map(|&k| whole(k)))
            .chain(std::iter::once(Boundary::of(&face.top)))
            .collect();
        let col_bounds: Vec<Boundary> = std::iter::once(Boundary::of(&face.left))
            .chain(a_index.iter().map(|&k| whole(k)))
            .chain(std::iter::once(Boundary::of(&face.right)))
            .collect();
        let side = |b: &Boundary, s0: f64, s1: f64| {
            let (ta, tb) = (b.param(s0), b.param(s1));
            FaceSide { chord: b.chord, t0: ta.min(tb), t1: ta.max(tb), forward: b.forward }
        };
        for c in 1..=na + 1 {
            for rw in 1..=nb + 1 {
                let child = Face {
                    corners: [grid.pos[c - 1][rw - 1], grid.pos[c][rw - 1], grid.pos[c][rw], grid.pos[c - 1][rw]],
                    ids: [grid.id[c - 1][rw - 1], grid.id[c][rw - 1], grid.id[c][rw], grid.id[c - 1][rw]],
                    bottom: side(&row_bounds[rw - 1], grid.along_row[c - 1][rw - 1], grid.along_row[c][rw - 1]),
                    top: side(&row_bounds[rw], grid.along_row[c - 1][rw], grid.along_row[c][rw]),
                    left: side(&col_bounds[c - 1], grid.along_col[c - 1][rw - 1], grid.along_col[c - 1][rw]),
                    right: side(&col_bounds[c], grid.along_col[c][rw - 1], grid.along_col[c][rw]),
                    finest: r,
                };
                self.expand(child, rung - 1);
            }
        }
    }

    fn finish(&mut self, face: Face) {
        let id = hash_words(&[face.ids[0].0, face.ids[1].0, face.ids[2].0, face.ids[3].0]);
        self.blocks.push(Block { id, corners: face.corners, rung: face.finest });
    }

    /// Match divisions of one rung on two opposite sides.
    fn match_sides(&self, from: &FaceSide, to: &FaceSide, rung: u8, len_from: f64, len_to: f64, extent: f64) -> Vec<(usize, f64, usize, f64)> {
        let (f, t) = (self.divisions_on(from, rung), self.divisions_on(to, rung));
        let mean_len = 0.5 * (len_from + len_to);
        let mut accepted: Vec<(usize, usize, f64, f64)> = Vec::new(); // (index in f, index in t, s_f, s_t)
        let mut taken = vec![false; t.len()];
        for (fi, &(sf, fk)) in f.iter().enumerate() {
            let spacing = self.chords[from.chord].chord.divisions[fk].spacing_m;
            let tolerance = (0.4 * spacing).min(0.3 * extent);
            let best = t
                .iter()
                .enumerate()
                .filter(|(ti, _)| !taken[*ti])
                .map(|(ti, &(st, _))| (ti, (st - sf).abs() * mean_len))
                .filter(|(_, miss)| *miss <= tolerance)
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            if let Some((ti, _)) = best {
                if accepted.iter().all(|&(fi2, ti2, _, _)| (fi as i64 - fi2 as i64).signum() == (ti as i64 - ti2 as i64).signum()) {
                    accepted.push((fi, ti, sf, t[ti].0));
                    taken[ti] = true;
                }
            }
        }
        accepted.sort_unstable_by_key(|a| a.0);
        accepted.into_iter().map(|(fi, ti, sf, st)| (f[fi].1, sf, t[ti].1, st)).collect()
    }

    /// Make the streets of a family: new chords with their own finer divisions.
    /// Nothing is recorded yet; see [`Build::commit`].
    fn streets(&self, pairs: &[(usize, f64, usize, f64)], from: &FaceSide, to: &FaceSide, rung: u8, salt: u64) -> Vec<Street> {
        let mut out = Vec::new();
        for &(fk, sf, tk, st) in pairs {
            let (df, dt) = (&self.chords[from.chord].chord.divisions[fk], &self.chords[to.chord].chord.divisions[tk]);
            let id = hash_words(&[df.id.0, dt.id.0, salt]);
            if to_unit(hash_words(&[self.fabric.0, id, 0x9E])) >= PRESENT[usize::from(rung)] {
                continue;
            }
            // A street through open country is not a street, however its ends came to match.
            if !self.is_built_along(df.pos, dt.pos, rung) {
                continue;
            }
            let chord = make_chord_below(self.fabric, self.urban, id, df.pos, dt.pos, i8::try_from(rung).unwrap() - 1);
            out.push(Street {
                chord,
                from: (from.chord, fk),
                to: (to.chord, tk),
                from_id: df.id,
                to_id: dt.id,
                s_from: sf,
                s_to: st,
                crossings: Vec::new(),
            });
        }
        out
    }

    /// Is most of the straight line `a`-`b` somewhere a street of this rung belongs?
    fn is_built_along(&self, a: V2, b: V2, rung: u8) -> bool {
        let spec = &LEVELS[usize::from(rung)];
        if spec.min_urban <= 0.0 {
            return true;
        }
        let n = ((a.dist(b) / 60.0).ceil() as usize).max(2);
        // A little slack: a street along the edge of a town belongs to it.
        let built = (0..=n)
            .filter(|&k| self.urban.urbanness(a.lerp(b, k as f64 / n as f64)) >= 0.7 * spec.min_urban)
            .count();
        built * 2 >= n + 1
    }

    /// Make streets part of the world: record the divisions they use, and drop the
    /// finer divisions that would start a few metres from a crossing. Returns each
    /// street's chord index.
    fn commit(&mut self, streets: Vec<Street>, rung: u8) -> Vec<usize> {
        let mut index = Vec::new();
        for st in streets {
            self.chords[st.from.0].used.insert(st.from.1);
            self.chords[st.to.0].used.insert(st.to.1);
            let len = st.chord.len;
            let mut chord = st.chord;
            chord.divisions.retain(|d| st.crossings.iter().all(|c| (c.0 - d.t).abs() * len > 0.3 * d.spacing_m));
            self.chords.push(ChordObj {
                chord,
                used: BTreeSet::new(),
                crossings: st.crossings,
                rung,
                start: st.from_id,
                end: st.to_id,
            });
            index.push(self.chords.len() - 1);
        }
        index
    }

    /// Every road, as pieces between consecutive nodes, once the faces are all cut.
    fn edges(&self) -> Vec<QEdge> {
        let mut out = Vec::new();
        for obj in self.chords.iter().skip(4) {
            let ch = &obj.chord;
            // The two end nodes are the divisions this street started from; recover
            // them from the first and last crossing-or-division by parameter.
            let mut stops: Vec<(f64, NodeId, V2)> = obj.crossings.clone();
            stops.extend(obj.used.iter().map(|&k| (ch.divisions[k].t, ch.divisions[k].id, ch.divisions[k].pos)));
            stops.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
            let mut chain = vec![(0.0, obj.start, ch.a)];
            chain.extend(stops);
            chain.push((1.0, obj.end, ch.b));
            for w in chain.windows(2) {
                if w[0].2.dist(w[1].2) > 1.0 {
                    out.push(QEdge { a: w[0].1, a_pos: w[0].2, b: w[1].1, b_pos: w[1].2, slot: ch.id, level: obj.rung });
                }
            }
        }
        out
    }
}

struct Street {
    chord: Chord,
    /// The (chord, division) it leaves from and arrives at.
    from: (usize, usize),
    to: (usize, usize),
    from_id: NodeId,
    to_id: NodeId,
    /// Fractions along the sides it leaves from and arrives at.
    s_from: f64,
    s_to: f64,
    crossings: Vec<(f64, NodeId, V2)>,
}

/// The crossings of a face's streets, as a grid of points between column and row boundaries.
fn grid(face: &Face, a_lines: &mut [Street], b_lines: &mut [Street]) -> Option<Grid> {
    let (na, nb) = (a_lines.len(), b_lines.len());
    let mut pos = vec![vec![V2::ZERO; nb + 2]; na + 2];
    let mut id = vec![vec![NodeId(0); nb + 2]; na + 2];
    let mut along_col = vec![vec![0.0; nb + 2]; na + 2];
    let mut along_row = vec![vec![0.0; nb + 2]; na + 2];
    let [bl, br, tr, tl] = face.corners;
    for (c, r, p, k) in [(0, 0, bl, 0), (na + 1, 0, br, 1), (na + 1, nb + 1, tr, 2), (0, nb + 1, tl, 3)] {
        pos[c][r] = p;
        id[c][r] = face.ids[k];
        along_row[c][r] = if c == 0 { 0.0 } else { 1.0 };
        along_col[c][r] = if r == 0 { 0.0 } else { 1.0 };
    }
    for (k, a) in a_lines.iter().enumerate() {
        let c = k + 1;
        for (r, p, i, row_s) in [(0, a.chord.a, a.from_id, a.s_from), (nb + 1, a.chord.b, a.to_id, a.s_to)] {
            pos[c][r] = p;
            id[c][r] = i;
            along_row[c][r] = row_s;
            along_col[c][r] = if r == 0 { 0.0 } else { 1.0 };
        }
    }
    for (k, b) in b_lines.iter().enumerate() {
        let r = k + 1;
        for (c, p, i, col_s) in [(0, b.chord.a, b.from_id, b.s_from), (na + 1, b.chord.b, b.to_id, b.s_to)] {
            pos[c][r] = p;
            id[c][r] = i;
            along_col[c][r] = col_s;
            along_row[c][r] = if c == 0 { 0.0 } else { 1.0 };
        }
    }
    for ka in 0..na {
        for kb in 0..nb {
            let (ca, cb) = (&a_lines[ka].chord, &b_lines[kb].chord);
            let (point, ta, tb) = segment_intersection(ca.a, ca.b, cb.a, cb.b)?;
            let node = NodeId(hash_words(&[ca.id.min(cb.id), ca.id.max(cb.id), 0xC805]));
            pos[ka + 1][kb + 1] = point;
            id[ka + 1][kb + 1] = node;
            along_col[ka + 1][kb + 1] = ta;
            along_row[ka + 1][kb + 1] = tb;
            a_lines[ka].crossings.push((ta, node, point));
            b_lines[kb].crossings.push((tb, node, point));
        }
    }
    Some(Grid { pos, id, along_col, along_row })
}

impl Boundary {
    fn of(side: &FaceSide) -> Boundary {
        Boundary { chord: side.chord, t0: side.t0, t1: side.t1, forward: side.forward }
    }
}
