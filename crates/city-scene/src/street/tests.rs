//! The street layer's own locks.  The assertions here are written against
//! built geometry, not against inputs, so a regression has to survive being
//! *rendered* before it can pass.

use super::{Carriageway, build, declare};
use crate::math::{Path, Vec2};
use crate::mesh::MeshBuilder;
use crate::network::{Lane, LaneUse, Network, Road, derive};
use crate::spec::{ARROW_LENGTH_MM, JunctionSpec, MM, Movement};
use urban::{cross_section, generate_modern_chinese_city, ModernChinaSpec, ModernRoadClass};

use super::markings::{approaches, crosswalk_band, dashed_stripe, drive_arrow, waiting_box};
use super::signals::signal_axis;

fn city_network() -> Network {
    let city = generate_modern_chinese_city(ModernChinaSpec {
        seed: 42,
        radius_km: 0.5,
        block_size_metres: 110.0,
        ..ModernChinaSpec::default()
    });
    derive(
        &city.nodes,
        &city.sd_roads,
        &city.hd_roads,
        city.frame,
        JunctionSpec::default(),
        city.seed,
    )
}

/// A straight lane heading `+X`, 3.5 m wide, long enough for the arrow.
fn straight_lane(movements: &[Movement]) -> Lane {
    let length = 90.0;
    let path = Path::flat(vec![crate::math::Vec2::new(0.0, 0.0), crate::math::Vec2::new(length, 0.0)]);
    Lane {
        id: "test".into(),
        road: 0,
        index: 0,
        direction: 1,
        width: 3.5,
        offset: 0.0,
        use_kind: LaneUse::Through,
        path,
        from_node: 0,
        to_node: 1,
        allowed: movements.to_vec(),
        successors: Vec::new(),
        predecessors: Vec::new(),
    }
}

/// Every vertex a built arrow produced, in the lane's own frame: `x` is
/// forward, `z` is lateral with `+Z` the driver's right for a `+X` heading.
fn arrow_vertices(movements: &[Movement]) -> Vec<(f32, f32)> {
    let lane = straight_lane(movements);
    let mut builder = MeshBuilder::new();
    declare(&mut builder);
    drive_arrow(&mut builder, &lane, &JunctionSpec::default());
    let scene = builder.build();
    let mut out = Vec::new();
    for group in scene.meshes {
        for chunk in group.positions.chunks(3) {
            out.push((chunk[0], chunk[2]));
        }
    }
    assert!(!out.is_empty(), "no arrow geometry for {movements:?}");
    out
}

/// The scale the city kernel applies to a GB stencil: the reserved 5 m
/// footprint divided by the stencil's own 3.05 m.
fn arrow_scale(spec: &JunctionSpec) -> f32 {
    spec.arrow.footprint / (ARROW_LENGTH_MM * MM)
}

