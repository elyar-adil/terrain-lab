use std::collections::HashMap;

use super::CityFrame;
use super::roads::{china_rules, lanes_for_modern_road};
use crate::model::morphology::MorphologyPrior;
use crate::model::probability::{ActionKind, GrowthState, ModelWeights, SplitMix64, sample_action};
use crate::model::{CityGraph, ModernRoadClass};
use crate::{HdRoad, Point, SdNode, SdRoad};

pub(super) struct GraphOutput {
    pub nodes: Vec<SdNode>,
    pub sd_roads: Vec<SdRoad>,
    pub hd_roads: Vec<HdRoad>,
    pub x_lines: Vec<(f32, ModernRoadClass)>,
    pub z_lines: Vec<(f32, ModernRoadClass)>,
    pub river: Vec<Point>,
    pub morphology_score: f64,
}

/// Borrowed view of the graph before the owning `ModernCity` exists, so the
/// morphology prior can score candidates during generation.
pub(super) struct GraphProbe<'a> {
    pub nodes: &'a [SdNode],
    pub sd_roads: &'a [SdRoad],
    pub hd_roads: &'a [HdRoad],
}

impl CityGraph for GraphProbe<'_> {
    fn nodes(&self) -> &[SdNode] {
        self.nodes
    }
    fn sd_roads(&self) -> &[SdRoad] {
        self.sd_roads
    }
    fn hd_roads(&self) -> &[HdRoad] {
        self.hd_roads
    }
}

