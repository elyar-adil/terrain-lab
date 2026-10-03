//! Street furniture, built once as instanced prototypes and placed per city.
//!
//! Ported from the source renderer's furniture pass.  Everything here is a
//! repeated object, so each is authored once at unit height and instanced — a
//! city's lamps, poles, wires, bollards, railings and parked cars cost a
//! handful of draw calls between them.
//!
//! The catenary wires are the one piece that cannot be instanced: each span
//! has its own length and sag, so they are emitted as merged geometry.  They
//! also matter more than their vertex count suggests — overhead wiring is what
//! makes an arterial read as Chinese rather than as a generic boulevard.

use urban::ModernRoadClass;

use crate::math::{Path, Rng, Vec2, Vec3};
use crate::mesh::{GroupStyle, Instance, MeshBuilder, box_at};
use crate::network::{Junction, Network};
use crate::street::level;

/// Every furniture prototype key, and the budget for each per city.
pub const CATENARY_BUDGET: usize = 620;

fn declare(builder: &mut MeshBuilder) {
    for material in [
        "steel",
        "pole.concrete",
        "pole.wood",
        "wire",
        "sign.blue",
        "sign.crossing",
        "barrier.concrete",
        "rail.steel",
        "hedge",
    ] {
        builder.style(
            material,
            GroupStyle {
                cast_shadow: true,
                receive_shadow: true,
                alpha_cutout: false,
                dynamic: false,
            },
        );
    }
    for material in ["sign.blue", "sign.crossing"] {
        builder.style(
            material,
            GroupStyle {
                cast_shadow: false,
                receive_shadow: true,
                alpha_cutout: false,
                dynamic: false,
            },
        );
    }
}

/// Yaw that makes a prototype's **local `+X` axis** point along `direction`.
///
/// The renderer writes `dummy.rotation.set(0, yaw, 0)` and three.js maps a local
/// `+X` axis to `(cos yaw, 0, -sin yaw)` in the `(x, z)` ground plane — so the
/// yaw is `atan2(-z, x)`, *not* the plan bearing `atan2(z, x)`.  The two differ
/// by the sign of the `z` component, which puts a car on a north-south street
/// facing backwards and a railing on a diagonal at twice the error.  Every
/// placement in this file goes through one of the two helpers below so the
/// convention is stated once.
pub fn yaw_along_x(direction: Vec2) -> f32 {
    (-direction.y).atan2(direction.x)
}

/// Yaw that makes a prototype's **local `+Z` axis** point along `direction`.
///
/// three.js maps a local `+Z` axis to `(sin yaw, 0, cos yaw)`, so the yaw is
/// `atan2(x, z)`.  A road lamp is authored this way — its bracket arm reaches
/// out along `+Z` over the carriageway.
pub fn yaw_along_z(direction: Vec2) -> f32 {
    direction.x.atan2(direction.y)
}

/// A Chinese road lamp, at true metric size.
///
/// # Not a European swan neck
///
/// The bracket is a **short, straight, near-horizontal arm** — 2.4 m of reach at
/// about 9 m of height — and the luminaire is a flat box slung under its tip.
/// The graceful upswept swan neck is not what stands over a Chinese arterial, and
/// the difference is visible in silhouette from 200 m away.
///
/// # Authored at true size, not unit height
///
/// The crate's other prototypes are authored at unit height and scaled by the
/// instance matrix, which stretches *everything* vertically: a unit-height
/// luminaire 0.08 m tall becomes 0.7 m of luminaire on a 9 m pole.  A lamp is
/// the one object where that is not acceptable, because the arm and the luminaire
/// are read as a shape rather than as a height.  So this prototype is at true
/// metres and is placed with a scale of 1.0, which also means its silhouette is
/// identical on every road class.
fn street_lamp(builder: &mut MeshBuilder, key: &str) {
    declare(builder);
    let pole_height = 9.0_f32;
    let reach = 2.4_f32;
    // A tapered column: 240 mm at the base, 120 mm at the head.
    builder.tube(
        key,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, pole_height, 0.0),
        0.12,
        0.06,
        8,
        None,
    );
    // A short, straight, near-horizontal bracket.  It rises 350 mm over its
    // reach — a lamp bracket, not a curve.
    let shoulder = Vec3::new(0.0, pole_height - 0.35, 0.0);
    let tip = Vec3::new(0.0, pole_height, reach);
    builder.tube(key, shoulder, tip, 0.055, 0.045, 6, None);
    // The luminaire: a shallow box slung under the tip, long axis across the arm.
    box_at(
        builder,
        key,
        Vec2::new(0.0, reach - 0.2),
        pole_height - 0.16,
        0.34,
        0.13,
        0.82,
        0.0,
    );
    // A maintenance hatch at two metres, which is the detail that makes a pole
    // read as a pole rather than as a line.
    box_at(
        builder,
        key,
        Vec2::new(0.0, 0.0),
        2.0,
        0.26,
        0.42,
        0.22,
        0.0,
    );
}

/// A utility pole with two cross-arms and a transformer, at unit height.
fn utility_pole(builder: &mut MeshBuilder, key: &str) {
    declare(builder);
    builder.tube(
        key,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.11,
        0.08,
        6,
        None,
    );
    for (height, span) in [(0.90_f32, 0.19_f32), (0.80, 0.15)] {
        builder.quad(
            key,
            Vec3::new(-span, height, 0.0),
            Vec3::new(span, height, 0.0),
            Vec3::new(span, height + 0.012, 0.0),
            Vec3::new(-span, height + 0.012, 0.0),
            None,
        );
    }
}

/// A kerbside bollard: the thing that stops a pedestrian mount the carriageway
/// at a junction mouth, and that the source renderer placed on the walk ring.
///
/// The two raised bands are retroreflective tape at the heights a driver's eye
/// actually sweeps.  They are a separate ring of slightly larger radius so they
/// catch a headlight as a bright dot rather than merging into the post, and they
/// are the difference between a kerb line that reads at night and one that
/// does not.
fn bollard(builder: &mut MeshBuilder, key: &str) {
    declare(builder);
    builder.tube(
        key,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.78, 0.0),
        0.085,
        0.078,
        8,
        None,
    );
    for height in [0.60_f32, 0.70] {
        builder.tube(
            key,
            Vec3::new(0.0, height, 0.0),
            Vec3::new(0.0, height + 0.055, 0.0),
            0.092,
            0.092,
            8,
            None,
        );
    }
    builder.tube(
        key,
        Vec3::new(0.0, 0.78, 0.0),
        Vec3::new(0.0, 0.83, 0.0),
        0.088,
        0.088,
        8,
        None,
    );
}

/// A stainless-steel rubbish bin: a drum with a swing lid and a liner ring.
///
/// Bins are the single most numerous piece of street furniture in a Chinese
/// district and the previous layer had none, which is a large part of why its
/// footways read as empty corridors rather than as lived-in streets.  They come
/// in pairs on most footways, which is what the placement does.
fn rubbish_bin(builder: &mut MeshBuilder, key: &str) {
    declare(builder);
    // The drum, 700 mm across and 900 mm tall, with a slight taper.
    builder.tube(
        key,
        Vec3::new(0.0, 0.06, 0.0),
        Vec3::new(0.0, 0.88, 0.0),
        0.30,
        0.35,
        12,
        None,
    );
    // A foot ring and a rolled rim, so it is a bin and not a cylinder.
    builder.tube(
        key,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.07, 0.0),
        0.26,
        0.26,
        10,
        None,
    );
    // The lid, tilted back on its hinge — the pose every street bin is in.
    let lid = box_at(
        builder,
        key,
        Vec2::new(0.0, -0.10),
        0.95,
        0.76,
        0.05,
        0.34,
        0.0,
    );
    let _ = lid;
    // A liner bag collar showing above the rim.
    builder.tube(
        key,
        Vec3::new(0.0, 0.86, 0.0),
        Vec3::new(0.0, 0.90, 0.0),
        0.30,
        0.26,
        12,
        None,
    );
}

