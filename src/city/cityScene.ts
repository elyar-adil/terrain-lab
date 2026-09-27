/**
 * The city scene layer.
 *
 * Everything here uploads geometry that Rust already derived. The renderer's job
 * is to decode base64 buffers into `BufferGeometry`, resolve a material key into a
 * `MeshStandardMaterial`, and drive the traffic and signal animation. It must not
 * re-derive geometry: the lane graph the paint was drawn from and the lane graph
 * the traffic follows are the same data, and a renderer that rebuilt either would
 * eventually disagree with the other.
 *
 * The one thing this layer *does* decide is appearance, and it decides it
 * physically. Every colour here is a reflectance; every light is a real light; the
 * tone curve is a filmic curve. Nothing is brightened to look better, because a
 * render that has been brightened to look better is the thing this project is
 * trying to stop producing.
 */

import * as THREE from "three";

/** One vertex buffer group from the payload. */
export interface SceneMesh {
  material: string;
  /** base64 little-endian `f32`, three per vertex. */
  positions: string;
  /** base64 little-endian `f32`, three per vertex. */
  normals: string;
  /** base64 `u8` RGBA, four per vertex. Present only where the material is tinted. */
  colors?: string;
  /** base64 little-endian `f32`, two per vertex, **in metres**. */
  uvs?: string;
  /** base64 little-endian `u32`. */
  indices: string;
  /** The instance list this geometry is drawn with, if it is a prototype. */
  instanceOf?: string;
  vertexCount: number;
  triangleCount: number;
  castShadow: boolean;
  receiveShadow: boolean;
  alphaCutout: boolean;
  dynamic: boolean;
}

/** A shared instance list: ten floats per instance. */
export interface SceneInstances {
  key: string;
  count: number;
  data: string;
}

export interface SceneTexture {
  name: string;
  width: number;
  height: number;
  tileWidthM: number;
  tileHeightM: number;
  hasNormalSource: boolean;
  data: string;
}

export interface SignalLamp {
  /** 0 red, 1 yellow, 2 green. */
  aspect: number;
  position: [number, number, number];
}

export interface SignalRig {
  id: string;
  junction: number;
  road: number;
  axis: "ns" | "ew";
  lamps: SignalLamp[];
}

export interface AgentPose {
  x: number;
  y: number;
  z: number;
  heading: number;
  speed: number;
  colour: number;
  stopped: boolean;
}

export interface TrafficState {
  time: number;
  agents: AgentPose[];
  aspects: number[];
  stalled: string[];
}

export interface CityScene {
  version: number;
  seed: number;
  /** World kilometre point the local frame's origin sits at. */
  origin: [number, number];
  rotationRadians: number;
  /** Local extent, metres: `minX, minZ, maxX, maxZ`. */
  extentM: [number, number, number, number];
  meshes: SceneMesh[];
  instances: SceneInstances[];
  textures: SceneTexture[];
  signals: SignalRig[];
  traffic: TrafficState;
  stats: Record<string, unknown>;
}

// --- buffer decoding --------------------------------------------------------

function base64ToBytes(base64: string): Uint8Array {
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  for (let index = 0; index < binary.length; index += 1) {
    bytes[index] = binary.charCodeAt(index);
  }
  return bytes;
}

/**
 * Decode a base64 little-endian `f32` buffer.
 *
 * A 400 MB JSON document cannot be parsed with `JSON.parse` in reasonable memory,
 * and a city scene is tens of megabytes of vertex data, so the payload is base64
 * rather than a JSON array of numbers.
 */
export function decodeFloats(base64: string, expected: number): Float32Array {
  const bytes = base64ToBytes(base64);
  // Every buffer is a whole number of 4-byte elements; a mismatch means the
  // payload and the renderer disagree about the schema, and silently
  // reinterpreting it would produce garbage geometry instead of an error.
  if (bytes.length !== expected * 4) {
    throw new Error(`buffer is ${bytes.length} bytes, expected ${expected * 4}`);
  }
  // Copy into a correctly-aligned view: `Uint8Array` from `atob` has no
  // guarantee of 4-byte alignment, and `Float32Array` requires it.
  const copy = bytes.slice();
  return new Float32Array(copy.buffer, 0, expected);
}