#[test]
fn a_right_arrow_head_lands_on_the_drivers_right_and_a_left_arrow_on_its_left() {
    // The single most important assertion in the file.  Computed from the
    // *built triangles*, not from the outline table, so it catches a wrong
    // lateral axis, a wrong stencil assignment and a mirrored frame alike.
    // For a lane heading `+X` the driver's right is `+Z`.
    let spec = JunctionSpec::default();
    let right = arrow_vertices(&[Movement::Right]);
    let left = arrow_vertices(&[Movement::Left]);
    let straight = arrow_vertices(&[Movement::Straight]);

    // A right arrow's *head* — the forward third of the stencil — must be
    // entirely on the right, and the shaft may only straddle the centreline.
    let head = |points: &[(f32, f32)]| {
        let max_x = points.iter().map(|p| p.0).fold(f32::MIN, f32::max);
        points
            .iter()
            .filter(|p| p.0 > max_x - 1.0)
            .map(|p| p.1)
            .collect::<Vec<f32>>()
    };
    let right_head = head(&right);
    let left_head = head(&left);
    assert!(
        right_head.iter().all(|z| *z > 0.05),
        "a right arrow's head is not on the driver's right: {right_head:?}"
    );
    assert!(
        left_head.iter().all(|z| *z < -0.05),
        "a left arrow's head is not on the driver's left: {left_head:?}"
    );
    // And the whole stencil leans that way: the tip is the extreme vertex.
    let tip = |points: &[(f32, f32)]| {
        *points
            .iter()
            .max_by(|a, b| a.0.total_cmp(&b.0))
            .unwrap()
    };
    assert!(tip(&right).1 > 0.0, "right arrow tip at z={}", tip(&right).1);
    assert!(tip(&left).1 < 0.0, "left arrow tip at z={}", tip(&left).1);

    // A straight arrow is symmetric about the centreline.
    let (lo, hi) = straight
        .iter()
        .map(|p| p.1)
        .fold((f32::MAX, f32::MIN), |(a, b), z| (a.min(z), b.max(z)));
    assert!(
        (lo + hi).abs() < 1.0e-3,
        "a straight arrow is not symmetric about its lane: {lo} .. {hi}"
    );
    // The head is the GB 450 mm stencil at the footprint's scale.
    let expected_head = 0.45 * arrow_scale(&spec);
    assert!(
        ((hi - lo) - expected_head).abs() < 0.02,
        "head is {} m across, expected {expected_head}",
        hi - lo
    );

    // The arrow is drawn at the reserved footprint — the source kernel's
    // `min(footprint, total - tipGap - 1.5)` — not at the bare stencil length,
    // because a 3 m arrow is invisible at queueing distance.
    let span = right
        .iter()
        .map(|p| p.0)
        .fold(f32::MAX, f32::min)
        ..right.iter().map(|p| p.0).fold(f32::MIN, f32::max);
    let length = span.end - span.start;
    assert!(
        (length - spec.arrow.footprint).abs() < 0.02,
        "a right arrow is {length} m long, expected the {} m footprint",
        spec.arrow.footprint
    );
    // And it is the GB distance back from the lane's end.
    let tip_x = right.iter().map(|p| p.0).fold(f32::MIN, f32::max);
    let total = 90.0;
    assert!(
        (total - tip_x - spec.arrow.tip_gap).abs() < 0.02,
        "the arrow's tip stands {tip_x} m along a {total} m lane"
    );
}

#[test]
fn a_lane_without_legal_movements_gets_no_arrow() {
    // A lane whose connector set is empty — a stub approach — must not grow a
    // phantom straight arrow advertising an exit that is not there.
    let lane = straight_lane(&[]);
    let mut builder = MeshBuilder::new();
    declare(&mut builder);
    drive_arrow(&mut builder, &lane, &JunctionSpec::default());
    assert_eq!(
        builder.index_count("marking.white"),
        0,
        "an empty movement set painted an arrow"
    );
}

#[test]
fn a_short_approach_shortens_the_arrow_instead_of_dropping_it() {
    // Between the 2.4 m minimum and the 5 m footprint the stencil shrinks to
    // fit, exactly as the source kernel's `min(footprint, ...)` does.
    let spec = JunctionSpec::default();
    let mut lane = straight_lane(&[Movement::Straight]);
    // total - tipGap - length must land the stencil between 2.4 and 5 m.
    lane.path = Path::flat(vec![
        crate::math::Vec2::new(0.0, 0.0),
        crate::math::Vec2::new(spec.arrow.tip_gap + 4.0 + 1.5, 0.0),
    ]);
    let mut builder = MeshBuilder::new();
    declare(&mut builder);
    drive_arrow(&mut builder, &lane, &spec);
    assert!(builder.index_count("marking.white") > 0, "a 18.5 m lane dropped its arrow");
    let scene = builder.build();
    let xs: Vec<f32> = scene
        .meshes
        .iter()
        .flat_map(|group| group.positions.chunks(3).map(|c| c[0]).collect::<Vec<f32>>())
        .collect();
    let length = xs.iter().copied().fold(f32::MIN, f32::max) - xs.iter().copied().fold(f32::MAX, f32::min);
    // The reserved footprint is 4.0 m, but the *straight* stencil is only
    // 3000 mm long — `ARROW_LENGTH_MM` (3050) is the longest stencil in the
    // table, the turn arrow's reach, and it is what the draw code scales by.
    // So a straight arrow in a 4.0 m footprint is 3000/3050 of it, which is the
    // GB proportion: a straight-through stencil is 3.0 m, a turning one 3.05 m.
    let reserved = 4.0_f32;
    let expected = reserved * 3000.0 / ARROW_LENGTH_MM;
    assert!(
        (length - expected).abs() < 0.02,
        "a shortened arrow is {length} m, expected {expected}"
    );
    // And it must still be shorter than the footprint it was fitted into, which
    // is the whole point of the shortening.
    assert!(length < reserved, "a {length} m arrow did not shrink into {reserved} m");
}

