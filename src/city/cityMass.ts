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
import { tileKey, tileOfPoint } from "./cityTiles";

/** Tiles the far form can hide. A city of more than this many tiles hides none past it. */
const MAX_MASS_TILES = 256;

export interface CityMassFrame {
  /** World kilometres (the plan's frame) to scene x/z. */
  toScene(xKm: number, yKm: number): [number, number];
  /** Terrain height in scene units. */
  heightAt(x: number, z: number): number;
  metresToScene: number;
}

type Ring = Array<[number, number]>;
type CityMassBuilding = ModernCityPayload["buildings"][number];

/** Deterministic 0..1 from an integer id, so a building keeps its colour. */
function hash01(id: number, salt: number): number {
  let h = (id * 0x9e3779b1) ^ (salt * 0x85ebca6b);
  h ^= h >>> 16;
  h = Math.imul(h, 0x7feb352d);
  h ^= h >>> 15;
  return (h >>> 0) / 4294967295;
}

// Red brick and red-brown tile, only on low and mid-rise blocks.
const BRICK = 0x9c5b4a;
// Facade and roof albedos (sRGB). Real walls are 0.3-0.6, never white.
const WALLS = [0xc9c3b6, 0xb9b2a4, 0xd2cdc1, 0xa8a398, 0xbfb59f, 0x9fa3a3, 0xd8cbb0, BRICK, 0xe6e4df];
// Office towers are glass and cladding: cooler, darker, and more often blue-grey.
const TOWER_WALLS = [0x5b6670, 0x6d7780, 0x8a939b, 0xb8bcc0, 0x4d5b66];
// Flat roofs seen from the air: cement screed and grey membrane, silver
// reflective membrane, the odd green-grey screed, and blue steel sheeting over a
// rooftop shed. Mostly light; the dark greys made every roof a hole.
const ROOFS = [0x9a978f, 0xa8a59d, 0x8c8a84, 0xb9b8b3, 0xc8c8c4, 0x8f948c, 0x6f8aa3];
// Profiled steel cladding and the blue or grey steel roofs of sheds.
const SHED_WALLS = [0xdcdedd, 0xc5c9cc, 0xaebccb, 0xd6cfbd, 0xbfc3bf];
const SHED_ROOFS = [0x5c7d9e, 0x4f6f8f, 0x8a9399, 0xa5acb0, 0x6e8aa0];
// Village houses: the measured palette of the detail (`cn_residential::
// VILLAGE_PAINTS`) as sRGB, each colour repeated by its share.
const VILLAGE_WALLS = [
  0xceb5a2, 0xceb5a2, 0xceb5a2, 0xceb5a2, 0xceb5a2, 0xc09e8e, 0xc09e8e, 0xc09e8e, 0xc09e8e, 0xac8e84, 0xac8e84, 0xac8e84, 0xac8e84, 0xd7c5b5, 0xd7c5b5, 0xd7c5b5, 0xd7c5b5, 0xa27973, 0xa27973, 0xa27973, 0xddd6cb, 0xddd6cb, 0xddd6cb, 0xc4c0bc, 0xc4c0bc, 0xc4c0bc, 0xc4c0bc, 0xa29e99, 0xa29e99, 0xa29e99, 0xe1dfdb, 0xe1dfdb, 0x8e6961, 0x8e6961, 0xc08976, 0xc08976, 0x6c8995, 0xacbcc4, 0xaac4b3,
];
const STEEL_BLUE = 0x5f7f9f;
const SOLAR = 0x3a4048;
const TANK = 0xd8d8d0;
const PLANT = 0x5e5c58;
const TERRACOTTA = 0x8a5a45;
// Aged asphalt, as the detail bakes it (about 13 %, neutral grey).
const ASPHALT = 0x64625e;
// Paving, plazas and forecourts between the buildings of a block.
const PAVING = 0x8b8982;
const SIDEWALK = 0x9a978f;
/** Footway width a side by street class, metres (`worldgen_contracts::cross_section`). */
const FOOTWAY_M: Record<string, number> = { arterial: 5, collector: 4, local: 3 };

