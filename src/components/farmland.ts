/**
 * The layout of cultivated land, shared by the terrain shader (which paints
 * it at every range) and the near-field crop meshes (which plant it), so a
 * cotton strip seen from a kilometre is the cotton the camera lands in.
 *
 * Xinjiang's oasis farmland, as the 兵团 laid it out and as the satellite
 * images of Shihezi, Kuitun and Changji show it: one grid of rectangular
 * blocks about 1 km by 0.55 km, every long side lined by a poplar shelterbelt
 * (农田防护林, 10 - 15 m) with a dirt farm road beside it, each block cut
 * along its length into three to five strips that each carry one crop:
 * cotton most of all, then maize, wheat (ripe or already cut to stubble),
 * alfalfa, and a strip or two ploughed bare. The whole grid is turned a
 * little off north, as the canals that set it run with the slope.
 */

/** Rotation of the field grid off the world axes, radians. */
export const FARM_ANGLE = 0.21;
/** A block's length and width, metres. */
export const FARM_BLOCK_M: [number, number] = [1040, 560];
/** Shelterbelt and the farm road beside it, metres across, on a block's long sides. */
export const FARM_BELT_M = 12;
export const FARM_ROAD_M = 6;
/** The ditch-and-bund between two strips, metres across. */
export const FARM_BUND_M = 2.5;

/** What a strip grows. */
export const enum FarmCrop {
  Cotton = 0,
  Maize = 1,
  Wheat = 2,
  Stubble = 3,
  Soil = 4,
  Alfalfa = 5,
}

function fract(x: number) {
  return x - Math.floor(x);
}

/** The same hash as `fHash` in the shader. */
function farmHash(x: number, y: number): number {
  let a = fract(x * 0.1031);
  let b = fract(y * 0.1031);
  let c = fract(x * 0.1031);
  const d = a * (b + 33.33) + b * (c + 33.33) + c * (a + 33.33);
  a += d;
  b += d;
  c += d;
  return fract((a + b) * c);
}

function cropOf(h: number): FarmCrop {
  if (h < 0.34) return FarmCrop.Cotton;
  if (h < 0.54) return FarmCrop.Maize;
  if (h < 0.68) return FarmCrop.Wheat;
  if (h < 0.78) return FarmCrop.Stubble;
  if (h < 0.86) return FarmCrop.Soil;
  return FarmCrop.Alfalfa;
}

export interface FarmPlot {
  crop: FarmCrop;
  /** True on a shelterbelt, a farm road or a bund, where nothing is sown. */
  margin: boolean;
  /** The direction the rows run, as a rotation about +Y for a box laid along x. */
  rowAngle: number;
}

/** The plot under a point in world metres (scene x/z over metres-to-scene). */
export function farmPlot(xm: number, zm: number): FarmPlot {
  const c = Math.cos(FARM_ANGLE), s = Math.sin(FARM_ANGLE);
  const qx = c * xm + s * zm, qy = -s * xm + c * zm;
  const bx = Math.floor(qx / FARM_BLOCK_M[0]), by = Math.floor(qy / FARM_BLOCK_M[1]);
  const fx = qx - bx * FARM_BLOCK_M[0], fy = qy - by * FARM_BLOCK_M[1];
  const strips = 3 + Math.floor(farmHash(bx, by) * 3);
  const width = FARM_BLOCK_M[1] / strips;
  const strip = Math.floor(fy / width);
  const halves = farmHash(bx + 17, by - 5) > 0.5;
  const split = halves && fx > FARM_BLOCK_M[0] * 0.5 ? 1 : 0;
  const crop = cropOf(farmHash(bx * 8 + strip, by * 8 + split * 4 + 1));
  const edge = Math.min(fy, FARM_BLOCK_M[1] - fy);
  const bund = Math.min(Math.abs(fy - strip * width), Math.abs(fy - (strip + 1) * width));
  const end = Math.min(fx, FARM_BLOCK_M[0] - fx, halves ? Math.abs(fx - FARM_BLOCK_M[0] * 0.5) : Infinity);
  const margin = edge < FARM_BELT_M + FARM_ROAD_M || bund < FARM_BUND_M * 0.5 + 0.6 || end < FARM_ROAD_M * 0.5 + 0.6;
  return { crop, margin, rowAngle: -FARM_ANGLE };
}

