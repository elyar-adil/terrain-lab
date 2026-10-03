//! A small, deterministic L-System engine plus the 3D turtle that turns a
//! symbol string into tapered branch segments.  Grammar conventions follow the
//! classic ABOP notation so species definitions stay readable:
//!
//! - `F` draw one step forward
//! - `+` / `-` yaw left / right, `&` / `^` pitch down / up, `/` / `\` roll
//! - `[` begin a branch, `]` close it (restores position, heading, thickness)
//! - `L` emit a foliage blob at the current tip
//! - `A`..`D` free non-terminals for species productions

use crate::{FoliageBlob, Segment, SplitMix64, Tropism};

#[derive(Clone, Debug)]
pub struct LSystem {
    axiom: String,
    rules: Vec<(char, String)>,
}

impl LSystem {
    pub fn new(axiom: &str) -> Self {
        Self {
            axiom: axiom.to_string(),
            rules: Vec::new(),
        }
    }

    pub fn with_rule(mut self, symbol: char, production: &str) -> Self {
        self.rules.push((symbol, production.to_string()));
        self
    }

    /// Expand the axiom `iterations` times.  Expansion is pure string
    /// rewriting, so the same grammar always yields the same symbols.
    pub fn expand(&self, iterations: usize) -> String {
        let mut current = self.axiom.clone();
        for _ in 0..iterations {
            let mut next = String::with_capacity(current.len() * 2);
            for symbol in current.chars() {
                match self.rules.iter().find(|(from, _)| *from == symbol) {
                    Some((_, production)) => next.push_str(production),
                    None => next.push(symbol),
                }
            }
            current = next;
        }
        current
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TurtleParams {
    /// Forward step per `F`, metres.
    pub step: f32,
    /// Turn angle for the rotation symbols, degrees.
    pub angle_deg: f32,
    /// Trunk radius at the root, metres.
    pub base_radius: f32,
    /// Radius multiplier applied per step so branches taper.
    pub radius_decay: f32,
    /// Multiplier applied to the step length inside branches.
    pub step_decay: f32,
    /// Phototropism: positive pulls tips toward +Y (light), negative droops.
    pub tropism: Tropism,
    /// Typical foliage blob radius, metres.
    pub foliage_radius: f32,
}

impl Default for TurtleParams {
    fn default() -> Self {
        Self {
            step: 1.0,
            angle_deg: 28.0,
            base_radius: 0.22,
            radius_decay: 0.82,
            step_decay: 0.94,
            tropism: Tropism { up: 0.18 },
            foliage_radius: 1.6,
        }
    }
}

/// Rotate `v` around the unit axis `k` by `theta` radians (Rodrigues).
fn rotate_around(v: [f32; 3], k: [f32; 3], theta: f32) -> [f32; 3] {
    let (sin, cos) = theta.sin_cos();
    let cross = [
        k[1] * v[2] - k[2] * v[1],
        k[2] * v[0] - k[0] * v[2],
        k[0] * v[1] - k[1] * v[0],
    ];
    let dot = v[0] * k[0] + v[1] * k[1] + v[2] * k[2];
    [
        v[0] * cos + cross[0] * sin + k[0] * dot * (1.0 - cos),
        v[1] * cos + cross[1] * sin + k[1] * dot * (1.0 - cos),
        v[2] * cos + cross[2] * sin + k[2] * dot * (1.0 - cos),
    ]
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if length > 1e-6 {
        [v[0] / length, v[1] / length, v[2] / length]
    } else {
        [0.0, 1.0, 0.0]
    }
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Interpret an expanded symbol string with the 3D turtle.  `jitter` is the
/// per-step angular noise in degrees so two trees of the same species grown
/// from different seeds never read as clones.
pub fn interpret(
    symbols: &str,
    params: &TurtleParams,
    jitter: f32,
    rng: &mut SplitMix64,
) -> (Vec<Segment>, Vec<FoliageBlob>) {
    let angle = params.angle_deg.to_radians();
    let jitter_rad = jitter.to_radians();
    let mut segments = Vec::new();
    let mut foliage = Vec::new();

    let mut position = [0.0_f32, 0.0, 0.0];
    let mut heading = [0.0_f32, 1.0, 0.0];
    // Side/binormal complete the local frame; `side` starts east, and roll
    // symbols spin it around the heading so branch whorls distribute evenly.
    let mut side = [1.0_f32, 0.0, 0.0];
    let mut step = params.step;
    let mut radius = params.base_radius;
    let mut stack: Vec<([f32; 3], [f32; 3], [f32; 3], f32, f32)> = Vec::new();

    let turn = |axis_kind: u8, sign: f32, heading: &mut [f32; 3], side: &mut [f32; 3]| {
        let axis = match axis_kind {
            // Yaw rotates around the local "up-left" axis: cross(heading, side).
            0 => cross(*heading, *side),
            // Pitch rotates around the side axis.
            1 => *side,
            // Roll rotates around the heading itself.
            _ => *heading,
        };
        let axis = normalize(axis);
        *heading = normalize(rotate_around(*heading, axis, sign * angle));
        if axis_kind != 2 {
            *side = normalize(rotate_around(*side, axis, sign * angle));
        }
    };

    for symbol in symbols.chars() {
        match symbol {
            'F' => {
                if jitter_rad > 0.0 {
                    let noise = (rng.next_f64() as f32 - 0.5) * 2.0 * jitter_rad;
                    heading = normalize(rotate_around(heading, side, noise));
                }
                // Tropism as an accumulated bend: phototropism pulls tips
                // toward +Y each step, gravity (negative) droops them toward
                // -Y.  A vertical trunk has no lever arm and stays straight —
                // only leaning shoots respond, which is how real trees behave.
                if params.tropism.up.abs() > 1e-4 {
                    let target: [f32; 3] = if params.tropism.up > 0.0 {
                        [0.0, 1.0, 0.0]
                    } else {
                        [0.0, -1.0, 0.0]
                    };
                    let axis = cross(heading, target);
                    let axis_length =
                        (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
                    if axis_length > 1e-5 {
                        let bend = (params.tropism.up.abs() * 0.5 * step.min(1.5)).min(0.6);
                        let k = [
                            axis[0] / axis_length,
                            axis[1] / axis_length,
                            axis[2] / axis_length,
                        ];
                        heading = normalize(rotate_around(heading, k, bend));
                    }
                }
                let end = [
                    position[0] + heading[0] * step,
                    position[1] + heading[1] * step,
                    position[2] + heading[2] * step,
                ];
                let end_radius = (radius * params.radius_decay).max(0.012);
                segments.push(Segment {
                    start: position,
                    end,
                    radius_start: radius,
                    radius_end: end_radius,
                });
                position = end;
                radius = end_radius;
                step *= params.step_decay;
            }
            '+' => turn(0, 1.0, &mut heading, &mut side),
            '-' => turn(0, -1.0, &mut heading, &mut side),
            '&' => turn(1, 1.0, &mut heading, &mut side),
            '^' => turn(1, -1.0, &mut heading, &mut side),
            '/' => turn(2, 1.0, &mut heading, &mut side),
            '\\' => turn(2, -1.0, &mut heading, &mut side),
            '[' => {
                stack.push((position, heading, side, radius, step));
                radius *= params.radius_decay;
                step *= params.step_decay;
            }
            ']' => {
                if let Some((p, h, s, r, st)) = stack.pop() {
                    position = p;
                    heading = h;
                    side = s;
                    radius = r;
                    step = st;
                }
            }
            'L' => {
                foliage.push(FoliageBlob {
                    centre: position,
                    radius: params.foliage_radius * (0.8 + rng.next_f64() as f32 * 0.4),
                    density: 0.55 + rng.next_f64() as f32 * 0.45,
                });
            }
            _ => {}
        }
    }
    (segments, foliage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expansion_is_pure_and_deterministic() {
        let system = LSystem::new("A").with_rule('A', "F[+FA]F[-FA]");
        let a = system.expand(3);
        let b = system.expand(3);
        assert_eq!(a, b);
        assert!(a.contains('F'));
    }

    #[test]
    fn turtle_yields_segments_and_respects_branches() {
        let params = TurtleParams::default();
        let mut rng = SplitMix64::new(7);
        let (segments, _foliage) = interpret("FF[-F]F[+F]L", &params, 0.0, &mut rng);
        assert_eq!(segments.len(), 5);
        // The trunk steps upward at roughly the step length per F.
        let last = segments.last().unwrap();
        assert!(last.end[1] > last.start[1]);
        assert!(last.radius_end < last.radius_start);
    }

    #[test]
    fn tropism_can_droop_willow_shoots() {
        let mut droop = TurtleParams::default();
        droop.tropism = Tropism { up: -0.9 };
        let mut rng = SplitMix64::new(11);
        // A pitched branch accumulates droop step by step and ends lower than
        // it started; a perfectly vertical trunk stays straight.
        let (segments, _) = interpret("F[&FFFFFF]", &droop, 0.0, &mut rng);
        let last = segments.last().unwrap();
        assert!(
            last.end[1] < last.start[1],
            "strong negative tropism droops"
        );
        let (upright, _) = interpret("FFFFFF", &droop, 0.0, &mut rng);
        let trunk_last = upright.last().unwrap();
        assert!(trunk_last.end[1] > trunk_last.start[1]);
    }
}