/// A public telephone kiosk: the box-with-a-canopy form that lines a Chinese
/// footway every 200 m or so, and one of the strongest "this is China, not
/// anywhere else" cues in the whole scene at street level.
///
/// It is a separate prototype rather than a scaled bus shelter because the
/// silhouettes are opposite — the kiosk is tall, narrow and opaque, the shelter is
/// low, wide and glazed — and a scaled shelter reads as neither.
fn phone_kiosk(builder: &mut MeshBuilder, key: &str) {
    declare(builder);
    // A 1.1 x 0.9 m cabin, closed on three sides, with a glazed front.
    box_at(
        builder,
        key,
        Vec2::new(0.0, 0.0),
        1.10,
        1.10,
        2.20,
        0.90,
        0.0,
    );
    // The canopy: a slab that oversails on all four sides, which is the whole
    // silhouette.
    box_at(
        builder,
        key,
        Vec2::new(0.0, 0.0),
        2.28,
        1.44,
        0.10,
        1.24,
        0.0,
    );
    // A fascia under the canopy, and the light box the sign would be on.
    box_at(
        builder,
        key,
        Vec2::new(0.0, 0.60),
        2.16,
        1.20,
        0.16,
        0.06,
        0.0,
    );
    // Four feet, so it stands on the footway rather than being let into it.
    for (x, z) in [(0.44, 0.34), (-0.44, 0.34), (0.44, -0.34), (-0.44, -0.34)] {
        builder.tube(
            key,
            Vec3::new(x, 0.0, z),
            Vec3::new(x, 0.14, z),
            0.06,
            0.06,
            5,
            None,
        );
    }
}

/// A three-metre pedestrian-railing segment: two rails and a post.
fn railing_segment(builder: &mut MeshBuilder, key: &str) {
    declare(builder);
    for height in [0.52_f32, 1.02] {
        builder.quad(
            key,
            Vec3::new(-1.5, height, 0.0),
            Vec3::new(1.5, height, 0.0),
            Vec3::new(1.5, height + 0.045, 0.0),
            Vec3::new(-1.5, height + 0.045, 0.0),
            None,
        );
    }
    builder.tube(
        key,
        Vec3::new(-1.5, 0.0, 0.0),
        Vec3::new(-1.5, 1.06, 0.0),
        0.022,
        0.022,
        5,
        None,
    );
}

/// A blue guide sign on two posts, facing along the street.
fn guide_sign(builder: &mut MeshBuilder, key: &str) {
    declare(builder);
    for post in [-1.4_f32, 1.4] {
        builder.tube(
            key,
            Vec3::new(post, 0.0, 0.0),
            Vec3::new(post, 3.4, 0.0),
            0.055,
            0.045,
            5,
            None,
        );
    }
    builder.quad(
        key,
        Vec3::new(-1.55, 2.4, 0.03),
        Vec3::new(1.55, 2.4, 0.03),
        Vec3::new(1.55, 3.4, 0.03),
        Vec3::new(-1.55, 3.4, 0.03),
        None,
    );
}

/// A pedestrian-crossing warning sign.
fn crossing_sign(builder: &mut MeshBuilder, key: &str) {
    declare(builder);
    builder.tube(
        key,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 2.4, 0.0),
        0.05,
        0.04,
        5,
        None,
    );
    builder.quad(
        key,
        Vec3::new(-0.42, 1.9, 0.03),
        Vec3::new(0.42, 1.9, 0.03),
        Vec3::new(0.42, 2.5, 0.03),
        Vec3::new(-0.42, 2.5, 0.03),
        None,
    );
}

/// An overhead gantry: the blue directional boards a Chinese arterial hangs
/// across the carriageway on a steel truss before every signalised junction.
///
/// It is one of the strongest "this is a Chinese arterial" signals in a street
/// photograph, which is why it earns two prototypes: the truss in galvanised
/// steel and the sign boards in the same blue as the post-mounted guide signs.
/// They are placed at the same transform, so together they read as one
/// structure while keeping one material per instance key.
///
/// Authored at the arterial's true geometry (kerb face at 15.5 m from the
/// centre, posts 1 m up the footway at ±16.5 m), with the truss at 6.6–7.45 m
/// — above the 5.5 m clearance any road vehicle needs, low enough that the
/// boards at 5.15–6.45 m read against the skyline rather than against the road.
fn gantry_steel(builder: &mut MeshBuilder, key: &str) {
    declare(builder);
    let span = 16.5_f32;
    for side in [-1.0_f32, 1.0] {
        builder.tube(
            key,
            Vec3::new(side * span, 0.0, 0.0),
            Vec3::new(side * span, 7.45, 0.0),
            0.14,
            0.10,
            7,
            None,
        );
    }
    for y in [6.6_f32, 7.45] {
        builder.tube(
            key,
            Vec3::new(-span, y, 0.0),
            Vec3::new(span, y, 0.0),
            0.07,
            0.07,
            2,
            None,
        );
    }
    let mut x = -span;
    while x <= span + 0.01 {
        builder.tube(
            key,
            Vec3::new(x, 6.6, 0.0),
            Vec3::new(x, 7.45, 0.0),
            0.05,
            0.05,
            4,
            None,
        );
        x += 33.0 / 14.0;
    }
    // The boards' backs: plain aluminium to the reverse direction, so the
    // gantry reads from both approaches instead of vanishing edge-on. One
    // board over each direction's motor lanes, whose centres sit at ±6.5 m.
    for centre in [-6.5_f32, 6.5] {
        builder.quad(
            key,
            Vec3::new(centre - 1.75, 5.15, -0.02),
            Vec3::new(centre + 1.75, 5.15, -0.02),
            Vec3::new(centre + 1.75, 6.45, -0.02),
            Vec3::new(centre - 1.75, 6.45, -0.02),
            None,
        );
    }
}

/// The gantry's facing boards, in guide-sign blue. Same transform as
/// [`gantry_steel`]; see that prototype for the structure. The boards hang
/// over the two motor carriageways, centred at ±6.5 m.
fn gantry_boards(builder: &mut MeshBuilder, key: &str) {
    declare(builder);
    for centre in [-6.5_f32, 6.5] {
        builder.quad(
            key,
            Vec3::new(centre - 1.7, 5.15, 0.03),
            Vec3::new(centre + 1.7, 5.15, 0.03),
            Vec3::new(centre + 1.7, 6.45, 0.03),
            Vec3::new(centre - 1.7, 6.45, 0.03),
            None,
        );
    }
}

/// A bus shelter: platform, canopy, two posts and an ad panel.
fn bus_shelter(builder: &mut MeshBuilder, key: &str) {
    declare(builder);
    box_at(builder, key, Vec2::new(0.0, 0.0), 0.07, 6.2, 0.14, 2.4, 0.0);
    box_at(
        builder,
        key,
        Vec2::new(0.0, -0.3),
        2.35,
        4.6,
        0.12,
        1.7,
        0.0,
    );
    for post in [-2.2_f32, 2.2] {
        builder.tube(
            key,
            Vec3::new(post, 0.0, 0.7),
            Vec3::new(post, 2.3, 0.7),
            0.06,
            0.06,
            5,
            None,
        );
    }
    box_at(
        builder,
        key,
        Vec2::new(-2.9, -1.0),
        1.2,
        0.12,
        1.9,
        1.1,
        0.0,
    );
}

