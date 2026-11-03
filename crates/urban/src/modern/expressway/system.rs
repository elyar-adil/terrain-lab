//! System interchanges (系统立交): where one expressway meets another.
//!
//! No street, signal or stop line is in one: every ramp runs from one deck to
//! the other, leaving and joining each on its kerb side in the air. The second
//! expressway (`X`) flies over the first (`M`): near the crossing it rises to
//! the upper deck level (18 m, `VIADUCT2_Y` in city-scene), and `M` runs on at
//! the lower (9 m) beneath it. Every ramp joins the two decks, so each is a
//! slope between the levels.
//!
//! The plan here is the cloverleaf (苜蓿叶式, the reference diagram's first
//! form): in each of the four quadrants an outer ramp for the right turn,
//! swinging out wide from one deck to the other, and a loop for the left turn,
//! turning 270 degrees tight against the crossing.
//!
//! Everything is laid out in the frame of the crossing: `x` along `M`, `y`
//! along `X`, the two exactly square, so the four quadrants are one drawing.

use super::*;

/// Where the outer ramps leave and join the decks, from the crossing.
const OUTER_M: f32 = 420.0;
/// A loop's radius, and the straight it runs in on.
const LOOP_R: f32 = 50.0;
const LOOP_LEAD_M: f32 = 30.0;
/// Where a collector-distributor road runs beside the lower deck: its
/// centreline this far from the deck's axis (the deck's edge, a ramp's width of
/// its own, and a barrier's gap), and how far past the outer ramps it parts
/// from and rejoins the deck.
const CD_LAT_M: f32 = 31.0;
const CD_RUN_M: f32 = 150.0;
const CD_EASE_M: f32 = 120.0;
/// `X` stays on the upper level this far past its last ramp, which gives the
/// slope down to the lower level the length its 5 % grade needs.
const SLOPE_M: f32 = 230.0;
/// How far the outer ramps run from the crossing, at most.
const REACH_M: f32 = OUTER_M + 120.0 + SLOPE_M + 60.0;

/// What the second pass needs of a system interchange.
#[derive(Debug, Clone)]
pub(super) struct SystemSite {
    /// The crossing's node.
    pub node: usize,
    pub centre: V,
    /// The directions of the lower and the upper expressway.
    pub m_dir: V,
    pub x_dir: V,
    /// How far along `X`, either way, the upper level holds.
    pub half: f32,
}

/// One ramp, deck to deck: (axis, station along it from the crossing) at each
/// end, axis 0 being `M`, 1 `X`, and 2 and 3 the collector-distributor roads
/// beside `M` (on its two sides), and its line.
struct SysRamp {
    from: (usize, f32),
    to: (usize, f32),
    line: Vec<V>,
}

struct SystemPlan {
    ramps: Vec<SysRamp>,
    /// Collector-distributor roads beside the lower deck.
    cd: bool,
    /// The lower deck's ramp offset, and a collector-distributor road's.
    abut: f32,
    cd_lat: f32,
    /// `X`'s upper level ends at these stations (one each way).
    ends: [f32; 2],
}

fn rot90(v: V) -> V {
    (-v.1, v.0)
}

fn rotate(v: V, degrees: f32) -> V {
    let (s, c) = degrees.to_radians().sin_cos();
    (v.0 * c - v.1 * s, v.0 * s + v.1 * c)
}

