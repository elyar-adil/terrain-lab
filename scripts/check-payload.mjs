// Check the payload's own consistency, independently of the renderer.
//
// The renderer's UV-length check disagreed with the Rust test that asserts the
// same invariant. One of them is wrong, and guessing which has already cost two
// runs, so this measures the bytes directly.

import { readFileSync } from "node:fs";

const path = process.argv[2] ?? "public/city-scenes.json";
const raw = readFileSync(path, "utf8");

// Pull the first city's `meshes` array out of the raw text without a full parse:
// a 55 MB document parses, but slowly, and the point here is to be certain.
const meshesStart = raw.indexOf('"meshes":[');
const texturesStart = raw.indexOf('"textures":[');
if (meshesStart < 0 || texturesStart < 0) throw new Error("could not find meshes/textures");
const slice = raw.slice(meshesStart, texturesStart);

// Split on the top level of that array by brace counting, which is enough because
// the entries contain no braces inside strings.
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

const field = (json, name) => {
  const match = json.match(new RegExp(`"${name}":"([^"]*)"`));
  return match ? match[1] : null;
};
const number = (json, name) => {
  const match = json.match(new RegExp(`"${name}":(\\d+)`));
  return match ? Number(match[1]) : null;
};

const bytesOf = (base64) => (base64.length * 3) / 4;
let short = 0;
let checked = 0;
const report = [];
for (const entry of entries) {
  const vertexCount = number(entry, "vertexCount");
  const triangleCount = number(entry, "triangleCount");
  const material = JSON.parse(`"${field(entry, "material")}"`);
  const positions = field(entry, "positions");
  const normals = field(entry, "normals");
  const uvs = field(entry, "uvs");
  const indices = field(entry, "indices");
  const colors = field(entry, "colors");

  const problems = [];
  if (positions && bytesOf(positions) !== vertexCount * 12) {
    problems.push(`positions ${bytesOf(positions)} != ${vertexCount * 12}`);
  }
  if (normals && bytesOf(normals) !== vertexCount * 12) {
    problems.push(`normals ${bytesOf(normals)} != ${vertexCount * 12}`);
  }
  if (indices && bytesOf(indices) !== triangleCount * 12) {
    problems.push(`indices ${bytesOf(indices)} != ${triangleCount * 12}`);
  }
  if (uvs) {
    const got = bytesOf(uvs);
    const want = vertexCount * 8;
    if (Math.abs(got - want) > 2) problems.push(`uvs ${got} != ${want}`);
  }
  if (colors && Math.abs(bytesOf(colors) - vertexCount * 4) > 2) {
    problems.push(`colors ${bytesOf(colors)} != ${vertexCount * 4}`);
  }
  checked += 1;
  if (problems.length) {
    short += 1;
    if (report.length < 12) report.push(`${material}: ${problems.join("; ")}`);
  }
}

console.log(`meshes: ${checked}, inconsistent: ${short}`);
for (const line of report) console.log(`  ${line}`);
