mod compat;
mod legacy;
mod math;
mod model;
pub mod modern;

pub use legacy::{BarcelonaGenerator, ManhattanGenerator, ParisianGenerator, UrbanGenerator};
pub use model::{
    ActionCandidate, ActionKind, ApproachSpec, ArrowStyle, BuildingFacade, BuildingMass, CityFrameInfo,
    CityGraph, CityJunction, CitySpec, CityStyle, Compound, DrivingSide, GrowthState, HdLane, HdRoad,
    JunctionKind, JunctionPhase, JurisdictionId, LaneMarking, LaneUse, MarkingKind,
    ModelWeights, ModernBuilding, ModernChinaSpec, ModernCity, ModernRoadClass, MorphologyPrior,
    MorphologyStats, Movement, Parcel, ParcelUse, Point, RoadConnector, RoadCrossSection, RoofStyle,
    SdNode, SdRoad, SignalHead, SignalStyle, SplitMix64, StreetClass, StreetSegment, TreeInstance,
    TreeSpecies, TrafficRules, Tributary, TurnArrow, UrbanBlock, UrbanModel, cross_section, measure,
    sample_action, synthesize_junction,
};
pub use modern::{
    CityOptions, ExternalFields, ExternalNode, ExternalRoad, ExternalStreets,
    generate_modern_chinese_city_from_streets, REGIONAL_ROAD_HANDOVER, RegionalApproach, generate_modern_chinese_city,
    generate_modern_chinese_city_with_approaches, generate_modern_chinese_city_with_options,
    hash_u32, junction_trim_m,
};