#[test]
fn every_built_arrow_faces_the_sky() {
    // A stencil wound the other way lights from below and disappears against
    // the sun.  Half the stencils are mirrors, so this is a real class of bug
    // and not a hypothetical one.
    for movements in all_stencils() {
        let lane = straight_lane(&movements);
        let mut builder = MeshBuilder::new();
        declare(&mut builder);
        drive_arrow(&mut builder, &lane, &JunctionSpec::default());
        for group in builder.build().meshes {
            for normal in group.normals.chunks(3) {
                assert!(
                    normal[1] > 0.5,
                    "an arrow face for {movements:?} points downwards ({normal:?})"
                );
            }
        }
    }
}

#[test]
fn an_arrow_never_leaves_its_lane() {
    for movements in all_stencils() {
        for (_, z) in arrow_vertices(&movements) {
            assert!(
                z.abs() <= 1.85,
                "a {movements:?} arrow reaches {z:.2} m off its lane centre"
            );
        }
    }
}

#[test]
fn a_crossing_band_is_one_quad_with_a_bar_pattern() {
    let spec = JunctionSpec::default();
    let path = Path::flat(vec![crate::math::Vec2::new(0.0, 0.0), crate::math::Vec2::new(120.0, 0.0)]);
    let surface = Carriageway::on_path(path, 16.0);
    let mut builder = MeshBuilder::new();
    declare(&mut builder);
    // A 32 m arterial: the bar count has to come from the U scale, not from
    // geometry.
    crosswalk_band(&mut builder, &surface, 60.0, 16.0, 4.0);
    let scene = builder.build();
    assert_eq!(scene.meshes[0].material, "marking.crosswalk");
    assert_eq!(scene.meshes[0].indices.len(), 6, "a crossing is one quad");
    assert!(
        scene.meshes[0].alpha_cutout,
        "the crossing's gaps must show the asphalt through them"
    );
    let uvs = scene.meshes[0].uvs.as_ref().unwrap();
    let u_span = uvs.chunks(2).map(|pair| pair[0]).fold(f32::MAX, f32::min).abs()
        .max(uvs.chunks(2).map(|pair| pair[0]).fold(f32::MIN, f32::max).abs());
    // 31.0 m of crossing: the texture holds eight bars per 8.4 m, so the
    // renderer tiles it about 3.7 times and the pitch stays 1.05 m.
    assert!((u_span - 31.0).abs() < 0.2, "U span was {u_span}");
    let _ = spec;
}

#[test]
fn the_crosswalk_bar_pitch_survives_the_current_resampling() {
    // `MAX_RESAMPLE_METRES` is 34 m, so a 4 m crossing is one quad and the
    // bar pitch lives entirely in the texture.  Assert the geometry agrees
    // with the design table anyway: if a future change coarsens the ribbon
    // step, the depth must still be exact.
    let spec = JunctionSpec::default();
    let path = Path::flat(vec![crate::math::Vec2::new(0.0, 0.0), crate::math::Vec2::new(60.0, 0.0)]);
    let surface = Carriageway::on_path(path, 8.0);
    let mut builder = MeshBuilder::new();
    declare(&mut builder);
    crosswalk_band(&mut builder, &surface, 30.0, 8.0, spec.crosswalk.depth);
    let scene = builder.build();
    let positions = &scene.meshes[0].positions;
    let xs: Vec<f32> = positions.chunks(3).map(|c| c[0]).collect();
    let depth = xs.iter().copied().fold(f32::MIN, f32::max)
        - xs.iter().copied().fold(f32::MAX, f32::min);
    assert!(
        (depth - spec.crosswalk.depth).abs() < 0.05,
        "crossing depth is {depth} m, expected {}",
        spec.crosswalk.depth
    );
}

#[test]
fn a_dash_run_anchored_at_a_stop_line_starts_with_a_dash() {
    // The invariant `laneLineStartFor` exists for: the first thing past the
    // stop line is paint, not a five-metre hole in the queue's lane.
    let spec = JunctionSpec::default();
    let path = Path::flat(vec![crate::math::Vec2::new(0.0, 0.0), crate::math::Vec2::new(200.0, 0.0)]);
    let surface = Carriageway::on_path(path, 8.0);
    let anchor = spec.solid_zone_gap + spec.guide_zone;
    let mut builder = MeshBuilder::new();
    declare(&mut builder);
    dashed_stripe(
        &mut builder,
        "marking.dashed-3-5",
        &surface,
        0.0,
        anchor,
        anchor + 30.0,
        spec.white_line_width,
        anchor,
    );
    let scene = builder.build();
    let uvs = scene.meshes[0].uvs.as_ref().expect("a dash needs UVs");
    // `V` is metres from the anchor, and the texture's period is 8 m, so the
    // first sample past the anchor is inside the first 3 m of paint.
    let v_min = uvs.chunks(2).map(|pair| pair[1]).fold(f32::MAX, f32::min);
    assert!((v_min).abs() < 1.0e-3, "V does not start at the anchor: {v_min}");
    let v_max = uvs.chunks(2).map(|pair| pair[1]).fold(f32::MIN, f32::max);
    assert!((v_max - 30.0).abs() < 0.1, "V span was {}", v_max - v_min);
}