export function decodeIndices(base64: string, expected: number): Uint32Array {
  const bytes = base64ToBytes(base64);
  if (bytes.length !== expected * 4) {
    throw new Error(`index buffer is ${bytes.length} bytes, expected ${expected * 4}`);
  }
  const copy = bytes.slice();
  return new Uint32Array(copy.buffer, 0, expected);
}

// --- materials --------------------------------------------------------------

/**
 * Aspects 0 red, 1 yellow, 2 green, plus the unlit state.
 *
 * The lit values are emissive rather than diffuse: a signal lamp that is *off* in
 * daylight still shows its own colour, and one that is *on* is a light source.
 * Driving both from the same number is why the previous render's signals were
 * either invisible or painted-on.
 */
const ASPECT_EMISSIVE = [0xff2a18, 0xffb400, 0x18ff5a];
const ASPECT_OFF = [0x2a0a08, 0x2a2208, 0x0a2a16];

export interface MaterialCatalogue {
  get(key: string): THREE.Material;
  /** Texture-backed materials need their normal map regenerated from alpha. */
  readonly textures: THREE.DataTexture[];
  /** Signal lenses, one instanced sphere per aspect, all lamps in one draw. */
  readonly lamps: THREE.InstancedMesh[];
  readonly missing: Set<string>;
  /**
   * Material keys whose UV buffer did not match its vertex count.
   *
   * This is a real defect in the scene layer, not a rendering problem: a group
   * that receives both `quad` (UVs optional) and `quad_uv` (UVs mandatory) ends
   * up with fewer UVs than vertices, and the UVs that are present are silently
   * attributed to the wrong vertices. The result is exactly the vertical-streak
   * facades this renderer was written to eliminate.
   */
  readonly uvMismatch: Set<string>;
  dispose(): void;
}

/** Every material key the scene can emit, so a missing one is a loud error. */
const FACADE_COUNT = 24;

const TREE_LEAF_PREFIX = "leaf/";

/**
 * Build every material the city binds.
 *
 * The `repeat` of every texture is the reciprocal of its physical tile size,
 * because UVs arrive in metres. That one line is the whole reason a 30-storey
 * tower and a 4-storey slab both show correctly scaled windows, and it is exactly
 * what the previous renderer got wrong: it emitted UVs in scene units, so every
 * wall sampled a single near-uniform texel and came out as a vertical streak.
 */
