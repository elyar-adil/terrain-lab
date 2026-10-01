//! Where a texture comes from, and the physical size it tiles at.
//!
//! The scene layer bakes every surface it draws. Splitting the bake across
//! modules keeps each one owned by whoever can actually judge it: the building
//! elevations live with the building code, the foliage cards live with the tree
//! code, and the ground and paint live with the street code. They meet here,
//! behind one list, so `scene.rs` never has to know.

pub use crate::facades::{
    FACADE_TILE_H, FACADE_TILE_W, GROUND_FLOOR_TILE_W, facade_textures, ground_floor_textures,
    roof_texture,
};
pub use crate::leaf_cards::{leaf_card_textures, tuft_texture};
pub use crate::textures::{
    GROUND_TILE_M, asphalt_texture, crosswalk_texture, dashed_line_texture, grass_texture,
    paving_texture, signage_texture,
};

use crate::textures::BakedTexture;

/// Every texture one city needs, keyed by the material the renderer binds.
///
/// Order is not significant — the renderer looks each up by name — but every
/// entry must correspond to a material some geometry actually uses. A texture
/// with no geometry is dead payload, and a geometry with no texture renders as
/// flat untextured colour, so `facade_elevations_cover_every_facade_material`
/// and its siblings assert both directions.
pub fn standard_set(facade_size: usize, ground_size: usize) -> Vec<BakedTexture> {
    let mut set = facade_textures(facade_size);
    set.extend(ground_floor_textures(ground_size));
    set.push(roof_texture(ground_size));
    for covering in crate::facades::RoofCovering::ALL {
        set.push(crate::facades::pitched_roof_texture(covering, ground_size));
    }
    set.push(asphalt_texture(ground_size));
    set.push(paving_texture(ground_size));
    set.push(grass_texture(ground_size));
    set.push(crosswalk_texture(ground_size));
    set.push(dashed_line_texture(64, 3.0, 5.0));
    set.push(dashed_line_texture(64, 6.0, 9.0));
    // One leaf card per species, not one for all foliage: a `紫花风铃木` in
    // flower and a `水杉` in winter have nothing in common in silhouette, and a
    // single generic leaf texture is exactly what made the last port's trees
    // read as green blobs.
    set.extend(leaf_card_textures(ground_size));
    set.push(tuft_texture(ground_size));
    set.push(signage_texture(256));
    set
}
