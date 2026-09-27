//! The shared weathering ("破损/老化") system.  One set of masks — edge wear,
//! grime, cracks, water stains, moss — is evaluated in surface space and every
//! material baker (building brick, floor pavers, asphalt, stone) consumes the
//! same functions, so a courtyard wall and the pavement beneath it age with
//! the same visual language instead of each renderer inventing its own.

use crate::noise::{fbm, hash01, value_noise};

/// Which surface family the masks evaluate for; layouts differ but the ageing
/// semantics are shared.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceKind {
    Brick,
    Tile,
    Asphalt,
    Concrete,
    Stone,
}

/// Intensity knobs, all in [0, 1].
#[derive(Clone, Copy, Debug)]
pub struct WeatheringProfile {
    /// Chipping and rounding at brick/tile edges and pavement corners.
    pub edge_wear: f32,
    /// Dirt accumulation: strong near the ground and on upward faces.
    pub grime: f32,
    /// Crack density driven by fBm ridges.
    pub cracks: f32,
    /// Vertical water-stain streaking (rain runs down, never up).
    pub stains: f32,
    /// Biological growth in damp joints.
    pub moss: f32,
    pub seed: u32,
}

impl Default for WeatheringProfile {
    fn default() -> Self {
        Self {
            edge_wear: 0.45,
            grime: 0.4,
            cracks: 0.25,
            stains: 0.3,
            moss: 0.2,
            seed: 7,
        }
    }
}

/// Brick courses in surface space.  `u` runs along the wall, `v` up.
#[derive(Clone, Copy, Debug)]
pub struct BrickCell {
    pub course: i32,
    pub brick: i32,
    /// Position inside the brick, each in [0, 1].
    pub local_u: f32,
    pub local_v: f32,
    /// True in the mortar joint between bricks.
    pub joint: bool,
}

/// Evaluate which brick a surface point belongs to.  `courses` is the number
/// of brick rows across the texture; the running offset alternates per course
/// like real masonry.
pub fn brick_cell(u: f32, v: f32, courses: i32, joint_uv: f32) -> BrickCell {
    let course = ((v * courses as f32).floor() as i32).clamp(0, courses - 1);
    let v_local = (v * courses as f32) - course as f32;
    // Running bond: alternate courses shift by half a brick.
    let offset = if course % 2 == 0 { 0.0 } else { 0.5 };
    let bricks = courses * 2;
    let brick_x = ((u + offset) * bricks as f32).floor() as i32;
    let u_local = ((u + offset) * bricks as f32) - brick_x as f32;
    let joint = (v_local < joint_uv || v_local > 1.0 - joint_uv)
        || (u_local < joint_uv * 2.0 || u_local > 1.0 - joint_uv * 2.0);
    BrickCell {
        course,
        brick: brick_x,
        local_u: u_local,
        local_v: v_local,
        joint,
    }
}

/// Square paver/tile layout with grouted joints.
#[derive(Clone, Copy, Debug)]
pub struct TileCell {
    pub tile_x: i32,
    pub tile_y: i32,
    pub local_u: f32,
    pub local_v: f32,
    pub joint: bool,
}

pub fn tile_cell(u: f32, v: f32, tiles: i32, joint_uv: f32) -> TileCell {
    let tx = ((u * tiles as f32).floor() as i32).rem_euclid(tiles);
    let ty = ((v * tiles as f32).floor() as i32).rem_euclid(tiles);
    let lu = (u * tiles as f32) - (u * tiles as f32).floor();
    let lv = (v * tiles as f32) - (v * tiles as f32).floor();
    TileCell {
        tile_x: tx,
        tile_y: ty,
        local_u: lu,
        local_v: lv,
        joint: lu < joint_uv || lu > 1.0 - joint_uv || lv < joint_uv || lv > 1.0 - joint_uv,
    }
}

/// One evaluated weathering sample.  Every field is in [0, 1].
#[derive(Clone, Copy, Debug, Default)]
pub struct WearSample {
    /// Per-unit random tone variation (brick to brick, tile to tile).
    pub tone: f32,
    /// Darkening from dirt; strongest low on the surface.
    pub grime: f32,
    /// Chipped/eroded areas: the baker removes material or brightens here.
    pub chip: f32,
    /// Crack lines (dark, thin).
    pub crack: f32,
    /// Water streak darkening.
    pub stain: f32,
    /// Moss/biological growth.
    pub moss: f32,
}

/// Evaluate all masks at one surface point.  `height01` is 0 at the ground
/// line and 1 at the top of the surface — grime and moss pool at the bottom,
/// streaks run downward from the top.
pub fn sample(
    kind: SurfaceKind,
    profile: &WeatheringProfile,
    u: f32,
    v: f32,
    height01: f32,
    cell_salt: i32,
) -> WearSample {
    let seed = profile.seed;
    let scale = match kind {
        SurfaceKind::Brick | SurfaceKind::Tile => 26.0,
        SurfaceKind::Asphalt => 14.0,
        SurfaceKind::Concrete => 9.0,
        SurfaceKind::Stone => 7.0,
    };
    let tone = hash01(seed, cell_salt, cell_salt * 7 + 3, 17);
    let grime_field = fbm(seed, u * scale, v * scale, 3, 23);
    // Grime pools near the ground and in sheltered pockets.
    let grime = (profile.grime * (1.0 - height01).powf(1.6) * (0.45 + 0.9 * grime_field)).min(1.0);
    // Edge wear chips the borders of each unit; the caller passes local
    // coordinates through the cell, so approximate with high-frequency fBm
    // ridges here.
    let wear_field = fbm(seed, u * scale * 2.4, v * scale * 2.4, 4, 41);
    let chip = (profile.edge_wear * (wear_field - 0.55).max(0.0) * 2.6).min(1.0);
    // Cracks ride the zero set of an fBm ridge.
    let ridge = (value_noise(seed, u * scale * 1.7, v * scale * 1.7, 57) - 0.5).abs();
    let crack = if ridge < 0.012 + profile.cracks * 0.02 {
        1.0 - ridge / (0.012 + profile.cracks * 0.02)
    } else {
        0.0
    };
    // Streaks: vertical bands modulated by noise, fading downward.
    let streak = value_noise(seed, u * 90.0, 3.0, 71);
    let stain = (profile.stains * streak * (1.0 - height01).powf(0.5)).min(1.0);
    // Moss prefers damp joints and shaded lower courses.
    let moss_field = fbm(seed, u * scale * 1.3, v * scale * 1.3, 2, 83);
    let moss = (profile.moss * moss_field * (1.0 - height01) * 1.8).min(1.0);
    WearSample {
        tone,
        grime,
        chip,
        crack,
        stain,
        moss,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brick_layout_alternates_courses() {
        let a = brick_cell(0.1, 0.05, 8, 0.08);
        let b = brick_cell(0.1, 0.30, 8, 0.08);
        assert_ne!(a.course, b.course);
        assert!(!a.joint);
        // The very edge of a brick is joint.
        let edge = brick_cell(0.0, 0.0, 8, 0.08);
        assert!(edge.joint);
    }

    #[test]
    fn grime_pools_low_and_streaks_fall() {
        let profile = WeatheringProfile {
            grime: 1.0,
            stains: 1.0,
            ..Default::default()
        };
        let low = sample(SurfaceKind::Brick, &profile, 0.4, 0.4, 0.05, 3);
        let high = sample(SurfaceKind::Brick, &profile, 0.4, 0.4, 0.95, 3);
        assert!(low.grime > high.grime);
        assert!(low.stain >= high.stain);
    }
}
