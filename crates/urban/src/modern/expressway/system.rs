//! System interchanges (系统立交): where one expressway meets another.
//!
//! No street, signal or stop line is in one: every ramp runs from one deck to
//! the other, leaving and joining each on its kerb side in the air. The second
//! expressway (`X`) flies over the first (`M`): near the crossing it rises to
//! the upper deck level (18 m, `VIADUCT2_Y` in city-scene), and `M` runs on at
//! the lower (9 m) beneath it. Every ramp joins the two decks, so each is a
//! slope between the levels.
//!
//! The plan is a cloverleaf (苜蓿叶式) or one of its partial forms: in each
//! quadrant an outer ramp for the right turn, swinging out wide from one deck
//! to the other, and a loop for the left turn, turning through more than a
//! half circle tight against the crossing. A cloverleaf is a free-flow form,
//! which is why it is here and not on a street.
//!
//! No two are alike. Each interchange takes its dimensions from the place it
//! is laid at ([`Shape`]): the crossing angle (60 to 90 degrees, so oblique
//! crossings occur), how far out the outer ramps leave their decks, the size of
//! every loop (60 - 90 m, the loops of real freeway cloverleafs), how wide the
//! outer ramps swing, whether collector-distributor roads run beside the lower
//! deck and how long, and which loops are left out (a partial cloverleaf).
//!
//! The two decks are not square, so nothing here is drawn in a square frame:
//! every ramp is laid in the corner between its two decks, with its lateral
//! offsets measured square to each deck, and the stations of its ends read
//! back from the line.

use super::*;

/// Where a collector-distributor road runs beside the lower deck: its
/// centreline this far from the deck's axis (the deck's edge, a ramp's width of
/// its own, and a barrier's gap).
const CD_LAT_M: f32 = 31.0;
/// `X` stays on the upper level this far past its last ramp, which gives the
/// slope down to the lower level the length its 5 % grade needs.
const SLOPE_M: f32 = 230.0;
/// The most the outer ramps may leave their decks at, from the crossing.
const OUTER_MAX_M: f32 = 600.0;
/// A ramp between the two levels climbs 9 m: at 5 % that is 180 m of length.
/// A loop is at least this long, and is meant to be longer.
const LOOP_MIN_LEN_M: f32 = 250.0;
/// The gentlest a ramp may bend, as a radius, and the clear ground a loop
/// keeps from the outer ramp round it.
const BEND_MIN_M: f32 = 45.0;
const LOOP_GAP_M: f32 = 28.0;

/// What the second pass needs of a system interchange.
#[derive(Debug, Clone)]
pub(super) struct SystemSite {
    /// The crossing's node.
    pub node: usize,
    pub centre: V,
    /// The direction of the upper expressway.
    pub x_dir: V,
    /// How far along `X`, either way, the upper level holds.
    pub half: f32,
}

/// One ramp, deck to deck: (axis, station along it from the crossing) at each
/// end, axis 0 being `M`, 1 `X`, and 2 and 3 the collector-distributor roads
/// beside `M` (on its two sides), and its line, relative to the crossing.
struct SysRamp {
    from: (usize, f32),
    to: (usize, f32),
    line: Vec<V>,
}

struct SystemPlan {
    ramps: Vec<SysRamp>,
    /// Collector-distributor roads beside the lower deck.
    cd: bool,
    /// Where they part from and rejoin the deck, either way along `M`.
    cd_end: [f32; 2],
    cd_ease: f32,
    /// The lower deck's ramp offset, and a collector-distributor road's.
    abut: f32,
    cd_lat: f32,
    /// `X`'s upper level ends at these stations (one each way).
    ends: [f32; 2],
    /// How far the chains are walked from the crossing.
    reach: f32,
    /// The direction of `X`.
    x_dir: V,
    /// Ground the interchange takes whole: the four corners between the ramps
    /// and the decks, relative to the crossing. No lot is laid in one.
    corners: Vec<Vec<V>>,
}

