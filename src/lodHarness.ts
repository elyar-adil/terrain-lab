/**
 * LOD continuity harness.
 *
 * Builds the far form of a few buildings of the render fixture, one at a time,
 * against a flat magenta ground-less background, and photographs each from a
 * ladder of distances (30 m to 4 km, a factor 1.2 apart) under one sun. For
 * every picture it measures what the building looks like to a person: the
 * mean colour of its pixels, the spread of their luminance, how many pixels
 * tall it is. `scripts/lod-continuity.mjs` drives this page and judges the
 * numbers: the mean must move smoothly along the ladder (no pop in colour),
 * and the spread must not collapse while the building still has pixels to
 * show structure in (no grey box).
 *
 *   ?ids=1439,16     buildings by plan id (default: one of each kind)
 *   ?size=384        picture size
 */

import * as THREE from "three";

import { buildCityMass } from "./city/cityMass";
import type { ModernCityPayload } from "./types";

declare global {
  interface Window {
    __LOD_DONE__?: boolean;
    __LOD_STATS__?: unknown;
    __LOD_SHEET__?: string;
  }
}

type Building = ModernCityPayload["buildings"][number];

const params = new URLSearchParams(window.location.search);
const SIZE = Number(params.get("size") ?? "384");
const SHEET_AT = [50, 150, 300, 800, 2000];

function pick(buildings: Building[]): Array<{ label: string; building: Building }> {
  const looks = (b: Building) => b.farLook;
  const find = (label: string, test: (b: Building) => boolean) => {
    const building = buildings.find(test);
    return building ? [{ label, building }] : [];
  };
  return [
    ...find("curtain-wall tower", (b) => b.useType === "commercial" && b.floors >= 18 && (looks(b)?.kind ?? 1) === 1 && (looks(b)?.startM ?? 0) === 0),
    ...find("residential tower", (b) => b.useType === "residential" && b.floors >= 28 && (looks(b)?.startM ?? 0) === 0),
    ...find("mid-rise slab", (b) => b.useType === "residential" && b.floors >= 9 && b.floors <= 14 && (looks(b)?.startM ?? 0) === 0),
    ...find("walk-up", (b) => b.useType === "residential" && b.floors >= 5 && b.floors <= 7 && (looks(b)?.startM ?? 0) === 0),
    ...find("mixed use", (b) => b.useType === "mixedUse" && b.floors >= 6),
    ...find("village house", (b) => b.useType === "village" && b.floors >= 5),
    ...find("villa", (b) => b.useType === "villa" && (b.roof === "gable" || b.roof === "hip")),
  ];
}

function linear(c: number): number {
  const v = c / 255;
  return v <= 0.04045 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4);
}