/// The two ramps of one quadrant (`qx`, `qy` are the signs of its corner in the
/// crossing's frame): the outer ramp and the loop.
///
/// `A` is the deck whose traffic comes toward the crossing in this quadrant, `B`
/// the one whose goes away: the outer ramp leaves `A` and joins `B`, the loop
/// leaves `B` and joins `A`.
fn quadrant(lat_m: f32, lat_x: f32, qx: f32, qy: f32, cd: bool) -> [SysRamp; 2] {
    // Unit vectors from the crossing along each deck into this quadrant.
    let (e_a, e_b, axis_a, axis_b) = if qx == -qy {
        ((qx, 0.0), (0.0, qy), 0, 1)
    } else {
        ((0.0, qy), (qx, 0.0), 1, 0)
    };
    let lat = |axis: usize| if axis == 0 { lat_m } else { lat_x };
    let (lat_a, lat_b) = (lat(axis_a), lat(axis_b));
    let at = |a: f32, b: f32| add(mul(e_a, a), mul(e_b, b));
    // A station on an axis, signed along that axis's own positive direction;
    // the lower deck's ends are on its collector-distributor road, one a side.
    let end = |axis: usize, along: f32| {
        let positive = if axis == 0 { (1.0, 0.0) } else { (0.0, 1.0) };
        let e = if axis == axis_a { e_a } else { e_b };
        let which = if axis == 0 && cd {
            if qy > 0.0 { 2 } else { 3 }
        } else {
            axis
        };
        (which, dot(e, positive) * along)
    };
    // The outer ramp, a quarter turn wide of the loop.
    let k = 0.55 * (OUTER_M - 0.5 * (lat_a + lat_b));
    let (p0, p3) = (at(OUTER_M, lat_a), at(lat_b, OUTER_M));
    let (p1, p2) = (sub(p0, mul(e_a, k)), sub(p3, mul(e_b, k)));
    let steps = 40;
    let outer: Vec<V> = (0..=steps)
        .map(|i| {
            let t = i as f32 / steps as f32;
            let u = 1.0 - t;
            let w = |a: f32, b: f32, c: f32, d: f32| {
                u * u * u * a + 3.0 * u * u * t * b + 3.0 * u * t * t * c + t * t * t * d
            };
            (w(p0.0, p1.0, p2.0, p3.0), w(p0.1, p1.1, p2.1, p3.1))
        })
        .collect();
    // The loop: out of `B` at station `lat_a + r`, round 270 degrees about a
    // centre `r` toward `A`, and in along `A` over a straight lead.
    let (start_b, end_a) = (lat_a + LOOP_R, lat_b + LOOP_R - LOOP_LEAD_M);
    let start = at(lat_b, start_b);
    let centre = add(start, mul(e_a, LOOP_R));
    let mut loop_line = vec![start];
    for step in 1..=18 {
        let theta = 15.0 * step as f32;
        let spoke = rotate(mul(e_a, -LOOP_R), theta);
        loop_line.push(add(centre, spoke));
    }
    let last = *loop_line.last().expect("a loop has points");
    loop_line.push(add(last, mul(e_a, -LOOP_LEAD_M)));
    [
        SysRamp {
            from: end(axis_a, OUTER_M),
            to: end(axis_b, OUTER_M),
            line: outer,
        },
        SysRamp {
            from: end(axis_b, start_b),
            to: end(axis_a, end_a),
            line: loop_line,
        },
    ]
}

/// Where along `X`'s chain the upper level ends, if there is room: at one of
/// its nodes at least `min` from the crossing, or at a cut of its own, with the
/// slope down to the next node long enough.
fn upper_end(chain: &Chain, side: f32) -> Option<f32> {
    let from = OUTER_M + 120.0;
    let nodes: Vec<f32> = chain
        .nodes
        .iter()
        .map(|(s, _)| side * (s - chain.at))
        .filter(|d| *d > 1.0)
        .collect();
    let total = if side > 0.0 {
        polyline_len(&chain.line) - chain.at
    } else {
        chain.at
    };
    let next_after = |c: f32| {
        nodes
            .iter()
            .copied()
            .filter(|d| *d > c + 1.0)
            .fold(f32::MAX, f32::min)
    };
    // A node of the chain that has the slope after it.
    for &c in nodes.iter().filter(|d| **d >= from && **d <= from + 400.0) {
        if next_after(c) - c >= SLOPE_M || (next_after(c) == f32::MAX && total - c >= SLOPE_M) {
            return Some(c);
        }
    }
    // Or a cut of its own where nothing lies in the stretch.
    let c = OUTER_M + 140.0;
    if next_after(c - 60.0) - c >= SLOPE_M && total - c >= SLOPE_M + 20.0 {
        return Some(c);
    }
    None
}