// The whole part of a vertex's seed is its kind: 0 a wall with windows, 1 an
// office (which alone may be a curtain wall), 2 a shed, 3 plant with no windows.
const OFFICE_SEED = 1;
const SHED_SEED = 2;
const PLAIN_SEED = 3;

/**
 * What stands on a roof, as seen from the air, by the kind of building:
 *
 * - a residential slab carries a lift and stair head over every unit, about
 *   every 16 - 20 m along it, and on a walk-up the rows of solar water heaters
 *   (太阳能热水器) that cover most old Chinese roofs;
 * - an office carries a plant screen and cooling towers;
 * - a village house often has a blue steel room built on top (加建) and a tank;
 * - a shed has a ridge vent along its length.
 */
function dressRoof(
  builder: MassBuilder,
  b: CityMassBuilding,
  ring: Ring,
  roofTop: number,
  wall: number,
  shed: boolean,
  m: number,
): void {
  // The long axis from the longest edge, and the extent along and across it.
  let ux = 1, uz = 0, best = 0;
  for (let i = 0; i < ring.length; i += 1) {
    const [ax, az] = ring[i];
    const [bx, bz] = ring[(i + 1) % ring.length];
    const len = Math.hypot(bx - ax, bz - az);
    if (len > best) { best = len; ux = (bx - ax) / len; uz = (bz - az) / len; }
  }
  const cx = ring.reduce((s, p) => s + p[0], 0) / ring.length;
  const cz = ring.reduce((s, p) => s + p[1], 0) / ring.length;
  let a0 = Infinity, a1 = -Infinity, w0 = Infinity, w1 = -Infinity;
  for (const [x, z] of ring) {
    const a = (x - cx) * ux + (z - cz) * uz;
    const w = -(x - cx) * uz + (z - cz) * ux;
    a0 = Math.min(a0, a); a1 = Math.max(a1, a); w0 = Math.min(w0, w); w1 = Math.max(w1, w);
  }
  const length = a1 - a0, width = w1 - w0;
  const along = (a: number, w: number): [number, number] => [cx + ux * a - uz * w, cz + uz * a + ux * w];
  const h = (salt: number) => hash01(b.id, salt);
  if (shed) {
    if (length > 24 * m) {
      const [x, z] = along((a0 + a1) * 0.5, (w0 + w1) * 0.5);
      builder.orientedBox(x, z, ux, uz, length * 0.4, 0.8 * m, roofTop, roofTop + 0.7 * m, 0xc8cccf);
    }
    return;
  }
  if (b.useType === "village") {
    if (h(71) < 0.45 && length > 6 * m) {
      const share = 0.35 + 0.35 * h(72);
      const [x, z] = along(a1 - length * share * 0.5, (w0 + w1) * 0.5);
      builder.orientedBox(x, z, ux, uz, length * share * 0.5, width * 0.45, roofTop, roofTop + 2.6 * m, STEEL_BLUE);
    }
    if (h(73) < 0.6) {
      const [x, z] = along(a0 + 1.5 * m, w0 + 1.5 * m);
      builder.orientedBox(x, z, ux, uz, 0.8 * m, 0.8 * m, roofTop, roofTop + 1.6 * m, TANK);
    }
    return;
  }
  if (b.heightMetres < 9) return;
  if (b.useType === "commercial") {
    // A plant screen over a share of the roof, and a cooling tower or two.
    const [x, z] = along((a0 + a1) * 0.5 + length * 0.1 * (h(74) - 0.5), (w0 + w1) * 0.5);
    builder.orientedBox(x, z, ux, uz, length * (0.18 + 0.12 * h(75)), width * (0.2 + 0.1 * h(76)), roofTop, roofTop + 3.2 * m, PLANT);
    const towers = 1 + Math.floor(h(77) * 2);
    for (let k = 0; k < towers; k += 1) {
      const [tx, tz] = along(a0 + length * (0.15 + 0.2 * k), w1 - width * 0.22);
      builder.orientedBox(tx, tz, ux, uz, 2.0 * m, 2.0 * m, roofTop, roofTop + 3.5 * m, TANK);
    }
    return;
  }
  // Residential: a head over each unit's stair and lift.
  const unit = (16 + 4 * h(78)) * m;
  const heads = Math.max(1, Math.round(length / unit));
  const tall = b.floors >= 8;
  for (let k = 0; k < heads; k += 1) {
    const a = a0 + (k + 0.5) * (length / heads);
    const [x, z] = along(a, w0 + width * 0.62);
    builder.orientedBox(x, z, ux, uz, (tall ? 3.2 : 2.0) * m, (tall ? 3.0 : 1.8) * m, roofTop, roofTop + (tall ? 3.6 : 2.6) * m, wall);
  }
  // Walk-ups: rows of solar water heaters across most of the roof.
  if (b.floors <= 7 && h(79) < 0.75) {
    const rows = Math.max(1, Math.floor((width - 3 * m) / (2.6 * m)));
    for (let r = 0; r < rows; r += 1) {
      const w = w0 + 1.6 * m + r * 2.6 * m;
      const [x, z] = along((a0 + a1) * 0.5, w);
      builder.orientedBox(x, z, ux, uz, length * (0.3 + 0.12 * h(80 + r)), 0.7 * m, roofTop, roofTop + 1.3 * m, SOLAR);
    }
  }
}

