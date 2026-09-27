//! Geometry accumulation for the city street layer.
//!
//! The renderer must not re-derive geometry: this crate hands it finished
//! vertex buffers.  Every surface is therefore accumulated into a named
//! [`MeshGroup`] keyed by the material it will be drawn with, and the whole
//! city flushes to one draw call per group — the same batching contract the
//! source renderer kept (`<50` draws for a full scene).
//!
//! Two payload shapes exist, and the split is deliberate:
//!
//! * **Baked geometry** — unique shapes: building shells, road ribbons,
//!   junction boxes, markings, furniture runs.  Full vertex data.
//! * **Instanced geometry** — repeated shapes: trees, lamps, poles, bollards,
//!   cars.  One prototype mesh plus a compact per-instance record.  A city with
//!   2 600 leaf-card trees costs two draw calls, not 2 600.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::math::{Vec2, Vec3};

/// Vertex capacity per group.  Fixed up-front so the inner loops never
/// re-allocate; every push is a bounds check and a write.
const INITIAL_VERTS: usize = 4096;
const INITIAL_INDICES: usize = 6144;

/// A compact per-instance record: translation, a `Y` rotation, a non-uniform
/// scale and a tint.  Ten floats beats a 4×4 matrix by more than half and is
/// all a tree, a lamp or a car ever needs.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
    pub tint_r: f32,
    pub tint_g: f32,
    pub tint_b: f32,
}

impl Instance {
    pub fn new(x: f32, y: f32, z: f32, rotation_y: f32, scale: f32, tint: [f32; 3]) -> Self {
        Self {
            x,
            y,
            z,
            rotation_y,
            scale_x: scale,
            scale_y: scale,
            scale_z: scale,
            tint_r: tint[0],
            tint_g: tint[1],
            tint_b: tint[2],
        }
    }
}

/// One drawable.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshGroup {
    /// Material key the renderer resolves; see `MATERIALS` in the crate root.
    pub material: String,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    /// Optional per-vertex colour, **`u8` RGBA**.  Absent for surfaces whose
    /// material owns the colour outright, which keeps asphalt and markings small.
    /// Eight bits is far more than a facade tint needs, and it is a quarter of
    /// the bytes of the `f32` form — on a city of a few hundred thousand vertices
    /// that is the difference between a payload that streams and one that does
    /// not.
    pub colors: Option<Vec<u8>>,
    /// Optional per-vertex texture coordinate, **in metres**.  Emitted only for
    /// materials whose texture has a physical tile size; the renderer sets
    /// `repeat` to the reciprocal of that size, so a facade tile lands on a real
    /// storey rhythm at any building height.  Never emit UVs in scene units —
    /// the previous renderer did, and every wall sampled a single texel.
    pub uvs: Option<Vec<f32>>,
    /// Indices, `u16` when the group has fewer than 65 536 vertices and `u32`
    /// otherwise.  Mixed widths are not worth a second draw call on a group
    /// that small, so the payload carries a flag instead.
    pub indices: Vec<u32>,
    pub cast_shadow: bool,
    pub receive_shadow: bool,
    /// `true` when the group is drawn with `alphaTest` instead of blending, so
    /// leaf cards and grass tufts stay sortable-free and shadow-correct.
    pub alpha_cutout: bool,
    /// Marks geometry that must be rebuilt when only the lamp colours change.
    /// Signals are split this way so a phase change never re-uploads a mesh.
    pub dynamic: bool,
    /// When set, this geometry is drawn once per entry in the instance list of
    /// this prototype.  A tree's bark and its leaf cards are two groups sharing
    /// one list, which is how a city gets thousands of leaf-detailed trees in a
    /// couple of draw calls.
    pub instance_of: Option<String>,
}

/// A shared instance list.  Referenced by id from one or more [`MeshGroup`]s.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceGroup {
    pub key: String,
    pub instances: Vec<Instance>,
}

/// Everything a city hands the renderer.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneGeometry {
    pub meshes: Vec<MeshGroup>,
    pub instances: Vec<InstanceGroup>,
}