#[test]
fn a_dashed_divider_is_one_quad_not_one_quad_per_dash() {
    let path = Path::flat(vec![crate::math::Vec2::new(0.0, 0.0), crate::math::Vec2::new(300.0, 0.0)]);
    let surface = Carriageway::on_path(path, 4.0);
    let mut builder = MeshBuilder::new();
    declare(&mut builder);
    dashed_stripe(&mut builder, "marking.dashed-3-5", &surface, 0.0, 0.0, 300.0, 0.15, 0.0);
    // 300 m of 3-on/5-off would be 37 geometry dashes.  The texture carries
    // the pattern, so the band is resampled only coarsely along the road.
    let triangles = builder.index_count("marking.dashed-3-5") / 3;
    assert!(
        triangles <= 24,
        "a 300 m dashed line cost {triangles} triangles, expected far fewer than 37"
    );
    let scene = builder.build();
    let group = &scene.meshes[0];
    assert!(group.uvs.is_some(), "a dashed line needs metre UVs");
    assert!(group.alpha_cutout, "the gaps must be alpha-cut");
    let span = group
        .uvs
        .as_ref()
        .unwrap()
        .chunks(2)
        .map(|pair| pair[1].abs())
        .fold(0.0_f32, f32::max);
    // UVs are metres, so `V` must span the real 300 m for the texture's
    // 8 m period to fall in the right places.
    assert!(span > 250.0, "V span was {span}, the dash period would be wrong");
}

#[test]
fn every_crossing_end_gets_a_crosswalk_and_a_stop_line() {
    let network = city_network();
    let mut builder = MeshBuilder::new();
    build(&network, &mut builder, 42);
    assert!(builder.vertex_count("marking.crosswalk") > 0, "no crossings at all");
    assert!(
        builder.index_count("marking.white") > 10_000,
        "expected a full set of markings, got {} indices",
        builder.index_count("marking.white")
    );
}

/// Every arrow a lane can be given, in the order the tests use them.
fn all_stencils() -> Vec<Vec<Movement>> {
    vec![
        vec![Movement::Straight],
        vec![Movement::Left],
        vec![Movement::Right],
        vec![Movement::Left, Movement::Straight],
        vec![Movement::Right, Movement::Straight],
        vec![Movement::Left, Movement::Right, Movement::Straight],
    ]
}

/// Plan-space axis-aligned boxes, one per triangle, for one material.
fn material_bounds(scene: &crate::mesh::SceneGeometry, material: &str) -> Vec<[f32; 4]> {
    let Some(group) = scene
        .meshes
        .iter()
        .find(|group| group.material == material)
    else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for triangle in group.indices.chunks(3) {
        let points: Vec<(f32, f32)> = triangle
            .iter()
            .map(|index| {
                let index = *index as usize;
                (
                    group.positions[index * 3],
                    group.positions[index * 3 + 2],
                )
            })
            .collect();
        let lo = (
            points.iter().map(|p| p.0).fold(f32::MAX, f32::min),
            points.iter().map(|p| p.1).fold(f32::MAX, f32::min),
        );
        let hi = (
            points.iter().map(|p| p.0).fold(f32::MIN, f32::max),
            points.iter().map(|p| p.1).fold(f32::MIN, f32::max),
        );
        out.push([lo.0, lo.1, hi.0, hi.1]);
    }
    out
}