pub(super) fn build_graph(frame: &CityFrame) -> GraphOutput {
    let radius_m = frame.radius_m;
    let block_m = frame.block_m;
    let organic = frame.organic;
    let river_half = frame.river_half;
    let seed = frame.spec.seed;
    let rules = china_rules();
    let mut river = Vec::new();
    for i in 0..=24 {
        let z = -radius_m + 2.0 * radius_m * i as f32 / 24.0;
        river.push(frame.to_world(frame.river_x(z), z));
    }

    let x_lines = modern_grid_lines(radius_m, block_m, organic, seed, 11);
    let z_lines = modern_grid_lines(radius_m, block_m, organic, seed, 29);
    let mut nodes = Vec::new();
    let mut node_ids = vec![vec![0_u32; z_lines.len()]; x_lines.len()];
    for (ix, &(x, _)) in x_lines.iter().enumerate() {
        for (iz, &(z, _)) in z_lines.iter().enumerate() {
            let id = nodes.len() as u32;
            node_ids[ix][iz] = id;
            nodes.push(SdNode {
                id,
                point: frame.to_world(x, z),
                role: if (x.abs() - radius_m).abs() < 1.0 || (z.abs() - radius_m).abs() < 1.0 {
                    "regional-gateway".into()
                } else if x.abs() < block_m * 1.2 && z.abs() < block_m * 1.2 {
                    "core-junction".into()
                } else {
                    "district-junction".into()
                },
            });
        }
    }

    let mut sd_roads = Vec::new();
    let mut hd_roads = Vec::new();
    let mut next_road = 0_u32;
    let push_segment = |from: u32,
                            to: u32,
                            class: ModernRoadClass,
                            bridge: bool,
                            nodes: &[SdNode],
                            sd_roads: &mut Vec<SdRoad>,
                            hd_roads: &mut Vec<HdRoad>,
                            next_road: &mut u32| {
        let id = *next_road;
        *next_road += 1;
        sd_roads.push(SdRoad {
            id,
            from,
            to,
            class,
            bridge,
        });
        let p0 = nodes[from as usize].point;
        let p1 = nodes[to as usize].point;
        hd_roads.push(HdRoad {
            id,
            sd_road: id,
            class,
            width_metres: class.width_metres(),
            median_metres: class.median_metres(),
            centreline: vec![
                p0,
                Point {
                    x_km: (p0.x_km + p1.x_km) * 0.5,
                    y_km: (p0.y_km + p1.y_km) * 0.5,
                },
                p1,
            ],
            bridge,
            layer: if bridge { 1 } else { 0 },
            lanes: lanes_for_modern_road(id, class, &rules),
            connectors: Vec::new(),
        });
    };

    // Local streets are the discretionary layer of a Chinese plan: outer
    // residential superblocks keep their interiors closed while the
    // commercial core holds a dense 支路 grid.  Each candidate segment runs
    // the ported growth model — accessibility, served demand, continuity,
    // block gain, construction cost and morphology decide probabilistically,
    // with the bias strengthened away from the centre where the superblock
    // (leaving the interior closed) is itself the planning outcome.
    let mut growth_rng = SplitMix64::new(seed as u64 ^ 0x6c6f_6361_6c73);
    let weights = ModelWeights::default();
    let mut local_segment_built = |centrality: f32| -> bool {
        let demand = (0.35 + 0.65 * centrality).clamp(0.15, 1.0) as f64;
        let mut growth = GrowthState::new(demand);
        let superblock_gain = 0.7 + (1.0 - centrality) * 0.9;
        let mut candidates = [
            growth.candidate(ActionKind::Extend, 0.25, 0.18, 0.12),
            growth.candidate(ActionKind::CloseBlock, 0.72, 0.34, 0.16),
            growth.candidate(ActionKind::Stop, 0.12, 0.0, 0.2),
        ];
        candidates[2].served_demand = 0.35 * demand;
        candidates[2].block_gain = superblock_gain as f64;
        let selected = sample_action(&candidates, weights, &mut growth_rng).unwrap_or(0);
        growth.apply(&candidates[selected]);
        candidates[selected].kind != ActionKind::Stop
    };

    for ix in 0..x_lines.len() {
        let (x, class) = x_lines[ix];
        for iz in 0..z_lines.len().saturating_sub(1) {
            let z0 = z_lines[iz].0;
            let z1 = z_lines[iz + 1].0;
            if (z1 - z0).abs() < 1.0 {
                continue;
            }
            let zm = (z0 + z1) * 0.5;
            let river_hit = (x - frame.river_x(zm)).abs() < river_half;
            let bridge = river_hit
                && matches!(
                    class,
                    ModernRoadClass::Expressway | ModernRoadClass::Arterial
                );
            if river_hit && !bridge {
                continue;
            }
            if class == ModernRoadClass::Local {
                let centrality = (1.0 - zm.abs() / radius_m).clamp(0.0, 1.0);
                if !local_segment_built(centrality) {
                    continue;
                }
            }
            push_segment(
                node_ids[ix][iz],
                node_ids[ix][iz + 1],
                class,
                bridge,
                &nodes,
                &mut sd_roads,
                &mut hd_roads,
                &mut next_road,
            );
        }
    }
    for iz in 0..z_lines.len() {
        let (z, class) = z_lines[iz];
        for ix in 0..x_lines.len().saturating_sub(1) {
            let x0 = x_lines[ix].0;
            let x1 = x_lines[ix + 1].0;
            if (x1 - x0).abs() < 1.0 {
                continue;
            }
            let xm = (x0 + x1) * 0.5;
            let river_hit = (xm - frame.river_x(z)).abs() < river_half;
            let bridge = river_hit
                && matches!(
                    class,
                    ModernRoadClass::Expressway | ModernRoadClass::Arterial
                );
            if river_hit && !bridge {
                continue;
            }
            if class == ModernRoadClass::Local {
                let centrality = (1.0 - xm.abs() / radius_m).clamp(0.0, 1.0);
                if !local_segment_built(centrality) {
                    continue;
                }
            }
            push_segment(
                node_ids[ix][iz],
                node_ids[ix + 1][iz],
                class,
                bridge,
                &nodes,
                &mut sd_roads,
                &mut hd_roads,
                &mut next_road,
            );
        }
    }

    // The source city has a rectangular inner ring and diagonal avenues
    // through the historic core.  Keep the ring explicit in the SD/HD graph;
    // which diagonal avenues are realised is chosen by the ported stochastic
    // model over candidate corridors, exactly like the source kernel's
    // regional-mobility choice set.
    let ring = radius_m * 0.76;
    append_corridor(
        frame,
        &rules,
        &mut nodes,
        &mut sd_roads,
        &mut hd_roads,
        &mut next_road,
        vec![
            (-ring, -ring),
            (ring, -ring),
            (ring, ring),
            (-ring, ring),
            (-ring, -ring),
        ],
        ModernRoadClass::Arterial,
        river_half,
    );
    let diagonal_extent = ring * 0.94;
    let mut corridor_rng = SplitMix64::new(seed as u64 ^ 0x6469_6167);
    let corridor_growth = GrowthState::new(0.78);
    let candidate_specs: [Option<(bool, bool)>; 4] = [
        Some((true, false)),
        Some((false, true)),
        Some((true, true)),
        None,
    ];
    let candidates: Vec<crate::model::ActionCandidate> = candidate_specs
        .iter()
        .map(|spec| match spec {
            Some((true, true)) => corridor_growth.candidate(ActionKind::Connect, 0.95, 0.95, 0.18),
            Some(_) => corridor_growth.candidate(ActionKind::Connect, 0.95, 0.52, 0.18),
            None => corridor_growth.candidate(ActionKind::Stop, 0.0, 0.0, 0.0),
        })
        .collect();
    let chosen = sample_action(&candidates, ModelWeights::default(), &mut corridor_rng)
        .unwrap_or(candidate_specs.len() - 1);
    for (index, spec) in candidate_specs.iter().enumerate() {
        let Some((a, b)) = spec else {
            continue;
        };
        if chosen != index {
            continue;
        }
        let (from_x, from_z, to_x, to_z) = if *a && !*b {
            (-1.0, -1.0, 1.0, 1.0)
        } else if *b && !*a {
            (-1.0, 1.0, 1.0, -1.0)
        } else {
            // The "both axes" candidate emits the two distinct diagonals.
            append_corridor(
                frame,
                &rules,
                &mut nodes,
                &mut sd_roads,
                &mut hd_roads,
                &mut next_road,
                vec![
                    (-diagonal_extent, -diagonal_extent),
                    (diagonal_extent, diagonal_extent),
                ],
                ModernRoadClass::Arterial,
                river_half,
            );
            append_corridor(
                frame,
                &rules,
                &mut nodes,
                &mut sd_roads,
                &mut hd_roads,
                &mut next_road,
                vec![
                    (-diagonal_extent, diagonal_extent),
                    (diagonal_extent, -diagonal_extent),
                ],
                ModernRoadClass::Arterial,
                river_half,
            );
            continue;
        };
        append_corridor(
            frame,
            &rules,
            &mut nodes,
            &mut sd_roads,
            &mut hd_roads,
            &mut next_road,
            vec![
                (from_x * diagonal_extent, from_z * diagonal_extent),
                (to_x * diagonal_extent, to_z * diagonal_extent),
            ],
            ModernRoadClass::Arterial,
            river_half,
        );
    }

    split_grade_crossings(
        &mut nodes,
        &mut sd_roads,
        &mut hd_roads,
        &mut next_road,
        &rules,
    );
    prune_isolated_nodes(&mut nodes, &mut sd_roads);

    let morphology_score = MorphologyPrior::default().score(&GraphProbe {
        nodes: &nodes,
        sd_roads: &sd_roads,
        hd_roads: &hd_roads,
    });

    GraphOutput {
        nodes,
        sd_roads,
        hd_roads,
        x_lines,
        z_lines,
        river,
        morphology_score,
    }
}

