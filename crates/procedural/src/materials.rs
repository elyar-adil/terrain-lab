//! Material bakers: CPU-baked RGBA textures for facades and ground surfaces,
//! assembled from the shared layouts and the shared weathering system.  Every
//! consumer — building walls, compound pavement, city streets — receives the
//! same baked set, so there is exactly one implementation of "brick with
//! grime" or "asphalt with wheel wear" in the project.

use crate::noise::hash01;
use crate::weathering::{SurfaceKind, WeatheringProfile, brick_cell, sample, tile_cell};

pub const TEXTURE_SIZE: u32 = 256;

/// Identifiers the frontend resolves textures by; the strings are stable API.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialName {
    /// Painted/concrete residential wall with punched windows (最常见).
    FacadeResidential,
    /// Curtain-wall glass grid for office/commercial towers.
    FacadeGlass,
    /// Civic stone ashlar.
    FacadeStone,
    /// Worn city asphalt.
    GroundAsphalt,
    /// Concrete sidewalk pavers.
    GroundSidewalk,
    /// Compound courtyard pavers (小区铺装).
    GroundPaver,
}

impl MaterialName {
    pub const fn slug(self) -> &'static str {
        match self {
            Self::FacadeResidential => "facade/residential",
            Self::FacadeGlass => "facade/glass",
            Self::FacadeStone => "facade/stone",
            Self::GroundAsphalt => "ground/asphalt",
            Self::GroundSidewalk => "ground/sidewalk",
            Self::GroundPaver => "ground/paver",
        }
    }
}

/// A baked texture payload ready for IPC (the caller base64-encodes `rgba`).
#[derive(Clone, Debug)]
pub struct BakedTexture {
    pub name: &'static str,
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

#[derive(Clone, Copy)]
struct Rgb([f32; 3]);

impl Rgb {
    fn new(hex: u32) -> Self {
        Rgb([
            ((hex >> 16) & 0xff) as f32 / 255.0,
            ((hex >> 8) & 0xff) as f32 / 255.0,
            (hex & 0xff) as f32 / 255.0,
        ])
    }

    fn mix(self, other: Rgb, t: f32) -> Rgb {
        let t = t.clamp(0.0, 1.0);
        Rgb([
            self.0[0] + (other.0[0] - self.0[0]) * t,
            self.0[1] + (other.0[1] - self.0[1]) * t,
            self.0[2] + (other.0[2] - self.0[2]) * t,
        ])
    }

    fn scale(self, t: f32) -> Rgb {
        Rgb([
            self.0[0] * t,
            self.0[1] * t,
            self.0[2] * t,
        ])
    }

