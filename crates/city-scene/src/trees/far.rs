//! Far-LOD tree prototypes for the terrain layer.
//!
//! The terrain renders forests as instanced trees long before the city's
//! per-tree geometry ([`super::grow`]) is affordable. Those far trees used to
//! be a renderer-side cone on a cylinder, the same stamp for every forest. They
//! are generated here instead, from the same species records and canopy
//! profiles as the near trees, so the distance a forest is seen from is the only
//! thing that changes between LODs, not the tree.
//!
//! A far tree is a bare trunk plus a crown of a few lumpy ellipsoid lobes laid
//! out along the species' own crown profile (a `水杉` is a stack of narrow lobes,
//! a `香樟` a wide round heap). Vertex colours carry the species' foliage and
//! bark reflectance with per-lobe value variation; normals are the ellipsoids'
//! so the lobes shade as rounded clumps. Prototypes are authored at unit height
//! (the instance scale supplies the metres), and come in two detail levels.

use super::forms;
use super::TAU;
use crate::math::{Rng, Vec3};
use crate::species::{SPECIES, Species};

/// Species the terrain draws forests with: three broadleaf silhouettes, a
/// spire, a tiered conifer and a column.
pub const FAR_SPECIES: [&str; 6] = [
    "xiang-zhang",
    "yu-shu",
    "gui-hua",
    "shui-shan",
    "xue-song",
    "yang-shu",
];

/// One far-LOD prototype: indexed triangles with per-vertex colour.
#[derive(Debug, Clone)]
pub struct FarTree {
    pub species: &'static str,
    /// 0 = mid (about a hundred triangles), 1 = far (about thirty).
    pub lod: u8,
    /// Real metres at the species' table mid-range; instance scale is relative.
    pub height_m: f32,
    pub crown_m: f32,
    pub trunk_m: f32,
    /// `x y z` per vertex, unit height.
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    /// Linear RGB per vertex.
    pub colors: Vec<f32>,
    pub indices: Vec<u32>,
}

struct Builder {
    positions: Vec<f32>,
    normals: Vec<f32>,
    colors: Vec<f32>,
    indices: Vec<u32>,
}

impl Builder {
    fn vertex(&mut self, p: Vec3, n: Vec3, c: [f32; 3]) -> u32 {
        let index = (self.positions.len() / 3) as u32;
        self.positions.extend_from_slice(&[p.x, p.y, p.z]);
        self.normals.extend_from_slice(&[n.x, n.y, n.z]);
        self.colors.extend_from_slice(&c);
        index
    }

    fn ring(&mut self, y: f32, r: f32, sides: usize, colour: [f32; 3]) -> Vec<u32> {
        (0..sides)
            .map(|s| {
                let a = s as f32 / sides as f32 * TAU;
                let n = Vec3::new(a.cos(), 0.0, a.sin());
                let shade = 0.88 + 0.12 * ((s * 5) as f32).sin();
                self.vertex(
                    Vec3::new(n.x * r, y, n.z * r),
                    n,
                    [colour[0] * shade, colour[1] * shade, colour[2] * shade],
                )
            })
            .collect()
    }

    /// A tapered tube along +y, open at both ends (the crown hides the top and
    /// the ground the foot).
    fn trunk(&mut self, height: f32, r0: f32, r1: f32, sides: usize, colour: [f32; 3]) {
        let foot = self.ring(0.0, r0, sides, colour);
        let top = self.ring(height, r1, sides, colour);
        for s in 0..sides {
            let t = (s + 1) % sides;
            self.indices
                .extend_from_slice(&[foot[s], top[s], top[t], foot[s], top[t], foot[t]]);
        }
    }