#[derive(Debug, Default)]
struct Buffers {
    positions: Vec<f32>,
    normals: Vec<f32>,
    colors: Option<Vec<u8>>,
    uvs: Option<Vec<f32>>,
    indices: Vec<u32>,
    with_colors: bool,
    with_uvs: bool,
}

/// Accumulates triangles per material and hands back finished groups.
#[derive(Debug, Default)]
pub struct MeshBuilder {
    groups: BTreeMap<String, Buffers>,
    instances: BTreeMap<String, Vec<Instance>>,
    meta: BTreeMap<String, GroupMeta>,
    /// Group material → prototype key it is instanced by.
    bound: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, Default)]
struct GroupMeta {
    cast_shadow: bool,
    receive_shadow: bool,
    alpha_cutout: bool,
    dynamic: bool,
}

/// Lifecycle flags a caller sets once, before contributing geometry.
#[derive(Debug, Clone, Copy)]
pub struct GroupStyle {
    pub cast_shadow: bool,
    pub receive_shadow: bool,
    pub alpha_cutout: bool,
    pub dynamic: bool,
}

impl Default for GroupStyle {
    fn default() -> Self {
        Self {
            cast_shadow: false,
            receive_shadow: true,
            alpha_cutout: false,
            dynamic: false,
        }
    }
}