    fn bytes(self) -> [u8; 3] {
        [
            (self.0[0].clamp(0.0, 1.0) * 255.0) as u8,
            (self.0[1].clamp(0.0, 1.0) * 255.0) as u8,
            (self.0[2].clamp(0.0, 1.0) * 255.0) as u8,
        ]
    }
}

/// Bake one material texture.  Deterministic for a given (name, size, seed).
pub fn bake(name: MaterialName, size: u32, seed: u32) -> BakedTexture {
    let mut rgba = vec![0_u8; (size * size * 4) as usize];
    let profile = WeatheringProfile {
        seed,
        ..weathering_for(name)
    };
    for y in 0..size {
        // Texture V runs top-to-bottom in image space; the ground line for
        // facades is the bottom edge, so height01 = 1 - (y / size).
        let v = y as f32 / size as f32;
        for x in 0..size {
            let u = x as f32 / size as f32;
            let colour = match name {
                MaterialName::FacadeResidential => facade_residential(u, v, &profile),
                MaterialName::FacadeGlass => facade_glass(u, v, &profile),
                MaterialName::FacadeStone => facade_stone(u, v, &profile),
                MaterialName::GroundAsphalt => ground_asphalt(u, v, &profile),
                MaterialName::GroundSidewalk => ground_sidewalk(u, v, &profile),
                MaterialName::GroundPaver => ground_paver(u, v, &profile),
            };
            let [r, g, b] = colour.bytes();
            let index = ((y * size + x) * 4) as usize;
            rgba[index] = r;
            rgba[index + 1] = g;
            rgba[index + 2] = b;
            rgba[index + 3] = 255;
        }
    }
    BakedTexture {
        name: name.slug(),
        width: size,
        height: size,
        rgba,
    }
}

/// The full set the renderer expects, in stable order.
pub fn standard_texture_set(size: u32) -> Vec<BakedTexture> {
    [
        MaterialName::FacadeResidential,
        MaterialName::FacadeGlass,
        MaterialName::FacadeStone,
        MaterialName::GroundAsphalt,
        MaterialName::GroundSidewalk,
        MaterialName::GroundPaver,
    ]
    .iter()
    .map(|name| bake(*name, size, seed_for(*name)))
    .collect()
}

fn seed_for(name: MaterialName) -> u32 {
    match name {
        MaterialName::FacadeResidential => 0x11facade,
        MaterialName::FacadeGlass => 0x2206a55,
        MaterialName::FacadeStone => 0x33057e,
        MaterialName::GroundAsphalt => 0x44a5ca,
        MaterialName::GroundSidewalk => 0x5507a1c,
        MaterialName::GroundPaver => 0x6607a7e,
    }
}

fn weathering_for(name: MaterialName) -> WeatheringProfile {
    match name {
        MaterialName::FacadeResidential => WeatheringProfile {
            edge_wear: 0.35,
            grime: 0.5,
            cracks: 0.2,
            stains: 0.45,
            moss: 0.12,
            ..Default::default()
        },
        MaterialName::FacadeGlass => WeatheringProfile {
            edge_wear: 0.1,
            grime: 0.3,
            cracks: 0.0,
            stains: 0.5,
            moss: 0.0,
            ..Default::default()
        },
        MaterialName::FacadeStone => WeatheringProfile {
            edge_wear: 0.5,
            grime: 0.45,
            cracks: 0.25,
            stains: 0.4,
            moss: 0.25,
            ..Default::default()
        },
        MaterialName::GroundAsphalt => WeatheringProfile {
            edge_wear: 0.25,
            grime: 0.2,
            cracks: 0.5,
            stains: 0.25,
            moss: 0.05,
            ..Default::default()
        },
        MaterialName::GroundSidewalk => WeatheringProfile {
            edge_wear: 0.55,
            grime: 0.45,
            cracks: 0.3,
            stains: 0.2,
            moss: 0.4,
            ..Default::default()
        },
        MaterialName::GroundPaver => WeatheringProfile {
            edge_wear: 0.45,
            grime: 0.5,
            cracks: 0.2,
            stains: 0.25,
            moss: 0.5,
            ..Default::default()
        },
    }
}

/// Punched-window residential wall: 6 bays × 10 storeys, each window with a
/// sill, frame and variable glass tint, wrapped in weathered render/brick.
fn facade_residential(u: f32, v: f32, profile: &WeatheringProfile) -> Rgb {
    const BAYS: i32 = 6;
    const FLOORS: i32 = 10;
    let cell = crate::weathering::tile_cell(u, v, BAYS, 0.0);
    let floor = ((v * FLOORS as f32).floor() as i32).clamp(0, FLOORS - 1);
    let local_v = v * FLOORS as f32 - (v * FLOORS as f32).floor();
    let wall = Rgb::new(0xc8c3b4);
    let mortar = Rgb::new(0xd9d5c8);
    let glass = Rgb::new(0x5c7683);
    let frame = Rgb::new(0xe8e6dd);
    // Window opening occupies the middle of each bay/storey cell.
    let wx = cell.local_u;
    let wy = local_v;
    let in_window_x = wx > 0.28 && wx < 0.72;
    let in_window_y = wy > 0.22 && wy < 0.78;
    let mut colour = if in_window_x && in_window_y {
        let reflect = hash01(profile.seed, cell.tile_x, floor, 31);
        let sill_band = wy < 0.30 || wy > 0.70;
        if sill_band {
            frame
        } else {
            glass.scale(0.8 + reflect * 0.5)
        }
    } else if (wx > 0.24 && wx < 0.28) || (wx > 0.72 && wx < 0.76) {
        frame
    } else {
        // Rendered wall with faint brick courses showing through the paint.
        let brick = brick_cell(u, v, 18, 0.05);
        if brick.joint {
            mortar
        } else {
            wall.scale(0.94 + 0.06 * (brick.course % 2) as f32)
        }
    };
    // Weathering washes the wall; windows stay glassy.
    if !(in_window_x && in_window_y) {
        let wear = sample(
            SurfaceKind::Brick,
            profile,
            u,
            v,
            1.0 - v,
            cell.tile_x * 31 + floor,
        );
        colour = colour
            .mix(Rgb::new(0x4a4438), wear.grime * 0.55)
            .mix(Rgb::new(0x6e6a5e), wear.stain * 0.4)
            .mix(Rgb::new(0x55602e), wear.moss * 0.5)
            .scale(1.0 - wear.chip * 0.18);
    }
    colour
}

fn facade_glass(u: f32, v: f32, profile: &WeatheringProfile) -> Rgb {
    // Curtain wall: vertical mullions every 1/8, spandrel band each storey.
    let mullion = (u * 8.0).fract() < 0.045;
    let spandrel = (v * 6.0).fract() < 0.14;
    let pane_noise = hash01(profile.seed, (u * 8.0) as i32, (v * 6.0) as i32, 13);
    let glass = Rgb::new(0x6d8894).scale(0.85 + pane_noise * 0.35);
    let mut colour = if mullion {
        Rgb::new(0x8f9494)
    } else if spandrel {
        Rgb::new(0x7d8684)
    } else {
        glass
    };
    let wear = sample(SurfaceKind::Concrete, profile, u, v, 1.0 - v, 0);
    colour = colour.mix(Rgb::new(0x3a3f3e), wear.stain * 0.3);
    colour
}

fn facade_stone(u: f32, v: f32, profile: &WeatheringProfile) -> Rgb {
    let courses = 8;
    let brick = brick_cell(u, v, courses, 0.045);
    let stone = Rgb::new(0xb5ab93).scale(0.9 + brick.brick.rem_euclid(7) as f32 * 0.012);
    let joint = Rgb::new(0x8b8574);
    let mut colour = if brick.joint { joint } else { stone };
    let wear = sample(
        SurfaceKind::Stone,
        profile,
        u,
        v,
        1.0 - v,
        brick.course * 17 + brick.brick,
    );
    colour = colour
        .mix(Rgb::new(0x4d4636), wear.grime * 0.5)
        .mix(Rgb::new(0x3f4a22), wear.moss * 0.55)
        .scale(1.0 - wear.chip * 0.22);
    colour
}

fn ground_asphalt(u: f32, v: f32, profile: &WeatheringProfile) -> Rgb {
    let base = Rgb::new(0x3a3d3d);
    // Aggregate speckle.
    let speck = hash01(profile.seed, (u * 512.0) as i32, (v * 512.0) as i32, 5);
    let mut colour = base.scale(0.85 + speck * 0.35);
    // Wheel tracks: two smoother, slightly darker bands along V.
    let track: f32 = if (u - 0.32).abs() < 0.055 || (u - 0.68).abs() < 0.055 { 1.0 } else { 0.0 };
    colour = colour.scale(1.0 - track * 0.10);
    // Patch repairs: large low-frequency blotches.
    let patch = crate::noise::fbm(profile.seed, u * 3.2, v * 3.2, 3, 19);
    if patch > 0.68 {
        colour = colour.mix(Rgb::new(0x2f3233), (patch - 0.68) * 2.4);
    }
    let wear = sample(SurfaceKind::Asphalt, profile, u, v, 0.5, 0);
    colour = colour.scale(1.0 - wear.crack * 0.5);
    colour = colour.mix(Rgb::new(0x5a5c50), wear.stain * 0.2);
    colour
}

fn ground_sidewalk(u: f32, v: f32, profile: &WeatheringProfile) -> Rgb {
    let tile = tile_cell(u, v, 8, 0.035);
    let concrete = Rgb::new(0xa9a89c).scale(0.9 + tile.tile_x.rem_euclid(5) as f32 * 0.02);
    let joint = Rgb::new(0x7c7b70);
    let mut colour = if tile.joint { joint } else { concrete };
    let wear = sample(
        SurfaceKind::Tile,
        profile,
        u,
        v,
        0.5,
        tile.tile_x * 13 + tile.tile_y,
    );
    colour = colour
        .mix(Rgb::new(0x4e4d42), wear.grime * 0.45)
        .mix(Rgb::new(0x4c5a2b), wear.moss * 0.6)
        .scale(1.0 - wear.chip * 0.25);
    colour
}

fn ground_paver(u: f32, v: f32, profile: &WeatheringProfile) -> Rgb {
    let brick = brick_cell(u, v, 12, 0.05);
    let paver = Rgb::new(0x9b8f7c).scale(0.88 + hash01(profile.seed, brick.brick, brick.course, 9) * 0.2);
    let joint = Rgb::new(0x6f6a5d);
    let mut colour = if brick.joint { joint } else { paver };
    let wear = sample(
        SurfaceKind::Brick,
        profile,
        u,
        v,
        0.5,
        brick.course * 23 + brick.brick,
    );
    colour = colour
        .mix(Rgb::new(0x45412f), wear.grime * 0.5)
        .mix(Rgb::new(0x3f4c22), wear.moss * 0.65)
        .scale(1.0 - wear.chip * 0.28);
    colour
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baked_textures_are_opaque_and_stable() {
        let a = bake(MaterialName::GroundAsphalt, 64, 11);
        let b = bake(MaterialName::GroundAsphalt, 64, 11);
        assert_eq!(a.rgba, b.rgba);
        assert_eq!(a.rgba.len(), (64 * 64 * 4) as usize);
        assert!(a.rgba.chunks_exact(4).all(|pixel| pixel[3] == 255));
    }

    #[test]
    fn standard_set_covers_all_materials() {
        let set = standard_texture_set(64);
        assert_eq!(set.len(), 6);
        assert!(set.iter().all(|texture| texture.rgba.len() == (64 * 64 * 4) as usize));
    }

    #[test]
    fn facades_have_windows_punched() {
        let texture = bake(MaterialName::FacadeResidential, 96, 3);
        // Sample a window centre: bay 0.5, storey centre should be darker than
        // the wall between windows.
        let size = texture.width;
        let at = |u: f32, v: f32| {
            let x = (u * size as f32) as usize;
            let y = (v * size as f32) as usize;
            let index = (y * size as usize + x) * 4;
            texture.rgba[index] as u32 + texture.rgba[index + 1] as u32 + texture.rgba[index + 2] as u32
        };
        let window = at(1.0 / 12.0, 0.05);
        let wall = at(0.02, 0.05);
        assert!(window < wall, "window ({window}) should be darker than wall ({wall})");
    }
}