/// The dimensions of one interchange, from its place.
#[derive(Clone, Debug)]
struct Shape {
    /// Degrees from `M`'s direction to `X`'s, 60 to 120 either way.
    angle: f32,
    /// Where the outer ramps leave each ray (`M` behind, `M` ahead, `X` behind,
    /// `X` ahead), from the crossing.
    outer: [f32; 4],
    /// Per quadrant (index `2 * (qx > 0) + (qy > 0)`): loop radius, the
    /// straight a loop runs in on and leaves by, how far the outer ramp swings
    /// out, and whether there is a loop at all.
    radius: [f32; 4],
    lead: [f32; 4],
    pre: [f32; 4],
    bulge: [f32; 4],
    loops: [bool; 4],
    cd: bool,
    cd_run: f32,
    cd_ease: f32,
}

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        let mut r = Rng(seed ^ 0x9E37_79B9_7F4A_7C15);
        r.next();
        r.next();
        r
    }
    /// A float in `[0, 1)` (SplitMix64).
    fn next(&mut self) -> f32 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^= z >> 31;
        (z >> 40) as f32 / (1u64 << 24) as f32
    }
    fn range(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.next()
    }
}

impl Shape {
    /// The shape a crossing laid out between `first` and `last` takes.
    fn from_place(first: V, last: V) -> Shape {
        let q = |x: f32| (x * 2.0).round() as i64 as u64;
        let seed = q(first.0)
            .wrapping_mul(0x0100_0000_01B3)
            .wrapping_add(q(first.1).wrapping_mul(0x9E37_79B1))
            .wrapping_add(q(last.0).wrapping_mul(0x85EB_CA6B))
            .wrapping_add(q(last.1).wrapping_mul(0xC2B2_AE35));
        let mut r = Rng::new(seed);
        // Square in two crossings of five, else anything down to 60 degrees.
        let angle = if r.next() < 0.4 {
            90.0
        } else {
            r.range(60.0, 90.0)
        };
        let angle = if r.next() < 0.5 { angle } else { -angle };
        let base = r.range(380.0, OUTER_MAX_M);
        let mut outer = [0.0; 4];
        for o in &mut outer {
            *o = (base * r.range(0.92, 1.08)).clamp(380.0, OUTER_MAX_M);
        }
        let big = r.range(62.0, 90.0);
        let mut radius = [0.0; 4];
        let mut lead = [0.0; 4];
        let mut pre = [0.0; 4];
        let mut bulge = [0.0; 4];
        for k in 0..4 {
            radius[k] = (big * r.range(0.9, 1.08)).clamp(60.0, 95.0);
            lead[k] = r.range(25.0, 50.0);
            pre[k] = r.range(15.0, 40.0);
            bulge[k] = r.range(0.85, 1.15);
        }
        // Whole, or a partial cloverleaf with one loop or two left out.
        let mut loops = [true; 4];
        let roll = r.next();
        let drop = |r: &mut Rng| (r.next() * 4.0) as usize % 4;
        if roll >= 0.5 {
            loops[drop(&mut r)] = false;
        }
        if roll >= 0.78 {
            loops[drop(&mut r)] = false;
        }
        if loops.iter().filter(|l| **l).count() < 2 {
            loops = [true, false, false, true];
        }
        Shape {
            angle,
            outer,
            radius,
            lead,
            pre,
            bulge,
            loops,
            cd: r.next() < 0.5,
            cd_run: r.range(110.0, 220.0),
            cd_ease: r.range(100.0, 150.0),
        }
    }

    /// The least that fits wherever any interchange does.
    fn compact() -> Shape {
        Shape {
            angle: 90.0,
            outer: [400.0; 4],
            radius: [60.0; 4],
            lead: [30.0; 4],
            pre: [20.0; 4],
            bulge: [1.0; 4],
            loops: [true; 4],
            cd: false,
            cd_run: 0.0,
            cd_ease: 120.0,
        }
    }

    fn outer_max(&self) -> f32 {
        self.outer.iter().copied().fold(0.0, f32::max)
    }