    /// A noise-displaced ellipsoid lobe.
    #[allow(clippy::too_many_arguments)]
    fn lobe(
        &mut self,
        centre: Vec3,
        radii: Vec3,
        lat: usize,
        lon: usize,
        colour: [f32; 3],
        seed: f32,
        top_light: f32,
    ) {
        let first = (self.positions.len() / 3) as u32;
        for i in 0..=lat {
            let v = i as f32 / lat as f32;
            let theta = v * std::f32::consts::PI;
            for j in 0..lon {
                let u = j as f32 / lon as f32 * TAU;
                let dir = Vec3::new(theta.sin() * u.cos(), theta.cos(), theta.sin() * u.sin());
                let bump = 1.0
                    + 0.16 * (u * 3.0 + seed + theta * 2.0).sin()
                    + 0.10 * (u * 5.0 - seed * 1.7 + theta * 4.0).sin();
                let p = centre
                    + Vec3::new(
                        dir.x * radii.x * bump,
                        dir.y * radii.y * bump,
                        dir.z * radii.z * bump,
                    );
                // Ellipsoid normal: gradient of the implicit surface.
                let n = Vec3::new(
                    dir.x / radii.x.max(1.0e-4),
                    dir.y / radii.y.max(1.0e-4),
                    dir.z / radii.z.max(1.0e-4),
                )
                .normalized_or_up();
                let lift = 1.0 + top_light * (dir.y * 0.5 + 0.5) - top_light * 0.5;
                self.vertex(p, n, [colour[0] * lift, colour[1] * lift, colour[2] * lift]);
            }
        }
        for i in 0..lat {
            for j in 0..lon {
                let a = first + (i * lon + j) as u32;
                let b = first + (i * lon + (j + 1) % lon) as u32;
                let c = first + ((i + 1) * lon + (j + 1) % lon) as u32;
                let d = first + ((i + 1) * lon + j) as u32;
                // Poles collapse to points; skip the degenerate half.
                if i > 0 {
                    self.indices.extend_from_slice(&[a, d, b]);
                }
                if i + 1 < lat {
                    self.indices.extend_from_slice(&[b, d, c]);
                }
            }
        }
    }
}

fn build(species: &'static Species, lod: u8) -> FarTree {
    let arch = forms::architecture(species.canopy);
    let height = (species.height_m.0 + species.height_m.1) * 0.5;
    let crown = (species.crown_m.0 + species.crown_m.1) * 0.5;
    let trunk = (species.trunk_m.0 + species.trunk_m.1) * 0.5;
    let radius = (crown / height) * 0.94;
    let base = (species.clear_stem * 0.80).clamp(0.10, 0.86);
    let mut seed = 0xfa27_0000_u32 ^ ((lod as u32) << 8);
    for byte in species.key.bytes() {
        seed = seed.wrapping_mul(0x0100_0193) ^ byte as u32;
    }
    let mut rng = Rng::new(seed);
    let mut b = Builder {
        positions: Vec::new(),
        normals: Vec::new(),
        colors: Vec::new(),
        indices: Vec::new(),
    };
    let trunk_r = (trunk / height).max(0.004);
    let bark = species.bark.colour;
    b.trunk(
        base + (1.0 - base) * 0.55,
        trunk_r,
        trunk_r * 0.55,
        if lod == 0 { 6 } else { 4 },
        bark,
    );

    let (levels, ring, lat, lon) = if lod == 0 {
        (2usize, 1usize, 3usize, 5usize)
    } else {
        (2, 0, 2, 6)
    };
    let span = 1.0 - base;
    let f = species.foliage;
    let level_h = span / levels as f32;
    for level in 0..levels {
        let t = (level as f32 + 0.5) / levels as f32;
        let y = base + span * t * 0.94;
        let r = radius * (arch.profile)(t).clamp(0.0, 1.0);
        let ry = level_h * 0.82;
        let shade = rng.range(0.86, 1.06);
        let phase = rng.range(0.0, TAU);
        // Core lobe: fills the crown so it reads as a mass, not a ring.
        b.lobe(
            Vec3::new(0.0, y, 0.0),
            Vec3::new(r * 0.62, ry, r * 0.62),
            lat,
            lon,
            [f[0] * shade, f[1] * shade, f[2] * shade],
            phase,
            0.30,
        );
        for k in 0..ring {
            let a = phase + k as f32 / ring as f32 * TAU + rng.range(-0.3, 0.3);
            let off = r * 0.55;
            let shade = rng.range(0.82, 1.10);
            b.lobe(
                Vec3::new(a.cos() * off, y + rng.range(-0.15, 0.15) * ry, a.sin() * off),
                Vec3::new(r * 0.46, ry * 0.9, r * 0.46),
                lat,
                lon,
                [f[0] * shade, f[1] * shade, f[2] * shade],
                phase + k as f32 * 2.1,
                0.30,
            );
        }
    }
    // A top lobe so the tip is rounded off (or pointed, for a spire: its own
    // profile is near zero there and the lobe is narrow).
    if lod == 0 {
        let r_top = radius * (arch.profile)(0.92).clamp(0.05, 1.0);
        b.lobe(
            Vec3::new(0.0, 1.0 - level_h * 0.62, 0.0),
            Vec3::new(r_top * 0.55, level_h * 0.50, r_top * 0.55),
            lat,
            lon,
            [f[0] * 1.05, f[1] * 1.05, f[2] * 1.05],
            rng.range(0.0, TAU),
            0.30,
        );
    }
    // Lumpy lobes overshoot the profile a little; scale the tree so it is
    // exactly unit height, which is the invariant instancing relies on.
    let top = b
        .positions
        .chunks(3)
        .map(|p| p[1])
        .fold(f32::MIN, f32::max)
        .max(1.0e-3);
    for p in b.positions.chunks_mut(3) {
        p[1] = (p[1] / top).max(0.0);
    }
    FarTree {
        species: species.key,
        lod,
        height_m: height,
        crown_m: crown,
        trunk_m: trunk,
        positions: b.positions,
        normals: b.normals,
        colors: b.colors,
        indices: b.indices,
    }
}