/** GLSL: `farmColour(p, foot)` gives the field colour at metres `p` for a pixel `foot` metres wide. */
export const FARM_GLSL = `
const float FARM_ANGLE = ${FARM_ANGLE.toFixed(4)};
const vec2 FARM_BLOCK = vec2(${FARM_BLOCK_M[0].toFixed(1)}, ${FARM_BLOCK_M[1].toFixed(1)});
const float FARM_BELT = ${FARM_BELT_M.toFixed(1)};
const float FARM_ROAD = ${FARM_ROAD_M.toFixed(1)};
const float FARM_BUND = ${FARM_BUND_M.toFixed(1)};
float fHash(vec2 p) {
  vec3 p3 = fract(vec3(p.xyx) * 0.1031);
  p3 += dot(p3, p3.yzx + 33.33);
  return fract((p3.x + p3.y) * p3.z);
}
// A line \`half\` wide at distance \`d\`, antialiased by the pixel \`fw\`, and
// averaged to its share of a \`period\` once the pixel outgrows it.
float farmLine(float d, float halfW, float fw, float period) {
  float m = clamp((halfW - d) / max(fw, 1e-3) + 0.5, 0.0, 1.0);
  return mix(m, min(1.0, 2.0 * halfW / period), smoothstep(halfW, halfW * 5.0, fw));
}
vec3 farmCrop(float h, vec2 q, float foot) {
  // Rows along the block, and a plant's worth of grain within them.
  float rowsNear = 1.0 - smoothstep(0.12, 0.5, foot);
  float rows = 0.5 + 0.5 * sin(q.y * 8.27);
  float tone = 0.86 + 0.28 * dFbm(q / 55.0) + 0.10 * (dNoise(q / 6.0) - 0.5) * (1.0 - smoothstep(2.0, 8.0, foot));
  vec3 soil = vec3(0.16, 0.115, 0.075);
  vec3 c;
  if (h < 0.34) {
    c = mix(soil * 1.1, vec3(0.075, 0.115, 0.042), 0.55 + 0.45 * mix(0.8, rows, rowsNear));
  } else if (h < 0.54) {
    c = mix(soil, vec3(0.048, 0.082, 0.028), 0.75 + 0.25 * mix(0.85, rows, rowsNear));
  } else if (h < 0.68) {
    c = vec3(0.36, 0.28, 0.115) * (0.92 + 0.12 * dNoise(q / 1.5));
  } else if (h < 0.78) {
    float bales = (1.0 - smoothstep(0.0, 0.5, abs(fract(q.y / 9.0) - 0.5) * 9.0)) * rowsNear;
    c = mix(vec3(0.33, 0.28, 0.17), vec3(0.42, 0.36, 0.21), 0.5 + 0.5 * sin(q.y * 2.1)) * (1.0 + 0.2 * bales);
  } else if (h < 0.86) {
    float furrow = 0.5 + 0.5 * sin(q.y * 6.28 / 0.9);
    c = soil * (0.82 + 0.3 * mix(0.5, furrow, rowsNear));
  } else {
    c = vec3(0.085, 0.13, 0.05) * (0.9 + 0.2 * dNoise(q / 3.0));
  }
  return c * tone;
}
vec3 farmColour(vec2 p, float foot) {
  float c = cos(FARM_ANGLE), s = sin(FARM_ANGLE);
  vec2 q = vec2(c * p.x + s * p.y, -s * p.x + c * p.y);
  vec2 b = floor(q / FARM_BLOCK);
  vec2 f = q - b * FARM_BLOCK;
  float strips = 3.0 + floor(fHash(b) * 3.0);
  float width = FARM_BLOCK.y / strips;
  float strip = floor(f.y / width);
  bool halves = fHash(b + vec2(17.0, -5.0)) > 0.5;
  float split = halves && f.x > FARM_BLOCK.x * 0.5 ? 1.0 : 0.0;
  vec3 col = farmCrop(fHash(vec2(b.x * 8.0 + strip, b.y * 8.0 + split * 4.0 + 1.0)), q, foot);
  // From far off a strip is narrower than a pixel: its crops average.
  vec3 mean = vec3(0.13, 0.13, 0.07);
  col = mix(col, mean, smoothstep(width * 0.25, width * 1.2, foot));
  // Bunds between strips, the road across a halved block, block ends.
  float bund = min(abs(f.y - strip * width), abs(f.y - (strip + 1.0) * width));
  col = mix(col, vec3(0.19, 0.16, 0.115), farmLine(bund, FARM_BUND * 0.5, foot, width));
  float end = min(f.x, FARM_BLOCK.x - f.x);
  if (halves) end = min(end, abs(f.x - FARM_BLOCK.x * 0.5));
  col = mix(col, vec3(0.23, 0.205, 0.165), farmLine(end, FARM_ROAD * 0.5, foot, FARM_BLOCK.x * (halves ? 0.5 : 1.0)));
  // A block's long sides: the farm road, then the shelterbelt on the line.
  float edge = min(f.y, FARM_BLOCK.y - f.y);
  col = mix(col, vec3(0.23, 0.205, 0.165), farmLine(abs(edge - FARM_BELT * 0.5 - FARM_ROAD * 0.5), FARM_ROAD * 0.5, foot, FARM_BLOCK.y));
  float crowns = 0.55 + 0.9 * dNoise(q / vec2(4.5, 3.2)) * (1.0 - smoothstep(1.0, 4.0, foot)) + 0.2 * (dNoise(q / 40.0) - 0.5);
  vec3 belt = vec3(0.032, 0.058, 0.026) * mix(1.0, crowns, 1.0 - smoothstep(2.0, 6.0, foot));
  col = mix(col, belt, farmLine(edge, FARM_BELT * 0.5, foot, FARM_BLOCK.y));
  // The belt's shadow on the strip on its away side.
  float shade = farmLine(abs(f.y - FARM_BELT * 0.5 - 4.0), 4.0, foot, FARM_BLOCK.y) * step(FARM_BELT * 0.5, f.y);
  return col * (1.0 - 0.35 * shade);
}
`;
