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

/** Deterministic 0..1 from an integer id, so a building keeps its colour. */
function hash01(id: number, salt: number): number {
  let h = (id * 0x9e3779b1) ^ (salt * 0x85ebca6b);
  h ^= h >>> 16;
  h = Math.imul(h, 0x7feb352d);
  h ^= h >>> 15;
  return (h >>> 0) / 4294967295;
}

// Facade and roof albedos (sRGB). Real walls are 0.3-0.6, never white.
const WALLS = [0xc9c3b6, 0xb9b2a4, 0xd2cdc1, 0xa8a398, 0xbfb59f, 0x9fa3a3, 0xd8cbb0, 0x9c5b4a, 0xe6e4df];
// Towers are glass and cladding: cooler, darker, and more often blue-grey.
const TOWER_WALLS = [0x5b6670, 0x6d7780, 0x8a939b, 0xb8bcc0, 0x4d5b66];
const ROOFS = [0x6f6b66, 0x77736c, 0x5f5c59, 0x80766a, 0xc8c8c4, 0x9a9a96];
const TANK = 0xd8d8d0;
const PLANT = 0x5e5c58;
const TERRACOTTA = 0x8a5a45;
const ASPHALT = 0x4a4c4f;
// Paving, plazas and forecourts between the buildings of a block.
const PAVING = 0x8b8982;

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
    const ring: Ring = [
      [cx - half, cz - half], [cx + half, cz - half], [cx + half, cz + half], [cx - half, cz + half],
    ];
    this.prism(ring, bottom, top, hex, hex);
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
  for (const block of city.blocks) {
    builder.select(...tileOfPoint(city.frame, ringMean(block.boundary)));
    builder.flat(toRing(block.boundary), frame.heightAt, 0.15 * m, PAVING);
  }

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
    builder.ribbon(pts, Math.max(road.widthMetres, 6) * m, ASPHALT);
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
    const towers = b.heightMetres >= 40;
    const palette = towers ? TOWER_WALLS : WALLS;
    const wall = palette[Math.floor(hash01(b.id, 1) * palette.length)];
    const roof = b.roof === "terracotta" ? TERRACOTTA : ROOFS[Math.floor(hash01(b.id, 2) * ROOFS.length)];
    const podium = b.podiumHeightMetres ?? 0;
    const roofTop = base + total;
    builder.seed = hash01(b.id, 61);
    if (b.tierRing && b.tierRing.length >= 3 && podium > 0 && podium * m < total) {
      builder.prism(ring, foot, base + podium * m, wall, roof);
      builder.prism(toRing(b.tierRing), base + podium * m, base + total, wall, roof);
    } else {
      builder.prism(ring, foot, base + total, wall, roof);
    }
    // Roof plant on the taller blocks: a tank, a machine box, a stair house.
    if (b.heightMetres >= 15) {
      const cx = ring.reduce((s, p) => s + p[0], 0) / ring.length;
      const cz = ring.reduce((s, p) => s + p[1], 0) / ring.length;
      const reach = Math.sqrt(Math.max(...ring.map(([x, z]) => (x - cx) ** 2 + (z - cz) ** 2))) * 0.4;
      const count = 1 + Math.floor(hash01(b.id, 3) * 3);
      for (let k = 0; k < count; k += 1) {
        const angle = hash01(b.id, 10 + k) * Math.PI * 2;
        const dist = reach * hash01(b.id, 20 + k);
        const half = (1.2 + 2.2 * hash01(b.id, 30 + k)) * m;
        const h = (1.5 + 2.5 * hash01(b.id, 40 + k)) * m;
        const hex = hash01(b.id, 50 + k) < 0.4 ? TANK : PLANT;
        builder.box(cx + Math.cos(angle) * dist, cz + Math.sin(angle) * dist, half, roofTop, roofTop + h, hex);
      }
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
        `if (vTile >= 0.0 && uHidden[int(vTile + 0.5)] > 0.5) discard;
        #include <color_fragment>
        float wallness = 1.0 - smoothstep(0.35, 0.7, abs(vMassNormal.y));
        vec2 along = vec2(-vMassNormal.z, vMassNormal.x);
        float s = dot(vMassWorld.xz, along) / uMetre;
        // Height above the block's own ground, so the storeys start at the pavement.
        float hM = (vMassWorld.y - vBase) / uMetre;
        // Each building has its own bay and storey, its own kind of facade, and some
        // faces with no windows at all, so no two towers repeat one grid.
        float r1 = fract(vSeed * 7.13);
        float r2 = fract(vSeed * 13.71);
        float r3 = fract(vSeed * 29.37);
        float bayM = mix(2.8, 4.2, r1);
        float storeyM = mix(3.0, 3.9, r2);
        float faceR = fract(sin(dot(vMassNormal.xz, vec2(12.9898, 78.233)) + vSeed * 19.19) * 43758.5453);
        float faceOpen = step(0.18, faceR);
        float fh = max(fwidth(hM), 1e-4) / storeyM;
        float fs = max(fwidth(s), 1e-4) / bayM;
        float storey = fract(hM / storeyM);
        float bay = fract(s / bayM);
        // Three kinds: a glass curtain wall that is nearly all glass, punched windows
        // about half of each bay and storey, and a sparse block with many blank bays.
        bool curtain = r3 > 0.66;
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
        float fade = 1.0 - smoothstep(1.5, 3.0, max(fh, fs));
        // Single windows are missing here and there: a dark room, a closed unit.
        vec2 cell = vec2(floor(s / bayM), floor(hM / storeyM)) + vSeed * 97.0;
        float cellR = fract(sin(dot(cell, vec2(127.1, 311.7))) * 43758.5453);
        float presentMean = curtain ? 1.0 : (sparse ? 0.4 : 0.88);
        float presentNow = curtain ? 1.0 : (sparse ? smoothstep(0.55, 0.65, cellR) : smoothstep(0.1, 0.25, cellR));
        // Below about five pixels a cell's own value is noise, so it averages to the mean.
        float present = mix(presentMean, presentNow, 1.0 - smoothstep(0.1, 0.2, max(fh, fs)));
        // A wall darkens toward the pavement, where the block next door takes the sky.
        float baseOcclusion = mix(0.6, 1.0, smoothstep(0.0, 10.0, hM));
        diffuseColor.rgb *= mix(vec3(1.0), vec3(baseOcclusion), wallness);
        // Glass is darker and cooler than the wall, and each building's glass differs.
        float glass = rows * cols * present * faceOpen * fade * wallness;
        vec3 tint = mix(vec3(0.40, 0.46, 0.54), vec3(0.30, 0.38, 0.47), r2);
        diffuseColor.rgb *= mix(vec3(1.0), tint, glass);
        // Glass sits back in its reveal, so the head of each window is in shade.
        float head = smoothstep(rHi - 0.2, rHi, storey);
        diffuseColor.rgb *= mix(vec3(1.0), vec3(0.72), glass * head);
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
