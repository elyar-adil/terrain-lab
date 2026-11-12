//! Cars, generated: every design from one number, inside and out.
//!
//! A car is the object in a street that people know best, so it is where a
//! model is judged. Nothing here is a fixed model. `design` draws a design
//! from a seed: a kind of car, its proportions inside what that kind allows,
//! and a kit of parts chosen and dimensioned for it (lamps and what is in
//! them, grille, tyres and their tread, rims, mirrors). `form` lofts the one
//! skin of its body; `body` cuts the arches, glass, lamps and shut lines into
//! that skin (`surface` is the knife), so every part meets its neighbour along
//! a shared edge; `lamps`, `wheels`, `fittings` and `interior` build what goes
//! in the openings and behind the glass.
//!
//! A city draws a fleet of [`CAR_VARIANTS`] designs from its own seed, so no
//! two cities park the same cars. All of that is for a car within a street's
//! width. Past [`CAR_NEAR_M`] the renderer draws the `car/far.*` prototypes
//! instead: the same outline in a few hundred triangles.
use std::cell::Cell;
use std::sync::{Arc, Mutex};

use crate::mesh::{GroupStyle, Instance, InstanceGroup, MeshBuilder, MeshGroup};

mod body;
mod design;
mod fittings;
mod form;
mod interior;
mod lamps;
mod parts;
mod surface;
mod wheels;

use design::Kind;
use form::{Form, Spec};

/// What a triangle of a cut surface is.
mod label {
    pub const BODY: u8 = 0;
    /// A wheel arch: nothing.
    pub const ARCH: u8 = 1;
    pub const GLASS: u8 = 2;
    /// Windscreen, rear glass, roof glass.
    pub const SCREEN: u8 = 3;
    /// The black border printed on a screen.
    pub const FRIT: u8 = 4;
    /// Black, flush: sills, pillars, cladding, seals.
    pub const BLACK: u8 = 5;
    pub const SEAM: u8 = 6;
    pub const SEAM_NEW: u8 = 7;
    /// The strip round the side glass.
    pub const SURROUND: u8 = 8;
    pub const HEAD: u8 = 9;
    pub const TAIL: u8 = 10;
    pub const CLEAR: u8 = 11;
    pub const AMBER: u8 = 12;
    pub const REFLECT: u8 = 13;
    pub const MARKER: u8 = 14;
    pub const GRILLE: u8 = 15;
    pub const INTAKE: u8 = 16;
    /// The dish behind a door handle.
    pub const POCKET: u8 = 17;
    pub const FOG: u8 = 18;
}

/// Within this distance a car is drawn in full; beyond it, by its far model.
/// The renderer's `cityLod.ts` holds the same number.
pub const CAR_NEAR_M: f32 = 45.0;

/// How many designs a city's fleet has; variant `n > 0` is keyed `car/<part>~n`.
pub const CAR_VARIANTS: usize = 6;

/// The kinds a fleet is made of, and the share of the street each variant
/// takes: saloons most of all, an SUV, the small van (面包车) that does every
/// delivery, and three drawn freely in the proportions of a Chinese street.
/// Variant 0, which the unsuffixed prototype keys draw, is always a saloon.
/// Within its kind every design is free.
const FLEET: [(Option<Kind>, f32); CAR_VARIANTS] = [
    (Some(Kind::Saloon), 0.21),
    (Some(Kind::Suv), 0.19),
    (None, 0.18),
    (None, 0.17),
    (None, 0.17),
    (Some(Kind::Van), 0.08),
];

thread_local! {
    /// The seed of the fleet being built: set by `build_prototypes`, read when
    /// a car is placed (its rims' and plate's tints belong to its design).
    static FLEET_SEED: Cell<u64> = const { Cell::new(0) };
}

/// The city a fleet is registered in, as a number in `0..1`: the plate shader
/// turns it into a province's character and a city's letter.
fn registry(fleet: u64) -> f32 {
    (fleet.wrapping_mul(0x9e37_79b9_7f4a_7c15) >> 40) as f32 / (1u64 << 24) as f32
}

/// The design of a variant of a fleet.
fn fleet_design(fleet: u64, variant: usize) -> Spec {
    let seed = fleet
        .wrapping_mul(0x2545_f491_4f6c_dd1d)
        .wrapping_add(variant as u64 * 0x9e37_79b9);
    match FLEET[variant % CAR_VARIANTS].0 {
        Some(kind) => design::design_of(kind, &mut design::Rng::new(seed)),
        None => design::design(seed),
    }
}

