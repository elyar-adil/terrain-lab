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

function smooth01(t: number): number {
  return t * t * (3 - 2 * t);
}

/** `hex` scaled by `light`, nudged warmer (`warm` near 1) or cooler (near 0). */
function shadeHex(hex: number, light: number, warm: number): number {
  const shift = (warm - 0.5) * 0.08;
  const r = Math.min(255, ((hex >> 16) & 255) * light * (1 + shift));
  const g = Math.min(255, ((hex >> 8) & 255) * light);
  const b = Math.min(255, (hex & 255) * light * (1 - shift));
  return (Math.round(r) << 16) | (Math.round(g) << 8) | Math.round(b);
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
// The height of an expressway deck over the ground, as the detail builds it.
const VIADUCT_M = 9;
// Pavement either side of a street.
const SIDEWALK = 0x9a958a;
// Unbuilt ground inside the town: rough grass and scrub.
const OPEN_GROUND = 0x66704d;
// The ground of a lot by what it is, as the detail plants it: a park is lawn and
// trees, a compound is lawn between its paths, a lot of shops or offices is a
// paved forecourt, a works yard is concrete, a village is packed lanes. One
// colour of grey under every lot was the first thing that gave the far form away.
const LOT_GROUND: Record<string, number> = {
  park: 0x5d7344,
  green: 0x5d7344,
  residential: 0x737d5c,
  commercial: 0x9b988f,
  mixedUse: 0x96938a,
  civic: 0x948f84,
  industrial: 0x8c8a84,
  village: 0x7a736a,
  farmstead: 0x7a736a,
  // A villa's garden is lawn and planting round the house.
  villa: 0x6a7752,
};
// Paved lots, whose ground the shader breaks up with planted beds and tree pits.
const PAVED_LOTS = new Set(["commercial", "mixedUse", "civic", "industrial"]);
// Each crop in its three looks: the mean of the detail's own field texture for that
// look (linear average, back to sRGB), so a field does not change colour when the
// detail replaces it.
const CROP_GROUND: Record<string, number[]> = {
  wheat: [0x4a5d38, 0x57683d, 0x85754c],
  rice: [0x5b6768, 0x476434, 0x837543],
  rapeseed: [0xa19d2f, 0xa09a33, 0x4d6c3c],
  corn: [0x535b3c, 0x57603d, 0x6c6044],
  vegetables: [0x554f3b, 0x333430, 0x544d3f],
  fallow: [0x5c4d3f, 0x5b673e, 0x857757],
  orchard: [0x50663a, 0x595b40, 0x50663a],
};

// The whole part of a vertex's seed is its kind: 0 a wall with windows, 1 an
// office (which alone may be a curtain wall), 2 a shed, 3 plant with no windows.
const OFFICE_SEED = 1;
const SHED_SEED = 2;
const PLAIN_SEED = 3;
// Ground kinds, which the shader textures instead of windowing: soft ground (lawn,
// scrub, packed earth), a paved lot, a field, a street.
const GROUND_SEED = 4;
const PAVED_SEED = 5;
const FIELD_SEED = 6;
const STREET_SEED = 7;

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
  /** The base of the prism being built; horizontal faces never read it, so a
   * field carries its row direction in it instead. */
  base = 0;
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
  // A block with no lots is open ground (a field, a verge, land not built on
  // yet), not a plaza: under the lots it is the paving between them.
  const built = new Set(city.parcels.map((parcel) => parcel.blockId));
  city.blocks.forEach((block, index) => {
    builder.select(...tileOfPoint(city.frame, ringMean(block.boundary)));
    // A parcel names its block by the block's index in the plan.
    const ground = built.has(index) ? PAVING : OPEN_GROUND;
    builder.seed = (built.has(index) ? PAVED_SEED : GROUND_SEED) + hash01(index, 71) * 0.999;
    builder.flat(toRing(block.boundary), frame.heightAt, 0.15 * m, ground);
  });

  // Each lot's own ground over the block's.
  for (const parcel of city.parcels) {
    const colour = LOT_GROUND[parcel.useType] ?? PAVING;
    const ground = parcel.useType === "residential" && !parcel.compound ? 0x7f7d72 : colour;
    builder.select(...(parcelTile.get(parcel.id) ?? tileOfPoint(city.frame, ringMean(parcel.ring))));
    // Neighbouring lots are never one shade: each is a little lighter or darker,
    // warmer or cooler, as a real forecourt or lawn is by its age and upkeep.
    const r = hash01(parcel.id, 73);
    builder.seed = (PAVED_LOTS.has(parcel.useType) ? PAVED_SEED : GROUND_SEED) + r * 0.999;
    builder.flat(toRing(parcel.ring), frame.heightAt, 0.25 * m, shadeHex(ground, 0.9 + 0.2 * hash01(parcel.id, 74), r));
  }

  // Farmland round the town, in its crop's colour, so the edge of the far form is
  // fields and not the terrain's own meadow under a grey rim.
  for (const field of city.fields ?? []) {
    const looks = CROP_GROUND[field.crop] ?? CROP_GROUND.fallow;
    builder.select(...tileOfPoint(city.frame, ringMean(field.ring)));
    builder.seed = FIELD_SEED + hash01(field.id, 75) * 0.999;
    const hex = shadeHex(looks[field.variant % looks.length], 0.92 + 0.16 * hash01(field.id, 76), hash01(field.id, 77));
    // Rows run along the plot's long side, as it is ploughed.
    const ring = toRing(field.ring);
    let longest = 0;
    for (let i = 0; i < ring.length; i += 1) {
      const [ax, az] = ring[i], [bx, bz] = ring[(i + 1) % ring.length];
      const len = Math.hypot(bx - ax, bz - az);
      if (len > longest) { longest = len; builder.base = Math.atan2(bz - az, bx - ax); }
    }
    builder.flat(ring, frame.heightAt, 0.2 * m, hex);
    builder.base = 0;
  }

  // Streets next, as flat ribbons a little above the ground. The metre-scale
  // scene draws the real carriageway; this is its line from afar.
  // An expressway on a viaduct is drawn at its deck, and a ramp climbs from the
  // street to the deck end it joins, as the detail's profile does.
  const deckKey = (p: { x_km: number; y_km: number }) => `${Math.round(p.x_km * 1e4)},${Math.round(p.y_km * 1e4)}`;
  const deckEnds = new Set<string>();
  const elevated = (road: (typeof city.hdRoads)[number]) => road.class === "expressway" && (road.layer ?? 0) !== 0;
  for (const road of city.hdRoads) {
    if (!elevated(road) || road.oneway || road.centreline.length < 2) continue;
    deckEnds.add(deckKey(road.centreline[0]));
    deckEnds.add(deckKey(road.centreline[road.centreline.length - 1]));
  }
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
    if (elevated(road)) {
      const n = road.centreline.length;
      const startUp = !road.oneway || deckEnds.has(deckKey(road.centreline[0]));
      const endUp = !road.oneway || deckEnds.has(deckKey(road.centreline[n - 1]));
      const along = [0];
      for (let i = 1; i < n; i += 1) along.push(along[i - 1] + Math.hypot(pts[i][0] - pts[i - 1][0], pts[i][2] - pts[i - 1][2]));
      const total = along[n - 1] || 1;
      pts.forEach((p, i) => {
        const t = along[i] / total;
        const lift = (startUp ? 1 - smooth01(t) : 0) + (endUp ? smooth01(t) : 0);
        p[1] += Math.min(lift, 1) * VIADUCT_M * m;
      });
    }
    for (const p of pts) p[1] += 0.35 * m;
    builder.seed = STREET_SEED + hash01(road.id, 79) * 0.999;
    // The pavement either side, light paving under the street trees, then the
    // carriageway: from the air a street is a dark band with light edges, not one
    // flat grey ribbon.
    const width = Math.max(road.widthMetres, 6);
    const kerb = road.class === "expressway" ? 0 : road.class === "local" || road.class === "service" ? 2 : 4;
    if (kerb > 0) builder.ribbon(pts.map(([x, y, z]) => [x, y - 0.05 * m, z]), (width + 2 * kerb) * m, SIDEWALK);
    builder.ribbon(pts, width * m, ASPHALT);
  }

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
        `#include <common>
        uniform float uMetre;
        uniform float uHidden[256];
        varying float vTile;
        varying float vBase;
        varying float vSeed;
        varying vec3 vMassWorld;
        varying vec3 vMassNormal;
        float massHash(vec2 p) { return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453); }
        float massNoise(vec2 p) {
          vec2 i = floor(p), f = fract(p);
          f = f * f * (3.0 - 2.0 * f);
          return mix(mix(massHash(i), massHash(i + vec2(1.0, 0.0)), f.x),
                     mix(massHash(i + vec2(0.0, 1.0)), massHash(i + vec2(1.0, 1.0)), f.x), f.y);
        }
        // One octave of noise at \`scale\` metres, faded to its mean once a cell is only a
        // couple of pixels, so the ground never sparkles far off.
        float massOctave(vec2 g, float scale, float px, float salt) {
          return mix(0.5, massNoise(g / scale + salt), 1.0 - smoothstep(scale * 0.25, scale * 0.5, px));
        }`,
      )
      .replace(
        "#include <color_fragment>",
        `float massGlass = 0.0;
        if (vTile >= 0.0 && uHidden[int(vTile + 0.5)] > 0.5) discard;
        #include <color_fragment>
        float wallness = 1.0 - smoothstep(0.35, 0.7, abs(vMassNormal.y));
        // The ground from the air is never one flat colour: lawn worn to earth in
        // places, a forecourt's stains and its planted beds, a field wetter in one
        // corner, a carriageway patched. All of it in world metres, at three scales.
        if (vSeed >= ${GROUND_SEED}.0) {
          float groundKind = floor(vSeed);
          float groundR = fract(vSeed);
          vec2 g = vMassWorld.xz / uMetre;
          float px = max(fwidth(g.x), fwidth(g.y));
          float n1 = massOctave(g, 70.0, px, groundR * 31.0);
          float n2 = massOctave(g, 19.0, px, 7.3);
          float n3 = massOctave(g, 5.5, px, 3.1);
          float n = 0.5 * n1 + 0.3 * n2 + 0.2 * n3;
          float luma = dot(diffuseColor.rgb, vec3(0.3, 0.55, 0.15));
          if (groundKind < ${PAVED_SEED}.5 && groundKind > ${GROUND_SEED}.5) {
            // A paved lot: concrete stained light and dark, and its beds of shrubs and
            // trees, a fifth of the lot or so, which from afar is a green-grey tone.
            diffuseColor.rgb *= 0.86 + 0.28 * n;
            float bedNoise = mix(n1, n2, 0.55);
            float bedSharp = smoothstep(0.6, 0.66, bedNoise);
            float bed = mix(0.2, bedSharp, 1.0 - smoothstep(10.0, 25.0, px));
            vec3 planting = vec3(0.075, 0.105, 0.05) * (0.8 + 0.4 * n3);
            diffuseColor.rgb = mix(diffuseColor.rgb, planting, bed);
          } else if (groundKind < ${GROUND_SEED}.5) {
            // Lawn and scrub: thicker and darker in clumps of shrubs, worn to dry earth
            // along desire lines and in patches.
            diffuseColor.rgb *= 0.7 + 0.6 * n;
            float clumpNoise = mix(n2, n3, 0.5);
            float clump = mix(0.12, smoothstep(0.62, 0.7, clumpNoise), 1.0 - smoothstep(6.0, 14.0, px));
            diffuseColor.rgb = mix(diffuseColor.rgb, vec3(0.05, 0.075, 0.035), clump);
            float worn = mix(0.1, smoothstep(0.6, 0.75, mix(n1, n2, 0.5)), 1.0 - smoothstep(15.0, 35.0, px)) * 0.5;
            diffuseColor.rgb = mix(diffuseColor.rgb, vec3(0.2, 0.16, 0.1) * (luma / 0.14), worn);
          } else if (groundKind < ${FIELD_SEED}.5) {
            // A field is farmed in strips along its rows, a few metres wide, each sown or
            // cut a little apart from the next, so it reads as stripes from the air.
            vec2 across = vec2(-sin(vBase), cos(vBase));
            float w = dot(g, across);
            float strip = floor(w / mix(4.0, 9.0, groundR));
            float stripR = massHash(vec2(strip, groundR * 17.0));
            float stripFade = 1.0 - smoothstep(1.5, 4.0, px);
            diffuseColor.rgb *= mix(1.0, 0.92 + 0.16 * stripR, stripFade);
            // The furrows themselves, under a metre apart, only as the camera nears.
            float row = abs(fract(w / 0.75) - 0.5) * 2.0;
            diffuseColor.rgb *= mix(1.0, 0.85 + 0.25 * row, 1.0 - smoothstep(0.1, 0.3, px));
            // Wetter and greener in one part, thinner in another.
            diffuseColor.rgb *= 0.82 + 0.36 * n;
            diffuseColor.rgb = mix(diffuseColor.rgb, diffuseColor.rgb * vec3(1.1, 1.0, 0.8), n1 - 0.5);
          } else {
            // A carriageway: newer patches darker, old seal lighter, a smear down the wheel paths.
            diffuseColor.rgb *= 0.9 + 0.2 * mix(n2, n3, 0.5);
          }
        }
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
  const trees = buildFarTrees(city, frame, builder, hidden);
  if (trees) mesh.add(trees.mesh);
  // The sun's frustum is fitted to the view, so the towers cast shadows across the
  // streets they stand over, and a tower in another's shadow goes dark on its wall.
  mesh.castShadow = true;
  mesh.customDepthMaterial = hiddenDepthMaterial(hidden);
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
      mesh.customDepthMaterial?.dispose();
      trees?.dispose();
    },
  };
}

/**
 * The shadow pass of a far-form mesh, which drops a hidden tile's share as the
 * colour pass does. Without it a tile the detail has replaced still cast the far
 * form's shadows: tree shadows on a lawn with no trees, a second shadow beside
 * each building.
 */
function hiddenDepthMaterial(hidden: Float32Array): THREE.MeshDepthMaterial {
  const material = new THREE.MeshDepthMaterial({ depthPacking: THREE.RGBADepthPacking });
  material.onBeforeCompile = (shader) => {
    shader.uniforms.uHidden = { value: hidden };
    shader.vertexShader = shader.vertexShader
      .replace("#include <common>", "#include <common>\nattribute float aTile;\nvarying float vTile;")
      .replace("#include <begin_vertex>", "#include <begin_vertex>\nvTile = aTile;");
    shader.fragmentShader = shader.fragmentShader
      .replace("#include <common>", "#include <common>\nuniform float uHidden[256];\nvarying float vTile;")
      .replace("#include <clipping_planes_fragment>", "if (vTile >= 0.0 && uHidden[int(vTile + 0.5)] > 0.5) discard;\n#include <clipping_planes_fragment>");
  };
  material.customProgramCacheKey = () => "far-depth";
  return material;
}

/**
 * How each species' crown sits from afar: its spread against the plan's crown
 * radius, its height against that spread, how far the crown's foot is below the
 * top (in crown heights), and its colours. A plane tree is broad and dense, a
 * ginkgo narrow and upright with lighter leaves, a willow wide, low and pale.
 */
const CROWN_FORMS: Record<string, { spread: number; tall: number; lift: number; colours: number[] }> = {
  chinesePlane: { spread: 1.0, tall: 1.0, lift: 1.0, colours: [0x3e5a33, 0x47633a, 0x51703f, 0x3a5230] },
  ginkgo: { spread: 0.75, tall: 1.6, lift: 1.0, colours: [0x55743d, 0x5f7c42, 0x4d6a38] },
  willow: { spread: 1.1, tall: 1.05, lift: 1.25, colours: [0x5f7c46, 0x6a8650, 0x58743f] },
  cedar: { spread: 0.65, tall: 2.0, lift: 1.0, colours: [0x34492f, 0x2f4330] },
};

/**
 * A crown from afar: a clump of lobes, not one smooth ball, so its outline is
 * ragged and its sunlit side broken by the shade between the lobes. Shaded darker
 * underneath and toward the inside, where the leaves are in each other's shadow;
 * normals bent toward the clump's own centre so the light falls on the crown
 * as a whole and not on each lobe as a separate ball.
 */
function farCrown(): THREE.BufferGeometry {
  const lobes: Array<[number, number, number, number]> = [[0, 0.05, 0, 0.62]];
  for (let i = 0; i < 6; i += 1) {
    const a = (i / 6) * Math.PI * 2 + (i % 2) * 0.4;
    const up = i % 2 ? 0.28 : -0.12;
    lobes.push([Math.cos(a) * 0.48, up, Math.sin(a) * 0.48, 0.42 + 0.08 * ((i * 7) % 3)]);
  }
  lobes.push([0.1, 0.48, -0.05, 0.4]);
  const positions: number[] = [];
  const normals: number[] = [];
  const colours: number[] = [];
  const lobe = new THREE.IcosahedronGeometry(1, 0);
  const p = lobe.attributes.position;
  const n = lobe.attributes.normal;
  for (const [cx, cy, cz, r] of lobes) {
    for (let i = 0; i < p.count; i += 1) {
      // A little jitter on each lobe's vertices, so no facet repeats another.
      const j = 1 + 0.12 * Math.sin(i * 12.9898 + cx * 78.233 + cz * 37.719);
      const x = cx + p.getX(i) * r * j, y = cy + p.getY(i) * r * j, z = cz + p.getZ(i) * r * j;
      positions.push(x, y, z);
      const len = Math.hypot(x, y, z) || 1;
      const bx = n.getX(i) * 0.45 + (x / len) * 0.8, by = n.getY(i) * 0.45 + (y / len) * 0.8, bz = n.getZ(i) * 0.45 + (z / len) * 0.8;
      const bl = Math.hypot(bx, by, bz) || 1;
      normals.push(bx / bl, by / bl, bz / bl);
      const height = 0.5 + 0.5 * Math.max(-1, Math.min(1, y / 0.9));
      const outer = Math.min(1, len / 0.95);
      const k = 0.42 + 0.33 * height + 0.25 * outer;
      colours.push(k, k, k);
    }
  }
  lobe.dispose();
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
  geometry.setAttribute("normal", new THREE.Float32BufferAttribute(normals, 3));
  geometry.setAttribute("color", new THREE.Float32BufferAttribute(colours, 3));
  return geometry;
}

/**
 * Every tree of the plan as a crown, one instanced draw: a low ellipsoid at the
 * tree's own height and spread, lighter on top where the sun falls, in the
 * colour range of the species the detail grows. Hidden tile by tile with the
 * masses, so it stands exactly where the detail's trees will. Without it a
 * town's streets and compounds were bare until the detail arrived, and then
 * suddenly full of trees.
 */
function buildFarTrees(
  city: ModernCityPayload,
  frame: CityMassFrame,
  builder: MassBuilder,
  hidden: Float32Array,
): { mesh: THREE.InstancedMesh; dispose(): void } | null {
  const trees = city.trees ?? [];
  if (!trees.length) return null;
  const m = frame.metresToScene;
  const crown = farCrown();
  const tiles = new Float32Array(trees.length);
  const material = new THREE.MeshStandardMaterial({ vertexColors: true, roughness: 0.95, metalness: 0 });
  material.onBeforeCompile = (shader) => {
    shader.uniforms.uHidden = { value: hidden };
    shader.vertexShader = shader.vertexShader
      .replace("#include <common>", "#include <common>\nattribute float aTile;\nvarying float vTile;")
      .replace("#include <begin_vertex>", "#include <begin_vertex>\nvTile = aTile;");
    shader.fragmentShader = shader.fragmentShader
      .replace("#include <common>", "#include <common>\nuniform float uHidden[256];\nvarying float vTile;")
      .replace("#include <color_fragment>", "if (vTile >= 0.0 && uHidden[int(vTile + 0.5)] > 0.5) discard;\n#include <color_fragment>");
  };
  material.customProgramCacheKey = () => "far-trees";
  const mesh = new THREE.InstancedMesh(crown, material, trees.length);
  const matrix = new THREE.Matrix4();
  const colour = new THREE.Color();
  const quaternion = new THREE.Quaternion();
  let count = 0;
  for (let i = 0; i < trees.length; i += 1) {
    const tree = trees[i];
    const point = tree.point ?? { x_km: tree.x_km, y_km: tree.y_km };
    if (point.x_km === undefined || point.y_km === undefined) continue;
    const [x, z] = frame.toScene(point.x_km, point.y_km);
    const height = Math.max(tree.heightMetres ?? 9, 3);
    const id = tree.id ?? i;
    const form = CROWN_FORMS[tree.species ?? ""] ?? CROWN_FORMS.chinesePlane;
    const spread = Math.max(tree.crownRadiusMetres ?? height * 0.3, 1.2) * form.spread;
    const tall = Math.min(spread * form.tall, height * 0.5);
    // The crown's foot is where the clear trunk ends, so a willow hangs low and a
    // ginkgo carries its crown high.
    const y = frame.heightAt(x, z) + (height - tall * form.lift) * m;
    // Each crown turned and leaning a little its own way, so the lobes never repeat.
    quaternion.setFromEuler(new THREE.Euler((hash01(id, 7) - 0.5) * 0.15, hash01(id, 8) * Math.PI * 2, (hash01(id, 9) - 0.5) * 0.15));
    matrix.compose(new THREE.Vector3(x, y, z), quaternion, new THREE.Vector3(spread * m, tall * m, spread * m * (0.85 + 0.3 * hash01(id, 10))));
    mesh.setMatrixAt(count, matrix);
    colour.setHex(form.colours[Math.floor(hash01(id, 5) * form.colours.length)]);
    colour.multiplyScalar(0.88 + 0.24 * hash01(id, 6));
    mesh.setColorAt(count, colour);
    builder.select(...tileOfPoint(city.frame, point));
    tiles[count] = builder.tileIndex.get(tileKey(...tileOfPoint(city.frame, point))) ?? -1;
    if (tiles[count] >= MAX_MASS_TILES) tiles[count] = -1;
    count += 1;
  }
  mesh.count = count;
  crown.setAttribute("aTile", new THREE.InstancedBufferAttribute(tiles, 1));
  mesh.instanceMatrix.needsUpdate = true;
  if (mesh.instanceColor) mesh.instanceColor.needsUpdate = true;
  mesh.computeBoundingSphere();
  mesh.castShadow = true;
  mesh.customDepthMaterial = hiddenDepthMaterial(hidden);
  mesh.receiveShadow = true;
  mesh.name = "cityMass/trees";
  return {
    mesh,
    dispose() {
      crown.dispose();
      material.dispose();
      mesh.customDepthMaterial?.dispose();
    },
  };
}
