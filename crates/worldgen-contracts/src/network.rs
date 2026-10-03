//! The road network contract: what a roads layer produces and consumers read.
//!
//! Identity is the point. A node or an edge has an id that comes from *how it was
//! generated* (which cell, which parent road, which slot), never from the order it
//! was found in. Two tiles that both contain the same road therefore report the
//! same id, and assembling a window from several tiles is a merge by id, not a
//! geometric guess. A merge that finds one id with two different contents has found
//! a seam bug, and says so.

use std::collections::BTreeMap;

use worldgen_core::Cell;
use worldgen_core::hash::{combine, hash_words};

use crate::geom::{Polyline, V2};
use crate::road::{RoadClass, Setting};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EdgeId(pub u64);

impl EdgeId {
    /// The id of the road joining two nodes. Symmetric: the same road whichever
    /// end you start from. `slot` tells apart two roads between the same nodes.
    pub fn between(a: NodeId, b: NodeId, slot: u64) -> EdgeId {
        let (lo, hi) = if a <= b { (a.0, b.0) } else { (b.0, a.0) };
        EdgeId(hash_words(&[lo, hi, slot, 0xED6E]))
    }
}

impl NodeId {
    /// A node named by the generator that makes it: the same inputs, the same id.
    pub fn named(layer_hash: u64, a: u64, b: u64) -> NodeId {
        NodeId(combine(combine(layer_hash, a), b))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    /// The centre a settlement's roads grow from.
    Hub,
    /// Three or more roads meet.
    Junction,
    /// Two roads of different class or setting meet, or the road changes character.
    Transition,
    DeadEnd,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RoadNode {
    pub id: NodeId,
    pub position: V2,
    pub kind: NodeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpanKind {
    Bridge,
    Tunnel,
}

/// A stretch of an edge that is not at grade, anchored by two points on its
/// centreline (not by arc length, which would change with the level of detail).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub kind: SpanKind,
    pub from: V2,
    pub to: V2,
}

/// Where a road came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EdgeSource {
    /// Made by the roads layer from its own rules.
    #[default]
    Generated,
    /// Laid down as given by a planner (a regional road routed over the terrain).
    Given,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RoadEdge {
    pub id: EdgeId,
    pub a: NodeId,
    pub b: NodeId,
    pub class: RoadClass,
    pub setting: Setting,
    pub spans: Vec<Span>,
    pub source: EdgeSource,
    /// The parts of the centreline that fall inside the tile this edge arrived in,
    /// running from `a` towards `b`, at the tile's level of detail. A whole edge
    /// is one piece; an edge crossing several tiles is one piece per tile.
    pub pieces: Vec<Polyline>,
}

impl RoadEdge {
    /// Everything about the edge except its geometry, for consistency checks.
    fn identity(&self) -> (NodeId, NodeId, RoadClass, Setting, &[Span], EdgeSource) {
        (
            self.a,
            self.b,
            self.class,
            self.setting,
            &self.spans,
            self.source,
        )
    }
}

/// What a roads layer returns for one cell.
#[derive(Debug, Clone, PartialEq)]
pub struct RoadTile {
    pub cell: Cell,
    /// Nodes inside the cell, plus the end nodes of every edge that enters it.
    pub nodes: Vec<RoadNode>,
    pub edges: Vec<RoadEdge>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Conflict {
    Node(NodeId),
    Edge(EdgeId),
    /// An edge references a node no tile supplied.
    MissingNode(EdgeId, NodeId),
}

/// Tiles merged into one network, by id.
#[derive(Debug, Clone, Default)]
pub struct RoadNetwork {
    pub nodes: BTreeMap<NodeId, RoadNode>,
    pub edges: BTreeMap<EdgeId, RoadEdge>,
}

impl RoadNetwork {
    pub fn assemble<'a>(
        tiles: impl IntoIterator<Item = &'a RoadTile>,
    ) -> Result<RoadNetwork, Conflict> {
        let mut net = RoadNetwork::default();
        for tile in tiles {
            for n in &tile.nodes {
                match net.nodes.get(&n.id) {
                    Some(old) if old != n => return Err(Conflict::Node(n.id)),
                    Some(_) => {}
                    None => {
                        net.nodes.insert(n.id, n.clone());
                    }
                }
            }
            for e in &tile.edges {
                match net.edges.get_mut(&e.id) {
                    Some(old) if old.identity() != e.identity() => {
                        return Err(Conflict::Edge(e.id));
                    }
                    Some(old) => {
                        for piece in &e.pieces {
                            if !old.pieces.contains(piece) {
                                old.pieces.push(piece.clone());
                            }
                        }
                    }
                    None => {
                        net.edges.insert(e.id, e.clone());
                    }
                }
            }
        }
        for e in net.edges.values() {
            for end in [e.a, e.b] {
                if !net.nodes.contains_key(&end) {
                    return Err(Conflict::MissingNode(e.id, end));
                }
            }
        }
        for e in net.edges.values_mut() {
            e.pieces = stitch(std::mem::take(&mut e.pieces));
        }
        Ok(net)
    }

    /// Roads meeting at a node.
    pub fn degree(&self, node: NodeId) -> usize {
        self.edges
            .values()
            .filter(|e| e.a == node || e.b == node)
            .count()
    }
}

/// Join pieces whose end points coincide into the fewest chains, in travel order.
/// Pieces from neighbouring tiles meet exactly because refinement does not depend
/// on the window, so equality here is exact, not a tolerance.
pub fn stitch(pieces: Vec<Polyline>) -> Vec<Polyline> {
    let mut chains: Vec<Vec<V2>> = pieces
        .into_iter()
        .filter(|p| p.0.len() >= 2)
        .map(|p| p.0)
        .collect();
    loop {
        let mut joined = false;
        'search: for i in 0..chains.len() {
            for j in 0..chains.len() {
                if i != j && chains[i].last() == chains[j].first() {
                    let tail = chains.remove(j);
                    let at = if j < i { i - 1 } else { i };
                    chains[at].extend(tail.into_iter().skip(1));
                    joined = true;
                    break 'search;
                }
            }
        }
        if !joined {
            break;
        }
    }
    chains.sort_by(|a, b| {
        a[0].x
            .partial_cmp(&b[0].x)
            .unwrap()
            .then(a[0].y.partial_cmp(&b[0].y).unwrap())
    });
    chains.into_iter().map(Polyline).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::v2;

    fn node(id: u64, x: f64, y: f64) -> RoadNode {
        RoadNode {
            id: NodeId(id),
            position: v2(x, y),
            kind: NodeKind::Junction,
        }
    }

    fn edge(a: u64, b: u64, pieces: Vec<Vec<V2>>) -> RoadEdge {
        RoadEdge {
            id: EdgeId::between(NodeId(a), NodeId(b), 0),
            a: NodeId(a),
            b: NodeId(b),
            class: RoadClass::Collector,
            setting: Setting::Rural,
            spans: vec![],
            source: EdgeSource::Generated,
            pieces: pieces.into_iter().map(Polyline).collect(),
        }
    }

    #[test]
    fn an_edge_id_does_not_depend_on_which_end_you_start_from() {
        let (a, b) = (NodeId(1), NodeId(2));
        assert_eq!(EdgeId::between(a, b, 0), EdgeId::between(b, a, 0));
        assert_ne!(EdgeId::between(a, b, 0), EdgeId::between(a, b, 1));
        assert_ne!(EdgeId::between(a, b, 0), EdgeId::between(a, NodeId(3), 0));
    }

    #[test]
    fn two_tiles_that_share_a_road_merge_into_one_edge_with_one_line() {
        let cell = Cell::new(4, 0, 0);
        let left = RoadTile {
            cell,
            nodes: vec![node(1, 0.0, 0.0), node(2, 200.0, 0.0)],
            edges: vec![edge(
                1,
                2,
                vec![vec![v2(0.0, 0.0), v2(60.0, 4.0), v2(100.0, 5.0)]],
            )],
        };
        let right = RoadTile {
            cell: cell.neighbour(1, 0),
            nodes: vec![node(1, 0.0, 0.0), node(2, 200.0, 0.0)],
            edges: vec![edge(
                1,
                2,
                vec![vec![v2(100.0, 5.0), v2(150.0, 2.0), v2(200.0, 0.0)]],
            )],
        };
        // The order tiles arrive in must not matter.
        for tiles in [[&left, &right], [&right, &left]] {
            let net = RoadNetwork::assemble(tiles).unwrap();
            assert_eq!(net.nodes.len(), 2);
            assert_eq!(net.edges.len(), 1);
            let line = &net.edges.values().next().unwrap().pieces;
            assert_eq!(line.len(), 1, "the two halves join into one line");
            assert_eq!(line[0].0.len(), 5);
            assert_eq!(line[0].first(), Some(v2(0.0, 0.0)));
            assert_eq!(line[0].last(), Some(v2(200.0, 0.0)));
        }
    }

    #[test]
    fn a_seam_bug_is_reported_not_papered_over() {
        let cell = Cell::new(4, 0, 0);
        let one = RoadTile {
            cell,
            nodes: vec![node(1, 0.0, 0.0)],
            edges: vec![],
        };
        let other = RoadTile {
            cell,
            nodes: vec![node(1, 3.0, 0.0)],
            edges: vec![],
        };
        assert_eq!(
            RoadNetwork::assemble([&one, &other]).unwrap_err(),
            Conflict::Node(NodeId(1))
        );

        let mut a = edge(1, 2, vec![]);
        let mut b = edge(1, 2, vec![]);
        b.class = RoadClass::Local;
        a.class = RoadClass::Collector;
        let nodes = vec![node(1, 0.0, 0.0), node(2, 9.0, 0.0)];
        let t1 = RoadTile {
            cell,
            nodes: nodes.clone(),
            edges: vec![a],
        };
        let t2 = RoadTile {
            cell,
            nodes,
            edges: vec![b],
        };
        assert!(matches!(
            RoadNetwork::assemble([&t1, &t2]),
            Err(Conflict::Edge(_))
        ));

        let dangling = RoadTile {
            cell,
            nodes: vec![node(1, 0.0, 0.0)],
            edges: vec![edge(1, 2, vec![])],
        };
        assert!(matches!(
            RoadNetwork::assemble([&dangling]),
            Err(Conflict::MissingNode(..))
        ));
    }

    #[test]
    fn degree_counts_the_roads_at_a_node() {
        let cell = Cell::new(1, 0, 0);
        let tile = RoadTile {
            cell,
            nodes: vec![node(1, 0.0, 0.0), node(2, 1.0, 0.0), node(3, 0.0, 1.0)],
            edges: vec![edge(1, 2, vec![]), edge(1, 3, vec![])],
        };
        let net = RoadNetwork::assemble([&tile]).unwrap();
        assert_eq!(net.degree(NodeId(1)), 2);
        assert_eq!(net.degree(NodeId(2)), 1);
    }
}