/// Plan the system interchange at `node`, where `M` runs along `m_dir` and `X`
/// across it.
fn plan_system(
    graph: &Graph,
    node: usize,
    m_dir: V,
    half_deck: f32,
    cd: bool,
) -> Option<SystemPlan> {
    let x_dir = rot90(m_dir);
    let m = chain_through(graph, node, REACH_M, Some(m_dir))?;
    let x = chain_through(graph, node, REACH_M, Some(x_dir))?;
    let (m_len, x_len) = (polyline_len(&m.line), polyline_len(&x.line));
    let m_reach = OUTER_M + if cd { CD_RUN_M } else { 0.0 } + 100.0;
    if m.at < m_reach
        || m_len - m.at < m_reach
        || x.at < OUTER_M + SLOPE_M + 100.0
        || x_len - x.at < OUTER_M + SLOPE_M + 100.0
    {
        return None;
    }
    // The chains' own orientation against the crossing's axes.
    let along_x = dot(sample(&x.line, x.at).1, x_dir).signum();
    // Station along the chain is `at + along * local`: the upper level's
    // ends, either way along `X`'s own positive direction.
    let toward = upper_end(&x, along_x)?;
    let against = upper_end(&x, -along_x)?;
    let abut = half_deck + RAMP_HALF_M;
    let mut ramps = Vec::new();
    let lat_m = if cd { CD_LAT_M.max(abut + 9.0) } else { abut };
    for qx in [-1.0_f32, 1.0] {
        for qy in [-1.0_f32, 1.0] {
            ramps.extend(quadrant(lat_m, abut, qx, qy, cd));
        }
    }
    Some(SystemPlan {
        ramps,
        cd,
        abut,
        cd_lat: lat_m,
        ends: [toward, -against],
    })
}

/// The node on the chain through `node` along `dir` at `place`: one that is
/// there, or a new one cut in.
fn deck_node(graph: &mut Graph, node: usize, dir: V, place: V, role: &str) -> Option<usize> {
    let chain = chain_through(graph, node, REACH_M, Some(dir))?;
    let station = project(&chain.line, place).0;
    if let Some(&(_, n)) = chain.nodes.iter().find(|(s, _)| (s - station).abs() < 1.0) {
        return Some(n);
    }
    cut_chain(graph, &chain, station, role)
}

/// Build a planned system interchange into the graph.
fn build(graph: &mut Graph, node: usize, m_dir: V, plan: SystemPlan, layout: &mut Layout) -> bool {
    let centre = graph.pts[node];
    let x_dir = rot90(m_dir);
    let axes = [m_dir, x_dir];
    let world = |p: V| add(centre, add(mul(m_dir, p.0), mul(x_dir, p.1)));
    let on_axis = |axis: usize, station: f32| add(centre, mul(axes[axis], station));
    let mut nodes: HashMap<(usize, i32), usize> = HashMap::new();
    let key = |axis: usize, station: f32| (axis, (station * 4.0).round() as i32);
    // The upper level's ends first, then every deck node a ramp joins, the
    // farther ones first so the chains' pieces hold for the nearer.
    let mut wanted: Vec<(usize, f32)> = Vec::new();
    for end in plan.ends {
        wanted.push((1, end));
    }
    let cd_end = OUTER_M + CD_RUN_M;
    if plan.cd {
        wanted.extend([(0, -cd_end), (0, cd_end)]);
    }
    for r in &plan.ramps {
        for (axis, station) in [r.from, r.to] {
            if axis < 2 {
                wanted.push((axis, station));
            }
        }
    }
    wanted.sort_by(|a, b| b.1.abs().total_cmp(&a.1.abs()));
    for (axis, station) in wanted {
        if nodes.contains_key(&key(axis, station)) {
            continue;
        }
        let Some(n) = deck_node(
            graph,
            node,
            axes[axis],
            on_axis(axis, station),
            "interchange",
        ) else {
            return false;
        };
        nodes.insert(key(axis, station), n);
    }
    // The collector-distributor roads' own nodes, where ramps join them.
    for r in &plan.ramps {
        for (end, (axis, station)) in [(0, r.from), (1, r.to)] {
            if axis < 2 || nodes.contains_key(&key(axis, station)) {
                continue;
            }
            let p = if end == 0 {
                r.line[0]
            } else {
                r.line[r.line.len() - 1]
            };
            let n = graph.node(world(p), "ramp");
            nodes.insert(key(axis, station), n);
        }
    }
    // The collector-distributor roads: one each side of the lower deck, from
    // the deck, along beside it past every ramp, and back onto it.
    if plan.cd {
        let lat = |x: f32| {
            let ease =
                smootherstep((x + cd_end) / CD_EASE_M) * smootherstep((cd_end - x) / CD_EASE_M);
            plan.abut + (plan.cd_lat - plan.abut) * ease
        };
        for axis in [2_usize, 3] {
            let side = if axis == 2 { 1.0 } else { -1.0 };
            // Travel runs with +x on the +y side, against it on the other.
            let sign = side;
            let mut stops: Vec<(f32, usize)> = nodes
                .iter()
                .filter(|((a, _), _)| *a == axis)
                .map(|((_, k), n)| (*k as f32 / 4.0, *n))
                .collect();
            stops.push((-cd_end, nodes[&key(0, -cd_end)]));
            stops.push((cd_end, nodes[&key(0, cd_end)]));
            stops.sort_by(|a, b| (sign * a.0).total_cmp(&(sign * b.0)));
            for w in stops.windows(2) {
                let ((x0, from), (x1, to)) = (w[0], w[1]);
                let pieces = ((x1 - x0).abs() / 15.0).ceil().max(1.0) as usize;
                let line: Vec<V> = (0..=pieces)
                    .map(|k| {
                        let x = x0 + (x1 - x0) * k as f32 / pieces as f32;
                        world((x, side * lat(x)))
                    })
                    .collect();
                let crossings = graph.lay_ramp(from, to, &line, &mut layout.tapers);
                layout.detach.extend(crossings);
            }
        }
    }
    for r in &plan.ramps {
        let (Some(&from), Some(&to)) = (
            nodes.get(&key(r.from.0, r.from.1)),
            nodes.get(&key(r.to.0, r.to.1)),
        ) else {
            continue;
        };
        let line: Vec<V> = r.line.iter().map(|p| world(*p)).collect();
        let crossings = graph.lay_ramp(from, to, &line, &mut layout.tapers);
        layout.detach.extend(crossings);
    }
    layout.systems.push(SystemSite {
        node,
        centre,
        m_dir,
        x_dir,
        half: plan.ends[0].abs().max(plan.ends[1].abs()),
    });
    true
}

