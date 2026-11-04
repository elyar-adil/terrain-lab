/**
 * Tree crowns for the far form (the mass layer): species-correct, lumpy and low
 * poly, from the same species table as the detail's trees.
 *
 * The detail scene grows every tree from species.rs; the far form only has
 * the plan, so a crown here is built from the same record (form, mean height and
 * crown width, clear stem, foliage and bark reflectance) with the species picked
 * the way the detail picks it: kerb trees from the town's planting-zone mix
 * (ModernCity.streetMix, from the terrain climate), plan trees from their own
 * species. A crown is two or three noise-displaced icosahedron lumps (a round or
 * ovoid crown), stacked irregular tiers (a conifer), a flat disc (an umbrella),
 * a column of stretched lumps (a poplar), or a bare stem with drooping fronds (a
 * palm), on a trunk that shows through the clear stem. Colour is the species'
 * leaf reflectance, lifted toward what a lit canopy of leaf cards reads as, and
 * varied tree to tree in value and hue; each lump darkens toward its base, which
 * is the ambient occlusion a clump has.
 */

export interface CrownSink {
  tri(a: number[], b: number[], c: number[], normal: number[], hex: number, shade?: number): void;
  /** A triangle with a normal per corner; clumps use it to shade as rounded. */
  triSmooth?(a: number[], b: number[], c: number[], na: number[], nb: number[], nc: number[], hex: number, shade?: number): void;
}

interface CrownSpecies {
  form: string;
  leaf: number;
  bark: number;
  height: number;
  crown: number;
  clear: number;
}