/// A parked car, authored facing `+X` at unit length, used for both the parked
/// fleet and the traffic simulation.
fn car_body(builder: &mut MeshBuilder, key: &str) {
    declare(builder);
    // Skirt, body and cabin as three stacked boxes: three draw-call-cheap shapes
    // that read as a car at street distance and cost 18 quads between them.
    box_at(
        builder,
        key,
        Vec2::new(0.0, 0.0),
        0.18,
        1.86,
        0.36,
        4.36,
        0.0,
    );
    box_at(
        builder,
        key,
        Vec2::new(0.0, 0.0),
        0.52,
        1.78,
        0.44,
        4.10,
        0.0,
    );
    box_at(
        builder,
        key,
        Vec2::new(-0.10, 0.0),
        0.86,
        1.58,
        0.34,
        2.10,
        0.0,
    );
}

fn car_glass(builder: &mut MeshBuilder, key: &str) {
    declare(builder);
    box_at(
        builder,
        key,
        Vec2::new(-0.10, 0.0),
        0.88,
        1.60,
        0.30,
        2.00,
        0.0,
    );
}

fn car_wheel(builder: &mut MeshBuilder, key: &str) {
    declare(builder);
    for (x, y) in [
        (1.36_f32, 0.82_f32),
        (1.36, -0.82),
        (-1.36, 0.82),
        (-1.36, -0.82),
    ] {
        builder.tube(
            key,
            Vec3::new(x - 0.06, y, 0.16),
            Vec3::new(x + 0.06, y, 0.16),
            0.16,
            0.16,
            7,
            None,
        );
    }
}