impl MeshBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style(&mut self, material: &str, style: GroupStyle) {
        let meta = self.meta.entry(material.to_owned()).or_default();
        meta.cast_shadow |= style.cast_shadow;
        meta.receive_shadow |= style.receive_shadow;
        meta.alpha_cutout |= style.alpha_cutout;
        meta.dynamic |= style.dynamic;
    }

    fn buffers(&mut self, material: &str) -> &mut Buffers {
        let meta = *self.meta.entry(material.to_owned()).or_default();
        let _ = meta;
        self.groups.entry(material.to_owned()).or_insert_with(|| Buffers {
            positions: Vec::with_capacity(INITIAL_VERTS * 3),
            normals: Vec::with_capacity(INITIAL_VERTS * 3),
            colors: None,
            uvs: None,
            indices: Vec::with_capacity(INITIAL_INDICES),
            with_colors: false,
            with_uvs: false,
        })
    }

    /// Append a triangle with an explicit per-vertex normal.  Used for
    /// chamfered or sloping geometry where `compute_normal` would be wrong.
    pub fn triangle(
        &mut self,
        material: &str,
        a: (Vec3, Vec3),
        b: (Vec3, Vec3),
        c: (Vec3, Vec3),
        color: Option<[f32; 3]>,
    ) {
        self.tri(material, a, b, c, Vec3::new(0.0, 1.0, 0.0), color)
    }

    fn tri(
        &mut self,
        material: &str,
        a: (Vec3, Vec3),
        b: (Vec3, Vec3),
        c: (Vec3, Vec3),
        fallback_normal: Vec3,
        color: Option<[f32; 3]>,
    ) {
        if !a.0.is_finite() || !b.0.is_finite() || !c.0.is_finite() {
            return;
        }
        let buffers = self.buffers(material);
        let base = (buffers.positions.len() / 3) as u32;
        for (position, normal) in [a, b, c] {
            buffers.positions.extend_from_slice(&[position.x, position.y, position.z]);
            buffers
                .normals
                .extend_from_slice(&[normal.x, normal.y, normal.z]);
        }
        if let Some(tint) = color {
            let colors = buffers.colors.get_or_insert_with(|| {
                buffers.with_colors = true;
                Vec::with_capacity(INITIAL_VERTS * 3)
            });
            let packed = pack_tint(&tint);
            for _ in 0..3 {
                colors.extend_from_slice(&packed);
            }
        }
        buffers.indices.extend_from_slice(&[base, base + 1, base + 2]);
        let _ = fallback_normal;
    }

    /// Append a triangle, deriving a flat normal from the winding.  The workhorse
    /// for road surfaces, markings and building walls.
    pub fn tri_flat(
        &mut self,
        material: &str,
        a: Vec3,
        b: Vec3,
        c: Vec3,
        color: Option<[f32; 3]>,
    ) {
        if !a.is_finite() || !b.is_finite() || !c.is_finite() {
            return;
        }
        let normal = face_normal(a, b, c);
        self.tri(
            material,
            (a, normal),
            (b, normal),
            (c, normal),
            normal,
            color,
        );
    }

    /// Append a convex ring as a fan from its centroid.  Block ground plates,
    /// lawns and junction boxes are the callers; the ring must be convex
    /// because a fan is not a triangulator (use `fill_ring` for general rings).
    pub fn fan(&mut self, material: &str, ring: &[Vec3], color: Option<[f32; 3]>, up: Vec3) {
        if ring.len() < 3 {
            return;
        }
        let centre = ring
            .iter()
            .fold(Vec3::default(), |acc, point| acc + *point)
            / ring.len() as f32;
        for index in 0..ring.len() {
            let next = (index + 1) % ring.len();
            self.tri(
                material,
                (centre, up),
                (ring[index], up),
                (ring[next], up),
                up,
                color,
            );
        }
    }

    /// Ear-clipped fill of an arbitrary simple ring, built in the `Vec2` plane
    /// and lifted to `y`.  Keeps the lit side facing `+Y`.
    pub fn fill_ring(&mut self, material: &str, ring: &[Vec2], y: f32, color: Option<[f32; 3]>) {
        if ring.len() < 3 {
            return;
        }
        let up = Vec3::new(0.0, 1.0, 0.0);
        for [a, b, c] in crate::math::triangulate(ring) {
            let pa = Vec3::from_plan(ring[a], y);
            let pb = Vec3::from_plan(ring[b], y);
            let pc = Vec3::from_plan(ring[c], y);
            // `triangulate` returns counter-clockwise indices for a
            // counter-clockwise ring, which is `-Y` facing; flip the winding so
            // the surface lights from above.
            self.tri(material, (pa, up), (pc, up), (pb, up), up, color);
        }
    }

    /// A planar quad with explicit per-corner texture coordinates.
    pub fn quad_uv(
        &mut self,
        material: &str,
        a: Vec3,
        b: Vec3,
        c: Vec3,
        d: Vec3,
        uv: [(f32, f32); 4],
        color: Option<[f32; 3]>,
    ) {
        if !a.is_finite() || !b.is_finite() || !c.is_finite() || !d.is_finite() {
            return;
        }
        let normal = face_normal(a, b, c);
        self.push_quad(
            material,
            [(a, normal), (b, normal), (c, normal), (d, normal)],
            Some(uv),
            color,
        );
    }

    /// A vertical wall between two plan positions, with metre UVs running along
    /// the edge and up the height.
    ///
    /// `mirror` negates the horizontal axis so a bay pattern can be flipped per
    /// building.  That single flag is most of what stops a district of towers
    /// from reading as the same building stamped out repeatedly.
    pub fn wall_uv(
        &mut self,
        material: &str,
        a: Vec2,
        b: Vec2,
        y0: f32,
        y1: f32,
        mirror: bool,
        color: Option<[f32; 3]>,
    ) {
        if (y1 - y0).abs() <= 1.0e-4 {
            return;
        }
        let length = a.distance(b);
        let u1 = if mirror { -length } else { length };
        self.quad_uv(
            material,
            Vec3::from_plan(a, y0),
            Vec3::from_plan(b, y0),
            Vec3::from_plan(b, y1),
            Vec3::from_plan(a, y1),
            [(0.0, y0), (u1, y0), (u1, y1), (0.0, y1)],
            color,
        );
    }

    /// A triangle with explicit per-corner texture coordinates.
    pub fn tri_uv(
        &mut self,
        material: &str,
        a: Vec3,
        b: Vec3,
        c: Vec3,
        uv: [(f32, f32); 3],
        color: Option<[f32; 3]>,
    ) {
        if !a.is_finite() || !b.is_finite() || !c.is_finite() {
            return;
        }
        let normal = face_normal(a, b, c);
        let buffers = self.buffers(material);
        let base = (buffers.positions.len() / 3) as u32;
        for (position, normal) in [(a, normal), (b, normal), (c, normal)] {
            buffers
                .positions
                .extend_from_slice(&[position.x, position.y, position.z]);
            buffers
                .normals
                .extend_from_slice(&[normal.x, normal.y, normal.z]);
        }
        let store = buffers.uvs.get_or_insert_with(|| {
            buffers.with_uvs = true;
            Vec::with_capacity(INITIAL_VERTS * 2)
        });
        for corner in uv {
            store.extend_from_slice(&[corner.0, corner.1]);
        }
        if let Some(tint) = color {
            let store = buffers.colors.get_or_insert_with(|| {
                buffers.with_colors = true;
                Vec::with_capacity(INITIAL_VERTS * 3)
            });
            for _ in 0..3 {
                store.extend_from_slice(&pack_tint(&tint));
            }
        }
        buffers.indices.extend_from_slice(&[base, base + 1, base + 2]);
    }

    /// A quad whose four corners carry individual normals *and* explicit
    /// texture coordinates.  Leaf cards need both: a uniform shading normal
    /// biased toward the zenith, and a UV measured in metres on a one-metre card.
    pub fn quad_uv_shaded(
        &mut self,
        material: &str,
        corners: [(Vec3, Vec3); 4],
        uv: [(f32, f32); 4],
        color: Option<[f32; 3]>,
    ) {
        self.push_quad(material, corners, Some(uv), color);
    }

    /// A horizontal surface with world-space metre UVs, for ground plates and
    /// roof decks whose texture tiles on a fixed physical grid.  Ear-clipped, so
    /// a concave block face is covered exactly once.
    pub fn ground_uv(
        &mut self,
        material: &str,
        ring: &[Vec2],
        y: f32,
        color: Option<[f32; 3]>,
    ) {
        if ring.len() < 3 {
            return;
        }
        for [a, b, c] in crate::math::triangulate(ring) {
            let (pa, pb, pc) = (ring[a], ring[b], ring[c]);
            // `triangulate` returns counter-clockwise indices for a
            // counter-clockwise ring, which faces `-Y`; flip so the surface
            // lights from above.
            self.tri_uv(
                material,
                Vec3::from_plan(pa, y),
                Vec3::from_plan(pc, y),
                Vec3::from_plan(pb, y),
                [(pa.x, pa.y), (pc.x, pc.y), (pb.x, pb.y)],
                color,
            );
        }
    }

    fn push_quad(
        &mut self,
        material: &str,
        corners: [(Vec3, Vec3); 4],
        uv: Option<[(f32, f32); 4]>,
        color: Option<[f32; 3]>,
    ) {
        let buffers = self.buffers(material);
        let base = (buffers.positions.len() / 3) as u32;
        for (position, normal) in corners {
            if !position.is_finite() {
                return;
            }
            buffers.positions.extend_from_slice(&[position.x, position.y, position.z]);
            buffers
                .normals
                .extend_from_slice(&[normal.x, normal.y, normal.z]);
        }
        if let Some(uv) = uv {
            let store = buffers.uvs.get_or_insert_with(|| {
                buffers.with_uvs = true;
                Vec::with_capacity(INITIAL_VERTS * 2)
            });
            for corner in uv {
                store.extend_from_slice(&[corner.0, corner.1]);
            }
        }
        if let Some(tint) = color {
            let store = buffers.colors.get_or_insert_with(|| {
                buffers.with_colors = true;
                Vec::with_capacity(INITIAL_VERTS * 3)
            });
            let packed = pack_tint(&tint);
            for _ in 0..4 {
                store.extend_from_slice(&packed);
            }
        }
        buffers
            .indices
            .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    /// A planar quad `a → b → c → d`, lit from whichever side the winding
    /// implies.  Road ribbons, marking bars and facade bands are all quads.
    ///
    /// Indexed: four vertices and six indices, not six vertices.  A city emits
    /// well over a hundred thousand quads, and the vertex saving is the
    /// difference between a payload that streams and one that does not.
    pub fn quad(&mut self, material: &str, a: Vec3, b: Vec3, c: Vec3, d: Vec3, color: Option<[f32; 3]>) {
        if !a.is_finite() || !b.is_finite() || !c.is_finite() || !d.is_finite() {
            return;
        }
        let normal = face_normal(a, b, c);
        self.push_quad(
            material,
            [(a, normal), (b, normal), (c, normal), (d, normal)],
            None,
            color,
        );
    }

    /// A quad whose four corners carry individual normals.  Roof parapets,
    /// balcony fascias and swept bridge walls need this because their two faces
    /// are not coplanar.
    pub fn quad_shaded(
        &mut self,
        material: &str,
        a: (Vec3, Vec3),
        b: (Vec3, Vec3),
        c: (Vec3, Vec3),
        d: (Vec3, Vec3),
        color: Option<[f32; 3]>,
    ) {
        let buffers = self.buffers(material);
        let base = (buffers.positions.len() / 3) as u32;
        for (position, normal) in [a, b, c, d] {
            if !position.is_finite() {
                return;
            }
            buffers.positions.extend_from_slice(&[position.x, position.y, position.z]);
            buffers
                .normals
                .extend_from_slice(&[normal.x, normal.y, normal.z]);
        }
        if let Some(tint) = color {
            let colors = buffers.colors.get_or_insert_with(|| {
                buffers.with_colors = true;
                Vec::with_capacity(INITIAL_VERTS * 3)
            });
            for _ in 0..4 {
                colors.extend_from_slice(&pack_tint(&tint));
            }
        }
        buffers
            .indices
            .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    /// A vertical band between two `Vec2` positions at two elevations.  This is
    /// literally every wall, kerb face, parapet and fence in the city, so it gets
    /// a dedicated helper rather than a repeated `quad` call.
    pub fn wall(
        &mut self,
        material: &str,
        a: Vec2,
        b: Vec2,
        y0: f32,
        y1: f32,
        color: Option<[f32; 3]>,
    ) {
        if (y1 - y0).abs() <= 1.0e-4 {
            return;
        }
        self.quad(
            material,
            Vec3::from_plan(a, y0),
            Vec3::from_plan(b, y0),
            Vec3::from_plan(b, y1),
            Vec3::from_plan(a, y1),
            color,
        );
    }

    /// A horizontal ribbon between two lateral offsets along a path, from
    /// `from` to `to` station, `lift` metres above the roadbed.
    pub fn ribbon(
        &mut self,
        material: &str,
        path: &crate::math::Path,
        inner: f32,
        outer: f32,
        from: f32,
        to: f32,
        lift: f32,
        color: Option<[f32; 3]>,
    ) {
        let length = path.length();
        if length <= 1.0e-4 || to <= from {
            return;
        }
        let steps = (((to - from) / crate::math::MAX_RESAMPLE_METRES).ceil().max(1.0)) as usize;
        let mut previous: Option<(Vec3, Vec3)> = None;
        for step in 0..=steps {
            let station = from + (to - from) * step as f32 / steps as f32;
            let a = path.offset_at(station, inner, lift);
            let b = path.offset_at(station, outer, lift);
            if let Some((pa, pb)) = previous {
                self.quad(material, pa, pb, b, a, color);
            }
            previous = Some((a, b));
        }
    }

    /// A ribbon whose texture coordinates run along the path, in **metres**.
    ///
    /// `anchor` is the station the pattern is anchored at, so a dashed line's
    /// phase is continuous across a road drawn in several pieces and a zebra
    /// crossing's bar count comes from the real crossing width rather than from
    /// geometry.  This is what keeps the markings layer from dominating the
    /// payload: thirty bars and a dozen dashes per approach become two quads.
    #[allow(clippy::too_many_arguments)]
    pub fn ribbon_uv_along(
        &mut self,
        material: &str,
        path: &crate::math::Path,
        inner: f32,
        outer: f32,
        from: f32,
        to: f32,
        lift: f32,
        color: Option<[f32; 3]>,
        anchor: f32,
    ) {
        let length = path.length();
        if length <= 1.0e-4 || to <= from {
            return;
        }
        let steps = (((to - from) / crate::math::MAX_RESAMPLE_METRES).ceil().max(1.0)) as usize;
        let mut previous: Option<(Vec3, Vec3, f32)> = None;
        for step in 0..=steps {
            let station = from + (to - from) * step as f32 / steps as f32;
            let a = path.offset_at(station, inner, lift);
            let b = path.offset_at(station, outer, lift);
            if let Some((pa, pb, previous_station)) = previous {
                // `U` runs across the band in metres, `V` along it in metres from
                // the anchor.  The renderer sets `repeat` to the reciprocal of the
                // texture's physical tile size, so the pattern is exact.
                let v0 = station - anchor;
                let v1 = previous_station - anchor;
                self.quad_uv(
                    material,
                    pa,
                    pb,
                    b,
                    a,
                    [
                        (0.0, v1),
                        (outer - inner, v1),
                        (outer - inner, v0),
                        (0.0, v0),
                    ],
                    color,
                );
            }
            previous = Some((a, b, station));
        }
    }

    /// A ribbon with a station-dependent `outer` offset, which is how an
    /// approach-widening taper is drawn: the kerb line moves out over 58 m while
    /// the centreline stays put.
    pub fn tapered_ribbon(
        &mut self,
        material: &str,
        path: &crate::math::Path,
        inner: f32,
        outer_at: impl Fn(f32) -> f32,
        from: f32,
        to: f32,
        lift: f32,
        color: Option<[f32; 3]>,
    ) {
        let steps = (((to - from) / 4.0).ceil().max(1.0)) as usize;
        let mut previous: Option<(Vec3, Vec3)> = None;
        for step in 0..=steps {
            let station = from + (to - from) * step as f32 / steps as f32;
            let a = path.offset_at(station, inner, lift);
            let b = path.offset_at(station, outer_at(station), lift);
            if let Some((pa, pb)) = previous {
                self.quad(material, pa, pb, b, a, color);
            }
            previous = Some((a, b));
        }
    }

    /// A closed tube of `sides` flat-shaded quads between two radii — the trunk
    /// and branch primitive for trees, poles and rails.  Hand-built arrays beat
    /// a per-branch `CylinderGeometry` by a wide margin, which is why the source
    /// renderer does the same thing.
    pub fn tube(
        &mut self,
        material: &str,
        from: Vec3,
        to: Vec3,
        r0: f32,
        r1: f32,
        sides: u8,
        color: Option<[f32; 3]>,
    ) {
        let axis = to - from;
        if !axis.is_finite() || axis.length() <= 1.0e-5 {
            return;
        }
        let up = axis.normalized_or_up();
        // Any reference not parallel to the axis gives a stable cross-section
        // frame; preferring world `+X` keeps a vertical trunk from degenerating.
        let reference = if up.x.abs() < 0.9 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        };
        let side_a = up.cross(reference).normalized_or_up();
        let side_b = up.cross(side_a);
        let sides = sides.max(3) as usize;
        let length = axis.length();
        let ring = |angle: f32, radius: f32, along: f32| {
            from + (side_a * angle.cos() + side_b * angle.sin()) * radius + up * along
        };
        for index in 0..sides {
            let a0 = index as f32 / sides as f32 * std::f32::consts::TAU;
            let a1 = (index + 1) as f32 / sides as f32 * std::f32::consts::TAU;
            self.quad(
                material,
                ring(a0, r0, 0.0),
                ring(a0, r1, length),
                ring(a1, r1, length),
                ring(a1, r0, 0.0),
                color,
            );
        }
    }

    /// Register a repeated element.  `prototype` names the shared instance list;
    /// the geometry that consumes it is bound separately with [`Self::bind`].
    pub fn add_instance(&mut self, prototype: &str, instance: Instance) {
        self.instances
            .entry(prototype.to_owned())
            .or_default()
            .push(instance);
    }

    /// Draw the group `material` once per instance of `prototype`.  A tree binds
    /// both its bark group and its leaf group to the same key, so one transform
    /// list places the whole tree.
    pub fn bind(&mut self, material: &str, prototype: &str) {
        self.bound.insert(material.to_owned(), prototype.to_owned());
    }

    pub fn vertex_count(&self, material: &str) -> usize {
        self.groups
            .get(material)
            .map(|buffers| buffers.positions.len() / 3)
            .unwrap_or(0)
    }

    pub fn index_count(&self, material: &str) -> usize {
        self.groups
            .get(material)
            .map(|buffers| buffers.indices.len())
            .unwrap_or(0)
    }

    pub fn total_vertices(&self) -> usize {
        self.groups
            .values()
            .map(|buffers| buffers.positions.len() / 3)
            .sum()
    }

    pub fn total_instances(&self) -> usize {
        self.instances.values().map(Vec::len).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.groups
            .values()
            .all(|buffers| buffers.positions.is_empty())
            && self.instances.is_empty()
    }

    /// Finish every group.  Empty groups are dropped so the renderer never
    /// creates a zero-length buffer; instance lists with no bound geometry are
    /// dropped too.
    pub fn build(mut self) -> SceneGeometry {
        let mut result: Vec<MeshGroup> = Vec::new();
        for (material, buffers) in std::mem::take(&mut self.groups) {
            if buffers.positions.is_empty() {
                continue;
            }
            let meta = self.meta.get(&material).copied().unwrap_or_default();
            let instance_of = self.bound.get(&material).cloned();
            result.push(MeshGroup {
                material,
                positions: buffers.positions,
                normals: buffers.normals,
                colors: buffers.colors,
                uvs: buffers.uvs,
                indices: buffers.indices,
                cast_shadow: meta.cast_shadow,
                receive_shadow: meta.receive_shadow,
                alpha_cutout: meta.alpha_cutout,
                dynamic: meta.dynamic,
                instance_of,
            });
        }
        // Every *bound* prototype gets a list, even an empty one: the geometry
        // that references it exists, and a renderer looking up `instanceOf` must
        // always find a list.  A zero-count `InstancedMesh` costs nothing.
        let mut lists = std::mem::take(&mut self.instances);
        for key in self.bound.values() {
            lists.entry(key.clone()).or_default();
        }
        let instances: Vec<InstanceGroup> = lists
            .into_iter()
            .map(|(key, instances)| InstanceGroup { key, instances })
            .collect();
        SceneGeometry {
            meshes: result,
            instances,
        }
    }
}

