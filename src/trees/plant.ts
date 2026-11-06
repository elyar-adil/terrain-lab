import * as THREE from "three";

import type { CityHandles, CityScene } from "../city/cityScene";
import { type PlantedTree, UniqueForest } from "./forest";
import { treeGrowerIfReady, treePoolOrNull, treeSeed } from "./index";
import { type TreeGrower } from "./wasm";

/** What this module needs of an instance list: the city renderer's `LodInstanceSet`. */
export interface TreeInstanceSet {
  mesh: THREE.InstancedMesh;
  key: string;
  count: number;
  matrices: Float32Array;
  far: THREE.InstancedMesh | null;
}

export interface Planting {
  trees: PlantedTree[];
  /** Every prototype mesh the planting replaces. */
  replaced: THREE.InstancedMesh[];
  /** The instance lists it was read from. */
  sets: TreeInstanceSet[];
}

/**
 * City species the grower has no row for, mapped to the grown species nearest in
 * habit and season. Without this every one of them fell back to the camphor, so a
 * street of crape myrtle or poplar stood as black-green evergreen domes.
 */
const SPECIES_ALIAS: Record<string, string> = {
  "long-bai": "sha-shu",
  "zi-wei": "ying-hua",
  "bai-hua": "yang-shu",
  "hu-yang": "yang-shu",
  "sha-zao": "liu-shu",
};

/**
 * The day of the year the city's trees are grown at, 0..1. Late October: ginkgo
 * gold, ash and scholar trees half turned, the evergreens still green, which is
 * the Beijing street of the reference photographs. `?season=0.5` in the page
 * address grows them in midsummer instead.
 */
export const CITY_TREE_SEASON = (() => {
  const raw = typeof location !== "undefined" ? new URLSearchParams(location.search).get("season") : null;
  const value = raw === null ? Number.NaN : Number(raw);
  return Number.isFinite(value) ? Math.min(Math.max(value, 0), 1) : 0.81;
})();

function unit(a: number, b: number): number {
  return ((Math.sin(a * 12.9898 + b * 78.233) * 43758.5453) % 1 + 1) % 1;
}

/**
 * Read the tree instance lists of a city (`tree/<species>/<variant>`: one matrix
 * per tree) and turn each into a tree to be grown. The prototype fixes a size
 * and the instance's scale stretches it, so a tree's height is its prototype's
 * height times the instance's vertical scale; where it stands is where the
 * prototype was put. The tree's own seed comes from its place, so planting the
 * same city twice grows the same trees.
 */
export function plantFromInstances(sets: TreeInstanceSet[], grower: TreeGrower, citySeed: number): Planting {
  const trees: PlantedTree[] = [];
  const replaced: THREE.InstancedMesh[] = [];
  const used: TreeInstanceSet[] = [];
  // A prototype is several parts (bark, leaf, core) sharing one list of places;
  // read each list once.
  const read = new Set<string>();
  const heights = new Map<string, number>();
  for (const set of sets) {
    if (!set.key.startsWith("tree/")) continue;
    replaced.push(set.mesh);
    if (set.far) replaced.push(set.far);
    used.push(set);
    const geometry = set.mesh.geometry;
    if (!geometry.boundingBox) geometry.computeBoundingBox();
    const top = geometry.boundingBox?.max.y ?? 0;
    heights.set(set.key, Math.max(heights.get(set.key) ?? 0, top));
  }
  const seedBase = Number(citySeed) % 1_000_003;
  for (const set of used) {
    if (read.has(set.key)) continue;
    read.add(set.key);
    const name = set.key.split("/")[1];
    const found = grower.speciesIndex(name);
    const index = found >= 0 ? found : grower.speciesIndex(SPECIES_ALIAS[name] ?? name);
    const species = index >= 0 ? index : Math.max(0, grower.speciesIndex("xiang-zhang"));
    const base = heights.get(set.key) || 10;
    const m = set.matrices;
    for (let i = 0; i < set.count; i += 1) {
      const o = i * 16;
      const scaleY = Math.hypot(m[o + 4], m[o + 5], m[o + 6]);
      const x = m[o + 12];
      const y = m[o + 13];
      const z = m[o + 14];
      const seed = treeSeed(seedBase, x, z, species);
      trees.push({
        x,
        y,
        z,
        species,
        height: Math.max(2, base * scaleY),
        seed,
        openness: 0.55 + 0.4 * unit(x, z),
        age: 0.55 + 0.45 * unit(z, x + 3.1),
        health: 0.85 + 0.15 * unit(x + 1.7, z - 0.4),
        lift: 0,
      });
    }
  }
  return { trees, replaced, sets: used };
}

/**
 * Replace a built city's prototype trees by a forest of individually grown ones.
 * Returns null (and leaves the city as it was) if the grower has not loaded or
 * the city has no trees. The forest is not added to any scene; the caller mounts
 * `forest.group` beside the city and calls `forest.update` each frame.
 */
export function installUniqueTrees(handles: CityHandles, scene: CityScene): UniqueForest | null {
  const grower = treeGrowerIfReady();
  if (!grower) return null;
  // A viewer that builds the city without LOD lists still has its instanced meshes.
  const sets: TreeInstanceSet[] = handles.lodSets.length > 0
    ? handles.lodSets
    : [...handles.instanced.entries()].map(([key, mesh]) => ({
      mesh,
      key,
      count: mesh.count,
      matrices: (mesh.instanceMatrix.array as Float32Array).slice(0, mesh.count * 16),
      far: null,
    }));
  const planting = plantFromInstances(sets, grower, scene.seed);
  if (planting.trees.length === 0) return null;
  const forest = new UniqueForest(grower, planting.trees, { season: CITY_TREE_SEASON }, treePoolOrNull());
  for (const mesh of planting.replaced) mesh.removeFromParent();
  const gone = new Set(planting.sets);
  handles.lodSets = handles.lodSets.filter((set) => !gone.has(set));
  // Every part of every prototype tree goes. The parts of one prototype (bark,
  // leaf, crown mass) share one key, so the instance map holds only the last of
  // them: the bark was left in the scene and stood as a second, black tree inside
  // each grown one, its bare limbs sticking out of the crown. So sweep the scene.
  const prototypes: THREE.Object3D[] = [];
  handles.group.traverse((object) => {
    if ((object as THREE.InstancedMesh).isInstancedMesh && object.name.startsWith("tree/")) prototypes.push(object);
  });
  for (const object of prototypes) object.removeFromParent();
  for (const key of [...handles.instanced.keys()]) if (key.startsWith("tree/")) handles.instanced.delete(key);
  return forest;
}
