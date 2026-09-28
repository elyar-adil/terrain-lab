// Measure an audit PNG: its real dimensions, and where the content actually ends.
//
// A screenshot that fills only part of its own frame has three possible causes —
// a wrongly sized canvas, a render that did not cover the viewport, or a capture
// that lost part of the surface — and they look identical in an image viewer.
// This distinguishes them from the pixels alone.

import { readFileSync } from "node:fs";

const path = process.argv[2] ?? "docs/render-validation/street-cheap.png";
const png = readFileSync(path);

// --- dimensions, straight out of the IHDR chunk ---
const width = png.readUInt32BE(16);
const height = png.readUInt32BE(20);
const bitDepth = png[24];
const colourType = png[25];
console.log(`file      ${path}`);
console.log(`size      ${width} x ${height}`);
console.log(`format    bitDepth=${bitDepth} colourType=${colourType} (6 = RGBA)`);

// --- decode ---
// Only 8-bit RGB/RGBA non-interlaced is handled, which is what a canvas produces.
if (bitDepth !== 8 || (colourType !== 2 && colourType !== 6)) {
  console.log("unsupported PNG variant; dimensions above are still valid");
  process.exit(0);
}
const channels = colourType === 6 ? 4 : 3;

let offset = 8;
const idat = [];
while (offset < png.length) {
  const length = png.readUInt32BE(offset);
  const type = png.toString("ascii", offset + 4, offset + 8);
  if (type === "IDAT") idat.push(png.subarray(offset + 8, offset + 8 + length));
  offset += 12 + length;
  if (type === "IEND") break;
}

// Inflate with the platform's zlib rather than pulling in a dependency.
const { inflateSync } = await import("node:zlib");
const raw = inflateSync(Buffer.concat(idat));

const stride = width * channels;
const pixels = Buffer.alloc(height * stride);
let previous = Buffer.alloc(stride);
let position = 0;
for (let y = 0; y < height; y += 1) {
  const filter = raw[position];
  position += 1;
  const line = Buffer.from(raw.subarray(position, position + stride));
  position += stride;
  for (let x = 0; x < stride; x += 1) {
    const a = x >= channels ? line[x - channels] : 0;
    const b = previous[x];
    const c = x >= channels ? previous[x - channels] : 0;
    let value = line[x];
    if (filter === 1) value += a;
    else if (filter === 2) value += b;
    else if (filter === 3) value += (a + b) >> 1;
    else if (filter === 4) {
      const p = a + b - c;
      const pa = Math.abs(p - a);
      const pb = Math.abs(p - b);
      const pc = Math.abs(p - c);
      value += pa <= pb && pa <= pc ? a : pb <= pc ? b : c;
    }
    line[x] = value & 0xff;
  }
  line.copy(pixels, y * stride);
  previous = line;
}

const at = (x, y) => {
  const i = y * stride + x * channels;
  return [pixels[i], pixels[i + 1], pixels[i + 2], channels === 4 ? pixels[i + 3] : 255];
};

/**
 * A column is blank when it is *transparent*, or when every sampled pixel is the
 * same near-white value.
 *
 * The alpha test is the one that matters and the one that was missing. An image
 * viewer composites a fully transparent region against white, so a canvas that
 * was never painted past a certain column looks like a bright margin — and
 * checking only RGB reported such an image as fully covered, because transparent
 * black is not near-white. That is how a half-rendered canvas passed as a
 * complete one.
 */
const isBlankColumn = (x) => {
  for (let y = 0; y < height; y += 4) {
    const [r, g, b, a] = at(x, y);
    if (a < 8) continue;
    if (r < 245 || g < 245 || b < 245) return false;
  }
  return true;
};

let firstContent = -1;
let lastContent = -1;
let transparent = 0;
let sampled = 0;
for (let x = 0; x < width; x += 1) {
  if (!isBlankColumn(x)) {
    if (firstContent < 0) firstContent = x;
    lastContent = x;
  }
}
for (let y = 0; y < height; y += 4) {
  for (let x = 0; x < width; x += 4) {
    sampled += 1;
    if (at(x, y)[3] < 8) transparent += 1;
  }
}
const covered = lastContent >= firstContent ? lastContent - firstContent + 1 : 0;
console.log(
  `content   columns ${firstContent}..${lastContent} of 0..${width - 1}` +
    `  (${((covered / width) * 100).toFixed(1)}% of the width)`,
);
console.log(
  `alpha     ${((transparent / sampled) * 100).toFixed(1)}% of sampled pixels are fully transparent`,
);
if (firstContent > 0 || lastContent < width - 1) {
  console.log(
    `          margin: ${firstContent} px at the left, ${width - 1 - lastContent} px at the right`,
  );
  const mid = height >> 1;
  console.log(`          margin pixel  rgba(${at(width - 1, mid).join(",")})`);
  console.log(
    `          content pixel rgba(${at((Math.max(0, firstContent) + lastContent) >> 1, mid).join(",")})`,
  );
} else {
  console.log("          the image fills its frame edge to edge");
}
