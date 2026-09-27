//! The structure that carries a street over something else: parapets with a
//! capping rail, and piers on real ground.

use crate::mesh::MeshBuilder;
use crate::network::Road;

use super::{Carriageway, offset_path, sweep};

/// Parapets, piers and abutments for an elevated road.
///
/// The previous renderer lifted a bridge ribbon six metres and left it floating.
/// A viaduct without a deck edge and piers reads as a road pasted onto the sky,
/// which is worse than not drawing the bridge at all.
pub(super) fn bridge_structure(road: &Road, builder: &mut MeshBuilder) {
    if road.layer == 0 {
        return;
    }
    let path = road.carriageway.clone();
    let length = path.length();
    if length < 4.0 {
        return;
    }
    let surface = Carriageway::on_path(path, road.half_width());
    let half = road.half_width();
    // Parapet: a capping ribbon plus its full-height face on both sides.
    for side in [-1.0_f32, 1.0] {
        let edge = side * (half - 0.3);
        let cap = offset_path(&surface, edge);
        builder.ribbon("bridge.concrete", &cap, -0.18, 0.18, 0.0, length, 0.85, None);
        sweep(builder, "bridge.concrete", &Carriageway::on_path(cap, 0.2), -0.18, 0.18, 0.0, length, 0.85);
    }
    // Piers every 28 m, skipping any that would land in the carriageway of
    // another road — the same avoidance the source kernel applies.
    let mut station = 14.0;
    while station < length - 12.0 {
        let (position, tangent) = surface.path.sample(station);
        let top = position.y - road.deck_thickness;
        if top > 1.2 {
            let height = top;
            let normal = tangent.left_normal();
            let base = crate::math::Vec3::new(position.x - normal.x * 0.7, 0.0, position.z - normal.y * 0.7);
            let top_point = crate::math::Vec3::new(position.x - normal.x * 0.7, height, position.z - normal.y * 0.7);
            builder.tube("bridge.concrete", base, top_point, 0.7, 0.6, 6, None);
            // Pier cap, spread to carry the deck.
            let cap_a = crate::math::Vec3::new(position.x - normal.x * 1.1, height, position.z - normal.y * 1.1);
            let cap_b = crate::math::Vec3::new(position.x + normal.x * 1.1, height, position.z + normal.y * 1.1);
            builder.tube("bridge.concrete", cap_a, cap_b, 0.6, 0.6, 4, None);
        }
        station += 28.0;
    }
}
