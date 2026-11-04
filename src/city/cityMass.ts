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
import { GROUND_GLSL, GROUND_KIND } from "./parcelGround";
import { drawCrown, planSpecies, speciesSize, streetSpecies } from "./treeCrown";
import {
  type FarLook, KIND_BLANK, KIND_CURTAIN, KIND_GRID, KIND_PUNCHED, KIND_RIBBON,
  blankLook, packLooks, podiumOf, shaftOf,
} from "./farLook";
import {
  MASS_COLOR_CHUNK, MASS_EMISSIVE_CHUNK, MASS_FRAGMENT_PARS, MASS_INTERIOR_DEFINE,
  MASS_ROUGHNESS_CHUNK, MASS_SPECULAR_CHUNK, MASS_VERTEX_MAIN, MASS_VERTEX_PARS,
} from "./massFacade";

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
// Flat roofs of buildings without a far look: the detail's bitumen membrane
// (about 0.165 linear, `roof_texture`), a hair apart so a town is not one swatch.
const ROOFS = [0x737171, 0x707070, 0x757372];
// Profiled steel cladding and the blue or grey steel roofs of sheds.
const SHED_WALLS = [0xdcdedd, 0xc5c9cc, 0xaebccb, 0xd6cfbd, 0xbfc3bf];
const SHED_ROOFS = [0x5c7d9e, 0x4f6f8f, 0x8a9399, 0xa5acb0, 0x6e8aa0];
// Village houses: the measured palette of the detail (`cn_residential::
// VILLAGE_PAINTS`) as sRGB, each colour repeated by its share.
const VILLAGE_WALLS = [
  0xceb5a2, 0xceb5a2, 0xceb5a2, 0xceb5a2, 0xceb5a2, 0xc09e8e, 0xc09e8e, 0xc09e8e, 0xc09e8e, 0xac8e84, 0xac8e84, 0xac8e84, 0xac8e84, 0xd7c5b5, 0xd7c5b5, 0xd7c5b5, 0xd7c5b5, 0xa27973, 0xa27973, 0xa27973, 0xddd6cb, 0xddd6cb, 0xddd6cb, 0xc4c0bc, 0xc4c0bc, 0xc4c0bc, 0xc4c0bc, 0xa29e99, 0xa29e99, 0xa29e99, 0xe1dfdb, 0xe1dfdb, 0x8e6961, 0x8e6961, 0xc08976, 0xc08976, 0x6c8995, 0xacbcc4, 0xaac4b3,
];
const STEEL_BLUE = 0x5f7f9f;
// A row of solar water heaters seen from the air: white tanks and dark tube racks
// with gaps, so about 0.15 linear on average, not the racks' own near-black.
const SOLAR = 0x6e757c;
const TANK = 0xd8d8d0;
// Louvre screen of a plant enclosure: pale blades with dark gaps, mean about 0.3.
const PLANT = 0x949494;
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
  solar?: boolean,
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
    // The detail's plant screen: a ring of louvres 3.2 m high, 3.5 m in from the
    // parapet, open to the roof inside. Not a solid lid, which read as a black slab.
    const screen = insetRing(ring, 3.5 * m);
    if (screen && ringArea(screen) > 60 * m * m) builder.parapet(screen, roofTop, 3.2 * m, PLANT);
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
  if (solar ?? (b.floors <= 7 && h(79) < 0.75)) {
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
  /** Per vertex: the row of the look table its facade is drawn from, or -1. */
  readonly blds: number[] = [];
  /** Per vertex: metres-free position along its wall and the wall's length, scene units. */
  readonly faces: number[] = [];
  /** The look row of the primitives being built, or -1 for none. */
  bld = -1;
  /** The look row of things with no facade (plant, parapets, roofs), drawn in their own colour. */
  plainBld = -1;
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

  private push(a: number[], b: number[], c: number[], normal: number[], hex: number, shade: number, wall?: { u: [number, number, number]; length: number }) {
    this.colour.setHex(hex);
    [a, b, c].forEach((p, i) => {
      this.positions.push(p[0], p[1], p[2]);
      this.normals.push(normal[0], normal[1], normal[2]);
      this.colors.push(this.colour.r * shade, this.colour.g * shade, this.colour.b * shade);
      this.tileIds.push(this.current);
      this.bases.push(this.base);
      this.seeds.push(this.seed);
      this.blds.push(this.bld);
      this.faces.push(wall ? wall.u[i] : 0, wall ? wall.length : 1);
    });
  }

  /** A triangle whose winding is corrected to face `normal`. */
  tri(a: number[], b: number[], c: number[], normal: number[], hex: number, shade = 1, wall?: { u: [number, number, number]; length: number }) {
    const ux = b[0] - a[0], uy = b[1] - a[1], uz = b[2] - a[2];
    const vx = c[0] - a[0], vy = c[1] - a[1], vz = c[2] - a[2];
    const nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
    if (nx * normal[0] + ny * normal[1] + nz * normal[2] >= 0) this.push(a, b, c, normal, hex, shade, wall);
    else this.push(a, c, b, normal, hex, shade, wall && { u: [wall.u[0], wall.u[2], wall.u[1]], length: wall.length });
  }

  /**
   * A prism over `ring` from `bottom` to `top`, walls and flat roof. `datum` is
   * where its storeys count from (the building's ground; the top of a podium for
   * the shaft above it), `roofToo` false leaves the lid off for a tier that
   * another stands on.
   */
  prism(ring: Ring, bottom: number, top: number, wall: number, roof: number, datum = bottom, roofToo = true) {
    const n = ring.length;
    if (n < 3 || !(top > bottom)) return;
    this.base = datum;
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
      // The sun and the shadow map shade the walls; nothing is baked here, so the
      // far form takes the same light as the detail does.
      this.tri([ax, bottom, az], [bx, bottom, bz], [bx, top, bz], normal, wall, 1, { u: [0, len, len], length: len });
      this.tri([ax, bottom, az], [bx, top, bz], [ax, top, az], normal, wall, 1, { u: [0, len, 0], length: len });
    }
    if (!roofToo) return;
    const contour = ring.map(([x, z]) => new THREE.Vector2(x, z));
    for (const [i, j, k] of THREE.ShapeUtils.triangulateShape(contour, [])) {
      this.tri([ring[i][0], top, ring[i][1]], [ring[j][0], top, ring[j][1]], [ring[k][0], top, ring[k][1]], [0, 1, 0], roof, 1);
    }
  }

  /**
   * A parapet round `ring` at `top`, `height` high: the outer face, and the inner
   * face the roof's far edge shows, so a flat roof reads as a roof inside a rim.
   */
  parapet(ring: Ring, top: number, height: number, hex: number) {
    const n = ring.length;
    if (n < 3 || !(height > 0)) return;
    let cx = 0, cz = 0;
    for (const [x, z] of ring) { cx += x; cz += z; }
    cx /= n; cz /= n;
    this.base = top;
    for (let i = 0; i < n; i += 1) {
      const [ax, az] = ring[i];
      const [bx, bz] = ring[(i + 1) % n];
      const ex = bx - ax, ez = bz - az;
      const len = Math.hypot(ex, ez);
      if (len < 1e-9) continue;
      let nx = ez / len, nz = -ex / len;
      if (nx * ((ax + bx) * 0.5 - cx) + nz * ((az + bz) * 0.5 - cz) < 0) { nx = -nx; nz = -nz; }
      for (const sign of [1, -1]) {
        const normal = [nx * sign, 0, nz * sign];
        this.tri([ax, top, az], [bx, top, bz], [bx, top + height, bz], normal, hex, 1, { u: [0, len, len], length: len });
        this.tri([ax, top, az], [bx, top + height, bz], [ax, top + height, az], normal, hex, 1, { u: [0, len, 0], length: len });
      }
    }
  }

  /**
   * A pitched roof over the oriented rectangle that bounds `ring`: a hip, whose
   * ridge is shortened by the roof's own half-width at each end, or a gable
   * with vertical ends. `eave` overhangs the walls, `pitch` is rise over run.
   */
  pitchedRoof(ring: Ring, top: number, eave: number, pitch: number, hip: boolean, hex: number, gableHex: number) {
    let ux = 1, uz = 0, best = 0;
    for (let i = 0; i < ring.length; i += 1) {
      const [ax, az] = ring[i];
      const [bx, bz] = ring[(i + 1) % ring.length];
      const len = Math.hypot(bx - ax, bz - az);
      if (len > best) { best = len; ux = (bx - ax) / len; uz = (bz - az) / len; }
    }
    const cx = ring.reduce((sum, p) => sum + p[0], 0) / ring.length;
    const cz = ring.reduce((sum, p) => sum + p[1], 0) / ring.length;
    let a0 = Infinity, a1 = -Infinity, w0 = Infinity, w1 = -Infinity;
    for (const [x, z] of ring) {
      const a = (x - cx) * ux + (z - cz) * uz;
      const w = -(x - cx) * uz + (z - cz) * ux;
      a0 = Math.min(a0, a); a1 = Math.max(a1, a); w0 = Math.min(w0, w); w1 = Math.max(w1, w);
    }
    const at = (a: number, w: number, y: number): number[] => [cx + ux * a - uz * w, y, cz + uz * a + ux * w];
    const halfW = (w1 - w0) * 0.5 + eave;
    const rise = halfW * pitch;
    const y0 = top;
    const y1 = top + rise;
    const wm = (w0 + w1) * 0.5;
    const ea0 = a0 - eave, ea1 = a1 + eave;
    const r0 = hip ? Math.min(a0 + (w1 - w0) * 0.5, (a0 + a1) * 0.5) : ea0 + eave;
    const r1 = hip ? Math.max(a1 - (w1 - w0) * 0.5, (a0 + a1) * 0.5) : ea1 - eave;
    const face = (p: number[][], colour: number) => {
      const nx = (p[1][1] - p[0][1]) * (p[2][2] - p[0][2]) - (p[1][2] - p[0][2]) * (p[2][1] - p[0][1]);
      const ny = (p[1][2] - p[0][2]) * (p[2][0] - p[0][0]) - (p[1][0] - p[0][0]) * (p[2][2] - p[0][2]);
      const nz = (p[1][0] - p[0][0]) * (p[2][1] - p[0][1]) - (p[1][1] - p[0][1]) * (p[2][0] - p[0][0]);
      const l = Math.hypot(nx, ny, nz) || 1;
      const sgn = ny < 0 ? -1 : 1;
      const normal = [(nx / l) * sgn, (ny / l) * sgn, (nz / l) * sgn];
      this.tri(p[0], p[1], p[2], normal, colour);
      if (p.length === 4) this.tri(p[0], p[2], p[3], normal, colour);
    };
    // The two long slopes.
    face([at(ea0, w0 - eave, y0), at(ea1, w0 - eave, y0), at(r1, wm, y1), at(r0, wm, y1)], hex);
    face([at(ea0, w1 + eave, y0), at(ea1, w1 + eave, y0), at(r1, wm, y1), at(r0, wm, y1)], hex);
    // The ends: a hip slope, or a gable wall in the wall's colour.
    if (hip) {
      face([at(ea0, w0 - eave, y0), at(ea0, w1 + eave, y0), at(r0, wm, y1)], hex);
      face([at(ea1, w0 - eave, y0), at(ea1, w1 + eave, y0), at(r1, wm, y1)], hex);
    } else {
      face([at(a0, w0, y0), at(a0, w1, y0), at(a0, wm, y1)], gableHex);
      face([at(a1, w0, y0), at(a1, w1, y0), at(a1, wm, y1)], gableHex);
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
    const bld = this.bld;
    this.seed = PLAIN_SEED + (seed % 1);
    this.bld = this.plainBld;
    this.prism(ring, bottom, top, hex, hex);
    this.seed = seed;
    this.bld = bld;
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

  /** A triangle with a colour per corner, facing up: water and its banks. */
  smoothTri(a: number[], ca: THREE.Color, b: number[], cb: THREE.Color, c: number[], cc: THREE.Color) {
    const up = (b[0] - a[0]) * (c[2] - a[2]) - (b[2] - a[2]) * (c[0] - a[0]) < 0;
    const corners: Array<[number[], THREE.Color]> = up ? [[a, ca], [b, cb], [c, cc]] : [[a, ca], [c, cc], [b, cb]];
    for (const [p, colour] of corners) {
      this.positions.push(p[0], p[1], p[2]);
      this.normals.push(0, 1, 0);
      this.colors.push(colour.r, colour.g, colour.b);
      this.tileIds.push(this.current);
      this.bases.push(0);
      this.seeds.push(PLAIN_SEED);
      this.blds.push(-1);
      this.faces.push(0, 1);
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
    g.setAttribute("aBld", new THREE.Float32BufferAttribute(this.blds, 1));
    g.setAttribute("aFace", new THREE.Float32BufferAttribute(this.faces, 2));
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

/**
 * A tree's crown from afar: the species' own form, size and colour, the same
 * table the detail trees are grown from (see treeCrown.ts).
 */
function crown(builder: MassBuilder, x: number, ground: number, z: number, height: number, radius: number, species: string) {
  drawCrown(builder, x, ground, z, height, radius, species, Math.abs(Math.sin(x * 12.9898 + z * 78.233) * 43758.5453) % 1);
}

export interface CityMass {
  mesh: THREE.Mesh;
  /** The river and its tributaries as water, or null where the plan has none. */
  water: THREE.Mesh | null;
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

/**
 * A river reach drawn from afar: the plan's centre line, rounded by corner
 * cutting, as a continuous ribbon of the plan's width. Across it the colour goes
 * from silt-green shallows to the dark, turbid channel, and a strip of wet mud
 * and reed edges it where it meets the grass, so it has banks and not a painted
 * line. Each vertex takes the highest ground across the section, so the surface
 * never sinks into a valley side. Sky reflection is added by the material.
 */
const WATER_PROFILE: Array<{ at: number; colour: number }> = [
  { at: -1, colour: 0x4d5a3e }, // reed and wet mud on the bank
  { at: -0.9, colour: 0x6f7b5f },
  { at: -0.62, colour: 0x4f6860 },
  { at: -0.25, colour: 0x37535c },
  { at: 0, colour: 0x2f4a56 }, // the channel
  { at: 0.25, colour: 0x37535c },
  { at: 0.62, colour: 0x4f6860 },
  { at: 0.9, colour: 0x6f7b5f },
  { at: 1, colour: 0x4d5a3e },
];

/** The river's colours from the terrain simulation: sediment and climate set turbidity and bank material. */
function waterProfileFor(setting: ModernCityPayload["riverSetting"]): Array<{ at: number; colour: number }> {
  if (!setting) return WATER_PROFILE;
  const moisture = Math.min(1, Math.max(0, setting.moisture));
  const turb = Math.min(1, Math.max(0, 0.3 + 1.2 * setting.sediment + 0.25 * (1 - moisture)));
  const lerp = (a: number, b: number, t: number) => new THREE.Color(a).lerp(new THREE.Color(b), t).getHex();
  const channel = lerp(0x2c4a52, 0x4a4a30, turb);
  const mid = lerp(0x37585c, 0x5a553a, turb);
  const shallow = lerp(0x4f6860, 0x7a7048, turb);
  // Dry climates leave pale gravel and sand at the edge, wet ones dark silt and reeds.
  const edge = lerp(0x4d5a3e, 0x9a9078, Math.min(1, (1 - moisture) * 1.4));
  const edgeIn = lerp(0x6f7b5f, 0xb0a384, Math.min(1, (1 - moisture) * 1.4));
  return [
    { at: -1, colour: edge },
    { at: -0.9, colour: edgeIn },
    { at: -0.62, colour: shallow },
    { at: -0.25, colour: mid },
    { at: 0, colour: channel },
    { at: 0.25, colour: mid },
    { at: 0.62, colour: shallow },
    { at: 0.9, colour: edgeIn },
    { at: 1, colour: edge },
  ];
}

function waterReach(
  builder: MassBuilder,
  line: Array<{ x_km: number; y_km: number }>,
  widthMetres: number,
  frame: CityMassFrame,
  select: (p: { x_km: number; y_km: number }) => void,
  setting?: ModernCityPayload["riverSetting"],
): void {
  const WATER_PROFILE = waterProfileFor(setting);
  const m = frame.metresToScene;
  let pts = line.map((p) => frame.toScene(p.x_km, p.y_km));
  if (pts.length < 2) return;
  for (let pass = 0; pass < 3 && pts.length < 600; pass += 1) {
    const next: Array<[number, number]> = [pts[0]];
    for (let i = 0; i + 1 < pts.length; i += 1) {
      const [ax, az] = pts[i], [bx, bz] = pts[i + 1];
      next.push([ax * 0.75 + bx * 0.25, az * 0.75 + bz * 0.25], [ax * 0.25 + bx * 0.75, az * 0.25 + bz * 0.75]);
    }
    next.push(pts[pts.length - 1]);
    pts = next;
  }
  const half = widthMetres * 0.5 * m;
  const outer = half + Math.max(2.0 * m, half * 0.1);
  const n = pts.length;
  const normal: Array<[number, number]> = pts.map((_, i) => {
    const a = pts[Math.max(0, i - 1)], b = pts[Math.min(n - 1, i + 1)];
    const dx = b[0] - a[0], dz = b[1] - a[1];
    const len = Math.hypot(dx, dz) || 1;
    return [-dz / len, dx / len];
  });
  const surface = pts.map(([x, z], i) => {
    let y = frame.heightAt(x, z);
    for (const side of [-1, 1]) {
      y = Math.max(y, frame.heightAt(x + normal[i][0] * half * side, z + normal[i][1] * half * side));
    }
    return y + 0.22 * m;
  });
  const colours = WATER_PROFILE.map((s) => new THREE.Color(s.colour));
  const turbid = new THREE.Color(0x5f6a4a);
  let arc = 0;
  for (let i = 0; i + 1 < n; i += 1) {
    select(line[Math.min(line.length - 1, Math.round((i / (n - 1)) * (line.length - 1)))]);
    arc += Math.hypot(pts[i + 1][0] - pts[i][0], pts[i + 1][1] - pts[i][1]);
    // Slowly varying turbidity along the reach: brown-green patches of silt.
    const patch = 0.5 + 0.5 * Math.sin((arc / m) * 0.011 + 1.3) * Math.sin((arc / m) * 0.031);
    const vertex = (index: number, k: number) => {
      const t = WATER_PROFILE[k].at;
      const off = t * (Math.abs(t) === 1 ? outer : half);
      const colour = colours[k].clone();
      if (k > 0 && k < WATER_PROFILE.length - 1) colour.lerp(turbid, patch * 0.28);
      return { p: [pts[index][0] + normal[index][0] * off, surface[index], pts[index][1] + normal[index][1] * off], colour };
    };
    for (let k = 0; k + 1 < WATER_PROFILE.length; k += 1) {
      const a0 = vertex(i, k), a1 = vertex(i, k + 1), b0 = vertex(i + 1, k), b1 = vertex(i + 1, k + 1);
      builder.smoothTri(a0.p, a0.colour, a1.p, a1.colour, b1.p, b1.colour);
      builder.smoothTri(a0.p, a0.colour, b1.p, b1.colour, b0.p, b0.colour);
    }
  }
}


const hexScratch = new THREE.Color();
/** A linear colour as the sRGB hex the builder takes. */
function linearHex(rgb: [number, number, number]): number {
  return hexScratch.setRGB(rgb[0], rgb[1], rgb[2]).getHex();
}
/** The linear colour of an sRGB hex. */
function hexLinear(hex: number): [number, number, number] {
  hexScratch.setHex(hex);
  return [hexScratch.r, hexScratch.g, hexScratch.b];
}

/**
 * A ring moved inward by `d` (scene units), corner by corner: the shaft of a
 * podium tower. Null where the result collapses, flips, or is under a third of
 * the plan.
 */
function ringArea(ring: Ring): number {
  let a = 0;
  for (let i = 0; i < ring.length; i += 1) {
    const [ax, az] = ring[i];
    const [bx, bz] = ring[(i + 1) % ring.length];
    a += ax * bz - bx * az;
  }
  return Math.abs(a) * 0.5;
}

function insetRing(ring: Ring, d: number): Ring | null {
  const n = ring.length;
  if (n < 3 || d <= 0) return null;
  let area = 0;
  for (let i = 0; i < n; i += 1) {
    const [ax, az] = ring[i];
    const [bx, bz] = ring[(i + 1) % n];
    area += ax * bz - bx * az;
  }
  const sign = area >= 0 ? 1 : -1;
  const lines = ring.map((p, i) => {
    const q = ring[(i + 1) % n];
    const len = Math.hypot(q[0] - p[0], q[1] - p[1]) || 1;
    // Inward is to the left of a counter-clockwise edge (in x, z with sign).
    const nx = (-(q[1] - p[1]) / len) * sign;
    const nz = ((q[0] - p[0]) / len) * sign;
    return { px: p[0] + nx * d, pz: p[1] + nz * d, dx: (q[0] - p[0]) / len, dz: (q[1] - p[1]) / len };
  });
  const out: Ring = [];
  for (let i = 0; i < n; i += 1) {
    const a = lines[(i + n - 1) % n];
    const b = lines[i];
    const cross = a.dx * b.dz - a.dz * b.dx;
    if (Math.abs(cross) < 1e-6) { out.push([b.px, b.pz]); continue; }
    const t = ((b.px - a.px) * b.dz - (b.pz - a.pz) * b.dx) / cross;
    out.push([a.px + a.dx * t, a.pz + a.dz * t]);
  }
  let inner = 0;
  for (let i = 0; i < n; i += 1) {
    inner += out[i][0] * out[(i + 1) % n][1] - out[(i + 1) % n][0] * out[i][1];
  }
  if (inner * sign <= 0 || Math.abs(inner) < Math.abs(area) / 3) return null;
  return out;
}

/**
 * The look of a building the facade grammar does not describe, from the plan:
 * the wall palette by use, storey height from the plan's own height and floors,
 * and the bay and opening share by facade kind (`procedural::facade_metrics`).
 */
function planLook(b: CityMassBuilding, shed: boolean): FarLook {
  const towers = b.heightMetres >= 40 && b.useType === "commercial";
  const palette = shed ? SHED_WALLS : b.useType === "village" ? VILLAGE_WALLS : towers ? TOWER_WALLS : WALLS;
  let wallHex = palette[Math.floor(hash01(b.id, 1) * palette.length)];
  if (wallHex === BRICK && b.heightMetres > 25) wallHex = WALLS[0];
  const wall = hexLinear(wallHex);
  const look = blankLook(wall);
  const shop = b.useType === "commercial" || b.useType === "mixedUse";
  const shopM = shop && b.floors >= 2 ? 4.5 : 0;
  const storeys = Math.max(b.floors - (shopM > 0 ? 1 : 0), 1);
  look.shopM = shopM;
  look.roofM = Math.max(b.heightMetres, 3);
  look.storeyM = Math.min(Math.max((look.roofM - shopM) / storeys, 2.7), 4.8);
  look.weather = 0.2 + 0.4 * hash01(b.id, 5);
  look.baseBand = [wall[0] * 0.7, wall[1] * 0.7, wall[2] * 0.7];
  look.topBand = wall;
  look.trim = wall;
  look.glassKind = b.useType === "commercial" ? 2 : hash01(b.id, 7) < 0.4 ? 1 : 0;
  const tint = hash01(b.id, 8);
  look.glass = tint < 0.5 ? [0.30, 0.55, 0.52] : [0.28, 0.42, 0.58];
  look.frame = [0.2, 0.2, 0.2];
  look.flags = hash01(b.id, 9) < 0.35 ? 2 : 0;
  const tall = (b.heightMetres >= 24);
  const compact = Math.max(...b.footprint.map((p) => Math.hypot(p.x_km - b.footprint[0].x_km, p.y_km - b.footprint[0].y_km))) * 1000 < 34;
  if (shed) {
    look.kind = KIND_BLANK;
  } else if (b.facade === "curtainWall") {
    look.kind = KIND_CURTAIN;
    look.moduleM = 1.2 + 0.6 * hash01(b.id, 10);
    look.unitW = look.moduleM * 3;
    look.glassKind = 3;
    look.glass = [[0.42, 0.66, 0.66], [0.38, 0.52, 0.74], [0.66, 0.70, 0.72], [0.30, 0.33, 0.36]][Math.floor(hash01(b.id, 11) * 4)] as [number, number, number];
    look.frame = [0.075, 0.08, 0.085];
    look.window = [0, look.unitW, 0.9, look.storeyM];
  } else if (b.facade === "concreteGlass" || b.facade === "stoneCivic") {
    look.kind = b.facade === "stoneCivic" ? KIND_GRID : KIND_RIBBON;
    look.unitW = (b.facade === "stoneCivic" ? 4.9 : 4.6) * (compact ? 0.78 : 1);
    const pier = look.kind === KIND_GRID ? 0.9 : 0.3;
    look.window = [pier, look.unitW - pier, 0.8, look.storeyM - 0.5];
    look.glassKind = 2;
  } else {
    look.kind = KIND_PUNCHED;
    look.unitW = tall ? 3.3 : 3.6;
    const balcony = !!b.balconyBays && b.balconyBays > 0 && b.useType !== "village";
    look.window = balcony ? [0.35, 1.55, 0.9, Math.min(2.4, look.storeyM - 0.45)] : [look.unitW * 0.5 - 0.9, look.unitW * 0.5 + 0.9, 0.9, Math.min(2.4, look.storeyM - 0.45)];
    if (balcony) look.balcony = [1.75, look.unitW - 0.35, look.storeyM - 0.45];
  }
  return look;
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
  // The detail's block plate is grass (`block.ground`), so what no parcel covers
  // is a drier, trampled lawn, not a grey slab.
  for (const block of city.blocks) {
    builder.select(...tileOfPoint(city.frame, ringMean(block.boundary)));
    builder.seed = GROUND_KIND.lawn + hash01(block.boundary.length, 9) * 0.999;
    builder.flat(toRing(block.boundary), frame.heightAt, 0.15 * m, 0x6c7a4e);
  }

  // Each parcel's ground over its block, by what it is used for: from a few
  // hundred metres a town is lawns, forecourts and yards, not one grey slab.
  // Base colour and surface kind. The kind (parcelGround.ts) is drawn by the
  // shader as joints, bays, containers, stains; a use with several surfaces
  // picks one by the parcel's id, so neighbouring lots differ.
  const PARCEL_GROUND: Record<string, number> = {
    park: 0x4e6b36,
    residential: 0x667c4c,
    villa: 0x5f7746,
    farmstead: 0x6b7447,
    village: 0x7a7a62,
    commercial: 0x6f6d69,
    mixedUse: 0x74726d,
    civic: 0x9a968c,
    industrial: 0x8c877c,
  };
  const groundOf = (useType: string, id: number): { colour: number; kind: number } => {
    // The detail's own rule (`hard_standing` in buildings/mod.rs).
    const pick = ((id * 7919) % 100) / 100;
    switch (useType) {
      case "industrial":
        return pick < 0.55
          ? { colour: PARCEL_GROUND.industrial, kind: GROUND_KIND.yard }
          : { colour: 0x62605c, kind: GROUND_KIND.asphaltLot };
      case "commercial":
      case "mixedUse":
        return pick < 0.7
          ? { colour: PARCEL_GROUND[useType], kind: GROUND_KIND.parking }
          : { colour: PARCEL_GROUND.civic, kind: GROUND_KIND.plaza };
      case "civic":
        return { colour: PARCEL_GROUND.civic, kind: GROUND_KIND.plaza };
      case "park":
      case "residential":
      case "villa":
      case "farmstead":
        return { colour: PARCEL_GROUND[useType], kind: GROUND_KIND.lawn };
      default:
        return { colour: PARCEL_GROUND[useType] ?? PAVING, kind: PLAIN_SEED };
    }
  };
  for (const parcel of city.parcels) {
    builder.select(...tileOfPoint(city.frame, ringMean(parcel.ring)));
    const ground = groundOf(parcel.useType, parcel.id);
    builder.seed = ground.kind + hash01(parcel.id, 6) * 0.999;
    builder.flat(toRing(parcel.ring), frame.heightAt, 0.25 * m, ground.colour);
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
    const species = planSpecies(tree.species);
    const size = speciesSize(species);
    crown(builder, x, y, z, (tree.heightMetres ?? size.height) * m, Math.max(tree.crownRadiusMetres ?? size.crown, 1.5) * m, species);
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
          const species = streetSpecies(city.streetMix, x, z);
          const size = speciesSize(species);
          crown(builder, x, ay + (by - ay) * t - 0.35 * m, z, size.height * m, size.crown * m, species);
          carry += 8 * m;
        }
        carry -= len;
      }
    }
  }

  builder.seed = 0;
  // One row of the look table per building (two for a podium and its shaft).
  const looks: Array<{ look: FarLook; storeys: number }> = [];
  const lookRow = (look: FarLook, tierMetres: number): number => {
    const above = Math.max(1, Math.round((tierMetres - look.shopM) / Math.max(look.storeyM, 2.4)));
    looks.push({ look, storeys: above });
    return looks.length - 1;
  };
  builder.plainBld = lookRow(blankLook([0.5, 0.5, 0.5]), 3);
  for (const b of city.buildings) {
    builder.select(...(parcelTile.get(b.parcelId) ?? tileOfPoint(city.frame, ringMean(b.footprint))));
    const ring = toRing(b.footprint);
    if (ring.length < 3) continue;
    let base = Infinity;
    for (const [x, z] of ring) base = Math.min(base, frame.heightAt(x, z));
    // Sink the foot below the lowest corner so a building on a slope has no gap.
    const startM = b.farLook?.startM ?? 0;
    // A tower on another building's podium starts at its roof; the rest sink the foot.
    const foot = startM > 0 ? base + startM * m : base - 1.5 * m;
    const shed = b.useType === "industrial" && b.floors <= 1;
    // The far look is the detail's own design; a plan without one (or a kind the
    // grammar does not draw) gets the look its plan metrics imply.
    const look = b.farLook ?? planLook(b, shed);
    // The detail's roof deck height, so the building does not change size when
    // its detail arrives.
    const roofM = b.farLook ? look.roofM : Math.max(b.heightMetres, 3);
    const total = roofM * m;
    const wall = linearHex(look.wall);
    // The roof is the detail's own covering (bitumen membrane, grey or red clay),
    // so the roof does not change colour at the hand-off.
    const roof = b.farLook
      ? linearHex(look.roof)
      : shed
        ? SHED_ROOFS[Math.floor(hash01(b.id, 2) * SHED_ROOFS.length)]
        : b.roof === "terracotta"
          ? TERRACOTTA
          : ROOFS[Math.floor(hash01(b.id, 2) * ROOFS.length)];
    const roofTop = base + total;
    builder.seed = hash01(b.id, 61) * 0.999 + (shed ? SHED_SEED : 0);
    const topPaint = linearHex(look.topN > 0 || look.baseN > 0 ? look.topBand : look.wall);
    const parapetM = shed || roofM < 9 ? 0 : b.useType === "commercial" ? (b.floors >= 12 ? 1.6 : 1.1) : 1.2;
    // A hip roof where the design has one, a gable or hip where the plan says so.
    const tower = b.floors >= 19;
    const designHip = b.farLook ? (look.crown === 2 && !tower) || (look.crown === 4 && !tower && b.floors <= 7) || (look.crown === 3 && !tower) : false;
    const planPitch = !b.farLook && (b.roof === "gable" || b.roof === "hip") && ring.length === 4;
    const pitched = (designHip || planPitch) && ring.length === 4;
    const clay = b.farLook ? roof : b.roof === "terracotta" ? TERRACOTTA : 0x7d5a4a;
    const podiumTop = look.podiumFloors > 0 ? (look.shopM + look.podiumFloors * look.podiumStoreyM) * m : 0;
    const shaftRing = startM === 0 && podiumTop > 0 && podiumTop < total - 6 * m ? insetRing(ring, look.setbackM * m) : null;
    if (shaftRing) {
      // A retail podium under a shaft set back from it, as the detail builds it.
      builder.bld = lookRow(podiumOf(look), podiumTop / m);
      builder.prism(ring, foot, base + podiumTop, wall, roof, base, true);
      builder.bld = builder.plainBld;
      builder.parapet(ring, base + podiumTop, 1.1 * m, topPaint);
      builder.bld = lookRow(shaftOf(look), (total - podiumTop) / m);
      builder.prism(shaftRing, base + podiumTop, roofTop, wall, roof, base + podiumTop, true);
    } else if (b.tierRing && b.tierRing.length >= 3 && (b.podiumHeightMetres ?? 0) > 0 && (b.podiumHeightMetres ?? 0) * m < total) {
      const podium = (b.podiumHeightMetres ?? 0) * m;
      builder.bld = lookRow(look, b.podiumHeightMetres ?? 0);
      builder.prism(ring, foot, base + podium, wall, roof, base, true);
      builder.bld = lookRow(shaftOf(look), roofM - (b.podiumHeightMetres ?? 0));
      builder.prism(toRing(b.tierRing), base + podium, roofTop, wall, roof, base + podium, true);
    } else {
      builder.bld = lookRow(look, roofM - startM);
      builder.prism(ring, foot, roofTop, wall, roof, startM > 0 ? foot : base, !pitched);
      if (pitched) {
        builder.bld = builder.plainBld;
        const hipRoof = designHip || b.roof === "hip";
        builder.pitchedRoof(ring, roofTop, 0.6 * m, look.crown === 3 ? 0.7 : designHip ? 0.5 : 0.62, hipRoof, clay, wall);
      }
    }
    builder.bld = builder.plainBld;
    if (!pitched && parapetM > 0) builder.parapet(shaftRing ?? ring, roofTop, parapetM * m, topPaint);
    if (!pitched) dressRoof(builder, b, shaftRing ?? ring, roofTop, wall, shed, m, b.farLook ? look.solar : undefined);
    builder.bld = -1;
  }

  // Water: its own mesh and material, since it reflects the sky instead of being
  // a plaster wall with windows. Hidden per tile with the same uniform.
  const waterBuilder = new MassBuilder();
  const selectWater = (p: { x_km: number; y_km: number }) => {
    const [c, r] = tileOfPoint(city.frame, p);
    const key = tileKey(c, r);
    let index = builder.tileIndex.get(key);
    if (index === undefined) {
      index = builder.tileIndex.size;
      builder.tileIndex.set(key, index);
    }
    waterBuilder.tileIndex.set(key, index);
    waterBuilder.select(c, r);
  };
  if (city.river && city.river.length >= 2) {
    waterReach(waterBuilder, city.river, city.riverWidthMetres ?? 60, frame, selectWater, city.riverSetting);
  }
  for (const tributary of city.tributaries ?? []) {
    if (tributary.line.length >= 2) waterReach(waterBuilder, tributary.line, tributary.widthMetres, frame, selectWater, city.riverSetting);
  }
  let water: THREE.Mesh | null = null;
  let waterMaterial: THREE.MeshStandardMaterial | null = null;
  if (waterBuilder.triangles() > 0) {
    waterMaterial = new THREE.MeshStandardMaterial({
      vertexColors: true,
      roughness: 0.32,
      metalness: 0,
      envMapIntensity: 0.4,
      polygonOffset: true,
      polygonOffsetFactor: -3,
      polygonOffsetUnits: -3,
    });
    waterMaterial.onBeforeCompile = (shader) => {
      shader.uniforms.uHidden = { value: hidden };
      shader.vertexShader = shader.vertexShader
        .replace("#include <common>", "#include <common>\nattribute float aTile;\nvarying float vTile;\nvarying vec3 vWaterWorld;")
        .replace("#include <worldpos_vertex>", "#include <worldpos_vertex>\nvTile = aTile;\nvWaterWorld = (modelMatrix * vec4(transformed, 1.0)).xyz;");
      shader.fragmentShader = shader.fragmentShader
        .replace("#include <common>", "#include <common>\nuniform float uHidden[256];\nvarying float vTile;\nvarying vec3 vWaterWorld;")
        .replace("#include <color_fragment>", "if (vTile >= 0.0 && uHidden[int(vTile + 0.5)] > 0.5) discard;\n#include <color_fragment>")
        .replace(
          "#include <emissivemap_fragment>",
          `#include <emissivemap_fragment>
          {
            // Sky on the surface: a little face-on, most of it at a grazing angle.
            float facing = clamp(abs(dot(normalize(vViewPosition), normal)), 0.0, 1.0);
            float fresnel = pow(1.0 - facing, 3.0);
            vec3 sky = vec3(0.30, 0.44, 0.60);
            // A broad, slow, non-repeating swell moves the reflection about; it is
            // value noise on world position, averaged out once a pixel spans it, so
            // from the air the river is a smooth sky reflection.
            vec2 wp = vWaterWorld.xz / 0.0016;
            vec2 wi = floor(wp); vec2 wf = fract(wp); wf = wf * wf * (3.0 - 2.0 * wf);
            float h00 = fract(sin(dot(wi, vec2(127.1, 311.7))) * 43758.5453);
            float h10 = fract(sin(dot(wi + vec2(1.0, 0.0), vec2(127.1, 311.7))) * 43758.5453);
            float h01 = fract(sin(dot(wi + vec2(0.0, 1.0), vec2(127.1, 311.7))) * 43758.5453);
            float h11 = fract(sin(dot(wi + vec2(1.0, 1.0), vec2(127.1, 311.7))) * 43758.5453);
            float swell = mix(mix(h00, h10, wf.x), mix(h01, h11, wf.x), wf.y);
            float pix = max(length(dFdx(wp)), length(dFdy(wp)));
            float ripple = mix(swell, 0.5, smoothstep(0.35, 1.2, pix));
            totalEmissiveRadiance += sky * (0.02 + 0.55 * fresnel) * (0.88 + 0.24 * ripple);
          }`,
        );
    };
    const wg = waterBuilder.geometry();
    water = new THREE.Mesh(wg, waterMaterial);
    water.name = "cityWater";
    water.receiveShadow = false;
    water.castShadow = false;
    water.renderOrder = 3;
  }

  const geometry = builder.geometry();
  const lookTexture = packLooks(looks);
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
  // The facade of every building, drawn analytically from its look (massFacade.ts):
  // floors, bays, windows, shopfronts and colour, each filtered by the pixel's own
  // footprint, so nothing switches with distance and nothing turns into a flat box.
  material.onBeforeCompile = (shader) => {
    shader.uniforms.uMetre = { value: m };
    shader.uniforms.uHidden = { value: hidden };
    shader.uniforms.uMassLook = { value: lookTexture };
    shader.vertexShader = shader.vertexShader
      .replace("#include <common>", "#include <common>\nattribute float aTile;\nattribute float aBase;\nattribute float aSeed;\nvarying float vTile;\nvarying float vBase;\nvarying float vSeed;\nvarying vec3 vMassWorld;\nvarying vec3 vMassNormal;\n" + MASS_VERTEX_PARS)
      .replace(
        "#include <worldpos_vertex>",
        "#include <worldpos_vertex>\nvTile = aTile;\nvBase = aBase;\nvSeed = aSeed;\nvMassWorld = (modelMatrix * vec4(transformed, 1.0)).xyz;\nvMassNormal = normalize(mat3(modelMatrix) * objectNormal);\n" + MASS_VERTEX_MAIN,
      );
    shader.fragmentShader = shader.fragmentShader
      .replace(
        "#include <common>",
        "#include <common>\n" + MASS_INTERIOR_DEFINE + "uniform float uMetre;\nuniform float uHidden[256];\nvarying float vTile;\nvarying float vBase;\nvarying float vSeed;\nvarying vec3 vMassWorld;\nvarying vec3 vMassNormal;\n" + GROUND_GLSL + MASS_FRAGMENT_PARS,
      )
      .replace(
        "#include <color_fragment>",
        `if (vTile >= 0.0 && uHidden[int(vTile + 0.5)] > 0.5) discard;
        ${MASS_COLOR_CHUNK}
        // Open ground of a parcel: joints, bays, containers, stains by use.
        if (seedKind > 3.5 && vMassNormal.y > 0.9) {
          vec2 gp = vMassWorld.xz / uMetre;
          diffuseColor.rgb = groundSurface(seedKind, diffuseColor.rgb, gp, length(fwidth(gp)), fract(vSeed));
        }
        `,
      )
      .replace("#include <roughnessmap_fragment>", MASS_ROUGHNESS_CHUNK)
      .replace("#include <lights_physical_fragment>", MASS_SPECULAR_CHUNK)
      .replace("#include <emissivemap_fragment>", MASS_EMISSIVE_CHUNK);
  };
  material.customProgramCacheKey = () => "city-mass-look";
  const mesh = new THREE.Mesh(geometry, material);
  mesh.name = "cityMass";
  // The sun's frustum is fitted to the view, so the towers cast shadows across the
  // streets they stand over, and a tower in another's shadow goes dark on its wall.
  mesh.castShadow = true;
  mesh.receiveShadow = true;
  return {
    mesh,
    water,
    triangles: builder.triangles() + waterBuilder.triangles(),
    setTileHidden(column, row, isHidden) {
      const index = builder.tileIndex.get(tileKey(column, row));
      if (index !== undefined && index < MAX_MASS_TILES) hidden[index] = isHidden ? 1 : 0;
    },
    dispose() {
      geometry.dispose();
      material.dispose();
      lookTexture.dispose();
      water?.geometry.dispose();
      waterMaterial?.dispose();
    },
  };
}