/// Every prototype, built once per generation.
pub fn build_prototypes(builder: &mut MeshBuilder) {
    street_lamp(builder, "furniture/lamp");
    utility_pole(builder, "furniture/pole");
    bollard(builder, "furniture/bollard");
    railing_segment(builder, "furniture/railing");
    guide_sign(builder, "furniture/sign.guide");
    crossing_sign(builder, "furniture/sign.crossing");
    gantry_steel(builder, "furniture/gantry.steel");
    gantry_boards(builder, "furniture/gantry.board");
    bus_shelter(builder, "furniture/shelter");
    rubbish_bin(builder, "furniture/bin");
    phone_kiosk(builder, "furniture/kiosk");
    car_body(builder, "car/body");
    car_glass(builder, "car/glass");
    car_wheel(builder, "car/wheel");
    for key in [
        "furniture/lamp",
        "furniture/pole",
        "furniture/bollard",
        "furniture/railing",
        "furniture/sign.guide",
        "furniture/sign.crossing",
        "furniture/shelter",
        "furniture/bin",
        "furniture/kiosk",
        "car/body",
        "car/glass",
        "car/wheel",
    ] {
        builder.bind(key, key);
    }
    // The gantry gets its own lists.  It cannot share the railings' or the guide
    // signs': `bind` only says *which instance list* a group draws with, it does
    // not change the group's own `material` key, so sharing would have left every
    // gantry transform in a list nothing reads — and merging the geometry into
    // those groups instead would draw a truss at every railing.  Two lists and
    // two new material keys is the honest cost of one object; the renderer
    // bindings are requested in the report.
    builder.bind("furniture/gantry.steel", "furniture/gantry.steel");
    builder.bind("furniture/gantry.board", "furniture/gantry.board");
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FurnitureOutput {
    pub lamps: usize,
    pub poles: usize,
    pub bollards: usize,
    pub railings: usize,
    pub signs: usize,
    /// Everything that *encloses* something on the footway: bus shelters, public
    /// telephone kiosks and rubbish-bin pairs.  They share one field because they
    /// share one job — density of small structures along a footway — and because
    /// the payload schema is shared with the renderer, so the field list is fixed.
    pub shelters: usize,
    pub spans: usize,
}

/// Place every piece of furniture a city should have.
pub fn place(network: &Network, builder: &mut MeshBuilder, seed: u32) -> FurnitureOutput {
    declare(builder);
    let mut rng = Rng::new(seed ^ 0xfa12);
    let mut output = FurnitureOutput::default();
    let mut spans = 0;
    // The last pole wired, as an actual world position and its cross-arm
    // heading. It used to be a `(road, station)` pair, which was the whole bug:
    // the station could only be re-evaluated against the *current* road's path and
    // the *current* side, so a wire whose two poles were on opposite sides of the
    // carriageway was drawn from a point where no pole stood. Carrying the real
    // endpoint makes that class of mistake unrepresentable rather than merely
    // unlikely.
    let mut previous_pole: Option<PoleTop> = None;

    for road in &network.roads {
        if road.layer != 0 || road.is_motorway() {
            continue;
        }
        let path = road.carriageway.clone();
        let length = path.length();
        if length < 20.0 {
            continue;
        }
        let section = road.section;
        let half = section.half_width();
        // `half_width()` is the ribbon edge (the property line), and the
        // sidewalk is *inside* it — so the kerb face sits at `half - sidewalk`.
        // Every roadside object anchors to the kerb, not to the ribbon edge:
        // anchoring to `half` pushed a whole generation of furniture metres
        // into the buildings.
        let sidewalk = section.sidewalk_metres.max(2.0);
        let kerb = half - sidewalk;
        let arterial = matches!(
            road.class,
            ModernRoadClass::Arterial | ModernRoadClass::Collector
        );

        // Lamps and poles alternate down one side, as they are actually built.
        if arterial {
            // Lamps go down **both** sides, alternating, because that is how a
            // Chinese arterial is lit and a one-sided lamp line reads as an
            // unfinished street. Poles do not alternate: a pole line is a
            // straight run in a line, and alternation is what produced wires
            // crossing the carriageway diagonally.
            let mut lamp_side = 1.0_f32;
            let mut station = 16.0;
            while station < length - 16.0 {
                let offset = lamp_side * (kerb + 0.55);
                let point = path.offset_at(station, offset, 0.0);
                // The bracket reaches *over the carriageway*, so the prototype's
                // local `+Z` axis is pointed back at the road.  The prototype is
                // at true metres, so the scale is 1.
                let toward_road = path.tangent_at(station).left_normal() * -lamp_side;
                builder.add_instance(
                    "furniture/lamp",
                    Instance::new(
                        point.x,
                        level::KERB,
                        point.z,
                        yaw_along_z(toward_road),
                        1.0,
                        [1.0, 1.0, 1.0],
                    ),
                );
                output.lamps += 1;
                lamp_side = -lamp_side;
                station += 34.0;
            }

            // One pole line, on the right-hand kerb as the road is travelled.
            // Real pole lines do not alternate sides: they run in a straight
            // line, and the conductors hang between consecutive poles *in that
            // line*. Everything about the wires follows from this.
            let pole_side = 1.0_f32;
            let pole_offset = pole_side * (kerb + 1.9);
            let pole_height = 9.0_f32;
            let mut pole_station = 20.0_f32;
            while pole_station < length - 14.0 {
                let base = path.offset_at(pole_station, pole_offset, 0.0);
                // The cross-arms run perpendicular to the line of poles, so the
                // conductors pass over the footway rather than over the road. The
                // prototype authors its arms along local +X, and a rotation of
                // `theta` about Y maps +X to `(cos t, 0, -sin t)`, so aligning
                // with the road's lateral `(t.y, -t.x)` gives `atan2(t.x, t.y)`.
                //
                // The code said that and then computed `atan2(t.y, t.x)`, which
                // maps +X to `(t.x, 0, -t.y)` — the *tangent*. So the arms were
                // lying along the road with the conductors strung on the pole
                // axis line, and `PoleTop::conductor` then displaced each wire
                // lengthwise by its own arm half-span, so successive spans were
                // offset from each other by a metre. Both the comment and the
                // code have to agree or the conductors do not meet the arms.
                let tangent = path.tangent_at(pole_station);
                let rotation = tangent.x.atan2(tangent.y);
                builder.add_instance(
                    "furniture/pole",
                    Instance::new(
                        base.x,
                        level::KERB,
                        base.z,
                        rotation,
                        pole_height,
                        [1.0, 1.0, 1.0],
                    ),
                );
                output.poles += 1;

                let top = PoleTop {
                    road: road.id,
                    base: Vec3::new(base.x, level::KERB, base.z),
                    // The arms are at prototype `y` 0.90 and 0.80 of a pole whose
                    // height *is* the instance scale, so the conductor hang points
                    // scale with the pole. Hard-coded offsets were the fourth
                    // defect: the wires floated about a metre inboard of the arm
                    // tips and touched nothing.
                    arm_top: 0.90 * pole_height,
                    arm_low: 0.80 * pole_height,
                    arm_span_top: 0.19 * pole_height,
                    arm_span_low: 0.15 * pole_height,
                    rotation,
                };
                if spans < CATENARY_BUDGET
                    && let Some(previous) = previous_pole
                    && previous.road == road.id
                {
                    spans += catenary(builder, &previous, &top, arterial);
                }
                previous_pole = Some(top);
                pole_station += 34.0;
            }
            // Pedestrian railings on an arterial's sidewalk, and the median
            // guardrail that goes with them.
            //
            // The median rail is the same three-metre segment at a different
            // height and tint, which is exactly what it is on a real road: a
            // pedestrian railing and a median barrier are the same welded steel
            // section.  It matters because a planted median with nothing on it
            // reads as a strip of grass, and a median with a barrier on it reads
            // as a divided carriageway — which is what stops a crossing driver
            // from trying to cross six lanes.
            if road.class == ModernRoadClass::Arterial {
                for side in [-1.0_f32, 1.0] {
                    let offset = side * (kerb + 0.35);
                    let mut station = 8.0;
                    while station < length - 8.0 {
                        let point = path.offset_at(station, offset, 0.0);
                        builder.add_instance(
                            "furniture/railing",
                            Instance::new(
                                point.x,
                                level::KERB,
                                point.z,
                                yaw_along_x(path.tangent_at(station)),
                                1.0,
                                [1.0, 1.0, 1.0],
                            ),
                        );
                        output.railings += 1;
                        station += 3.0;
                    }
                }
            }
            if section.has_median() {
                let offset = section.median_metres * 0.5 + 0.24;
                let mut station = 6.0;
                while station < length - 6.0 {
                    let point = path.offset_at(station, offset, 0.0);
                    builder.add_instance(
                        "furniture/railing",
                        Instance::new(
                            point.x,
                            level::MEDIAN + 0.02,
                            point.z,
                            yaw_along_x(path.tangent_at(station)),
                            0.82,
                            // Galvanised, and a touch warmer than the footway
                            // railings so the two lines are distinguishable.
                            [1.04, 1.02, 0.96],
                        ),
                    );
                    output.railings += 1;
                    station += 3.0;
                }
            }
            // A guide sign and a crossing sign on the approach to each junction.
            for at_start in [true, false] {
                if at_start && !road.crossing_start {
                    continue;
                }
                if !at_start && !road.crossing_end {
                    continue;
                }
                let base = if at_start { 0.0 } else { length };
                let toward = if at_start { 1.0_f32 } else { -1.0 };
                // The gantry goes in first, 34 m upstream of the junction —
                // past the 27 m guide signs, inside the 58 m approach taper,
                // where a driver has already chosen a lane. Arterials only:
                // collector junctions carry post-mounted signs, and the
                // prototype's span is authored for the arterial cross-section.
                if road.class == ModernRoadClass::Arterial && length > 78.0 {
                    let station = (base + toward * 34.0).clamp(8.0, length - 8.0);
                    let point = path.offset_at(station, 0.0, 0.0);
                    // The boards face the approaching driver, like the guide
                    // signs: local `+Z` along the opposing tangent.
                    let yaw = yaw_along_z(-path.tangent_at(base) * toward);
                    for key in ["furniture/gantry.steel", "furniture/gantry.board"] {
                        builder.add_instance(
                            key,
                            Instance::new(point.x, level::KERB, point.z, yaw, 1.0, [1.0, 1.0, 1.0]),
                        );
                    }
                    output.signs += 2;
                }
                for side in [-1.0_f32, 1.0] {
                    let guide = path.offset_at(
                        (base + toward * 27.0 + rng.unit() * 7.0).clamp(2.0, length - 2.0),
                        side * (kerb + 0.55),
                        0.0,
                    );
                    let facing = -path.tangent_at(base) * toward;
                    // The sign board is authored in the prototype's XY plane, so
                    // its local `+Z` faces the driver.
                    builder.add_instance(
                        "furniture/sign.guide",
                        Instance::new(
                            guide.x,
                            level::KERB,
                            guide.z,
                            yaw_along_z(facing),
                            1.0,
                            [1.0, 1.0, 1.0],
                        ),
                    );
                    output.signs += 1;
                    if rng.chance(0.72) {
                        let crossing = path.offset_at(
                            (base + toward * 10.5).clamp(2.0, length - 2.0),
                            side * (kerb + 0.55),
                            0.0,
                        );
                        builder.add_instance(
                            "furniture/sign.crossing",
                            Instance::new(
                                crossing.x,
                                level::KERB,
                                crossing.z,
                                yaw_along_z(facing),
                                1.0,
                                [1.0, 1.0, 1.0],
                            ),
                        );
                        output.signs += 1;
                    }
                }
            }
            // One shelter per arterial leg, halfway between junctions, plus the
            // phone kiosks and the bin pairs that actually make a footway look
            // inhabited.
            //
            // Density matters more than variety here.  A Chinese footway carries
            // a shelter every 300 m or so, a kiosk every 200 m, and a bin pair
            // every 40 m; those are the numbers that produce the visual
            // signature, and a scene with one shelter per block and nothing else
            // reads as a render.
            if road.class == ModernRoadClass::Arterial && length > 120.0 {
                let point = path.offset_at(length * 0.45, kerb + 1.7, 0.0);
                builder.add_instance(
                    "furniture/shelter",
                    Instance::new(
                        point.x,
                        level::KERB,
                        point.z,
                        // The shelter's long axis is its local `X`, so it runs
                        // along the street.
                        yaw_along_x(path.tangent_at(length * 0.45)),
                        1.0,
                        [1.0, 1.0, 1.0],
                    ),
                );
                output.shelters += 1;
            }
            // Bins, in pairs, every 40 m along both footways.  Two, not one,
            // because a single bin is a target for the one person who wants to
            // put something in it, and nobody puts two bins side by side.
            if arterial && section.sidewalk_metres >= 2.0 {
                for side in [-1.0_f32, 1.0] {
                    let offset = side * (kerb + 0.45);
                    let mut bin_station = 22.0_f32;
                    while bin_station < length - 22.0 {
                        for pair in [-0.62_f32, 0.62] {
                            let point = path.offset_at(bin_station + pair, offset, 0.0);
                            let shade = 0.86 + rng.unit() * 0.22;
                            builder.add_instance(
                                "furniture/bin",
                                Instance::new(
                                    point.x,
                                    level::KERB,
                                    point.z,
                                    rng.unit() * std::f32::consts::TAU,
                                    1.0,
                                    [shade, shade * 1.01, shade * 0.97],
                                ),
                            );
                            output.shelters += 1;
                        }
                        bin_station += 40.0;
                    }
                }
            }
            // Phone kiosks, every 200 m, on the kerbside.  Not on a 300 m
            // arterial only — a collector has them too, which is most of what
            // makes a side street read as Chinese.
            if arterial && length > 90.0 {
                for side in [-1.0_f32, 1.0] {
                    let offset = side * (kerb + 0.75);
                    let mut kiosk_station = 30.0_f32;
                    while kiosk_station < length - 30.0 {
                        let point = path.offset_at(kiosk_station, offset, 0.0);
                        builder.add_instance(
                            "furniture/kiosk",
                            Instance::new(
                                point.x,
                                level::KERB,
                                point.z,
                                // The kiosk's glazed front is its local `+Z`.
                                yaw_along_z(path.tangent_at(kiosk_station) * -side),
                                1.0,
                                [0.94 + rng.unit() * 0.1, 0.95, 0.97],
                            ),
                        );
                        output.shelters += 1;
                        kiosk_station += 200.0;
                    }
                }
            }
        }
    }

    // Junction bollards, on the walk ring, skipped at road mouths.
    for junction in &network.junctions {
        junction_bollards(junction, network, builder, &mut rng, &mut output);
    }
    output.spans = spans;
    output
}

/// A pole's conductors, as the world-space points a wire actually attaches to.
///
/// The arms are at fixed fractions of the pole's height, and the pole's height
/// *is* the instance scale, so every one of these numbers has to be derived from
/// the pole rather than written down. An earlier version hard-coded the wire
/// offsets, which put every conductor about a metre inboard of the arm tip: the
/// wires were visibly attached to nothing.
#[derive(Debug, Clone, Copy)]
struct PoleTop {
    road: u32,
    base: Vec3,
    /// Height of the upper cross-arm, metres.
    arm_top: f32,
    /// Height of the lower cross-arm, metres.
    arm_low: f32,
    /// Half-length of the upper cross-arm, metres.
    arm_span_top: f32,
    /// Half-length of the lower cross-arm, metres.
    arm_span_low: f32,
    /// Yaw of the pole, so the arm axis can be recovered.
    rotation: f32,
}

impl PoleTop {
    /// A conductor's hang point on this pole: at an arm tip, dropped by the
    /// insulator. `upper` selects the cross-arm.
    fn conductor(&self, upper: bool, towards: f32) -> Vec3 {
        let (height, span) = if upper {
            (self.arm_top, self.arm_span_top)
        } else {
            (self.arm_low, self.arm_span_low)
        };
        // The prototype's arm lies along local +X, and a yaw of `theta` maps +X
        // to `(cos t, 0, -sin t)`.
        let (sin, cos) = self.rotation.sin_cos();
        let axis = Vec3::new(cos, 0.0, -sin);
        // A conductor hangs *inboard* of the arm tip, not at it, and insulators
        // are short. Pulling it in by a fixed fraction is closer to the truth
        // than attaching at the very tip.
        self.base + axis * (span * 0.82 * towards) + Vec3::new(0.0, height - 0.12, 0.0)
    }
}

/// A catenary span between two poles: four cables, each a pair of orthogonal
/// ribbons so the wire has visible thickness from any angle. The sag is a true
/// parabola, `y = lerp - sag·4t(1-t)`.
///
/// Both endpoints are supplied as real hang points on real poles, so a span
/// cannot be drawn between two things that are not there. The sag is taken as a
/// fraction of the *span*, not a constant, because a fixed sag on a 10 m span
/// and on a 40 m span gives a taut wire and a rope respectively.
fn catenary(
    builder: &mut MeshBuilder,
    from_pole: &PoleTop,
    to_pole: &PoleTop,
    arterial: bool,
) -> usize {
    // The upper arm carries the three phase conductors, the lower arm the
    // communications bundle. Four drawn keeps the count down and still reads as
    // a pole line from any distance.
    let mut drawn = 0_usize;
    for (upper, lateral_sign) in [(true, 1.0_f32), (true, -1.0), (false, 1.0), (false, -1.0)] {
        let from = from_pole.conductor(upper, lateral_sign);
        let to = to_pole.conductor(upper, lateral_sign);
        let span = (to - from).length();
        // A span shorter than a few metres is not a span, and one longer than
        // about 60 m is a different pole line that has been joined by mistake.
        if !(6.0..60.0).contains(&span) {
            continue;
        }
        let direction = (to - from).normalized_or_up();
        let side = direction.cross(Vec3::new(0.0, 1.0, 0.0));
        let side = if side.length() < 0.05 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            side.normalized_or_up()
        };
        // Real conductor sag is roughly 1-2% of the span for a strung line. The
        // old constant of 0.44-0.55 m was right only for one span length.
        let sag = span * if arterial { 0.016 } else { 0.013 };
        let thickness = 0.045;
        let steps = 8;
        let mut previous: Option<(Vec3, Vec3)> = None;
        for step in 0..=steps {
            let t = step as f32 / steps as f32;
            let centre = from.lerp_point(to, t);
            let y = centre.y - sag * 4.0 * t * (1.0 - t);
            let a = centre + Vec3::new(0.0, y - centre.y, 0.0) + side * thickness;
            let b = centre + Vec3::new(0.0, y - centre.y, 0.0) - side * thickness;
            if let Some((pa, pb)) = previous {
                builder.quad("wire", pa, pb, b, a, None);
                // The second, vertical ribbon is what gives the cable a body
                // rather than reading as a line from above.
                let up = Vec3::new(0.0, thickness, 0.0);
                builder.quad("wire", pa, pa + up, pb + up, pb, None);
            }
            previous = Some((a, b));
        }
        drawn += 1;
    }
    // A span that was rejected is not a span, so the count is what was actually
    // drawn rather than a constant. This is what makes the reported figure mean
    // something.
    drawn
}

