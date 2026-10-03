//! A window of the road network, as the contract's [`RoadTile`].

use worldgen_contracts::{RoadClass, RoadEdge, RoadNode, RoadTile};
use worldgen_core::{Cell, Context, Dependency, Error, Layer, LayerId};

use crate::config::RoadsConfig;
use crate::lattice::cell_size;
use crate::network::CellNetwork;
use crate::overlay::OVERLAY;
use crate::shape::clip;

pub const ROADS: LayerId = LayerId("roads");

pub struct RoadsLayer {
    pub config: RoadsConfig,
}

impl RoadsLayer {
    /// How far a lattice cell's roads can reach beyond the cell itself.
    fn reach(&self, cell_m: f64) -> f64 {
        (self.config.corner_jitter + 0.1) * cell_m
            + 2.0 * (self.config.warp_amp_m + self.config.wiggle_amp_m)
            + 40.0
    }
}

impl Layer for RoadsLayer {
    type Output = RoadTile;

    fn id(&self) -> LayerId {
        ROADS
    }

    fn inputs(&self) -> Vec<Dependency> {
        vec![Dependency::required(OVERLAY)]
    }

    fn collapse(&self, ctx: &Context<'_>, cell: Cell) -> Result<RoadTile, Error> {
        let frame = ctx.frame();
        let rect = cell.rect(frame);
        let s = cell_size(frame, self.config.lattice_level);
        let reach = self.reach(s);
        let range = |lo: f64, hi: f64, origin: f64| {
            (
                ((lo - reach - origin) / s).floor() as i64,
                ((hi + reach - origin) / s).floor() as i64,
            )
        };
        let (i0, i1) = range(rect.min[0], rect.max[0], frame.origin[0]);
        let (j0, j1) = range(rect.min[1], rect.max[1], frame.origin[1]);

        let mut edges: Vec<RoadEdge> = Vec::new();
        let mut nodes: std::collections::BTreeMap<_, RoadNode> = std::collections::BTreeMap::new();
        for j in j0..=j1 {
            for i in i0..=i1 {
                let net =
                    ctx.input::<CellNetwork>(OVERLAY, Cell::new(self.config.lattice_level, i, j))?;
                for e in &net.edges {
                    if e.class < self.config.min_class {
                        continue;
                    }
                    let pieces: Vec<_> = e.pieces.iter().flat_map(|p| clip(p, rect)).collect();
                    if pieces.is_empty() {
                        continue;
                    }
                    edges.push(RoadEdge {
                        pieces,
                        ..e.clone()
                    });
                }
                for n in &net.nodes {
                    nodes.entry(n.id).or_insert_with(|| n.clone());
                }
            }
        }
        // A node belongs to the tile only because a road does.
        let referenced: std::collections::BTreeSet<_> =
            edges.iter().flat_map(|e| [e.a, e.b]).collect();
        let nodes = nodes
            .into_values()
            .filter(|n| referenced.contains(&n.id))
            .collect();
        edges.sort_by_key(|e| e.id);
        Ok(RoadTile { cell, nodes, edges })
    }
}

/// The classes a given level of detail keeps.
pub fn at_least(class: RoadClass) -> impl Fn(&RoadEdge) -> bool {
    move |e| e.class >= class
}