/// The parts of the full model, and of the far one.
const NEAR_PARTS: [&str; 14] = [
    "body",
    "glass",
    "wheel",
    "rim",
    "trim",
    "chrome",
    "lens",
    "lens.red",
    "lamp.front",
    "lamp.rear",
    "lamp.amber",
    "interior",
    "plate",
    "well",
];
const FAR_PARTS: [&str; 3] = ["far.body", "far.glass", "far.wheel"];

/// The prototype key of a car part for a variant.
pub fn car_key(part: &str, variant: usize) -> String {
    if variant == 0 {
        format!("car/{part}")
    } else {
        format!("car/{part}~{variant}")
    }
}

/// Every prototype key this module binds.
pub fn prototype_keys() -> Vec<String> {
    let mut keys = Vec::new();
    for variant in 0..CAR_VARIANTS {
        for part in NEAR_PARTS.iter().chain(FAR_PARTS.iter()) {
            keys.push(car_key(part, variant));
        }
    }
    keys
}

/// Which design stands at a place: by where it is, so the choice draws nothing
/// from the placement's random stream.
pub fn car_variant_at(x: f32, z: f32) -> usize {
    let cell = worldgen_core::hash::avalanche01(
        0x00ca_7a11,
        (x * 7.0).round() as i32,
        (z * 7.0).round() as i32,
        3,
    );
    let mut at = cell;
    for (variant, (_, share)) in FLEET.iter().enumerate() {
        if at < *share {
            return variant;
        }
        at -= share;
    }
    0
}

/// Upholstery, by place: cloth greys most often, then beige, black and tan.
fn interior_tone(x: f32, z: f32) -> [f32; 3] {
    let cell = worldgen_core::hash::avalanche01(
        0x1a7e_0105,
        (x * 5.0).round() as i32,
        (z * 5.0).round() as i32,
        9,
    );
    const TONES: [[f32; 3]; 5] = [
        [0.115, 0.115, 0.12],
        [0.20, 0.20, 0.205],
        [0.40, 0.35, 0.28],
        [0.045, 0.045, 0.05],
        [0.30, 0.19, 0.12],
    ];
    TONES[((cell * TONES.len() as f32) as usize).min(TONES.len() - 1)]
}

/// The tint a part is drawn with: the body's paint and the upholstery belong
/// to the car, the rims' finish and the plate's colour to its design.
fn part_tint(
    part: &str,
    spec: &Spec,
    paint: [f32; 3],
    tone: [f32; 3],
    plate: [f32; 2],
) -> [f32; 3] {
    match part {
        "body" | "far.body" => paint,
        "glass" | "far.glass" => [0.6, 0.7, 0.78],
        "wheel" | "far.wheel" => [0.15, 0.15, 0.16],
        "rim" => spec.rim.tint,
        "interior" => tone,
        // A plate's tint is not a colour but its registration: the number it
        // carries, whether it is a blue plate or a new-energy green one, and
        // the city that issued it. The renderer's plate shader reads the three
        // and draws the plate (`carMaterials.ts`).
        "plate" => [plate[0], if spec.electric { 1.0 } else { 0.0 }, plate[1]],
        _ => [1.0, 1.0, 1.0],
    }
}

/// Every part of a variant of a fleet with its tint, in a showroom red: for
/// the harness.
pub fn part_tints(fleet: u64, variant: usize) -> Vec<(&'static str, [f32; 3])> {
    let spec = fleet_design(fleet, variant);
    NEAR_PARTS
        .iter()
        .chain(FAR_PARTS.iter())
        .map(|part| {
            (
                *part,
                part_tint(
                    part,
                    &spec,
                    [0.33, 0.03, 0.025],
                    [0.115, 0.115, 0.12],
                    [0.137 + variant as f32 * 0.171, registry(fleet)],
                ),
            )
        })
        .collect()
}

fn instances_of(x: f32, y: f32, z: f32, yaw: f32, paint: [f32; 3]) -> Vec<(String, Instance)> {
    let variant = car_variant_at(x, z);
    let tone = interior_tone(x, z);
    let fleet = FLEET_SEED.get();
    let spec = fleet_design(fleet, variant);
    // Every car its own number, from where it stands.
    let number = worldgen_core::hash::avalanche01(
        0x9_1a7e,
        (x * 11.0).round() as i32,
        (z * 11.0).round() as i32,
        5,
    );
    let plate = [number, registry(fleet)];
    NEAR_PARTS
        .iter()
        .chain(FAR_PARTS.iter())
        .map(|part| {
            (
                car_key(part, variant),
                Instance::new(
                    x,
                    y,
                    z,
                    yaw,
                    1.0,
                    part_tint(part, &spec, paint, tone, plate),
                ),
            )
        })
        .collect()
}