/// Bollards along a junction's kerb return.
///
/// Two filters, and both are load-bearing.  The walk ring is a *radially pushed*
/// copy of the junction's kerb fillet, so at a bend — or where a wide arterial
/// meets a narrow street — it can land on the far carriageway.  A spatial test
/// against every at-grade road's centreline is the only reliable guard; a
/// geometric "is it outside the box" test is not, and produced bollards in the
/// middle of the road.
fn junction_bollards(
    junction: &Junction,
    network: &Network,
    builder: &mut MeshBuilder,
    rng: &mut Rng,
    output: &mut FurnitureOutput,
) {
    // A two-port "junction" is a bend, not an intersection: its walk ring has no
    // kerb return to protect.
    if junction.ports.len() < 3 || junction.walk_ring.len() != junction.ring.len() {
        return;
    }
    let at_grade: Vec<(Vec<Vec2>, f32)> = network
        .roads
        .iter()
        .filter(|road| road.layer == 0)
        .map(|road| {
            (
                road.centreline
                    .points()
                    .iter()
                    .map(|v| Vec2::new(v.x, v.z))
                    .collect(),
                road.half_width() + 0.8,
            )
        })
        .collect();
    let ring = junction.walk_ring.clone();
    let mut accumulator = 0.0;
    for index in 0..ring.len() {
        let current = ring[index];
        let next = ring[(index + 1) % ring.len()];
        let segment = current.distance(next);
        if segment < 1.0e-3 {
            continue;
        }
        let mut travelled = accumulator;
        while travelled < segment {
            let t = travelled / segment;
            let mut point = current.lerp(next, t);
            // Pull a few percent toward the centre so the post stands on the
            // paving rather than half-buried in the kerb line.
            point += (junction.centre - point) * 0.03;
            travelled += 6.5;
            if crate::math::point_in_ring(point, &junction.ring) {
                continue;
            }
            if at_grade.iter().any(|(plan, clearance)| {
                crate::math::distance_to_polyline(point, plan) < *clearance
            }) {
                continue;
            }
            builder.add_instance(
                "furniture/bollard",
                Instance::new(
                    point.x,
                    level::KERB,
                    point.y,
                    // The bollard is radially symmetric, but its reflective bands
                    // are the detail, and a bar laid across the kerb return
                    // instead of along it is immediately wrong.
                    yaw_along_x(next - current),
                    1.0,
                    [0.92 + rng.unit() * 0.12, 0.92, 0.90],
                ),
            );
            output.bollards += 1;
        }
        accumulator = travelled - segment;
    }
}

