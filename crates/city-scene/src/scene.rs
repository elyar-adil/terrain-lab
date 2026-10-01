//! The city scene: everything a renderer needs, assembled and serialised.
//!
//! This is the boundary the rest of the project is built around.  `urban`
//! decides *what* a city is; this module turns that plan into finished vertex
//! buffers, instanced prototype lists, baked textures, signal states and a
//! traffic fleet, and hands the renderer nothing it has to derive.
//!
//! # Why the payload is shaped the way it is
//!
//! Buffers are **base64 `f32`**, matching the heightfield and mask encoding the
//! project already uses.  JSON arrays of floats cost roughly 1.5x the bytes and
//! several times the parse time, and a city is tens of megabytes of vertex
//! data — the difference between a payload that streams and one that does not.
//!
//! Instanced geometry is stored **once** and referenced by key.  Ten tree
//! prototypes carry every tree in every city, so a leaf-detailed avenue costs
//! two draw calls rather than one per tree.  This is the single biggest reason
//! the previous port could not afford leaf-level foliage: it built every tree
//! individually and then capped the whole world at 1 200 of them.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::Serialize;

use urban::ModernCity;

use crate::buildings;
use crate::furniture::{self, FurnitureOutput};
use crate::math::Rng;
use crate::mesh::{Instance, MeshBuilder, SceneGeometry};
use crate::network::{self, Network};
use crate::spec::JunctionSpec;
use crate::street::{self, SignalRig, StreetOutput};
use crate::textures::BakedTexture;
use crate::traffic::{self, TrafficState};
use crate::trees::{self, TreePrototype, TreeOutput};

/// Knobs that trade payload size against fidelity.
///
/// The defaults are chosen so a 1.8 km city with a few thousand buildings lands
/// in the low tens of megabytes — the same order as the rest of the generation
/// payload — while still being leaf-detailed and fully traffic-simulated.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneBudget {
    /// Hard cap on buildings.  Beyond this the extras are dropped, not the
    /// shells, so a skyline never develops a hole where the detail stopped.
    pub max_buildings: usize,
    /// Per-city tree budget.
    pub max_trees: usize,
    /// Facade texture bake resolution.
    pub facade_texture_size: usize,
    /// Ground texture bake resolution.
    pub ground_texture_size: usize,
    /// Vehicle count.
    pub vehicles: usize,
    /// Rooftop plant, balconies and signage are only worth generating for
    /// buildings at least this large; below it they are sub-pixel.
    pub detail_min_footprint_m2: f32,
}

impl Default for SceneBudget {
    fn default() -> Self {
        Self {
            max_buildings: 4000,
            max_trees: trees::TREE_BUDGET,
            facade_texture_size: 256,
            ground_texture_size: 256,
            // A Chinese arterial at midday carries wall-to-wall traffic in the
            // reference photographs; a fleet that renders as forty scattered
            // cars reads as an empty city. Poses cost ten floats each, so the
            // budget is bounded by the simulation, not the payload.
            vehicles: 160,
            detail_min_footprint_m2: 180.0,
        }
    }
}

