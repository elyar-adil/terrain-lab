/**
 * The far and middle form of a city: every building as a plain extruded block,
 * every street as a flat ribbon, built from the same plan the metre-scale scene
 * is built from.
 *
 * Why this exists. The detailed scene is tens of megabytes and is streamed in
 * when the camera comes within a few kilometres, so before it arrived a city
 * was a coloured patch in the satellite texture and then, all at once, a city.
 * The plan (`modernCities`) is already in the world payload, so the silhouette
 * can be drawn from it at every zoom with no loading: the same footprints, the
 * same heights, the same street lines. The detailed scene then replaces the
 * mass in place and only surface detail changes, instead of a city appearing.
 *
 * One merged mesh per city, one draw call, flat-shaded vertex colours.
 */

import * as THREE from "three";

import type { ModernCityPayload } from "../types";

export interface CityMassFrame {
  /** World kilometres (the plan's frame) to scene x/z. */
  toScene(xKm: number, yKm: number): [number, number];
  /** Terrain height in scene units. */
  heightAt(x: number, z: number): number;
  metresToScene: number;
}

type Ring = Array<[number, number]>;

/** Deterministic 0..1 from an integer id, so a building keeps its colour. */
function hash01(id: number, salt: number): number {
  let h = (id * 0x9e3779b1) ^ (salt * 0x85ebca6b);
  h ^= h >>> 16;
  h = Math.imul(h, 0x7feb352d);
  h ^= h >>> 15;
  return (h >>> 0) / 4294967295;
}

// Facade and roof albedos (sRGB). Real walls are 0.3-0.6, never white.
const WALLS = [0xc9c3b6, 0xb9b2a4, 0xd2cdc1, 0xa8a398, 0xbfb59f, 0x9fa3a3];
const ROOFS = [0x6f6b66, 0x77736c, 0x5f5c59, 0x80766a];
const TERRACOTTA = 0x8a5a45;
const ASPHALT = 0x4a4c4f;
// Paving, plazas and forecourts between the buildings of a block.
const PAVING = 0x8b8982;

class MassBuilder {
  readonly positions: number[] = [];
  readonly normals: number[] = [];
  readonly colors: number[] = [];
  private readonly colour = new THREE.Color();

  triangles(): number {
    return this.positions.length / 9;
  }

  private push(a: number[], b: number[], c: number[], normal: number[], hex: number, shade: number) {
    this.colour.setHex(hex);
    for (const p of [a, b, c]) {
      this.positions.push(p[0], p[1], p[2]);
      this.normals.push(normal[0], normal[1], normal[2]);
      this.colors.push(this.colour.r * shade, this.colour.g * shade, this.colour.b * shade);
    }
  }

  /** A triangle whose winding is corrected to face `normal`. */
  tri(a: number[], b: number[], c: number[], normal: number[], hex: number, shade = 1) {
    const ux = b[0] - a[0], uy = b[1] - a[1], uz = b[2] - a[2];
    const vx = c[0] - a[0], vy = c[1] - a[1], vz = c[2] - a[2];
    const nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
    if (nx * normal[0] + ny * normal[1] + nz * normal[2] >= 0) this.push(a, b, c, normal, hex, shade);
    else this.push(a, c, b, normal, hex, shade);
  }

  /** A prism over `ring` from `bottom` to `top`, walls and flat roof. */
  prism(ring: Ring, bottom: number, top: number, wall: number, roof: number) {
    const n = ring.length;
    if (n < 3 || !(top > bottom)) return;
    let cx = 0, cz = 0;
    for (const [x, z] of ring) { cx += x; cz += z; }
    cx /= n; cz /= n;
    for (let i = 0; i < n; i += 1) {
      const [ax, az] = ring[i];
      const [bx, bz] = ring[(i + 1) % n];
      const ex = bx - ax, ez = bz - az;
      const len = Math.hypot(ex, ez);
      if (len < 1e-9) continue;
      let nx = ez / len, nz = -ex / len;
      if (nx * ((ax + bx) * 0.5 - cx) + nz * ((az + bz) * 0.5 - cz) < 0) { nx = -nx; nz = -nz; }
      const normal = [nx, 0, nz];
      // Light from one side reads the block's form at a distance.
      const shade = 0.82 + 0.18 * Math.max(0, nx * -0.5 + nz * -0.8);
      this.tri([ax, bottom, az], [bx, bottom, bz], [bx, top, bz], normal, wall, shade);
      this.tri([ax, bottom, az], [bx, top, bz], [ax, top, az], normal, wall, shade);
    }
    const contour = ring.map(([x, z]) => new THREE.Vector2(x, z));
    for (const [i, j, k] of THREE.ShapeUtils.triangulateShape(contour, [])) {
      this.tri([ring[i][0], top, ring[i][1]], [ring[j][0], top, ring[j][1]], [ring[k][0], top, ring[k][1]], [0, 1, 0], roof, 1);
    }
  }