/// Cars parked along the kerb, which is both a large part of a Chinese street's
/// visual density and a physical obstacle traffic must avoid.
///
/// # 违停 — illegally parked cars
///
/// The single most characteristic thing about a Chinese street at ground level
/// is that cars are parked *in* the carriageway: on the running lane, on the
/// crossing approach, half on the kerb, and often two deep.  A scene that only
/// puts cars tidily in a lay-by reads as a different country.  So a fraction of
/// the cars here are placed in the live lane and a fraction of *those* are placed
/// on the footway itself — and the traffic model drives through them, which is
/// both the honest outcome and a useful stress test of the follower logic.
pub fn park_cars(network: &Network, builder: &mut MeshBuilder, seed: u32) -> usize {
    let mut rng = Rng::new(seed ^ 0x9a12);
    let mut count = 0;
    for road in &network.roads {
        if road.layer != 0 || road.is_motorway() {
            continue;
        }
        let path = road.carriageway.clone();
        let length = path.length();
        if length < 30.0 {
            continue;
        }
        let section = road.section;
        let half = section.half_width();
        // Same kerb-face anchor as `place`: the ribbon edge is the property
        // line, and the kerb is one sidewalk width inside it.
        let kerb = half - section.sidewalk_metres.max(2.0);
        let curb = kerb - 0.85;
        // How much of the kerb is a lay-by, a running lane or the footway.  A
        // collector's kerbside lane is a running lane, so almost everything on it
        // is 违停; on an arterial the lay-by is a real bay.
        let layby = match road.class {
            ModernRoadClass::Arterial => 0.55,
            ModernRoadClass::Collector => 0.16,
            _ => 0.05,
        };
        for side in [1.0_f32, -1.0] {
            let mut station = 14.0 + rng.unit() * 20.0;
            while station < length - 14.0 {
                let roll = rng.unit();
                if roll < layby {
                    // In the lay-by, or in the kerbside lane pretending to be one.
                    let point = path.offset_at(station, side * curb, 0.0);
                    // The nose points the way the traffic on that side runs, and
                    // `side > 0` is the `+1` direction, which runs forward.
                    let heading = path.tangent_at(station) * if side > 0.0 { 1.0 } else { -1.0 };
                    let yaw = yaw_along_x(heading);
                    let colour = car_paint(&mut rng);
                    builder.add_instance(
                        "car/body",
                        Instance::new(point.x, level::ROAD, point.z, yaw, 1.0, colour),
                    );
                    builder.add_instance(
                        "car/glass",
                        Instance::new(point.x, level::ROAD, point.z, yaw, 1.0, [0.6, 0.7, 0.78]),
                    );
                    builder.add_instance(
                        "car/wheel",
                        Instance::new(point.x, level::ROAD, point.z, yaw, 1.0, [0.15, 0.15, 0.16]),
                    );
                    count += 1;
                    station += 6.4 + rng.unit() * 1.4;
                    continue;
                }
                if roll < layby + 0.14 {
                    // 违停: half on the footway, blocking nothing but the pavement.
                    let point = path.offset_at(station, side * (kerb + 0.45), 0.0);
                    let heading = path.tangent_at(station) * if side > 0.0 { 1.0 } else { -1.0 };
                    let yaw = yaw_along_x(heading)
                        + if rng.chance(0.5) {
                            0.0
                        } else {
                            std::f32::consts::FRAC_PI_2
                        };
                    let colour = car_paint(&mut rng);
                    builder.add_instance(
                        "car/body",
                        Instance::new(point.x, level::KERB - 0.03, point.z, yaw, 1.0, colour),
                    );
                    builder.add_instance(
                        "car/glass",
                        Instance::new(
                            point.x,
                            level::KERB - 0.03,
                            point.z,
                            yaw,
                            1.0,
                            [0.6, 0.7, 0.78],
                        ),
                    );
                    builder.add_instance(
                        "car/wheel",
                        Instance::new(
                            point.x,
                            level::KERB - 0.03,
                            point.z,
                            yaw,
                            1.0,
                            [0.15, 0.15, 0.16],
                        ),
                    );
                    count += 1;
                    station += 6.0 + rng.unit() * 3.0;
                    continue;
                }
                station += 9.0 + rng.unit() * 16.0;
            }
        }
    }
    count
}

/// A polyline helper the wire pass and the traffic model both need.
pub fn polyline(points: Vec<Vec2>) -> Path {
    Path::flat(points)
}