    /// How far the outer ramps run from the crossing, at most.
    fn reach(&self) -> f32 {
        self.outer_max() + 120.0 + SLOPE_M + 60.0
    }
}

fn rot90(v: V) -> V {
    (-v.1, v.0)
}

fn rotate(v: V, degrees: f32) -> V {
    let (s, c) = degrees.to_radians().sin_cos();
    (v.0 * c - v.1 * s, v.0 * s + v.1 * c)
}

/// The smallest radius of curvature along a line, sampled at its vertices.
fn tightest_bend(line: &[V]) -> f32 {
    let mut least = f32::MAX;
    for w in line.windows(3) {
        let (a, b, c) = (w[0], w[1], w[2]);
        let area2 = cross(sub(b, a), sub(c, a)).abs();
        if area2 < 1.0e-3 {
            continue;
        }
        let r = len(sub(b, a)) * len(sub(c, b)) * len(sub(c, a)) / (2.0 * area2);
        least = least.min(r);
    }
    least
}

/// One quadrant's ramps, loop and all, or just the outer ramp.
struct Corner {
    ramps: Vec<SysRamp>,
    /// Where the loop's ends stand on the two decks: (axis, station).
    loop_ends: Option<[(usize, f32); 2]>,
    /// The ground between the decks and the outer ramp.
    ground: Vec<V>,
}

