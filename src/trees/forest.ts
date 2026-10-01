/**
 * A set of individually grown trees, drawn in as few calls as the camera allows.
 *
 * Every tree is its own: the grower is called with the tree's own seed, so no two
 * share a branch or a leaf. What is shared is only the machinery. Trees are
 * grouped into square chunks; a chunk is grown at a level of detail chosen from
 * its distance to the camera and drawn as two instanced meshes, one for all its
 * branches and one for all its leaves. Growing is spread over frames, nearest
 * chunks first, and a chunk keeps showing its old level until the new one is
 * ready, so the camera never sees a hole.
 *
 * Growing a tree is a pure function of its spec, so rebuilding a chunk gives back
 * exactly the same trees, only finer or coarser.
 */

import * as THREE from "three";

import { type TreeData, type TreeGrower } from "./wasm";
import { type TreeMaterials, createTreeMaterials, treeTable } from "./shaders";

export interface PlantedTree {
  /** Position of the foot, in the forest's own frame (metres). */
  x: number;
  y: number;
  z: number;
  /** Index into the grower's species catalogue. */
  species: number;
  height: number;
  seed: number;
  /** 0: grown in a stand; 1: open-grown. */
  openness: number;
  age: number;
  health: number;
  lift: number;
}

export interface ForestOptions {
  /** Side of a chunk, metres. */
  chunkM?: number;
  /** Distances at which a chunk drops from level 0 to 1, 1 to 2, and 2 to 3. */
  lodDistancesM?: [number, number, number];
  /** Beyond this a chunk is not drawn. */
  farM?: number;
  /** Day of the year, 0..1. */
  season?: number;
}

interface Chunk {
  key: string;
  trees: number[];
  min: THREE.Vector3;
  max: THREE.Vector3;
  centre: THREE.Vector2;
  /** The level its meshes were grown at, or -1 when it has none. */
  lod: number;
  wanted: number;
  distance: number;
  wood: THREE.Mesh | null;
  leaf: THREE.Mesh | null;
  job: Job | null;
}

interface Job {
  chunk: Chunk;
  lod: number;
  next: number;
  grown: TreeData[];
}

const SIDES = [10, 8, 6, 5];

function woodBase(sides: number): THREE.InstancedBufferGeometry {
  const columns = sides + 1;
  const position = new Float32Array(columns * 2 * 3);
  const normal = new Float32Array(columns * 2 * 3);
  const kind = new Float32Array(columns * 2).fill(1);
  for (let i = 0; i < columns; i += 1) {
    const angle = (i / sides) * Math.PI * 2;
    const c = Math.cos(angle);
    const s = Math.sin(angle);
    for (let k = 0; k < 2; k += 1) {
      const v = (i * 2 + k) * 3;
      position[v] = c;
      position[v + 1] = k;
      position[v + 2] = s;
      normal[v] = c;
      normal[v + 2] = s;
    }
  }
  const index: number[] = [];
  for (let i = 0; i < sides; i += 1) {
    const a = i * 2;
    const b = a + 1;
    const c = a + 2;
    const d = a + 3;
    index.push(a, c, b, b, c, d);
  }
  const geometry = new THREE.InstancedBufferGeometry();
  geometry.setAttribute("position", new THREE.BufferAttribute(position, 3));
  geometry.setAttribute("normal", new THREE.BufferAttribute(normal, 3));
  geometry.setAttribute("aTreeKind", new THREE.BufferAttribute(kind, 1));
  geometry.setIndex(index);
  return geometry;
}

function leafBase(): THREE.InstancedBufferGeometry {
  const position: number[] = [];
  const normal: number[] = [];
  const kind: number[] = [];
  for (let row = 0; row < 3; row += 1) {
    for (let column = 0; column < 3; column += 1) {
      position.push(column - 1, row * 0.5, 0);
      normal.push(0, 0, 1);
      kind.push(2);
    }
  }
  const index: number[] = [];
  for (let row = 0; row < 2; row += 1) {
    for (let column = 0; column < 2; column += 1) {
      const a = row * 3 + column;
      index.push(a, a + 1, a + 3, a + 1, a + 4, a + 3);
    }
  }
  const geometry = new THREE.InstancedBufferGeometry();
  geometry.setAttribute("position", new THREE.Float32BufferAttribute(position, 3));
  geometry.setAttribute("normal", new THREE.Float32BufferAttribute(normal, 3));
  geometry.setAttribute("aTreeKind", new THREE.Float32BufferAttribute(kind, 1));
  geometry.setIndex(index);
  return geometry;
}

