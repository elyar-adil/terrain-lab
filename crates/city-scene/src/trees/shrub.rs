//! The clipped shrub and the grass tuft: ground clutter, not trees.
//!
//! Both are prototypes built once and instanced by the thousand, and both
//! exist because their previous versions were the props that made the last
//! port's streets read as a model railway: a solid green ball for every hedge
//! and a transparent card standing in for every tuft.

use super::TAU;
use crate::math::{Rng, Vec3};
use crate::mesh::{Instance, MeshBuilder};
use crate::network::Network;

/// A clipped shrub, instanced along medians and park edges.
///
/// Two crossed cards of a real leaf-mass silhouette rather than a low-poly ball:
/// a ball of 7-sided quads is the other thing that made the last port's parks
/// read as props, and this geometry is seen from two metres away.
pub fn build_shrub_prototype(builder: &mut MeshBuilder) {
    super::declare(builder);
    builder.style(
        "hedge",
        crate::mesh::GroupStyle {
            cast_shadow: true,
            receive_shadow: true,
            alpha_cutout: false,
            dynamic: false,
        },
    );
    // A dense clipped dome: a vertical profile that is widest just below the
    // middle and a flat top, which is exactly what a pruning shear leaves.
    let (rings, sides) = (5_u32, 9_u32);
    let mut points: Vec<Vec3> = Vec::new();
    for ring in 0..=rings {
        let phi = ring as f32 / rings as f32 * std::f32::consts::PI;
        let y = phi.cos().abs().powf(0.7);
        let radius = phi.sin().powf(0.8) * (0.94 - 0.10 * ring as f32 / rings as f32);
        for side in 0..sides {
            let theta = side as f32 / sides as f32 * TAU;
            // Lobed rather than circular: a clipped hedge is never a lathe form.
            let lobe = 1.0 + 0.07 * (theta * 3.0).sin();
            points.push(Vec3::new(
                radius * lobe * theta.cos() * 0.62,
                y * 0.62 + 0.10,
                radius * lobe * theta.sin() * 0.62,
            ));
        }
    }
    for ring in 0..rings as usize {
        for side in 0..sides as usize {
            let a = ring * sides as usize + side;
            let b = ring * sides as usize + (side + 1) % sides as usize;
            let c = (ring + 1) * sides as usize + (side + 1) % sides as usize;
            let d = (ring + 1) * sides as usize + side;
            builder.quad("hedge", points[a], points[b], points[c], points[d], None);
        }
    }
    // One transform list places every shrub in the city.
    builder.bind("hedge", "hedge");
}

/// A crossed-quad grass tuft, sampling the baked tuft card so the blades are
/// blades and not a green box.
pub fn build_tuft_prototype(builder: &mut MeshBuilder) {
    super::declare(builder);
    builder.style(
        "tuft",
        crate::mesh::GroupStyle {
            cast_shadow: false,
            receive_shadow: true,
            alpha_cutout: true,
            dynamic: false,
        },
    );
    for index in 0..2 {
        let angle = index as f32 * std::f32::consts::FRAC_PI_2;
        let (sin, cos) = angle.sin_cos();
        let axis_u = Vec3::new(cos, 0.0, sin);
        // Metre UVs: the tuft texture's tile is one metre, so `0..1` is the whole
        // image on a one-metre card.
        let corners = [
            Vec3::new(0.0, 0.0, 0.0) - axis_u * 0.5,
            Vec3::new(0.0, 0.0, 0.0) + axis_u * 0.5,
            Vec3::new(0.0, 1.0, 0.0) + axis_u * 0.35,
            Vec3::new(0.0, 1.0, 0.0) - axis_u * 0.35,
        ];
        // White, because the albedo is in the texture now. The old `tuft` card
        // was transparent, so this quad was a solid `[0.34, 0.52, 0.26]` cross
        // standing in every park.
        let tint = [1.0_f32, 1.0_f32, 1.0_f32];
        builder.quad_uv(
            "tuft",
            corners[0],
            corners[1],
            corners[2],
            corners[3],
            [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            Some(tint),
        );
        builder.quad_uv(
            "tuft",
            corners[1],
            corners[0],
            corners[3],
            corners[2],
            [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            Some(tint),
        );
    }
    builder.bind("tuft", "tuft");
}

/// Median shrubs along a divided road, as instances of the shrub prototype.
pub fn plant_median_shrubs(
    network: &Network,
    builder: &mut MeshBuilder,
    seed: u32,
) -> usize {
    let mut rng = Rng::new(seed ^ 0x5b12);
    let mut count = 0;
    for road in &network.roads {
        if road.layer != 0 || !road.section.has_median() {
            continue;
        }
        let path = road.carriageway.clone();
        let length = path.length();
        let half = road.section.median_metres * 0.5;
        let mut station = 4.0;
        while station < length - 4.0 {
            let point = path.offset_at(station, (rng.unit() - 0.5) * half * 0.7, 0.0);
            // Downwards only, for the same reason the tree tint is: an instance
            // tint above 1.0 clips in the payload's `u8` and the shrub ends up
            // brighter than any clipped hedge can be.
            let green = 0.80 + rng.unit() * 0.20;
            builder.add_instance(
                "hedge",
                Instance::new(
                    point.x,
                    crate::street::level::MEDIAN + 0.10,
                    point.z,
                    rng.unit() * TAU,
                    0.8 + rng.unit() * 0.5,
                    [green * 0.94, green, green * 0.88],
                ),
            );
            count += 1;
            station += 1.35;
        }
    }
    count
}