/// The ramps of one quadrant (`qx`, `qy` are the signs of its rays along `u`
/// and `v`, the two decks): the outer ramp and the loop.
///
/// `A` is the deck whose traffic comes toward the crossing in this quadrant, `B`
/// the one whose goes away: the outer ramp leaves `A` and joins `B`, the loop
/// leaves `B` and joins `A`.
#[allow(clippy::too_many_arguments)]
fn quadrant(
    sh: &Shape,
    index: usize,
    (u, v): (V, V),
    (qx, qy): (f32, f32),
    (lat_m, lat_x): (f32, f32),
    cd: bool,
    want_loop: bool,
) -> Corner {
    let ray_m = mul(u, qx);
    let ray_x = mul(v, qy);
    // Unit vectors from the crossing along each deck into this quadrant; B is
    // a quarter turn clockwise of A in this frame (right-hand traffic).
    let m_is_a = cross(ray_m, ray_x) < 0.0;
    let (e_a, e_b, axis_a, axis_b) = if m_is_a {
        (ray_m, ray_x, 0, 1)
    } else {
        (ray_x, ray_m, 1, 0)
    };
    let sin_phi = cross(e_a, e_b).abs();
    let cos_phi = dot(e_a, e_b);
    let phi = sin_phi.atan2(cos_phi).to_degrees();
    // Square to each deck, into the quadrant.
    let n_a = mul(rot90(e_a), -1.0);
    let n_b = rot90(e_b);
    let lat = |axis: usize| if axis == 0 { lat_m } else { lat_x };
    let (lat_a, lat_b) = (lat(axis_a), lat(axis_b));
    let ray = |axis: usize| {
        let sign = if axis == 0 { qx } else { qy };
        axis * 2 + usize::from(sign > 0.0)
    };
    let (oa, ob) = (sh.outer[ray(axis_a)], sh.outer[ray(axis_b)]);
    // A deck node: the lower deck's ends are on its collector-distributor
    // road, one a side. The station is read off the point, along the axis.
    let side_of_m = if cross(u, ray_x) > 0.0 { 2 } else { 3 };
    let end = |axis: usize, p: V| {
        let axis_dir = if axis == 0 { u } else { v };
        let which = if axis == 0 && cd { side_of_m } else { axis };
        (which, dot(p, axis_dir))
    };

    // The outer ramp: a cubic from the foot on `A` to the foot on `B`, tangent
    // to each deck there.
    let p0 = add(mul(e_a, oa), mul(n_a, lat_a));
    let p3 = add(mul(e_b, ob), mul(n_b, lat_b));
    let chord = len(sub(p3, p0));
    let bezier = |k: f32| -> Vec<V> {
        let (p1, p2) = (sub(p0, mul(e_a, k)), sub(p3, mul(e_b, k)));
        let steps = 48;
        (0..=steps)
            .map(|i| {
                let t = i as f32 / steps as f32;
                let u = 1.0 - t;
                let w = |a: f32, b: f32, c: f32, d: f32| {
                    u * u * u * a + 3.0 * u * u * t * b + 3.0 * u * t * t * c + t * t * t * d
                };
                (w(p0.0, p1.0, p2.0, p3.0), w(p0.1, p1.1, p2.1, p3.1))
            })
            .collect()
    };
    let k0 = 0.38 * chord * sh.bulge[index];

    // The loop's circle: its centre is a radius further in from each deck than
    // the ramp's own line, which in skew coordinates along `A` and `B` is
    // divided by the sine of the angle between them.
    let try_loop = |r: f32| -> Option<(Vec<V>, V, [(usize, f32); 2])> {
        let c = add(
            mul(e_a, (lat_b + r) / sin_phi),
            mul(e_b, (lat_a + r) / sin_phi),
        );
        let turn = 180.0 + phi;
        let s0 = sub(c, mul(n_b, r));
        let mut line = Vec::new();
        let pre = sh.pre[index];
        // The straight run in along `B`, then the arc.
        let start = sub(s0, mul(e_b, pre));
        line.push(start);
        let arc_steps = (turn / 7.5).ceil() as usize;
        for i in 0..=arc_steps {
            let theta = turn * i as f32 / arc_steps as f32;
            line.push(add(c, rotate(mul(n_b, -r), theta)));
        }
        let e_pt = *line.last().expect("an arc has points");
        let lead = sh.lead[index];
        let finish = sub(e_pt, mul(e_a, lead));
        line.push(finish);
        let (s_b, s_a) = (dot(start, e_b), dot(finish, e_a));
        let length = polyline_len(&line);
        if s_b < 22.0 || s_a < 22.0 || length < LOOP_MIN_LEN_M {
            return None;
        }
        Some((
            line,
            c,
            [end(axis_b, mul(e_b, s_b)), end(axis_a, mul(e_a, s_a))],
        ))
    };

    let mut outer = bezier(k0);
    let mut loop_part: Option<(Vec<V>, [(usize, f32); 2])> = None;
    if want_loop && let Some((line, centre, ends)) = try_loop(sh.radius[index]) {
        // The outer ramp swings no nearer the loop than its gap: flatter, the
        // nearer the chord it lies.
        let c = centre;
        let r = sh.radius[index];
        for s in [1.0_f32, 0.85, 0.7, 0.55, 0.4] {
            let candidate = bezier(k0 * s);
            let near = candidate
                .iter()
                .map(|p| len(sub(*p, c)))
                .fold(f32::MAX, f32::min);
            if near >= r + LOOP_GAP_M && tightest_bend(&candidate) >= BEND_MIN_M {
                outer = candidate;
                loop_part = Some((line, ends));
                break;
            }
        }
    }
    // Without a loop the outer ramp keeps its own shape, if it bends gently
    // enough; else it is flattened till it does.
    if loop_part.is_none() {
        for s in [1.0_f32, 0.85, 0.7, 0.55] {
            let candidate = bezier(k0 * s);
            if tightest_bend(&candidate) >= BEND_MIN_M || s < 0.6 {
                outer = candidate;
                break;
            }
        }
    }
    let mut ground: Vec<V> = vec![(0.0, 0.0)];
    ground.extend(outer.iter().copied());
    let mut ramps = vec![SysRamp {
        from: end(axis_a, p0),
        to: end(axis_b, p3),
        line: outer,
    }];
    let mut loop_ends = None;
    if let Some((line, ends)) = loop_part {
        ramps.push(SysRamp {
            from: ends[0],
            to: ends[1],
            line,
        });
        loop_ends = Some(ends);
    }
    Corner {
        ramps,
        loop_ends,
        ground,
    }
}

