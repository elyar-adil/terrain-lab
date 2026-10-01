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
    TreeSpecies, TrafficRules, TurnArrow, UrbanBlock, UrbanModel, cross_section, measure,
    sample_action, synthesize_junction,
};
pub use modern::{
    REGIONAL_ROAD_HANDOVER, RegionalApproach, generate_modern_chinese_city, generate_modern_chinese_city_with_approaches,
    hash_u32,
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
}
