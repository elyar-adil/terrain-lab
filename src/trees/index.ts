import { TreeGrower } from "./wasm";

export { TreeGrower, type TreeData, type TreeSpec } from "./wasm";
export { UniqueForest, type ForestOptions, type PlantedTree } from "./forest";
export { patchTreeMaterial } from "./shaders";

let loading: Promise<TreeGrower | null> | null = null;
let loaded: TreeGrower | null = null;

/**
 * Start loading the grower. Never rejects: a host without the WebAssembly file
 * simply keeps whatever trees it had, so this resolves to null.
 */
export function preloadTreeGrower(url?: string): Promise<TreeGrower | null> {
  loading ??= TreeGrower.load(url)
    .then((grower) => {
      loaded = grower;
      return grower;
    })
    .catch((error: unknown) => {
      console.warn("tree grower unavailable:", error);
      return null;
    });
  return loading;
}

/** The grower if it has finished loading. */
export function treeGrowerIfReady(): TreeGrower | null {
  return loaded;
}

/** A 53-bit seed from a few numbers: the same inputs always give the same tree. */
export function treeSeed(...parts: number[]): number {
  let a = 0x9e3779b9;
  let b = 0x85ebca6b;
  for (const part of parts) {
    const x = Math.round(part * 8) | 0;
    a = Math.imul(a ^ x, 0x85ebca6b);
    a ^= a >>> 13;
    b = Math.imul(b + a + 0x27d4eb2f, 0xc2b2ae35);
    b ^= b >>> 16;
  }
  a = Math.imul(a ^ (a >>> 15), 0x2c1b3c6d);
  b = Math.imul(b ^ (b >>> 12), 0x297a2d39);
  return ((b >>> 0) & 0x1fffff) * 4294967296 + (a >>> 0);
}
