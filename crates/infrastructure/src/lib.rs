use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::BinaryHeap};
use terrain_core::{SimulationConfig, TerrainData};
use thiserror::Error;
use urban::{CitySpec, CityStyle, Point as UrbanPoint, UrbanModel, generate_city};
use world_core::{GridPoint, ScalarLayer, WorldError, WorldGrid};

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error(transparent)]
    World(#[from] WorldError),
    #[error("terrain fields do not match the configured grid")]
    InvalidTerrain,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SettlementClass {
    RegionalCentre,
    Town,
    Village,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlementSite {
    pub id: u32,
    pub location: GridPoint,
    pub score: f32,
    pub class: SettlementClass,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RoadClass {
    Motorway,
    Arterial,
    Collector,
    Local,
    Rural,
}

/// Physical cross-section used by renderers and land-cover accounting.  These
/// are metres in the world, never pixels.  A renderer may make a sub-pixel road
/// more legible, but must not feed that exaggeration back into occupancy masks.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RoadProfile {
    pub carriageway_width_metres: f32,
    pub right_of_way_width_metres: f32,
    pub lanes: u8,
    pub paved: bool,
}

impl RoadClass {
    pub const fn profile(self) -> RoadProfile {
        match self {
            Self::Motorway => RoadProfile {
                carriageway_width_metres: 24.6,
                right_of_way_width_metres: 38.0,
                lanes: 4,
                paved: true,
            },
            Self::Arterial => RoadProfile {
                carriageway_width_metres: 13.2,
                right_of_way_width_metres: 20.0,
                lanes: 4,
                paved: true,
            },
            Self::Collector => RoadProfile {
                carriageway_width_metres: 7.2,
                right_of_way_width_metres: 11.0,
                lanes: 2,
                paved: true,
            },
            Self::Local => RoadProfile {
                carriageway_width_metres: 5.5,
                right_of_way_width_metres: 8.0,
                lanes: 2,
                paved: true,
            },
            Self::Rural => RoadProfile {
                carriageway_width_metres: 4.2,
                right_of_way_width_metres: 6.5,
                lanes: 1,
                paved: false,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Road {
    pub id: u32,
    pub from_settlement: u32,
    pub to_settlement: u32,
    pub class: RoadClass,
    pub length_km: f32,
    pub path: Vec<GridPoint>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CrossingKind {
    Bridge,
    Tunnel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossingCandidate {
    pub road_id: u32,
    pub location: GridPoint,
    pub kind: CrossingKind,
    pub importance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfrastructureData {
    pub travel_cost: ScalarLayer,
    pub hazard: ScalarLayer,
    pub settlement_suitability: ScalarLayer,
    pub agricultural_suitability: ScalarLayer,
    pub urban_land: ScalarLayer,
    pub cultivated_land: ScalarLayer,
    pub settlements: Vec<SettlementSite>,
    pub roads: Vec<Road>,
    pub crossings: Vec<CrossingCandidate>,
    pub cities: Vec<UrbanModel>,
}

#[derive(Clone, Copy)]
struct QueueNode {
    estimate: f32,
    index: usize,
}

impl PartialEq for QueueNode {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.estimate.to_bits() == other.estimate.to_bits()
    }
}
impl Eq for QueueNode {}
impl PartialOrd for QueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for QueueNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .estimate
            .total_cmp(&self.estimate)
            .then_with(|| other.index.cmp(&self.index))
    }
}

pub fn generate_infrastructure(
    terrain: &TerrainData,
    config: &SimulationConfig,
) -> Result<InfrastructureData, InfrastructureError> {
    let grid = WorldGrid::new(terrain.size, config.world_size_km)?;
    if terrain.height.len() != grid.len() || terrain.water.len() != grid.len() {
        return Err(InfrastructureError::InvalidTerrain);
    }
    let cell_metres = grid.cell_metres();
    let max_flow = terrain.flow.par_iter().copied().reduce(|| 1.0, f32::max);
    let water_distance = distance_to_water(terrain, grid, max_flow);
    let mut travel_cost = vec![0.0_f32; grid.len()];
    let mut hazard = vec![0.0_f32; grid.len()];
    let mut suitability = vec![0.0_f32; grid.len()];
    let mut agricultural_suitability = vec![0.0_f32; grid.len()];

    travel_cost
        .par_iter_mut()
        .zip(hazard.par_iter_mut())
        .zip(suitability.par_iter_mut())
        .zip(agricultural_suitability.par_iter_mut())
        .enumerate()
        .for_each(|(index, (((cost, risk), score), farm_score))| {
            let point = grid.point(index).unwrap();
            let slope = slope_ratio(terrain, grid, point);
            let ocean = terrain.height[index] < 58.0;
            let standing_water = terrain.water[index] > 0.52;
            let flood = terrain.floodplain[index];
            let wetland = terrain.wetland[index];
            let steep_hazard = smoothstep(0.28, 0.72, slope);
            let coastal_hazard = if (58.0..=76.0).contains(&terrain.height[index]) {
                0.35
            } else {
                0.0
            };
            *risk = clamp01(flood * 0.56 + wetland * 0.44 + steep_hazard * 0.72 + coastal_hazard);
            *cost = if ocean {
                1.0
            } else {
                clamp01(
                    slope * 2.7
                        + terrain.forest[index] * 0.18
                        + wetland * 0.48
                        + terrain.snow[index] * 0.38
                        + f32::from(standing_water) * 0.62,
                )
            };

            let distance_km = water_distance[index] * cell_metres / 1000.0;
            let water_access = (-distance_km / 4.5).exp() * smoothstep(0.18, 0.65, distance_km);
            let flat_land = (-slope * 9.0).exp();
            let temperature = smoothstep(-8.0, 12.0, terrain.temperature[index])
                * (1.0 - smoothstep(29.0, 40.0, terrain.temperature[index]));
            let productive_land = terrain.soil_depth[index] * 0.55
                + terrain.grassland[index] * 0.24
                + terrain.forest[index] * 0.12
                + terrain.sediment[index] * 0.18;
            *score = if ocean || standing_water {
                0.0
            } else {
                clamp01(
                    flat_land.powf(0.72)
                        * (0.34 + productive_land)
                        * (0.45 + water_access * 0.75)
                        * (0.55 + temperature * 0.45)
                        * (1.0 - *risk * 0.78),
                )
            };
            *farm_score = if ocean || standing_water {
                0.0
            } else {
                clamp01(
                    flat_land.powf(0.88)
                        * (0.34 + terrain.soil_depth[index] * 0.76)
                        * (0.62 + terrain.sediment[index] * 0.30)
                        * (0.50 + water_access * 0.68)
                        * (0.58 + temperature * 0.42)
                        * (1.0 - *risk * 0.82)
                        * (1.0 - terrain.bare_ground[index] * 0.52),
                )
            };
        });

    let settlements = select_settlements(terrain, grid, &suitability);
    let roads = route_network(terrain, grid, &travel_cost, &settlements);
    let urban_land = realize_urban_land(grid, &suitability, &settlements, &roads);
    let cultivated_land =
        realize_cultivated_land(grid, &agricultural_suitability, &urban_land, &settlements);
    let crossings = find_crossings(terrain, grid, &roads);
    let cities = settlements
        .iter()
        .map(|settlement| {
            let style = match (config.seed.wrapping_add(settlement.id)) % 3 {
                0 => CityStyle::Parisian,
                1 => CityStyle::BarcelonaEixample,
                _ => CityStyle::Manhattan,
            };
            let radius_km = match settlement.class {
                SettlementClass::RegionalCentre => 2.4,
                SettlementClass::Town => 1.25,
                SettlementClass::Village => 0.38,
            };
            generate_city(
                style,
                CitySpec {
                    centre: UrbanPoint {
                        x_km: settlement.location.x as f32 / (grid.size - 1) as f32
                            * grid.world_size_km,
                        y_km: settlement.location.y as f32 / (grid.size - 1) as f32
                            * grid.world_size_km,
                    },
                    radius_km,
                    rotation_radians: (config.seed ^ settlement.id.wrapping_mul(7919)) as f32
                        * 0.000_013,
                    seed: config.seed ^ settlement.id.wrapping_mul(0x9e37_79b9),
                    density: settlement.score,
                },
            )
        })
        .collect();
    Ok(InfrastructureData {
        travel_cost: ScalarLayer::new("travelCost", grid, travel_cost)?,
        hazard: ScalarLayer::new("hazard", grid, hazard)?,
        settlement_suitability: ScalarLayer::new("settlementSuitability", grid, suitability)?,
        agricultural_suitability: ScalarLayer::new(
            "agriculturalSuitability",
            grid,
            agricultural_suitability,
        )?,
        urban_land: ScalarLayer::new("urbanLand", grid, urban_land)?,
        cultivated_land: ScalarLayer::new("cultivatedLand", grid, cultivated_land)?,
        settlements,
        roads,
        crossings,
        cities,
    })
}

fn realize_urban_land(
    grid: WorldGrid,
    suitability: &[f32],
    settlements: &[SettlementSite],
    roads: &[Road],
) -> Vec<f32> {
    let road_distance = distance_to_roads(grid, roads);
    let mut urban = vec![0.0_f32; grid.len()];
    urban.par_iter_mut().enumerate().for_each(|(index, value)| {
        let point = grid.point(index).unwrap();
        *value = settlements
            .iter()
            .map(|settlement| {
                let radius = match settlement.class {
                    SettlementClass::RegionalCentre => 6.4,
                    SettlementClass::Town => 3.8,
                    SettlementClass::Village => 1.65,
                };
                let dx =
                    (point.x as f32 - settlement.location.x as f32) * grid.cell_metres() / 1000.0;
                let dy =
                    (point.y as f32 - settlement.location.y as f32) * grid.cell_metres() / 1000.0;
                let distance = grid.distance_km(point, settlement.location);
                let angle = dy.atan2(dx);
                let phase = settlement.id as f32 * 1.731;
                let road_km = road_distance[index] * grid.cell_metres() / 1000.0;
                let corridor = 1.0 - smoothstep(0.18, 1.35, road_km);
                let rotation = phase * 0.37;
                let rotated_x = dx * rotation.cos() - dy * rotation.sin();
                let rotated_y = dx * rotation.sin() + dy * rotation.cos();
                // District-sized cells are deliberately larger than individual city blocks. The
                // world layer can be sampled at a few hundred metres per cell, so encoding streets
                // here produced a blurred circular mask. Instead this layer records discontinuous
                // developed districts; the satellite compositor adds the sub-cell street fabric.
                let district_width = match settlement.class {
                    SettlementClass::RegionalCentre => 1.18,
                    SettlementClass::Town => 0.94,
                    SettlementClass::Village => 0.72,
                };
                let district_height = district_width * (0.64 + hash01(settlement.id, 17, 3) * 0.28);
                let warped_x = rotated_x + (rotated_y * 0.73 + phase).sin() * 0.19;
                let warped_y = rotated_y + (rotated_x * 0.61 - phase).sin() * 0.16;
                let block_x = (warped_x / district_width).floor() as i32;
                let block_y = (warped_y / district_height).floor() as i32;
                let parcel = hash01(
                    block_x as u32,
                    block_y as u32,
                    settlement.id.wrapping_mul(97),
                );
                let directional_radius = radius
                    * (0.70
                        + (angle * 3.0 + phase).sin() * 0.15
                        + (angle * 5.0 - phase * 0.63).sin() * 0.09);
                let envelope =
                    1.0 - smoothstep(directional_radius * 0.38, directional_radius, distance);
                let land_quality = smoothstep(0.18, 0.58, suitability[index]);
                let development = envelope * land_quality * (0.58 + corridor * 0.64);

                // A district is either developed or left as a park, field, industrial buffer or
                // future growth land. Road-adjacent districts get priority and form linear arms,
                // while the deterministic parcel threshold prevents a single filled radial blob.
                let vacancy = 0.28 + parcel * 0.43 - corridor * 0.18;
                let occupied = smoothstep(vacancy, vacancy + 0.16, development);
                let district_centre_x = (warped_x / district_width).rem_euclid(1.0) - 0.5;
                let district_centre_y = (warped_y / district_height).rem_euclid(1.0) - 0.5;
                let edge = (0.5 - district_centre_x.abs()).min(0.5 - district_centre_y.abs());
                let district_edge = smoothstep(0.025, 0.115, edge);
                occupied * district_edge * (0.48 + development * 0.52)
            })
            .fold(0.0_f32, f32::max);
        *value = value.clamp(0.0, 1.0);
    });
    urban
}

fn distance_to_roads(grid: WorldGrid, roads: &[Road]) -> Vec<f32> {
    let mut distance = vec![f32::INFINITY; grid.len()];
    for road in roads {
        for segment in road.path.windows(2) {
            let mut x = segment[0].x as isize;
            let mut y = segment[0].y as isize;
            let target_x = segment[1].x as isize;
            let target_y = segment[1].y as isize;
            let dx = (target_x - x).abs();
            let sx = if x < target_x { 1 } else { -1 };
            let dy = -(target_y - y).abs();
            let sy = if y < target_y { 1 } else { -1 };
            let mut error = dx + dy;
            loop {
                if x >= 0 && y >= 0 && x < grid.size as isize && y < grid.size as isize {
                    distance[y as usize * grid.size + x as usize] = 0.0;
                }
                if x == target_x && y == target_y {
                    break;
                }
                let doubled = error * 2;
                if doubled >= dy {
                    error += dy;
                    x += sx;
                }
                if doubled <= dx {
                    error += dx;
                    y += sy;
                }
            }
        }
    }
    distance_transform(grid, &mut distance);
    distance
}

fn distance_transform(grid: WorldGrid, distance: &mut [f32]) {
    for y in 0..grid.size {
        for x in 0..grid.size {
            let index = y * grid.size + x;
            if x > 0 {
                distance[index] = distance[index].min(distance[index - 1] + 1.0);
            }
            if y > 0 {
                distance[index] = distance[index].min(distance[index - grid.size] + 1.0);
            }
            if x > 0 && y > 0 {
                distance[index] = distance[index].min(distance[index - grid.size - 1] + 1.414);
            }
        }
    }
    for y in (0..grid.size).rev() {
        for x in (0..grid.size).rev() {
            let index = y * grid.size + x;
            if x + 1 < grid.size {
                distance[index] = distance[index].min(distance[index + 1] + 1.0);
            }
            if y + 1 < grid.size {
                distance[index] = distance[index].min(distance[index + grid.size] + 1.0);
            }
            if x + 1 < grid.size && y + 1 < grid.size {
                distance[index] = distance[index].min(distance[index + grid.size + 1] + 1.414);
            }
        }
    }
}

fn hash01(x: u32, y: u32, salt: u32) -> f32 {
    let mut value = x
        .wrapping_mul(0x9e37_79b9)
        .wrapping_add(y.wrapping_mul(0x85eb_ca6b))
        .wrapping_add(salt.wrapping_mul(0xc2b2_ae35));
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846c_a68b);
    value ^= value >> 16;
    value as f32 / u32::MAX as f32
}

fn realize_cultivated_land(
    grid: WorldGrid,
    agricultural_suitability: &[f32],
    urban_land: &[f32],
    settlements: &[SettlementSite],
) -> Vec<f32> {
    let mut cultivated = vec![0.0_f32; grid.len()];
    cultivated
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, value)| {
            let point = grid.point(index).unwrap();
            let influence = settlements
                .iter()
                .map(|settlement| {
                    let radius: f32 = match settlement.class {
                        SettlementClass::RegionalCentre => 18.0,
                        SettlementClass::Town => 14.0,
                        SettlementClass::Village => 9.5,
                    };
                    (-grid.distance_km(point, settlement.location).powi(2) / radius.powi(2)).exp()
                })
                .fold(0.0_f32, f32::max);
            *value = smoothstep(
                0.16,
                0.55,
                agricultural_suitability[index] * (0.35 + influence * 0.85),
            ) * (1.0 - urban_land[index]);
        });
    cultivated
}

fn distance_to_water(terrain: &TerrainData, grid: WorldGrid, max_flow: f32) -> Vec<f32> {
    let mut distance = vec![f32::INFINITY; grid.len()];
    for (index, value) in distance.iter_mut().enumerate() {
        let river = terrain.flow[index] / max_flow > 0.006;
        if terrain.water[index] > 0.38 || river {
            *value = 0.0;
        }
    }
    for y in 0..grid.size {
        for x in 0..grid.size {
            let i = y * grid.size + x;
            if x > 0 {
                distance[i] = distance[i].min(distance[i - 1] + 1.0);
            }
            if y > 0 {
                distance[i] = distance[i].min(distance[i - grid.size] + 1.0);
            }
            if x > 0 && y > 0 {
                distance[i] = distance[i].min(distance[i - grid.size - 1] + 1.414);
            }
        }
    }
    for y in (0..grid.size).rev() {
        for x in (0..grid.size).rev() {
            let i = y * grid.size + x;
            if x + 1 < grid.size {
                distance[i] = distance[i].min(distance[i + 1] + 1.0);
            }
            if y + 1 < grid.size {
                distance[i] = distance[i].min(distance[i + grid.size] + 1.0);
            }
            if x + 1 < grid.size && y + 1 < grid.size {
                distance[i] = distance[i].min(distance[i + grid.size + 1] + 1.414);
            }
        }
    }
    distance
}

fn select_settlements(
    terrain: &TerrainData,
    grid: WorldGrid,
    suitability: &[f32],
) -> Vec<SettlementSite> {
    let step = (grid.size / 128).max(2);
    let margin = ((3.0 * 1000.0 / grid.cell_metres()).ceil() as usize).max(2);
    let mut candidates = Vec::new();
    for y in (margin..grid.size - margin).step_by(step) {
        for x in (margin..grid.size - margin).step_by(step) {
            let index = y * grid.size + x;
            let score = suitability[index];
            if score < 0.32 || terrain.water[index] > 0.25 {
                continue;
            }
            let radius = step * 2;
            let local_maximum =
                (y.saturating_sub(radius)..=(y + radius).min(grid.size - 1)).all(|yy| {
                    (x.saturating_sub(radius)..=(x + radius).min(grid.size - 1))
                        .all(|xx| suitability[yy * grid.size + xx] <= score)
                });
            if local_maximum {
                candidates.push((score, GridPoint { x, y }));
            }
        }
    }
    candidates.sort_unstable_by(|a, b| b.0.total_cmp(&a.0));
    let mut selected: Vec<(f32, GridPoint)> = Vec::new();
    for candidate in candidates {
        if selected
            .iter()
            .all(|(_, point)| grid.distance_km(*point, candidate.1) >= 5.0)
        {
            selected.push(candidate);
            if selected.len() == 20 {
                break;
            }
        }
    }
    selected
        .into_iter()
        .enumerate()
        .map(|(index, (score, location))| SettlementSite {
            id: index as u32 + 1,
            location,
            score,
            class: if index == 0 {
                SettlementClass::RegionalCentre
            } else if index < 4 {
                SettlementClass::Town
            } else {
                SettlementClass::Village
            },
        })
        .collect()
}

fn route_network(
    terrain: &TerrainData,
    grid: WorldGrid,
    travel_cost: &[f32],
    settlements: &[SettlementSite],
) -> Vec<Road> {
    if settlements.len() < 2 {
        return Vec::new();
    }
    // The regional grid selects a corridor; retaining up to 512 cells avoids
    // kilometre-scale stair steps before continuous centreline refinement.
    let route_size = grid.size.min(512);
    let route_grid = WorldGrid::new(route_size, grid.world_size_km).unwrap();
    let mut route_cost = vec![0.0; route_grid.len()];
    for (index, value) in route_cost.iter_mut().enumerate() {
        let point = route_grid.point(index).unwrap();
        let source = map_point(point, route_grid, grid);
        let source_index = grid.index(source).unwrap();
        *value = if terrain.height[source_index] < 58.0 {
            10_000.0
        } else {
            1.0 + travel_cost[source_index] * 32.0
        };
    }
    let mut connected = vec![false; settlements.len()];
    connected[0] = true;
    let mut connections = Vec::new();
    while connected.iter().any(|value| !*value) {
        let mut best: Option<(usize, usize, f32)> = None;
        for (from, is_connected) in connected.iter().enumerate() {
            if !*is_connected {
                continue;
            }
            for (to, target_connected) in connected.iter().enumerate() {
                if *target_connected {
                    continue;
                }
                let distance =
                    grid.distance_km(settlements[from].location, settlements[to].location);
                if best.is_none_or(|(_, _, current)| distance < current) {
                    best = Some((from, to, distance));
                }
            }
        }
        let Some((from, to, _)) = best else { break };
        connected[to] = true;
        connections.push((from, to));
    }

    let mut extra_candidates: Vec<(f32, usize, usize)> = Vec::new();
    for from in 0..settlements.len() {
        let mut neighbours: Vec<(f32, usize)> = (0..settlements.len())
            .filter(|to| *to != from)
            .map(|to| {
                (
                    grid.distance_km(settlements[from].location, settlements[to].location),
                    to,
                )
            })
            .collect();
        neighbours.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
        for (distance, to) in neighbours.into_iter().take(3) {
            let edge = (from.min(to), from.max(to));
            let duplicate = connections
                .iter()
                .any(|(a, b)| ((*a).min(*b), (*a).max(*b)) == edge)
                || extra_candidates
                    .iter()
                    .any(|(_, a, b)| ((*a).min(*b), (*a).max(*b)) == edge);
            let important = !matches!(settlements[from].class, SettlementClass::Village)
                || !matches!(settlements[to].class, SettlementClass::Village);
            if !duplicate && important && distance < 24.0 {
                extra_candidates.push((distance, from, to));
            }
        }
    }
    extra_candidates.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
    for (_, from, to) in extra_candidates
        .into_iter()
        .take((settlements.len() / 5).clamp(2, 5))
    {
        connections.push((from, to));
    }

    connections.sort_by_key(|(from, to)| {
        road_class_for(settlements[*from].class, settlements[*to].class) as u8
    });
    let mut roads = Vec::new();
    for (from, to) in connections {
        let start = map_point(settlements[from].location, grid, route_grid);
        let end = map_point(settlements[to].location, grid, route_grid);
        if let Some(route_path) = shortest_path(route_grid, &route_cost, start, end) {
            let mapped: Vec<GridPoint> = route_path
                .iter()
                .copied()
                .map(|point| map_point(point, route_grid, grid))
                .collect();
            let path = smooth_road_path(terrain, grid, travel_cost, mapped);
            let length_km = path
                .windows(2)
                .map(|points| grid.distance_km(points[0], points[1]))
                .sum();
            roads.push(Road {
                id: roads.len() as u32 + 1,
                from_settlement: settlements[from].id,
                to_settlement: settlements[to].id,
                class: road_class_for(settlements[from].class, settlements[to].class),
                length_km,
                path,
            });
            reinforce_corridor(route_grid, &mut route_cost, &route_path);
        }
    }
    roads
}

fn road_class_for(from: SettlementClass, to: SettlementClass) -> RoadClass {
    use SettlementClass::{RegionalCentre, Town, Village};
    match (from, to) {
        (RegionalCentre, RegionalCentre) => RoadClass::Motorway,
        (RegionalCentre, Town) | (Town, RegionalCentre) | (Town, Town) => RoadClass::Arterial,
        (RegionalCentre, Village)
        | (Village, RegionalCentre)
        | (Town, Village)
        | (Village, Town) => RoadClass::Collector,
        (Village, Village) => RoadClass::Rural,
    }
}

fn reinforce_corridor(route_grid: WorldGrid, route_cost: &mut [f32], path: &[GridPoint]) {
    for point in path {
        for oy in -1_isize..=1 {
            for ox in -1_isize..=1 {
                let x = point.x as isize + ox;
                let y = point.y as isize + oy;
                if x < 0 || y < 0 || x >= route_grid.size as isize || y >= route_grid.size as isize
                {
                    continue;
                }
                let index = y as usize * route_grid.size + x as usize;
                if route_cost[index] < 1000.0 {
                    route_cost[index] *= if ox == 0 && oy == 0 { 0.38 } else { 0.68 };
                }
            }
        }
    }
}

fn smooth_road_path(
    terrain: &TerrainData,
    grid: WorldGrid,
    travel_cost: &[f32],
    mut path: Vec<GridPoint>,
) -> Vec<GridPoint> {
    for _ in 0..2 {
        let original = path.clone();
        for index in 1..path.len().saturating_sub(1) {
            let candidate = GridPoint {
                x: (original[index - 1].x + original[index].x * 2 + original[index + 1].x) / 4,
                y: (original[index - 1].y + original[index].y * 2 + original[index + 1].y) / 4,
            };
            let cell = grid.index(candidate).unwrap();
            if terrain.height[cell] >= 58.0 && travel_cost[cell] < 0.82 {
                path[index] = candidate;
            }
        }
    }
    path.dedup();
    path
}

fn shortest_path(
    grid: WorldGrid,
    cost: &[f32],
    start: GridPoint,
    end: GridPoint,
) -> Option<Vec<GridPoint>> {
    let start_index = grid.index(start)?;
    let end_index = grid.index(end)?;
    let states_per_cell = 9;
    let start_state = start_index * states_per_cell + 8;
    let mut distance = vec![f32::INFINITY; grid.len() * states_per_cell];
    let mut previous = vec![usize::MAX; grid.len() * states_per_cell];
    let mut queue = BinaryHeap::new();
    distance[start_state] = 0.0;
    queue.push(QueueNode {
        estimate: 0.0,
        index: start_state,
    });
    let directions = [
        (1_isize, 0_isize),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut end_state = usize::MAX;
    while let Some(node) = queue.pop() {
        let cell = node.index / states_per_cell;
        let incoming_direction = node.index % states_per_cell;
        if cell == end_index {
            end_state = node.index;
            break;
        }
        let point = grid.point(cell)?;
        for (direction, (ox, oy)) in directions.into_iter().enumerate() {
            let x = point.x as isize + ox;
            let y = point.y as isize + oy;
            if x < 0 || y < 0 || x >= grid.size as isize || y >= grid.size as isize {
                continue;
            }
            let next_cell = y as usize * grid.size + x as usize;
            let next_state = next_cell * states_per_cell + direction;
            let step = if ox != 0 && oy != 0 { 1.414 } else { 1.0 };
            let turn = if incoming_direction == 8 {
                0.0
            } else {
                let difference = incoming_direction.abs_diff(direction);
                difference.min(8 - difference) as f32
            };
            let turn_cost = turn * turn * 0.72;
            let tentative =
                distance[node.index] + (cost[cell] + cost[next_cell]) * 0.5 * step + turn_cost;
            if tentative < distance[next_state] {
                distance[next_state] = tentative;
                previous[next_state] = node.index;
                let dx = x as f32 - end.x as f32;
                let dy = y as f32 - end.y as f32;
                queue.push(QueueNode {
                    estimate: tentative + (dx * dx + dy * dy).sqrt(),
                    index: next_state,
                });
            }
        }
    }
    if end_state == usize::MAX {
        return None;
    }
    let mut path = vec![end_state / states_per_cell];
    let mut current = end_state;
    while current != start_state {
        current = previous[current];
        if current == usize::MAX {
            return None;
        }
        path.push(current / states_per_cell);
    }
    path.reverse();
    Some(
        path.into_iter()
            .filter_map(|index| grid.point(index))
            .collect(),
    )
}

fn find_crossings(
    terrain: &TerrainData,
    grid: WorldGrid,
    roads: &[Road],
) -> Vec<CrossingCandidate> {
    let mut candidates: Vec<CrossingCandidate> = Vec::new();
    for road in roads {
        for point in &road.path {
            let index = grid.index(*point).unwrap();
            let bridge = terrain.height[index] >= 58.0
                && (terrain.river_order[index] >= 2 || terrain.water[index] > 0.35);
            let tunnel = slope_ratio(terrain, grid, *point) > 0.38;
            let (kind, importance) = if bridge {
                (
                    CrossingKind::Bridge,
                    terrain.river_order[index] as f32 / 6.0,
                )
            } else if tunnel {
                (CrossingKind::Tunnel, slope_ratio(terrain, grid, *point))
            } else {
                continue;
            };
            if candidates.iter().all(|candidate| {
                candidate.road_id != road.id || grid.distance_km(candidate.location, *point) > 1.5
            }) {
                candidates.push(CrossingCandidate {
                    road_id: road.id,
                    location: *point,
                    kind,
                    importance: clamp01(importance),
                });
            }
        }
    }
    candidates
}

fn map_point(point: GridPoint, from: WorldGrid, to: WorldGrid) -> GridPoint {
    GridPoint {
        x: (point.x * (to.size - 1) + (from.size - 1) / 2) / (from.size - 1),
        y: (point.y * (to.size - 1) + (from.size - 1) / 2) / (from.size - 1),
    }
}

fn slope_ratio(terrain: &TerrainData, grid: WorldGrid, point: GridPoint) -> f32 {
    let left = terrain.height[point.y * grid.size + point.x.saturating_sub(1)];
    let right = terrain.height[point.y * grid.size + (point.x + 1).min(grid.size - 1)];
    let up = terrain.height[point.y.saturating_sub(1) * grid.size + point.x];
    let down = terrain.height[(point.y + 1).min(grid.size - 1) * grid.size + point.x];
    let dx = (right - left) / (2.0 * grid.cell_metres());
    let dy = (down - up) / (2.0 * grid.cell_metres());
    (dx * dx + dy * dy).sqrt()
}

fn smoothstep(edge0: f32, edge1: f32, value: f32) -> f32 {
    let t = ((value - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn clamp01(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use terrain_core::{Landform, TerrainPreset, generate};

    fn config() -> SimulationConfig {
        SimulationConfig {
            seed: 42,
            preset: TerrainPreset::Temperate,
            landform: Landform::Coastal,
            grid_size: 128,
            world_size_km: 80.0,
            rainfall: 1200.0,
            evaporation: 600.0,
            wind_speed: 8.0,
            wind_direction: 220.0,
            sun_azimuth: 235.0,
            sun_elevation: 42.0,
            haze: 2.5,
            cloud_coverage: 35.0,
            cloud_speed: 24.0,
        }
    }

    #[test]
    fn infrastructure_generation_is_deterministic_and_bounded() {
        let config = config();
        let terrain = generate(&config, |_, _| {}).unwrap();
        let first = generate_infrastructure(&terrain, &config).unwrap();
        let second = generate_infrastructure(&terrain, &config).unwrap();
        assert_eq!(first.settlements.len(), second.settlements.len());
        assert_eq!(first.roads.len(), second.roads.len());
        assert!(first.settlements.len() >= 3);
        assert!(first.roads.len() >= first.settlements.len() - 1);
        assert!(first.roads.len() <= first.settlements.len() + 4);
        assert!(
            first
                .settlement_suitability
                .values
                .iter()
                .all(|value| (0.0..=1.0).contains(value))
        );
        assert!(first.settlements.iter().all(|site| {
            terrain.water[site.location.y * terrain.size + site.location.x] < 0.25
        }));
        assert!(
            first
                .agricultural_suitability
                .values
                .iter()
                .all(|value| (0.0..=1.0).contains(value))
        );
        assert!(
            first
                .cultivated_land
                .values
                .iter()
                .any(|value| *value > 0.25)
        );
    }

    #[test]
    fn road_network_connects_selected_sites() {
        let config = config();
        let terrain = generate(&config, |_, _| {}).unwrap();
        let infrastructure = generate_infrastructure(&terrain, &config).unwrap();
        if infrastructure.settlements.len() >= 2 {
            assert!(!infrastructure.roads.is_empty());
            assert!(
                infrastructure
                    .roads
                    .iter()
                    .all(|road| road.path.len() >= 2 && road.length_km > 0.0)
            );
        }
    }

    #[test]
    fn road_hierarchy_uses_realistic_cross_sections() {
        let ordered = [
            RoadClass::Motorway,
            RoadClass::Arterial,
            RoadClass::Collector,
            RoadClass::Local,
            RoadClass::Rural,
        ];
        for pair in ordered.windows(2) {
            assert!(
                pair[0].profile().carriageway_width_metres
                    > pair[1].profile().carriageway_width_metres
            );
        }
        assert_eq!(RoadClass::Motorway.profile().lanes, 4);
        assert_eq!(RoadClass::Rural.profile().lanes, 1);
        assert!(RoadClass::Motorway.profile().carriageway_width_metres < 30.0);
        assert!(RoadClass::Rural.profile().carriageway_width_metres >= 3.0);
        assert_eq!(
            road_class_for(SettlementClass::RegionalCentre, SettlementClass::Village),
            RoadClass::Collector
        );
        assert_eq!(
            road_class_for(SettlementClass::Village, SettlementClass::Village),
            RoadClass::Rural
        );
    }

    #[test]
    fn regional_backbone_density_and_paved_coverage_are_bounded() {
        let config = config();
        let terrain = generate(&config, |_, _| {}).unwrap();
        let infrastructure = generate_infrastructure(&terrain, &config).unwrap();
        let world_area_km2 = config.world_size_km * config.world_size_km;
        let length_density = infrastructure
            .roads
            .iter()
            .map(|road| road.length_km)
            .sum::<f32>()
            / world_area_km2;
        let carriageway_coverage = infrastructure
            .roads
            .iter()
            .map(|road| road.length_km * road.class.profile().carriageway_width_metres / 1000.0)
            .sum::<f32>()
            / world_area_km2;
        assert!(
            (0.01..=0.15).contains(&length_density),
            "regional backbone density {length_density} km/km² is implausible"
        );
        assert!(
            (0.000_02..=0.003).contains(&carriageway_coverage),
            "carriageway coverage {carriageway_coverage} is implausible"
        );
    }

    #[test]
    fn urban_footprint_contains_distinct_districts_and_open_land() {
        let grid = WorldGrid::new(128, 80.0).unwrap();
        let centre = GridPoint { x: 64, y: 64 };
        let settlements = vec![SettlementSite {
            id: 1,
            location: centre,
            score: 1.0,
            class: SettlementClass::RegionalCentre,
        }];
        let roads = vec![Road {
            id: 1,
            from_settlement: 1,
            to_settlement: 2,
            class: RoadClass::Arterial,
            length_km: 20.0,
            path: vec![GridPoint { x: 40, y: 64 }, GridPoint { x: 88, y: 64 }],
        }];
        let urban = realize_urban_land(grid, &vec![0.82; grid.len()], &settlements, &roads);
        let mut developed = 0_usize;
        let mut vacant = 0_usize;
        for (index, value) in urban.iter().enumerate() {
            let point = grid.point(index).unwrap();
            if grid.distance_km(point, centre) < 4.8 {
                if *value > 0.22 {
                    developed += 1;
                } else if *value < 0.04 {
                    vacant += 1;
                }
            }
        }
        assert!(
            developed > 18,
            "city should contain several developed districts"
        );
        assert!(
            vacant > developed / 8,
            "city must retain internal gaps instead of a filled blob"
        );
    }
}