const SPECIES: Record<string, CrownSpecies> = {
  "xiang-zhang": { form: "Rounded", leaf: 0x405f40, bark: 0x6f6d69, height: 14.0, crown: 6.8, clear: 0.30 },
  "yin-xing": { form: "Fan", leaf: 0x5f7146, bark: 0x7c7974, height: 16.0, crown: 6.2, clear: 0.44 },
  "wu-tong": { form: "Open", leaf: 0x65715f, bark: 0x8a8883, height: 16.0, crown: 7.5, clear: 0.45 },
  "huai-shu": { form: "Rounded", leaf: 0x5b6e4d, bark: 0x77746f, height: 13.0, crown: 6.8, clear: 0.34 },
  "yu-shu": { form: "Rounded", leaf: 0x5b6f54, bark: 0x716c66, height: 16.0, crown: 7.2, clear: 0.36 },
  "gui-hua": { form: "Oval", leaf: 0x375440, bark: 0x6e6b68, height: 6.0, crown: 2.8, clear: 0.26 },
  "xue-song": { form: "Layered", leaf: 0x5b6a64, bark: 0x6a655f, height: 16.0, crown: 5.5, clear: 0.18 },
  "liu-shu": { form: "Weeping", leaf: 0x718058, bark: 0x69655e, height: 11.0, crown: 6.2, clear: 0.28 },
  "song-shu": { form: "Irregular", leaf: 0x4d6346, bark: 0x6c645b, height: 12.5, crown: 5.2, clear: 0.35 },
  "shui-shan": { form: "Conical", leaf: 0x696e63, bark: 0x746d69, height: 24.0, crown: 3.0, clear: 0.62 },
  "ci-huai": { form: "Rounded", leaf: 0x607361, bark: 0x74716c, height: 13.0, crown: 5.5, clear: 0.40 },
  "rong-shu": { form: "Banyan", leaf: 0x46654a, bark: 0x74706b, height: 14.0, crown: 10.0, clear: 0.24 },
  "tao-shu": { form: "Vase", leaf: 0x526c58, bark: 0x79736c, height: 5.0, crown: 3.5, clear: 0.30 },
  "sha-shu": { form: "Conical", leaf: 0x425c59, bark: 0x65605c, height: 18.0, crown: 2.9, clear: 0.40 },
  "he-huan": { form: "Umbrella", leaf: 0x6a7258, bark: 0x716d67, height: 11.0, crown: 6.2, clear: 0.42 },
  "yang-shu": { form: "Fastigiate", leaf: 0x64735b, bark: 0x77746f, height: 21.0, crown: 3.0, clear: 0.18 },
  "bai-la": { form: "Rounded", leaf: 0x597950, bark: 0x736f6b, height: 15.0, crown: 6.0, clear: 0.36 },
  "long-bai": { form: "Fastigiate", leaf: 0x456c60, bark: 0x6c645e, height: 10.0, crown: 1.8, clear: 0.14 },
  "zi-wei": { form: "Vase", leaf: 0x4d633c, bark: 0x736d68, height: 5.5, crown: 2.8, clear: 0.20 },
  "luan-shu": { form: "Rounded", leaf: 0x6c7940, bark: 0x77716a, height: 11.0, crown: 5.8, clear: 0.32 },
  "yu-lan": { form: "Oval", leaf: 0x516c44, bark: 0x898682, height: 8.5, crown: 4.1, clear: 0.24 },
  "bai-hua": { form: "Open", leaf: 0x587254, bark: 0x93918c, height: 10.5, crown: 3.9, clear: 0.38 },
  "hu-yang": { form: "Open", leaf: 0x677150, bark: 0x7c7369, height: 11.5, crown: 5.5, clear: 0.30 },
  "sha-zao": { form: "Rounded", leaf: 0x6c755f, bark: 0x6c655d, height: 6.0, crown: 3.4, clear: 0.20 },
  "zong-lu": { form: "Palm", leaf: 0x4d6c45, bark: 0x655749, height: 6.0, crown: 3.1, clear: 0.80 },
  "pu-kui": { form: "Palm", leaf: 0x557345, bark: 0x736c63, height: 9.0, crown: 4.8, clear: 0.82 },
  "da-wang-ye": { form: "Palm", leaf: 0x52794b, bark: 0x898682, height: 16.0, crown: 8.0, clear: 0.84 },
  "ye-zi": { form: "Palm", leaf: 0x5d7e45, bark: 0x79736a, height: 12.5, crown: 7.0, clear: 0.86 },
  "mu-ma-huang": { form: "Conical", leaf: 0x4b6156, bark: 0x73695f, height: 17.0, crown: 4.0, clear: 0.40 },
  "zhu": { form: "Bamboo", leaf: 0x527144, bark: 0x6c804d, height: 8.0, crown: 2.3, clear: 0.35 },
  "lu-wei": { form: "Reed", leaf: 0x5d6f4b, bark: 0x898061, height: 2.9, crown: 1.2, clear: 0.10 },
  "hong-shu": { form: "Shrub", leaf: 0x3c5944, bark: 0x695b50, height: 3.8, crown: 2.2, clear: 0.25 },
  "sha-ji": { form: "Shrub", leaf: 0x67715b, bark: 0x655b52, height: 2.9, crown: 1.8, clear: 0.12 },
  "suo-suo": { form: "Shrub", leaf: 0x5d6a57, bark: 0x777369, height: 2.8, crown: 2.1, clear: 0.10 },
  "dian-zhuang": { form: "Cushion", leaf: 0x4b634d, bark: 0x766f65, height: 0.4, crown: 0.5, clear: 0.00 },
};

/** The street-tree mix of the original ChineseModern planting, for a plan with no zone. */
const DEFAULT_MIX: Array<[string, number]> = [
  ["huai-shu", 32],
  ["wu-tong", 28],
  ["xiang-zhang", 14],
  ["yin-xing", 9],
  ["bai-la", 9],
  ["yang-shu", 8],
];

/** The plan's own species names, as the detail maps them. */
const PLAN_SPECIES: Record<string, string> = {
  ChinesePlane: "wu-tong",
  Ginkgo: "yin-xing",
  Cedar: "xue-song",
  Bamboo: "zhu",
  Willow: "liu-shu",
  chinesePlane: "wu-tong",
  ginkgo: "yin-xing",
  cedar: "xue-song",
  bamboo: "zhu",
  willow: "liu-shu",
};