/// Where along `X`'s chain the upper level ends, if there is room: at one of
/// its nodes at least `from` from the crossing, or at a cut of its own, with the
/// slope down to the next node long enough.
fn upper_end(chain: &Chain, side: f32, from: f32) -> Option<f32> {
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
    let c = from + 20.0;
    if next_after(c - 60.0) - c >= SLOPE_M && total - c >= SLOPE_M + 20.0 {
        return Some(c);
    }
    None
}

/// Plan the system interchange at `node`, where `M` runs along `m_dir` and `X`
/// along `x_dir`.
fn plan_system(
    graph: &Graph,
    node: usize,
    (m_dir, x_dir): (V, V),
    half_deck: f32,
    sh: &Shape,
) -> Option<SystemPlan> {
    let reach = sh.reach();
    let m = chain_through(graph, node, reach, Some(m_dir))?;
    let x = chain_through(graph, node, reach, Some(x_dir))?;
    let (m_len, x_len) = (polyline_len(&m.line), polyline_len(&x.line));
    let run = if sh.cd { sh.cd_run } else { 0.0 };
    // Rays are indexed `axis * 2 + (sign > 0)`: `M` behind, `M` ahead, `X`
    // behind, `X` ahead. The chains run along their directions.
    let along_x = dot(sample(&x.line, x.at).1, x_dir).signum();
    let x_fwd = sh.outer[if along_x > 0.0 { 3 } else { 2 }];
    let x_back = sh.outer[if along_x > 0.0 { 2 } else { 3 }];
    if m.at < sh.outer[0] + run + 100.0
        || m_len - m.at < sh.outer[1] + run + 100.0
        || x.at < x_back + SLOPE_M + 100.0
        || x_len - x.at < x_fwd + SLOPE_M + 100.0
    {
        return None;
    }
    // Station along the chain is `at + along * local`: the upper level's
    // ends, either way along `X`'s own positive direction.
    let toward = upper_end(&x, along_x, x_fwd + 120.0)?;
    let against = upper_end(&x, -along_x, x_back + 120.0)?;
    let abut = half_deck + RAMP_HALF_M;
    let lat_m = if sh.cd {
        CD_LAT_M.max(abut + 9.0)
    } else {
        abut
    };
    let axes = (m_dir, x_dir);
    let mut ramps = Vec::new();
    let mut corners = Vec::new();
    let mut have_loop = [false; 4];
    // (axis, station, quadrant) of every loop end, to keep them apart.
    let mut taken: Vec<(usize, f32)> = Vec::new();
    for qx in [-1.0_f32, 1.0] {
        for qy in [-1.0_f32, 1.0] {
            let index = 2 * usize::from(qx > 0.0) + usize::from(qy > 0.0);
            let mut corner = quadrant(
                sh,
                index,
                axes,
                (qx, qy),
                (lat_m, abut),
                sh.cd,
                sh.loops[index],
            );
            // A loop's ends stand clear of any other loop end on the same deck.
            if let Some(ends) = corner.loop_ends
                && ends
                    .iter()
                    .any(|(a, s)| taken.iter().any(|(b, t)| a == b && (s - t).abs() < 30.0))
            {
                corner = quadrant(sh, index, axes, (qx, qy), (lat_m, abut), sh.cd, false);
            }
            if let Some(ends) = corner.loop_ends {
                taken.extend(ends);
                have_loop[index] = true;
            }
            ramps.extend(corner.ramps);
            corners.push(corner.ground);
        }
    }
    // A partial cloverleaf still has two loops at least.
    if have_loop.iter().filter(|l| **l).count() < 2 {
        return None;
    }
    Some(SystemPlan {
        ramps,
        cd: sh.cd,
        cd_end: [sh.outer[0] + run, sh.outer[1] + run],
        cd_ease: sh.cd_ease,
        abut,
        cd_lat: lat_m,
        ends: if along_x > 0.0 {
            [toward, -against]
        } else {
            [against, -toward]
        },
        reach,
        x_dir,
        corners,
    })
}

