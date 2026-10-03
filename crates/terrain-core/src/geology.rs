//! Deterministic, explicit geology fields for erosion and surface rendering.
//!
//! This module is intentionally independent from `TerrainData`.  Integration
//! can therefore replace the legacy scalar geology proxy without coupling the
//! field generator to a particular erosion or rendering implementation.

use serde::{Deserialize, Serialize};
use worldgen_core::hash::mix32;
use worldgen_core::{lerp, smooth01};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Lithology {
    Shale,
    Sandstone,
    Limestone,
    Granite,
    Basalt,
    Metamorphic,
}

impl Lithology {
    /// Relative resistance to mechanical erosion. One is a moderately
    /// resistant reference rock; it is not an artificial terrain-height bound.
    pub const fn erosion_resistance(self) -> f32 {
        match self {
            Self::Shale => 0.58,
            Self::Sandstone => 1.02,
            Self::Limestone => 0.88,
            Self::Granite => 1.82,
            Self::Basalt => 2.18,
            Self::Metamorphic => 1.54,
        }
    }

    const fn chemical_weatherability(self) -> f32 {
        match self {
            Self::Shale => 0.78,
            Self::Sandstone => 0.43,
            Self::Limestone => 0.92,
            Self::Granite => 0.31,
            Self::Basalt => 0.52,
            Self::Metamorphic => 0.38,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeologyConfig {
    pub size: usize,
    pub world_size_km: f32,
    pub seed: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeologyError {
    InvalidSize,
    InvalidWorldSize,
}

impl std::fmt::Display for GeologyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSize => {
                formatter.write_str("geology grid must contain at least two cells per axis")
            }
            Self::InvalidWorldSize => {
                formatter.write_str("geology world size must be finite and positive")
            }
        }
    }
}

impl std::error::Error for GeologyError {}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaultTrace {
    /// Unit normal in world XY coordinates.
    pub normal_x: f32,
    pub normal_y: f32,
    /// Signed offset from the world centre, in kilometres.
    pub offset_km: f32,
    pub damage_half_width_km: f32,
    /// Vertical separation of strata across the fault plane.
    pub throw_metres: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeologyField {
    pub size: usize,
    pub world_size_km: f32,
    pub lithology: Vec<Lithology>,
    pub erosion_resistance: Vec<f32>,
    /// Bedding strike clockwise from world north, in radians.
    pub bedding_strike: Vec<f32>,
    /// Bedding dip from horizontal, in radians.
    pub bedding_dip: Vec<f32>,
    /// Continuous phase of folded bedding, in radians.
    pub fold_phase: Vec<f32>,
    /// Signed stratigraphic coordinate at zero elevation, in metres.
    pub stratigraphic_base_metres: Vec<f32>,
    /// Continuous layer coordinate at the currently exposed surface. Its
    /// integer part selects a bed and its fractional part locates contacts.
    pub stratigraphic_phase: Vec<f32>,
    pub layer_thickness_metres: f32,
    /// Distance to the nearest fault trace, in metres.
    pub fault_distance_metres: Vec<f32>,
    /// Zero outside fault damage zones and one on a fault core.
    pub fracture_intensity: Vec<f32>,
    /// Relative chemical/physical weathering potential in `[0, 1]`.
    pub weathering_potential: Vec<f32>,
    pub faults: Vec<FaultTrace>,
}

impl GeologyField {
    /// Intersect the 3D stratigraphic volume with a changing topographic
    /// surface. Erosion therefore reveals the next physical bed instead of
    /// leaving a painted 2D geology map stuck to the terrain.
    pub fn expose_surface(&mut self, elevation_metres: &[f32]) {
        assert_eq!(elevation_metres.len(), self.lithology.len());
        const SEQUENCE: [Lithology; 7] = [
            Lithology::Shale,
            Lithology::Sandstone,
            Lithology::Limestone,
            Lithology::Sandstone,
            Lithology::Metamorphic,
            Lithology::Granite,
            Lithology::Basalt,
        ];
        for (index, elevation) in elevation_metres.iter().copied().enumerate() {
            let coordinate =
                self.stratigraphic_base_metres[index] + elevation * self.bedding_dip[index].cos();
            let phase = coordinate / self.layer_thickness_metres;
            let layer_index = phase.floor() as i32;
            let rock = SEQUENCE[layer_index.rem_euclid(SEQUENCE.len() as i32) as usize];
            self.stratigraphic_phase[index] = phase;
            self.lithology[index] = rock;
            let damage = self.fracture_intensity[index];
            self.erosion_resistance[index] =
                (rock.erosion_resistance() * (1.0 - damage * 0.42)).clamp(0.34, 2.35);
            self.weathering_potential[index] = (rock.chemical_weatherability() * 0.72
                + damage * 0.30
                + (1.0 - self.erosion_resistance[index] / 2.35) * 0.18)
                .clamp(0.0, 1.0);
        }
    }
}

pub fn generate_geology(config: GeologyConfig) -> Result<GeologyField, GeologyError> {
    if config.size < 2 {
        return Err(GeologyError::InvalidSize);
    }
    if !config.world_size_km.is_finite() || config.world_size_km <= 0.0 {
        return Err(GeologyError::InvalidWorldSize);
    }

    let len = config.size * config.size;
    let mut lithology = Vec::with_capacity(len);
    let mut erosion_resistance = Vec::with_capacity(len);
    let mut bedding_strike = Vec::with_capacity(len);
    let mut bedding_dip = Vec::with_capacity(len);
    let mut fold_phase = Vec::with_capacity(len);
    let mut stratigraphic_base_metres = Vec::with_capacity(len);
    let stratigraphic_phase = vec![0.0; len];
    let mut fault_distance_metres = Vec::with_capacity(len);
    let mut fracture_intensity = Vec::with_capacity(len);
    let mut weathering_potential = Vec::with_capacity(len);

    let regional_strike = hash01(config.seed, 11, 29) * std::f32::consts::PI;
    let fold_amplitude_km = 0.35 + hash01(config.seed, 17, 43) * 1.15;
    // This grid represents formations, not centimetre-scale beds. Keep each
    // unit resolvable by the regional terrain mesh; thinner bedding belongs in
    // the future near-field geology clipmap.
    let layer_thickness_metres = 240.0 + hash01(config.seed, 23, 47) * 360.0;
    let fold_wavelength_km = config.world_size_km * (0.23 + hash01(config.seed, 31, 7) * 0.22);
    let fault_count = 2 + (mix32(config.seed ^ 0xa512_3f49) % 3) as usize;
    let faults = (0..fault_count)
        .map(|index| {
            let angle = hash01(config.seed, index as u32 + 71, 19) * std::f32::consts::PI;
            FaultTrace {
                normal_x: angle.cos(),
                normal_y: angle.sin(),
                offset_km: (hash01(config.seed, index as u32 + 101, 37) - 0.5)
                    * config.world_size_km
                    * 0.68,
                damage_half_width_km: config.world_size_km
                    * (0.006 + hash01(config.seed, index as u32 + 151, 53) * 0.012),
                throw_metres: (hash01(config.seed, index as u32 + 181, 67) - 0.5) * 720.0,
            }
        })
        .collect::<Vec<_>>();

    for y in 0..config.size {
        for x in 0..config.size {
            let world_x = (x as f32 / (config.size - 1) as f32 - 0.5) * config.world_size_km;
            let world_y = (y as f32 / (config.size - 1) as f32 - 0.5) * config.world_size_km;
            let broad = value_noise(
                world_x / (config.world_size_km * 0.22),
                world_y / (config.world_size_km * 0.22),
                config.seed ^ 0x8da6_b343,
            );
            let local_strike =
                (regional_strike + (broad - 0.5) * 0.72).rem_euclid(std::f32::consts::PI);
            let across_x = local_strike.cos();
            let across_y = -local_strike.sin();
            let along_x = local_strike.sin();
            let along_y = local_strike.cos();
            let across = world_x * across_x + world_y * across_y;
            let along = world_x * along_x + world_y * along_y;
            let phase = across / fold_wavelength_km * std::f32::consts::TAU
                + value_noise(along / 18.0, across / 24.0, config.seed ^ 0x1f12_a91d) * 1.15;
            let fold = phase.sin();
            let warped_layer =
                across + fold * fold_amplitude_km + (broad - 0.5) * fold_amplitude_km * 0.65;
            let sequence = [
                Lithology::Shale,
                Lithology::Sandstone,
                Lithology::Limestone,
                Lithology::Sandstone,
                Lithology::Metamorphic,
                Lithology::Granite,
                Lithology::Basalt,
            ];
            let (nearest_fault_km, damage) =
                faults
                    .iter()
                    .fold((f32::INFINITY, 0.0_f32), |(nearest, strongest), fault| {
                        let distance = (world_x * fault.normal_x + world_y * fault.normal_y
                            - fault.offset_km)
                            .abs();
                        let intensity = (1.0 - distance / fault.damage_half_width_km)
                            .clamp(0.0, 1.0)
                            .powi(2);
                        (nearest.min(distance), strongest.max(intensity))
                    });
            let fault_displacement = faults
                .iter()
                .map(|fault| {
                    let signed =
                        world_x * fault.normal_x + world_y * fault.normal_y - fault.offset_km;
                    if signed >= 0.0 {
                        fault.throw_metres
                    } else {
                        0.0
                    }
                })
                .sum::<f32>();
            let resistance_variation =
                0.92 + value_noise(world_x / 7.0, world_y / 7.0, config.seed ^ 0x77c1_058d) * 0.16;
            let dip = (0.08 + fold.abs() * 0.62 + (broad - 0.5) * 0.08).clamp(0.02, 0.78);
            let stratigraphic_base = warped_layer * 1000.0 * dip.sin() + fault_displacement;
            let layer_index = (stratigraphic_base / layer_thickness_metres).floor() as i32;
            let rock = sequence[layer_index.rem_euclid(sequence.len() as i32) as usize];
            let resistance =
                (rock.erosion_resistance() * resistance_variation * (1.0 - damage * 0.42))
                    .clamp(0.34, 2.35);
            let weathering = (rock.chemical_weatherability() * 0.72
                + damage * 0.30
                + (1.0 - resistance / 2.35) * 0.18)
                .clamp(0.0, 1.0);

            lithology.push(rock);
            erosion_resistance.push(resistance);
            bedding_strike.push(local_strike);
            bedding_dip.push(dip);
            fold_phase.push(phase);
            stratigraphic_base_metres.push(stratigraphic_base);
            fault_distance_metres.push(nearest_fault_km * 1000.0);
            fracture_intensity.push(damage);
            weathering_potential.push(weathering);
        }
    }

    Ok(GeologyField {
        size: config.size,
        world_size_km: config.world_size_km,
        lithology,
        erosion_resistance,
        bedding_strike,
        bedding_dip,
        fold_phase,
        stratigraphic_base_metres,
        stratigraphic_phase,
        layer_thickness_metres,
        fault_distance_metres,
        fracture_intensity,
        weathering_potential,
        faults,
    })
}

fn hash01(seed: u32, x: u32, y: u32) -> f32 {
    let bits = mix32(seed ^ x.wrapping_mul(0x9e37_79b9) ^ y.wrapping_mul(0x85eb_ca6b));
    (bits >> 8) as f32 / 16_777_215.0
}

fn value_noise(x: f32, y: f32, seed: u32) -> f32 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let tx = smooth01(x - x.floor());
    let ty = smooth01(y - y.floor());
    let corner = |ix: i32, iy: i32| hash01(seed, ix as u32, iy as u32);
    let top = lerp(corner(x0, y0), corner(x0 + 1, y0), tx);
    let bottom = lerp(corner(x0, y0 + 1), corner(x0 + 1, y0 + 1), tx);
    lerp(top, bottom, ty)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(seed: u32) -> GeologyField {
        generate_geology(GeologyConfig {
            size: 128,
            world_size_km: 80.0,
            seed,
        })
        .unwrap()
    }