function hash(a: number, b = 0, c = 0): number {
  const h = Math.sin(a * 12.9898 + b * 78.233 + c * 37.719) * 43758.5453;
  return h - Math.floor(h);
}

export function planSpecies(name: string | undefined): string {
  return (name && PLAN_SPECIES[name]) || "huai-shu";
}

/** A kerb tree's species: a weighted pick by position, from the town's zone mix. */
export function streetSpecies(mix: Array<[string, number]> | undefined, x: number, z: number): string {
  const list = mix && mix.length ? mix.filter(([k]) => SPECIES[k]) : DEFAULT_MIX;
  const entries = list.length ? list : DEFAULT_MIX;
  const total = entries.reduce((s, [, w]) => s + w, 0);
  let edge = hash(x, z, 5.1) * total;
  for (const [key, weight] of entries) {
    if (edge < weight) return key;
    edge -= weight;
  }
  return entries[entries.length - 1][0];
}

/** A species' mean size, metres, for a tree the plan gives no size. */
export function speciesSize(key: string): { height: number; crown: number } {
  const s = SPECIES[key] ?? SPECIES["huai-shu"];
  return { height: s.height, crown: s.crown * 0.5 };
}

function mix(hex: number, value: number, hue: number): number {
  const r = (hex >> 16) & 255, g = (hex >> 8) & 255, b = hex & 255;
  const c = (x: number) => Math.max(0, Math.min(255, Math.round(x)));
  return (c(r * value * (1 + 0.10 * hue)) << 16) | (c(g * value) << 8) | c(b * value * (1 - 0.16 * hue));
}

const PHI = (1 + Math.sqrt(5)) / 2;
const ICO_V: number[][] = [
  [-1, PHI, 0], [1, PHI, 0], [-1, -PHI, 0], [1, -PHI, 0],
  [0, -1, PHI], [0, 1, PHI], [0, -1, -PHI], [0, 1, -PHI],
  [PHI, 0, -1], [PHI, 0, 1], [-PHI, 0, -1], [-PHI, 0, 1],
].map((v) => {
  const l = Math.hypot(v[0], v[1], v[2]);
  return [v[0] / l, v[1] / l, v[2] / l];
});
const ICO_F: number[][] = [
  [0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11],
  [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8],
  [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9],
  [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1],
];

function unit(v: number[]): number[] {
  const l = Math.hypot(v[0], v[1], v[2]) || 1;
  return [v[0] / l, v[1] / l, v[2] / l];
}

/** The icosahedron split once: 42 vertices, 80 faces, a rounder clump. */
const SUB_V: number[][] = ICO_V.map((v) => v.slice());
const SUB_F: number[][] = [];
{
  const mid = new Map<string, number>();
  const half = (i: number, j: number) => {
    const key = i < j ? `${i}_${j}` : `${j}_${i}`;
    let at = mid.get(key);
    if (at === undefined) {
      SUB_V.push(unit([(SUB_V[i][0] + SUB_V[j][0]) / 2, (SUB_V[i][1] + SUB_V[j][1]) / 2, (SUB_V[i][2] + SUB_V[j][2]) / 2]));
      at = SUB_V.length - 1;
      mid.set(key, at);
    }
    return at;
  };
  for (const [i, j, k] of ICO_F) {
    const a = half(i, j), b = half(j, k), c = half(k, i);
    SUB_F.push([i, a, c], [a, j, b], [c, b, k], [a, b, c]);
  }
}