export function createMaterials(textures: SceneTexture[]): MaterialCatalogue {
  const sources = new Map<string, SceneTexture>();
  for (const texture of textures) sources.set(texture.name, texture);

  const built: THREE.Material[] = [];
  const builtTextures: THREE.DataTexture[] = [];
  const cache = new Map<string, THREE.DataTexture>();
  const missing = new Set<string>();
  const table = new Map<string, THREE.Material>();

  const textureFor = (name: string): THREE.DataTexture | null => {
    const cached = cache.get(name);
    if (cached) return cached;
    const source = sources.get(name);
    if (!source) return null;
    const texture = new THREE.DataTexture(
      base64ToBytes(source.data),
      source.width,
      source.height,
      THREE.RGBAFormat,
    );
    texture.wrapS = THREE.RepeatWrapping;
    texture.wrapT = THREE.RepeatWrapping;
    // A road marking seen at a grazing angle is the worst case for aliasing in
    // the whole scene, so anisotropy is not optional.
    texture.anisotropy = 16;
    texture.magFilter = THREE.LinearFilter;
    texture.minFilter = THREE.LinearMipmapLinearFilter;
    texture.generateMipmaps = true;
    texture.colorSpace = THREE.SRGBColorSpace;
    /**
     * `flipY = false` is load-bearing, and it is the default, which is exactly
     * why it is stated here.
     *
     * A `DataTexture` built from a raw byte array samples **row 0** at `V = 0`.
     * Rust bakes row 0 as the *top* of the image, so `V` counts downward through
     * the image as `V` increases. Every facade's `V` coordinate therefore counts
     * *down* the wall from its head — `buildings::facade_wall` anchors `V = 0` at
     * the parapet for exactly this reason. Setting `flipY = true` here, or
     * flipping the convention in Rust without flipping it here, renders every
     * building upside down with its floor lines in the wrong place, and nothing
     * errors.
     */
    texture.flipY = false;
    // UVs are metres; one tile covers the texture's physical size.
    texture.repeat.set(1 / source.tileWidthM, 1 / source.tileHeightM);
    texture.needsUpdate = true;
    cache.set(name, texture);
    builtTextures.push(texture);
    return texture;
  };

  /**
   * Derive a normal map from a texture's height field.
   *
   * The height lives in the **alpha** channel, which is what `hasNormalSource`
   * promises. It used to be read from luminance, which is a silent mismatch: the
   * bake put the relief in alpha precisely so the albedo could stay independent of
   * it, and reading luminance produced a normal map of whatever the roof's colour
   * happened to be. Alpha is used when it carries variation, and luminance is
   * accepted as a fallback so a texture that sets the flag without encoding a
   * field still gets relief rather than a flat plane.
   *
   * Sampling is 4-connected rather than Sobel. The height field is already smooth
   * at bake resolution, and a wide kernel over a low-resolution field flattens
   * real relief into noise that shimmers under a moving sun.
   */
  const normalFor = (name: string, strength: number): THREE.DataTexture | null => {
    const source = sources.get(name);
    if (!source?.hasNormalSource) return null;
    const key = `${name}#normal`;
    const cached = cache.get(key);
    if (cached) return cached;
    const albedo = textureFor(name);
    if (!albedo) return null;
    const { width, height } = source;
    const pixels = albedo.image.data as Uint8Array;
    // Decide once which channel carries the field, rather than per texel: mixing
    // the two inside a single normal map produces a seam down the middle of it.
    let lowest = 255;
    let highest = 0;
    for (let index = 3; index < pixels.length; index += 4) {
      lowest = Math.min(lowest, pixels[index]);
      highest = Math.max(highest, pixels[index]);
    }
    const alphaCarries = highest - lowest > 8;
    const sample = (x: number, y: number) => {
      const xx = (x + width) % width;
      const yy = (y + height) % height;
      const index = (yy * width + xx) * 4;
      if (alphaCarries) return pixels[index + 3] / 255;
      return (pixels[index] * 0.2126
        + pixels[index + 1] * 0.7152
        + pixels[index + 2] * 0.0722) / 255;
    };
    const normal = new Uint8Array(width * height * 4);
    for (let y = 0; y < height; y += 1) {
      for (let x = 0; x < width; x += 1) {
        const nx = (sample(x - 1, y) - sample(x + 1, y)) * strength;
        const ny = (sample(x, y - 1) - sample(x, y + 1)) * strength;
        const nz = 1.0;
        const length = Math.hypot(nx, ny, nz);
        const index = (y * width + x) * 4;
        normal[index] = ((nx / length) * 0.5 + 0.5) * 255;
        normal[index + 1] = ((ny / length) * 0.5 + 0.5) * 255;
        normal[index + 2] = ((nz / length) * 0.5 + 0.5) * 255;
        normal[index + 3] = 255;
      }
    }
    const map = new THREE.DataTexture(normal, width, height, THREE.RGBAFormat);
    map.wrapS = THREE.RepeatWrapping;
    map.wrapT = THREE.RepeatWrapping;
    map.flipY = false;
    map.anisotropy = 8;
    map.minFilter = THREE.LinearMipmapLinearFilter;
    map.generateMipmaps = true;
    map.repeat.copy(albedo.repeat);
    map.needsUpdate = true;
    cache.set(key, map);
    builtTextures.push(map);
    return map;
  };

  const register = (key: string, material: THREE.Material) => {
    table.set(key, material);
    built.push(material);
    return material;
  };

  const standard = (
    key: string,
    options: THREE.MeshStandardMaterialParameters,
    colour: number,
  ) =>
    register(
      key,
      new THREE.MeshStandardMaterial({ color: colour, ...options }),
    ) as THREE.MeshStandardMaterial;

  // --- ground ---------------------------------------------------------------
  // Asphalt's albedo is 4-12%: it is nearly black, and it is *slightly blue*.
  // Painting it mid-grey is the single most common way a rendered road looks
  // like poured concrete, which is why the number here is low and the normal map
  // carries the detail.
  const asphaltMap = textureFor("ground/asphalt");
  standard(
    "asphalt",
    {
      map: asphaltMap ?? undefined,
      normalMap: normalFor("ground/asphalt", 2.4) ?? undefined,
      roughness: 0.88,
      metalness: 0.0,
      // Wet-looking wheel tracks are a lower-roughness stripe, not a lighter
      // colour; the bake has already darkened them.
    },
    asphaltMap ? 0xffffff : 0x2a2c30,
  );
  // The shoulder / carriageway sweep wall: weathered, dustier than the road.
  standard("asphalt.pavement", { roughness: 0.95 }, 0x4a4a46);

  const pavingMap = textureFor("ground/paving");
  standard(
    "sidewalk",
    {
      map: pavingMap ?? undefined,
      normalMap: normalFor("ground/paving", 1.6) ?? undefined,
      roughness: 0.9,
    },
    pavingMap ? 0xffffff : 0x6e6e68,
  );
  standard("parcel.paving", { map: pavingMap ?? undefined, roughness: 0.92 }, 0xffffff);
  // Kerb concrete: 25-35% reflectance, and a shade warmer than the asphalt it
  // sits beside, which is what makes a kerb line read at distance.
  standard("kerb", { roughness: 0.86 }, 0x8e8b82);

  const grassMap = textureFor("ground/grass");
  for (const key of ["block.ground", "parcel.green", "median.plant"]) {
    standard(
      key,
      { map: grassMap ?? undefined, normalMap: normalFor("ground/grass", 1.2) ?? undefined, roughness: 0.95 },
      grassMap ? 0xffffff : 0x2e3a22,
    );
  }
  standard("hedge", { roughness: 0.9, flatShading: true }, 0x22381a);
  standard("barrier.concrete", { roughness: 0.9 }, 0x9a988e);
  // Hot-dip galvanised steel: bright, but metallic, so it takes its colour from
  // the sky rather than from its own albedo.
  standard("rail.steel", { roughness: 0.42, metalness: 0.75, envMapIntensity: 0.9 }, 0x8f9498);
  standard("bridge.concrete", { roughness: 0.9 }, 0x7c7a72);
  standard("wire", { roughness: 0.55, metalness: 0.3, side: THREE.DoubleSide }, 0x14161a);

  // --- road paint -----------------------------------------------------------
  /**
   * Paint is 55-70% reflectance — one of the brightest things in the scene — and
   * it is *retroreflective*, so it stays bright at a grazing angle where an
   * ordinary diffuse surface would vanish. A low roughness plus a small emissive
   * term reproduces that without pretending the paint emits light.
   *
   * `polygonOffset` pulls the paint toward the camera in depth rather than
   * lifting it physically, so a marking is millimetres above the asphalt instead
   * of a modelled slab that would catch a shadow and z-fight.
   */
  const paint = (key: string, colour: number, map: THREE.DataTexture | null, cutout = false) =>
    register(
      key,
      new THREE.MeshStandardMaterial({
        color: map ? 0xffffff : colour,
        map: map ?? undefined,
        alphaTest: cutout ? 0.5 : 0,
        transparent: false,
        roughness: 0.45,
        metalness: 0.0,
        emissive: new THREE.Color(map ? 0x000000 : colour).multiplyScalar(0.06),
        polygonOffset: true,
        polygonOffsetFactor: -4,
        polygonOffsetUnits: -4,
        envMapIntensity: 0.5,
      }),
    );
  // Chinese lane paint is a warm off-white, never pure white.
  paint("marking.white", 0xe8e6dc, null);
  // Chinese centre lines are a deep chrome yellow, not a pastel one.
  paint("marking.yellow", 0xd8a41c, null);
  for (const key of ["marking.crosswalk", "marking.dashed-3-5", "marking.dashed-6-9"]) {
    paint(key, 0xe8e6dc, textureFor(key), true);
  }

  // --- buildings ------------------------------------------------------------
  const roofMap = textureFor("roof");
  standard(
    "roof",
    { map: roofMap ?? undefined, normalMap: normalFor("roof", 1.4) ?? undefined, roughness: 0.94 },
    roofMap ? 0xffffff : 0x6a6862,
  );
  for (let index = 0; index < FACADE_COUNT; index += 1) {
    const key = `facade/${index.toString().padStart(2, "0")}`;
    const map = textureFor(key);
    // Tiles 8..15 are the curtain walls. Glass has a *low* albedo and is bright
    // only because it reflects the sky, so it gets low roughness, some metalness
    // and a strong environment term. Masonry stays rough and mostly diffuse.
    const glass = index >= 8 && index <= 15;
    standard(
      key,
      {
        map: map ?? undefined,
        roughness: glass ? 0.16 : 0.85,
        metalness: glass ? 0.55 : 0.0,
        envMapIntensity: glass ? 1.25 : 0.5,
      },
      map ? 0xffffff : 0xa8a49c,
    );
  }
  for (const kind of ["shop", "lobby", "home"]) {
    standard(`ground/${kind}`, { map: textureFor(`ground/${kind}`) ?? undefined, roughness: 0.72 }, 0xffffff);
  }
  // Backlit shop signage: emissive, because a lit sign is a light source.
  register(
    "sign/shop",
    new THREE.MeshStandardMaterial({
      map: textureFor("sign/shop") ?? undefined,
      roughness: 0.4,
      emissiveMap: textureFor("sign/shop") ?? undefined,
      emissive: 0xffffff,
      emissiveIntensity: 0.55,
    }),
  );
  standard("trim.light", { roughness: 0.72 }, 0x8e887c);
  standard("trim.dark", { roughness: 0.45, metalness: 0.2 }, 0x2a2f34);
  standard("balcony.slab", { roughness: 0.82 }, 0x7e7a70);
  // Air-conditioner condensers: off-white plastic, and there are hundreds of
  // them on a Chinese residential block.
  standard("metal.ac", { roughness: 0.5, metalness: 0.05 }, 0xb8b6ae);
  standard("awning", { roughness: 0.55 }, 0x1c262c);
  standard("wall.render", { roughness: 0.88 }, 0x9c968a);

  // --- street furniture and signals ----------------------------------------
  standard("pole.concrete", { roughness: 0.9 }, 0x807c74);
  standard("steel", { roughness: 0.38, metalness: 0.72, envMapIntensity: 0.9 }, 0x9aa0a6);
  standard("furniture/lamp", { roughness: 0.55, metalness: 0.35 }, 0x74797c);
  standard("furniture/pole", { roughness: 0.9 }, 0x807c74);
  standard("furniture/bollard", { roughness: 0.5, metalness: 0.2 }, 0xb8bcb6);
  standard("furniture/railing", { roughness: 0.45, metalness: 0.5 }, 0xa8adb0);
  standard("furniture/shelter", { roughness: 0.55, metalness: 0.25 }, 0x6e747a);
  // Chinese guide signs are blue with white text and a white border.
  standard("furniture/sign.guide", { roughness: 0.45 }, 0x0f4c96);
  standard("furniture/sign.crossing", { roughness: 0.45 }, 0x0f4c96);
  standard("signal.body", { roughness: 0.55, metalness: 0.35 }, 0x1a1e21);
  standard("car/body", { roughness: 0.3, metalness: 0.35, envMapIntensity: 1.0, vertexColors: true }, 0xffffff);
  standard("car/glass", { roughness: 0.12, metalness: 0.5, envMapIntensity: 1.1 }, 0x1c2429);
  standard("car/wheel", { roughness: 0.9 }, 0x101012);
  standard("bark", { roughness: 0.95, flatShading: true, vertexColors: true }, 0xffffff);

  // Foliage: alpha-cut, double-sided, tinted per instance and per vertex.
  // `side: DoubleSide` is required — a leaf card is a quad, and half of them face
  // away from the camera. Without it a canopy has holes in it.
  const leafMaterial = (
    key: string,
    cardName: string,
  ): THREE.MeshStandardMaterial => {
    const map = textureFor(cardName);
    return register(
      key,
      new THREE.MeshStandardMaterial({
        map: map ?? undefined,
        alphaTest: map ? 0.42 : 0,
        side: THREE.DoubleSide,
        vertexColors: true,
        roughness: 0.86,
        metalness: 0.0,
        // Leaves are thin and translucent: a little transmission is what
        // separates a lit canopy from a painted green sphere.
        emissive: 0x000000,
      }),
    ) as THREE.MeshStandardMaterial;
  };
  // The generic card, used by anything that does not name a species.
  leafMaterial("leaf", "vegetation/leaf");

  const tuftMap = textureFor("vegetation/tuft");
  register(
    "tuft",
    new THREE.MeshStandardMaterial({
      map: tuftMap ?? undefined,
      alphaTest: tuftMap ? 0.45 : 0,
      side: THREE.DoubleSide,
      vertexColors: true,
      roughness: 0.92,
    }),
  );

  // Signal lenses. One instanced sphere per aspect for the whole city, so a
  // phase change is a colour write and never a geometry rebuild. `emissive` is
  // what makes a lit lamp read as a light source rather than a painted dot.
  const lamps: THREE.InstancedMesh[] = [];
  for (let aspect = 0; aspect < 3; aspect += 1) {
    const geometry = new THREE.SphereGeometry(0.17, 10, 8);
    const material = new THREE.MeshStandardMaterial({
      color: ASPECT_OFF[aspect],
      emissive: ASPECT_EMISSIVE[aspect],
      emissiveIntensity: 0,
      roughness: 0.25,
    });
    built.push(material);
    lamps.push(new THREE.InstancedMesh(geometry, material, 1));
    lamps[aspect].count = 0;
    lamps[aspect].frustumCulled = false;
  }

  return {
    get(key: string): THREE.Material {
      const found = table.get(key);
      if (found) return found;
      // A tree leaf material names its species: `leaf/<species-key>`.
      if (key.startsWith(TREE_LEAF_PREFIX)) {
        const species = key.slice(TREE_LEAF_PREFIX.length);
        const card = textureFor(`vegetation/leaf/${species}`);
        if (card) {
          const made = leafMaterial(key, `vegetation/leaf/${species}`);
          return made;
        }
      }
      missing.add(key);
      // Loud on purpose. A silent fallback to white is how the previous renderer
      // produced a city of white monoliths without anybody noticing.
      const fallback = new THREE.MeshBasicMaterial({ color: 0xff00ff });
      built.push(fallback);
      table.set(key, fallback);
      return fallback;
    },
    textures: builtTextures,
    lamps,
    missing,
    uvMismatch,
    dispose() {
      for (const material of built) material.dispose();
      for (const texture of builtTextures) texture.dispose();
      for (const lamp of lamps) lamp.geometry.dispose();
    },
  };
}