export class UniqueForest {
  readonly group = new THREE.Group();
  private readonly chunks: Chunk[] = [];
  private readonly trees: PlantedTree[];
  /** First row of this forest trees in the shared table. */
  private readonly rowBase: number;
  private readonly materials: TreeMaterials;
  private readonly rowDone: Uint8Array;
  private readonly lodDistances: [number, number, number];
  private readonly farM: number;
  private season: number;
  private casting = true;
  private generation = 0;
  /** Counts for diagnostics: trees, branch segments and leaves currently drawn. */
  stats = { chunks: 0, built: 0, trees: 0, segments: 0, leaves: 0, growMs: 0, jobs: 0 };

  constructor(
    private readonly grower: TreeGrower,
    trees: PlantedTree[],
    options: ForestOptions = {},
  ) {
    this.trees = trees;
    this.lodDistances = options.lodDistancesM ?? [45, 110, 260];
    this.farM = options.farM ?? 1500;
    this.season = options.season ?? 0.5;
    this.rowBase = treeTable.allocate(trees.length);
    this.materials = createTreeMaterials();
    this.rowDone = new Uint8Array(trees.length);
    this.group.name = "unique-forest";

    const cell = options.chunkM ?? 64;
    const byKey = new Map<string, Chunk>();
    trees.forEach((tree, index) => {
      const cx = Math.floor(tree.x / cell);
      const cz = Math.floor(tree.z / cell);
      const key = `${cx},${cz}`;
      let chunk = byKey.get(key);
      if (!chunk) {
        chunk = {
          key,
          trees: [],
          min: new THREE.Vector3(Infinity, Infinity, Infinity),
          max: new THREE.Vector3(-Infinity, -Infinity, -Infinity),
          centre: new THREE.Vector2(),
          lod: -1,
          wanted: -1,
          distance: Infinity,
          wood: null,
          leaf: null,
          job: null,
        };
        byKey.set(key, chunk);
        this.chunks.push(chunk);
      }
      chunk.trees.push(index);
      // Crowns reach beyond the foot of the trunk.
      const reach = tree.height * 0.7;
      chunk.min.min(new THREE.Vector3(tree.x - reach, tree.y, tree.z - reach));
      chunk.max.max(new THREE.Vector3(tree.x + reach, tree.y + tree.height * 1.05, tree.z + reach));
    });
    for (const chunk of this.chunks) {
      chunk.centre.set((chunk.min.x + chunk.max.x) / 2, (chunk.min.z + chunk.max.z) / 2);
    }
    this.stats.chunks = this.chunks.length;
  }

  /** Whether any chunk is still waiting to be grown at the level it wants. */
  get pending(): number {
    let n = 0;
    for (const chunk of this.chunks) if (chunk.wanted >= 0 && chunk.lod !== chunk.wanted) n += 1;
    return n;
  }

  get treeCount(): number {
    return this.trees.length;
  }

  /** Change the day of the year; every tree is grown again for it. */
  setSeason(season: number): void {
    if (Math.abs(season - this.season) < 1e-4) return;
    this.season = season;
    this.generation += 1;
    this.rowDone.fill(0);
    for (const chunk of this.chunks) {
      chunk.job = null;
      chunk.lod = -1;
    }
  }

  /** Shadow casting is expensive and shows nothing from far above. */
  setCasting(on: boolean): void {
    if (on === this.casting) return;
    this.casting = on;
    for (const chunk of this.chunks) this.applyCasting(chunk);
  }

  private applyCasting(chunk: Chunk): void {
    const cast = this.casting && chunk.lod >= 0 && chunk.lod <= 2;
    if (chunk.wood) chunk.wood.castShadow = cast;
    if (chunk.leaf) chunk.leaf.castShadow = cast;
  }

  private wantedLevel(distance: number, current: number): number {
    if (distance > this.farM) return -2;
    const [a, b, c] = this.lodDistances;
    // A little hysteresis, so a chunk on the border does not flicker between levels.
    const slack = (level: number) => (current >= 0 && current <= level ? 1.1 : 0.92);
    if (distance < a * slack(0)) return 0;
    if (distance < b * slack(1)) return 1;
    if (distance < c * slack(2)) return 2;
    return 3;
  }

