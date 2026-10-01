//! How long the first tile of a dense city takes, layer by layer.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use worldgen_contracts::{UrbanField, V2};

struct Counting(Arc<dyn UrbanField>, Arc<AtomicUsize>);
impl UrbanField for Counting {
    fn urbanness(&self, p: V2) -> f64 {
        self.1.fetch_add(1, Ordering::Relaxed);
        self.0.urbanness(p)
    }
}

use worldgen_contracts::RoadTile;
use worldgen_core::{Cell, Frame, Seed};
use worldgen_roads::lattice::{CHORDS, CellChords};
use worldgen_roads::network::{CELLS, CellNetwork};
use worldgen_roads::quad::{QUADS, Quad};
use worldgen_roads::{Fields, HashedTowns, ROADS, RoadsConfig, engine};

fn main() {
    let frame = Frame::new([0.0, 0.0], 1_048_576.0);
    let calls = Arc::new(AtomicUsize::new(0));
    let urban = Arc::new(Counting(HashedTowns::shared(Seed::new(7)), calls.clone()));
    let e = engine(Seed::new(7), frame, RoadsConfig::default(), Fields::new(urban)).unwrap();
    let cell = Cell::containing(&frame, [41800.0, 4400.0], 9);
    let t = Instant::now();
    for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
        e.get::<CellChords>(CHORDS, cell.neighbour(dx, dy)).unwrap();
    }
    println!("chords (4 cells):   {:>7.1} ms  {} field samples", t.elapsed().as_secs_f64() * 1e3, calls.load(Ordering::Relaxed));
    let t = Instant::now();
    let q = e.get::<Quad>(QUADS, cell).unwrap();
    println!("quad (one):         {:>7.1} ms  ({} edges, {} blocks, {} samples total)", t.elapsed().as_secs_f64() * 1e3, q.edges.len(), q.blocks.len(), calls.load(Ordering::Relaxed));
    let t = Instant::now();
    let n = e.get::<CellNetwork>(CELLS, cell).unwrap();
    println!("cell network (+2 quads): {:>7.1} ms  ({} edges)", t.elapsed().as_secs_f64() * 1e3, n.edges.len());
    let t = Instant::now();
    let tile = e.get::<RoadTile>(ROADS, Cell::containing(&frame, [41800.0, 4400.0], 11)).unwrap();
    println!("a 512 m tile (rest): {:>7.1} ms  ({} edges)", t.elapsed().as_secs_f64() * 1e3, tile.edges.len());
    let t = Instant::now();
    let tile = e.get::<RoadTile>(ROADS, Cell::containing(&frame, [41800.0, 4400.0], 11).neighbour(1, 0)).unwrap();
    println!("the next tile:      {:>7.1} ms  ({} edges)", t.elapsed().as_secs_f64() * 1e3, tile.edges.len());
}