/// One car standing at a place, nose along local `+X`: every part of both models.
pub fn add_car(builder: &mut MeshBuilder, x: f32, y: f32, z: f32, yaw: f32, paint: [f32; 3]) {
    for (key, instance) in instances_of(x, y, z, yaw, paint) {
        builder.add_instance(&key, instance);
    }
}

/// The same, into instance lists that have already been built (the traffic's
/// vehicles are known only after the geometry is).
pub fn add_car_to(
    groups: &mut Vec<InstanceGroup>,
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
    paint: [f32; 3],
) {
    for (key, instance) in instances_of(x, y, z, yaw, paint) {
        match groups.iter_mut().find(|group| group.key == key) {
            Some(group) => group.instances.push(instance),
            None => groups.push(InstanceGroup {
                key,
                instances: vec![instance],
            }),
        }
    }
}

/// Paint for a moving vehicle, by the traffic model's colour index: the whites,
/// blacks and silvers that are most of a Chinese street, then the rest.
pub fn fleet_paint(index: u8) -> [f32; 3] {
    const PAINTS: [[f32; 3]; 12] = [
        [0.87, 0.87, 0.85],
        [0.87, 0.87, 0.85],
        [0.87, 0.87, 0.85],
        [0.009, 0.009, 0.011],
        [0.009, 0.009, 0.011],
        [0.009, 0.009, 0.011],
        [0.34, 0.37, 0.39],
        [0.34, 0.37, 0.39],
        [0.105, 0.112, 0.125],
        [0.262, 0.012, 0.012],
        [0.018, 0.051, 0.158],
        [0.147, 0.102, 0.058],
    ];
    PAINTS[index as usize % PAINTS.len()]
}

/// Every part of one design, under the keys `key` gives.
fn build_design(builder: &mut MeshBuilder, spec: &Spec, key: &dyn Fn(&str) -> String) {
    let form = Form::new(spec);
    let soup = body::build(builder, &form, key);
    lamps::build(builder, &soup, &form, key);
    fittings::build(builder, &soup, &form, key);
    interior::build(builder, spec, key);
    wheels::build(builder, spec, key);
    body::build_far(builder, &form, &key("far.body"), &key("far.glass"));
    wheels::build_far(builder, spec, &key("far.wheel"));
}

/// The last fleet built, kept: a city is generated tile by tile, and every
/// tile of it wants the same cars.
static BUILT: Mutex<Option<(u64, Arc<Vec<MeshGroup>>)>> = Mutex::new(None);

fn fleet_meshes(fleet: u64) -> Arc<Vec<MeshGroup>> {
    let mut built = BUILT
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some((seed, meshes)) = built.as_ref()
        && *seed == fleet
    {
        return meshes.clone();
    }
    let mut builder = MeshBuilder::new();
    for variant in 0..CAR_VARIANTS {
        let spec = fleet_design(fleet, variant);
        build_design(&mut builder, &spec, &|part: &str| car_key(part, variant));
    }
    let meshes = Arc::new(builder.build().meshes);
    *built = Some((fleet, meshes.clone()));
    meshes
}

/// The fleet of the city whose seed is `fleet`: [`CAR_VARIANTS`] designs.
pub fn build_prototypes(builder: &mut MeshBuilder, fleet: u64) {
    FLEET_SEED.set(fleet);
    for mesh in fleet_meshes(fleet).iter() {
        builder.append(
            &mesh.material,
            &mesh.positions,
            &mesh.normals,
            mesh.uvs.as_deref(),
            &mesh.indices,
        );
    }
    for variant in 0..CAR_VARIANTS {
        let key = |part: &str| car_key(part, variant);
        for part in NEAR_PARTS.iter().chain(FAR_PARTS.iter()) {
            let key = key(part);
            // Glass and lenses cast no shadow (they would black out what they
            // cover), and nothing inside the car needs to cast one.
            let solid = matches!(
                *part,
                "body" | "wheel" | "rim" | "far.body" | "far.glass" | "far.wheel"
            );
            builder.style(
                &key,
                GroupStyle {
                    cast_shadow: solid,
                    receive_shadow: !matches!(*part, "glass" | "lens" | "lens.red"),
                    ..GroupStyle::default()
                },
            );
            builder.bind(&key, &key);
        }
    }
}

#[cfg(test)]
mod tests;
