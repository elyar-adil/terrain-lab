/**
 * The far form of a building: the look its facade grammar averages to.
 *
 * The plan payload carries a `farLook` per building, read off the very design
 * the metre-scale scene is built from (`crates/city-scene/src/buildings/far.rs`):
 * roof height, storey height, wall colour and its base and top bands, the
 * flat's width, the window and balcony in it, the glass colour. The merged mass
 * draws from that, so a building keeps its colour, height and window rhythm
 * as the camera leaves it, instead of being swapped for a different object.
 *
 * Buildings the grammar does not draw (sheds, villas, civic blocks) and plans
 * from before the field existed get a look derived from the plan alone, by the
 * same rules (`procedural::facade_metrics`: bay width and opening share by
 * facade kind), so every vertex of the mass has one path through the shader.
 *
 * The looks are packed one building (or one tier of one) per row of a float
 * texture, eight texels wide, and read in the fragment shader with `texelFetch`.
 */

import * as THREE from "three";

export const KIND_PUNCHED = 0;
export const KIND_CURTAIN = 1;
export const KIND_GRID = 2;
export const KIND_RIBBON = 3;
/** No windows: sheds, roof plant, parapets. */
export const KIND_BLANK = 4;

export interface FarLook {
  roofM: number;
  /** Where the first storey starts above the ground; above 0 the building stands on a podium. */
  startM: number;
  storeyM: number;
  shopM: number;
  wall: [number, number, number];
  baseBand: [number, number, number];
  baseN: number;
  topBand: [number, number, number];
  topN: number;
  unitW: number;
  kind: number;
  /** x0, x1, sill, head */
  window: [number, number, number, number];
  /** x0, x1, head (x1 = 0 for none) */
  balcony: [number, number, number];
  moduleM: number;
  glass: [number, number, number];
  glassKind: number;
  frame: [number, number, number];
  trim: [number, number, number];
  skin: number;
  flags: number;
  weather: number;
  crown: number;
  terracotta: boolean;
  solar: boolean;
  podiumFloors: number;
  podiumStoreyM: number;
  setbackM: number;
  podiumUnitW: number;
  podiumKind: number;
  podiumWindow: [number, number, number, number];
}

/** Texels per look row. */
export const LOOK_TEXELS = 8;

/** The look of the parts of a building that have no facade: parapets, plant, roofs. */
export function blankLook(wall: [number, number, number]): FarLook {
  return {
    roofM: 0, startM: 0, storeyM: 3, shopM: 0, wall, baseBand: wall, baseN: 0, topBand: wall, topN: 0,
    unitW: 3.6, kind: KIND_BLANK, window: [0, 0, 0, 0], balcony: [0, 0, 0], moduleM: 0,
    glass: [0.1, 0.12, 0.14], glassKind: 0, frame: [0.3, 0.3, 0.3], trim: wall, skin: 0, flags: 0,
    weather: 0.4, crown: 0, terracotta: false, solar: false, podiumFloors: 0, podiumStoreyM: 3,
    setbackM: 0, podiumUnitW: 3.6, podiumKind: KIND_BLANK, podiumWindow: [0, 0, 0, 0],
  };
}

/** The shaft of a look as a look of its own (the podium has its own row). */
export function podiumOf(look: FarLook): FarLook {
  return {
    ...look,
    storeyM: look.podiumStoreyM,
    unitW: look.podiumUnitW,
    kind: look.podiumKind,
    window: look.podiumWindow,
    balcony: [0, 0, 0],
    podiumFloors: 0,
  };
}

/** The upper tier of a stepped building: no shop storey of its own. */
export function shaftOf(look: FarLook): FarLook {
  return { ...look, shopM: 0 };
}

/**
 * Pack looks into the float texture the shader reads: eight RGBA texels per
 * row, row = look index.
 *
 *   0 wall rgb, storey          4 sill, head, balcony x0, x1
 *   1 base band rgb, base n + 256 * storeys above the shop   5 glass rgb, module
 *   2 top band rgb, top n       6 frame rgb, skin * 8 + flags
 *   3 unit w, window x0, x1, kind   7 shop height, weather, glass kind, balcony head
 */
export function packLooks(rows_: Array<{ look: FarLook; storeys: number }>): THREE.DataTexture {
  const rows = Math.max(rows_.length, 1);
  const data = new Float32Array(rows * LOOK_TEXELS * 4);
  rows_.forEach(({ look: l, storeys }, row) => {
    const o = row * LOOK_TEXELS * 4;
    const put = (texel: number, a: number, b: number, c: number, d: number) => {
      data.set([a, b, c, d], o + texel * 4);
    };
    put(0, l.wall[0], l.wall[1], l.wall[2], l.storeyM);
    put(1, l.baseBand[0], l.baseBand[1], l.baseBand[2], l.baseN + 256 * storeys);
    put(2, l.topBand[0], l.topBand[1], l.topBand[2], l.topN);
    put(3, l.unitW, l.window[0], l.window[1], l.kind);
    put(4, l.window[2], l.window[3], l.balcony[0], l.balcony[1]);
    put(5, l.glass[0], l.glass[1], l.glass[2], l.moduleM);
    put(6, l.frame[0], l.frame[1], l.frame[2], l.skin * 8 + l.flags);
    put(7, l.shopM, l.weather, l.glassKind, l.balcony[2]);
  });
  const texture = new THREE.DataTexture(data, LOOK_TEXELS, rows, THREE.RGBAFormat, THREE.FloatType);
  texture.colorSpace = THREE.NoColorSpace;
  texture.magFilter = THREE.NearestFilter;
  texture.minFilter = THREE.NearestFilter;
  texture.generateMipmaps = false;
  texture.flipY = false;
  texture.needsUpdate = true;
  return texture;
}

/** The mean of a look's glazing share of its wall, for the continuity checks. */
export function glazingShare(look: FarLook): number {
  if (look.kind === KIND_BLANK) return 0;
  if (look.kind === KIND_CURTAIN) return Math.max(0, (look.storeyM - 0.9) / look.storeyM);
  const w = Math.max(look.window[1] - look.window[0], 0);
  const h = Math.max(look.window[3] - look.window[2], 0);
  return Math.min(1, (w * h) / Math.max(look.unitW * look.storeyM, 1e-3));
}
