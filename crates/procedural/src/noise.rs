//! Deterministic value noise and fBm shared by every procedural system —
//! weathering masks, asphalt aggregate, facade variation, terrain detail.
//! One implementation lives here so no consumer grows its own hash.

/// 2-D integer hash to a float in [0, 1).  Same construction as the world
/// generators use, kept here as the single source of truth.
pub fn hash01(seed: u32, x: i32, y: i32, salt: i32) -> f32 {
    let mut value = seed
        ^ (x as u32).wrapping_mul(0x9e37_79b9)
        ^ (y as u32).wrapping_mul(0x85eb_ca6b)
        ^ (salt as u32).wrapping_mul(0xc2b2_ae35);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value as f32 / u32::MAX as f32
}

fn smooth(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Bilinear value noise in [0, 1) at an arbitrary (possibly fractional) point.
pub fn value_noise(seed: u32, x: f32, y: f32, salt: i32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let tx = smooth(x - x.floor());
    let ty = smooth(y - y.floor());
    let a = hash01(seed, xi, yi, salt);
    let b = hash01(seed, xi + 1, yi, salt);
    let c = hash01(seed, xi, yi + 1, salt);
    let d = hash01(seed, xi + 1, yi + 1, salt);
    a + (b - a) * tx + (c - a) * ty + (a - b - c + d) * tx * ty
}

/// Fractal Brownian motion over `value_noise`, normalised to [0, 1).
pub fn fbm(seed: u32, x: f32, y: f32, octaves: u32, salt: i32) -> f32 {
    let mut sum = 0.0_f32;
    let mut amplitude = 1.0_f32;
    let mut frequency = 1.0_f32;
    let mut norm = 0.0_f32;
    for octave in 0..octaves {
        sum += amplitude
            * value_noise(
                seed,
                x * frequency,
                y * frequency,
                salt + octave as i32 * 131,
            );
        norm += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    if norm <= 0.0 { 0.0 } else { sum / norm }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise_is_deterministic_and_bounded() {
        let a = fbm(42, 1.7, 3.2, 4, 7);
        let b = fbm(42, 1.7, 3.2, 4, 7);
        assert_eq!(a.to_bits(), b.to_bits());
        assert!((0.0..=1.0).contains(&a));
        let c = fbm(42, 100.0, 100.0, 4, 7);
        assert!((0.0..=1.0).contains(&c));
    }
}