#[allow(clippy::too_many_arguments)]
fn append_corridor(
    frame: &CityFrame,
    rules: &crate::model::TrafficRules,
    nodes: &mut Vec<SdNode>,
    sd_roads: &mut Vec<SdRoad>,
    hd_roads: &mut Vec<HdRoad>,
    next_road: &mut u32,
    local_points: Vec<(f32, f32)>,
    class: ModernRoadClass,
    river_half: f32,
) {
    if local_points.len() < 2 {
        return;
    }
    let node_ids_for_corridor: Vec<u32> = local_points
        .iter()
        .map(|(x, z)| {
            let id = nodes.len() as u32;
            nodes.push(SdNode {
                id,
                point: frame.to_world(*x, *z),
                role: "ring-or-diagonal-junction".into(),
            });
            id
        })
        .collect();
    for pair in node_ids_for_corridor.windows(2) {
        let from = pair[0];
        let to = pair[1];
        let p0 = nodes[from as usize].point;
        let p1 = nodes[to as usize].point;
        let mid_local = (
            (local_points[(from - node_ids_for_corridor[0]) as usize].0
                + local_points[(to - node_ids_for_corridor[0]) as usize].0)
                * 0.5,
            (local_points[(from - node_ids_for_corridor[0]) as usize].1
                + local_points[(to - node_ids_for_corridor[0]) as usize].1)
                * 0.5,
        );
        let bridge = (mid_local.0 - frame.river_x(mid_local.1)).abs() < river_half;
        let id = *next_road;
        *next_road += 1;
        sd_roads.push(SdRoad {
            id,
            from,
            to,
            class,
            bridge,
        });
        hd_roads.push(HdRoad {
            id,
            sd_road: id,
            class,
            width_metres: class.width_metres(),
            median_metres: class.median_metres(),
            centreline: vec![
                p0,
                Point {
                    x_km: (p0.x_km + p1.x_km) * 0.5,
                    y_km: (p0.y_km + p1.y_km) * 0.5,
                },
                p1,
            ],
            bridge,
            layer: if bridge { 1 } else { 0 },
            lanes: lanes_for_modern_road(id, class, rules),
            connectors: Vec::new(),
        });
    }
}