/// The node on the chain through `node` along `dir` at `place`: one that is
/// there, or a new one cut in.
fn deck_node(
    graph: &mut Graph,
    node: usize,
    dir: V,
    place: V,
    reach: f32,
    role: &str,
) -> Option<usize> {
    let chain = chain_through(graph, node, reach, Some(dir))?;
    let station = project(&chain.line, place).0;
    if let Some(&(_, n)) = chain.nodes.iter().find(|(s, _)| (s - station).abs() < 1.0) {
        return Some(n);
    }
    cut_chain(graph, &chain, station, role)
}

/// Build a planned system interchange into the graph.
fn build(graph: &mut Graph, node: usize, m_dir: V, plan: SystemPlan, layout: &mut Layout) -> bool {
    let centre = graph.pts[node];
    let x_dir = plan.x_dir;
    let axes = [m_dir, x_dir];
    let world = |p: V| add(centre, p);
    let on_axis = |axis: usize, station: f32| add(centre, mul(axes[axis], station));
    let mut nodes: HashMap<(usize, i32), usize> = HashMap::new();
    let key = |axis: usize, station: f32| (axis, (station * 4.0).round() as i32);
    // The upper level's ends first, then every deck node a ramp joins, the
    // farther ones first so the chains' pieces hold for the nearer.
    let mut wanted: Vec<(usize, f32)> = Vec::new();
    for end in plan.ends {
        wanted.push((1, end));
    }
    let (cd_back, cd_ahead) = (-plan.cd_end[0], plan.cd_end[1]);
    if plan.cd {
        wanted.extend([(0, cd_back), (0, cd_ahead)]);
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
            plan.reach,
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
        let normal = rot90(m_dir);
        let lat = |x: f32| {
            let ease = smootherstep((x - cd_back) / plan.cd_ease)
                * smootherstep((cd_ahead - x) / plan.cd_ease);
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
            stops.push((cd_back, nodes[&key(0, cd_back)]));
            stops.push((cd_ahead, nodes[&key(0, cd_ahead)]));
            stops.sort_by(|a, b| (sign * a.0).total_cmp(&(sign * b.0)));
            for w in stops.windows(2) {
                let ((x0, from), (x1, to)) = (w[0], w[1]);
                let pieces = ((x1 - x0).abs() / 15.0).ceil().max(1.0) as usize;
                let line: Vec<V> = (0..=pieces)
                    .map(|k| {
                        let x = x0 + (x1 - x0) * k as f32 / pieces as f32;
                        add(centre, add(mul(m_dir, x), mul(normal, side * lat(x))))
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
    for corner in &plan.corners {
        layout
            .reserved
            .push(corner.iter().map(|p| world(*p)).collect());
    }
    layout.systems.push(SystemSite {
        node,
        centre,
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
    let shapes = [Shape::from_place(first, last), Shape::compact()];
    for sh in shapes {
        let x_dir = rotate(m_dir, sh.angle);
        let margin = sh.outer_max() + 220.0;
        let mut stations: Vec<f32> = Vec::new();
        let mut s = margin;
        while s < total - margin {
            stations.push(s);
            s += 60.0;
        }
        stations.sort_by(|a, b| {
            len(add(first, mul(m_dir, *a))).total_cmp(&len(add(first, mul(m_dir, *b))))
        });
        let half = (reach * 1.3).max(sh.reach() + 400.0);
        for s in stations {
            let p = add(first, mul(m_dir, s));
            let x_line = vec![add(p, mul(x_dir, -half)), add(p, mul(x_dir, half))];
            let mut trial = graph.clone();
            insert_corridor(&mut trial, &x_line);
            let Some(node) = cross_at(&mut trial, p) else {
                continue;
            };
            let Some(plan) = plan_system(&trial, node, (m_dir, x_dir), half_deck, &sh) else {
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
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shapes_vary_with_place_and_stay_in_range() {
        let mut angles = std::collections::BTreeSet::new();
        let mut radii = std::collections::BTreeSet::new();
        let mut partial = 0;
        let mut with_cd = 0;
        for k in 0..64 {
            let a = (k as f32 * 137.0 - 2_000.0, k as f32 * 71.0 - 900.0);
            let sh = Shape::from_place(a, (-a.0, -a.1));
            assert!(
                (60.0..=90.0).contains(&sh.angle.abs()),
                "angle {}",
                sh.angle
            );
            assert!(sh.outer.iter().all(|o| (380.0..=OUTER_MAX_M).contains(o)));
            assert!(sh.radius.iter().all(|r| (60.0..=95.0).contains(r)));
            assert!(sh.loops.iter().filter(|l| **l).count() >= 2);
            angles.insert(sh.angle.round() as i32);
            radii.insert(sh.radius[0].round() as i32);
            partial += usize::from(sh.loops.iter().any(|l| !l));
            with_cd += usize::from(sh.cd);
        }
        assert!(
            angles.len() > 10 && radii.len() > 10,
            "{angles:?} {radii:?}"
        );
        assert!(partial > 8 && partial < 56, "{partial} partial of 64");
        assert!(
            with_cd > 8 && with_cd < 56,
            "{with_cd} with collector roads of 64"
        );
        // The same place gives the same shape.
        let (a, b) = ((10.0, 20.0), (30.0, -40.0));
        assert_eq!(
            format!("{:?}", Shape::from_place(a, b)),
            format!("{:?}", Shape::from_place(a, b))
        );
    }

    /// Distance of `p` from the axis through the crossing along `dir`.
    fn off_axis(dir: V, p: V) -> f32 {
        cross(dir, p).abs()
    }

    #[test]
    fn every_quadrant_lands_on_its_decks_at_any_crossing_angle() {
        let abut = 17.0 + RAMP_HALF_M;
        for angle in [60.0_f32, 67.0, 75.0, 90.0, -62.0, -80.0, -90.0] {
            let u = (1.0, 0.0);
            let v = rotate(u, angle);
            let mut sh = Shape::compact();
            sh.angle = angle;
            let mut loops = 0;
            for (qi, (qx, qy)) in [(-1.0, -1.0), (-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)]
                .into_iter()
                .enumerate()
            {
                let corner = quadrant(&sh, qi, (u, v), (qx, qy), (abut, abut), false, true);
                let axes = [u, v];
                for ramp in &corner.ramps {
                    let (a, b) = (ramp.line[0], ramp.line[ramp.line.len() - 1]);
                    // Each end stands beside its deck, square to it, at the
                    // ramp offset, and the station read back is its own.
                    for (end, point) in [(ramp.from, a), (ramp.to, b)] {
                        assert!(
                            (off_axis(axes[end.0], point) - abut).abs() < 0.05,
                            "angle {angle}: end off by {}",
                            off_axis(axes[end.0], point) - abut
                        );
                        assert!((dot(point, axes[end.0]) - end.1).abs() < 0.05);
                    }
                    // Nothing gentler to ride than 5 % over the 9 m between
                    // the decks.
                    assert!(
                        polyline_len(&ramp.line) >= 180.0,
                        "angle {angle}: ramp {:.0} m",
                        polyline_len(&ramp.line)
                    );
                    assert!(tightest_bend(&ramp.line) >= BEND_MIN_M - 1.0);
                }
                loops += corner.ramps.len() - 1;
                // The loop is bare grass inside: the ground reserved holds its
                // whole circle.
                if corner.loop_ends.is_some() {
                    let loop_line = &corner.ramps[1].line;
                    let mid = loop_line[loop_line.len() / 2];
                    assert!(point_in_ring(&corner.ground, mid), "angle {angle}");
                }
            }
            assert!(loops >= 2, "angle {angle}: only {loops} loops fit");
        }
    }

    fn point_in_ring(ring: &[V], q: V) -> bool {
        let mut inside = false;
        let mut j = ring.len() - 1;
        for i in 0..ring.len() {
            let (a, b) = (ring[i], ring[j]);
            if (a.1 > q.1) != (b.1 > q.1) && q.0 < (b.0 - a.0) * (q.1 - a.1) / (b.1 - a.1) + a.0 {
                inside = !inside;
            }
            j = i;
        }
        inside
    }
}