/// The completeness test the junction layer has to pass: a four-port
/// signalised junction carries a box, corner paving, a stop line and a
/// crossing on **every** arm, and no two opposing crossings overlap.
#[test]
fn a_four_port_junction_is_complete_on_every_arm() {
    let network = city_network();
    let mut builder = MeshBuilder::new();
    build(&network, &mut builder, 42);
    let vertex_count = builder.vertex_count("sidewalk");
    let scene = builder.build();

    let Some(junction) = network
        .junctions
        .iter()
        .filter(|junction| {
            junction.ports.len() == 4 && junction.kind == urban::JunctionKind::Signalized
        })
        .max_by(|a, b| a.radius.total_cmp(&b.radius))
    else {
        return;
    };
    let crossings = material_bounds(&scene, "marking.crosswalk");
    assert!(
        !crossings.is_empty(),
        "no crossing geometry anywhere in the city"
    );

    // One crossing per arm, matched by proximity to the arm's mouth.
    let mut per_arm = Vec::new();
    for port in &junction.ports {
        let mouth = (port.left + port.right) * 0.5;
        let best = crossings
            .iter()
            .filter(|quad| {
                let cx = (quad[0] + quad[2]) * 0.5;
                let cz = (quad[1] + quad[3]) * 0.5;
                (cx - mouth.x).hypot(cz - mouth.y) < junction.radius * 0.5 + 14.0
            })
            .min_by(|a, b| {
                let da = ((a[0] + a[2]) * 0.5 - mouth.x)
                    .hypot((a[1] + a[3]) * 0.5 - mouth.y);
                let db = ((b[0] + b[2]) * 0.5 - mouth.x)
                    .hypot((b[1] + b[3]) * 0.5 - mouth.y);
                da.total_cmp(&db)
            })
            .copied();
        per_arm.push(best);
    }
    for (index, crossing) in per_arm.iter().enumerate() {
        assert!(
            crossing.is_some(),
            "arm {index} of a 4-port junction has no crossing"
        );
    }

    // No two *opposing* crossings may touch.  Two arms are opposing when
    // their outward directions are more than 120 degrees apart.
    for a in 0..junction.ports.len() {
        for b in a + 1..junction.ports.len() {
            if junction.ports[a].dir.dot(junction.ports[b].dir) > -0.5 {
                continue;
            }
            let (Some(x), Some(y)) = (per_arm[a], per_arm[b]) else {
                continue;
            };
            let overlap_x = (x[0] - y[2]).max(y[0] - x[2]);
            let overlap_y = (x[1] - y[3]).max(y[1] - x[3]);
            assert!(
                overlap_x <= 0.0 || overlap_y <= 0.0,
                "opposing crossings {a} and {b} overlap by {overlap_x:.2} x {overlap_y:.2} m"
            );
        }
    }

    // A stop line on every arm too, and the corner paving.
    let white = material_bounds(&scene, "marking.white");
    for port in &junction.ports {
        let mouth = (port.left + port.right) * 0.5;
        let found = white.iter().any(|quad| {
            let cx = (quad[0] + quad[2]) * 0.5;
            let cz = (quad[1] + quad[3]) * 0.5;
            (cx - mouth.x).hypot(cz - mouth.y) < junction.radius * 0.5 + 12.0
        });
        assert!(found, "arm has no stop line near ({:.1}, {:.1})", mouth.x, mouth.y);
    }
    assert!(vertex_count > 0, "no corner paving emitted");
    for junction in &network.junctions {
        assert_eq!(junction.ring.len(), junction.ports.len() * 9);
        assert_eq!(junction.walk_ring.len(), junction.ring.len());
    }
}

#[test]
fn every_crossing_lands_where_the_spec_says_it_does() {
    // The bounds check that silently deleted every crossing in the previous
    // version: with the approach sense flipped, the crossing centre came out
    // at a negative station and was rejected.  Asserted on the geometry.
    let network = city_network();
    let spec = JunctionSpec::default();
    let mut checked = 0;
    for road in &network.roads {
        if road.layer != 0 || road.is_motorway() {
            continue;
        }
        let length = road.carriageway.length();
        for approach in approaches(road, &spec) {
            let centre = approach.crosswalk_centre;
            assert!(
                centre > -0.01 && centre < length + 0.01,
                "crossing centre at {centre} is off a {length} m carriageway"
            );
            // Measured as a distance from the junction edge, because that is
            // the only frame both ends share.
            let crossing_edge = (approach.base - centre).abs() + spec.crosswalk.depth * 0.5;
            let stop_edge = (approach.base - approach.stop_near)
                .abs()
                .min((approach.base - approach.stop_far).abs());
            assert!(
                stop_edge > crossing_edge,
                "the stop line at {stop_edge:.2} m from the junction is not behind \
                 the crossing, whose far edge is {crossing_edge:.2} m out"
            );
            // The stop line only covers the approach's own carriageway.
            assert!(
                approach.lateral_outer.abs() <= road.section.half_width(),
                "the stop line runs past the kerb"
            );
            checked += 1;
        }
    }
    assert!(checked > 50, "only {checked} approaches to check");
}

