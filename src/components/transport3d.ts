// Railways and the airport in the terrain view, built from the planner's data
// (docs/transport-infrastructure.md). Everything is in metres converted with the
// same factor as elevation, so a 12 m formation and a 45 m runway are those
// widths from any camera.

import * as THREE from "three";
import type { RenderAirport, RenderRailLine, RenderRailway } from "../types";

export interface TransportContext {
  metresToScene: number;
  worldSizeKm: number;
  terrainHeightAt: (x: number, z: number) => number;
  /** Depth bias like the road surfaces use, so thin layers win over the terrain. */
  layer: (colour: number, offset: number, extra?: THREE.MeshStandardMaterialParameters) => THREE.MeshStandardMaterial;
}

class Batch {
  positions: number[] = [];
  indices: number[] = [];
  quad(a: number[], b: number[], c: number[], d: number[]) {
    const base = this.positions.length / 3;
    this.positions.push(...a, ...b, ...c, ...d);
    this.indices.push(base, base + 1, base + 2, base, base + 2, base + 3);
  }
  geometry() {
    const g = new THREE.BufferGeometry();
    g.setAttribute("position", new THREE.Float32BufferAttribute(this.positions, 3));
    g.setIndex(this.indices);
    g.computeVertexNormals();
    return g;
  }
  get empty() {
    return this.positions.length === 0;
  }
}

/** Both faces of a quad so the winding never hides it. */
function quad2(batch: Batch, a: number[], b: number[], c: number[], d: number[]) {
  batch.quad(a, b, c, d);
  batch.quad(d, c, b, a);
}

/** The whole line as one unbroken polyline, seen at any distance (through towns too). */
export function buildRailLines(lines: RenderRailLine[], ctx: TransportContext) {
  const group = new THREE.Group();
  const geometries: THREE.BufferGeometry[] = [];
  const half = ctx.worldSizeKm * 0.5;
  const material = new THREE.LineBasicMaterial({ color: 0x2b2a28 });
  for (const line of lines) {
    const positions: number[] = [];
    line.pathKm.forEach(([kx, kz], i) => {
      const x = (kx - half) * 1000 * ctx.metresToScene;
      const z = (kz - half) * 1000 * ctx.metresToScene;
      positions.push(x, ctx.terrainHeightAt(x, z) + (Math.max(line.fillM[i], 0) + 6) * ctx.metresToScene, z);
    });
    const geometry = new THREE.BufferGeometry();
    geometry.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
    geometries.push(geometry);
    const mesh = new THREE.Line(geometry, material);
    mesh.frustumCulled = false;
    group.add(mesh);
  }
  return { group, geometries, materials: [material] as THREE.Material[] };
}

