// Report every mesh group's bounding box, largest first.
//
// A viewport that is only partly covered, with the unpainted part following the
// camera, is a *giant piece of geometry* occluding the scene rather than a
// capture problem. Finding it means looking at extents, so this decodes the
// position buffers and prints the extremes.
//
// A group's box is the union of its vertices' extents, which is exactly the thing
// a bad vertex poisons: one NaN or one coordinate that escaped a clamp turns a
// 200 m building into a 40 km wall that swallows the frame.

import { readFileSync } from "node:fs";

const path = process.argv[2] ?? "public/city-scenes.json";
const limit = Number(process.argv[3] ?? 10);
const raw = readFileSync(path, "utf8");

const meshesStart = raw.indexOf('"meshes":[');
const texturesStart = raw.indexOf('"textures":[');
if (meshesStart < 0 || texturesStart < 0) throw new Error("could not find meshes");
const slice = raw.slice(meshesStart, texturesStart);

const entries = [];
let depth = 0;
let start = -1;
let inString = false;
let escape = false;
for (let i = 0; i < slice.length; i += 1) {
  const ch = slice[i];
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
    if (depth === 0 && start >= 0) { entries.push(slice.slice(start, i + 1)); start = -1; }
  }
}

const rows = [];
let nonFinite = 0;
for (const entry of entries) {
  const name = entry.match(/"material":"([^"]*)"/)?.[1];
  const positions = entry.match(/"positions":"([^"]*)"/)?.[1];
  const vertexCount = Number(entry.match(/"vertexCount":(\d+)/)?.[1] ?? 0);
  // The brace scan also picks up the instance-list objects, which have no
  // geometry. Skip anything that is not a mesh group rather than assuming.
  if (!name || !positions || !vertexCount) continue;
  const material = JSON.parse(`"${name}"`);
  // Copy into a right-sized `ArrayBuffer` before viewing it as floats.
  //
  // `Buffer.from(string, "base64")` allocates out of Node's shared 8 KiB pool for
  // anything under the pool size, so `buffer.buffer` is the *whole pool* and not
  // the payload. Viewing that as `Float32Array` reads 8 KiB of unrelated memory,
  // which reported a 72-vertex car body as extending 8e35 metres. Several
  // minutes went into that phantom before the buffer size was checked.
  const raw4 = Buffer.from(positions, "base64");
  const exact = new ArrayBuffer(raw4.length);
  new Uint8Array(exact).set(raw4);
  const floats = new Float32Array(exact);
  if (floats.length < vertexCount * 3) {
    throw new Error(`${material}: ${vertexCount} vertices but only ${floats.length} floats`);
  }
  const box = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
  let bad = 0;
  for (let i = 0; i < vertexCount; i += 1) {
    const x = floats[i * 3];
    const y = floats[i * 3 + 1];
    const z = floats[i * 3 + 2];
    if (!Number.isFinite(x) || !Number.isFinite(y) || !Number.isFinite(z)) {
      bad += 1;
      continue;
    }
    if (x < box[0]) box[0] = x;
    if (y < box[1]) box[1] = y;
    if (z < box[2]) box[2] = z;
    if (x > box[3]) box[3] = x;
    if (y > box[4]) box[4] = y;
    if (z > box[5]) box[5] = z;
  }
  nonFinite += bad;
  const size = [
    box[3] - box[0],
    box[4] - box[1],
    box[5] - box[2],
  ];
  const diagonal = Math.hypot(size[0], size[1], size[2]);
  rows.push({ material, vertexCount, bad, size, box, diagonal });
}

rows.sort((a, b) => b.diagonal - a.diagonal);
console.log(`meshes: ${rows.length}, vertices that are not finite: ${nonFinite}`);
console.log(`\nlargest by bounding-box diagonal (the city's own extent is about 3400 m):`);
for (const row of rows.slice(0, limit)) {
  const [sx, sy, sz] = row.size;
  const flag = row.diagonal > 5000 ? "  <-- ENORMOUS" : row.bad ? "  <-- HAS NaN/Inf" : "";
  console.log(
    `  ${row.material.padEnd(26)} ${String(row.vertexCount).padStart(8)} verts  ` +
      `size ${sx.toFixed(0).padStart(7)} x ${sy.toFixed(0).padStart(6)} x ${sz.toFixed(0).padStart(7)}  ` +
      `y ${row.box[1].toFixed(0)}..${row.box[4].toFixed(0)}${flag}`,
  );
}

const tall = rows.filter((row) => row.box[4] > 400).slice(0, 8);
if (tall.length) {
  console.log(`\ngroups reaching above 400 m (a bridge deck or a lamp is a few metres):`);
  for (const row of tall) {
    console.log(`  ${row.material.padEnd(26)} y up to ${row.box[4].toFixed(0)} m`);
  }
}

/**
 * Instance transforms.
 *
 * A group's own geometry can be perfectly sized and still be drawn a million
 * times too large, because an `InstancedMesh`'s extent is its geometry *scaled by
 * each instance*. A single bad scale is enough to produce a dark wedge across the
 * sky that no amount of geometry auditing will find, because nothing in the
 * position buffers is wrong.
 *
 * Ten floats per instance: x y z, yaw, scale x y z, tint r g b.
 */
const instancesStart = raw.indexOf('"instances":[');
const signalsStart = raw.indexOf('"signals":[');
const instanceSlice = raw.slice(instancesStart, signalsStart);
const lists = [];
depth = 0;
start = -1;
inString = false;
escape = false;
for (let i = 0; i < instanceSlice.length; i += 1) {
  const ch = instanceSlice[i];
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
    if (depth === 0 && start >= 0) { lists.push(instanceSlice.slice(start, i + 1)); start = -1; }
  }
}

console.log(`\ninstance lists: ${lists.length}`);
const badScales = [];
let worstScale = 0;
let worstKey = "";
for (const list of lists) {
  const key = list.match(/"key":"([^"]*)"/)?.[1];
  const data = list.match(/"data":"([^"]*)"/)?.[1];
  const count = Number(list.match(/"count":(\d+)/)?.[1] ?? 0);
  if (!key || !data || !count) continue;
  const raw4 = Buffer.from(data, "base64");
  const exact = new ArrayBuffer(raw4.length);
  new Uint8Array(exact).set(raw4);
  const floats = new Float32Array(exact);
  for (let index = 0; index < count; index += 1) {
    const base = index * 10;
    const scale = Math.max(
      Math.abs(floats[base + 4]),
      Math.abs(floats[base + 5]),
      Math.abs(floats[base + 6]),
    );
    const position = Math.max(
      Math.abs(floats[base]),
      Math.abs(floats[base + 1]),
      Math.abs(floats[base + 2]),
    );
    if (!Number.isFinite(scale) || !Number.isFinite(position) || scale > 1e4 || position > 1e5) {
      badScales.push(
        `${key}[${index}]: scale ${scale.toExponential(2)} at ${position.toExponential(2)} m`,
      );
    }
    if (scale > worstScale) {
      worstScale = scale;
      worstKey = `${key}[${index}]`;
    }
  }
}
console.log(`  largest instance scale: ${worstScale.toExponential(3)} at ${worstKey}`);
if (badScales.length) {
  console.log(`  ${badScales.length} implausible transforms:`);
  for (const line of badScales.slice(0, 12)) console.log(`    ${line}`);
} else {
  console.log("  every transform is within plausible range");
}
