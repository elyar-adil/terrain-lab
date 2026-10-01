/**
 * The tree grower, loaded as WebAssembly.
 *
 * The grower is Rust (`crates/worldgen-trees`) and is the same code whether a game,
 * a lidar simulator or this renderer asks for a tree: a pure function from a tree's
 * spec to its branches and its leaves. This module is only the foreign-function
 * glue: it calls the C interface in `ffi.rs` and cuts the returned buffer into typed
 * arrays the GPU can take.
 */

/** What the grower needs to know to grow one particular tree. */
export interface TreeSpec {
  /** Index into the species catalogue (see {@link speciesIndex}). */
  species: number;
  /** The tree's own identity; every branch and leaf is derived from it. */
  seed: number;
  height: number;
  /** 0: grown in a stand (narrow); 1: open-grown (broad). */
  openness?: number;
  age?: number;
  /** Day of the year, 0..1. */
  season?: number;
  health?: number;
  /** Lowest limb, metres. */
  lift?: number;
}

/** One grown tree, ready to upload. All geometry is in the tree's own frame, metres, y up. */
export interface TreeData {
  species: number;
  leafForm: number;
  lod: number;
  height: number;
  crownRadius: number;
  crownBase: number;
  trunkRadius: number;
  bark: [number, number, number];
  cover: number;
  autumn: number;
  flush: number;
  bloom: number;
  foliage: [number, number, number];
  autumnColour: [number, number, number];
  bloomColour: [number, number, number];
  leafAspect: number;
  fissure: number;
  /** `segmentCount * 8`: `a.xyz, ra, b.xyz, rb`. */
  segments: Float32Array;
  segmentCount: number;
  /** `leafCount * 4`: `pos.xyz, length`. */
  leafPlacement: Float32Array;
  /** `leafCount * 4` signed bytes: blade axis, blade normal. */
  leafDirection: Int8Array;
  leafNormal: Int8Array;
  /** `leafCount * 4` bytes: outline parameters and tint. */
  leafShape: Uint8Array;
  leafTint: Uint8Array;
  leafCount: number;
}

interface Exports {
  memory: WebAssembly.Memory;
  tree_species_count(): number;
  tree_species_key(index: number): number;
  tree_generate(
    species: number,
    seedLo: number,
    seedHi: number,
    height: number,
    openness: number,
    age: number,
    season: number,
    health: number,
    lift: number,
    lod: number,
  ): number;
  tree_buffer_ptr(): number;
}

const HEADER_WORDS = 32;

export class TreeGrower {
  private readonly exports: Exports;
  readonly speciesKeys: string[];

  private constructor(exports: Exports) {
    this.exports = exports;
    const keys: string[] = [];
    const decoder = new TextDecoder();
    for (let i = 0; i < exports.tree_species_count(); i += 1) {
      const length = exports.tree_species_key(i);
      keys.push(decoder.decode(new Uint8Array(exports.memory.buffer, exports.tree_buffer_ptr(), length)));
    }
    this.speciesKeys = keys;
  }

  static async load(url = "/worldgen_trees.wasm"): Promise<TreeGrower> {
    const response = await fetch(url);
    if (!response.ok) throw new Error(`${url}: ${response.status} ${response.statusText}`);
    const bytes = await response.arrayBuffer();
    const { instance } = await WebAssembly.instantiate(bytes, {});
    return new TreeGrower(instance.exports as unknown as Exports);
  }

  /** The catalogue index of a species key, or -1. */
  speciesIndex(key: string): number {
    return this.speciesKeys.indexOf(key);
  }

  /** Grow a tree at a level of detail (0 nearest .. 3 farthest). */
  grow(spec: TreeSpec, lod: number): TreeData {
    const seed = Math.floor(spec.seed);
    const length = this.exports.tree_generate(
      spec.species,
      seed >>> 0,
      Math.floor(seed / 4294967296) >>> 0,
      spec.height,
      spec.openness ?? 0.7,
      spec.age ?? 0.7,
      spec.season ?? 0.5,
      spec.health ?? 1,
      spec.lift ?? 0,
      lod,
    );
    // Copy out: the next call overwrites the buffer, and memory growth detaches views.
    const raw = new Uint8Array(this.exports.memory.buffer, this.exports.tree_buffer_ptr(), length).slice().buffer;
    const words = new DataView(raw);
    const u32 = (i: number) => words.getUint32(i * 4, true);
    const f32 = (i: number) => words.getFloat32(i * 4, true);
    if (u32(0) !== 0x45455254) throw new Error("not a tree buffer");
    const segmentCount = u32(2);
    const leafCount = u32(3);
    const vec3 = (i: number): [number, number, number] => [f32(i), f32(i + 1), f32(i + 2)];
    let at = HEADER_WORDS * 4;
    const segments = new Float32Array(raw, at, segmentCount * 8);
    at += segmentCount * 32;
    const leafPlacement = new Float32Array(raw, at, leafCount * 4);
    at += leafCount * 16;
    const leafDirection = new Int8Array(raw, at, leafCount * 4);
    at += leafCount * 4;
    const leafNormal = new Int8Array(raw, at, leafCount * 4);
    at += leafCount * 4;
    const leafShape = new Uint8Array(raw, at, leafCount * 4);
    at += leafCount * 4;
    const leafTint = new Uint8Array(raw, at, leafCount * 4);
    return {
      species: u32(15),
      leafForm: u32(16),
      lod: u32(28),
      height: f32(4),
      crownRadius: f32(5),
      crownBase: f32(6),
      trunkRadius: f32(7),
      bark: vec3(8),
      cover: f32(11),
      autumn: f32(12),
      flush: f32(13),
      bloom: f32(14),
      foliage: vec3(17),
      autumnColour: vec3(20),
      bloomColour: vec3(23),
      leafAspect: f32(26),
      fissure: f32(27),
      segments,
      segmentCount,
      leafPlacement,
      leafDirection,
      leafNormal,
      leafShape,
      leafTint,
      leafCount,
    };
  }
}