export function buildRailways(railways: RenderRailway[], ctx: TransportContext) {
  const group = new THREE.Group();
  const geometries: THREE.BufferGeometry[] = [];
  const materials: THREE.Material[] = [];
  const mts = ctx.metresToScene;
  const half = ctx.worldSizeKm * 0.5;
  const ballast = ctx.layer(0x7d7a72, -3);
  const earth = ctx.layer(0x6b6451, -2, { side: THREE.DoubleSide });
  const concrete = ctx.layer(0xa9a69e, -3, { side: THREE.DoubleSide });
  const steel = ctx.layer(0x4b4d50, -5, { metalness: 0.6, roughness: 0.4 });
  const platformMat = ctx.layer(0xb9b6ae, -4);
  const roofMat = ctx.layer(0xd5d8da, -4, { side: THREE.DoubleSide });
  const wallMat = ctx.layer(0xcfccc4, -3, { side: THREE.DoubleSide });
  materials.push(ballast, earth, concrete, steel, platformMat, roofMat, wallMat);
  for (const line of railways) {
    const n = line.pathKm.length;
    if (n < 2) continue;
    const pts = line.pathKm.map(([kx, kz], i) => {
      const x = (kx - half) * 1000 * mts;
      const z = (kz - half) * 1000 * mts;
      const y0 = ctx.terrainHeightAt(x, z);
      const fill = Math.max(line.designM[i] - line.groundM[i], 0);
      return { x, z, y0, fill, structure: line.structure[i], s: line.sM[i] };
    });
    const tangent = pts.map((p, i) => {
      const a = pts[Math.max(0, i - 1)];
      const b = pts[Math.min(n - 1, i + 1)];
      const l = Math.max(1e-12, Math.hypot(b.x - a.x, b.z - a.z));
      return { tx: (b.x - a.x) / l, tz: (b.z - a.z) / l };
    });
    const at = (i: number, off: number, lift: number) => {
      const p = pts[i];
      const t = tangent[i];
      return [p.x + -t.tz * off * mts, p.y0 + (p.fill + lift) * mts, p.z + t.tx * off * mts];
    };
    const bed = new Batch();
    const slopes = new Batch();
    const walls = new Batch();
    const rails = new Batch();
    const plat = new Batch();
    const roofs = new Batch();
    const bldg = new Batch();
    const w = line.formationWidthM * 0.5;
    const sp = line.trackSpacingM * 0.5;
    for (let i = 0; i + 1 < n; i += 1) {
      const j = i + 1;
      const elevated = pts[i].structure >= 3 && pts[i].structure <= 4;
      const lift = elevated ? 0.5 : 0.3;
      bed.quad(at(i, -w, lift), at(i, w, lift), at(j, w, lift), at(j, -w, lift));
      for (const side of [-1, 1]) {
        if (!elevated && pts[i].fill > 0.4) {
          const run = pts[i].fill * 1.5 + 0.6;
          const runJ = pts[j].fill * 1.5 + 0.6;
          const lo = (k: number, r: number) => [at(k, side * (w + r), 0)[0], pts[k].y0 - 0.2 * mts, at(k, side * (w + r), 0)[2]];
          slopes.quad(at(i, side * w, lift), lo(i, run), lo(j, runJ), at(j, side * w, lift));
        }
        if (elevated) {
          const down = (k: number) => [at(k, side * w, 0)[0], pts[k].y0, at(k, side * w, 0)[2]];
          walls.quad(at(i, side * w, lift), at(j, side * w, lift), down(j), down(i));
        }
        for (const gauge of [-1, 1]) {
          const o = sp * (side) + gauge * 0.75;
          rails.quad(at(i, o - 0.06, lift + 0.2), at(i, o + 0.06, lift + 0.2), at(j, o + 0.06, lift + 0.2), at(j, o - 0.06, lift + 0.2));
        }
      }
    }
    // Piers of a viaduct at the 32.6 m spans.
    const piers = new Batch();
    let nextPier = Math.ceil(pts[0].s / 32.6) * 32.6;
    for (let i = 0; i + 1 < n; i += 1) {
      while (nextPier <= pts[i + 1].s) {
        if (pts[i].structure >= 3 && pts[i].structure <= 4 && pts[i].fill > 3) {
          const t = (nextPier - pts[i].s) / Math.max(1e-6, pts[i + 1].s - pts[i].s);
          const x = pts[i].x + (pts[i + 1].x - pts[i].x) * t;
          const z = pts[i].z + (pts[i + 1].z - pts[i].z) * t;
          const y0 = ctx.terrainHeightAt(x, z);
          const top = y0 + (pts[i].fill - 0.5) * mts;
          const hw = 1.3 * mts;
          const hd = 3.0 * mts;
          const tx = tangent[i].tx;
          const tz = tangent[i].tz;
          const corner = (a: number, b: number, y: number) => [x + tx * a * mts - tz * b * mts, y, z + tz * a * mts + tx * b * mts];
          void hw; void hd;
          for (const [a0, b0, a1, b1] of [[-1.3, -3, 1.3, -3], [1.3, -3, 1.3, 3], [1.3, 3, -1.3, 3], [-1.3, 3, -1.3, -3]]) {
            piers.quad(corner(a0, b0, y0), corner(a1, b1, y0), corner(a1, b1, top), corner(a0, b0, top));
          }
        }
        nextPier += 32.6;
      }
    }
    // Stations: platforms, canopy and a station building on the town's side.
    for (const st of line.stations) {
      const sc = st.plan.sM;
      const len = st.plan.platformLengthM;
      let i0 = pts.findIndex((p) => p.s >= sc - len / 2);
      let i1 = pts.findIndex((p) => p.s >= sc + len / 2);
      if (i0 < 0 || i1 < 0) continue;
      i0 = Math.max(0, i0);
      i1 = Math.max(i0 + 1, i1);
      const big = st.plan.tracks >= 4;
      const spans: Array<[number, number]> = big ? [[-16.25, -4.25], [4.25, 16.25]] : [[-(sp + 9.75), -(sp + 1.75)], [sp + 1.75, sp + 9.75]];
      for (let i = i0; i < i1; i += 1) {
        for (const [a, b] of spans) {
          plat.quad(at(i, a, 1.35), at(i, b, 1.35), at(i + 1, b, 1.35), at(i + 1, a, 1.35));
          roofs.quad(at(i, a - 1, 6.5), at(i, b + 1, 6.5), at(i + 1, b + 1, 6.5), at(i + 1, a - 1, 6.5));
        }
      }
      const mid = Math.floor((i0 + i1) / 2);
      const side = st.plan.buildingSide >= 0 ? 1 : -1;
      const across = (big ? 18 + 20 : sp + 12 + 20) * side;
      const lenB = big ? 150 : 60;
      const depth = big ? 38 : 20;
      const height = big ? 17 : 8.5;
      const c = at(mid, across, 0);
      const t = tangent[mid];
      const corner = (a: number, b: number, y: number) => [c[0] + t.tx * a * mts - t.tz * b * mts, y, c[2] + t.tz * a * mts + t.tx * b * mts];
      const yb = pts[mid].y0;
      const rect: Array<[number, number]> = [[-lenB / 2, -depth / 2], [lenB / 2, -depth / 2], [lenB / 2, depth / 2], [-lenB / 2, depth / 2]];
      for (let k = 0; k < 4; k += 1) {
        const [a0, b0] = rect[k];
        const [a1, b1] = rect[(k + 1) % 4];
        bldg.quad(corner(a0, b0, yb), corner(a1, b1, yb), corner(a1, b1, yb + height * mts), corner(a0, b0, yb + height * mts));
      }
      bldg.quad(corner(-lenB / 2 - 1, -depth / 2 - 1, yb + height * mts), corner(lenB / 2 + 1, -depth / 2 - 1, yb + height * mts), corner(lenB / 2 + 1, depth / 2 + 1, yb + height * mts), corner(-lenB / 2 - 1, depth / 2 + 1, yb + height * mts));
    }
    const put = (batch: Batch, material: THREE.Material, order: number) => {
      if (batch.empty) return;
      const geometry = batch.geometry();
      geometries.push(geometry);
      const mesh = new THREE.Mesh(geometry, material);
      mesh.renderOrder = order;
      mesh.frustumCulled = false;
      group.add(mesh);
    };
    put(slopes, earth, 2);
    put(bed, ballast, 3);
    put(walls, concrete, 3);
    put(piers, concrete, 3);
    put(rails, steel, 5);
    put(plat, platformMat, 4);
    put(roofs, roofMat, 5);
    put(bldg, wallMat, 4);
  }
  return { group, geometries, materials };
}