/// One mesh group's buffers, base64 encoded.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EncodedMesh {
    pub material: String,
    pub positions: String,
    pub normals: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uvs: Option<String>,
    pub indices: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_of: Option<String>,
    pub vertex_count: usize,
    pub triangle_count: usize,
    pub cast_shadow: bool,
    pub receive_shadow: bool,
    pub alpha_cutout: bool,
    pub dynamic: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EncodedInstances {
    pub key: String,
    pub count: usize,
    /// Ten floats per instance: `x, y, z, rotationY, scaleX, scaleY, scaleZ,
    /// tintR, tintG, tintB`.
    pub data: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EncodedTexture {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub tile_width_m: f32,
    pub tile_height_m: f32,
    pub has_normal_source: bool,
    /// `u8` RGBA.
    pub data: String,
}

/// Everything a renderer needs for one city.  All coordinates are **city-local
/// metres** relative to `origin`, so the renderer only has to add a single
/// offset and a terrain height.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CityScene {
    pub version: u32,
    pub seed: u32,
    /// World kilometre point the local frame's origin sits at.
    pub origin: [f32; 2],
    pub rotation_radians: f32,
    /// Local extent, metres, so the renderer can frame the city without reading
    /// every vertex.
    pub extent_m: [f32; 4],
    pub meshes: Vec<EncodedMesh>,
    pub instances: Vec<EncodedInstances>,
    pub textures: Vec<EncodedTexture>,
    pub signals: Vec<SignalRig>,
    pub traffic: TrafficState,
    pub network: network::export::NetworkRecord,
    pub stats: SceneStats,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneStats {
    pub buildings: usize,
    pub trees: usize,
    pub tree_instances: usize,
    pub tufts: usize,
    pub parked_cars: usize,
    pub vehicles: usize,
    pub roads: usize,
    pub lanes: usize,
    pub connectors: usize,
    pub junctions: usize,
    pub signals: usize,
    pub bollards: usize,
    pub lamps: usize,
    pub poles: usize,
    pub railings: usize,
    pub signs: usize,
    pub shelters: usize,
    pub shrubs: usize,
    pub wire_spans: usize,
    pub draw_calls: usize,
    pub vertices: usize,
    pub triangles: usize,
    /// Engineering problems found while deriving.  Reported, never hidden.
    pub warnings: Vec<String>,
    pub tree_roles: Vec<(String, usize)>,
}

fn encode_f32(values: &[f32]) -> String {
    let mut bytes = Vec::with_capacity(values.len() * 4);
    for value in values {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    STANDARD.encode(bytes)
}

fn encode_u32(values: &[u32]) -> String {
    let mut bytes = Vec::with_capacity(values.len() * 4);
    for value in values {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    STANDARD.encode(bytes)
}

fn encode_bytes(values: &[u8]) -> String {
    STANDARD.encode(values)
}

fn encode_geometry(geometry: SceneGeometry) -> (Vec<EncodedMesh>, Vec<EncodedInstances>) {
    let mut meshes: Vec<EncodedMesh> = Vec::with_capacity(geometry.meshes.len());
    let mut instances: Vec<EncodedInstances> = Vec::with_capacity(geometry.instances.len());
    for group in geometry.meshes {
        let vertex_count = group.positions.len() / 3;
        meshes.push(EncodedMesh {
            material: group.material,
            positions: encode_f32(&group.positions),
            normals: encode_f32(&group.normals),
            colors: group.colors.as_deref().map(encode_bytes),
            uvs: group.uvs.as_deref().map(|values| encode_f32(values)),
            indices: encode_u32(&group.indices),
            instance_of: group.instance_of.clone(),
            vertex_count,
            triangle_count: group.indices.len() / 3,
            cast_shadow: group.cast_shadow,
            receive_shadow: group.receive_shadow,
            alpha_cutout: group.alpha_cutout,
            dynamic: group.dynamic,
        });
    }
    for list in geometry.instances {
        let mut data = Vec::with_capacity(list.instances.len() * 10);
        for instance in &list.instances {
            data.extend_from_slice(&[
                instance.x,
                instance.y,
                instance.z,
                instance.rotation_y,
                instance.scale_x,
                instance.scale_y,
                instance.scale_z,
                instance.tint_r,
                instance.tint_g,
                instance.tint_b,
            ]);
        }
        instances.push(EncodedInstances {
            key: list.key,
            count: list.instances.len(),
            data: encode_f32(&data),
        });
    }
    (meshes, instances)
}

fn encode_textures(textures: &[BakedTexture]) -> Vec<EncodedTexture> {
    textures
        .iter()
        .map(|texture| EncodedTexture {
            name: texture.name.clone(),
            width: texture.width,
            height: texture.height,
            tile_width_m: texture.tile_width_m,
            tile_height_m: texture.tile_height_m,
            has_normal_source: texture.has_normal_source,
            data: encode_bytes(&texture.rgba),
        })
        .collect()
}

fn extent_of(network: &Network) -> [f32; 4] {
    let mut extent = [f32::MAX, f32::MAX, f32::MIN, f32::MIN];
    for junction in &network.junctions {
        extent[0] = extent[0].min(junction.centre.x);
        extent[1] = extent[1].min(junction.centre.y);
        extent[2] = extent[2].max(junction.centre.x);
        extent[3] = extent[3].max(junction.centre.y);
    }
    if extent[0] > extent[2] {
        return [0.0, 0.0, 0.0, 0.0];
    }
    extent
}

/// Derive a whole city scene.
pub fn build_city_scene(city: &ModernCity, budget: SceneBudget) -> CityScene {
    let spec = JunctionSpec::default();
    let network = network::derive(
        &city.nodes,
        &city.sd_roads,
        &city.hd_roads,
        city.frame,
        spec,
        city.seed,
    );

    let mut builder = MeshBuilder::new();
    // Prototypes first, so their groups exist and can be bound before anything
    // instances into them.
    let prototypes: Vec<TreePrototype> = trees::prototype_set();
    trees::build_prototypes(&prototypes, &mut builder);
    trees::build_shrub_prototype(&mut builder);
    trees::build_tuft_prototype(&mut builder);
    furniture::build_prototypes(&mut builder);

    let StreetOutput {
        signals,
        traffic_lights_built: _,
    } = street::build(&network, &mut builder, city.seed);
    let FurnitureOutput {
        lamps,
        poles,
        bollards,
        railings,
        signs,
        shelters,
        spans,
    } = furniture::place(&network, &mut builder, city.seed);
    let shrubs = trees::plant_median_shrubs(&network, &mut builder, city.seed);
    let parked = furniture::park_cars(&network, &mut builder, city.seed);
    let tree_output = trees::plant(
        &network,
        &city.parcels,
        city.river.as_deref(),
        city.river_width_metres,
        city.frame,
        &prototypes,
        &mut builder,
        city.seed,
    );
    let mut buildings = city.buildings.clone();
    buildings.truncate(budget.max_buildings);
    buildings::build(
        &city.blocks,
        &city.parcels,
        &buildings,
        &city.compounds,
        city.frame,
        &mut builder,
    );

    if city.river.is_some() || !city.tributaries.is_empty() {
        // Discs along every river bridge: the quay stops short of them.
        let mut crossings: Vec<(crate::math::Vec2, f32)> = Vec::new();
        for road in network.roads.iter().filter(|road| road.bridge) {
            let length = road.carriageway.length();
            let mut station = 0.0;
            while station <= length {
                let point = road.carriageway.sample(station).0;
                crossings.push((crate::math::Vec2::new(point.x, point.z), road.half_width() + 9.0));
                station += 6.0;
            }
        }
        if let Some(river) = city.river.as_deref() {
            buildings::build_water(river, city.river_width_metres, city.frame, &crossings, &mut builder);
        }
        for tributary in &city.tributaries {
            buildings::build_water(&tributary.line, tributary.width_metres, city.frame, &crossings, &mut builder);
        }
    }

    let geometry = builder.build();
    let vertices = geometry
        .meshes
        .iter()
        .map(|group| group.positions.len() / 3)
        .sum();
    let triangles = geometry
        .meshes
        .iter()
        .map(|group| group.indices.len() / 3)
        .sum();
    let draw_calls = geometry.meshes.len();
    let (meshes, instances) = encode_geometry(geometry);
    let textures = encode_textures(&crate::bake::standard_set(
        budget.facade_texture_size,
        budget.ground_texture_size,
    ));

    let traffic = traffic::simulate(&network, signals.clone(), city.seed, budget.vehicles);
    let traffic_state = match &traffic {
        Some(sim) => traffic::initial_state(sim),
        None => TrafficState::default(),
    };
    let vehicles = traffic.as_ref().map(|sim| sim.agent_count()).unwrap_or(0);
    let signal_count = signals.len();

    let mut warnings = network.warnings.clone();
    if city.buildings.len() > budget.max_buildings {
        warnings.push(format!(
            "city has {} buildings; {} detail shells were dropped to stay inside the scene budget",
            city.buildings.len(),
            city.buildings.len() - budget.max_buildings
        ));
    }

    CityScene {
        version: 1,
        seed: city.seed,
        origin: [city.frame.origin.x_km, city.frame.origin.y_km],
        rotation_radians: city.frame.rotation_radians,
        extent_m: extent_of(&network),
        meshes,
        instances,
        textures,
        signals,
        traffic: traffic_state,
        network: network.record(),
        stats: SceneStats {
            buildings: buildings.len(),
            trees: prototypes.len(),
            tree_instances: tree_output.instances,
            tufts: tree_output.tufts,
            parked_cars: parked,
            vehicles,
            roads: network.roads.len(),
            lanes: network.lanes.len(),
            connectors: network.connectors.len(),
            junctions: network.junctions.len(),
            signals: signal_count,
            bollards,
            lamps,
            poles,
            railings,
            signs,
            shelters,
            shrubs,
            wire_spans: spans,
            draw_calls,
            vertices,
            triangles,
            warnings,
            tree_roles: tree_output.by_role,
        },
    }
}

/// A tiny deterministic jitter source for callers that want to vary the scene
/// without regenerating the city.
pub fn scene_variant(seed: u32) -> u32 {
    let mut rng = Rng::new(seed);
    rng.next_u32()
}

// --- binary container -------------------------------------------------------
//
// The JSON form above is what tests, `dump_scene` and the audit harnesses read.
// The desktop app uses this one: a city is tens of megabytes of vertex data, and
// as JSON it costs a base64 expansion (+33 %), a multi-second `JSON.parse` of a
// string that big, and a base64 decode of every buffer — all on the webview's
// main thread. As a binary container the webview receives one `ArrayBuffer`,
// parses only a small header, and makes typed-array *views* into it: no copies.
//
// Layout (little-endian):
//
//     b"CSB1"            magic
//     u32                header length in bytes (unpadded)
//     header JSON        the scene with each buffer replaced by [offset, bytes]
//     zero padding       to a multiple of 4
//     blobs              each 4-byte aligned; offsets are from the first blob
//
// Normals are stored as signed-normalised bytes (`round(n * 127)`, three per
// vertex) instead of `f32`, cutting the second-largest buffer to a quarter; the
// mesh entry says so with `normalsSnorm: true`. Everything else is verbatim.
// `network` (lane graph for tooling) is not sent: no renderer reads it.

struct BlobWriter {
    data: Vec<u8>,
}

impl BlobWriter {
    fn push(&mut self, bytes: &[u8]) -> [usize; 2] {
        while self.data.len() % 4 != 0 {
            self.data.push(0);
        }
        let offset = self.data.len();
        self.data.extend_from_slice(bytes);
        [offset, bytes.len()]
    }
}

fn decode_b64(text: &str) -> Result<Vec<u8>, String> {
    STANDARD.decode(text).map_err(|error| format!("scene buffer is not base64: {error}"))
}

/// Serialise a scene into the binary container described above.
pub fn encode_binary(scene: &CityScene) -> Result<Vec<u8>, String> {
    use serde_json::{Value, json};

    let mut blobs = BlobWriter { data: Vec::new() };
    let mut meshes: Vec<Value> = Vec::with_capacity(scene.meshes.len());
    for mesh in &scene.meshes {
        let positions = blobs.push(&decode_b64(&mesh.positions)?);
        let normal_bytes = decode_b64(&mesh.normals)?;
        let snorm: Vec<u8> = normal_bytes
            .chunks_exact(4)
            .map(|chunk| {
                let value = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                ((value.clamp(-1.0, 1.0) * 127.0).round() as i8) as u8
            })
            .collect();
        let normals = blobs.push(&snorm);
        let mut entry = json!({
            "material": mesh.material,
            "positions": positions,
            "normals": normals,
            "normalsSnorm": true,
            "vertexCount": mesh.vertex_count,
            "triangleCount": mesh.triangle_count,
            "castShadow": mesh.cast_shadow,
            "receiveShadow": mesh.receive_shadow,
            "alphaCutout": mesh.alpha_cutout,
            "dynamic": mesh.dynamic,
        });
        if let Some(colors) = &mesh.colors {
            entry["colors"] = json!(blobs.push(&decode_b64(colors)?));
        }
        if let Some(uvs) = &mesh.uvs {
            entry["uvs"] = json!(blobs.push(&decode_b64(uvs)?));
        }
        entry["indices"] = json!(blobs.push(&decode_b64(&mesh.indices)?));
        if let Some(key) = &mesh.instance_of {
            entry["instanceOf"] = json!(key);
        }
        meshes.push(entry);
    }
    let mut instances: Vec<Value> = Vec::with_capacity(scene.instances.len());
    for list in &scene.instances {
        instances.push(json!({
            "key": list.key,
            "count": list.count,
            "data": blobs.push(&decode_b64(&list.data)?),
        }));
    }
    let mut textures: Vec<Value> = Vec::with_capacity(scene.textures.len());
    for texture in &scene.textures {
        textures.push(json!({
            "name": texture.name,
            "width": texture.width,
            "height": texture.height,
            "tileWidthM": texture.tile_width_m,
            "tileHeightM": texture.tile_height_m,
            "hasNormalSource": texture.has_normal_source,
            "data": blobs.push(&decode_b64(&texture.data)?),
        }));
    }
    let to_value = |value: Result<Value, serde_json::Error>| value.map_err(|error| error.to_string());
    let header = json!({
        "version": scene.version,
        "seed": scene.seed,
        "origin": scene.origin,
        "rotationRadians": scene.rotation_radians,
        "extentM": scene.extent_m,
        "meshes": meshes,
        "instances": instances,
        "textures": textures,
        "signals": to_value(serde_json::to_value(&scene.signals))?,
        "traffic": to_value(serde_json::to_value(&scene.traffic))?,
        "stats": to_value(serde_json::to_value(&scene.stats))?,
    });
    let header_bytes = serde_json::to_vec(&header).map_err(|error| error.to_string())?;
    let mut out = Vec::with_capacity(12 + header_bytes.len() + blobs.data.len());
    out.extend_from_slice(b"CSB1");
    out.extend_from_slice(&(header_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(&header_bytes);
    while out.len() % 4 != 0 {
        out.push(0);
    }
    out.extend_from_slice(&blobs.data);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use urban::{ModernChinaSpec, generate_modern_chinese_city};

    fn city() -> urban::ModernCity {
        generate_modern_chinese_city(ModernChinaSpec {
            seed: 42,
            radius_km: 0.5,
            block_size_metres: 110.0,
            ..ModernChinaSpec::default()
        })
    }

    #[test]
    fn a_city_scene_carries_everything_a_renderer_needs() {
        let scene = build_city_scene(&city(), SceneBudget::default());
        assert!(!scene.meshes.is_empty());
        assert!(!scene.instances.is_empty());
        assert!(!scene.textures.is_empty());
        assert!(!scene.signals.is_empty());
        assert!(!scene.traffic.agents.is_empty());
        assert!(scene.network.junctions.iter().any(|j| j.ring.len() >= 27));
        assert!(scene.network.connectors.len() > 100);
        assert!(scene.stats.trees >= 10);
        assert!(scene.stats.tree_instances > 500);
        assert!(scene.stats.wire_spans > 5);
        assert!(scene.extent_m[2] > scene.extent_m[0]);
    }

    #[test]
    /**
     * The regression test for the defect that made every textured surface in the
     * city render untextured.
     *
     * `MeshBuilder::quad_uv` pushes UVs unconditionally while `MeshBuilder::quad`
     * pushes them only when asked. Any material that received both ended up with
     * fewer UVs than vertices, so the renderer attributed the UVs it *did* have to
     * the wrong vertices — the vertical-streak facade, reproduced exactly.
     *
     * It is worth being precise about why this hid for so long. Every count in
     * the payload was plausible, the base64 decoded cleanly, no index ran off the
     * end of a buffer, and nothing threw. The scene simply rendered as flat
     * colour, which looks like a *material* problem rather than a *geometry*
     * problem, so it was investigated in the texture bakes for a long time before
     * anyone compared a UV array's length to its vertex count.
     */
    #[test]
    fn a_uv_buffer_is_never_shorter_than_its_vertex_count() {
        let scene = build_city_scene(&city(), SceneBudget::default());
        let mut short: Vec<String> = Vec::new();
        for mesh in &scene.meshes {
            let Some(uvs) = mesh.uvs.as_deref() else { continue };
            let expected = mesh.vertex_count * 2;
            if STANDARD.decode(uvs).map(|bytes| bytes.len() / 4) != Ok(expected) {
                short.push(mesh.material.clone());
            }
        }
        assert!(
            short.is_empty(),
            "these materials have a short UV buffer and render untextured: {}",
            short.join(", ")
        );
    }

    /**
     * The same invariant for vertex colours, which the building layer had the
     * same class of bug with.
     */
    #[test]
    fn a_colour_buffer_is_never_shorter_than_its_vertex_count() {
        let scene = build_city_scene(&city(), SceneBudget::default());
        for mesh in &scene.meshes {
            let Some(colors) = mesh.colors.as_deref() else { continue };
            assert_eq!(
                STANDARD.decode(colors).map(|bytes| bytes.len()),
                Ok(mesh.vertex_count * 4),
                "{} has a short colour buffer",
                mesh.material
            );
        }
    }

    #[test]
    fn every_mesh_carries_a_decodable_buffer() {
        let scene = build_city_scene(&city(), SceneBudget::default());
        for mesh in &scene.meshes {
            let positions = STANDARD
                .decode(&mesh.positions)
                .unwrap_or_else(|_| panic!("{} positions are not base64", mesh.material));
            assert_eq!(positions.len(), mesh.vertex_count * 12);
            let indices = STANDARD
                .decode(&mesh.indices)
                .unwrap_or_else(|_| panic!("{} indices are not base64", mesh.material));
            assert_eq!(indices.len(), mesh.triangle_count * 12);
            // Every index must address a real vertex, or the renderer silently
            // drops triangles.
            let count = mesh.vertex_count as u32;
            for chunk in indices.chunks_exact(4) {
                let value = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                assert!(value < count, "{} indexes past its buffer", mesh.material);
            }
        }
    }

    #[test]
    fn instanced_groups_reference_a_known_instance_list() {
        let scene = build_city_scene(&city(), SceneBudget::default());
        let keys: Vec<&str> = scene.instances.iter().map(|list| list.key.as_str()).collect();
        // A prototype this city happens not to use may legitimately have no
        // instances — a small city has no bus shelter.  What must hold is that
        // every *populated* list is consumed by some geometry, and that no
        // geometry claims an instance list which does not exist.
        for list in &scene.instances {
            if list.count == 0 {
                // A prototype this city does not use.  It must still be present
                // so the renderer's `instanceOf` lookup never misses.
                assert!(
                    scene
                        .meshes
                        .iter()
                        .any(|mesh| mesh.instance_of.as_deref() == Some(list.key.as_str())),
                    "{} has no geometry at all",
                    list.key
                );
                continue;
            }
            assert!(
                scene
                    .meshes
                    .iter()
                    .any(|mesh| mesh.instance_of.as_deref() == Some(list.key.as_str())),
                "{} has instances but nothing draws them",
                list.key
            );
            let bytes = STANDARD.decode(&list.data).unwrap();
            assert_eq!(bytes.len(), list.count * 40);
        }
        for mesh in &scene.meshes {
            if let Some(key) = &mesh.instance_of {
                assert!(keys.contains(&key.as_str()), "{key} is not a known list");
            }
        }
        // The whole point of the instancing design: trees are the bulk of the
        // city and must ride on a handful of lists.
        let tree_lists = keys.iter().filter(|key| key.starts_with("tree/")).count();
        assert!(tree_lists >= 8, "only {tree_lists} tree prototypes");
    }

    #[test]
    fn the_scene_stays_inside_a_sane_draw_call_budget() {
        let scene = build_city_scene(&city(), SceneBudget::default());
        // One group per material, plus one per instanced prototype.  A city that
        // needs hundreds of draws has lost its batching somewhere.
        assert!(
            scene.meshes.len() < 80,
            "the city needs {} draw calls",
            scene.meshes.len()
        );
    }

    #[test]
    fn the_building_budget_drops_detail_not_shells() {
        let mut budget = SceneBudget::default();
        budget.max_buildings = 40;
        let scene = build_city_scene(&city(), budget);
        assert_eq!(scene.stats.buildings, 40);
        assert!(
            scene
                .stats
                .warnings
                .iter()
                .any(|warning| warning.contains("scene budget")),
            "dropping buildings must be reported, not hidden"
        );
    }

    #[test]
    fn generation_is_deterministic() {
        let city = city();
        let first = build_city_scene(&city, SceneBudget::default());
        let second = build_city_scene(&city, SceneBudget::default());
        assert_eq!(first.meshes.len(), second.meshes.len());
        for (a, b) in first.meshes.iter().zip(second.meshes.iter()) {
            assert_eq!(a.material, b.material);
            assert_eq!(a.positions, b.positions);
        }
        for (a, b) in first.instances.iter().zip(second.instances.iter()) {
            assert_eq!(a.key, b.key);
            assert_eq!(a.data, b.data);
        }
    }
}

#[cfg(test)]
mod binary_tests {
    use super::*;
    use urban::{ModernChinaSpec, generate_modern_chinese_city};

    /// The binary container must describe exactly the buffers the JSON form
    /// does: every reference in bounds, aligned, and the right size for its
    /// vertex and triangle counts.
    #[test]
    fn the_binary_container_matches_the_json_scene() {
        let city = generate_modern_chinese_city(ModernChinaSpec {
            seed: 7,
            radius_km: 0.3,
            block_size_metres: 110.0,
            ..ModernChinaSpec::default()
        });
        let scene = build_city_scene(&city, SceneBudget::default());
        let bytes = encode_binary(&scene).expect("encodes");
        assert_eq!(&bytes[0..4], b"CSB1");
        let header_len = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
        let header: serde_json::Value = serde_json::from_slice(&bytes[8..8 + header_len]).unwrap();
        let base = (8 + header_len).div_ceil(4) * 4;
        let region = bytes.len() - base;
        let check = |reference: &serde_json::Value, expected_bytes: usize| {
            let offset = reference[0].as_u64().unwrap() as usize;
            let length = reference[1].as_u64().unwrap() as usize;
            assert_eq!(offset % 4, 0, "blob is 4-byte aligned");
            assert!(offset + length <= region, "blob is inside the file");
            assert_eq!(length, expected_bytes);
        };
        let meshes = header["meshes"].as_array().unwrap();
        assert_eq!(meshes.len(), scene.meshes.len());
        for (entry, mesh) in meshes.iter().zip(&scene.meshes) {
            check(&entry["positions"], mesh.vertex_count * 12);
            check(&entry["normals"], mesh.vertex_count * 3);
            check(&entry["indices"], mesh.triangle_count * 12);
            if mesh.colors.is_some() {
                check(&entry["colors"], mesh.vertex_count * 4);
            }
        }
        for (entry, texture) in header["textures"].as_array().unwrap().iter().zip(&scene.textures) {
            // Sized by what the JSON form carries, not by width x height.
            check(&entry["data"], STANDARD.decode(&texture.data).unwrap().len());
        }
        assert!(header.get("network").is_none());
    }
}