  /**
   * @param camera the camera in the forest's frame
   * @param budgetMs how long may be spent growing trees this frame
   * @returns true if the set of drawn meshes changed
   */
  update(camera: THREE.Vector3, budgetMs: number): boolean {
    const started = performance.now();
    let changed = false;
    const queue: Chunk[] = [];
    for (const chunk of this.chunks) {
      const dx = Math.max(chunk.min.x - camera.x, 0, camera.x - chunk.max.x);
      const dy = Math.max(chunk.min.y - camera.y, 0, camera.y - chunk.max.y);
      const dz = Math.max(chunk.min.z - camera.z, 0, camera.z - chunk.max.z);
      chunk.distance = Math.sqrt(dx * dx + dy * dy + dz * dz);
      chunk.wanted = this.wantedLevel(chunk.distance, chunk.lod);
      if (chunk.wanted === -2) {
        chunk.job = null;
        if (chunk.wood?.visible || chunk.leaf?.visible) {
          this.hide(chunk);
          changed = true;
        }
        continue;
      }
      if (chunk.lod !== chunk.wanted) {
        if (chunk.job && chunk.job.lod !== chunk.wanted) chunk.job = null;
        queue.push(chunk);
      } else if (chunk.wood && !chunk.wood.visible) {
        chunk.wood.visible = true;
        if (chunk.leaf) chunk.leaf.visible = true;
        changed = true;
      }
    }
    this.stats.jobs = queue.length;
    queue.sort((p, q) => p.distance - q.distance);
    for (const chunk of queue) {
      if (performance.now() - started >= budgetMs) break;
      chunk.job ??= { chunk, lod: chunk.wanted, next: 0, grown: [] };
      if (this.advance(chunk.job, started + budgetMs)) changed = true;
    }
    return changed;
  }

  private hide(chunk: Chunk): void {
    if (chunk.wood) chunk.wood.visible = false;
    if (chunk.leaf) chunk.leaf.visible = false;
  }

  /** Grow trees for a job until the deadline; finish the chunk when all are grown. */
  private advance(job: Job, deadline: number): boolean {
    const { chunk } = job;
    const generation = this.generation;
    while (job.next < chunk.trees.length) {
      const id = chunk.trees[job.next];
      const tree = this.trees[id];
      const t0 = performance.now();
      const data = this.grower.grow(
        {
          species: tree.species,
          seed: tree.seed,
          height: tree.height,
          openness: tree.openness,
          age: tree.age,
          season: this.season,
          health: tree.health,
          lift: tree.lift,
        },
        job.lod,
      );
      this.stats.growMs += performance.now() - t0;
      if (!this.rowDone[id]) this.writeRow(id, data);
      job.grown.push(data);
      job.next += 1;
      if (performance.now() >= deadline && job.next < chunk.trees.length) return false;
    }
    if (generation !== this.generation) return false;
    this.finish(job);
    return true;
  }

  private writeRow(id: number, d: TreeData): void {
    // The grower speaks sRGB colours; the shaders light in linear.
    const lin = (c: [number, number, number]) => c.map((x) => x ** 2.2);
    treeTable.put(this.rowBase + id, [
      [...lin(d.foliage), d.leafForm],
      [...lin(d.autumnColour), d.autumn],
      [...lin(d.bloomColour), d.bloom],
      [...lin(d.bark), d.fissure],
      [d.flush, d.leafAspect, d.cover, 0],
    ]);
    this.rowDone[id] = 1;
  }

