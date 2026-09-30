/**
 * Distance LOD for a mounted city.
 *
 * A city is a few hundred draw calls but millions of triangles, and most of them
 * are things nobody can see from where the camera is: a bin is sub-pixel at
 * 300 m, a leaf card is sub-pixel at 700 m, and a shadow map stretched over a
 * whole district resolves nothing. Everything here is driven by the camera's
 * distance and costs nothing when the camera is still.
 *
 * - Instance lists (trees, hedges, furniture) are *compacted*: the instance
 *   buffer holds only the instances that pass their distance test, front-packed,
 *   and `mesh.count` is the number kept. The full list stays in `LodInstanceSet`.
 * - Tree crowns switch, per instance, from leaf cards to a solid core to a
 *   very coarse ellipsoid as they recede.
 * - Static pieces (already split into grid cells by the builder) are hidden past
 *   a per-material distance, measured to the cell's box, so a far cell that is
 *   only a few pixels wide is not drawn at all.
 * - Shadow casting degrades with view span, because the sun's shadow map covers
 *   a span proportional to it: small props, then trees, then everything stop
 *   casting as the map gets too coarse to show them.
 *
 * Nothing allocates per call.
 */

import * as THREE from "three";

import type { CityHandles, LodInstanceSet } from "./cityScene";

/** Distances are city-local metres. */
export const LOD = {
  /** Leaf cards and bark are drawn within this distance of a tree. */
  treeDetail: 700,
  /** Trees nearer than this show only their leaves; the solid core is hidden. */
  treeCoreNear: 150,
  /** Past this a crown core is replaced by the coarse ellipsoid. */
  treeCoreFar: 1100,
  hedge: 350,
  tuft: 180,
  /** Tall furniture: lamps, poles, gantries, guide signs. */
  tallProp: 380,
  /** Low furniture: bins, bollards, railings, kiosks, crossing signs. */
  lowProp: 160,
  /** Fine static detail: trim, wire, signs, signal housings, balconies, awnings. */
  fineStatic: 700,
  /** Flat street detail: kerb, markings, barriers, median planting. */
  flatStatic: 1500,
  /** The camera has to move this far before the lists are rebuilt... */
  moveEpsilon: 2.5,
  /** ...or turn this far (cosine of the angle). */
  turnCosine: 0.9997,
  /**
   * Instances outside the view frustum by more than this are dropped. The margin
   * keeps casters just outside the frame, whose shadows fall inside it.
   */
  frustumMarginM: 90,
  /** View spans (m) beyond which shadow casting is reduced. */
  shadowTrees: 700,
  shadowAll: 2200,
} as const;

const FINE = /^(trim\.|wire|sign\/|signal\.|awning|balcony)/;
const FLAT = /^(kerb|marking\.|barrier|median)/;

function instanceRadius(set: LodInstanceSet): number {
  const { key, part } = set;
  if (key.startsWith("tree/")) return Number.POSITIVE_INFINITY; // handled by part
  if (key === "hedge") return LOD.hedge;
  if (key === "tuft") return LOD.tuft;
  if (key.startsWith("furniture/")) {
    return /lamp|pole|gantry|sign\.guide|shelter/.test(key) ? LOD.tallProp : LOD.lowProp;
  }
  void part;
  return Number.POSITIVE_INFINITY;
}

function staticRadius(name: string): number {
  if (FINE.test(name)) return LOD.fineStatic;
  if (FLAT.test(name)) return LOD.flatStatic;
  return Number.POSITIVE_INFINITY;
}

interface StaticEntry {
  object: THREE.Mesh;
  box: THREE.Box3;
  radius: number;
  cast: boolean;
}

export class CityLod {
  private readonly statics: StaticEntry[] = [];
  private readonly sets: LodInstanceSet[];
  private readonly setRadius: number[];
  private readonly lastCamera = new THREE.Vector3(Number.NaN, 0, 0);
  private readonly lastForward = new THREE.Vector3(0, 0, -1);
  private frustum: THREE.Frustum | null = null;
  private shadowTier = -1;
  /** Counts of what the last rebuild kept, for the perf overlay. */
  keptInstances = 0;
  hiddenStatics = 0;

  constructor(private readonly handles: CityHandles) {
    for (const object of handles.statics) {
      const box = object.geometry.boundingBox ?? new THREE.Box3().setFromObject(object);
      this.statics.push({ object, box, radius: staticRadius(object.name), cast: object.castShadow });
    }
    this.sets = handles.lodSets;
    this.setRadius = this.sets.map(instanceRadius);
    for (const set of this.sets) set.mesh.userData.cast = set.mesh.castShadow;
  }