    #[test]
    fn generation_is_deterministic() {
        assert_eq!(fixture(42), fixture(42));
        assert_ne!(fixture(42).lithology, fixture(43).lithology);
    }

    #[test]
    fn lowering_the_surface_exposes_successive_finite_thickness_beds() {
        let mut field = fixture(913);
        let index = field.size / 2 * field.size + field.size / 2;
        let first_elevation = 250.0;
        let mut surface = vec![first_elevation; field.size * field.size];
        field.expose_surface(&surface);
        let first_phase = field.stratigraphic_phase[index];
        let first_rock = field.lithology[index];

        // Moving one bed-normal thickness through the same x/y coordinate
        // must expose exactly the neighbouring physical layer.
        surface[index] += field.layer_thickness_metres / field.bedding_dip[index].cos() * 1.01;
        field.expose_surface(&surface);
        let phase_advance = field.stratigraphic_phase[index] - first_phase;
        assert!((phase_advance - 1.01).abs() < 1.0e-3);
        assert_ne!(field.lithology[index], first_rock);
    }

    #[test]
    fn fault_damage_zone_is_stronger_near_faults() {
        let field = fixture(551);
        let mut near = Vec::new();
        let mut far = Vec::new();
        for (&distance, &fracture) in field
            .fault_distance_metres
            .iter()
            .zip(&field.fracture_intensity)
        {
            if distance < 350.0 {
                near.push(fracture);
            } else if distance > 2_000.0 {
                far.push(fracture);
            }
        }
        let mean = |values: &[f32]| values.iter().sum::<f32>() / values.len() as f32;
        assert!(!near.is_empty() && !far.is_empty());
        assert!(mean(&near) > mean(&far) + 0.2);
    }

    #[test]
    fn physical_coefficients_stay_in_declared_ranges() {
        let field = fixture(7);
        assert!(
            field
                .erosion_resistance
                .iter()
                .all(|value| (0.34..=2.35).contains(value))
        );
        assert!(
            field
                .weathering_potential
                .iter()
                .all(|value| (0.0..=1.0).contains(value))
        );
        assert!(
            field
                .bedding_dip
                .iter()
                .all(|value| (0.02..=0.78).contains(value))
        );
    }
}