/// Prune every node that no surviving road references, remapping the surviving
/// nodes so ids stay dense.  Dropped local streets otherwise leave anonymous
/// orphan vertices in the payload.
fn prune_isolated_nodes(nodes: &mut Vec<SdNode>, sd_roads: &mut [SdRoad]) {
    let mut referenced = vec![false; nodes.len()];
    for road in sd_roads.iter() {
        referenced[road.from as usize] = true;
        referenced[road.to as usize] = true;
    }
    if referenced.iter().all(|kept| *kept) {
        return;
    }
    let mut remap = vec![0_u32; nodes.len()];
    let mut kept = Vec::with_capacity(nodes.len());
    for (index, node) in nodes.iter().enumerate() {
        if referenced[index] {
            remap[index] = kept.len() as u32;
            kept.push(node.clone());
        }
    }
    for road in sd_roads.iter_mut() {
        road.from = remap[road.from as usize];
        road.to = remap[road.to as usize];
    }
    for (id, node) in kept.iter_mut().enumerate() {
        node.id = id as u32;
    }
    *nodes = kept;
}

/// Split every same-level geometric crossing into a shared SD node. Corridors
/// are authored as long planning lines, but downstream junction generation
/// must see the actual intersection topology rather than two visually
/// crossing meshes with no legal movement between them.
fn split_grade_crossings(
    nodes: &mut Vec<SdNode>,
    sd_roads: &mut Vec<SdRoad>,
    hd_roads: &mut Vec<HdRoad>,
    next_road: &mut u32,
    rules: &crate::model::TrafficRules,
) {
    let quantize = |point: Point| {
        (
            (point.x_km * 1_000_000.0).round() as i64,
            (point.y_km * 1_000_000.0).round() as i64,
        )
    };
    let mut node_by_position: HashMap<(i64, i64), u32> = nodes
        .iter()
        .map(|node| (quantize(node.point), node.id))
        .collect();
    let layer_by_road: HashMap<u32, i8> =
        hd_roads.iter().map(|road| (road.id, road.layer)).collect();
    let mut cuts: Vec<Vec<(f32, u32)>> = sd_roads
        .iter()
        .map(|road| vec![(0.0, road.from), (1.0, road.to)])
        .collect();
    for left in 0..sd_roads.len() {
        for right in (left + 1)..sd_roads.len() {
            if layer_by_road.get(&sd_roads[left].id).copied().unwrap_or(0) != 0
                || layer_by_road.get(&sd_roads[right].id).copied().unwrap_or(0) != 0
            {
                continue;
            }
            let left_a = nodes[sd_roads[left].from as usize].point;
            let left_b = nodes[sd_roads[left].to as usize].point;
            let right_a = nodes[sd_roads[right].from as usize].point;
            let right_b = nodes[sd_roads[right].to as usize].point;
            let Some((left_t, right_t, point)) =
                segment_intersection(left_a, left_b, right_a, right_b)
            else {
                continue;
            };
            if left_t <= 1.0e-5
                || left_t >= 1.0 - 1.0e-5
                || right_t <= 1.0e-5
                || right_t >= 1.0 - 1.0e-5
            {
                continue;
            }
            let node_id = if let Some(id) = node_by_position.get(&quantize(point)).copied() {
                id
            } else {
                let id = nodes.len() as u32;
                nodes.push(SdNode {
                    id,
                    point,
                    role: "grade-crossing".into(),
                });
                node_by_position.insert(quantize(point), id);
                id
            };
            cuts[left].push((left_t, node_id));
            cuts[right].push((right_t, node_id));
        }
    }
    if cuts.iter().all(|cut| cut.len() == 2) {
        return;
    }

    let original_sd = std::mem::take(sd_roads);
    let original_hd = std::mem::take(hd_roads);
    for (index, source) in original_sd.into_iter().enumerate() {
        let mut split = cuts[index].clone();
        split.sort_by(|left, right| left.0.total_cmp(&right.0));
        split.dedup_by(|left, right| (left.0 - right.0).abs() < 1.0e-6);
        let template = original_hd.iter().find(|road| road.id == source.id);
        for (part, window) in split.windows(2).enumerate() {
            let from = window[0].1;
            let to = window[1].1;
            if from == to {
                continue;
            }
            let id = if part == 0 {
                source.id
            } else {
                let value = *next_road;
                *next_road += 1;
                value
            };
            let p0 = nodes[from as usize].point;
            let p1 = nodes[to as usize].point;
            sd_roads.push(SdRoad {
                id,
                from,
                to,
                class: source.class,
                bridge: source.bridge,
            });
            hd_roads.push(HdRoad {
                id,
                sd_road: id,
                class: source.class,
                width_metres: source.class.width_metres(),
                median_metres: source.class.median_metres(),
                centreline: vec![
                    p0,
                    Point {
                        x_km: (p0.x_km + p1.x_km) * 0.5,
                        y_km: (p0.y_km + p1.y_km) * 0.5,
                    },
                    p1,
                ],
                bridge: source.bridge,
                layer: template.map(|road| road.layer).unwrap_or(0),
                lanes: lanes_for_modern_road(id, source.class, rules),
                connectors: Vec::new(),
            });
        }
    }
}