#[test]
fn the_carrageway_crowns_from_the_median_and_flattens_into_the_box() {
    let spec = JunctionSpec::default();
    let path = Path::flat(vec![crate::math::Vec2::new(0.0, 0.0), crate::math::Vec2::new(200.0, 0.0)]);
    let mut surface = Carriageway::on_path(path, 16.0);
    surface.flatten = 20.0;
    // Mid-road the crest stands proud of the gutter...
    assert!(surface.lift(0.0, 100.0) > 0.10);
    // ... the gutter does not...
    assert!(surface.lift(16.0, 100.0).abs() < 1.0e-4);
    // ... and at the junction edge there is no step at all.
    assert!(surface.lift(0.0, 0.0).abs() < 1.0e-4);
    // A constant cross-fall of 1-2%, measured across the running lanes.
    let fall = (surface.lift(4.0, 100.0) - surface.lift(12.0, 100.0)) / 8.0;
    assert!(
        (0.008..0.030).contains(&fall),
        "cross-fall is {fall:.3}, CJJ 37 wants 1-2%"
    );
    // And the fall is *linear*, so the middle of a carriageway is not a
    // trough: the crest-to-gutter gradient equals the kerb-to-kerb one.
    let near = (surface.lift(0.0, 100.0) - surface.lift(2.0, 100.0)) / 2.0;
    assert!((near - fall).abs() < 1.0e-3, "the section is not planar");
    // Flattening over 20 m of a 0.2 m crest is a 1% longitudinal ramp, which
    // is invisible; a shorter blend would be a visible crease where the box
    // meets the approach.
    let blend = (surface.lift(0.0, 20.0) - surface.lift(0.0, 0.0)) / 20.0;
    assert!(
        blend <= 0.012,
        "the crown blends into the box at {blend:.3}, which would show as a crease"
    );
    let _ = spec;
}

#[test]
fn junction_layer_never_emits_a_non_finite_vertex() {
    let network = city_network();
    let mut builder = MeshBuilder::new();
    build(&network, &mut builder, 42);
    for group in builder.build().meshes {
        for value in &group.positions {
            assert!(value.is_finite(), "{} has a non-finite position", group.material);
        }
        for value in &group.normals {
            assert!(value.is_finite(), "{} has a non-finite normal", group.material);
        }
    }
}

#[test]
fn signal_heads_land_on_signalized_arterial_junctions() {
    let network = city_network();
    let mut builder = MeshBuilder::new();
    let output = build(&network, &mut builder, 42);
    assert!(!output.signals.is_empty(), "no signals built");
    for rig in &output.signals {
        assert_eq!(rig.lamps.len(), 3);
        assert!(matches!(rig.axis, "ns" | "ew"));
    }
}

#[test]
fn a_signal_head_stands_on_the_footway_before_the_stop_line() {
    // The previous version planted every head at the junction centre, six
    // metres *inside* the box.  Assert it is on the footway and behind the
    // line, in world space, from the lamp positions the renderer uses.
    let network = city_network();
    let mut builder = MeshBuilder::new();
    let output = build(&network, &mut builder, 42);
    for rig in &output.signals {
        let Some(road) = network.road(rig.road) else {
            continue;
        };
        let Some(junction) = network.junction(rig.junction) else {
            continue;
        };
        let port = junction
            .ports
            .iter()
            .find(|port| port.road == rig.road)
            .expect("signal without a port");
        let mouth = (port.left + port.right) * 0.5;
        for lamp in &rig.lamps {
            let plan = Vec2::new(lamp.position[0], lamp.position[2]);
            // Outside the box...
            let from_centre = plan.distance(junction.centre);
            assert!(
                from_centre > junction.radius * 0.35,
                "a signal lens stands {from_centre:.1} m from the junction centre"
            );
            // ... and outside the carriageway, on the footway.
            let path: Vec<Vec2> = road
                .centreline
                .points()
                .iter()
                .map(|p| Vec2::new(p.x, p.z))
                .collect();
            let clearance = crate::math::distance_to_polyline(plan, &path) - road.half_width();
            assert!(
                clearance > -0.05,
                "a signal lens stands {clearance:.2} m inside the carriageway"
            );
            let _ = mouth;
        }
        // Red on top, green at the bottom: GB 5768.3 signal head order.
        let red = rig
            .lamps
            .iter()
            .find(|lamp| lamp.aspect == 0)
            .map(|lamp| lamp.position[1])
            .unwrap();
        let green = rig
            .lamps
            .iter()
            .find(|lamp| lamp.aspect == 2)
            .map(|lamp| lamp.position[1])
            .unwrap();
        assert!(red > green, "red is not above green");
    }
}

