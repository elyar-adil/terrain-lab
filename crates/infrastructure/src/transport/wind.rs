//! A deterministic wind rose from the simulation's wind setting, and the
//! usability factor of a runway heading (ICAO Annex 14 Att. A; 95 % target).

use serde::{Deserialize, Serialize};

/// Surface wind climate: a von Mises direction distribution about the
/// prevailing direction and a Weibull (k = 2) speed distribution.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WindRose {
    /// Direction the prevailing flow moves toward, radians in the grid frame.
    pub prevailing_rad: f32,
    pub kappa: f32,
    /// Weibull scale, m/s.
    pub scale_ms: f32,
}

impl WindRose {
    /// `speed_ms` and `direction_deg` are the simulation's (`SimulationConfig`)
    /// flow aloft. Surface wind at airport height is weaker (boundary layer):
    /// the mean is taken as half of it. Concentration grows with the flow.
    pub fn from_config(direction_deg: f32, speed_ms: f32) -> Self {
        let mean = (speed_ms * 0.5).max(0.5);
        Self {
            prevailing_rad: direction_deg.to_radians(),
            kappa: 0.8 + 0.25 * speed_ms,
            // Weibull k = 2 mean = c * sqrt(pi)/2.
            scale_ms: mean / 0.886_227,
        }
    }

    /// Share of time the crosswind on a runway with axis angle `axis_rad`
    /// (either direction of use) stays at or below `limit_ms`.
    pub fn usability(&self, axis_rad: f32, limit_ms: f32) -> f32 {
        const BINS: usize = 180;
        let mut norm = 0.0_f64;
        let mut ok = 0.0_f64;
        for k in 0..BINS {
            let phi = (k as f32 + 0.5) / BINS as f32 * std::f32::consts::TAU;
            let w = f64::from((self.kappa * (phi - self.prevailing_rad).cos()).exp());
            norm += w;
            let s = (phi - axis_rad).sin().abs();
            let p = if s < 1.0e-4 {
                1.0
            } else {
                let x = limit_ms / (s * self.scale_ms);
                1.0 - (-(x * x)).exp()
            };
            ok += w * f64::from(p);
        }
        (ok / norm) as f32
    }
}