impl Vec3 {
    /// Component-wise linear interpolation, spelled out because `f32` lerps
    /// across two fields read badly at a call site.
    pub fn lerp_point(self, other: Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::derive;
    use crate::spec::JunctionSpec;
    use urban::{ModernChinaSpec, generate_modern_chinese_city};

    fn city() -> (urban::ModernCity, Network) {
        let city = generate_modern_chinese_city(ModernChinaSpec {
            seed: 42,
            radius_km: 0.5,
            block_size_metres: 110.0,
            ..ModernChinaSpec::default()
        });
        let network = derive(
            &city.nodes,
            &city.sd_roads,
            &city.hd_roads,
            city.frame,
            JunctionSpec::default(),
            city.seed,
        );
        (city, network)
    }

    #[test]
    fn prototypes_are_bound_so_one_transform_list_places_a_whole_object() {
        let mut builder = MeshBuilder::new();
        build_prototypes(&mut builder);
        for _ in 0..5 {
            builder.add_instance(
                "furniture/lamp",
                Instance::new(1.0, 0.0, 2.0, 0.3, 1.0, [1.0, 1.0, 1.0]),
            );
        }
        let scene = builder.build();
        // Every prototype gets a list, so a renderer can look one up without a
        // special case for "this city placed none of those".
        let prototypes = [
            "furniture/lamp",
            "furniture/pole",
            "furniture/bollard",
            "furniture/railing",
            "furniture/sign.guide",
            "furniture/sign.crossing",
            "furniture/gantry.steel",
            "furniture/gantry.board",
            "furniture/shelter",
            "furniture/bin",
            "furniture/kiosk",
            "car/body",
            "car/glass",
            "car/wheel",
        ];
        // One list per prototype.  Report both sides on failure: a bare "13 != 14"
        // does not say which prototype is missing.
        let present: Vec<&str> = scene
            .instances
            .iter()
            .map(|list| list.key.as_str())
            .collect();
        assert_eq!(
            present.len(),
            prototypes.len(),
            "one transform list per prototype. Present: {present:?}. Missing: {:?}",
            prototypes
                .iter()
                .filter(|key| !present.contains(&key.as_ref()))
                .collect::<Vec<_>>()
        );
        for key in prototypes {
            assert!(
                present.contains(&key),
                "{key} has no instance list; present: {present:?}"
            );
        }
        let lamps = scene
            .instances
            .iter()
            .find(|list| list.key == "furniture/lamp")
            .expect("no lamp instances");
        assert_eq!(lamps.instances.len(), 5);
        let lamp = scene
            .meshes
            .iter()
            .find(|group| group.material == "furniture/lamp")
            .expect("no lamp geometry");
        assert_eq!(lamp.instance_of.as_deref(), Some("furniture/lamp"));
    }

    #[test]
    fn a_city_gets_lamps_poles_wires_and_bollards() {
        let (city, network) = city();
        let mut builder = MeshBuilder::new();
        build_prototypes(&mut builder);
        let output = place(&network, &mut builder, city.seed);
        assert!(output.lamps > 20, "only {} lamps", output.lamps);
        assert!(output.poles > 10, "only {} poles", output.poles);
        assert!(output.spans > 10, "only {} wire spans", output.spans);
        assert!(output.bollards > 40, "only {} bollards", output.bollards);
        assert!(output.signs > 4, "only {} signs", output.signs);
    }

    /// The regression test for the defect that made a city look like a scribble.
    ///
    /// The wires were drawn by re-evaluating the *previous* pole's station
    /// against the *current* road's path and the *current* side, while poles
    /// alternated sides every 34 m. So a span whose poles stood on opposite
    /// kerbs was drawn from a point where no pole existed, crossing the
    /// carriageway diagonally. Nothing about that was visible in a count — the
    /// span count was correct — so it is asserted geometrically here: every wire
    /// must lie entirely on one side of every road's centreline, and must
    /// terminate at a point that a real pole occupies.
    /// Signed lateral offset of a point from a path's centreline.
    ///
    /// `Path` has no projection, and a road centreline is two or three points, so
    /// scanning the segments is both simpler and exact. The sign comes from the
    /// 2D cross product, which is what makes it a *signed* offset: a point left of
    /// the direction of travel is positive and one right is negative.
    ///
    /// A point that is not *beside* the path has no lateral offset, and must not
    /// be given one. The obvious implementation — clamp the projection to the
    /// segment, then take the cross product against the unclamped vector — is
    /// wrong in a way that silently inverts the test that uses it: for a point
    /// beyond the end of a straight run, `edge × (point − a)` is only the
    /// perpendicular departure, because the longitudinal part cancels. So a
    /// centreline ending at x = 346 reported a lateral of 1.0 m for a wire
    /// standing at x = −456, eight hundred metres away, and every "beside this
    /// road" filter downstream accepted it. Returning `None` unless the
    /// perpendicular foot actually lands on the path is what makes the sign mean
    /// what it says.
    fn signed_lateral(path: &crate::math::Path, point: Vec2) -> Option<f32> {
        let points = path.points();
        if points.len() < 2 {
            return None;
        }
        let mut best: Option<(f32, f32)> = None;
        for window in points.windows(2) {
            let a = Vec2::new(window[0].x, window[0].z);
            let b = Vec2::new(window[1].x, window[1].z);
            let edge = b - a;
            let length_squared = edge.length_squared();
            if length_squared < 1.0e-9 {
                continue;
            }
            let raw = (point - a).dot(edge) / length_squared;
            // Outside the segment's span, this point is past the end of the road,
            // not beside it. A hair of tolerance keeps a pole standing exactly on
            // the last node from being discarded.
            if !(-1.0e-3..=1.0 + 1.0e-3).contains(&raw) {
                continue;
            }
            let t = raw.clamp(0.0, 1.0);
            let closest = a + edge * t;
            let distance = (point - closest).length();
            if best.is_none_or(|(current, _)| distance < current) {
                best = Some((distance, edge.cross(point - a) / edge.length().max(1.0e-6)));
            }
        }
        best.map(|(_, lateral)| lateral)
    }

    #[test]
    fn no_wire_crosses_the_carriageway_it_belongs_to() {
        let (city, network) = city();
        let mut builder = MeshBuilder::new();
        build_prototypes(&mut builder);
        place(&network, &mut builder, city.seed);
        let scene = builder.build();
        let wires = scene
            .meshes
            .iter()
            .find(|group| group.material == "wire")
            .expect("no wire geometry");
        assert!(!wires.positions.is_empty(), "no wires");

        // A span must not cross **its own** road's carriageway.
        //
        // Testing against every road is wrong, and was: a pole line running down
        // one street correctly passes over the centreline of the street it
        // crosses, so the sign of its lateral offset relative to *that* road
        // necessarily flips at the junction. Flagging that is flagging correct
        // geometry. What identifies the owning road is that the span runs
        // *parallel* to it and close by — so the owner is the road the span hugs
        // most tightly, and only that road is asserted.
        let mut crossings = 0;
        let mut worst_gap = 0.0_f32;
        let mut checked = 0;
        for quad in wires.positions.chunks_exact(12) {
            let corners: Vec<Vec2> = (0..4)
                .map(|index| Vec2::new(quad[index * 3], quad[index * 3 + 2]))
                .collect();
            // Pick the road this span runs beside: the one with the smallest mean
            // absolute lateral offset.
            let mut owner: Option<(f32, Vec<f32>)> = None;
            for road in &network.roads {
                if road.layer != 0 || road.is_motorway() {
                    continue;
                }
                let path = road.carriageway.clone();
                let laterals: Vec<f32> = corners
                    .iter()
                    .filter_map(|corner| signed_lateral(&path, *corner))
                    .collect();
                if laterals.len() < 4 {
                    continue;
                }
                let mean =
                    laterals.iter().map(|value| value.abs()).sum::<f32>() / laterals.len() as f32;
                // It has to actually be beside it, not a distant road that happens
                // to be marginally closer.
                let limit = road.section.half_width() + road.section.sidewalk_metres + 3.0;
                if mean > limit {
                    continue;
                }
                if owner.as_ref().is_none_or(|(best, _)| mean < *best) {
                    owner = Some((mean, laterals));
                }
            }
            let Some((_, laterals)) = owner else { continue };
            checked += 1;
            // A span down one kerb has one sign throughout. A span that cuts
            // diagonally across the carriageway has both, and that was the defect:
            // the previous pole's station was re-evaluated against the *current*
            // side, so a span between opposite kerbs started at a point where no
            // pole stood.
            if laterals.iter().any(|value| *value > 0.05)
                && laterals.iter().any(|value| *value < -0.05)
            {
                worst_gap = worst_gap.max(laterals.iter().fold(0.0_f32, |a, v| a.max(v.abs())));
                crossings += 1;
            }
        }
        assert!(
            checked > 20,
            "only {checked} spans could be attributed to a road"
        );
        assert_eq!(
            crossings, 0,
            "{crossings} of {checked} wire spans cross the carriageway they belong to \
             (worst reached {worst_gap:.1} m past the centreline); a pole line is a \
             straight run down one kerb"
        );
    }

    /// The second half of the same defect: the wires were attached to points that
    /// did not exist, so they also did not line up with the poles they appeared to
    /// come from — the conductors floated about a metre inboard of the arm tips.
    ///
    /// Distance is measured **horizontally**, from the wire to a pole's *axis*.
    /// Measuring three-dimensionally from the instance origin compares a point
    /// eight metres up the pole against the ground it stands on, so every vertex
    /// is "unattached" and the test passes for the wrong reason or fails for a
    /// reason that has nothing to do with the arms.
    /// The second half of the same defect: the wires were attached to points that
    /// did not exist, so they also did not line up with the poles they appeared to
    /// come from — the conductors floated about a metre inboard of the arm tips.
    ///
    /// Distance is measured **horizontally**, from the wire to a pole's *axis*.
    /// Measuring three-dimensionally from the instance origin compares a point
    /// eight metres up the pole against the ground it stands on, so every vertex
    /// is "unattached" and the test passes for the wrong reason or fails for a
    /// reason that has nothing to do with the arms.
    ///
    /// The assertion is per *span endpoint*, not a proportion of all vertices. A
    /// proportion is not a testable quantity here and never was: a span is 34 m
    /// long, sampled at 9 stations, so its interior stations are 4 m to 17 m from
    /// the nearest pole no matter how correctly the wire is strung. Only 2 of the
    /// 9 stations can ever be within reach of an arm, which caps the achievable
    /// attachment rate at about 12% — and the old code's 88% "orphans" figure was
    /// the same 88% you get from a *perfect* wire. What distinguishes a wire that
    /// meets the arms from one that misses them is entirely in the endpoints, so
    /// that is what is checked: every span must begin and end within an arm's
    /// reach of a real pole, and those two poles must be different ones.
    #[test]
    fn every_wire_ends_at_a_pole_arm() {
        let (city, network) = city();
        let mut builder = MeshBuilder::new();
        build_prototypes(&mut builder);
        place(&network, &mut builder, city.seed);
        let scene = builder.build();
        let wires = scene
            .meshes
            .iter()
            .find(|group| group.material == "wire")
            .expect("no wire geometry");
        let poles = scene
            .instances
            .iter()
            .find(|list| list.key == "furniture/pole")
            .expect("no poles")
            .instances
            .clone();
        assert!(poles.len() > 10, "only {} poles to wire", poles.len());
        // The arm tip of a 9 m pole, plus a little for the insulator.
        let reach = 0.19 * 9.0 + 0.3;

        // One conductor is 8 ribbon steps of 2 quads. Asserting on the internal
        // layout of those quads would make the test a hostage to the ribbon's
        // implementation, so the two hang points are found geometrically instead:
        // within one conductor's own vertices, two of them must lie within an arm's
        // reach of two *different* poles. That is the whole claim — the wire is
        // strung from one arm to the next — and it does not care which corner of
        // which quad the endpoints were emitted into.
        const QUADS_PER_CONDUCTOR: usize = 16;
        let quads: Vec<[f32; 12]> = wires
            .positions
            .chunks_exact(12)
            .map(|quad| {
                let mut out = [0.0_f32; 12];
                out.copy_from_slice(quad);
                out
            })
            .collect();
        assert!(
            quads.len() % QUADS_PER_CONDUCTOR == 0,
            "{} wire quads is not a whole number of conductors; the span layout \
             changed and this test is no longer looking at what it thinks",
            quads.len()
        );
        // The index of the nearest pole to a point, and how far away it is.
        let nearest_pole = |point: Vec2| -> (usize, f32) {
            poles
                .iter()
                .enumerate()
                .map(|(index, pole)| (index, (point - Vec2::new(pole.x, pole.z)).length()))
                .fold((usize::MAX, f32::INFINITY), |best, candidate| {
                    if candidate.1 < best.1 {
                        candidate
                    } else {
                        best
                    }
                })
        };
        let mut conductors = 0_usize;
        let mut unstrung = 0_usize;
        for block in quads.chunks_exact(QUADS_PER_CONDUCTOR) {
            conductors += 1;
            // Every corner of every quad in this conductor, as plan points.
            let corners: Vec<Vec2> = block
                .iter()
                .flat_map(|quad| quad.iter().copied())
                .collect::<Vec<f32>>()
                .chunks_exact(3)
                .map(|vertex| Vec2::new(vertex[0], vertex[2]))
                .collect();
            // The distinct poles this conductor's own vertices reach.
            let mut attached: Vec<usize> = corners
                .iter()
                .map(|corner| nearest_pole(*corner))
                .filter(|(_, distance)| *distance <= reach)
                .map(|(index, _)| index)
                .collect();
            attached.sort_unstable();
            attached.dedup();
            // Fewer than two means the conductor is a stub hanging off a single
            // arm, or is drawn to a point where no pole stands at all.
            if attached.len() < 2 {
                unstrung += 1;
            }
        }
        assert!(conductors > 20, "only {conductors} conductors to check");
        assert_eq!(
            unstrung, 0,
            "{unstrung} of {conductors} conductors do not reach two different pole \
             arms within {reach:.2} m horizontally; the wires touch nothing"
        );
    }

    /// The cross-arms point in a random direction, which makes a pole line look
    /// like a scribble from any angle.
    ///
    /// The test cannot simply require every pole to share one heading, because a
    /// city has many streets and each has its own. What distinguishes real
    /// alignment from a uniform random yaw is *clustering*: an aligned city has a
    /// handful of dominant headings, one per street, each covering a real share of
    /// the poles. A uniform random yaw spreads poles evenly over the circle, so
    /// its largest cluster is a fixed small fraction no matter how many poles
    /// there are. Asserting on the largest cluster separates the two without
    /// needing to know which road each pole belongs to.
    #[test]
    fn pole_cross_arms_cluster_per_street_rather_than_scattering_at_random() {
        let (city, network) = city();
        let mut builder = MeshBuilder::new();
        build_prototypes(&mut builder);
        place(&network, &mut builder, city.seed);
        let scene = builder.build();
        let poles = scene
            .instances
            .iter()
            .find(|list| list.key == "furniture/pole")
            .expect("no poles")
            .instances
            .clone();
        assert!(poles.len() > 40, "only {} poles to check", poles.len());

        // Bucket headings and take the largest bucket, allowing a street's own
        // curvature to spread its poles slightly either side of its mean.
        const BUCKETS: usize = 72;
        let bucket_of = |yaw: f32| {
            let wrapped = yaw.rem_euclid(std::f32::consts::TAU);
            (wrapped / std::f32::consts::TAU * BUCKETS as f32) as usize % BUCKETS
        };
        let mut histogram = vec![0_usize; BUCKETS];
        for pole in &poles {
            histogram[bucket_of(pole.rotation_y)] += 1;
        }
        // Three neighbouring buckets, so a street spanning a bucket boundary
        // still reads as one cluster.
        let largest = (0..BUCKETS)
            .map(|index| {
                histogram[index]
                    + histogram[(index + 1) % BUCKETS]
                    + histogram[(index + BUCKETS - 1) % BUCKETS]
            })
            .max()
            .unwrap_or(0);
        let share = largest as f32 / poles.len() as f32;
        // With a uniform yaw over 2*pi, three of 72 buckets hold about 4% of the
        // poles whatever the count. A real city puts a fifth or more down a
        // single street, so this separates the two cases decisively.
        assert!(
            share > 0.12,
            "the largest cross-arm heading cluster holds only {largest} of {} poles \
             ({:.0}%); a uniform random yaw would give about 4%",
            poles.len(),
            share * 100.0
        );
    }

    #[test]
    fn no_bollard_stands_in_a_carriageway() {
        let (city, network) = city();
        let mut builder = MeshBuilder::new();
        build_prototypes(&mut builder);
        place(&network, &mut builder, city.seed);
        let scene = builder.build();
        let bollards = scene
            .instances
            .iter()
            .find(|list| list.key == "furniture/bollard")
            .expect("no bollards");
        for instance in &bollards.instances {
            let point = Vec2::new(instance.x, instance.z);
            for road in &network.roads {
                // An elevated road crosses in plan but not in space, so a bollard
                // beneath a bridge is correct.  The placement rule skips those
                // roads and so must the audit.
                if road.layer != 0 {
                    continue;
                }
                // Measured against the *untrimmed* centreline.  The trimmed
                // carriageway stops at the junction edge, so a bollard correctly
                // standing on the corner paving two metres outside an arterial's
                // kerb would read as being inside the road if the finite trimmed
                // segment were the reference.
                let plan: Vec<Vec2> = road
                    .centreline
                    .points()
                    .iter()
                    .map(|v| Vec2::new(v.x, v.z))
                    .collect();
                let clearance = crate::math::distance_to_polyline(point, &plan) - road.half_width();
                assert!(
                    clearance > 0.0,
                    "a bollard sits {clearance:.2} m inside road {}",
                    road.id
                );
            }
        }
    }
}

/// Car body colour as an instance tint.  Chinese roads are mostly white, black,
/// silver and grey, with a minority of red, blue and dark green — never a
/// uniform white fleet, which reads as a row of paper cut-outs.
fn car_paint(rng: &mut Rng) -> [f32; 3] {
    let pick = rng.unit();
    let shade = 0.9 + rng.unit() * 0.2;
    let base = if pick < 0.28 {
        [0.95, 0.95, 0.94]
    } else if pick < 0.52 {
        [0.10, 0.10, 0.11]
    } else if pick < 0.72 {
        [0.55, 0.57, 0.60]
    } else if pick < 0.84 {
        [0.28, 0.29, 0.31]
    } else if pick < 0.90 {
        [0.62, 0.10, 0.10]
    } else if pick < 0.96 {
        [0.14, 0.24, 0.48]
    } else {
        [0.42, 0.34, 0.24]
    };
    [base[0] * shade, base[1] * shade, base[2] * shade]
}