/** A noise-displaced sphere: a clump of foliage. `fine` uses the split mesh. */
function lump(
  out: CrownSink,
  cx: number, cy: number, cz: number,
  rx: number, ry: number, rz: number,
  seed: number, hex: number, shadeBase: number, rough: number,
  fine = true,
): void {
  const V = fine ? SUB_V : ICO_V;
  const F = fine ? SUB_F : ICO_F;
  const pts = V.map((u, i) => {
    // Two scales of bump: the clump's own lobes and the leaf-cluster knobbles.
    const lobe = hash(seed, i, 1.7) * 2 - 1;
    const knob = hash(seed, i, 8.3) * 2 - 1;
    const d = 1 + rough * (0.7 * lobe + (fine ? 0.35 * knob : 0));
    return [cx + u[0] * rx * d, cy + u[1] * ry * d, cz + u[2] * rz * d];
  });
  const big = Math.max(rx, ry, rz);
  // Corner normals: radial on the ellipsoid, so neighbouring facets shade as one
  // rounded surface.
  const vn = V.map((u) => unit([u[0] / rx * big, u[1] / ry * big, u[2] / rz * big]));
  for (let f = 0; f < F.length; f += 1) {
    const [i, j, k] = F[f];
    const centre = [
      (V[i][0] + V[j][0] + V[k][0]) / 3,
      (V[i][1] + V[j][1] + V[k][1]) / 3,
      (V[i][2] + V[j][2] + V[k][2]) / 3,
    ];
    // Soft shading: the radial direction of the ellipsoid, not the facet's own,
    // so a lump reads as a rounded clump of leaves.
    const n = unit([centre[0] / rx * big, centre[1] / ry * big, centre[2] / rz * big]);
    // Underside darkens (the clump's own occlusion and the sky it cannot see).
    const lit = 0.46 + 0.54 * (0.5 + 0.5 * n[1]);
    const jitter = 0.96 + 0.08 * hash(seed, f, 3.3);
    const colour = mix(hex, jitter, (hash(seed, f, 9.1) - 0.5) * 0.25);
    if (out.triSmooth) out.triSmooth(pts[i], pts[j], pts[k], vn[i], vn[j], vn[k], colour, shadeBase * lit);
    else out.tri(pts[i], pts[j], pts[k], n, colour, shadeBase * lit);
  }
}

/** A tier of a conifer: a ragged cone of `sides` facets. */
function tier(
  out: CrownSink,
  x: number, y0: number, z: number, y1: number,
  r: number, seed: number, hex: number, sides: number,
): void {
  const ring: number[][] = [];
  for (let k = 0; k < sides; k += 1) {
    const a = (k / sides) * Math.PI * 2;
    const rr = r * (0.78 + 0.44 * hash(seed, k, 2.2));
    ring.push([x + Math.cos(a) * rr, y0 + (hash(seed, k, 4.4) - 0.5) * (y1 - y0) * 0.18, z + Math.sin(a) * rr]);
  }
  const apex = [x, y1, z];
  const base = [x, y0 - (y1 - y0) * 0.1, z];
  for (let k = 0; k < sides; k += 1) {
    const p0 = ring[k], p1 = ring[(k + 1) % sides];
    const am = ((k + 0.5) / sides) * Math.PI * 2;
    const n = unit([Math.cos(am) * 0.7, 0.55, Math.sin(am) * 0.7]);
    const j = 0.92 + 0.16 * hash(seed, k, 6.6);
    out.tri(apex, p1, p0, n, mix(hex, j * 0.78, 0.6), 1.0);
    out.tri(base, p0, p1, [n[0], -0.7, n[2]], mix(hex, 0.5, 0.6), 0.55);
  }
}