/// Public style dispatcher. Modern Chinese generation stays in `modern`,
/// legacy styles stay in `legacy`, and this API layer is the only place that
/// knows both families exist.
pub fn generate_city(style: CityStyle, spec: CitySpec) -> UrbanModel {
    match style {
        CityStyle::ChineseModern => generate_modern_chinese_city(ModernChinaSpec {
            centre: spec.centre,
            radius_km: spec.radius_km,
            rotation_radians: spec.rotation_radians,
            seed: spec.seed,
            density: spec.density,
            ..ModernChinaSpec::default()
        })
        .urban_model(),
        CityStyle::Parisian => ParisianGenerator.generate(spec),
        CityStyle::BarcelonaEixample => BarcelonaGenerator.generate(spec),
        CityStyle::Manhattan => ManhattanGenerator.generate(spec),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn spec() -> CitySpec {
        CitySpec {
            centre: Point {
                x_km: 10.0,
                y_km: 8.0,
            },
            radius_km: 1.4,
            rotation_radians: 0.31,
            seed: 42,
            density: 0.8,
        }
    }

    #[test]
    fn city_styles_use_distinct_network_and_building_generators() {
        let paris = generate_city(CityStyle::Parisian, spec());
        let barcelona = generate_city(CityStyle::BarcelonaEixample, spec());
        let manhattan = generate_city(CityStyle::Manhattan, spec());
        assert!(
            paris
                .streets
                .iter()
                .any(|s| s.class == StreetClass::Boulevard)
        );
        assert!(barcelona.blocks.iter().all(|b| b.boundary.len() == 8));
        assert!(manhattan.buildings.len() > manhattan.blocks.len() * 2);
        assert!(manhattan.buildings.iter().any(|b| b.height_metres > 80.0));
        assert!(paris.buildings.iter().all(|b| b.roof == RoofStyle::Mansard));
    }

    #[test]
    fn generation_is_deterministic() {
        let a = generate_city(CityStyle::BarcelonaEixample, spec());
        let b = generate_city(CityStyle::BarcelonaEixample, spec());
        assert_eq!(a.streets.len(), b.streets.len());
        assert_eq!(
            a.buildings[0].height_metres.to_bits(),
            b.buildings[0].height_metres.to_bits()
        );
    }

    fn polygon_area_km2(points: &[Point]) -> f32 {
        points
            .iter()
            .zip(points.iter().cycle().skip(1))
            .take(points.len())
            .map(|(a, b)| a.x_km * b.y_km - b.x_km * a.y_km)
            .sum::<f32>()
            .abs()
            * 0.5
    }

    #[test]
    fn all_city_styles_keep_streets_and_buildings_at_human_scale() {
        for style in [
            CityStyle::Parisian,
            CityStyle::BarcelonaEixample,
            CityStyle::Manhattan,
        ] {
            let city = generate_city(style, spec());
            assert!(
                city.streets
                    .iter()
                    .all(|street| { (3.0..=45.0).contains(&street.width_metres) })
            );
            assert!(city.buildings.iter().all(|building| {
                let area_m2 = polygon_area_km2(&building.footprint) * 1_000_000.0;
                (12.0..=50_000.0).contains(&area_m2)
                    && (2.5..=350.0).contains(&building.height_metres)
            }));
            assert!(
                city.buildings
                    .iter()
                    .flat_map(|building| {
                        building
                            .footprint
                            .iter()
                            .zip(building.footprint.iter().cycle().skip(1))
                            .take(building.footprint.len())
                            .map(|(a, b)| (a.x_km - b.x_km).hypot(a.y_km - b.y_km) * 1000.0)
                    })
                    .all(|edge_metres| edge_metres <= 280.0)
            );
        }
    }

    #[test]
    fn modern_chinese_city_keeps_sd_hd_and_parcel_identity() {
        let city = generate_modern_chinese_city(ModernChinaSpec {
            centre: Point {
                x_km: 5.0,
                y_km: 4.0,
            },
            radius_km: 1.4,
            rotation_radians: 0.17,
            seed: 42,
            density: 0.82,
            block_size_metres: 100.0,
            organic: 0.68,
            river_width_metres: 64.0,
        });
        assert_eq!(city.style, CityStyle::ChineseModern);
        assert!(!city.nodes.is_empty());
        assert!(city.nodes.iter().any(|node| node.role == "grade-crossing"));
        assert!(city.sd_roads.len() > city.blocks.len());
        assert_eq!(city.sd_roads.len(), city.hd_roads.len());
        assert!(
            city.hd_roads
                .iter()
                .any(|road| road.class == ModernRoadClass::Expressway)
        );
        assert!(!city.parcels.is_empty());
        assert!(city.parcels.iter().all(|parcel| parcel.ring.len() >= 4));
        assert!(
            city.buildings
                .iter()
                .all(|building| building.height_metres.is_finite() && building.floors >= 3)
        );
        assert!(city.hd_roads.iter().any(|road| road.bridge));
        assert!(city.hd_roads.iter().all(|road| !road.lanes.is_empty()));
        assert!(
            city.hd_roads
                .iter()
                .flat_map(|road| &road.lanes)
                .any(|lane| {
                    lane.markings.iter().any(|marking| {
                        marking.kind == MarkingKind::Arrow && marking.arrow.is_some()
                    })
                })
        );
        assert!(!city.compounds.is_empty());
        assert!(!city.trees.is_empty());
        assert!(city.river.as_ref().is_some_and(|path| path.len() > 8));
    }

    #[test]
    fn the_city_frame_inverts_the_generator_transform_exactly() {
        let spec = ModernChinaSpec {
            centre: Point {
                x_km: 12.5,
                y_km: -3.25,
            },
            rotation_radians: 0.41,
            ..ModernChinaSpec::default()
        };
        let city = generate_modern_chinese_city(spec);
        // A scene builder that converts kilometre payloads into city-local
        // metres with a re-derived frame would drift; the frame travels with
        // the payload precisely so it cannot.
        for node in city.nodes.iter().take(64) {
            let [x, z] = city.frame.to_local(node.point);
            let back = city.frame.to_world(x, z);
            assert!((back.x_km - node.point.x_km).abs() < 1.0e-4);
            assert!((back.y_km - node.point.y_km).abs() < 1.0e-4);
        }
        let [x, z] = city.frame.to_local(spec.centre);
        assert!(x.abs() < 1.0e-3 && z.abs() < 1.0e-3);
    }

    #[test]
    fn modern_chinese_generation_is_reproducible() {
        let spec = ModernChinaSpec {
            seed: 90210,
            ..ModernChinaSpec::default()
        };
        let first = generate_modern_chinese_city(spec);
        let second = generate_modern_chinese_city(spec);
        assert_eq!(first.nodes.len(), second.nodes.len());
        assert_eq!(first.sd_roads.len(), second.sd_roads.len());
        assert_eq!(first.parcels.len(), second.parcels.len());
        assert_eq!(first.buildings.len(), second.buildings.len());
        assert_eq!(
            first
                .buildings
                .first()
                .map(|building| building.height_metres.to_bits()),
            second
                .buildings
                .first()
                .map(|building| building.height_metres.to_bits())
        );
    }

    #[test]
    fn a_regional_road_enters_town_as_the_same_road() {
        let spec = ModernChinaSpec {
            centre: Point { x_km: 10.0, y_km: 10.0 },
            radius_km: 1.2,
            seed: 7,
            ..ModernChinaSpec::default()
        };
        // Roads reach the town from the west and the south and stop in the middle.
        let road = |dx: f32, dy: f32| RegionalApproach {
            class: ModernRoadClass::Arterial,
            path_km: (0..=40)
                .map(|i| {
                    let t = i as f32 / 40.0;
                    Point {
                        x_km: spec.centre.x_km - dx * (1.0 - t),
                        y_km: spec.centre.y_km - dy * (1.0 - t),
                    }
                })
                .collect(),
        };
        let approaches = [road(3.0, 0.4), road(-0.3, 3.0)];
        let city = generate_modern_chinese_city_with_approaches(spec, &approaches);
        let plain = generate_modern_chinese_city(spec);
        assert_ne!(city.sd_roads.len(), plain.sd_roads.len());

        let ring_m = spec.radius_km * 1000.0;
        for approach in &approaches {
            // Where the road crosses the outer ring, there must be a street node
            // within a junction's width, and an arterial leaving it.
            let r = |p: Point| (p.x_km - spec.centre.x_km).hypot(p.y_km - spec.centre.y_km) * 1000.0;
            let entry = approach
                .path_km
                .windows(2)
                .find_map(|w| {
                    (r(w[0]) > ring_m && r(w[1]) <= ring_m).then(|| {
                        let t = (r(w[0]) - ring_m) / (r(w[0]) - r(w[1]));
                        Point {
                            x_km: w[0].x_km + (w[1].x_km - w[0].x_km) * t,
                            y_km: w[0].y_km + (w[1].y_km - w[0].y_km) * t,
                        }
                    })
                })
                .expect("road crosses the ring");
            // An arterial street must leave the ring right at the entry and run inward.
            let to_entry = |n: &SdNode| (n.point.x_km - entry.x_km).hypot(n.point.y_km - entry.y_km) * 1000.0;
            let from_centre = |n: &SdNode| (n.point.x_km - spec.centre.x_km).hypot(n.point.y_km - spec.centre.y_km) * 1000.0;
            let found = city.sd_roads.iter().any(|r| {
                if r.class != ModernRoadClass::Arterial {
                    return false;
                }
                let (a, b) = (&city.nodes[r.from as usize], &city.nodes[r.to as usize]);
                let (outer, inner) = if from_centre(a) > from_centre(b) { (a, b) } else { (b, a) };
                to_entry(outer) < 40.0 && from_centre(inner) < from_centre(outer) - 20.0
            });
            assert!(found, "no arterial street leaves the ring at the regional road's entry");
        }
        // One connected network: every road reaches every other.
        let mut parent: Vec<usize> = (0..city.nodes.len()).collect();
        fn root(p: &mut [usize], mut x: usize) -> usize {
            while p[x] != x {
                p[x] = p[p[x]];
                x = p[x];
            }
            x
        }
        for r in &city.sd_roads {
            let (a, b) = (root(&mut parent, r.from as usize), root(&mut parent, r.to as usize));
            parent[a] = b;
        }
        let first = root(&mut parent, city.sd_roads[0].from as usize);
        assert!(city.sd_roads.iter().all(|r| root(&mut parent, r.to as usize) == first));
    }

    #[test]
    fn an_organic_town_is_not_a_disc() {
        let spec = ModernChinaSpec {
            centre: Point { x_km: 10.0, y_km: 10.0 },
            radius_km: 1.6,
            seed: 31,
            ..ModernChinaSpec::default()
        };
        let disc = generate_modern_chinese_city(spec);
        let organic = generate_modern_chinese_city_with_options(
            spec,
            &[],
            CityOptions { organic_footprint: true },
        );
        assert!(!organic.buildings.is_empty());
        assert!(organic.buildings.len() < disc.buildings.len(), "the built-up area must be smaller than the disc");
        // Reach of the buildings in twelve compass sectors: a disc is the same
        // in all of them, a real town is not.
        let reach = |city: &ModernCity| -> Vec<f32> {
            let mut sectors = vec![0.0_f32; 12];
            for b in &city.buildings {
                let c = &b.footprint[0];
                let (dx, dy) = ((c.x_km - spec.centre.x_km) * 1000.0, (c.y_km - spec.centre.y_km) * 1000.0);
                let sector = ((dy.atan2(dx) + std::f32::consts::PI) / std::f32::consts::TAU * 12.0) as usize % 12;
                sectors[sector] = sectors[sector].max(dx.hypot(dy));
            }
            sectors
        };
        let spread = |v: &[f32]| {
            let mean = v.iter().sum::<f32>() / v.len() as f32;
            (v.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / v.len() as f32).sqrt() / mean
        };
        assert!(spread(&reach(&organic)) > spread(&reach(&disc)) * 1.5, "outline is as round as a disc");
        // No ring road bounds it.
        assert!(
            organic.nodes.iter().all(|n| n.role != "regional-gateway"),
            "an organic town has no outer ring to put gateways on"
        );
    }

    /// Where only two streets of one class meet there is a bend, not a junction,
    /// and the road must carry on through it without a kink: a kinked centreline
    /// is a kinked kerb, which reads as a zigzag.
    #[test]
    fn streets_run_through_bends_without_a_kink() {
        let city = generate_modern_chinese_city(ModernChinaSpec {
            centre: Point { x_km: 5.0, y_km: 5.0 },
            radius_km: 1.0,
            seed: 12,
            ..ModernChinaSpec::default()
        });
        let heading = |a: Point, b: Point| (b.y_km - a.y_km).atan2(b.x_km - a.x_km);
        let mut checked = 0;
        let mut worst = 0.0_f32;
        for node in &city.nodes {
            let at: Vec<&HdRoad> = city
                .hd_roads
                .iter()
                .filter(|r| {
                    let sd = &city.sd_roads[r.id as usize];
                    sd.from == node.id || sd.to == node.id
                })
                .collect();
            if at.len() != 2 || at[0].class != at[1].class || at[0].bridge || at[1].bridge {
                continue;
            }
            // Direction of each road as it leaves this node.
            let leaving = |road: &HdRoad| -> Option<f32> {
                let sd = &city.sd_roads[road.id as usize];
                let c = &road.centreline;
                (c.len() >= 2).then(|| {
                    if sd.from == node.id { heading(c[0], c[1]) } else { heading(c[c.len() - 1], c[c.len() - 2]) }
                })
            };
            let (Some(h0), Some(h1)) = (leaving(at[0]), leaving(at[1])) else { continue };
            // A straight-through road has the two leaving directions opposite.
            let mut kink = (h0 - h1).abs() % std::f32::consts::TAU;
            if kink > std::f32::consts::PI {
                kink = std::f32::consts::TAU - kink;
            }
            let bend = (std::f32::consts::PI - kink).to_degrees();
            // Only the gentle bends are smoothed; corners (35+ degrees) stay corners.
            if bend > 30.0 {
                continue;
            }
            checked += 1;
            worst = worst.max(bend);
        }
        assert!(checked > 10, "the fixture has too few bends to test ({checked})");
        // A smooth curve still turns a few degrees across one four-metre chord
        // (curvature times chord), and that is what is measured here; an
        // unsmoothed bend turns 10 to 30 degrees at once.
        assert!(worst < 7.0, "a street kinks {worst:.1} degrees where it should run straight on");
    }

    /// A square street grid at `spacing_m`, `n` streets each way, centred on the
    /// origin, as an external network would supply it.
    fn grid_streets(n: i32, spacing_m: f32, river: Vec<Point>) -> (ExternalStreets, f32) {
        grid_streets_at(n, spacing_m, river, Point { x_km: 0.0, y_km: 0.0 })
    }

    fn grid_streets_at(n: i32, spacing_m: f32, river: Vec<Point>, centre: Point) -> (ExternalStreets, f32) {
        let half = (n - 1) as f32 * spacing_m * 0.5;
        let p = |i: i32, j: i32| Point {
            x_km: centre.x_km + (i as f32 * spacing_m - half) / 1_000.0,
            y_km: centre.y_km + (j as f32 * spacing_m - half) / 1_000.0,
        };
        let id = |i: i32, j: i32| (j * n + i) as u64;
        let mut nodes = Vec::new();
        let mut roads = Vec::new();
        for j in 0..n {
            for i in 0..n {
                nodes.push(ExternalNode { id: id(i, j), point: p(i, j) });
                for (di, dj) in [(1, 0), (0, 1)] {
                    let (i2, j2) = (i + di, j + dj);
                    if i2 < n && j2 < n {
                        let class = if (i + j) % 4 == 0 { ModernRoadClass::Arterial } else { ModernRoadClass::Collector };
                        roads.push(ExternalRoad {
                            from: id(i, j),
                            to: id(i2, j2),
                            class,
                            bridge: false,
                            centreline: vec![p(i, j), p(i2, j2)],
                        });
                    }
                }
            }
        }
        (ExternalStreets { nodes, roads, river, tributaries: Vec::new() }, half)
    }

    #[test]
    fn a_street_network_from_outside_gets_blocks_lots_and_buildings() {
        let centre = Point { x_km: 10.0, y_km: 20.0 };
        let (streets, half) = grid_streets_at(9, 140.0, Vec::new(), centre);
        let spec = ModernChinaSpec {
            centre,
            radius_km: half / 1_000.0 * 1.5,
            rotation_radians: 0.0,
            ..ModernChinaSpec::default()
        };
        let fields = ExternalFields { urbanness: Box::new(|_| 1.0), intensity: Box::new(|_| 0.7) };
        let city = generate_modern_chinese_city_from_streets(spec, streets, fields);
        assert_eq!(city.sd_roads.len(), 2 * 9 * 8);
        assert_eq!(city.hd_roads.len(), city.sd_roads.len());
        assert!(city.hd_roads.iter().all(|r| !r.lanes.is_empty()), "every road has lanes");
        assert!(city.blocks.len() >= 40, "{} blocks", city.blocks.len());
        assert!(city.parcels.len() >= 100, "{} parcels", city.parcels.len());
        assert!(city.buildings.len() >= 80, "{} buildings", city.buildings.len());
        // Buildings stay off the carriageway: each footprint corner is clear of every street centreline.
        let lines: Vec<(Point, Point, f32)> = city
            .hd_roads
            .iter()
            .flat_map(|r| r.centreline.windows(2).map(move |w| (w[0], w[1], r.width_metres)))
            .collect();
        for b in city.buildings.iter().take(200) {
            for corner in &b.footprint {
                for (a, c, width) in &lines {
                    let (ax, ay, cx, cy) = (a.x_km * 1000.0, a.y_km * 1000.0, c.x_km * 1000.0, c.y_km * 1000.0);
                    let (px, py) = (corner.x_km * 1000.0, corner.y_km * 1000.0);
                    let (dx, dy) = (cx - ax, cy - ay);
                    let t = (((px - ax) * dx + (py - ay) * dy) / (dx * dx + dy * dy).max(1e-6)).clamp(0.0, 1.0);
                    let d = (px - (ax + dx * t)).hypot(py - (ay + dy * t));
                    assert!(d > width * 0.5, "a building corner is {d:.1} m from a street {width} m wide");
                }
            }
        }
    }

    #[test]
    fn a_town_the_fields_say_is_empty_gets_no_buildings() {
        let (streets, half) = grid_streets(7, 140.0, Vec::new());
        let spec = ModernChinaSpec {
            centre: Point { x_km: 0.0, y_km: 0.0 },
            radius_km: half / 1_000.0 * 1.5,
            rotation_radians: 0.0,
            ..ModernChinaSpec::default()
        };
        // Built up only on the east side: the west stays fields.
        let fields = ExternalFields {
            urbanness: Box::new(|p| if p.x_km > 0.0 { 1.0 } else { 0.0 }),
            intensity: Box::new(|p| if p.x_km > 0.0 { 0.8 } else { 0.0 }),
        };
        let city = generate_modern_chinese_city_from_streets(spec, streets, fields);
        assert!(!city.buildings.is_empty());
        let west = city.buildings.iter().filter(|b| b.footprint.iter().all(|c| c.x_km < -0.02)).count();
        let east = city.buildings.iter().filter(|b| b.footprint.iter().all(|c| c.x_km > 0.0)).count();
        assert_eq!(west, 0, "{west} buildings stand in the fields");
        assert!(east > 20, "{east} buildings in the town");
    }

    #[test]
    fn the_river_a_network_is_given_splits_the_blocks_it_runs_through() {
        // A river running north to south, bending, through the middle of the grid.
        let river: Vec<Point> = (0..=20)
            .map(|k| {
                let y = -0.4 + 0.04 * k as f32;
                Point { x_km: 0.02 + 0.03 * (y * 6.0).sin(), y_km: y }
            })
            .collect();
        let (mut streets, half) = grid_streets(9, 140.0, river.clone());
        // Streets that would run along the water are not built.
        streets.roads.retain(|r| {
            let a = streets.nodes.iter().find(|n| n.id == r.from).unwrap().point;
            let b = streets.nodes.iter().find(|n| n.id == r.to).unwrap().point;
            !(a.x_km == b.x_km && (a.x_km - 0.02).abs() < 0.05)
        });
        let spec = ModernChinaSpec {
            centre: Point { x_km: 0.0, y_km: 0.0 },
            radius_km: half / 1_000.0 * 1.5,
            rotation_radians: 0.0,
            river_width_metres: 50.0,
            ..ModernChinaSpec::default()
        };
        let fields = ExternalFields { urbanness: Box::new(|_| 1.0), intensity: Box::new(|_| 0.6) };
        let city = generate_modern_chinese_city_from_streets(spec, streets, fields);
        assert_eq!(city.river.as_ref().map(Vec::len), Some(river.len()));
        // No building stands in the water.
        let in_river = city
            .buildings
            .iter()
            .flat_map(|b| b.footprint.iter())
            .filter(|c| {
                river
                    .windows(2)
                    .map(|w| {
                        let (ax, ay, bx, by) = (w[0].x_km * 1000.0, w[0].y_km * 1000.0, w[1].x_km * 1000.0, w[1].y_km * 1000.0);
                        let (px, py) = (c.x_km * 1000.0, c.y_km * 1000.0);
                        let (dx, dy) = (bx - ax, by - ay);
                        let t = (((px - ax) * dx + (py - ay) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
                        (px - (ax + dx * t)).hypot(py - (ay + dy * t))
                    })
                    .fold(f32::MAX, f32::min)
                    < 22.0
            })
            .count();
        assert_eq!(in_river, 0, "{in_river} footprint corners are in the river");
        assert!(city.buildings.len() > 30);
    }
}
