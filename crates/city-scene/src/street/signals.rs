//! Signal heads on the approaches of signalised junctions, and the rig
//! records the renderer's phase driver recolours.
//!
//! # Mounting heights are a standard, not a taste
//!
//! GB 14886 puts the housing of a cantilever- or pole-mounted motor-vehicle
//! signal head between **5.2 m and 6.5 m above the carriageway, measured to the
//! bottom of the housing**: low enough that the lenses sit inside a queueing
//! driver's comfortable cone of view, high enough that a truck passes under
//! them.  The previous rig hung its housing at a 5.02 m bottom with a pole that
//! stopped below its own bracket arm — both out of spec — so the head read as
//! pedestrian furniture.  [`build_signals`] holds the bottom at 5.37 m and the
//! pole clears the bracket, and
//! `a_signal_housing_hangs_inside_the_gb_14886_window` locks it.

use crate::math::{Rng, Vec2, Vec3};
use crate::mesh::{MeshBuilder, box_at};
use crate::network::Network;
use urban::{JunctionKind, ModernRoadClass};

/// A signal lamp that the renderer recolours as the phase advances.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalLamp {
    /// Which lamp of the head, 0 = red, 1 = yellow, 2 = green.
    pub aspect: u8,
    pub position: [f32; 3],
}

/// A signal rig the renderer can drive: which junction, which road, which phase
/// programme, and where every lens is.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalRig {
    pub id: String,
    pub junction: u32,
    pub road: u32,
    /// `"ns"` or `"ew"`: which phase programme this approach runs on.
    pub axis: &'static str,
    pub lamps: Vec<SignalLamp>,
}

/// The housing's vertical centre, chosen so its 1.16 m tall box bottoms out at
/// 5.37 m — inside the GB 14886 5.2-6.5 m window.
const HEAD_CENTRE_Y: f32 = 5.95;
/// Housing height.  Bottom = `HEAD_CENTRE_Y - 0.58`.
const HEAD_HEIGHT: f32 = 1.16;
/// Pole height: clears the bracket arm at `HEAD_CENTRE_Y + 0.66` with room to
/// spare, so the arm hangs off the pole instead of past its top.
const POLE_HEIGHT: f32 = 6.8;

/// Which signal axis an approach belongs to.
///
/// Two perpendicular diagonal approaches must land on *different* axes, or they
/// share a green and collide inside the box.  A 45-degree tie is broken by the
/// sign of the bearing product rather than by a threshold, which is what the
/// source kernel does and what makes the tie total.
pub fn signal_axis(bearing: Vec2) -> &'static str {
    if bearing.x.abs() > bearing.y.abs() {
        "ew"
    } else if bearing.y.abs() > bearing.x.abs() {
        "ns"
    } else if bearing.x * bearing.y >= 0.0 {
        "ew"
    } else {
        "ns"
    }
}