  private finish(job: Job): void {
    const { chunk, lod, grown } = job;
    let segments = 0;
    let leaves = 0;
    for (const d of grown) {
      segments += d.segmentCount;
      leaves += d.leafCount;
    }
    const woodA = new Float32Array(segments * 4);
    const woodB = new Float32Array(segments * 4);
    const woodId = new Uint16Array(segments);
    const leafPos = new Float32Array(leaves * 4);
    const leafDir = new Int8Array(leaves * 4);
    const leafNor = new Int8Array(leaves * 4);
    const leafShape = new Uint8Array(leaves * 4);
    const leafTint = new Uint8Array(leaves * 4);
    const leafId = new Uint16Array(leaves);
    let s = 0;
    let l = 0;
    grown.forEach((d, k) => {
      const id = chunk.trees[k];
      const tree = this.trees[id];
      const { x, y, z } = tree;
      for (let i = 0; i < d.segmentCount; i += 1) {
        const f = i * 8;
        const o = (s + i) * 4;
        woodA[o] = d.segments[f] + x;
        woodA[o + 1] = d.segments[f + 1] + y;
        woodA[o + 2] = d.segments[f + 2] + z;
        woodA[o + 3] = d.segments[f + 3];
        woodB[o] = d.segments[f + 4] + x;
        woodB[o + 1] = d.segments[f + 5] + y;
        woodB[o + 2] = d.segments[f + 6] + z;
        woodB[o + 3] = d.segments[f + 7];
        woodId[s + i] = this.rowBase + id;
      }
      s += d.segmentCount;
      for (let i = 0; i < d.leafCount; i += 1) {
        const f = i * 4;
        const o = (l + i) * 4;
        leafPos[o] = d.leafPlacement[f] + x;
        leafPos[o + 1] = d.leafPlacement[f + 1] + y;
        leafPos[o + 2] = d.leafPlacement[f + 2] + z;
        leafPos[o + 3] = d.leafPlacement[f + 3];
        leafId[l + i] = this.rowBase + id;
      }
      leafDir.set(d.leafDirection, l * 4);
      leafNor.set(d.leafNormal, l * 4);
      leafShape.set(d.leafShape, l * 4);
      leafTint.set(d.leafTint, l * 4);
      l += d.leafCount;
    });

    const sphere = new THREE.Sphere();
    new THREE.Box3(chunk.min, chunk.max).getBoundingSphere(sphere);
    sphere.radius += 2;

    const wood = woodBase(SIDES[lod]);
    wood.setAttribute("aSegA", new THREE.InstancedBufferAttribute(woodA, 4));
    wood.setAttribute("aSegB", new THREE.InstancedBufferAttribute(woodB, 4));
    wood.setAttribute("aTreeId", new THREE.InstancedBufferAttribute(woodId, 1));
    wood.instanceCount = segments;
    wood.boundingSphere = sphere.clone();

    const leaf = leafBase();
    leaf.setAttribute("aLeafPos", new THREE.InstancedBufferAttribute(leafPos, 4));
    leaf.setAttribute("aLeafDir", new THREE.InstancedBufferAttribute(leafDir, 4, true));
    leaf.setAttribute("aLeafNor", new THREE.InstancedBufferAttribute(leafNor, 4, true));
    leaf.setAttribute("aLeafShape", new THREE.InstancedBufferAttribute(leafShape, 4, true));
    leaf.setAttribute("aLeafTint", new THREE.InstancedBufferAttribute(leafTint, 4, true));
    leaf.setAttribute("aTreeId", new THREE.InstancedBufferAttribute(leafId, 1));
    leaf.instanceCount = leaves;
    leaf.boundingSphere = sphere.clone();

    const woodMesh = new THREE.Mesh(wood, this.materials.wood);
    woodMesh.customDepthMaterial = this.materials.woodDepth;
    woodMesh.receiveShadow = true;
    woodMesh.name = `forest/${chunk.key}/wood`;
    const leafMesh = new THREE.Mesh(leaf, this.materials.leaf);
    leafMesh.customDepthMaterial = this.materials.leafDepth;
    leafMesh.receiveShadow = true;
    leafMesh.name = `forest/${chunk.key}/leaf`;

    this.release(chunk);
    chunk.wood = woodMesh;
    chunk.leaf = leafMesh;
    chunk.lod = lod;
    chunk.job = null;
    this.group.add(woodMesh, leafMesh);
    this.applyCasting(chunk);
    this.recount();
  }

  private release(chunk: Chunk): void {
    for (const mesh of [chunk.wood, chunk.leaf]) {
      if (!mesh) continue;
      mesh.removeFromParent();
      mesh.geometry.dispose();
    }
    chunk.wood = null;
    chunk.leaf = null;
  }

  private recount(): void {
    let built = 0;
    let trees = 0;
    let segments = 0;
    let leaves = 0;
    for (const chunk of this.chunks) {
      if (!chunk.wood || !chunk.leaf) continue;
      built += 1;
      trees += chunk.trees.length;
      segments += (chunk.wood.geometry as THREE.InstancedBufferGeometry).instanceCount;
      leaves += (chunk.leaf.geometry as THREE.InstancedBufferGeometry).instanceCount;
    }
    Object.assign(this.stats, { built, trees, segments, leaves });
  }

  dispose(): void {
    for (const chunk of this.chunks) this.release(chunk);
    this.materials.dispose();
    treeTable.release(this.rowBase, this.trees.length);
    this.group.removeFromParent();
  }
}