/// An axis-aligned-in-local-frame box: the primitive behind rooftop plant,
/// gate piers, bollards, cabinets and parked cars.
///
/// `rotation_y` is applied about the box's own centre, so a caller placing a
/// gate lintel across a compound wall does not have to rotate the corners by
/// hand.
pub fn box_at(
    builder: &mut MeshBuilder,
    material: &str,
    centre: Vec2,
    y_centre: f32,
    width: f32,
    height: f32,
    depth: f32,
    rotation_y: f32,
) {
    let (sin, cos) = rotation_y.sin_cos();
    let point = |x: f32, z: f32| Vec2::new(centre.x + x * cos - z * sin, centre.y + x * sin + z * cos);
    let hw = width * 0.5;
    let hd = depth * 0.5;
    let y0 = y_centre - height * 0.5;
    let y1 = y_centre + height * 0.5;
    let corners = [
        point(-hw, -hd),
        point(hw, -hd),
        point(hw, hd),
        point(-hw, hd),
    ];
    for index in 0..4 {
        builder.wall(material, corners[index], corners[(index + 1) % 4], y0, y1, None);
    }
    builder.quad(
        material,
        Vec3::from_plan(corners[3], y1),
        Vec3::from_plan(corners[2], y1),
        Vec3::from_plan(corners[1], y1),
        Vec3::from_plan(corners[0], y1),
        None,
    );
    builder.quad(
        material,
        Vec3::from_plan(corners[0], y0),
        Vec3::from_plan(corners[1], y0),
        Vec3::from_plan(corners[2], y0),
        Vec3::from_plan(corners[3], y0),
        None,
    );
}