/**
 * One foliage card texture, for the occlusion pass's alpha discard.
 *
 * The species do not all share a card layout, but the occlusion test only has
 * to reject the empty corners of a quad, so any baked card's alpha is close
 * enough — the same trade the reference project makes.
 */
export function foliageMaskTexture(textures: SceneTexture[]): THREE.DataTexture | null {
  const source = textures.find((texture) => texture.name.startsWith("vegetation/leaf"));
  if (!source) return null;
  const texture = new THREE.DataTexture(
    base64ToBytes(source.data),
    source.width,
    source.height,
    THREE.RGBAFormat,
  );
  texture.magFilter = THREE.LinearFilter;
  // No mipmaps: the discard test has to see the real texel, or distant cards
  // fade to transparent in the G-buffer and stop occluding at all.
  texture.minFilter = THREE.LinearFilter;
  texture.generateMipmaps = false;
  texture.needsUpdate = true;
  return texture;
}

// --- scene assembly ---------------------------------------------------------

/** Everything the animation loop needs to touch after the first upload. */
export interface CityHandles {
  group: THREE.Group;
  materials: MaterialCatalogue;
  /** One instanced mesh per prototype, keyed by instance-list key. */
  instanced: Map<string, THREE.InstancedMesh>;
  /**
   * The decoded prototype behind each instance list: the geometry and
   * material the instanced meshes share. Read-only access for tools like the
   * component gallery that want to place *one* of something; the resources
   * stay owned (and disposed) by the handles.
   */
  prototypes: Map<string, { geometry: THREE.BufferGeometry; material: THREE.Material }>;
  /** The signal lens meshes, by aspect. */
  lamps: THREE.InstancedMesh[];
  /** The prototype geometry for the car, used for the moving traffic fleet. */
  carBody?: THREE.BufferGeometry;
  carGlass?: THREE.BufferGeometry;
  vehicleCount: number;
  dispose(): void;
}