async function main() {
  const fixture = await (await fetch("/render-fixture.json")).json();
  const city: ModernCityPayload = fixture.result.modernCities[0];
  const ids = params.get("ids");
  const chosen = ids
    ? ids.split(",").map((id) => ({ label: `building ${id}`, building: city.buildings.find((b) => b.id === Number(id))! }))
    : pick(city.buildings);

  const renderer = new THREE.WebGLRenderer({ antialias: false, preserveDrawingBuffer: true });
  renderer.setSize(SIZE, SIZE, false);
  renderer.outputColorSpace = THREE.SRGBColorSpace;
  renderer.toneMapping = THREE.NoToneMapping;
  document.body.appendChild(renderer.domElement);
  const probe = document.createElement("canvas");
  probe.width = SIZE;
  probe.height = SIZE;
  const pctx = probe.getContext("2d", { willReadFrequently: true })!;

  const ladder: number[] = [];
  for (let d = 30; d < 4100; d *= 1.2) ladder.push(d);
  const sheetSteps = SHEET_AT.map((target) => ladder.reduce((best, d, i) => (Math.abs(d - target) < Math.abs(ladder[best] - target) ? i : best), 0));
  const tile = 256;
  const sheet = document.createElement("canvas");
  sheet.width = tile * SHEET_AT.length;
  sheet.height = tile * chosen.length;
  const sctx = sheet.getContext("2d")!;

  const rows: unknown[] = [];
  for (const [rowIndex, { label, building }] of chosen.entries()) {
    const n = building.footprint.length;
    const cx = building.footprint.reduce((s, p) => s + p.x_km, 0) / n;
    const cy = building.footprint.reduce((s, p) => s + p.y_km, 0) / n;
    const payload = {
      ...city,
      buildings: [building],
      parcels: [], blocks: [], hdRoads: [], trees: [], river: null, tributaries: [], compounds: [],
    } as unknown as ModernCityPayload;
    const mass = buildCityMass(payload, {
      toScene: (x, y) => [(x - cx) * 1000, (y - cy) * 1000],
      heightAt: () => 0,
      metresToScene: 1,
    });
    const scene = new THREE.Scene();
    scene.background = new THREE.Color(1, 0, 1);
    scene.add(mass.mesh);
    const sun = new THREE.DirectionalLight(0xfff0dc, 2.6);
    sun.position.set(-0.45, 0.8, -0.4);
    scene.add(sun, new THREE.HemisphereLight(0xbfd2ff, 0x70705f, 1.1));
    const box = new THREE.Box3().setFromObject(mass.mesh);
    const height = box.max.y;
    const camera = new THREE.PerspectiveCamera(35, 1, 1, 20000);
    const azimuth = THREE.MathUtils.degToRad(35);
    const elevation = THREE.MathUtils.degToRad(16);
    const steps: unknown[] = [];
    for (const [i, d] of ladder.entries()) {
      camera.position.set(Math.sin(azimuth) * Math.cos(elevation) * d, height * 0.5 + Math.sin(elevation) * d, Math.cos(azimuth) * Math.cos(elevation) * d);
      camera.lookAt(0, height * 0.5, 0);
      camera.updateProjectionMatrix();
      // Four renders half a pixel apart, pooled: a building a few dozen pixels
      // across otherwise changes its mean with which pixels the edge happens to hit.
      let count = 0, minY = SIZE, maxY = -1;
      let r = 0, g = 0, b = 0;
      const lums: number[] = [];
      let lcSum = 0, lcCount = 0;
      let cropped = false;
      for (const [ox, oy] of [[0, 0], [0.5, 0.25], [0.25, 0.5], [0.75, 0.75]]) {
        camera.setViewOffset(SIZE, SIZE, ox, oy, SIZE, SIZE);
        renderer.render(scene, camera);
        pctx.drawImage(renderer.domElement, 0, 0);
        const data = pctx.getImageData(0, 0, SIZE, SIZE).data;
        const grid = new Float32Array(SIZE * SIZE).fill(-1);
        for (let y = 0; y < SIZE; y += 1) {
          for (let x = 0; x < SIZE; x += 1) {
            const o = (y * SIZE + x) * 4;
            if (data[o] > 250 && data[o + 1] < 5 && data[o + 2] > 250) continue;
            if (y === 0 || y === SIZE - 1 || x === 0 || x === SIZE - 1) cropped = true;
            count += 1;
            minY = Math.min(minY, y);
            maxY = Math.max(maxY, y);
            const lr = linear(data[o]), lg = linear(data[o + 1]), lb = linear(data[o + 2]);
            r += lr; g += lg; b += lb;
            const lum = 0.2126 * lr + 0.7152 * lg + 0.0722 * lb;
            lums.push(lum);
            grid[y * SIZE + x] = lum;
          }
        }
        // Local contrast: how far a pixel sits from the mean of its four
        // neighbours, over pixels whose neighbours are all the building. Light and
        // shade on whole faces cancel; floors, bays and windows do not.
        for (let y = 1; y < SIZE - 1; y += 1) {
          for (let x = 1; x < SIZE - 1; x += 1) {
            const v = grid[y * SIZE + x];
            const a = grid[y * SIZE + x - 1], b2 = grid[y * SIZE + x + 1], c = grid[(y - 1) * SIZE + x], e = grid[(y + 1) * SIZE + x];
            if (v < 0 || a < 0 || b2 < 0 || c < 0 || e < 0) continue;
            lcSum += Math.abs(v - (a + b2 + c + e) * 0.25);
            lcCount += 1;
          }
        }
      }
      camera.clearViewOffset();
      const mean = lums.reduce((s, v) => s + v, 0) / Math.max(count, 1);
      const variance = lums.reduce((s, v) => s + (v - mean) * (v - mean), 0) / Math.max(count, 1);
      steps.push({
        d: Math.round(d), px: Math.round(count / 4), cropped, heightPx: count ? maxY - minY + 1 : 0,
        mean: [r / Math.max(count, 1), g / Math.max(count, 1), b / Math.max(count, 1)],
        lum: mean, std: Math.sqrt(variance), cv: mean > 0 ? Math.sqrt(variance) / mean : 0,
        lc: lcCount > 20 && mean > 0 ? lcSum / lcCount / mean : 0, lcPx: lcCount,
      });
      const col = sheetSteps.indexOf(i);
      if (col >= 0) sctx.drawImage(renderer.domElement, 0, 0, SIZE, SIZE, col * tile, rowIndex * tile, tile, tile);
    }
    // Straight down from above: what the roof is. Real roofs are screed, membrane,
    // tile or blue steel, grey to light; a black slab here is a defect.
    camera.position.set(0, height + 120, 0.001);
    camera.up.set(0, 0, -1);
    camera.lookAt(0, height, 0);
    camera.updateProjectionMatrix();
    renderer.render(scene, camera);
    pctx.drawImage(renderer.domElement, 0, 0);
    const top = pctx.getImageData(0, 0, SIZE, SIZE).data;
    const roofLums: number[] = [];
    for (let o = 0; o < top.length; o += 4) {
      if (top[o] > 250 && top[o + 1] < 5 && top[o + 2] > 250) continue;
      roofLums.push(0.2126 * linear(top[o]) + 0.7152 * linear(top[o + 1]) + 0.0722 * linear(top[o + 2]));
    }
    const sorted = [...roofLums].sort((x, y) => x - y);
    const median = sorted.length ? sorted[Math.floor(sorted.length / 2)] : 0;
    const tn = roofLums.length;
    // Pixels far darker than the roof's own median: a black slab on a grey roof.
    const tdark = roofLums.filter((v) => v < 0.55 * median).length;
    const tsum = roofLums.reduce((x, y) => x + y, 0);
    camera.up.set(0, 1, 0);
    const roof = { px: tn, darkShare: tn ? tdark / tn : 0, mean: tn ? tsum / tn : 0 };
    rows.push({ label, id: building.id, roof, floors: building.floors, heightM: height, useType: building.useType, farLook: !!building.farLook, steps });
    mass.dispose();
  }
  window.__LOD_STATS__ = { ladder: ladder.map(Math.round), sheetAt: SHEET_AT, rows };
  window.__LOD_SHEET__ = sheet.toDataURL("image/png");
  window.__LOD_DONE__ = true;
}

main().catch((error) => {
  console.error(error);
  window.__LOD_STATS__ = { error: String(error) };
  window.__LOD_DONE__ = true;
});