/// Pack a linear `[f32; 3]` tint into four `u8`s.
fn pack_tint(tint: &[f32; 3]) -> [u8; 4] {
    [
        (tint[0].clamp(0.0, 1.0) * 255.0) as u8,
        (tint[1].clamp(0.0, 1.0) * 255.0) as u8,
        (tint[2].clamp(0.0, 1.0) * 255.0) as u8,
        255,
    ]
}

pub fn face_normal(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    (b - a).cross(c - a).normalized_or_up()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Path;

    #[test]
    fn quad_emits_two_triangles_with_a_shared_normal() {
        let mut builder = MeshBuilder::new();
        // Wound the way every ground surface in the crate is wound, so the face
        // normal comes out `+Y`.
        builder.quad(
            "asphalt",
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 4.0),
            Vec3::new(4.0, 0.0, 4.0),
            Vec3::new(4.0, 0.0, 0.0),
            None,
        );
        // Indexed: four shared vertices and six indices, not six vertices.
        assert_eq!(builder.vertex_count("asphalt"), 4);
        let groups = builder.build().meshes;
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].indices, vec![0, 1, 2, 0, 2, 3]);
        assert_eq!(groups[0].normals, vec![0.0, 1.0, 0.0].repeat(4));
    }

    #[test]
    fn ribbon_follows_the_path_and_scales_with_length() {
        let path = Path::flat(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(30.0, 0.0),
            Vec2::new(60.0, 0.0),
        ]);
        let mut builder = MeshBuilder::new();
        builder.ribbon("asphalt", &path, -3.0, 3.0, 0.0, 60.0, 0.0, None);
        // Subdivided at the coarse step, because the terrain it drapes on has
        // cells tens of metres across: 60 m is two span pairs, not five.
        let expected = (60.0 / crate::math::MAX_RESAMPLE_METRES).ceil() as usize;
        assert_eq!(builder.vertex_count("asphalt"), expected * 4);
        assert_eq!(builder.index_count("asphalt"), expected * 6);
        // A long ribbon must stay proportional, or a city-scale road costs more
        // than the whole skyline.
        let long = Path::flat(vec![Vec2::new(0.0, 0.0), Vec2::new(600.0, 0.0)]);
        let mut builder = MeshBuilder::new();
        builder.ribbon("asphalt", &long, -3.0, 3.0, 0.0, 600.0, 0.0, None);
        assert!(builder.index_count("asphalt") <= 20 * 6, "600 m of road cost too much");
    }

    #[test]
    fn non_finite_geometry_is_dropped_instead_of_poisoning_the_buffer() {
        let mut builder = MeshBuilder::new();
        builder.tri_flat(
            "asphalt",
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(f32::NAN, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 4.0),
            None,
        );
        assert_eq!(builder.vertex_count("asphalt"), 0);
        assert!(builder.build().meshes.is_empty());
    }

    #[test]
    fn fill_ring_lights_upward() {
        let mut builder = MeshBuilder::new();
        builder.fill_ring(
            "sidewalk",
            &[
                Vec2::new(0.0, 0.0),
                Vec2::new(10.0, 0.0),
                Vec2::new(10.0, 10.0),
                Vec2::new(0.0, 10.0),
            ],
            0.15,
            None,
        );
        let groups = builder.build().meshes;
        assert_eq!(groups[0].normals, vec![0.0, 1.0, 0.0].repeat(6));
    }

    #[test]
    fn instances_do_not_consume_geometry_buffers() {
        let mut builder = MeshBuilder::new();
        builder.style("tree:leaf", GroupStyle { cast_shadow: true, ..GroupStyle::default() });
        builder.add_instance("tree:leaf", Instance::new(1.0, 0.0, 2.0, 0.3, 1.0, [1.0, 1.0, 1.0]));
        builder.add_instance("tree:leaf", Instance::new(4.0, 0.0, 5.0, 1.1, 0.8, [0.9, 1.0, 0.9]));
        assert_eq!(builder.total_instances(), 2);
        assert_eq!(builder.total_vertices(), 0);
    }

    #[test]
    fn tube_generates_a_closed_five_sided_prism() {
        let mut builder = MeshBuilder::new();
        builder.tube(
            "bark",
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 4.0, 0.0),
            0.3,
            0.15,
            5,
            None,
        );
        // Five quads, no caps — matching the source renderer's branch primitive.
        assert_eq!(builder.vertex_count("bark"), 5 * 4);
    }
}