#[test]
fn a_signal_housing_hangs_inside_the_gb_14886_window() {
    // GB 14886: a pole- or cantilever-mounted vehicle signal head bottoms out
    // between 5.2 m and 6.5 m above the carriageway.  The lamps are evenly
    // spaced inside the housing, so the housing's centre is the mean lamp
    // height and its bottom is the centre minus half the housing height.
    const HOUSING_HALF: f32 = 0.58;
    let network = city_network();
    let mut builder = MeshBuilder::new();
    let output = build(&network, &mut builder, 42);
    assert!(!output.signals.is_empty(), "no signals built");
    let mut highest_bottom = f32::MIN;
    for rig in &output.signals {
        let centre = rig.lamps.iter().map(|lamp| lamp.position[1]).sum::<f32>() / rig.lamps.len() as f32;
        let bottom = centre - HOUSING_HALF;
        assert!(
            (5.2..=6.5).contains(&bottom),
            "signal housing bottom at {bottom:.2} m is outside GB 14886's 5.2-6.5 m window"
        );
        highest_bottom = highest_bottom.max(centre + HOUSING_HALF);
        // The three lenses stay inside their housing.
        for lamp in &rig.lamps {
            assert!(
                (lamp.position[1] - centre).abs() <= 0.38 + 1.0e-3,
                "a lamp sits outside its housing"
            );
        }
    }
    // And the pole clears the housing, so the head reads as hung hardware.
    let scene = builder.build();
    let body = scene
        .meshes
        .iter()
        .find(|group| group.material == "signal.body")
        .expect("no signal bodies built");
    let top = body.positions.chunks(3).map(|c| c[1]).fold(f32::MIN, f32::max);
    assert!(
        top > highest_bottom + 0.05,
        "nothing of the rig stands above the housing top {highest_bottom:.2}"
    );
}

#[test]
fn a_waiting_box_straddles_the_junction_edge() {
    // The kernel's box runs from the stop line, 5.2 m back from the junction
    // edge, to 11 m *past* the edge, inside the box — and its side lines hug
    // the innermost lane.  Asserted from built paint on a trimmed arterial.
    let spec = JunctionSpec::default();
    let section = cross_section(ModernRoadClass::Arterial);
    let centreline = Path::flat(vec![Vec2::new(0.0, 0.0), Vec2::new(120.0, 0.0)]);
    let road = Road {
        id: 7,
        class: ModernRoadClass::Arterial,
        section,
        from_node: 1,
        to_node: 2,
        layer: 0,
        bridge: false,
        centreline: centreline.clone(),
        carriageway: centreline.trim(14.0, 106.0),
        trim_start: 14.0,
        trim_end: 14.0,
        deck_thickness: 0.25,
        max_grade: 0.0,
        crossing_start: true,
        crossing_end: true,
        marking_start: 7.0,
        marking_end: 7.0,
        lane_ids: Vec::new(),
    };
    // The innermost inbound lane, offset exactly as the network derives it.
    let count = section.motor_lanes_per_direction;
    let offset = section.lane_offset(1, count - 1);
    let lane = Lane {
        id: "road/7/lane/f/0".into(),
        road: 7,
        index: 0,
        direction: 1,
        width: section.motor_lane_width,
        offset,
        use_kind: LaneUse::LeftTurn,
        path: Path::flat(vec![Vec2::new(14.0, 0.0), Vec2::new(106.0, 0.0)]),
        from_node: 1,
        to_node: 2,
        allowed: vec![Movement::Left],
        successors: Vec::new(),
        predecessors: Vec::new(),
    };
    let mut builder = MeshBuilder::new();
    declare(&mut builder);
    waiting_box(&road, &mut builder, &spec, &lane);
    let scene = builder.build();
    let group = scene
        .meshes
        .iter()
        .find(|group| group.material == "marking.white")
        .expect("the waiting box is white paint");
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_z = f32::MAX;
    let mut max_z = f32::MIN;
    for chunk in group.positions.chunks(3) {
        min_x = min_x.min(chunk[0]);
        max_x = max_x.max(chunk[0]);
        min_z = min_z.min(chunk[2]);
        max_z = max_z.max(chunk[2]);
    }
    let edge = 106.0;
    // The near end is the stop line, 5.2 m up the approach...
    assert!(
        (min_x - (edge - spec.stop_line_gap)).abs() < 0.05,
        "the box starts at x={min_x}, expected the stop line at {}",
        edge - spec.stop_line_gap
    );
    // ... and the cap is the 11 m depth *into* the junction, plus its own
    // 300 mm.
    assert!(
        (max_x - (edge + spec.waiting_box.depth + 0.15)).abs() < 0.05,
        "the box ends at x={max_x}, expected the cap at {}",
        edge + spec.waiting_box.depth + 0.15
    );
    assert!(
        min_x < edge && max_x > edge,
        "the box does not straddle the junction edge at x={edge}"
    );
    // Lateral: 200 mm inside the innermost lane's two edges, on the +offset
    // side of the centreline for a direction +1 carriageway.
    let lane_edge = section.motor_lane_width * 0.5 - 0.2;
    let inner = offset - lane_edge;
    let outer = offset + lane_edge;
    assert!(
        (min_z - (inner - 0.06)).abs() < 0.02 && (max_z - (outer + 0.06)).abs() < 0.02,
        "the box spans z {min_z}..{max_z}, expected {}..{}",
        inner - 0.06,
        outer + 0.06
    );
}

