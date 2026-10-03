//! A default `UrbanField` for a world with no settlement data: towns and villages
//! at hashed places. Fully stateless, so a standalone user gets towns anywhere.

use std::sync::Arc;

use worldgen_contracts::{UrbanField, V2, v2};
use worldgen_core::noise::fbm;
use worldgen_core::{Cell, Seed};

use crate::config::smoothstep;

#[derive(Debug, Clone)]
pub struct HashedTowns {
    seed: Seed,
    /// Spacing of the lattice candidate cities sit on, metres.
    pub city_cell_m: f64,
    pub city_presence: f64,
    pub city_radius_m: (f64, f64),
    pub village_cell_m: f64,
    pub village_presence: f64,
    pub village_radius_m: (f64, f64),
}

impl HashedTowns {
    pub fn new(seed: Seed) -> Self {
        Self {
            seed: seed.derive("towns"),
            city_cell_m: 9000.0,
            city_presence: 0.75,
            city_radius_m: (1200.0, 3600.0),
            village_cell_m: 2600.0,
            village_presence: 0.55,
            village_radius_m: (180.0, 520.0),
        }
    }

    pub fn shared(seed: Seed) -> Arc<dyn UrbanField> {
        Arc::new(Self::new(seed))
    }

    /// The strongest bump from the towns of one kind near `p`.
    fn kind(
        &self,
        salt: &str,
        cell_m: f64,
        presence: f64,
        radius: (f64, f64),
        cap: f64,
        p: V2,
    ) -> f64 {
        let seed = self.seed.derive(salt);
        let (cx, cy) = ((p.x / cell_m).floor() as i64, (p.y / cell_m).floor() as i64);
        let mut best = 0.0_f64;
        for iy in cy - 1..=cy + 1 {
            for ix in cx - 1..=cx + 1 {
                let h = seed.derive_cell(Cell::new(0, ix, iy));
                if h.derive("here").unit() >= presence {
                    continue;
                }
                let centre = v2(
                    (ix as f64 + 0.15 + 0.7 * h.derive("x").unit()) * cell_m,
                    (iy as f64 + 0.15 + 0.7 * h.derive("y").unit()) * cell_m,
                );
                let r = radius.0 + (radius.1 - radius.0) * h.derive("r").unit().powi(2);
                // Towns are not round: stretch along a random axis.
                let stretch = 1.0 + 0.55 * (h.derive("e").unit() - 0.3);
                let angle = h.derive("a").unit() * std::f64::consts::PI;
                let local = (p - centre).rotate(-angle);
                let d = v2(local.x / stretch, local.y * stretch).len() / r;
                // And ragged at the edge.
                let rag =
                    0.72 + 0.56 * fbm(h.derive("n"), p.x / (r * 0.55), p.y / (r * 0.55), 3, 0.5);
                best = best.max(cap * smoothstep(1.1, 0.12, d * (2.0 - rag)));
            }
        }
        best
    }
}

impl UrbanField for HashedTowns {
    fn urbanness(&self, p: V2) -> f64 {
        let city = self.kind(
            "city",
            self.city_cell_m,
            self.city_presence,
            self.city_radius_m,
            1.0,
            p,
        );
        let village = self.kind(
            "village",
            self.village_cell_m,
            self.village_presence,
            self.village_radius_m,
            0.5,
            p,
        );
        city.max(village)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn towns_are_deterministic_bounded_and_not_everywhere() {
        let towns = HashedTowns::new(Seed::new(7));
        let mut urban = 0;
        let mut dense = 0;
        let n = 6000;
        for k in 0..n {
            let p = v2((k % 100) as f64 * 400.0, (k / 100) as f64 * 400.0);
            let u = towns.urbanness(p);
            assert!((0.0..=1.0).contains(&u));
            assert_eq!(u, towns.urbanness(p));
            urban += usize::from(u > 0.3);
            dense += usize::from(u > 0.8);
        }
        assert!(
            urban > n / 40 && urban < n / 2,
            "{urban} of {n} places are built up"
        );
        assert!(dense > 0 && dense < urban);
    }

    #[test]
    fn a_town_fades_out_smoothly_rather_than_ending_at_a_wall() {
        let towns = HashedTowns::new(Seed::new(7));
        let mut worst = 0.0_f64;
        for k in 0..4000 {
            let p = v2(k as f64 * 5.0, 3000.0);
            worst = worst.max((towns.urbanness(p) - towns.urbanness(p + v2(5.0, 0.0))).abs());
        }
        assert!(
            worst < 0.08,
            "urbanness jumps by {worst} across five metres"
        );
    }
}
