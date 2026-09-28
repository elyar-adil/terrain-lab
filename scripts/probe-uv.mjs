// Probe one material's UV buffer directly.
//
// The renderer's length check and a Rust unit test disagreed about the same
// payload. Rather than reason about base64 padding in the abstract, this reads
// the real string and reports every length involved, so the disagreement is
// visible rather than inferred.

import { readFileSync } from "node:fs";

const path = process.argv[2] ?? "public/city-scenes.json";
const wanted = process.argv[3] ?? "asphalt";
const raw = readFileSync(path, "utf8");

const anchor = raw.indexOf(`"${wanted}"`);
if (anchor < 0) throw new Error(`no ${wanted} in ${path}`);
// One mesh entry is well under a megabyte of text, so a fixed window is plenty
// and avoids having to find the entry's end.
const window = raw.slice(anchor, anchor + 4_000_000);
const uvs = window.match(/"uvs":"([^"]*)"/)?.[1];
const vertexCount = Number(window.match(/"vertexCount":(\d+)/)?.[1]);
const triangleCount = Number(window.match(/"triangleCount":(\d+)/)?.[1]);

console.log(`material       ${wanted}`);
console.log(`vertexCount    ${vertexCount}`);
console.log(`triangleCount  ${triangleCount}`);
if (!uvs) {
  console.log("uvs            (absent)");
} else {
  const decoded = Buffer.from(uvs, "base64");
  console.log(`uvs base64     ${uvs.length} chars`);
  console.log(`uvs decoded    ${decoded.length} bytes  (chars*3/4 = ${(uvs.length * 3) / 4})`);
  console.log(`uvs floats     ${decoded.length / 4}`);
  console.log(`uvs pairs      ${decoded.length / 8}`);
  console.log(`wanted bytes   ${vertexCount * 8}  (${vertexCount} verts * 2 floats * 4)`);
  console.log(`difference     ${decoded.length - vertexCount * 8} bytes`);
  // A base64 string's own length is a cheap cross-check: it is
  // `ceil(bytes / 3) * 4`, so it must be a multiple of four and must not be
  // shorter than the decoded length.
  console.log(`chars % 4      ${uvs.length % 4}`);
  console.log(`ceil(bytes/3)*4 ${Math.ceil(decoded.length / 3) * 4}`);
}