function geometryFor(mesh: SceneMesh, uvMismatch: Set<string>): THREE.BufferGeometry {
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute(
    "position",
    new THREE.BufferAttribute(decodeFloats(mesh.positions, mesh.vertexCount * 3), 3),
  );
  // Foliage is drawn from alpha-cut cards, and the occlusion pass needs to
  // know that per vertex: its G-buffer sees every quad as solid otherwise,
  // which turns each canopy into a self-occluding blob. `post.ts` patches its
  // override material to discard on this attribute; everything else omits it
  // and the attribute defaults to zero.
  if (mesh.alphaCutout && (mesh.material === "tuft" || mesh.material.startsWith("leaf"))) {
    geometry.setAttribute(
      "aLeafCard",
      new THREE.BufferAttribute(new Float32Array(mesh.vertexCount).fill(1), 1),
    );
  }
  geometry.setAttribute(
    "normal",
    new THREE.BufferAttribute(decodeFloats(mesh.normals, mesh.vertexCount * 3), 3),
  );
  if (mesh.colors) {
    // `u8` normalised: four bytes per vertex instead of twelve, and a facade
    // tint needs nowhere near eight bits of headroom to look right.
    geometry.setAttribute(
      "color",
      new THREE.BufferAttribute(base64ToBytes(mesh.colors), 4, true),
    );
  }
  if (mesh.uvs) {
    // In metres. The texture's `repeat` does the conversion.
    const expected = mesh.vertexCount * 2;
    // base64 length -> bytes, without decoding twice: four bytes per float, and
    // base64 expands by 4/3.
    const bytes = (atob(mesh.uvs).length * 3) / 4;
    if (Math.abs(bytes - expected * 4) > 2) {
      /**
       * A short UV buffer is a scene-layer defect, not a rendering one.
       *
       * `MeshBuilder::quad_uv` pushes UVs unconditionally while `MeshBuilder::quad`
       * pushes them only when asked, so a material group that receives both ends
       * up with fewer UVs than vertices — and the UVs that *are* present belong to
       * whichever quads happened to be emitted first. Reinterpreting the array
       * would smear one wall's texture across another.
       *
       * There is no way to recover which vertices are missing, so the honest
       * recovery is to drop UVs for the whole group: the surface renders
       * untextured, which is plainly wrong and therefore plainly visible, rather
       * than subtly wrong. And it is reported, so the caller finds out which
       * material is at fault instead of hunting a wall that looks striped.
       */
      uvMismatch.add(mesh.material);
    } else {
      geometry.setAttribute(
        "uv",
        new THREE.BufferAttribute(decodeFloats(mesh.uvs, expected), 2),
      );
    }
  }
  geometry.setIndex(new THREE.BufferAttribute(decodeIndices(mesh.indices, mesh.triangleCount * 3), 1));
  // The payload states the real extent, so the renderer does not have to walk
  // every vertex to work out a bounding sphere. Doing it correctly matters
  // because an instanced mesh is bounded by its geometry, not by its instances.
  geometry.computeBoundingSphere();
  geometry.computeBoundingBox();
  return geometry;
}