/** Seven-segment strokes for a runway designator digit, as boxes in a 6 x 12 m cell. */
const SEGMENTS: Record<string, number[]> = {
  "0": [0, 1, 2, 4, 5, 6], "1": [2, 5], "2": [0, 2, 3, 4, 6], "3": [0, 2, 3, 5, 6],
  "4": [1, 2, 3, 5], "5": [0, 1, 3, 5, 6], "6": [0, 1, 3, 4, 5, 6], "7": [0, 2, 5],
  "8": [0, 1, 2, 3, 4, 5, 6], "9": [0, 1, 2, 3, 5, 6],
};
// Segment rectangles (x0, x1, y0, y1) in a 6 wide, 12 high cell, stroke 1.6 m.
const SEG_RECT: Array<[number, number, number, number]> = [
  [0, 6, 10.4, 12], [0, 1.6, 6, 12], [4.4, 6, 6, 12], [0, 6, 5.2, 6.8],
  [0, 1.6, 0, 6], [4.4, 6, 0, 6], [0, 6, 0, 1.6],
];

export function buildAirport(airport: RenderAirport, ctx: TransportContext) {
  const group = new THREE.Group();
  const geometries: THREE.BufferGeometry[] = [];
  const materials: THREE.Material[] = [];
  const mts = ctx.metresToScene;
  const half = ctx.worldSizeKm * 0.5;
  const [ax, az] = airport.axis;
  const [nx, nz] = airport.normal;
  const toScene = (x: number, z: number): [number, number] => {
    const kx = airport.centre.x_km + (x * ax + z * nx) / 1000;
    const kz = airport.centre.y_km + (x * az + z * nz) / 1000;
    return [(kx - half) * 1000 * mts, (kz - half) * 1000 * mts];
  };
  const [cx, cz] = toScene(0, 0);
  const y0 = ctx.terrainHeightAt(cx, cz);
  const batches = new Map<string, { batch: Batch; material: THREE.Material; order: number }>();
  const layer = (key: string, colour: number, offset: number, order: number, extra?: THREE.MeshStandardMaterialParameters) => {
    let entry = batches.get(key);
    if (!entry) {
      const material = ctx.layer(colour, offset, { side: THREE.DoubleSide, ...extra });
      materials.push(material);
      entry = { batch: new Batch(), material, order };
      batches.set(key, entry);
    }
    return entry.batch;
  };
  // A flat rectangle on the airfield (plan metres), lifted `lift` metres.
  const flat = (b: Batch, xa: number, xb: number, za: number, zb: number, lift: number) => {
    const p = (x: number, z: number) => {
      const [sx, sz] = toScene(x, z);
      return [sx, y0 + lift * mts, sz];
    };
    quad2(b, p(xa, za), p(xb, za), p(xb, zb), p(xa, zb));
  };
  const strip = (b: Batch, from: [number, number], to: [number, number], width: number, lift: number) => {
    const dx = to[0] - from[0];
    const dz = to[1] - from[1];
    const l = Math.max(1e-6, Math.hypot(dx, dz));
    const ox = (-dz / l) * width * 0.5;
    const oz = (dx / l) * width * 0.5;
    const p = (x: number, z: number) => {
      const [sx, sz] = toScene(x, z);
      return [sx, y0 + lift * mts, sz];
    };
    quad2(b, p(from[0] - ox, from[1] - oz), p(from[0] + ox, from[1] + oz), p(to[0] + ox, to[1] + oz), p(to[0] - ox, to[1] - oz));
  };
  const box = (b: Batch, xa: number, xb: number, za: number, zb: number, h: number, lift = 0) => {
    const P = (x: number, z: number, y: number) => {
      const [sx, sz] = toScene(x, z);
      return [sx, y0 + (lift + y) * mts, sz];
    };
    const c: Array<[number, number]> = [[xa, za], [xb, za], [xb, zb], [xa, zb]];
    for (let k = 0; k < 4; k += 1) {
      const [a0, b0] = c[k];
      const [a1, b1] = c[(k + 1) % 4];
      quad2(b, P(a0, b0, 0), P(a1, b1, 0), P(a1, b1, h), P(a0, b0, h));
    }
    quad2(b, P(xa, za, h), P(xb, za, h), P(xb, zb, h), P(xa, zb, h));
  };
  const L = airport.runwayLengthM * 0.5;
  const W = airport.runwayWidthM;
  const shoulder = layer("shoulder", 0x4c4d50, -2, 2);
  flat(shoulder, -L, L, -airport.pavedWidthM / 2, airport.pavedWidthM / 2, 0.05);
  const asphalt = layer("asphalt", 0x37383b, -3, 3);
  flat(asphalt, -L, L, -W / 2, W / 2, 0.07);
  const paint = layer("paint", 0xeeeeea, -6, 6, { emissive: 0x222222 });
  // Edge lines and centreline dashes (30 m on, 20 m off).
  flat(paint, -L, L, -W / 2 + 0.9, -W / 2 + 1.8, 0.1);
  flat(paint, -L, L, W / 2 - 1.8, W / 2 - 0.9, 0.1);
  for (let x = -L + 12; x + 30 < L - 12; x += 50) flat(paint, x, x + 30, -0.45, 0.45, 0.1);
  // Both ends: threshold bars, aiming point, touchdown pairs, designator.
  const stripes = airport.code.startsWith("3") ? 8 : 12;
  for (const end of [-1, 1]) {
    const dir = -end; // towards the runway interior
    const at = (d: number) => end * L + dir * d; // plan x at distance d from this end
    for (let k = 0; k < stripes; k += 1) {
      const z = -W / 2 + 1.8 + ((W - 3.6) / stripes) * (k + 0.5) - 0.9;
      const d0 = 6;
      flat(paint, Math.min(at(d0), at(d0 + 30)), Math.max(at(d0), at(d0 + 30)), z, z + 1.8, 0.1);
    }
    const aim = 400;
    for (const zs of [-1, 1]) {
      const z = zs * 9;
      flat(paint, Math.min(at(aim), at(aim + 45)), Math.max(at(aim), at(aim + 45)), z - 3, z + 3, 0.1);
    }
    for (let pair = 0; pair < 3; pair += 1) {
      const d = 150 * (pair + 1) + 400;
      if (d + 22.5 > L) continue;
      for (const zs of [-1, 1]) {
        for (let b = 0; b <= 2 - pair; b += 1) {
          const z = zs * (6 + 3.6 * b);
          flat(paint, Math.min(at(d), at(d + 22.5)), Math.max(at(d), at(d + 22.5)), z - 0.9, z + 0.9, 0.1);
        }
      }
    }
    // Designator: the landing direction's number is painted at the threshold you land on.
    const label = end < 0 ? airport.designatorHi : airport.designatorLo;
    const start = 50;
    [...label].forEach((ch, idx) => {
      for (const seg of SEGMENTS[ch] ?? []) {
        const [sx0, sx1, sy0, sy1] = SEG_RECT[seg];
        // y is "up the page" = away from the pilot = further along the runway.
        const along0 = start + sy0;
        const along1 = start + sy1;
        const across0 = (idx - (label.length - 1) / 2) * 9 - 3 + sx0;
        const across1 = (idx - (label.length - 1) / 2) * 9 - 3 + sx1;
        flat(paint, Math.min(at(along0), at(along1)), Math.max(at(along0), at(along1)), across0, across1, 0.1);
      }
    });
  }
  // Taxiway, links and apron.
  const taxi = layer("taxi", 0x45464a, -3, 3);
  const yellow = layer("yellow", 0xd8b732, -6, 6);
  const tz = airport.taxiwayZ;
  const tw = airport.taxiwayWidthM;
  flat(taxi, -L + 60, L - 60, tz - tw / 2, tz + tw / 2, 0.07);
  flat(yellow, -L + 60, L - 60, tz - 0.1, tz + 0.1, 0.1);
  for (const link of airport.links) {
    strip(taxi, link.from, link.to, tw, 0.07);
    strip(yellow, link.from, link.to, 0.2, 0.1);
  }
  const apron = layer("apron", 0x9b9993, -3, 3);
  flat(apron, airport.apron.x0, airport.apron.x1, airport.apron.z0, airport.apron.z1, 0.07);
  for (const [sx, sz] of airport.stands) strip(yellow, [sx, airport.apron.z0 + 6], [sx, sz - 10], 0.25, 0.1);
  const carPark = layer("carpark", 0x5e5f62, -3, 3);
  flat(carPark, airport.carPark.x0, airport.carPark.x1, airport.carPark.z0, airport.carPark.z1, 0.07);
  // Buildings.
  const white = layer("white", 0xdedcd6, -1, 4);
  const glass = layer("glass", 0x4f6f86, -1, 4, { metalness: 0.4, roughness: 0.2 });
  const grey = layer("grey", 0x8f9499, -1, 4);
  const t = airport.terminal;
  box(white, t.x0, t.x1, t.z0, t.z1, t.heightM, 0);
  box(glass, t.x0 - 0.2, t.x1 + 0.2, t.z0 - 0.2, t.z1 + 0.2, t.heightM * 0.45, t.heightM * 0.25);
  // Jet bridges from the terminal to the stands.
  for (const [sx] of airport.stands) box(grey, sx - 1.5, sx + 1.5, airport.apron.z1 - 38, t.z0, 0.3, 4.5);
  const tw0 = airport.tower;
  box(white, tw0.x0 + 4, tw0.x1 - 4, tw0.z0 + 4, tw0.z1 - 4, tw0.heightM - 7, 0);
  box(glass, tw0.x0 - 3, tw0.x1 + 3, tw0.z0 - 3, tw0.z1 + 3, 6, tw0.heightM - 7);
  box(grey, tw0.x0 - 3.5, tw0.x1 + 3.5, tw0.z0 - 3.5, tw0.z1 + 3.5, 0.6, tw0.heightM - 1);
  for (const h of airport.hangars) box(grey, h.x0, h.x1, h.z0, h.z1, h.heightM, 0);
  // Parked aircraft (A320-class: 37.6 m long, 35.8 m span, 4 m fuselage).
  airport.stands.forEach(([sx, sz], i) => {
    if (i % 2 === 1) return;
    box(white, sx - 2, sx + 2, sz - 18, sz + 18, 4, 1.2);
    box(white, sx - 17.9, sx + 17.9, sz - 4, sz - 1, 0.4, 1.8);
    box(white, sx - 0.2, sx + 0.2, sz + 13, sz + 18, 5, 1.2 + 2.5);
  });
  // Fence and approach lights.
  const fence = layer("fence", 0x30343a, -6, 6);
  const f = airport.fence;
  for (const [xa, xb, za, zb] of [[f.x0, f.x1, f.z0, f.z0], [f.x0, f.x1, f.z1, f.z1], [f.x0, f.x0, f.z0, f.z1], [f.x1, f.x1, f.z0, f.z1]]) {
    strip(fence, [xa, za], [xb, zb], 0.5, 0.12);
  }
  const lights = layer("lights", 0xfff3c0, -6, 6, { emissive: 0xfff3c0, emissiveIntensity: 0.9 });
  for (const end of [-1, 1]) {
    for (let d = 60; d <= 900; d += 30) {
      const x = end * (L + d);
      flat(lights, x - 0.4, x + 0.4, -2.5, 2.5, 0.5);
    }
    const x = end * (L + 300);
    flat(lights, x - 0.4, x + 0.4, -15, 15, 0.5);
    flat(lights, end * (L + 150) - 0.4, end * (L + 150) + 0.4, -11, 11, 0.5);
  }
  for (const { batch, material, order } of batches.values()) {
    const geometry = batch.geometry();
    geometries.push(geometry);
    const mesh = new THREE.Mesh(geometry, material);
    mesh.renderOrder = order;
    mesh.frustumCulled = false;
    group.add(mesh);
  }
  return { group, geometries, materials };
}