/// Every far prototype: [`FAR_SPECIES`] at two levels of detail each.
pub fn far_tree_set() -> Vec<FarTree> {
    let mut set = Vec::new();
    for key in FAR_SPECIES {
        if let Some(species) = SPECIES.iter().find(|species| species.key == key) {
            for lod in 0..2u8 {
                set.push(build(species, lod));
            }
        }
    }
    set
}

/// A far prototype as it ships in the terrain payload: typed arrays as base64
/// (`f32` little-endian for the attributes, `u16` little-endian indices).
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FarTreePayload {
    pub species: String,
    pub lod: u8,
    pub height_metres: f32,
    pub crown_radius_metres: f32,
    pub trunk_radius_metres: f32,
    pub positions: String,
    pub normals: String,
    pub colors: String,
    pub indices: String,
}

/// The prototype set as payload records, for `GenerationResult.farTrees`.
pub fn far_tree_payload() -> Vec<FarTreePayload> {
    use base64::{Engine, engine::general_purpose::STANDARD};
    let floats = |values: &[f32]| {
        STANDARD.encode(values.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>())
    };
    far_tree_set()
        .into_iter()
        .map(|tree| FarTreePayload {
            species: tree.species.to_string(),
            lod: tree.lod,
            height_metres: tree.height_m,
            crown_radius_metres: tree.crown_m * 0.5,
            trunk_radius_metres: tree.trunk_m * 0.5,
            positions: floats(&tree.positions),
            normals: floats(&tree.normals),
            colors: floats(&tree.colors),
            indices: STANDARD.encode(
                tree.indices
                    .iter()
                    .flat_map(|&i| (i as u16).to_le_bytes())
                    .collect::<Vec<u8>>(),
            ),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn far_trees_are_unit_height_cheap_and_species_shaped() {
        let set = far_tree_set();
        assert_eq!(set.len(), FAR_SPECIES.len() * 2);
        for tree in &set {
            let triangles = tree.indices.len() / 3;
            let top = tree.positions.chunks(3).map(|p| p[1]).fold(f32::MIN, f32::max);
            let bottom = tree.positions.chunks(3).map(|p| p[1]).fold(f32::MAX, f32::min);
            assert!((0.9..=1.12).contains(&top), "{} tops out at {top}", tree.species);
            assert!(bottom >= -0.02, "{} starts below ground: {bottom}", tree.species);
            assert!(tree.positions.iter().all(|v| v.is_finite()));
            assert!(
                tree.indices
                    .iter()
                    .all(|&i| (i as usize) < tree.positions.len() / 3)
            );
            let limit = if tree.lod == 0 { 170 } else { 60 };
            assert!(
                triangles > 20 && triangles <= limit,
                "{} lod {} has {triangles} triangles",
                tree.species,
                tree.lod
            );
        }
        let width = |key: &str| {
            set.iter()
                .find(|t| t.species == key && t.lod == 0)
                .map(|t| {
                    t.positions
                        .chunks(3)
                        .map(|p| p[0].hypot(p[2]))
                        .fold(0.0_f32, f32::max)
                })
                .unwrap()
        };
        assert!(
            width("xiang-zhang") > width("yang-shu") * 1.5,
            "a camphor is far broader than a poplar"
        );
    }
}