/// Signal heads, on the near-side footway of every signalised approach.
///
/// The previous version planted every head at `junction.centre + side + along`,
/// which put the pole **inside the junction box**, six metres past the kerb,
/// facing across the carriageway.  A signal head has one job: be in the driver's
/// field of view *before* the stop line and face back down the approach at it.
/// So the head is placed on the road's own geometry, at the station of the stop
/// line plus a metre, laterally out on the footway, and its housing is built
/// along the road axis with the lens face pointing back at the queue.
pub(super) fn build_signals(network: &Network, builder: &mut MeshBuilder) -> Vec<SignalRig> {
    let spec = &network.spec;
    let mut rigs = Vec::new();
    let mut rng = Rng::new(0);
    for junction in &network.junctions {
        if junction.kind != JunctionKind::Signalized || junction.ports.len() < 3 {
            continue;
        }
        let has_arterial = junction.ports.iter().any(|port| {
            network.road(port.road).is_some_and(|road| {
                matches!(
                    road.class,
                    ModernRoadClass::Arterial | ModernRoadClass::Expressway
                )
            })
        });
        if !has_arterial {
            continue;
        }
        // Signalise the junctions that carry the traffic, not every corner.
        rng.fork(junction.node);
        if junction.ports.len() >= 4 && rng.chance(0.35) {
            continue;
        }
        for port in &junction.ports {
            let Some(road) = network.road(port.road) else {
                continue;
            };
            if !road.has_sidewalk() {
                continue;
            }
            let path = &road.carriageway;
            let length = path.length();
            if length < 8.0 {
                continue;
            }
            let (edge, into) = if port.at_start {
                (0.0_f32, 1.0_f32)
            } else {
                (length, -1.0_f32)
            };
            // The stop line is `stop_line_gap` in from the edge; the pole stands a
            // metre further back, on the footway, on the approach's own side.
            let station = (edge + into * (spec.stop_line_gap + 1.3)).clamp(0.5, length - 0.5);
            let side = if port.at_start { -1.0_f32 } else { 1.0_f32 };
            let lateral = side * (road.half_width() + road.section.sidewalk_metres * 0.35 + 0.6);
            let base = path.offset_at(station, lateral, 0.0);
            let tangent = path.tangent_at(station);
            // The head faces back down the approach, at the queue.
            let facing = tangent * -into;
            let axis = signal_axis(facing);
            let foot = Vec3::new(base.x, super::WALK_Y, base.z);
            builder.tube(
                "signal.body",
                foot,
                foot + Vec3::new(0.0, POLE_HEIGHT, 0.0),
                0.12,
                0.09,
                8,
                None,
            );
            // A short bracket arm out over the kerb line, which is what actually
            // puts the lens in a driver's eye rather than behind the column.
            // It leaves the pole below its attach point and the housing hangs
            // beneath it.
            let arm_plan = Vec2::new(base.x, base.z) + tangent.left_normal() * (-side * 0.9);
            let arm_tip = Vec3::new(arm_plan.x, HEAD_CENTRE_Y, arm_plan.y);
            builder.tube(
                "signal.body",
                foot + Vec3::new(0.0, HEAD_CENTRE_Y + 0.66, 0.0),
                arm_tip + Vec3::new(0.0, 0.66, 0.0),
                0.06,
                0.05,
                5,
                None,
            );
            // The housing: 340 mm along the road, 440 mm across it, 1.16 m tall,
            // wound so the lens face looks back at the stop line.
            let plan = Vec2::new(arm_tip.x, arm_tip.z);
            box_at(
                builder,
                "signal.body",
                plan,
                HEAD_CENTRE_Y,
                0.34,
                HEAD_HEIGHT,
                0.44,
                facing.angle(),
            );
            let right3 = Vec2::new(-facing.y, facing.x);
            let mut lamps = Vec::with_capacity(3);
            for (aspect, dy) in [(0_u8, 0.38_f32), (1, 0.0), (2, -0.38)] {
                let lens = plan - facing * 0.19;
                let centre = Vec3::new(lens.x, HEAD_CENTRE_Y + dy, lens.y);
                // A bezel ring around each lens, so the head reads as hardware.
                builder.tube(
                    "signal.body",
                    Vec3::new(
                        centre.x + right3.x * 0.02,
                        centre.y,
                        centre.z + right3.y * 0.02,
                    ),
                    Vec3::new(
                        centre.x - right3.x * 0.05,
                        centre.y,
                        centre.z - right3.y * 0.05,
                    ),
                    0.15,
                    0.13,
                    8,
                    None,
                );
                // The visor over the top lens, which is what shades it from the
                // low sun that is otherwise straight down the approach.
                if aspect == 0 {
                    builder.wall(
                        "signal.body",
                        Vec2::new(lens.x, lens.y),
                        Vec2::new(lens.x - facing.x * 0.26, lens.y - facing.y * 0.26),
                        centre.y + 0.13,
                        centre.y + 0.18,
                        None,
                    );
                }
                lamps.push(SignalLamp {
                    aspect,
                    position: [centre.x, centre.y, centre.z],
                });
            }
            rigs.push(SignalRig {
                id: format!("signal/{}/{}", junction.node, port.road),
                junction: junction.node,
                road: port.road,
                axis,
                lamps,
            });
        }
    }
    rigs
}
