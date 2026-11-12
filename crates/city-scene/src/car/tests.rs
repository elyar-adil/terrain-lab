use super::*;

fn extent(scene: &crate::mesh::SceneGeometry, key: &str) -> ([f32; 3], [f32; 3]) {
    let group = scene
        .meshes
        .iter()
        .find(|g| g.material == key)
        .unwrap_or_else(|| panic!("no {key}"));
    let mut lo = [f32::MAX; 3];
    let mut hi = [f32::MIN; 3];
    for v in group.positions.chunks(3) {
        for k in 0..3 {
            lo[k] = lo[k].min(v[k]);
            hi[k] = hi[k].max(v[k]);
        }
    }
    (lo, hi)
}

#[test]
fn every_design_is_a_car_sized_car_standing_on_its_wheels() {
    for fleet in [1_u64, 77, 20_240_607] {
        let mut builder = MeshBuilder::new();
        build_prototypes(&mut builder, fleet);
        let scene = builder.build();
        for variant in 0..CAR_VARIANTS {
            let name = format!("fleet {fleet} design {variant}");
            // Length along +X (placed with `yaw_along_x`), width over the mirrors.
            let (lo, hi) = extent(&scene, &car_key("body", variant));
            let (length, width, height) = (hi[0] - lo[0], hi[2] - lo[2], hi[1]);
            assert!((2.8..5.3).contains(&length), "{name} is {length:.2} m long");
            assert!((1.45..2.35).contains(&width), "{name} is {width:.2} m wide");
            assert!(
                (1.35..2.0).contains(&height),
                "{name} is {height:.2} m high"
            );
            let (wlo, whi) = extent(&scene, &car_key("wheel", variant));
            assert!(
                wlo[1].abs() < 0.012,
                "{name}'s wheels are {:.3} m off the road",
                wlo[1]
            );
            assert!(
                whi[0] - wlo[0] > 2.3 && whi[2] - wlo[2] < width,
                "{name}: axles across the car"
            );
            // The tyres are inside the body's width, and clear of the arches.
            assert!(
                whi[2] < hi[2] && wlo[2] > lo[2],
                "{name}'s tyres stand proud"
            );
            // The seats are inside the roof, and the far model is the same size.
            let (_, ihi) = extent(&scene, &car_key("interior", variant));
            assert!(ihi[1] < height, "{name}'s interior comes through its roof");
            let (flo, fhi) = extent(&scene, &car_key("far.body", variant));
            assert!(
                (fhi[0] - flo[0] - length).abs() < 0.1,
                "{name}: far model length"
            );
            // Every part was made.
            for part in NEAR_PARTS {
                let key = car_key(part, variant);
                assert!(
                    scene.meshes.iter().any(|g| g.material == key),
                    "{name} has no {part}"
                );
            }
        }
    }
}

#[test]
fn a_seed_is_a_design_and_seeds_differ() {
    let a = design::design(41);
    let b = design::design(41);
    assert_eq!(a.top, b.top);
    assert_eq!(a.head_style, b.head_style);
    // A hundred seeds: every kind of lamp, tread and rim turns up, and the
    // proportions are never twice the same.
    let designs: Vec<Spec> = (0..100).map(design::design).collect();
    let mut lengths: Vec<i32> = designs
        .iter()
        .map(|d| ((d.top[d.top.len() - 1].0 - d.top[0].0) * 1000.0) as i32)
        .collect();
    lengths.sort_unstable();
    lengths.dedup();
    assert!(
        lengths.len() > 90,
        "only {} lengths in 100 designs",
        lengths.len()
    );
    for style in [
        lamps::HeadStyle::Halogen,
        lamps::HeadStyle::Projector,
        lamps::HeadStyle::Matrix,
        lamps::HeadStyle::Slim,
    ] {
        assert!(
            designs.iter().any(|d| d.head_style == style),
            "no {style:?}"
        );
    }
    for style in [
        wheels::RimStyle::Spoke,
        wheels::RimStyle::Split,
        wheels::RimStyle::Mesh,
        wheels::RimStyle::Turbine,
        wheels::RimStyle::Aero,
        wheels::RimStyle::Steel,
    ] {
        assert!(designs.iter().any(|d| d.rim.style == style), "no {style:?}");
    }
    // And each is a sane car: wheels inside the body, glasshouse in order.
    for (seed, d) in designs.iter().enumerate() {
        let [rear_foot, rear_head, front_head, front_foot] = d.cabin;
        assert!(
            rear_foot < rear_head && rear_head < front_head && front_head < front_foot,
            "seed {seed}"
        );
        let (tail, nose) = (d.top[0].0, d.top[d.top.len() - 1].0);
        let reach = d.wheelbase * 0.5 + d.wheel_r + 0.12;
        assert!(
            -reach > tail && reach < nose,
            "seed {seed}: wheels past the bumpers"
        );
        assert!(front_head - rear_head > 0.7, "seed {seed}: no roof");
        assert!(
            d.tyre.rim_r < d.wheel_r - 0.05,
            "seed {seed}: no tyre on the rim"
        );
    }
}

#[test]
fn a_placed_car_is_one_design_in_every_part() {
    let parts = instances_of(12.0, 0.0, -40.0, 0.3, [0.5, 0.1, 0.1]);
    assert_eq!(parts.len(), NEAR_PARTS.len() + FAR_PARTS.len());
    let variant = car_variant_at(12.0, -40.0);
    for (key, _) in &parts {
        assert_eq!(key.contains('~'), variant > 0, "{key}");
    }
}