class MassBuilder {
  readonly positions: number[] = [];
  readonly normals: number[] = [];
  readonly colors: number[] = [];
  /** Per vertex: the tile it belongs to, or -1 past the hiding limit. */
  readonly tileIds: number[] = [];
  /** Per vertex: the height the block it belongs to stands on, scene units. */
  readonly bases: number[] = [];
  /** Per vertex: the building's seed, which sets its window rhythm and facade kind. */
  readonly seeds: number[] = [];
  /** The base of the prism being built; horizontal faces never read it. */
  private base = 0;
  /** The seed of the building being built, in 0..1. */
  seed = 0;
  /** Tile key to the index the shader hides it by. */
  readonly tileIndex = new Map<string, number>();
  private current = -1;
  private readonly colour = new THREE.Color();

  /** Make `column, row` the tile the following primitives belong to. */
  select(column: number, row: number): void {
    const key = tileKey(column, row);
    let index = this.tileIndex.get(key);
    if (index === undefined) {
      index = this.tileIndex.size;
      this.tileIndex.set(key, index);
    }
    this.current = index < MAX_MASS_TILES ? index : -1;
  }

  triangles(): number {
    return this.positions.length / 9;
  }

  private push(a: number[], b: number[], c: number[], normal: number[], hex: number, shade: number) {
    this.colour.setHex(hex);
    for (const p of [a, b, c]) {
      this.positions.push(p[0], p[1], p[2]);
      this.normals.push(normal[0], normal[1], normal[2]);
      this.colors.push(this.colour.r * shade, this.colour.g * shade, this.colour.b * shade);
      this.tileIds.push(this.current);
      this.bases.push(this.base);
      this.seeds.push(this.seed);
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
    this.base = bottom;
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

  /** A small box on a roof: plant, a tank, a stair house. */
  box(cx: number, cz: number, half: number, bottom: number, top: number, hex: number) {
    this.orientedBox(cx, cz, 1, 0, half, half, bottom, top, hex);
  }

  /**
   * A box turned to the unit direction (ux, uz), `halfLength` along it. Roof
   * plant: drawn plain, with no window pattern, whatever the building's kind.
   */
  orientedBox(cx: number, cz: number, ux: number, uz: number, halfLength: number, halfWidth: number, bottom: number, top: number, hex: number) {
    const vx = -uz, vz = ux;
    const at = (a: number, b: number): [number, number] => [cx + ux * a + vx * b, cz + uz * a + vz * b];
    const ring: Ring = [at(-halfLength, -halfWidth), at(halfLength, -halfWidth), at(halfLength, halfWidth), at(-halfLength, halfWidth)];
    const seed = this.seed;
    this.seed = PLAIN_SEED + (seed % 1);
    this.prism(ring, bottom, top, hex, hex);
    this.seed = seed;
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
    g.setAttribute("aTile", new THREE.Float32BufferAttribute(this.tileIds, 1));
    g.setAttribute("aBase", new THREE.Float32BufferAttribute(this.bases, 1));
    g.setAttribute("aSeed", new THREE.Float32BufferAttribute(this.seeds, 1));
    g.computeBoundingSphere();
    g.computeBoundingBox();
    return g;
  }
}

/** A line moved sideways by `d` (scene units), point by point. */
function offsetLine(pts: Array<[number, number, number]>, d: number): Array<[number, number, number]> {
  return pts.map((p, i) => {
    const a = pts[Math.max(0, i - 1)], b = pts[Math.min(pts.length - 1, i + 1)];
    const ex = b[0] - a[0], ez = b[2] - a[2];
    const len = Math.hypot(ex, ez) || 1;
    return [p[0] - (ez / len) * d, p[1], p[2] + (ex / len) * d];
  });
}

/** A summer crown's green, varied tree to tree. */
function treeTint(x: number, z: number): number {
  const h = Math.abs(Math.sin(x * 12.9898 + z * 78.233) * 43758.5453) % 1;
  const greens = [0x3f5f2c, 0x4a6a30, 0x557236, 0x3c5a34, 0x5d7a3a];
  return greens[Math.floor(h * greens.length)];
}

/**
 * A tree's crown from afar: an eight-sided double cone, wider than high, on a
 * trunk-height of clear stem, lit from above and darker below.
 */
function crown(builder: MassBuilder, x: number, ground: number, z: number, height: number, radius: number, hex: number) {
  const bottom = ground + height * 0.35;
  const middle = ground + height * 0.62;
  const top = ground + height;
  const sides = 8;
  for (let k = 0; k < sides; k += 1) {
    const a0 = (k / sides) * Math.PI * 2, a1 = ((k + 1) / sides) * Math.PI * 2;
    const p0 = [x + Math.cos(a0) * radius, middle, z + Math.sin(a0) * radius];
    const p1 = [x + Math.cos(a1) * radius, middle, z + Math.sin(a1) * radius];
    const am = (a0 + a1) * 0.5;
    const out = [Math.cos(am), 0, Math.sin(am)];
    builder.tri([x, top, z], p1, p0, [out[0] * 0.6, 0.8, out[2] * 0.6], hex, 1.0);
    builder.tri([x, bottom, z], p0, p1, [out[0] * 0.6, -0.8, out[2] * 0.6], hex, 0.62);
  }
}

export interface CityMass {
  mesh: THREE.Mesh;
  triangles: number;
  /** Hide or show one tile's share of the far form, as its detail is mounted or dropped. */
  setTileHidden(column: number, row: number, hidden: boolean): void;
  dispose(): void;
}

/** The mean of a ring's corners, which is how the detail cut places a block or parcel. */
function ringMean(points: Array<{ x_km: number; y_km: number }>): { x_km: number; y_km: number } {
  const n = Math.max(points.length, 1);
  return {
    x_km: points.reduce((sum, p) => sum + p.x_km, 0) / n,
    y_km: points.reduce((sum, p) => sum + p.y_km, 0) / n,
  };
}

export function buildCityMass(city: ModernCityPayload, frame: CityMassFrame): CityMass {
  const m = frame.metresToScene;
  const builder = new MassBuilder();
  // A building is in the tile of its parcel, as the detail cut keeps it.
  const parcelTile = new Map<number, [number, number]>();
  for (const parcel of city.parcels) parcelTile.set(parcel.id, tileOfPoint(city.frame, ringMean(parcel.ring)));
  // Hidden by a shader uniform, so one mesh can lose any tile's share of itself.
  const hidden = new Float32Array(MAX_MASS_TILES);
  const toRing = (points: Array<{ x_km: number; y_km: number }>): Ring =>
    points.map((p) => frame.toScene(p.x_km, p.y_km));

  // The ground of every block, so buildings stand on paving rather than on the
  // meadow the terrain texture shows.
  for (const block of city.blocks) {
    builder.select(...tileOfPoint(city.frame, ringMean(block.boundary)));
    builder.flat(toRing(block.boundary), frame.heightAt, 0.15 * m, PAVING);
  }

  // Each parcel's ground over its block, by what it is used for: from a few
  // hundred metres a town is lawns, forecourts and yards, not one grey slab.
  const PARCEL_GROUND: Record<string, number> = {
    park: 0x4e6b36,
    residential: 0x667c4c,
    villa: 0x5f7746,
    farmstead: 0x6b7447,
    village: 0x7a7a62,
    commercial: 0x96948c,
    mixedUse: 0x8f8d86,
    civic: 0x8a8a7e,
    industrial: 0x837d72,
  };
  for (const parcel of city.parcels) {
    builder.select(...tileOfPoint(city.frame, ringMean(parcel.ring)));
    builder.seed = PLAIN_SEED;
    builder.flat(toRing(parcel.ring), frame.heightAt, 0.25 * m, PARCEL_GROUND[parcel.useType] ?? PAVING);
  }

  const streetTreeLines: Array<{ pts: Array<[number, number, number]>; offset: number; tile: [number, number] | null }> = [];
  // Streets next, as flat ribbons a little above the ground. The metre-scale
  // scene draws the real carriageway; this is its line from afar.
  for (const road of city.hdRoads) {
    const mid = road.centreline[Math.floor(road.centreline.length / 2)];
    if (mid) builder.select(...tileOfPoint(city.frame, mid));
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
    // Footways either side of the carriageway, so a street reads as a street.
    const width = Math.max(road.widthMetres, 6);
    // The footway each side, furniture belt included (the cross-section table, CJJ 37).
    const walk = road.layer ? 0 : (FOOTWAY_M[road.class] ?? 0);
    builder.ribbon(pts, (width - 2 * walk) * m, ASPHALT);
    if (walk > 0) {
      for (const side of [-1, 1]) {
        builder.ribbon(offsetLine(pts, side * (width - walk) * 0.5 * m), walk * m, SIDEWALK);
      }
    }
    // The detail's pits: a metre or so in from the kerb, every 8 m both sides.
    if (walk >= 2.4) streetTreeLines.push({ pts, offset: (width * 0.5 - walk + 1.05) * m, tile: mid ? tileOfPoint(city.frame, mid) : null });
  }

  // Tree crowns: every tree of the plan, and the avenues along the streets the
  // detail plants. A town seen from a kilometre is half trees.
  builder.seed = PLAIN_SEED;
  for (const tree of city.trees ?? []) {
    if (!tree.point) continue;
    builder.select(...tileOfPoint(city.frame, tree.point));
    const [x, z] = frame.toScene(tree.point.x_km, tree.point.y_km);
    const y = frame.heightAt(x, z);
    crown(builder, x, y, z, (tree.heightMetres ?? 9) * m, Math.max(tree.crownRadiusMetres ?? 3, 1.5) * m, treeTint(x, z));
  }
  for (const line of streetTreeLines) {
    // The street's own tile, as the road it lines is hidden with.
    if (line.tile) builder.select(...line.tile);
    for (const side of [-1, 1]) {
      const placed = offsetLine(line.pts, side * line.offset);
      let carry = 4;
      for (let i = 0; i + 1 < placed.length; i += 1) {
        const [ax, ay, az] = placed[i];
        const [bx, by, bz] = placed[i + 1];
        const len = Math.hypot(bx - ax, bz - az);
        while (carry < len) {
          const t = carry / len;
          const x = ax + (bx - ax) * t, z = az + (bz - az) * t;
          crown(builder, x, ay + (by - ay) * t - 0.35 * m, z, 10 * m, 3.4 * m, treeTint(x, z));
          carry += 8 * m;
        }
        carry -= len;
      }
    }
  }

  builder.seed = 0;
  for (const b of city.buildings) {
    builder.select(...(parcelTile.get(b.parcelId) ?? tileOfPoint(city.frame, ringMean(b.footprint))));
    const ring = toRing(b.footprint);
    if (ring.length < 3) continue;
    let base = Infinity;
    for (const [x, z] of ring) base = Math.min(base, frame.heightAt(x, z));
    // Sink the foot below the lowest corner so a building on a slope has no gap.
    const foot = base - 1.5 * m;
    const total = Math.max(b.heightMetres, 3) * m;
    // Only office towers are glass. A residential tower is plaster or tile, as light
    // as the slabs, so a city of flats does not read as a city of dark glass.
    const towers = b.heightMetres >= 40 && b.useType === "commercial";
    const shed = b.useType === "industrial" && b.floors <= 1;
    const palette = shed ? SHED_WALLS : b.useType === "village" ? VILLAGE_WALLS : towers ? TOWER_WALLS : WALLS;
    let wall = palette[Math.floor(hash01(b.id, 1) * palette.length)];
    // A brick or red-tiled tower is rare; above about eight storeys it is plaster.
    if (wall === BRICK && b.heightMetres > 25) wall = WALLS[0];
    const roof = shed
      ? SHED_ROOFS[Math.floor(hash01(b.id, 2) * SHED_ROOFS.length)]
      : b.roof === "terracotta"
        ? TERRACOTTA
        : ROOFS[Math.floor(hash01(b.id, 2) * ROOFS.length)];
    const podium = b.podiumHeightMetres ?? 0;
    const roofTop = base + total;
    // The whole part says whether the building is an office, which alone may be glass.
    builder.seed = hash01(b.id, 61) * 0.999 + (shed ? SHED_SEED : b.useType === "commercial" ? OFFICE_SEED : 0);
    if (b.tierRing && b.tierRing.length >= 3 && podium > 0 && podium * m < total) {
      builder.prism(ring, foot, base + podium * m, wall, roof);
      builder.prism(toRing(b.tierRing), base + podium * m, base + total, wall, roof);
    } else {
      builder.prism(ring, foot, base + total, wall, roof);
    }
    dressRoof(builder, b, ring, roofTop, wall, shed, m);
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
  // Storey bands and window bays on every wall, from world position: they
  // resolve as the camera nears and average to a flat wall as it recedes, so
  // the facade never shimmers at the distances where it is only a few pixels.
  material.onBeforeCompile = (shader) => {
    shader.uniforms.uMetre = { value: m };
    shader.uniforms.uHidden = { value: hidden };
    shader.vertexShader = shader.vertexShader
      .replace("#include <common>", "#include <common>\nattribute float aTile;\nattribute float aBase;\nattribute float aSeed;\nvarying float vTile;\nvarying float vBase;\nvarying float vSeed;\nvarying vec3 vMassWorld;\nvarying vec3 vMassNormal;")
      .replace(
        "#include <worldpos_vertex>",
        "#include <worldpos_vertex>\nvTile = aTile;\nvBase = aBase;\nvSeed = aSeed;\nvMassWorld = (modelMatrix * vec4(transformed, 1.0)).xyz;\nvMassNormal = normalize(mat3(modelMatrix) * objectNormal);",
      );
    shader.fragmentShader = shader.fragmentShader
      .replace(
        "#include <common>",
        "#include <common>\nuniform float uMetre;\nuniform float uHidden[256];\nvarying float vTile;\nvarying float vBase;\nvarying float vSeed;\nvarying vec3 vMassWorld;\nvarying vec3 vMassNormal;",
      )
      .replace(
        "#include <color_fragment>",
        `float massGlass = 0.0;
        if (vTile >= 0.0 && uHidden[int(vTile + 0.5)] > 0.5) discard;
        #include <color_fragment>
        float wallness = 1.0 - smoothstep(0.35, 0.7, abs(vMassNormal.y));
        vec2 along = vec2(-vMassNormal.z, vMassNormal.x);
        float s = dot(vMassWorld.xz, along) / uMetre;
        // Height above the block's own ground, so the storeys start at the pavement.
        float hM = (vMassWorld.y - vBase) / uMetre;
        // Each building has its own bay and storey, its own kind of facade, and some
        // faces with no windows at all, so no two towers repeat one grid.
        float kind = floor(vSeed);
        bool office = kind > 0.5 && kind < 1.5;
        // Sheds and roof plant have no windows and no floor slabs.
        bool blank = kind > 1.5;
        float seed = fract(vSeed);
        float r1 = fract(seed * 7.13);
        float r2 = fract(seed * 13.71);
        float r3 = fract(seed * 29.37);
        float bayM = mix(2.8, 4.2, r1);
        float storeyM = mix(3.0, 3.9, r2);
        float faceR = fract(sin(dot(vMassNormal.xz, vec2(12.9898, 78.233)) + seed * 19.19) * 43758.5453);
        float faceOpen = blank ? 0.0 : step(0.18, faceR);
        float fh = max(fwidth(hM), 1e-4) / storeyM;
        float fs = max(fwidth(s), 1e-4) / bayM;
        float storey = fract(hM / storeyM);
        float bay = fract(s / bayM);
        // Three kinds: a glass curtain wall that is nearly all glass, punched windows
        // about half of each bay and storey, and a sparse block with many blank bays.
        bool curtain = office && r3 > 0.4;
        bool sparse = r3 < 0.2;
        float rLo = curtain ? 0.12 : 0.22;
        float rHi = curtain ? 0.88 : 0.72;
        float cLo = curtain ? 0.06 : 0.25;
        float cHi = curtain ? 0.94 : 0.75;
        // The edges widen with the pixel footprint, so a window a pixel or two across
        // averages to a grey that still reads as glazing; the pattern fades only once
        // several windows share one pixel.
        float rows = smoothstep(rLo - fh, rLo + fh, storey) * (1.0 - smoothstep(rHi - fh, rHi + fh, storey));
        float cols = smoothstep(cLo - fs, cLo + fs, bay) * (1.0 - smoothstep(cHi - fs, cHi + fs, bay));
        // Full detail while a window is four or more pixels across; below two pixels it
        // is only a tone, the average of the pattern, or it aliases into dashes.
        float fade = 1.0 - smoothstep(0.25, 0.5, max(fh, fs));
        // Single windows are missing here and there: a dark room, a closed unit.
        vec2 cell = vec2(floor(s / bayM), floor(hM / storeyM)) + seed * 97.0;
        float cellR = fract(sin(dot(cell, vec2(127.1, 311.7))) * 43758.5453);
        float presentMean = curtain ? 1.0 : (sparse ? 0.4 : 0.88);
        float presentNow = curtain ? 1.0 : (sparse ? smoothstep(0.55, 0.65, cellR) : smoothstep(0.1, 0.25, cellR));
        // Below about five pixels a cell's own value is noise, so it averages to the mean.
        float present = mix(presentMean, presentNow, 1.0 - smoothstep(0.1, 0.2, max(fh, fs)));
        // A wall darkens toward the pavement, where the block next door takes the sky.
        float baseOcclusion = mix(0.6, 1.0, smoothstep(0.0, 10.0, hM));
        diffuseColor.rgb *= mix(vec3(1.0), vec3(baseOcclusion), wallness);
        // Glass is darker and cooler than the wall, and each building's glass differs.
        // Far enough off, the glass averages to its share of the facade, not to nothing.
        float meanGlass = (rHi - rLo) * (cHi - cLo) * presentMean;
        float glass = mix(meanGlass, rows * cols * present, fade) * faceOpen * wallness;
        vec3 tint = mix(vec3(0.40, 0.46, 0.54), vec3(0.30, 0.38, 0.47), r2);
        diffuseColor.rgb *= mix(vec3(1.0), tint, glass);
        // Dirt washed down the wall in columns, the far form of the detail facade's
        // weathering; it averages to a slightly darker wall once a column is
        // under a pixel, so the hand-off to the detail does not change the tone.
        float column = floor(s / 0.8);
        float columnR = fract(sin(column * 12.9898 + seed * 78.233) * 43758.5453);
        float runR = fract(sin(column * 4.13 + floor(hM / 9.0) * 7.71 + seed * 3.3) * 43758.5453);
        float streak = smoothstep(0.55, 0.95, columnR) * (0.55 + 0.45 * runR);
        streak = mix(streak, 0.13, smoothstep(0.2, 0.8, fwidth(s)));
        diffuseColor.rgb *= 1.0 - vec3(0.20, 0.22, 0.24) * streak * wallness * (1.0 - glass);
        // Glass sits back in its reveal, so the head of each window is in shade.
        float head = smoothstep(rHi - 0.2, rHi, storey);
        diffuseColor.rgb *= mix(vec3(1.0), vec3(0.72), glass * head);
        // A residential block reads from afar as floor slabs: the slab edge (or the
        // balcony front) a light line at each floor, the soffit's shadow under it.
        // Each band fades to its mean once it is under a couple of pixels.
        float bandFade = 1.0 - smoothstep(0.05, 0.15, fh);
        float slabEdge = mix(0.07, smoothstep(-fh, fh, storey) * (1.0 - smoothstep(0.07 - fh, 0.07 + fh, storey)), bandFade);
        float soffit = mix(0.08, smoothstep(0.07 - fh, 0.07 + fh, storey) * (1.0 - smoothstep(0.15 - fh, 0.15 + fh, storey)), bandFade);
        float slabs = office || blank ? 0.0 : wallness * faceOpen;
        diffuseColor.rgb *= (1.0 + 0.12 * slabEdge * slabs) * (1.0 - 0.22 * soffit * slabs);
        massGlass = glass;
        `,
      )
      .replace(
        "#include <emissivemap_fragment>",
        `#include <emissivemap_fragment>
        // Glass reflects the sky, more at a grazing angle, so far glazing is a cool
        // mid grey and not a hole in the wall.
        {
          float facing = clamp(abs(dot(normalize(vViewPosition), normal)), 0.0, 1.0);
          float fresnel = pow(1.0 - facing, 3.0);
          totalEmissiveRadiance += massGlass * vec3(0.055, 0.065, 0.08) * (0.5 + 4.0 * fresnel);
        }
        `,
      );
  };
  const mesh = new THREE.Mesh(geometry, material);
  mesh.name = "cityMass";
  // The sun's frustum is fitted to the view, so the towers cast shadows across the
  // streets they stand over, and a tower in another's shadow goes dark on its wall.
  mesh.castShadow = true;
  mesh.receiveShadow = true;
  return {
    mesh,
    triangles: builder.triangles(),
    setTileHidden(column, row, isHidden) {
      const index = builder.tileIndex.get(tileKey(column, row));
      if (index !== undefined && index < MAX_MASS_TILES) hidden[index] = isHidden ? 1 : 0;
    },
    dispose() {
      geometry.dispose();
      material.dispose();
    },
  };
}
