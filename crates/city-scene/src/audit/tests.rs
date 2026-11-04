//! The auditor is only worth its gate if it sees what it is meant to see, so
//! each family of check is shown a city with one planted defect.

use urban::{
    CityOptions, ModernChinaSpec, ModernCity, ModernRoadClass, Point, RegionalApproach,
    generate_modern_chinese_city_with_options,
};

use super::*;
use crate::scene::{SceneBudget, derive_city};

/// The city `dump_scene` and `sanity_audit` build with no arguments.
pub(crate) fn demo_city() -> ModernCity {
    let (seed, radius_km) = (42_u32, 0.5_f32);
    let classes = [
        ModernRoadClass::Arterial,
        ModernRoadClass::Collector,
        ModernRoadClass::Collector,
        ModernRoadClass::Local,
    ];
    let approaches: Vec<RegionalApproach> = classes
        .iter()
        .enumerate()
        .map(|(i, class)| {
            let h = urban::hash_u32(seed, i as i32, 77);
            let angle = (i as f32 + 0.15 + 0.7 * h) * std::f32::consts::TAU / 4.0;
            let (c, s) = (angle.cos(), angle.sin());
            let path_km = (0..=16)
                .map(|k| {
                    let t = k as f32 / 16.0;
                    let r = radius_km * (1.7 - 1.7 * t);
                    let bow = (t * std::f32::consts::PI).sin() * radius_km * 0.08 * (h - 0.5);
                    Point {
                        x_km: r * c - bow * s,
                        y_km: r * s + bow * c,
                    }
                })
                .collect();
            RegionalApproach {
                class: *class,
                path_km,
            }
        })
        .collect();
    generate_modern_chinese_city_with_options(
        ModernChinaSpec {
            seed,
            radius_km,
            block_size_metres: 110.0,
            ..ModernChinaSpec::default()
        },
        &approaches,
        CityOptions {
            organic_footprint: true,
        },
    )
}

fn square(city: &ModernCity, x: f32, z: f32, half: f32) -> Vec<Point> {
    [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
        .iter()
        .map(|(a, b)| city.frame.to_world(x + a * half, z + b * half))
        .collect()
}

fn plan_only(city: &ModernCity) -> Report {
    let network = derive_city(city);
    audit(city, &network, None, None, usize::MAX)
}

/// The regression gate: the demo city, every check, against the numbers in
/// [`CHECKS`]. A fix lowers a gate; nothing raises one.
#[test]
fn the_demo_city_is_within_the_gate() {
    let city = demo_city();
    let report = audit_city(&city, SceneBudget::default());
    let over = report.over_gate();
    assert!(
        over.is_empty(),
        "sanity audit over its gate: {over:?}\n{}",
        report
            .ranked()
            .iter()
            .flat_map(|s| s.worst.iter().take(3))
            .map(|v| format!("{} at ({:.0}, {:.0}): {}", v.check, v.x, v.z, v.detail))
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(report.inventory.scene_checked && report.inventory.trees > 0);
}

/// A scene holds no tree and no piece of street furniture inside a building.
#[test]
fn nothing_planted_stands_inside_a_building() {
    let city = demo_city();
    let report = audit_city(&city, SceneBudget::default());
    assert_eq!(report.count("overlap.tree_building"), 0);
    assert_eq!(report.count("overlap.furniture_building"), 0);
}

/// Every junction the network makes signalised carries heads.
#[test]
fn every_signalised_junction_has_signal_heads() {
    let city = demo_city();
    let report = audit_city(&city, SceneBudget::default());
    assert!(report.inventory.signalised > 0);
    assert_eq!(report.count("net.junction_control"), 0);
}

#[test]
fn it_sees_two_buildings_on_one_spot() {
    let mut city = demo_city();
    let mut twin = city.buildings[0].clone();
    twin.id = 90_001;
    city.buildings.push(twin);
    // Same footprint, same height: not a podium, a duplicate.
    assert!(plan_only(&city).count("overlap.building_building") >= 1);
}

#[test]
fn it_sees_a_building_in_the_carriageway() {
    let mut city = demo_city();
    let network = derive_city(&city);
    let road = network
        .roads
        .iter()
        .find(|r| r.layer == 0 && r.carriageway.length() > 60.0)
        .expect("a road");
    let (p, _) = road.carriageway.sample(road.carriageway.length() * 0.5);
    let mut b = city.buildings[0].clone();
    b.id = 90_002;
    b.footprint = square(&city, p.x, p.z, 5.0);
    city.buildings.push(b);
    assert!(plan_only(&city).count("overlap.building_road") >= 1);
}

#[test]
fn it_sees_a_building_in_the_river() {
    let mut city = demo_city();
    let river = city.river.clone().expect("the demo city has a river");
    let mid = river[river.len() / 2];
    let [x, z] = city.frame.to_local(mid);
    let mut b = city.buildings[0].clone();
    b.id = 90_003;
    b.footprint = square(&city, x, z, 6.0);
    city.buildings.push(b);
    assert!(plan_only(&city).count("water.building") >= 1);
}

#[test]
fn it_sees_a_road_fragment_cut_off_from_the_network() {
    let mut city = demo_city();
    let a = city.nodes.len() as u32;
    let template = city.nodes[0].clone();
    for (k, id) in [a, a + 1].into_iter().enumerate() {
        let mut node = template.clone();
        node.id = id;
        node.point = city.frame.to_world(2000.0 + 100.0 * k as f32, 2000.0);
        city.nodes.push(node);
    }
    let mut road = city.sd_roads[0].clone();
    road.id = 90_004;
    road.from = a;
    road.to = a + 1;
    city.sd_roads.push(road);
    assert!(plan_only(&city).count("net.orphan") >= 1);
}

#[test]
fn it_sees_a_tower_with_the_wrong_storey_height() {
    let mut city = demo_city();
    let b = city
        .buildings
        .iter_mut()
        .find(|b| b.floors >= 4)
        .expect("a multi-storey building");
    b.height_metres = b.floors as f32 * 9.0;
    assert!(plan_only(&city).count("bldg.storey") >= 1);
}

#[test]
fn a_tower_on_its_podium_is_not_an_overlap() {
    // The demo city has shop-podium towers (商住楼); they are one building.
    let city = demo_city();
    let podiums = crate::buildings::far::podiums_of(&city.buildings);
    if podiums.iter().any(|p| p.is_some()) {
        assert_eq!(plan_only(&city).count("overlap.building_building"), 0);
    }
}