  /** A flat polygon draped on the terrain, `lift` above it. */
  flat(ring: Ring, heightAt: (x: number, z: number) => number, lift: number, hex: number) {
    if (ring.length < 3) return;
    const contour = ring.map(([x, z]) => new THREE.Vector2(x, z));
    const at = (i: number) => [ring[i][0], heightAt(ring[i][0], ring[i][1]) + lift, ring[i][1]];
    for (const [i, j, k] of THREE.ShapeUtils.triangulateShape(contour, [])) {
      this.tri(at(i), at(j), at(k), [0, 1, 0], hex);
    }
  }

  /** A flat ribbon along a polyline. Heights are per point. */
  ribbon(points: Array<[number, number, number]>, width: number, hex: number) {
    for (let i = 0; i + 1 < points.length; i += 1) {
      const [ax, ay, az] = points[i];
      const [bx, by, bz] = points[i + 1];
      const ex = bx - ax, ez = bz - az;
      const len = Math.hypot(ex, ez);
      if (len < 1e-9) continue;
      const px = (-ez / len) * width * 0.5, pz = (ex / len) * width * 0.5;
      const a0 = [ax + px, ay, az + pz], a1 = [ax - px, ay, az - pz];
      const b0 = [bx + px, by, bz + pz], b1 = [bx - px, by, bz - pz];
      this.tri(a0, a1, b1, [0, 1, 0], hex);
      this.tri(a0, b1, b0, [0, 1, 0], hex);
    }
  }

  geometry(): THREE.BufferGeometry {
    const g = new THREE.BufferGeometry();
    g.setAttribute("position", new THREE.Float32BufferAttribute(this.positions, 3));
    g.setAttribute("normal", new THREE.Float32BufferAttribute(this.normals, 3));
    g.setAttribute("color", new THREE.Float32BufferAttribute(this.colors, 3));
    g.computeBoundingSphere();
    g.computeBoundingBox();
    return g;
  }
}

export interface CityMass {
  mesh: THREE.Mesh;
  triangles: number;
  dispose(): void;
}

export function buildCityMass(city: ModernCityPayload, frame: CityMassFrame): CityMass {
  const m = frame.metresToScene;
  const builder = new MassBuilder();
  const toRing = (points: Array<{ x_km: number; y_km: number }>): Ring =>
    points.map((p) => frame.toScene(p.x_km, p.y_km));

  // The ground of every block, so buildings stand on paving rather than on the
  // meadow the terrain texture shows.
  for (const block of city.blocks) builder.flat(toRing(block.boundary), frame.heightAt, 0.15 * m, PAVING);

  // Streets next, as flat ribbons a little above the ground. The metre-scale
  // scene draws the real carriageway; this is its line from afar.
  for (const road of city.hdRoads) {
    const pts = road.centreline.map((p) => {
      const [x, z] = frame.toScene(p.x_km, p.y_km);
      return [x, frame.heightAt(x, z), z] as [number, number, number];
    });
    if (pts.length < 2) continue;
    if (road.bridge) {
      // A deck runs straight between its abutments, not down into the water.
      const a = pts[0], b = pts[pts.length - 1];
      pts.forEach((p, i) => { p[1] = a[1] + (b[1] - a[1]) * (i / (pts.length - 1)); });
    }
    for (const p of pts) p[1] += 0.35 * m;
    builder.ribbon(pts, Math.max(road.widthMetres, 6) * m, ASPHALT);
  }

  for (const b of city.buildings) {
    const ring = toRing(b.footprint);
    if (ring.length < 3) continue;
    let base = Infinity;
    for (const [x, z] of ring) base = Math.min(base, frame.heightAt(x, z));
    // Sink the foot below the lowest corner so a building on a slope has no gap.
    const foot = base - 1.5 * m;
    const total = Math.max(b.heightMetres, 3) * m;
    const wall = WALLS[Math.floor(hash01(b.id, 1) * WALLS.length)];
    const roof = b.roof === "terracotta" ? TERRACOTTA : ROOFS[Math.floor(hash01(b.id, 2) * ROOFS.length)];
    const podium = b.podiumHeightMetres ?? 0;
    if (b.tierRing && b.tierRing.length >= 3 && podium > 0 && podium * m < total) {
      builder.prism(ring, foot, base + podium * m, wall, roof);
      builder.prism(toRing(b.tierRing), base + podium * m, base + total, wall, roof);
    } else {
      builder.prism(ring, foot, base + total, wall, roof);
    }
  }

  const geometry = builder.geometry();
  const material = new THREE.MeshStandardMaterial({
    vertexColors: true,
    roughness: 0.92,
    metalness: 0,
    // Streets sit a few decimetres above terrain that is kilometres from the
    // camera; without an offset they z-fight with it.
    polygonOffset: true,
    polygonOffsetFactor: -2,
    polygonOffsetUnits: -2,
  });
  const mesh = new THREE.Mesh(geometry, material);
  mesh.name = "cityMass";
  mesh.castShadow = false;
  mesh.receiveShadow = false;
  return {
    mesh,
    triangles: builder.triangles(),
    dispose() {
      geometry.dispose();
      material.dispose();
    },
  };
}
