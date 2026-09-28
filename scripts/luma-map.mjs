// Print a coarse luminance map of a PNG, so a region that "looks white" can be
// checked instead of argued about.
//
// `measure-png.mjs` established that the image fills its frame, which means a
// bright region is *rendered* content and not a capture artefact. The next
// question is what it is, and the answer is in the values.

import { readFileSync } from "node:fs";
import { inflateSync } from "node:zlib";

const path = process.argv[2] ?? "docs/render-validation/street-cheap.png";
const columns = Number(process.argv[3] ?? 48);
const rows = Number(process.argv[4] ?? 20);

const png = readFileSync(path);
const width = png.readUInt32BE(16);
const height = png.readUInt32BE(20);
const channels = png[25] === 6 ? 4 : 3;

let offset = 8;
const idat = [];
while (offset < png.length) {
  const length = png.readUInt32BE(offset);
  const type = png.toString("ascii", offset + 4, offset + 8);
  if (type === "IDAT") idat.push(png.subarray(offset + 8, offset + 8 + length));
  offset += 12 + length;
  if (type === "IEND") break;
}
const raw = inflateSync(Buffer.concat(idat));
const stride = width * channels;
const pixels = Buffer.alloc(height * stride);
let position = 0;
let previous = Buffer.alloc(stride);
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

const sample = (x, y) => {
  const i = y * stride + x * channels;
  return [pixels[i], pixels[i + 1], pixels[i + 2]];
};

const RAMP = " .:-=+*#%@";
console.log(`${path}  ${width}x${height}   (darker is closer to black)`);
let header = "     ";
for (let c = 0; c < columns; c += 1) header += (c % 10 === 0 ? String(Math.floor((c / columns) * 10)) : " ");
console.log(header);
for (let r = 0; r < rows; r += 1) {
  const y = Math.floor(((r + 0.5) / rows) * height);
  let line = String(r).padStart(3) + "  ";
  for (let c = 0; c < columns; c += 1) {
    const x = Math.floor(((c + 0.5) / columns) * width);
    const [red, green, blue] = sample(x, y);
    const luma = (0.2126 * red + 0.7152 * green + 0.0722 * blue) / 255;
    // Also flag anything that is both very light and very desaturated, because
    // that is the signature of a blown-out surface rather than of a bright one.
    const peak = Math.max(red, green, blue);
    const chroma = peak - Math.min(red, green, blue);
    const blown = luma > 0.92 && chroma < 12;
    line += blown ? "!" : RAMP[Math.min(9, Math.floor(luma * 10))];
  }
  console.log(line);
}
console.log("\nlegend: space=black  @=white   !=blown out (light AND colourless)");

// Statistics over the right half, where the problem is.
let bright = 0;
let blown = 0;
let total = 0;
const hues = new Map();
for (let y = 0; y < height; y += 2) {
  for (let x = Math.floor(width / 2); x < width; x += 2) {
    const [red, green, blue] = sample(x, y);
    const luma = (0.2126 * red + 0.7152 * green + 0.0722 * blue) / 255;
    total += 1;
    if (luma > 0.9) bright += 1;
    if (luma > 0.92 && Math.max(red, green, blue) - Math.min(red, green, blue) < 12) blown += 1;
    const key = `${red >> 5},${green >> 5},${blue >> 5}`;
    hues.set(key, (hues.get(key) ?? 0) + 1);
  }
}
console.log(
  `\nright half: ${((bright / total) * 100).toFixed(1)}% above 0.90 luma, ` +
    `${((blown / total) * 100).toFixed(1)}% blown out`,
);
const top = [...hues.entries()].sort((a, b) => b[1] - a[1]).slice(0, 6);
console.log(
  "most common colours (r,g,b >> 5, 0-7): " +
    top.map(([key, n]) => `${key} ${((n / total) * 100).toFixed(0)}%`).join("  "),
);