  /**
   * @param camera city-local camera position, metres
   * @param forward city-local view direction (unit)
   * @param frustum the view frustum in city-local metres, or null to skip view culling
   * @param viewSpanM approximate width of ground in view, metres
   * @returns true when shadow casters changed and the shadow map should refresh
   */
  update(camera: THREE.Vector3, forward: THREE.Vector3, frustum: THREE.Frustum | null, viewSpanM: number): boolean {
    const tier = viewSpanM < LOD.shadowTrees ? 0 : viewSpanM < LOD.shadowAll ? 1 : 2;
    let shadowChanged = false;
    if (tier !== this.shadowTier) {
      this.shadowTier = tier;
      shadowChanged = true;
      for (const entry of this.statics) entry.object.castShadow = entry.cast && tier < 2;
      for (const set of this.sets) set.mesh.castShadow = Boolean(set.mesh.userData.cast) && tier < 1;
    }
    const first = Number.isNaN(this.lastCamera.x);
    if (
      !first
      && camera.distanceToSquared(this.lastCamera) <= LOD.moveEpsilon * LOD.moveEpsilon
      && forward.dot(this.lastForward) >= LOD.turnCosine
    ) {
      return shadowChanged;
    }
    this.lastCamera.copy(camera);
    this.lastForward.copy(forward);
    this.frustum = frustum;
    this.rebuild(camera);
    return shadowChanged;
  }

  private rebuild(camera: THREE.Vector3): void {
    let hidden = 0;
    for (const entry of this.statics) {
      const visible = entry.radius === Number.POSITIVE_INFINITY
        || entry.box.distanceToPoint(camera) <= entry.radius;
      if (entry.object.visible !== visible) entry.object.visible = visible;
      if (!visible) hidden += 1;
    }
    this.hiddenStatics = hidden;

    let kept = 0;
    for (let s = 0; s < this.sets.length; s += 1) {
      const set = this.sets[s];
      const limit = this.setRadius[s];
      const isTree = set.key.startsWith("tree/");
      const isCore = isTree && set.part === "mass";
      const target = set.mesh;
      const dst = target.instanceMatrix.array as Float32Array;
      const dstColor = target.instanceColor ? (target.instanceColor.array as Float32Array) : null;
      const far = set.far;
      const farDst = far ? (far.instanceMatrix.array as Float32Array) : null;
      const farColor = far && far.instanceColor ? (far.instanceColor.array as Float32Array) : null;
      const bounds = set.bounds;
      const planes = this.frustum ? this.frustum.planes : null;
      const margin = LOD.frustumMarginM;
      const src = set.matrices;
      const tint = set.colors;
      let n = 0;
      let nf = 0;
      for (let i = 0; i < set.count; i += 1) {
        const b = i * 4;
        const dx = bounds[b] - camera.x;
        const dy = bounds[b + 1] - camera.y;
        const dz = bounds[b + 2] - camera.z;
        const d = Math.sqrt(dx * dx + dy * dy + dz * dz) - bounds[b + 3];
        if (planes) {
          let outside = false;
          for (let k = 0; k < 6; k += 1) {
            const plane = planes[k];
            const n = plane.normal;
            if (n.x * bounds[b] + n.y * bounds[b + 1] + n.z * bounds[b + 2] + plane.constant + margin + bounds[b + 3] < 0) {
              outside = true;
              break;
            }
          }
          if (outside) continue;
        }
        let toFar = false;
        if (isTree) {
          if (isCore) {
            if (d < LOD.treeCoreNear) continue;
            toFar = d >= LOD.treeCoreFar;
          } else if (d >= LOD.treeDetail) {
            continue;
          }
        } else if (d >= limit) {
          continue;
        }
        const from = i * 16;
        if (toFar && farDst) {
          const to = nf * 16;
          for (let k = 0; k < 16; k += 1) farDst[to + k] = src[from + k];
          if (farColor && tint) {
            farColor[nf * 3] = tint[i * 3];
            farColor[nf * 3 + 1] = tint[i * 3 + 1];
            farColor[nf * 3 + 2] = tint[i * 3 + 2];
          }
          nf += 1;
        } else {
          const to = n * 16;
          for (let k = 0; k < 16; k += 1) dst[to + k] = src[from + k];
          if (dstColor && tint) {
            dstColor[n * 3] = tint[i * 3];
            dstColor[n * 3 + 1] = tint[i * 3 + 1];
            dstColor[n * 3 + 2] = tint[i * 3 + 2];
          }
          n += 1;
        }
      }
      this.commit(target, n, dstColor !== null);
      if (far) this.commit(far, nf, farColor !== null);
      kept += n + nf;
    }
    this.keptInstances = kept;
  }

  private commit(mesh: THREE.InstancedMesh, n: number, colours: boolean): void {
    mesh.count = n;
    mesh.visible = n > 0;
    if (n === 0) return;
    mesh.instanceMatrix.clearUpdateRanges();
    mesh.instanceMatrix.addUpdateRange(0, n * 16);
    mesh.instanceMatrix.needsUpdate = true;
    if (colours && mesh.instanceColor) {
      mesh.instanceColor.clearUpdateRanges();
      mesh.instanceColor.addUpdateRange(0, n * 3);
      mesh.instanceColor.needsUpdate = true;
    }
    // Frustum culling reads the bounding sphere, so it has to describe the
    // instances that are actually in the buffer now.
    mesh.computeBoundingSphere();
  }
}