fn segment_intersection(a: Point, b: Point, c: Point, d: Point) -> Option<(f32, f32, Point)> {
    let ab_x = b.x_km - a.x_km;
    let ab_y = b.y_km - a.y_km;
    let cd_x = d.x_km - c.x_km;
    let cd_y = d.y_km - c.y_km;
    let denominator = ab_x * cd_y - ab_y * cd_x;
    if denominator.abs() < 1.0e-9 {
        return None;
    }
    let ac_x = c.x_km - a.x_km;
    let ac_y = c.y_km - a.y_km;
    let t = (ac_x * cd_y - ac_y * cd_x) / denominator;
    let u = (ac_x * ab_y - ac_y * ab_x) / denominator;
    if !(0.0..=1.0).contains(&t) || !(0.0..=1.0).contains(&u) {
        return None;
    }
    Some((
        t,
        u,
        Point {
            x_km: a.x_km + t * ab_x,
            y_km: a.y_km + t * ab_y,
        },
    ))
}

pub(super) fn modern_grid_lines(
    half: f32,
    block: f32,
    organic: f32,
    seed: u32,
    salt: i32,
) -> Vec<(f32, ModernRoadClass)> {
    let count = (half / block).ceil() as i32;
    let mut lines: Vec<(f32, ModernRoadClass)> = Vec::with_capacity((count * 2 + 3) as usize);
    for i in -count..=count {
        let base = i as f32 * block;
        if base.abs() > half + 1.0 {
            continue;
        }
        let jitter = if i.abs() <= 1 {
            0.0
        } else {
            (modern_hash(seed, i, salt, 733) - 0.5) * block * 0.07 * organic
        };
        let value = (base + jitter).clamp(-half, half);
        let class = if i.abs() == count {
            ModernRoadClass::Expressway
        } else if i.rem_euclid(5) == 0 {
            ModernRoadClass::Arterial
        } else if i.rem_euclid(2) == 0 {
            ModernRoadClass::Collector
        } else {
            ModernRoadClass::Local
        };
        if lines
            .last()
            .map(|(last, _)| (value - *last).abs() < block * 0.35)
            .unwrap_or(false)
        {
            continue;
        }
        lines.push((value, class));
    }
    if lines
        .first()
        .map(|(v, _)| *v > -half + 12.0)
        .unwrap_or(true)
    {
        lines.insert(0, (-half, ModernRoadClass::Expressway));
    }
    if lines.last().map(|(v, _)| *v < half - 12.0).unwrap_or(true) {
        lines.push((half, ModernRoadClass::Expressway));
    }
    lines
}

pub(super) fn modern_phase(seed: u32) -> f32 {
    modern_hash(seed, 3, 5, 751) * std::f32::consts::TAU
}

pub(super) fn modern_hash(seed: u32, x: i32, y: i32, salt: i32) -> f32 {
    let mut value = seed
        ^ (x as u32).wrapping_mul(0x9e37_79b9)
        ^ (y as u32).wrapping_mul(0x85eb_ca6b)
        ^ (salt as u32).wrapping_mul(0xc2b2_ae35);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value as f32 / u32::MAX as f32
}