/// Add a node where the two expressways cross at `p`, in both.
fn cross_at(graph: &mut Graph, p: V) -> Option<usize> {
    let hits: Vec<(usize, f32)> = graph
        .roads
        .iter()
        .enumerate()
        .filter(|(_, r)| main_line(r) && r.line.len() >= 2 && r.from != usize::MAX)
        .filter_map(|(i, r)| {
            let (station, lateral) = project(&r.line, p);
            let total = polyline_len(&r.line);
            (lateral.abs() < 1.5 && station > 6.0 && station < total - 6.0).then_some((i, station))
        })
        .collect();
    if hits.len() != 2 {
        return None;
    }
    let node = graph.node(p, "system");
    for (index, station) in hits {
        graph.split(index, vec![(station, node)]);
    }
    Some(node)
}

/// Lay a system interchange where the expressway along `line` is crossed by a
/// second, and say where. The crossing nearest the town's middle that has the
/// room is the one.
pub(super) fn lay(
    graph: &mut Graph,
    layout: &mut Layout,
    line: &[V],
    reach: f32,
    half_deck: f32,
) -> Option<V> {
    let (first, last) = (line[0], line[line.len() - 1]);
    let total = len(sub(last, first));
    let m_dir = unit(sub(last, first));
    let n = rot90(m_dir);
    let margin = OUTER_M + 220.0;
    let mut stations: Vec<f32> = Vec::new();
    let mut s = margin;
    while s < total - margin {
        stations.push(s);
        s += 60.0;
    }
    stations.sort_by(|a, b| {
        len(add(first, mul(m_dir, *a))).total_cmp(&len(add(first, mul(m_dir, *b))))
    });
    let half = (reach * 1.3).max(REACH_M + 400.0);
    // A collector-distributor form where the town's coordinates say so, else
    // the plain one, and either given up for the other where there is no room.
    let wants_cd = ((first.0 * 0.37 + first.1 * 0.61).abs() as i64) % 2 == 0;
    for s in stations {
        let p = add(first, mul(m_dir, s));
        let x_line = vec![add(p, mul(n, -half)), add(p, mul(n, half))];
        let mut trial = graph.clone();
        insert_corridor(&mut trial, &x_line);
        let Some(node) = cross_at(&mut trial, p) else {
            continue;
        };
        let Some(plan) = plan_system(&trial, node, m_dir, half_deck, wants_cd)
            .or_else(|| plan_system(&trial, node, m_dir, half_deck, !wants_cd))
        else {
            continue;
        };
        // Real now: laid into the graph itself, the same way.
        insert_corridor(graph, &x_line);
        let node = cross_at(graph, p)?;
        if !build(graph, node, m_dir, plan, layout) {
            return None;
        }
        return Some(p);
    }
    None
}