/**
 * Upload one city scene.
 *
 * Static groups become plain meshes. Groups that name an instance list become
 * `InstancedMesh`, which is the whole reason a city can have thousands of
 * leaf-detailed trees in a few dozen draw calls: the tree geometry is uploaded
 * once and the instance buffer positions it.
 */
export function buildCityScene(scene: CityScene, materials: MaterialCatalogue): CityHandles {
  const group = new THREE.Group();
  group.name = "city";
  // Every GPU resource this call allocated. A `THREE.Mesh` does not implement
  // `dispose` — only its geometry does — so the list holds geometries and
  // instanced meshes explicitly rather than objects, which is what makes
  // teardown total.
  const ownedGeometries: THREE.BufferGeometry[] = [];
  const ownedInstanced: THREE.InstancedMesh[] = [];
  const instanced = new Map<string, THREE.InstancedMesh>();
  const prototypes = new Map<string, {
    geometry: THREE.BufferGeometry;
    material: THREE.Material;
  }>();

  // Decode every instance list first: a prototype's geometry may be declared
  // before the list it draws, and the order in the payload is not a contract.
  const lists = new Map<string, { count: number; floats: Float32Array }>();
  for (const list of scene.instances) {
    lists.set(list.key, {
      count: list.count,
      floats: decodeFloats(list.data, list.count * 10),
    });
  }

  const dummy = new THREE.Object3D();
  const colour = new THREE.Color();

  for (const mesh of scene.meshes) {
    if (!mesh.vertexCount || !mesh.triangleCount) continue;
    const geometry = geometryFor(mesh, materials.uvMismatch);
    const material = materials.get(mesh.material);
    const instanceKey = mesh.instanceOf;

    if (instanceKey) {
      const list = lists.get(instanceKey);
      if (!list) {
        // Reported, not silently skipped: an instance list with no geometry is
        // a payload bug and the objects simply do not exist.
        geometry.dispose();
        materials.missing.add(`instance-list:${instanceKey}`);
        continue;
      }
      const target = new THREE.InstancedMesh(geometry, material, Math.max(1, list.count));
      target.name = `${instanceKey}`;
      target.count = list.count;
      target.castShadow = mesh.castShadow;
      target.receiveShadow = mesh.receiveShadow;
      // The geometry is the unit prototype and the instances scale it, so the
      // bounding sphere has to be scaled out to the largest instance or the
      // whole thing is frustum-culled the moment it leaves the origin.
      target.frustumCulled = false;
      for (let index = 0; index < list.count; index += 1) {
        const base = index * 10;
        const floats = list.floats;
        dummy.position.set(floats[base], floats[base + 1], floats[base + 2]);
        dummy.rotation.set(0, floats[base + 3], 0);
        dummy.scale.set(floats[base + 4], floats[base + 5], floats[base + 6]);
        dummy.updateMatrix();
        target.setMatrixAt(index, dummy.matrix);
        // The tenth slot onwards is the instance tint. Only write it when the
        // material actually reads vertex colours, or the attribute buffer is
        // allocated for nothing.
        if ((material as THREE.MeshStandardMaterial).vertexColors) {
          colour.setRGB(floats[base + 7], floats[base + 8], floats[base + 9]);
          target.setColorAt(index, colour);
        }
      }
      target.instanceMatrix.needsUpdate = true;
      if (target.instanceColor) target.instanceColor.needsUpdate = true;
      instanced.set(instanceKey, target);
      prototypes.set(instanceKey, { geometry, material });
      ownedInstanced.push(target);
      group.add(target);
      continue;
    }

    const object = new THREE.Mesh(geometry, material);
    object.name = mesh.material;
    object.castShadow = mesh.castShadow;
    object.receiveShadow = mesh.receiveShadow;
    // Static geometry is not instanced, so its own bounds are correct.
    ownedGeometries.push(geometry);
    group.add(object);
  }

  // The moving fleet draws from the same prototype as the parked cars, so a
  // moving car and a parked car cannot be different vehicles.
  const carPrototype = instanced.get("car/body");
  const glassPrototype = instanced.get("car/glass");
  // Cloned rather than shared: an `InstancedMesh` and a plain `Mesh` may legally
  // point at one buffer, but then the disposal order decides whether the second
  // user gets a dangling handle, and two cities in one page would share it.
  const carBody = carPrototype ? carPrototype.geometry.clone() : undefined;
  const carGlass = glassPrototype ? glassPrototype.geometry.clone() : undefined;
  if (carBody) ownedGeometries.push(carBody);
  if (carGlass) ownedGeometries.push(carGlass);

  return {
    group,
    materials,
    instanced,
    prototypes,
    lamps: materials.lamps,
    vehicleCount: scene.traffic.agents.length,
    carBody,
    carGlass,
    dispose() {
      for (const mesh of ownedInstanced) mesh.dispose();
      for (const geometry of ownedGeometries) geometry.dispose();
      group.clear();
    },
  };
}
