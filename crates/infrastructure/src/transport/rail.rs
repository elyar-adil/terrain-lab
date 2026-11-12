//! Railway planning. See `docs/transport-infrastructure.md` section 2.
//!
//! Route: least-cost path on a coarse grid (cost from slope, wetness, water,
//! settlement avoidance) between station sites; corners rounded to the class's
//! minimum radius; a design grade line fitted under the ruling grade; the
//! difference to the ground decides embankment, cutting, viaduct, bridge and
//! tunnel; road crossings are separated or protected by class.

use super::geometry::{self as g, P, Pin};
use super::{SEA_LEVEL_M, World};
use crate::{RoadClass, SettlementClass, SettlementSite, settlement_radius_km};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::BinaryHeap};
use urban::{Point, RailCorridor, RailGround, RailKind, RailStationPlan, RailVertex};
use world_core::WorldGrid;

pub const STEP_M: f32 = 25.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RailPoint {
    pub x_km: f32,
    pub y_km: f32,
    pub s_m: f32,
    pub ground_m: f32,
    pub design_m: f32,
    pub ground: RailGround,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RailStructure {
    pub kind: RailGround,
    pub start_m: f32,
    pub end_m: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RailStation {
    pub settlement: u32,
    pub plan: RailStationPlan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CrossingTreatment {
    /// Barriers, bells and lights, crossing panel (conventional lines only).
    LevelProtected,
    /// The line passes over the road on a structure.
    RailOverRoad,
    /// The line is in tunnel or a deep cutting under the road.
    RoadOverRail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RailRoadCrossing {
    pub road_id: u32,
    pub s_m: f32,
    pub x_km: f32,
    pub y_km: f32,
    pub treatment: CrossingTreatment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Railway {
    pub id: u32,
    pub kind: RailKind,
    pub from_settlement: u32,
    pub to_settlement: u32,
    pub length_km: f32,
    pub points: Vec<RailPoint>,
    pub structures: Vec<RailStructure>,
    pub stations: Vec<RailStation>,
    pub crossings: Vec<RailRoadCrossing>,
    /// Smallest radius found on the finished centreline, metres.
    pub min_radius_m: f32,
    /// Steepest grade of the design profile, metres per metre.
    pub max_grade: f32,
    /// Length of line worked at the class mountain grade, metres.
    #[serde(default)]
    pub mountain_m: f32,
}

impl Railway {
    pub fn point_xy_m(&self, i: usize) -> P {
        [self.points[i].x_km * 1000.0, self.points[i].y_km * 1000.0]
    }

    /// Share of the length on each kind of ground.
    pub fn share(&self, kind: RailGround) -> f32 {
        let total = self.points.last().map_or(1.0, |p| p.s_m).max(1.0);
        let mut sum = 0.0;
        for w in self.points.windows(2) {
            if w[0].ground == kind {
                sum += w[1].s_m - w[0].s_m;
            }
        }
        sum / total
    }

    /// The parts of the line within `radius_km` of a point, as corridors for a
    /// town's plan (each continuous run, padded by one vertex).
    pub fn corridors_near(&self, centre_km: [f32; 2], radius_km: f32) -> Vec<RailCorridor> {
        let inside: Vec<bool> = self
            .points
            .iter()
            .map(|p| (p.x_km - centre_km[0]).hypot(p.y_km - centre_km[1]) <= radius_km)
            .collect();
        let mut out = Vec::new();
        let mut i = 0;
        while i < self.points.len() {
            if !inside[i] {
                i += 1;
                continue;
            }
            let mut j = i;
            while j + 1 < self.points.len() && inside[j + 1] {
                j += 1;
            }
            let a = i.saturating_sub(1);
            let b = (j + 1).min(self.points.len() - 1);
            let vertices: Vec<RailVertex> = self.points[a..=b]
                .iter()
                .map(|p| RailVertex {
                    point: Point {
                        x_km: p.x_km,
                        y_km: p.y_km,
                    },
                    s_m: p.s_m,
                    above_ground_m: p.design_m - p.ground_m,
                    ground: p.ground,
                })
                .collect();
            if vertices.len() >= 2 {
                let (s0, s1) = (vertices[0].s_m, vertices[vertices.len() - 1].s_m);
                let stations = self
                    .stations
                    .iter()
                    .filter(|st| st.plan.s_m >= s0 && st.plan.s_m <= s1)
                    .map(|st| st.plan)
                    .collect();
                out.push(RailCorridor {
                    railway_id: self.id,
                    kind: self.kind,
                    vertices,
                    stations,
                });
            }
            i = j + 1;
        }
        out
    }
}

// --- routing ---------------------------------------------------------------

/// The grade routes are planned for: the conventional lines' mountain grade
/// (high-speed lines are steeper-capable, so a route that suits them suits
/// both). Above the height a line at this grade can reach is tunnel.
const ROUTE_GRADE: f32 = 0.025;
/// A cell of tunnel costs this many cells of open line on level ground...
const TUNNEL_CELL_COST: f32 = 6.0;
/// ...plus one more for every this many metres of rock above the line, so a
/// lower pass wins over a shorter crossing under the high peaks.
const TUNNEL_COST_DEPTH_M: f32 = 100.0;

struct Router {
    n: usize,
    cell_m: f32,
    cost: Vec<f32>,
    height: Vec<f32>,
}

#[derive(Clone, Copy)]
struct Node {
    f: f32,
    state: usize,
}
impl PartialEq for Node {
    fn eq(&self, o: &Self) -> bool {
        self.state == o.state && self.f.to_bits() == o.f.to_bits()
    }
}
impl Eq for Node {}
impl PartialOrd for Node {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Node {
    fn cmp(&self, o: &Self) -> Ordering {
        o.f.total_cmp(&self.f)
            .then_with(|| o.state.cmp(&self.state))
    }
}

/// A least-cost route between two points of the world (metres) over the
/// railway cost surface, used for roads that serve transport sites.
pub fn route_between(world: &World, urban_land: &[f32], from: P, to: P) -> Option<Vec<P>> {
    let router = Router::new(world, urban_land, &[]);
    router.route(from, to, None, None, ROUTE_GRADE)
}

impl Router {
    fn new(world: &World, urban_land: &[f32], nodes: &[StationNode]) -> Self {
        let n = world.n.min(320);
        let cell_m = world.size_m / (n - 1) as f32;
        let mut cost = vec![1.0_f32; n * n];
        let mut height = vec![0.0_f32; n * n];
        let tr = world.terrain;
        let last = (world.n - 1) as f32;
        for y in 0..n {
            for x in 0..n {
                let (px, py) = (x as f32 * cell_m, y as f32 * cell_m);
                let i = y * n + x;
                height[i] = world.height(px, py);
                let slope = world.slope(px, py, cell_m);
                // Sea (with a margin of two cells) and lakes are not buildable.
                let mut c = 1.0 + 60.0 * (slope - 0.012).max(0.0);
                let gi = ((py / world.size_m * last).round() as usize) * world.n
                    + (px / world.size_m * last).round() as usize;
                if height[i] < SEA_LEVEL_M + 1.0 || tr.lake[gi] > 0.3 {
                    c = 1.0e4;
                } else {
                    c += tr.wetland[gi] * 6.0 + tr.floodplain[gi] * 1.5 + tr.forest[gi] * 0.25;
                    if tr.river_order[gi] >= 2 || tr.water[gi] > 0.35 {
                        c += 10.0;
                    }
                    if !nodes
                        .iter()
                        .any(|s| g::dist([px, py], s.centre) < s.radius_m * 1.3)
                    {
                        c += 6.0 * urban_land[gi].clamp(0.0, 1.0);
                    }
                }
                cost[i] = c;
            }
        }
        // Dilate the forbidden mask by two cells.
        let forbidden: Vec<bool> = cost.iter().map(|c| *c >= 1.0e4).collect();
        for y in 0..n {
            for x in 0..n {
                if forbidden[y * n + x] {
                    continue;
                }
                'o: for dy in -2_i32..=2 {
                    for dx in -2_i32..=2 {
                        let (xx, yy) = (x as i32 + dx, y as i32 + dy);
                        if xx >= 0
                            && yy >= 0
                            && (xx as usize) < n
                            && (yy as usize) < n
                            && forbidden[yy as usize * n + xx as usize]
                        {
                            cost[y * n + x] = cost[y * n + x].max(40.0);
                            break 'o;
                        }
                    }
                }
            }
        }
        Self {
            n,
            cell_m,
            cost,
            height,
        }
    }

    fn cell(&self, p: P) -> (usize, usize) {
        let c = |v: f32| ((v / self.cell_m).round().max(0.0) as usize).min(self.n - 1);
        (c(p[0]), c(p[1]))
    }

    fn cost_at(&self, p: P) -> f32 {
        let (x, y) = self.cell(p);
        self.cost[y * self.n + x]
    }

    /// Least-cost path of cell centres with a turning penalty (rail curves).
    ///
    /// Ground higher than a line at `grade` can climb to from either end
    /// (straight-line distance, so an upper bound) can only be passed in
    /// tunnel; its cost grows with that excess height, which steers the
    /// line to low passes and around ranges instead of under them.
    fn route(
        &self,
        from: P,
        to: P,
        dir0: Option<P>,
        dir1: Option<P>,
        grade: f32,
    ) -> Option<Vec<P>> {
        let (sx, sy) = self.cell(from);
        let (ex, ey) = self.cell(to);
        let n = self.n;
        let (hs, he) = (self.height[sy * n + sx], self.height[ey * n + ex]);
        let reach = |cell: usize| -> f32 {
            let (cx, cy) = ((cell % n) as f32, (cell / n) as f32);
            let d0 = (cx - sx as f32).hypot(cy - sy as f32) * self.cell_m;
            let d1 = (cx - ex as f32).hypot(cy - ey as f32) * self.cell_m;
            (hs + grade * d0).min(he + grade * d1)
        };
        let states = 9;
        let dirs: [(i32, i32); 8] = [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];
        let mut dist = vec![f32::INFINITY; n * n * states];
        let mut prev = vec![usize::MAX; n * n * states];
        let unit_dirs: Vec<P> = dirs
            .iter()
            .map(|(x, y)| g::unit([*x as f32, *y as f32]))
            .collect();
        let nearest = |d: P| -> usize {
            (0..8)
                .max_by(|a, b| {
                    let da = unit_dirs[*a][0] * d[0] + unit_dirs[*a][1] * d[1];
                    let db = unit_dirs[*b][0] * d[0] + unit_dirs[*b][1] * d[1];
                    da.total_cmp(&db)
                })
                .unwrap()
        };
        let start_dir = dir0.map_or(8, nearest);
        let start = (sy * n + sx) * states + start_dir;
        dist[start] = 0.0;
        let mut q = BinaryHeap::new();
        q.push(Node {
            f: 0.0,
            state: start,
        });
        let mut end_state = usize::MAX;
        while let Some(node) = q.pop() {
            let cell = node.state / states;
            let din = node.state % states;
            if node.f - self.h(cell, ex, ey) > dist[node.state] + 1.0e-3 {
                continue;
            }
            if cell == ey * n + ex {
                let aligned = dir1.is_none_or(|d| {
                    din < 8 && unit_dirs[din][0] * d[0] + unit_dirs[din][1] * d[1] >= 0.85
                });
                if aligned {
                    end_state = node.state;
                    break;
                }
            }
            let (cx, cy) = ((cell % n) as i32, (cell / n) as i32);
            for (d, (ox, oy)) in dirs.iter().enumerate() {
                let (x, y) = (cx + ox, cy + oy);
                if x < 0 || y < 0 || x >= n as i32 || y >= n as i32 {
                    continue;
                }
                let turn = if din == 8 {
                    0
                } else {
                    let diff = din.abs_diff(d);
                    diff.min(8 - diff)
                };
                if turn > 2 {
                    continue;
                }
                let ncell = y as usize * n + x as usize;
                let len = if *ox != 0 && *oy != 0 { 1.414 } else { 1.0 };
                let c0 = self.cost[cell];
                let c1 = self.cost[ncell];
                let run = len * self.cell_m;
                let step_grade = ((self.height[ncell] - self.height[cell]) / run).abs();
                let over = self.height[ncell] - reach(ncell);
                let ground = if over > TUNNEL_DEPTH_M {
                    // Only in tunnel: the surface (its slopes, valleys and
                    // land) no longer matters, the tunnel's length and depth do.
                    len * (TUNNEL_CELL_COST + over / TUNNEL_COST_DEPTH_M)
                } else {
                    len * 0.5 * (c0 + c1) + 90.0 * (step_grade - 0.012).max(0.0) * len
                };
                let step = ground + (turn * turn) as f32 * 2.2;
                let ns = ncell * states + d;
                let t = dist[node.state] + step;
                if t < dist[ns] {
                    dist[ns] = t;
                    prev[ns] = node.state;
                    q.push(Node {
                        f: t + self.h(ncell, ex, ey),
                        state: ns,
                    });
                }
            }
        }
        if end_state == usize::MAX {
            return None;
        }
        if std::env::var("RAIL_DEBUG").is_ok() {
            eprintln!(
                "route ({sx},{sy})->({ex},{ey}) cost {} hs {hs} he {he}",
                dist[end_state]
            );
        }
        let mut path = vec![end_state / states];
        let mut cur = end_state;
        while cur != start {
            cur = prev[cur];
            path.push(cur / states);
        }
        path.reverse();
        Some(
            path.into_iter()
                .map(|c| [(c % n) as f32 * self.cell_m, (c / n) as f32 * self.cell_m])
                .collect(),
        )
    }

    /// Least travel cost from `from` to every cell over the open-line cost
    /// surface (no curvature or tunnel model): ranks which towns are near each
    /// other for a line, so a range between them counts and a plain does not.
    fn travel_costs(&self, from: P) -> Vec<f32> {
        let n = self.n;
        let (sx, sy) = self.cell(from);
        let mut dist = vec![f32::INFINITY; n * n];
        let mut q = BinaryHeap::new();
        dist[sy * n + sx] = 0.0;
        q.push(Node {
            f: 0.0,
            state: sy * n + sx,
        });
        while let Some(Node { f, state: cell }) = q.pop() {
            if f > dist[cell] {
                continue;
            }
            let (cx, cy) = ((cell % n) as i32, (cell / n) as i32);
            for oy in -1..=1_i32 {
                for ox in -1..=1_i32 {
                    let (x, y) = (cx + ox, cy + oy);
                    if (ox == 0 && oy == 0) || x < 0 || y < 0 || x >= n as i32 || y >= n as i32 {
                        continue;
                    }
                    let ncell = y as usize * n + x as usize;
                    let len = if ox != 0 && oy != 0 { 1.414 } else { 1.0 };
                    let grade =
                        ((self.height[ncell] - self.height[cell]) / (len * self.cell_m)).abs();
                    let t = f
                        + len * 0.5 * (self.cost[cell] + self.cost[ncell])
                        + 90.0 * (grade - 0.012).max(0.0) * len;
                    if t < dist[ncell] {
                        dist[ncell] = t;
                        q.push(Node { f: t, state: ncell });
                    }
                }
            }
        }
        dist
    }

    fn h(&self, cell: usize, ex: usize, ey: usize) -> f32 {
        let (cx, cy) = ((cell % self.n) as f32, (cell / self.n) as f32);
        (cx - ex as f32).hypot(cy - ey as f32)
    }
}

// --- the network ------------------------------------------------------------

#[derive(Clone)]
struct StationNode {
    settlement_index: usize,
    id: u32,
    class: SettlementClass,
    centre: P,
    radius_m: f32,
}

/// A planned station position: the platform centre, the line's direction of
/// travel through it, and the ends of the straight that holds the platforms.
struct Site {
    node: usize,
    s: P,
    dir: P,
}

pub struct RoadLine {
    pub id: u32,
    pub class: RoadClass,
    pub line_m: Vec<P>,
}

fn perp(d: P) -> P {
    [-d[1], d[0]]
}

fn station_offset(node: &StationNode) -> f32 {
    match node.class {
        SettlementClass::RegionalCentre => 1200.0,
        _ => 750.0,
    }
}

/// Cost of putting a station on one side of the line through its town.
fn side_score(router: &Router, node: &StationNode, dir: P, side: f32) -> f32 {
    let c = g::add_scaled(node.centre, perp(dir), side * station_offset(node));
    (-6..=6)
        .map(|k| router.cost_at(g::add_scaled(c, dir, k as f32 * 250.0)))
        .sum::<f32>()
}

fn site_on_side(node: &StationNode, nidx: usize, dir: P, side: f32) -> Site {
    Site {
        node: nidx,
        s: g::add_scaled(node.centre, perp(dir), side * station_offset(node)),
        dir,
    }
}

fn site_for(router: &Router, node: &StationNode, nidx: usize, dir: P) -> Site {
    let side = if side_score(router, node, dir, -1.0) < side_score(router, node, dir, 1.0) - 1.0e-3
    {
        -1.0
    } else {
        1.0
    };
    site_on_side(node, nidx, dir, side)
}

const STUB_M: f32 = 1800.0;
const TERMINAL_OVERRUN_M: f32 = 700.0;

pub fn plan_railways(
    world: &World,
    grid: WorldGrid,
    settlements: &[SettlementSite],
    roads: &[RoadLine],
    urban_land: &[f32],
) -> Vec<Railway> {
    let nodes: Vec<StationNode> = settlements
        .iter()
        .enumerate()
        .filter(|(_, s)| s.class != SettlementClass::Village)
        .map(|(i, s)| StationNode {
            settlement_index: i,
            id: s.id,
            class: s.class,
            centre: [
                s.location.x as f32 / (grid.size - 1) as f32 * world.size_m,
                s.location.y as f32 / (grid.size - 1) as f32 * world.size_m,
            ],
            radius_m: settlement_radius_km(s.class) * 1000.0,
        })
        .collect();
    if nodes.len() < 2 {
        return Vec::new();
    }
    let router = Router::new(world, urban_land, &nodes);
    // Spanning tree from the regional centre (node 0) by travel cost over the
    // terrain, so towns on the same side of a range join up first and the
    // range is crossed once, where it is cheapest.
    let m = nodes.len();
    let reach: Vec<Vec<f32>> = nodes
        .iter()
        .map(|nd| router.travel_costs(nd.centre))
        .collect();
    let link_cost = |a: usize, b: usize| -> f32 {
        let (bx, by) = router.cell(nodes[b].centre);
        let (ax, ay) = router.cell(nodes[a].centre);
        0.5 * (reach[a][by * router.n + bx] + reach[b][ay * router.n + ax])
    };
    let mut in_tree = vec![false; m];
    in_tree[0] = true;
    let mut edges: Vec<(usize, usize)> = Vec::new();
    while edges.len() + 1 < m {
        let mut best: Option<(f32, usize, usize)> = None;
        for a in (0..m).filter(|a| in_tree[*a]) {
            for b in (0..m).filter(|b| !in_tree[*b]) {
                let d = link_cost(a, b);
                if best.is_none_or(|(bd, _, _)| d < bd) {
                    best = Some((d, a, b));
                }
            }
        }
        let (_, a, b) = best.unwrap();
        in_tree[b] = true;
        edges.push((a, b));
    }
    // Main chain: the longest path in the tree.
    let mut adj = vec![Vec::new(); m];
    for (a, b) in &edges {
        adj[*a].push(*b);
        adj[*b].push(*a);
    }
    let mut best_chain: (f32, Vec<usize>) = (-1.0, Vec::new());
    for s in 0..m {
        fn walk(
            at: usize,
            from: usize,
            adj: &[Vec<usize>],
            nodes: &[StationNode],
            path: &mut Vec<usize>,
            len: f32,
            best: &mut (f32, Vec<usize>),
        ) {
            path.push(at);
            if len > best.0 + 1.0e-3 {
                *best = (len, path.clone());
            }
            for &nx in &adj[at] {
                if nx != from {
                    let d = g::dist(nodes[at].centre, nodes[nx].centre);
                    walk(nx, at, adj, nodes, path, len + d, best);
                }
            }
            path.pop();
        }
        walk(
            s,
            usize::MAX,
            &adj,
            &nodes,
            &mut Vec::new(),
            0.0,
            &mut best_chain,
        );
    }
    let mut chain = best_chain.1;
    // A chain that turns sharply at a town ends there: the rest leaves it as
    // a branch from a junction short of the station (a Y), the way real lines
    // meet at a junction town instead of bending 90 degrees through it.
    for j in 1..chain.len().saturating_sub(1) {
        let d0 = g::unit(g::sub(nodes[chain[j]].centre, nodes[chain[j - 1]].centre));
        let d1 = g::unit(g::sub(nodes[chain[j + 1]].centre, nodes[chain[j]].centre));
        if d0[0] * d1[0] + d0[1] * d1[1] < 0.5 {
            chain.truncate(j + 1);
            break;
        }
    }
    let mut railways: Vec<Railway> = Vec::new();

    // --- the chain ---
    let k = chain.len();
    let dir_at = |j: usize| -> P {
        let prev = if j > 0 { chain[j - 1] } else { chain[0] };
        let next = if j + 1 < k {
            chain[j + 1]
        } else {
            chain[k - 1]
        };
        let (a, b) = if j == 0 {
            (nodes[chain[0]].centre, nodes[chain[1]].centre)
        } else if j + 1 == k {
            (nodes[chain[k - 2]].centre, nodes[chain[k - 1]].centre)
        } else {
            (nodes[prev].centre, nodes[next].centre)
        };
        g::unit(g::sub(b, a))
    };
    // One side of the line for every station of the chain, so the line stays
    // straight between them.
    let chain_side = {
        let total = |side: f32| -> f32 {
            (0..k)
                .map(|j| side_score(&router, &nodes[chain[j]], dir_at(j), side))
                .sum()
        };
        if total(-1.0) < total(1.0) - 1.0e-3 {
            -1.0
        } else {
            1.0
        }
    };
    let sites: Vec<Site> = (0..k)
        .map(|j| site_on_side(&nodes[chain[j]], chain[j], dir_at(j), chain_side))
        .collect();
    let chain_length: f32 = (0..k - 1)
        .map(|j| g::dist(nodes[chain[j]].centre, nodes[chain[j + 1]].centre))
        .sum();
    let has_centre = chain
        .iter()
        .any(|c| nodes[*c].class == SettlementClass::RegionalCentre);
    let kinds: &[RailKind] = if has_centre && chain_length >= 30_000.0 {
        &[RailKind::HighSpeed, RailKind::Mainline, RailKind::Branch]
    } else {
        &[RailKind::Mainline, RailKind::Branch]
    };
    // Vertex list.
    let mut verts: Vec<(P, Pin)> = Vec::new();
    let mut straight: Vec<bool> = Vec::new();
    let mut ok = true;
    for j in 0..k {
        let site = &sites[j];
        let back = if j == 0 { TERMINAL_OVERRUN_M } else { STUB_M };
        let fwd = if j + 1 == k {
            TERMINAL_OVERRUN_M
        } else {
            STUB_M
        };
        let a = g::add_scaled(site.s, site.dir, -back);
        let b = g::add_scaled(site.s, site.dir, fwd);
        if j > 0 {
            let from = *verts.last().map(|v| &v.0).unwrap();
            match router.route(from, a, Some(sites[j - 1].dir), Some(site.dir), ROUTE_GRADE) {
                Some(path) => push_interior(&router, &mut verts, &mut straight, from, a, &path),
                None => {
                    if std::env::var("RAIL_DEBUG").is_ok() {
                        eprintln!("chain: no route between stations {}", j);
                    }
                    ok = false;
                    break;
                }
            }
        }
        verts.push((a, if j == 0 { Pin::Fixed } else { Pin::Fixed }));
        straight.push(true);
        verts.push((b, Pin::Fixed));
        straight.push(false);
    }
    if ok {
        for kind in kinds {
            let stations: Vec<(u32, P, SettlementClass, P, bool, bool)> = (0..k)
                .map(|j| {
                    (
                        nodes[chain[j]].id,
                        sites[j].s,
                        nodes[chain[j]].class,
                        nodes[chain[j]].centre,
                        j == 0 || j + 1 == k,
                        false,
                    )
                })
                .collect();
            if let Some(rw) = finish(
                world,
                &nodes,
                railways.len() as u32 + 1,
                *kind,
                &verts,
                &straight,
                &stations,
                None,
                roads,
                (nodes[chain[0]].id, nodes[chain[k - 1]].id),
            ) {
                railways.push(rw);
                break;
            }
        }
    }

    // --- branches off the chain ---
    let mut built: Vec<usize> = chain.clone();
    let mut progress = true;
    while progress {
        progress = false;
        for &(a, b) in &edges {
            let (parent, child) = if built.contains(&a) && !built.contains(&b) {
                (a, b)
            } else if built.contains(&b) && !built.contains(&a) {
                (b, a)
            } else {
                continue;
            };
            progress = true;
            built.push(child);
            if let Some(rw) = branch(
                world,
                &router,
                &nodes,
                &railways,
                parent,
                child,
                roads,
                railways.len() as u32 + 1,
            ) {
                railways.push(rw);
            }
        }
    }
    railways
}

fn push_interior(
    router: &Router,
    verts: &mut Vec<(P, Pin)>,
    straight: &mut Vec<bool>,
    from: P,
    to: P,
    path: &[P],
) {
    let mut pts = path.to_vec();
    if let Some(first) = pts.first_mut() {
        *first = from;
    }
    if let Some(last) = pts.last_mut() {
        *last = to;
    }
    let simple = g::simplify(&pts, router.cell_m * 1.2);
    for p in simple.iter().skip(1).take(simple.len().saturating_sub(2)) {
        verts.push((*p, Pin::Free));
        straight.push(false);
    }
}

#[allow(clippy::too_many_arguments)]
fn branch(
    world: &World,
    router: &Router,
    nodes: &[StationNode],
    railways: &[Railway],
    parent: usize,
    child: usize,
    roads: &[RoadLine],
    id: u32,
) -> Option<Railway> {
    // The parent's line that serves it.
    let parent_id = nodes[parent].id;
    let rw = railways
        .iter()
        .find(|r| r.stations.iter().any(|s| s.settlement == parent_id))?;
    let st = rw.stations.iter().find(|s| s.settlement == parent_id)?;
    let target = nodes[child].centre;
    // Junction: on the parent line 1.5-4.5 km either side of its station.
    let mut best: Option<(f32, usize)> = None;
    for (i, p) in rw.points.iter().enumerate() {
        let off = (p.s_m - st.plan.s_m).abs();
        if !(1500.0..=4500.0).contains(&off) {
            continue;
        }
        let d = g::dist([p.x_km * 1000.0, p.y_km * 1000.0], target);
        if best.is_none_or(|(bd, _)| d < bd) {
            best = Some((d, i));
        }
    }
    let (_, ji) = best?;
    let jp = rw.point_xy_m(ji);
    // The merge runs 2.4 km along the parent line, on the child's side of it.
    let step = (3000.0 / STEP_M) as usize;
    // Merge on whichever side of the junction the child lies ahead of.
    let pick = |sign: isize| -> usize {
        (ji as isize + sign * step as isize).clamp(0, rw.points.len() as isize - 1) as usize
    };
    let ahead = |ki: usize| -> f32 {
        let k = rw.point_xy_m(ki);
        let d = g::unit(g::sub(k, jp));
        let t = g::unit(g::sub(target, k));
        d[0] * t[0] + d[1] * t[1]
    };
    let ki = if ahead(pick(1)) >= ahead(pick(-1)) {
        pick(1)
    } else {
        pick(-1)
    };
    let kp0 = rw.point_xy_m(ki);
    let along = g::unit(g::sub(kp0, jp));
    let n = perp(along);
    let side = if g::sub(target, jp)[0] * n[0] + g::sub(target, jp)[1] * n[1] >= 0.0 {
        1.0
    } else {
        -1.0
    };
    let kp = g::add_scaled(kp0, n, side * 55.0);
    // Direction of travel into the child's station: from the merge toward it.
    let dir = g::unit(g::sub(target, kp));
    let site = site_for(router, &nodes[child], child, dir);
    let a = g::add_scaled(site.s, site.dir, -STUB_M);
    let b = g::add_scaled(site.s, site.dir, TERMINAL_OVERRUN_M);
    let path = router.route(kp, a, Some(along), Some(site.dir), ROUTE_GRADE)?;
    let mut verts: Vec<(P, Pin)> = vec![(jp, Pin::Fixed), (kp, Pin::Fixed)];
    let mut straight = vec![false, false];
    push_interior(router, &mut verts, &mut straight, kp, a, &path);
    verts.push((a, Pin::Fixed));
    straight.push(true);
    verts.push((b, Pin::Fixed));
    straight.push(false);
    let stations = vec![(
        nodes[child].id,
        site.s,
        nodes[child].class,
        nodes[child].centre,
        true,
        false,
    )];
    let start_h = rw.points[ji].design_m;
    finish(
        world,
        nodes,
        id,
        RailKind::Branch,
        &verts,
        &straight,
        &stations,
        Some(start_h),
        roads,
        (parent_id, nodes[child].id),
    )
}

// --- profile and structures -------------------------------------------------

fn moving_average(v: &[f32], half: usize) -> Vec<f32> {
    let n = v.len();
    let mut out = vec![0.0; n];
    let mut prefix = vec![0.0_f64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + f64::from(v[i]);
    }
    for i in 0..n {
        let a = i.saturating_sub(half);
        let b = (i + half + 1).min(n);
        out[i] = ((prefix[b] - prefix[a]) / (b - a) as f64) as f32;
    }
    out
}

/// A line this far under the ground at its ruling grade is crossing a range:
/// the approaches are refitted at the class mountain grade.
const MOUNTAIN_TRIGGER_M: f32 = 60.0;

/// Below this depth under the ground the line is in tunnel, and the raises
/// for rivers, roads and soft ground above it no longer apply.
const TUNNEL_DEPTH_M: f32 = 10.0;

/// Design grade line: the highest line, never steeper than the per-sample
/// grade limit, that stays under the smoothed ground; then raised to the lower
/// bounds where the line is near the surface.
///
/// Fitting under the ground means a climb steeper than the ruling grade is a
/// cutting or tunnel, never an embankment: a line that must cross a 1,700 m
/// pass at 1.2 % goes through a base tunnel instead of standing on a
/// kilometre-high viaduct across the plain on either side.
fn fit_profile(ground: &[f32], lo: &[f32], limit: &[f32], ds: f32, start: Option<f32>) -> Vec<f32> {
    let n = ground.len();
    let lim = |i: usize, j: usize| limit[i].min(limit[j]) * ds;
    // Smoothed, but never above the ground by more than a low bank.
    let mut h: Vec<f32> = moving_average(ground, 32)
        .iter()
        .zip(ground)
        .map(|(m, g)| m.min(g + 3.0))
        .collect();
    // Grade-limited lower envelope (exact in two passes along a line).
    for i in 1..n {
        h[i] = h[i].min(h[i - 1] + lim(i - 1, i));
    }
    for i in (0..n - 1).rev() {
        h[i] = h[i].min(h[i + 1] + lim(i, i + 1));
    }
    // A branch leaves its parent at the junction's level.
    if let Some(h0) = start {
        h[0] = h0;
        for i in 1..n {
            let l = lim(i - 1, i);
            let v = h[i].clamp(h[i - 1] - l, h[i - 1] + l);
            if v == h[i] {
                break;
            }
            h[i] = v;
        }
    }
    // Raises (bridges, road crossings, soft ground) only where the line is not
    // already passing beneath in tunnel; the ramps up to them keep the grade.
    for i in 0..n {
        if lo[i].is_finite() && h[i] > ground[i] - TUNNEL_DEPTH_M {
            h[i] = h[i].max(lo[i]);
        }
    }
    for i in 1..n {
        h[i] = h[i].max(h[i - 1] - lim(i - 1, i));
    }
    for i in (0..n - 1).rev() {
        h[i] = h[i].max(h[i + 1] - lim(i, i + 1));
    }
    h
}

fn seg_intersection(a: P, b: P, c: P, d: P) -> Option<f32> {
    let r = g::sub(b, a);
    let s = g::sub(d, c);
    let den = r[0] * s[1] - r[1] * s[0];
    if den.abs() < 1.0e-9 {
        return None;
    }
    let q = g::sub(c, a);
    let t = (q[0] * s[1] - q[1] * s[0]) / den;
    let u = (q[0] * r[1] - q[1] * r[0]) / den;
    ((0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u)).then_some(t)
}

type StationSpec = (u32, P, SettlementClass, P, bool, bool);

#[allow(clippy::too_many_arguments)]
fn finish(
    world: &World,
    nodes: &[StationNode],
    id: u32,
    kind: RailKind,
    verts: &[(P, Pin)],
    straight: &[bool],
    stations: &[StationSpec],
    start_height: Option<f32>,
    roads: &[RoadLine],
    ends: (u32, u32),
) -> Option<Railway> {
    let limits = kind.limits();
    let curve = g::fillet(
        verts,
        straight,
        limits.min_radius_m,
        (limits.min_radius_m * 1.5).min(9000.0),
    );
    if curve.is_none() && std::env::var("RAIL_DEBUG").is_ok() {
        eprintln!(
            "rail {id} {kind:?}: fillet failed ({} vertices)",
            verts.len()
        );
    }
    let curve = curve?;
    let (pts, s) = g::resample(&curve, STEP_M);
    let n = pts.len();
    if n < 8 {
        return None;
    }
    // The sea and lakes are not crossed.
    if pts
        .iter()
        .any(|p| world.is_sea(p[0], p[1]) || world.is_lake(p[0], p[1]))
    {
        if std::env::var("RAIL_DEBUG").is_ok() {
            eprintln!("rail {id} {kind:?}: line crosses sea or lake");
        }
        return None;
    }
    let ground: Vec<f32> = pts.iter().map(|p| world.height(p[0], p[1])).collect();

    // Lower bounds on the design (where the line must be raised).
    let mut lo = vec![f32::NEG_INFINITY; n];
    let raise = |range: std::ops::Range<usize>, clearance: f32, lo: &mut Vec<f32>| {
        for i in range {
            lo[i] = lo[i].max(ground[i] + clearance);
        }
    };
    let mut bridge = vec![false; n];
    let pad = (75.0 / STEP_M) as usize;
    let mut i = 0;
    while i < n {
        if world.is_river(pts[i][0], pts[i][1]) {
            let mut j = i;
            while j + 1 < n && world.is_river(pts[j + 1][0], pts[j + 1][1]) {
                j += 1;
            }
            let (a, b) = (i.saturating_sub(pad), (j + pad + 1).min(n));
            raise(a..b, 7.0, &mut lo);
            for flag in bridge.iter_mut().take(b).skip(a) {
                *flag = true;
            }
            i = j + 1;
        } else {
            i += 1;
        }
    }
    // Soft ground: a viaduct when the run is long enough to matter.
    let mut i = 0;
    while i < n {
        if world.wetness(pts[i][0], pts[i][1]) > 0.5 {
            let mut j = i;
            while j + 1 < n && world.wetness(pts[j + 1][0], pts[j + 1][1]) > 0.5 {
                j += 1;
            }
            if (j - i) as f32 * STEP_M >= 250.0 {
                raise(i..j + 1, 5.5, &mut lo);
            }
            i = j + 1;
        } else {
            i += 1;
        }
    }
    // A regional centre's built-up area: continuous viaduct, every street passes under.
    let mut elevated_urban = vec![false; n];
    for node in nodes
        .iter()
        .filter(|nd| nd.class == SettlementClass::RegionalCentre)
    {
        for i in 0..n {
            if g::dist(pts[i], node.centre) < node.radius_m * 0.9 {
                elevated_urban[i] = true;
            }
        }
    }
    for i in 0..n {
        if elevated_urban[i] {
            lo[i] = lo[i].max(ground[i] + 9.5);
        }
    }
    // Crossings with roads.
    struct Raw {
        road: usize,
        i: usize,
        point: P,
    }
    let mut raw: Vec<Raw> = Vec::new();
    for (ri, road) in roads.iter().enumerate() {
        let mut last_i = usize::MAX;
        for w in road.line_m.windows(2) {
            let (lo_x, hi_x) = (w[0][0].min(w[1][0]) - 30.0, w[0][0].max(w[1][0]) + 30.0);
            let (lo_y, hi_y) = (w[0][1].min(w[1][1]) - 30.0, w[0][1].max(w[1][1]) + 30.0);
            for i in 0..n - 1 {
                let (a, b) = (pts[i], pts[i + 1]);
                if a[0].max(b[0]) < lo_x
                    || a[0].min(b[0]) > hi_x
                    || a[1].max(b[1]) < lo_y
                    || a[1].min(b[1]) > hi_y
                {
                    continue;
                }
                if let Some(t) = seg_intersection(a, b, w[0], w[1]) {
                    if last_i == usize::MAX || i.abs_diff(last_i) > 4 {
                        raw.push(Raw {
                            road: ri,
                            i,
                            point: [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t],
                        });
                        last_i = i;
                    }
                }
            }
        }
    }
    let must_separate = |class: RoadClass| {
        limits.closed
            || matches!(
                class,
                RoadClass::Motorway | RoadClass::Arterial | RoadClass::Collector
            )
    };
    let mut crossing_bridge = vec![false; n];
    for c in &raw {
        if must_separate(roads[c.road].class) {
            let a = c.i.saturating_sub(2);
            let b = (c.i + 3).min(n);
            for i in a..b {
                lo[i] = lo[i].max(ground[i] + 7.6);
                crossing_bridge[i] = true;
            }
        }
    }

    // Grade limits: the class ruling grade, flat through stations.
    let mut limit = vec![limits.max_grade; n];
    let mut plans: Vec<(u32, RailStationPlan)> = Vec::new();
    for (sid, sp, class, centre, terminal, _) in stations {
        let (mut best, mut bi) = (f32::INFINITY, 0);
        for (i, p) in pts.iter().enumerate() {
            let d = g::dist(*p, *sp);
            if d < best {
                best = d;
                bi = i;
            }
        }
        let s_m = s[bi];
        let plan_len = limits.platform_length_m;
        let half = plan_len * 0.5 + 300.0;
        for i in 0..n {
            if (s[i] - s_m).abs() <= half {
                limit[i] = 0.0025;
            }
        }
        // Side of the building: toward the town centre.
        let d = g::unit(g::sub(pts[(bi + 1).min(n - 1)], pts[bi.saturating_sub(1)]));
        let to_centre = g::sub(*centre, pts[bi]);
        let left = perp(d)[0] * to_centre[0] + perp(d)[1] * to_centre[1] >= 0.0;
        plans.push((
            *sid,
            RailStationPlan {
                s_m,
                platform_length_m: plan_len,
                tracks: if *class == SettlementClass::RegionalCentre {
                    4
                } else {
                    2
                },
                building_side: if left { 1 } else { -1 },
                terminal: *terminal,
                yard: *class == SettlementClass::RegionalCentre,
            },
        ));
    }

    let mut design = fit_profile(&ground, &lo, &limit, STEP_M, start_height);
    // Mountain sections: where the ruling grade leaves the line deep in rock,
    // the climb on either side is worked at the class mountain grade so the
    // summit tunnel is short and high, as a real line over a pass is built.
    let mut mountain = vec![false; n];
    for _ in 0..2 {
        let deep: Vec<usize> = (0..n)
            .filter(|&i| ground[i] - design[i] > MOUNTAIN_TRIGGER_M)
            .collect();
        if deep.is_empty() {
            break;
        }
        for &i in &deep {
            // The approach long enough to climb that depth at the extra grade.
            let gain = (limits.mountain_grade - limits.max_grade).max(1.0e-3);
            let pad = (((ground[i] - design[i]) / gain).clamp(3000.0, 25_000.0) / STEP_M) as usize;
            for flag in mountain
                .iter_mut()
                .take((i + pad + 1).min(n))
                .skip(i.saturating_sub(pad))
            {
                *flag = true;
            }
        }
        for i in 0..n {
            // Stations keep their level platform grade.
            if mountain[i] && limit[i] > limits.max_grade - 1.0e-6 {
                limit[i] = limits.mountain_grade;
            }
        }
        design = fit_profile(&ground, &lo, &limit, STEP_M, start_height);
    }
    let mountain_m = (0..n.saturating_sub(1))
        .filter(|&i| (design[i + 1] - design[i]).abs() > limits.max_grade * STEP_M + 1.0e-3)
        .count() as f32
        * STEP_M;
    let max_grade = design
        .windows(2)
        .map(|w| (w[1] - w[0]).abs() / STEP_M)
        .fold(0.0_f32, f32::max);

    // Structures from design against ground.
    let mut kinds = vec![RailGround::AtGrade; n];
    for i in 0..n {
        let d = design[i] - ground[i];
        // Deep under the ground the river, road or town is above the tunnel.
        let surface = d > -TUNNEL_DEPTH_M;
        kinds[i] = if bridge[i] && surface {
            RailGround::Bridge
        } else if (surface && (crossing_bridge[i] || elevated_urban[i]))
            || d > 9.0
            || (lo[i].is_finite() && d >= 5.0)
        {
            RailGround::Viaduct
        } else if d > 1.0 {
            RailGround::Embankment
        } else if d < -1.5 {
            RailGround::Cutting
        } else {
            RailGround::AtGrade
        };
    }
    // A deep, long cut is a tunnel.
    let mut i = 0;
    while i < n {
        if ground[i] - design[i] > 14.0 {
            let mut j = i;
            while j + 1 < n && ground[j + 1] - design[j + 1] > 10.0 {
                j += 1;
            }
            if (j - i) as f32 * STEP_M >= 300.0 {
                for k in kinds
                    .iter_mut()
                    .take((j + 2).min(n))
                    .skip(i.saturating_sub(2))
                {
                    *k = RailGround::Tunnel;
                }
            }
            i = j + 1;
        } else {
            i += 1;
        }
    }
    let mut structures = Vec::new();
    let mut i = 0;
    while i < n {
        let k = kinds[i];
        let mut j = i;
        while j + 1 < n && kinds[j + 1] == k {
            j += 1;
        }
        if !matches!(
            k,
            RailGround::AtGrade | RailGround::Embankment | RailGround::Cutting
        ) {
            structures.push(RailStructure {
                kind: k,
                start_m: s[i],
                end_m: s[j],
            });
        }
        i = j + 1;
    }

    let crossings: Vec<RailRoadCrossing> = raw
        .iter()
        .map(|c| {
            let above = design[c.i] - ground[c.i];
            RailRoadCrossing {
                road_id: roads[c.road].id,
                s_m: s[c.i],
                x_km: c.point[0] / 1000.0,
                y_km: c.point[1] / 1000.0,
                treatment: if above >= 5.0 {
                    CrossingTreatment::RailOverRoad
                } else if above <= -6.5 {
                    CrossingTreatment::RoadOverRail
                } else {
                    CrossingTreatment::LevelProtected
                },
            }
        })
        .collect();

    let min_radius = g::min_radius(&pts, &s, 100.0);
    let points: Vec<RailPoint> = (0..n)
        .map(|i| RailPoint {
            x_km: pts[i][0] / 1000.0,
            y_km: pts[i][1] / 1000.0,
            s_m: s[i],
            ground_m: ground[i],
            design_m: design[i],
            ground: kinds[i],
        })
        .collect();
    Some(Railway {
        id,
        kind,
        from_settlement: ends.0,
        to_settlement: ends.1,
        length_km: s[n - 1] / 1000.0,
        points,
        structures,
        stations: plans
            .into_iter()
            .map(|(settlement, plan)| RailStation { settlement, plan })
            .collect(),
        crossings,
        min_radius_m: min_radius,
        max_grade,
        mountain_m,
    })
}