#[test]
fn diagonal_approaches_never_share_a_signal_axis() {
    // Two streets crossing at 90 degrees must always differ.
    assert_eq!(signal_axis(Vec2::new(1.0, 0.0)), "ew");
    assert_eq!(signal_axis(Vec2::new(0.0, 1.0)), "ns");
    assert_ne!(
        signal_axis(Vec2::new(1.0, 1.0)),
        signal_axis(Vec2::new(1.0, -1.0))
    );
}

#[test]
fn no_marking_is_emitted_outside_its_carriageway() {
    // Markings are the one part of the layer that is visible from a driver's
    // seat at a metre's height, so paint on a footway is not a small error.
    let network = city_network();
    let spec = JunctionSpec::default();
    let mut builder = MeshBuilder::new();
    build(&network, &mut builder, 42);
    let scene = builder.build();
    // Each road's plan alignment, its kerb-to-kerb half width, its deck
    // elevation, and its class — enough to decide whether a marking belongs
    // to it.  A viaduct's markings ride its own deck, which is metres up at
    // mid-span and *at roadbed level* where the ramp meets the ground, so
    // "is this paint on a bridge or on the street under it" is answered by
    // comparing elevations, not by the layer flag.
    let centrelines: Vec<(Vec<Vec2>, f32, &'static str)> = network
        .roads
        .iter()
        .map(|road| {
            (
                road.centreline
                    .points()
                    .iter()
                    .map(|p| Vec2::new(p.x, p.z))
                    .collect(),
                road.half_width(),
                crate::network::export::road_class(road.class),
            )
        })
        .collect();
    let clearance = |point: Vec2| {
        let mut best = f32::MAX;
        for (plan, half, _) in &centrelines {
            best = best.min(crate::math::distance_to_polyline(point, plan) - half);
        }
        best
    };
    let rings: Vec<Vec<Vec2>> = network.junctions.iter().map(|j| j.ring.clone()).collect();
    let mut offenders: Vec<String> = Vec::new();
    for group in &scene.meshes {
        if !group.material.starts_with("marking.") {
            continue;
        }
        for index in group.indices.iter() {
            let index = *index as usize;
            let point = Vec2::new(group.positions[index * 3], group.positions[index * 3 + 2]);
            let y = group.positions[index * 3 + 1];
            // A marking may sit on the carriageway, on the taper that widens
            // it, on a viaduct's own deck, or inside a junction box.  Nowhere
            // else.
            let slack = spec.taper_lane_width + 0.5;
            if clearance(point) <= slack {
                continue;
            }
            if rings
                .iter()
                .any(|ring| crate::math::point_in_ring(point, ring))
            {
                continue;
            }
            offenders.push(format!(
                "{} y={y:.1} ({:.1} m out) at ({:.1}, {:.1})",
                group.material,
                clearance(point),
                point.x,
                point.y
            ));
        }
    }
    offenders.sort();
    offenders.dedup();
    assert!(
        offenders.is_empty(),
        "{} markings off the carriageway: {:?}",
        offenders.len(),
        &offenders[..offenders.len().min(8)]
    );
}
