//! Conservative, persistent terrain evolution.
//!
//! This module deliberately owns no renderer and does not modify one-shot
//! terrain generation. Lengths are metres and stored water/sediment are
//! equivalent depths in metres per cell.

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionForcing {
    /// Atmospheric condensation supply (m/s). Rain is never inserted
    /// directly: vapour condenses to cloud water before it can precipitate.
    pub precipitation: f64,
    /// Potential evaporation rate (m/s).
    pub evaporation: f64,
    /// Fraction of potential evaporation available from wet cells.
    pub evaporation_efficiency: f64,
    /// Water vapour advected into and out of each atmospheric column (m/s).
    pub boundary_vapour_input: f64,
    pub boundary_vapour_output: f64,
    /// Spatially uniform near-surface air temperature (degrees Celsius).
    pub temperature_c: f64,
}

impl Default for EvolutionForcing {
    fn default() -> Self {
        Self {
            precipitation: 2.0e-8,
            evaporation: 1.3e-8,
            evaporation_efficiency: 1.0,
            boundary_vapour_input: 0.0,
            boundary_vapour_output: 0.0,
            temperature_c: 12.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionParameters {
    pub cell_size: f64,
    pub max_substep: f64,
    pub water_mobility: f64,
    pub open_boundary_rate: f64,
    pub sediment_capacity: f64,
    pub bedrock_erosion_rate: f64,
    pub sediment_entrainment_rate: f64,
    pub deposition_rate: f64,
    pub critical_slope: f64,
    pub hillslope_mobility: f64,
    /// First-order reservoir exchange rates (s^-1).
    pub cloud_precipitation_rate: f64,
    pub infiltration_rate: f64,
    pub percolation_rate: f64,
    pub groundwater_discharge_rate: f64,
    pub snowmelt_rate: f64,
    pub soil_field_capacity: f64,
}

impl Default for EvolutionParameters {
    fn default() -> Self {
        Self {
            cell_size: 250.0,
            max_substep: 900.0,
            water_mobility: 0.18,
            open_boundary_rate: 0.08,
            sediment_capacity: 0.08,
            bedrock_erosion_rate: 1.0e-10,
            sediment_entrainment_rate: 1.0e-5,
            deposition_rate: 3.0e-4,
            critical_slope: 0.7,
            hillslope_mobility: 0.06,
            cloud_precipitation_rate: 1.0 / 21_600.0,
            infiltration_rate: 1.0 / 10_800.0,
            percolation_rate: 1.0 / 172_800.0,
            groundwater_discharge_rate: 1.0 / 1_209_600.0,
            snowmelt_rate: 2.0e-7,
            soil_field_capacity: 0.25,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EvolutionDiagnostics {
    pub elapsed_time: f64,
    pub substeps: u64,
    pub water_storage: f64,
    pub solid_storage: f64,
    pub cumulative_precipitation: f64,
    pub cumulative_evaporation: f64,
    pub cumulative_transpiration: f64,
    pub cumulative_boundary_vapour_input: f64,
    pub cumulative_boundary_vapour_output: f64,
    pub cumulative_water_outflow: f64,
    pub cumulative_sediment_outflow: f64,
    pub water_balance_error: f64,
    pub solid_balance_error: f64,
    pub max_bedrock_erosion: f64,
    /// Numerical failures detected before committing a substep. This is kept
    /// separate from all physical flux ledgers.
    pub numerical_rollbacks: u64,
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum EvolutionError {
    #[error("evolution grid dimensions or field lengths are invalid")]
    InvalidGrid,
    #[error("evolution parameters or time step are invalid")]
    InvalidParameter,
    #[error("evolution produced a non-finite or negative conserved field")]
    InvalidState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionState {
    pub width: usize,
    pub height: usize,
    /// Elevation of the consolidated rock surface (m).
    pub bedrock: Vec<f64>,
    /// Unconsolidated material resting on the rock (m equivalent depth).
    pub mobile_sediment: Vec<f64>,
    /// Material carried by water (m equivalent depth).
    pub suspended_sediment: Vec<f64>,
    pub surface_water: Vec<f64>,
    /// Persistent stores, all expressed as metres water equivalent per cell.
    pub atmospheric_vapour: Vec<f64>,
    pub cloud_water: Vec<f64>,
    pub soil_water: Vec<f64>,
    pub groundwater: Vec<f64>,
    pub snow_water: Vec<f64>,
    /// Dimensionless multiplier; larger values denote more resistant rock.
    pub bedrock_resistance: Vec<f64>,
    pub forcing: EvolutionForcing,
    pub parameters: EvolutionParameters,
    pub diagnostics: EvolutionDiagnostics,
    initial_water: f64,
    initial_solid: f64,
}

impl EvolutionState {
    pub fn new(
        width: usize,
        height: usize,
        bedrock: Vec<f64>,
        mobile_sediment: Vec<f64>,
        surface_water: Vec<f64>,
        bedrock_resistance: Vec<f64>,
        forcing: EvolutionForcing,
        parameters: EvolutionParameters,
    ) -> Result<Self, EvolutionError> {
        let len = width
            .checked_mul(height)
            .ok_or(EvolutionError::InvalidGrid)?;
        if width < 2
            || height < 2
            || [
                bedrock.len(),
                mobile_sediment.len(),
                surface_water.len(),
                bedrock_resistance.len(),
            ]
            .iter()
            .any(|&n| n != len)
        {
            return Err(EvolutionError::InvalidGrid);
        }
        // A small, finite atmospheric/terrestrial inventory starts the
        // coupled cycle. It is part of the ledger, not an inexhaustible source.
        let atmospheric_vapour = vec![0.018; len];
        let cloud_water = vec![0.0015; len];
        let soil_water = vec![0.12; len];
        let groundwater = vec![0.08; len];
        let snow_water = vec![0.0; len];
        let initial_water = sum(&surface_water)
            + sum(&atmospheric_vapour)
            + sum(&cloud_water)
            + sum(&soil_water)
            + sum(&groundwater)
            + sum(&snow_water);
        let initial_solid = sum(&bedrock) + sum(&mobile_sediment);
        let mut state = Self {
            width,
            height,
            bedrock,
            mobile_sediment,
            suspended_sediment: vec![0.0; len],
            surface_water,
            atmospheric_vapour,
            cloud_water,
            soil_water,
            groundwater,
            snow_water,
            bedrock_resistance,
            forcing,
            parameters,
            diagnostics: EvolutionDiagnostics::default(),
            initial_water,
            initial_solid,
        };
        state.validate()?;
        state.refresh_diagnostics(0.0);
        Ok(state)
    }

    pub fn surface_elevation(&self, index: usize) -> f64 {
        self.bedrock[index] + self.mobile_sediment[index]
    }

    /// Advances the persistent state. The requested step is split to keep
    /// water, sediment transport and hillslope motion monotone and bounded.
    pub fn advance(&mut self, dt: f64) -> Result<&EvolutionDiagnostics, EvolutionError> {
        if !dt.is_finite() || dt <= 0.0 {
            return Err(EvolutionError::InvalidParameter);
        }
        self.validate()?;
        let mut remaining = dt;
        while remaining > 0.0 {
            let adaptive = self.stable_substep();
            let h = remaining.min(adaptive);
            // A rejected numerical step never leaks into physical ledgers.
            // Validation is a fault detector, not a terrain-height clamp.
            let checkpoint = self.clone();
            if let Err(error) = self.substep(h) {
                let rollbacks = checkpoint.diagnostics.numerical_rollbacks + 1;
                *self = checkpoint;
                self.diagnostics.numerical_rollbacks = rollbacks;
                return Err(error);
            }
            remaining -= h;
            self.diagnostics.elapsed_time += h;
            self.diagnostics.substeps += 1;
        }
        self.refresh_diagnostics(0.0);
        self.validate()?;
        Ok(&self.diagnostics)
    }

    fn stable_substep(&self) -> f64 {
        // Mobility coefficients are expressed as fractions per reference
        // step. Limiting every process to <= 25% prevents overshoot.
        let p = &self.parameters;
        let rate = p
            .water_mobility
            .max(p.open_boundary_rate)
            .max(p.hillslope_mobility)
            .max(p.sediment_entrainment_rate * 1000.0)
            .max(p.deposition_rate * 1000.0);
        p.max_substep
            .min(0.25 * 1000.0 / rate.max(1.0e-12))
            .max(1.0e-6)
    }

    fn substep(&mut self, dt: f64) -> Result<(), EvolutionError> {
        self.exchange_water_reservoirs(dt);
        self.route_water_and_sediment(dt);
        self.erode_and_deposit(dt);
        self.relax_hillslopes(dt);
        self.validate()
    }

    fn exchange_water_reservoirs(&mut self, dt: f64) {
        let n = self.width * self.height;
        let input = self.forcing.boundary_vapour_input * dt;
        let output_demand = self.forcing.boundary_vapour_output * dt;
        let condensation_demand = self.forcing.precipitation * dt;
        let evap_demand = self.forcing.evaporation * self.forcing.evaporation_efficiency * dt;
        let precip_fraction = 1.0 - (-self.parameters.cloud_precipitation_rate * dt).exp();
        let infiltration_fraction = 1.0 - (-self.parameters.infiltration_rate * dt).exp();
        let percolation_fraction = 1.0 - (-self.parameters.percolation_rate * dt).exp();
        let discharge_fraction = 1.0 - (-self.parameters.groundwater_discharge_rate * dt).exp();
        let mut precipitation = 0.0;
        let mut evaporation = 0.0;
        let mut transpiration = 0.0;
        let mut boundary_out = 0.0;

        for i in 0..n {
            self.atmospheric_vapour[i] += input;
            let exported = self.atmospheric_vapour[i].min(output_demand);
            self.atmospheric_vapour[i] -= exported;
            boundary_out += exported;

            // Cooling lowers atmospheric holding capacity. The forcing is an
            // uplift/convergence tendency, but water must already be present.
            let holding_capacity =
                (0.010 * (0.055 * self.forcing.temperature_c).exp()).clamp(0.001, 0.08);
            let supersaturation = (self.atmospheric_vapour[i] - holding_capacity).max(0.0);
            let condensed =
                self.atmospheric_vapour[i].min(condensation_demand + supersaturation * 0.35);
            self.atmospheric_vapour[i] -= condensed;
            self.cloud_water[i] += condensed;

            let rain_or_snow = self.cloud_water[i] * precip_fraction;
            self.cloud_water[i] -= rain_or_snow;
            precipitation += rain_or_snow;
            if self.forcing.temperature_c <= 0.0 {
                self.snow_water[i] += rain_or_snow;
            } else {
                self.surface_water[i] += rain_or_snow;
            }

            if self.forcing.temperature_c > 0.0 {
                let melt = self.snow_water[i]
                    .min(self.parameters.snowmelt_rate * self.forcing.temperature_c * dt);
                self.snow_water[i] -= melt;
                self.surface_water[i] += melt;
            }

            let infiltrated = self.surface_water[i] * infiltration_fraction;
            self.surface_water[i] -= infiltrated;
            self.soil_water[i] += infiltrated;

            let excess_soil = (self.soil_water[i] - self.parameters.soil_field_capacity).max(0.0);
            let percolated = excess_soil * percolation_fraction;
            self.soil_water[i] -= percolated;
            self.groundwater[i] += percolated;

            let baseflow = self.groundwater[i] * discharge_fraction;
            self.groundwater[i] -= baseflow;
            self.surface_water[i] += baseflow;

            let surface_evap = self.surface_water[i].min(evap_demand * 0.65);
            self.surface_water[i] -= surface_evap;
            let soil_demand = (evap_demand - surface_evap).max(0.0);
            let soil_available = (self.soil_water[i] - 0.02).max(0.0);
            let soil_evap = soil_available.min(soil_demand * 0.35);
            self.soil_water[i] -= soil_evap;
            self.atmospheric_vapour[i] += surface_evap + soil_evap;
            evaporation += surface_evap;
            transpiration += soil_evap;
        }
        self.diagnostics.cumulative_precipitation += precipitation;
        self.diagnostics.cumulative_evaporation += evaporation;
        self.diagnostics.cumulative_transpiration += transpiration;
        self.diagnostics.cumulative_boundary_vapour_input += input * n as f64;
        self.diagnostics.cumulative_boundary_vapour_output += boundary_out;
    }

    fn route_water_and_sediment(&mut self, dt: f64) {
        let n = self.width * self.height;
        let mut dw = vec![0.0; n];
        let mut ds = vec![0.0; n];
        let flow_fraction = (self.parameters.water_mobility * dt / 1000.0).min(0.24);
        let boundary_fraction = (self.parameters.open_boundary_rate * dt / 1000.0).min(0.24);
        let mut water_out = 0.0;
        let mut sediment_out = 0.0;
        for i in 0..n {
            let x = i % self.width;
            let y = i / self.width;
            let head = self.surface_elevation(i) + self.surface_water[i];
            let mut lower = [(usize::MAX, 0.0); 4];
            let mut count = 0;
            for (nx, ny) in neighbours(x, y, self.width, self.height) {
                let j = ny * self.width + nx;
                let drop = (head - self.surface_elevation(j) - self.surface_water[j]).max(0.0);
                if drop > 0.0 {
                    lower[count] = (j, drop);
                    count += 1;
                }
            }
            let drop_sum: f64 = lower[..count].iter().map(|x| x.1).sum();
            let internal = self.surface_water[i]
                * flow_fraction
                * (drop_sum / (drop_sum + self.parameters.cell_size * 0.001));
            let edge = x == 0 || y == 0 || x + 1 == self.width || y + 1 == self.height;
            let exported = if edge {
                (self.surface_water[i] - internal).max(0.0) * boundary_fraction
            } else {
                0.0
            };
            let total = internal + exported;
            if total <= 0.0 {
                continue;
            }
            let concentration = self.suspended_sediment[i] / self.surface_water[i].max(1.0e-15);
            dw[i] -= total;
            ds[i] -= total * concentration;
            if drop_sum > 0.0 {
                for &(j, drop) in &lower[..count] {
                    let amount = internal * drop / drop_sum;
                    dw[j] += amount;
                    ds[j] += amount * concentration;
                }
            }
            water_out += exported;
            sediment_out += exported * concentration;
        }
        for i in 0..n {
            self.surface_water[i] += dw[i];
            self.suspended_sediment[i] += ds[i];
        }
        self.diagnostics.cumulative_water_outflow += water_out;
        self.diagnostics.cumulative_sediment_outflow += sediment_out;
    }

    fn erode_and_deposit(&mut self, dt: f64) {
        let n = self.width * self.height;
        let mut maximum = 0.0_f64;
        for i in 0..n {
            let slope = self.local_downhill_slope(i);
            let water_power = self.surface_water[i].sqrt() * slope;
            let capacity = self.parameters.sediment_capacity * water_power * self.surface_water[i];
            if self.suspended_sediment[i] > capacity {
                let deposit = (self.parameters.deposition_rate * dt).min(0.25)
                    * (self.suspended_sediment[i] - capacity);
                self.suspended_sediment[i] -= deposit;
                self.mobile_sediment[i] += deposit;
            } else {
                let spare = capacity - self.suspended_sediment[i];
                let entrained =
                    self.mobile_sediment[i].min(spare.min(
                        self.mobile_sediment[i] * self.parameters.sediment_entrainment_rate * dt,
                    ));
                self.mobile_sediment[i] -= entrained;
                self.suspended_sediment[i] += entrained;
                let remaining = spare - entrained;
                // There is no artificial elevation floor. Incision naturally
                // shuts down without water/slope/capacity and slows in hard rock.
                let eroded = remaining.min(
                    self.parameters.bedrock_erosion_rate * dt * water_power
                        / self.bedrock_resistance[i].max(1.0e-6),
                );
                self.bedrock[i] -= eroded;
                self.suspended_sediment[i] += eroded;
                maximum = maximum.max(eroded);
            }
        }
        self.diagnostics.max_bedrock_erosion = self.diagnostics.max_bedrock_erosion.max(maximum);
    }

    fn relax_hillslopes(&mut self, dt: f64) {
        let n = self.width * self.height;
        let mut change = vec![0.0; n];
        let fraction = (self.parameters.hillslope_mobility * dt / 1000.0).min(0.24);
        for i in 0..n {
            if self.mobile_sediment[i] <= 0.0 {
                continue;
            }
            let x = i % self.width;
            let y = i / self.width;
            let zi = self.surface_elevation(i);
            let mut target = None;
            let mut greatest = self.parameters.critical_slope;
            for (nx, ny) in neighbours(x, y, self.width, self.height) {
                let j = ny * self.width + nx;
                let slope = (zi - self.surface_elevation(j)) / self.parameters.cell_size;
                if slope > greatest {
                    greatest = slope;
                    target = Some(j);
                }
            }
            if let Some(j) = target {
                let excess =
                    (greatest - self.parameters.critical_slope) * self.parameters.cell_size * 0.5;
                let moved = self.mobile_sediment[i].min(excess * fraction);
                change[i] -= moved;
                change[j] += moved;
            }
        }
        for (s, delta) in self.mobile_sediment.iter_mut().zip(change) {
            *s += delta;
        }
    }

    fn local_downhill_slope(&self, i: usize) -> f64 {
        let x = i % self.width;
        let y = i / self.width;
        let z = self.surface_elevation(i);
        neighbours(x, y, self.width, self.height)
            .map(|(nx, ny)| {
                ((z - self.surface_elevation(ny * self.width + nx)) / self.parameters.cell_size)
                    .max(0.0)
            })
            .fold(0.0, f64::max)
    }

    fn refresh_diagnostics(&mut self, _unused: f64) {
        self.diagnostics.water_storage = sum(&self.surface_water)
            + sum(&self.atmospheric_vapour)
            + sum(&self.cloud_water)
            + sum(&self.soil_water)
            + sum(&self.groundwater)
            + sum(&self.snow_water);
        self.diagnostics.solid_storage =
            sum(&self.bedrock) + sum(&self.mobile_sediment) + sum(&self.suspended_sediment);
        self.diagnostics.water_balance_error = self.diagnostics.water_storage
            + self.diagnostics.cumulative_water_outflow
            + self.diagnostics.cumulative_boundary_vapour_output
            - self.initial_water
            - self.diagnostics.cumulative_boundary_vapour_input;
        self.diagnostics.solid_balance_error = self.diagnostics.solid_storage
            + self.diagnostics.cumulative_sediment_outflow
            - self.initial_solid;
    }

    fn validate(&self) -> Result<(), EvolutionError> {
        let p = &self.parameters;
        let forcing_ok = self.forcing.precipitation.is_finite()
            && self.forcing.precipitation >= 0.0
            && self.forcing.evaporation.is_finite()
            && self.forcing.evaporation >= 0.0
            && self.forcing.evaporation_efficiency.is_finite()
            && self.forcing.evaporation_efficiency >= 0.0
            && self.forcing.boundary_vapour_input.is_finite()
            && self.forcing.boundary_vapour_input >= 0.0
            && self.forcing.boundary_vapour_output.is_finite()
            && self.forcing.boundary_vapour_output >= 0.0
            && self.forcing.temperature_c.is_finite();
        let params_ok = p.cell_size > 0.0
            && p.max_substep > 0.0
            && [
                p.cell_size,
                p.max_substep,
                p.water_mobility,
                p.open_boundary_rate,
                p.sediment_capacity,
                p.bedrock_erosion_rate,
                p.sediment_entrainment_rate,
                p.deposition_rate,
                p.critical_slope,
                p.hillslope_mobility,
                p.cloud_precipitation_rate,
                p.infiltration_rate,
                p.percolation_rate,
                p.groundwater_discharge_rate,
                p.snowmelt_rate,
                p.soil_field_capacity,
            ]
            .iter()
            .all(|v| v.is_finite() && *v >= 0.0);
        if !forcing_ok || !params_ok {
            return Err(EvolutionError::InvalidParameter);
        }
        let finite = self
            .bedrock
            .iter()
            .chain(&self.mobile_sediment)
            .chain(&self.suspended_sediment)
            .chain(&self.surface_water)
            .chain(&self.atmospheric_vapour)
            .chain(&self.cloud_water)
            .chain(&self.soil_water)
            .chain(&self.groundwater)
            .chain(&self.snow_water)
            .chain(&self.bedrock_resistance)
            .all(|v| v.is_finite());
        let nonnegative = self
            .mobile_sediment
            .iter()
            .chain(&self.suspended_sediment)
            .chain(&self.surface_water)
            .chain(&self.atmospheric_vapour)
            .chain(&self.cloud_water)
            .chain(&self.soil_water)
            .chain(&self.groundwater)
            .chain(&self.snow_water)
            .all(|v| *v >= -1.0e-12)
            && self.bedrock_resistance.iter().all(|v| *v > 0.0);
        let len = self.width.saturating_mul(self.height);
        let lengths_ok = [
            self.bedrock.len(),
            self.mobile_sediment.len(),
            self.suspended_sediment.len(),
            self.surface_water.len(),
            self.atmospheric_vapour.len(),
            self.cloud_water.len(),
            self.soil_water.len(),
            self.groundwater.len(),
            self.snow_water.len(),
            self.bedrock_resistance.len(),
        ]
        .iter()
        .all(|&n| n == len);
        if finite && nonnegative && lengths_ok {
            Ok(())
        } else {
            Err(EvolutionError::InvalidState)
        }
    }
}

fn neighbours(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let mut cells = [(0, 0); 4];
    let mut n = 0;
    if x > 0 {
        cells[n] = (x - 1, y);
        n += 1;
    }
    if x + 1 < width {
        cells[n] = (x + 1, y);
        n += 1;
    }
    if y > 0 {
        cells[n] = (x, y - 1);
        n += 1;
    }
    if y + 1 < height {
        cells[n] = (x, y + 1);
        n += 1;
    }
    cells.into_iter().take(n)
}

fn sum(values: &[f64]) -> f64 {
    values.iter().copied().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> EvolutionState {
        let (w, h) = (18, 14);
        let mut rock = Vec::new();
        for y in 0..h {
            for x in 0..w {
                rock.push(400.0 + (h - y) as f64 * 7.0 + ((x * 17 + y * 11) % 9) as f64);
            }
        }
        EvolutionState::new(
            w,
            h,
            rock,
            vec![1.5; w * h],
            vec![0.02; w * h],
            (0..w * h).map(|i| 0.8 + (i % 7) as f64 * 0.17).collect(),
            EvolutionForcing::default(),
            EvolutionParameters::default(),
        )
        .unwrap()
    }

    #[test]
    fn long_evolution_remains_finite_and_has_no_runaway_holes() {
        let mut s = fixture();
        let initial_min = s.bedrock.iter().copied().fold(f64::INFINITY, f64::min);
        for _ in 0..240 {
            s.advance(6.0 * 3600.0).unwrap();
        }
        assert!(
            s.bedrock
                .iter()
                .chain(&s.mobile_sediment)
                .chain(&s.surface_water)
                .all(|v| v.is_finite())
        );
        let final_min = s.bedrock.iter().copied().fold(f64::INFINITY, f64::min);
        assert!(
            initial_min - final_min < 5.0,
            "physical controls failed: incision {} m",
            initial_min - final_min
        );
    }

    #[test]
    fn water_and_solid_ledgers_conserve_mass() {
        let mut s = fixture();
        for _ in 0..40 {
            s.advance(12_345.0).unwrap();
        }
        let d = &s.diagnostics;
        assert!(d.water_balance_error.abs() < 1.0e-9 * (1.0 + d.cumulative_precipitation));
        assert!(d.solid_balance_error.abs() < 1.0e-9 * (1.0 + d.solid_storage.abs()));
        assert!(d.cumulative_water_outflow > 0.0);
    }

    #[test]
    fn closed_coupled_cycle_conserves_every_water_reservoir() {
        let mut s = fixture();
        s.forcing.boundary_vapour_input = 0.0;
        s.forcing.boundary_vapour_output = 0.0;
        s.parameters.open_boundary_rate = 0.0;
        for _ in 0..80 {
            s.advance(3_600.0).unwrap();
        }
        assert!(s.diagnostics.cumulative_precipitation > 0.0);
        assert!(s.diagnostics.cumulative_evaporation > 0.0);
        assert!(s.diagnostics.cumulative_transpiration > 0.0);
        assert!(
            s.diagnostics.water_balance_error.abs() < 1.0e-10 * (1.0 + s.diagnostics.water_storage)
        );
    }

    #[test]
    fn storm_water_has_a_delayed_groundwater_to_river_response() {
        let mut s = fixture();
        s.surface_water.fill(0.0);
        s.atmospheric_vapour.fill(0.0);
        s.cloud_water.fill(0.03);
        s.soil_water.fill(0.24);
        s.groundwater.fill(0.0);
        s.forcing.precipitation = 0.0;
        s.forcing.evaporation = 0.0;
        s.parameters.cloud_precipitation_rate = 1.0 / 1_800.0;
        s.parameters.infiltration_rate = 1.0 / 1_800.0;
        s.parameters.percolation_rate = 1.0 / 3_600.0;
        s.parameters.groundwater_discharge_rate = 1.0 / 172_800.0;

        s.advance(3_600.0).unwrap();
        let groundwater_after_storm = sum(&s.groundwater);
        assert!(groundwater_after_storm > 0.0);
        // End the atmospheric pulse. Stored subsurface water must continue
        // feeding surface drainage after rain has stopped.
        s.cloud_water.fill(0.0);
        let outflow_at_storm_end = s.diagnostics.cumulative_water_outflow;
        let mut without_baseflow = s.clone();
        without_baseflow.parameters.groundwater_discharge_rate = 0.0;
        s.advance(4.0 * 86_400.0).unwrap();
        without_baseflow.advance(4.0 * 86_400.0).unwrap();
        assert!(s.diagnostics.cumulative_water_outflow > outflow_at_storm_end);
        assert!(
            s.diagnostics.cumulative_water_outflow
                > without_baseflow.diagnostics.cumulative_water_outflow
        );
    }

    #[test]
    fn evaporation_returns_to_atmosphere_and_can_form_cloud_again() {
        let mut s = fixture();
        s.parameters.open_boundary_rate = 0.0;
        s.forcing.precipitation = 0.0;
        s.forcing.temperature_c = 30.0;
        s.atmospheric_vapour.fill(0.0);
        s.cloud_water.fill(0.0);
        s.initial_water = sum(&s.surface_water)
            + sum(&s.atmospheric_vapour)
            + sum(&s.cloud_water)
            + sum(&s.soil_water)
            + sum(&s.groundwater)
            + sum(&s.snow_water);
        let vapour_before = sum(&s.atmospheric_vapour);
        s.advance(3_600.0).unwrap();
        assert!(sum(&s.atmospheric_vapour) > vapour_before);

        let cloud_before = sum(&s.cloud_water);
        s.forcing.temperature_c = -5.0;
        s.forcing.precipitation = 2.0e-7;
        s.advance(900.0).unwrap();
        assert!(sum(&s.cloud_water) > cloud_before);
        assert!(s.diagnostics.water_balance_error.abs() < 1.0e-8);
    }

    #[test]
    fn evolution_is_deterministic() {
        let mut a = fixture();
        let mut b = fixture();
        for _ in 0..17 {
            a.advance(4321.0).unwrap();
            b.advance(4321.0).unwrap();
        }
        assert_eq!(a.bedrock, b.bedrock);
        assert_eq!(a.mobile_sediment, b.mobile_sediment);
        assert_eq!(a.surface_water, b.surface_water);
        assert_eq!(
            a.diagnostics.water_balance_error.to_bits(),
            b.diagnostics.water_balance_error.to_bits()
        );
    }
}