function trunk(out: CrownSink, x: number, ground: number, z: number, top: number, r: number, hex: number): void {
  const sides = 4;
  const bottom: number[][] = [], upper: number[][] = [];
  for (let k = 0; k < sides; k += 1) {
    const a = (k / sides) * Math.PI * 2 + 0.4;
    bottom.push([x + Math.cos(a) * r, ground, z + Math.sin(a) * r]);
    upper.push([x + Math.cos(a) * r * 0.72, top, z + Math.sin(a) * r * 0.72]);
  }
  for (let k = 0; k < sides; k += 1) {
    const k2 = (k + 1) % sides;
    const am = ((k + 0.5) / sides) * Math.PI * 2 + 0.4;
    const n = [Math.cos(am), 0, Math.sin(am)];
    out.tri(bottom[k], bottom[k2], upper[k2], n, hex, 0.9);
    out.tri(bottom[k], upper[k2], upper[k], n, hex, 0.9);
  }
}

/**
 * One tree's crown. `height` and `radius` are scene units (the caller's metres
 * times its scale); `species` a key of the species table; `seed` in 0..1.
 */
export function drawCrown(
  out: CrownSink,
  x: number, ground: number, z: number,
  height: number, radius: number,
  species: string, seed: number,
): void {
  const s = SPECIES[species] ?? SPECIES["huai-shu"];
  const value = 0.84 + 0.30 * hash(seed, 1.3, 7.7);
  const hue = hash(seed, 2.9, 1.1) * 2 - 1;
  // A lit canopy of leaf cards reads lighter than its leaf reflectance.
  const leaf = mix(s.leaf, 1.0 * value, hue);
  const wood = mix(s.bark, 0.9, 0);
  const sd = Math.floor(seed * 1e6);
  const clear = Math.min(0.7, Math.max(0.05, s.clear));
  const base = ground + height * clear;
  const crownH = height * (1 - clear);
  const trunkR = Math.max(radius * 0.09, height * 0.022);
  const bare = (top: number) => trunk(out, x, ground, z, top, trunkR, wood);
  const form = s.form;
  if (form === "Conical" || form === "Layered") {
    const tiers = form === "Layered" ? 4 : 3;
    const narrow = form === "Layered" ? 1.2 : 0.80;
    bare(base + crownH * 0.15);
    for (let t = 0; t < tiers; t += 1) {
      const f0 = t / tiers, f1 = (t + 1.35) / tiers;
      const r = radius * narrow * (1 - f0 * 0.82);
      tier(out, x, base + crownH * f0 * 0.92, z, base + crownH * Math.min(1, f1), r, sd + t, leaf, 7);
    }
  } else if (form === "Fastigiate") {
    bare(base);
    lump(out, x, base + crownH * 0.30, z, radius * 0.62, crownH * 0.32, radius * 0.62, sd, leaf, 1, 0.16);
    lump(out, x, base + crownH * 0.70, z, radius * 0.52, crownH * 0.32, radius * 0.52, sd + 1, leaf, 1, 0.16);
  } else if (form === "Umbrella") {
    bare(base + crownH * 0.3);
    lump(out, x, base + crownH * 0.68, z, radius * 1.12, crownH * 0.26, radius * 1.12, sd, leaf, 1, 0.14);
    lump(out, x + radius * 0.4, base + crownH * 0.55, z - radius * 0.2, radius * 0.6, crownH * 0.2, radius * 0.6, sd + 1, leaf, 1, 0.14);
  } else if (form === "Weeping") {
    bare(base + crownH * 0.2);
    lump(out, x, base + crownH * 0.58, z, radius * 0.95, crownH * 0.34, radius * 0.95, sd, leaf, 1, 0.14);
    lump(out, x, base + crownH * 0.28, z, radius * 0.86, crownH * 0.34, radius * 0.86, sd + 1, mix(leaf, 0.94, 0.2), 0.95, 0.20);
  } else if (form === "Palm") {
    const top = ground + height * 0.84;
    trunk(out, x, ground, z, top, Math.max(height * 0.017, 0.08 * radius), mix(wood, 1.5, 0));
    const fr = radius * 1.05;
    // Two rings of fronds: the old, drooping ones outside and the new, upright
    // ones inside, each a triangle bent over its midrib.
    for (let ring = 0; ring < 2; ring += 1) {
      const n = ring === 0 ? 9 : 6;
      const reach = fr * (ring === 0 ? 1.0 : 0.62);
      const lift = ring === 0 ? -height * 0.075 : height * 0.03;
      for (let k = 0; k < n; k += 1) {
        const a = (k / n) * Math.PI * 2 + hash(sd, k, 1.1) + ring * 0.4;
        const w = reach * (ring === 0 ? 0.26 : 0.20);
        const tip = [x + Math.cos(a) * reach, top + lift, z + Math.sin(a) * reach];
        const mid = [x + Math.cos(a) * reach * 0.5, top + height * 0.035, z + Math.sin(a) * reach * 0.5];
        const l = [mid[0] + Math.cos(a + 1.5708) * w, mid[1], mid[2] + Math.sin(a + 1.5708) * w];
        const r2 = [mid[0] + Math.cos(a - 1.5708) * w, mid[1], mid[2] + Math.sin(a - 1.5708) * w];
        const base = [x, top + height * 0.01, z];
        const c = mix(leaf, 0.85 + 0.2 * hash(sd, k, 2.2), 0);
        out.tri(base, l, mid, [0, 1, 0.2], c, 1.0);
        out.tri(base, mid, r2, [0, 1, 0.2], c, 1.0);
        out.tri(mid, l, tip, [0, 0.8, 0.3], c, 0.95);
        out.tri(mid, tip, r2, [0, 0.8, 0.3], c, 0.95);
      }
    }
  } else if (form === "Bamboo") {
    for (let k = 0; k < 3; k += 1) {
      const a = hash(sd, k, 8.1) * Math.PI * 2;
      lump(out, x + Math.cos(a) * radius * 0.35, ground + height * (0.55 + 0.1 * k), z + Math.sin(a) * radius * 0.35,
        radius * 0.55, height * 0.36, radius * 0.55, sd + k, leaf, 1, 0.2);
    }
  } else if (form === "Reed") {
    for (let k = 0; k < 2; k += 1) {
      tier(out, x + (k - 0.5) * radius * 0.5, ground, z, ground + height * (0.9 - 0.1 * k), radius * 0.7, sd + k, mix(leaf, 1.1, 0.3), 5);
    }
  } else if (form === "Shrub" || form === "Cushion") {
    lump(out, x, ground + height * 0.42, z, radius * 1.0, height * 0.42, radius * 1.0, sd, leaf, 1, 0.2);
    if (form === "Shrub") lump(out, x + radius * 0.5, ground + height * 0.35, z + radius * 0.3, radius * 0.7, height * 0.35, radius * 0.7, sd + 1, leaf, 1, 0.2);
  } else {
    // Rounded, Open, Oval, Fan, Vase, Banyan, Irregular: a main clump and
    // satellites, so the outline is irregular and the crown has depth.
    const open = form === "Open" || form === "Irregular";
    const tall = form === "Oval" ? 1.25 : form === "Fan" ? 1.1 : 1.0;
    const wide = form === "Banyan" ? 1.15 : 1;
    bare(base + crownH * 0.3);
    lump(out, x, base + crownH * 0.52, z, radius * 1.05 * wide, crownH * 0.44 * tall, radius * 1.05 * wide, sd, leaf, 1, open ? 0.22 : 0.17);
    const sat = open ? 3 : 2;
    for (let k = 0; k < sat; k += 1) {
      const a = hash(sd, k, 3.9) * Math.PI * 2;
      const d = radius * (0.45 + 0.25 * hash(sd, k, 4.1));
      const h = base + crownH * (0.40 + 0.35 * hash(sd, k, 5.7));
      const r = radius * (0.48 + 0.20 * hash(sd, k, 6.3));
      lump(out, x + Math.cos(a) * d, h, z + Math.sin(a) * d, r, crownH * 0.26, r, sd + 3 + k, mix(leaf, 0.94 + 0.12 * hash(sd, k, 2.5), 0), 1, 0.20, false);
    }
  }
}
