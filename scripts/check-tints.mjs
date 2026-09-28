// Report the per-instance tint in each instance list, and whether the matching
// prototype geometry carries per-vertex colours.
//
// Trunks rendering pure black is the classic symptom of a material with
// `vertexColors: true` whose geometry has no `color` attribute: WebGL supplies
// the default generic attribute, which is (0,0,0,1), so every vertex is black and
// no error is raised. Whether that is what is happening here is a question about
// the payload, so this answers it from the payload.

import { readFileSync } from "node:fs";

const path = process.argv[2] ?? "public/city-scenes.json";
const raw = readFileSync(path, "utf8");

const toFloats = (base64) => {
  const buffer = Buffer.from(base64, "base64");
  const exact = new ArrayBuffer(buffer.length);
  new Uint8Array(exact).set(buffer);
  return new Float32Array(exact);
};

// --- mesh groups: does each have a colour buffer? ---
const meshesStart = raw.indexOf('"meshes":[');
const texturesStart = raw.indexOf('"textures":[');
const meshSlice = raw.slice(meshesStart, texturesStart);
const groups = [];
{
  let depth = 0;
  let start = -1;
  let inString = false;
  let escape = false;
  for (let i = 0; i < meshSlice.length; i += 1) {
    const ch = meshSlice[i];
    if (inString) {
      if (escape) escape = false;
      else if (ch === "\\") escape = true;
      else if (ch === '"') inString = false;
      continue;
    }
    if (ch === '"') { inString = true; continue; }
    if (ch === "{") { if (depth === 0) start = i; depth += 1; continue; }
    if (ch === "}") {
      depth -= 1;
      if (depth === 0 && start >= 0) { groups.push(meshSlice.slice(start, i + 1)); start = -1; }
    }
  }
}

const hasColors = new Map();
for (const group of groups) {
  const material = group.match(/"material":"([^"]*)"/)?.[1];
  const positions = group.match(/"positions":"([^"]*)"/)?.[1];
  if (!material || !positions) continue;
  const vertexCount = Number(group.match(/"vertexCount":(\d+)/)?.[1] ?? 0);
  const colors = group.match(/"colors":"([^"]*)"/)?.[1];
  hasColors.set(material, { has: Boolean(colors), vertexCount });
}

console.log("mesh groups with per-vertex colours:");
const withColour = [...hasColors.entries()].filter(([, info]) => info.has);
console.log(`  ${withColour.length} of ${hasColors.size}`);
for (const [material, info] of withColour.slice(0, 12)) {
  console.log(`    ${material.padEnd(28)} ${info.vertexCount} verts`);
}
console.log("mesh groups WITHOUT per-vertex colours (a `vertexColors` material on these renders black):");
const without = [...hasColors.entries()].filter(([, info]) => !info.has);
console.log(`  ${without.length} of ${hasColors.size}`);
for (const [material] of without.slice(0, 40)) console.log(`    ${material}`);

// --- instance tints ---
const instancesStart = raw.indexOf('"instances":[');
const signalsStart = raw.indexOf('"signals":[');
const listSlice = raw.slice(instancesStart, signalsStart);
const lists = [];
{
  let depth = 0;
  let start = -1;
  let inString = false;
  let escape = false;
  for (let i = 0; i < listSlice.length; i += 1) {
    const ch = listSlice[i];
    if (inString) {
      if (escape) escape = false;
      else if (ch === "\\") escape = true;
      else if (ch === '"') inString = false;
      continue;
    }
    if (ch === '"') { inString = true; continue; }
    if (ch === "{") { if (depth === 0) start = i; depth += 1; continue; }
    if (ch === "}") {
      depth -= 1;
      if (depth === 0 && start >= 0) { lists.push(listSlice.slice(start, i + 1)); start = -1; }
    }
  }
}

console.log("\nper-vertex colour ranges for tinted prototypes:");
for (const group of groups) {
  const material = group.match(/"material":"([^"]*)"/)?.[1];
  const colors = group.match(/"colors":"([^"]*)"/)?.[1];
  if (!material || !colors) continue;
  if (!/#(bark|leaf)$/.test(material)) continue;
  // Vertex colours are `u8` RGBA, not floats. Viewing them as `Float32Array`
  // reinterprets four bytes as one enormous number, which reported every bark
  // tint as -2.2e38 and every leaf tint as NaN — and made a perfectly ordinary
  // palette look catastrophically broken.
  const bytes = Buffer.from(colors, "base64");
  const min = [255, 255, 255];
  const max = [0, 0, 0];
  const sum = [0, 0, 0];
  const count = Math.floor(bytes.length / 4);
  for (let index = 0; index < count; index += 1) {
    for (let channel = 0; channel < 3; channel += 1) {
      const value = bytes[index * 4 + channel];
      min[channel] = Math.min(min[channel], value);
      max[channel] = Math.max(max[channel], value);
      sum[channel] += value;
    }
  }
  const mean = sum.map((value) => value / Math.max(1, count));
  // The renderer multiplies albedo by this, so a mean below about 5/255 is black
  // on screen no matter what the material colour is.
  const flag = Math.max(...mean) < 5 ? "  <-- BLACK ON SCREEN" : "";
  if (material.endsWith("#bark") || material === "tree/tao-shu/0#leaf") {
    console.log(
      `  ${material.padEnd(28)} ${String(count).padStart(5)} verts  ` +
        `bytes ${min.join(",")}..${max.join(",")}  ` +
        `mean ${mean.map((v) => v.toFixed(1)).join(",")}  ` +
        `= linear ${mean.map((v) => (v / 255).toFixed(3)).join(",")}${flag}`,
    );
  }
}

console.log("\ninstance tints (ten floats: x y z yaw sx sy sz r g b):");
for (const list of lists) {
  const key = list.match(/"key":"([^"]*)"/)?.[1];
  const data = list.match(/"data":"([^"]*)"/)?.[1];
  const count = Number(list.match(/"count":(\d+)/)?.[1] ?? 0);
  if (!key || !data || !count) continue;
  if (!/tree|car|shrub|tuft/.test(key)) continue;
  const floats = toFloats(data);
  let min = Infinity;
  let max = -Infinity;
  let sum = 0;
  for (let index = 0; index < count; index += 1) {
    for (let channel = 0; channel < 3; channel += 1) {
      const value = floats[index * 10 + 7 + channel];
      min = Math.min(min, value);
      max = Math.max(max, value);
      sum += value;
    }
  }
  const mean = sum / (count * 3);
  const flag = max < 0.02 ? "  <-- BLACK" : "";
  console.log(
    `  ${key.padEnd(30)} ${String(count).padStart(5)}  tint ${min.toFixed(3)}..${max.toFixed(3)} ` +
      `mean ${mean.toFixed(3)}${flag}`,
  );
}
