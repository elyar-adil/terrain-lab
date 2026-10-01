/**
 * How a grown tree is drawn.
 *
 * The grower hands over branches (a pair of end points and radii) and leaves (a
 * place, an axis, a normal, a size and a few numbers describing *this* leaf). No
 * mesh is stored for either. The vertex shader raises a tapered tube from every
 * branch segment and a gently curled blade from every leaf; the fragment shader
 * cuts each blade to its own outline, draws its own veins and colours it from its
 * own tint. So two leaves differ because their numbers differ, not because one of
 * a few textures was picked: there is no atlas to repeat.
 *
 * What a tree as a whole looks like (leaf and bark colours, how far autumn has
 * come) lives in one small float table, one row per tree, so a single draw call
 * can carry hundreds of different trees.
 */

import * as THREE from "three";

/** Texels per tree in the table, and trees per table row. */
export const TABLE_TEXELS = 5;
export const TABLE_PER_ROW = 256;

const VERTEX_COMMON = /* glsl */ `
attribute float aTreeKind;
attribute vec4 aSegA;
attribute vec4 aSegB;
attribute vec4 aLeafPos;
attribute vec4 aLeafDir;
attribute vec4 aLeafNor;
attribute vec4 aLeafShape;
attribute vec4 aLeafTint;
attribute float aTreeId;
uniform sampler2D uTreeTable;
varying float vTreeKind;
varying vec4 vTreeColour;
varying vec4 vTreeA;
varying vec4 vTreeB;

vec4 treeRow(float id, int k) {
  int i = int(id + 0.5);
  return texelFetch(uTreeTable, ivec2((i & ${TABLE_PER_ROW - 1}) * ${TABLE_TEXELS} + k, i >> 8), 0);
}

float treeHash(vec3 p) {
  return fract(sin(dot(p, vec3(12.9898, 78.233, 37.719))) * 43758.5453);
}

// One vertex of a branch or a leaf: where it is, which way it faces.
void treeShape(out vec3 pos, out vec3 nor) {
  vTreeKind = aTreeKind;
  if (aTreeKind < 1.5) {
    vec3 axis = aSegB.xyz - aSegA.xyz;
    float len = max(length(axis), 1e-4);
    vec3 w = axis / len;
    vec3 refv = abs(w.y) < 0.9 ? vec3(0.0, 1.0, 0.0) : vec3(1.0, 0.0, 0.0);
    vec3 u = normalize(cross(refv, w));
    vec3 v = cross(w, u);
    float r = mix(aSegA.w, aSegB.w, position.y);
    vec3 radial = u * position.x + v * position.z;
    pos = mix(aSegA.xyz, aSegB.xyz, position.y) + radial * r;
    nor = normalize(radial * len + w * (aSegA.w - aSegB.w));
    vec4 bark = treeRow(aTreeId, 3);
    vTreeColour = vec4(bark.rgb, 1.0);
    vTreeA = vec4(w, bark.w);
    vTreeB = vec4(pos, 0.0);
  } else {
    float form = treeRow(aTreeId, 0).w;
    vec4 foliage = treeRow(aTreeId, 0);
    vec4 autumn = treeRow(aTreeId, 1);
    vec4 bloom = treeRow(aTreeId, 2);
    vec4 extra = treeRow(aTreeId, 4);
    float len = aLeafPos.w;
    vec3 d = normalize(aLeafDir.xyz);
    vec3 n = normalize(aLeafNor.xyz - d * dot(aLeafNor.xyz, d));
    vec3 sd = cross(d, n);
    vec4 sh = aLeafShape;
    vec4 tn = aLeafTint;

    // A flower where the tree is in bloom, a leaf elsewhere.
    float lot = treeHash(aLeafPos.xyz * 7.3 + tn.xyz);
    bool flower = lot < bloom.w;
    float f = flower ? 10.0 + form : form;

    float hw = 0.5;
    if (flower) hw = 0.5;
    else if (form < 0.5 || (form > 1.5 && form < 2.5) || form > 4.5 && form < 5.5) hw = clamp(0.5 * extra.y * (0.9 + 0.25 * sh.z), 0.14, 0.62);
    else if (form < 1.5) hw = 0.62;
    else if (form < 3.5) hw = 0.34;
    else if (form < 4.5) hw = 0.38;
    else hw = 0.6;

    float x = position.x;
    float y = position.y;
    float curl = (sh.w - 0.5) * 2.0;
    pos = aLeafPos.xyz + d * (len * y) + sd * (len * hw * x);
    pos += n * len * (curl * 0.30 * y * y + 0.16 * hw * x * x * (1.0 - 0.5 * y));
    pos -= d * len * abs(curl) * 0.08 * y * y;
    nor = normalize(n - sd * 0.32 * x - d * curl * 0.5 * y);

    // Colour: the tree's foliage, this leaf's own hue, shade and age, and whether
    // it has turned yet.
    float turned = clamp((autumn.w * 1.25 - tn.w * 0.5) / 0.35, 0.0, 1.0);
    vec3 col = foliage.rgb * (0.80 + 0.40 * tn.x);
    col.g *= 0.94 + 0.12 * tn.x;
    col *= mix(0.66, 1.08, tn.y);
    col = mix(col, vec3(0.34, 0.26, 0.12), 0.55 * pow(tn.z, 3.0));
    vec3 aut = autumn.rgb * (0.75 + 0.5 * tn.x) * mix(0.8, 1.05, tn.y);
    col = mix(col, aut, turned);
    col = mix(col, col * vec3(1.22, 1.28, 0.62), extra.x);
    if (flower) col = bloom.rgb * (0.88 + 0.24 * tn.x);

    vTreeColour = vec4(col, tn.y);
    vTreeA = sh;
    vTreeB = vec4(x * hw, y, f, hw);
  }
}
`;

const VERTEX_NORMAL = /* glsl */ `
vec3 objectNormal;
{
  if (aTreeKind > 0.5) {
    vec3 tp;
    treeShape(tp, objectNormal);
  } else {
    vTreeKind = 0.0;
    vTreeColour = vec4(0.0);
    vTreeA = vec4(0.0);
    vTreeB = vec4(0.0);
    objectNormal = vec3(normal);
  }
}
#ifdef USE_TANGENT
  vec3 objectTangent = vec3(tangent.xyz);
#endif
`;

const VERTEX_POSITION = /* glsl */ `
vec3 transformed;
{
  if (aTreeKind > 0.5) {
    vec3 tn;
    treeShape(transformed, tn);
  } else {
    vTreeKind = 0.0;
    vTreeColour = vec4(0.0);
    vTreeA = vec4(0.0);
    vTreeB = vec4(0.0);
    transformed = vec3(position);
  }
}
#ifdef USE_ALPHAHASH
  vPosition = vec3(position);
#endif
`;

/**
 * Value noise, the leaf outlines, and the bark. Shared by every pass that has to
 * agree about where a leaf is: the lit pass, the shadow pass and the occlusion
 * pass all cut the same blades.
 */
const FRAGMENT_COMMON = /* glsl */ `
varying float vTreeKind;
varying vec4 vTreeColour;
varying vec4 vTreeA;
varying vec4 vTreeB;

float tHash(vec3 p) {
  p = fract(p * 0.3183099 + 0.1);
  p *= 17.0;
  return fract(p.x * p.y * p.z * (p.x + p.y + p.z));
}

float tNoise(vec3 x) {
  vec3 i = floor(x);
  vec3 f = fract(x);
  f = f * f * (3.0 - 2.0 * f);
  return mix(
    mix(mix(tHash(i), tHash(i + vec3(1, 0, 0)), f.x), mix(tHash(i + vec3(0, 1, 0)), tHash(i + vec3(1, 1, 0)), f.x), f.y),
    mix(mix(tHash(i + vec3(0, 0, 1)), tHash(i + vec3(1, 0, 1)), f.x), mix(tHash(i + vec3(0, 1, 1)), tHash(i + vec3(1, 1, 1)), f.x), f.y),
    f.z);
}

float sdSeg(vec2 p, vec2 a, vec2 b) {
  vec2 pa = p - a;
  vec2 ba = b - a;
  float h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
  return length(pa - ba * h);
}

// Not an exact distance, but zero on the outline and right in sign and scale.
float sdEllipse(vec2 p, vec2 c, vec2 ab, float ang) {
  vec2 q = p - c;
  float cs = cos(ang);
  float sn = sin(ang);
  q = vec2(cs * q.x + sn * q.y, -sn * q.x + cs * q.y);
  return (length(q / ab) - 1.0) * min(ab.x, ab.y);
}

// The outline of one leaf, in units of its own length; negative inside. q.x is
// across the blade from the midrib, q.y along it from the stalk. 'vein' is how
// much of a vein this spot is.
float leafOutline(vec2 q, float form, float hw, vec4 sh, out float vein) {
  vein = 0.0;
  float t = q.y;
  float asym = sh.z - 0.5;
  if (form > 9.5) {
    // A flower: five petals about a centre.
    vec2 v = q - vec2(0.0, 0.5);
    float r = length(v);
    float th = atan(v.x, v.y);
    float R = 0.34 * (0.72 + 0.28 * abs(cos(2.5 * th + sh.z * 6.0)));
    vein = 1.0 - smoothstep(0.0, 0.07, r);
    return r - R;
  }
  if (form < 0.5 || (form > 1.5 && form < 2.5) || (form > 4.5 && form < 5.5)) {
    float a = form < 0.5 ? 0.62 + 0.38 * sh.z : (form < 2.5 ? 0.9 : 0.8);
    float b = form < 0.5 ? mix(0.95, 0.55, sh.y) : (form < 2.5 ? mix(0.8, 0.5, sh.y) : mix(1.3, 0.9, sh.y));
    float qx = q.x - asym * 0.14 * sin(3.14159 * t);
    float wn = pow(max(sin(3.14159 * pow(clamp(t, 0.0, 1.0), a)), 0.0), b);
    if (form > 1.5 && form < 2.5) {
      wn *= 1.0 + 0.09 * sh.x * (abs(fract(t * (9.0 + 9.0 * sh.x)) - 0.5) * 2.0 - 0.5);
    }
    float side = hw * 0.96 * wn * (1.0 + asym * 0.5 * sign(qx));
    float d = abs(qx) - side;
    float mid = 1.0 - smoothstep(0.004, 0.010 + 0.006 * (1.0 - t), abs(qx));
    float nv = 5.0 + 5.0 * sh.x + (form > 4.5 ? 2.0 : 0.0);
    float ph = t * nv - abs(qx) / max(hw, 0.1) * nv * 0.55;
    float lines = smoothstep(0.40, 0.5, abs(fract(ph) - 0.5));
    lines *= 1.0 - smoothstep(0.7, 0.95, abs(qx) / max(side, 1e-3));
    lines *= smoothstep(0.04, 0.12, t);
    vein = max(mid * step(t, 0.97), lines * 0.8);
    // The stalk, below the blade.
    d = min(d, sdSeg(q, vec2(0.0), vec2(0.0, 0.05)) - 0.010);
    return d;
  }
  if (form < 1.5) {
    // Palmately lobed: plane, maple.
    vec2 v = q - vec2(0.0, 0.40);
    float r = length(v);
    float th = atan(v.x, v.y);
    float lobe = 0.5 + 0.5 * cos(6.2832 * th);
    float E = (0.30 + 0.55 * sh.x) * (1.0 - smoothstep(2.1, 2.8, abs(th)));
    float R = 0.30 * (1.0 + E * pow(lobe, 1.4 + 1.2 * sh.y)) * (1.0 + asym * 0.25 * sin(th));
    float d = r - R;
    vein = pow(1.0 - abs(sin(3.14159 * th)), 14.0) * (1.0 - smoothstep(0.75, 0.98, r / R)) * step(abs(th), 2.3);
    vein = max(vein, (1.0 - smoothstep(0.0, 0.015, abs(q.x))) * step(q.y, 0.4) * step(0.03, q.y));
    d = min(d, sdSeg(q, vec2(0.0), vec2(0.0, 0.4)) - 0.010);
    return d;
  }
  if (form < 3.5) {
    // A spray of needles.
    float d = 1.0;
    float spread = 0.17 + 0.07 * sh.x;
    for (int k = 0; k < 5; k++) {
      float fk = float(k) - 2.0;
      float phi = fk * spread + (sh.z - 0.5) * 0.1;
      float lk = 0.72 + 0.28 * fract(sin(float(k) * 91.7 + sh.y * 40.0) * 437.5);
      vec2 tip = vec2(sin(phi), cos(phi)) * lk;
      d = min(d, sdSeg(q, vec2(0.0), tip) - 0.016 * (1.0 - 0.55 * t));
    }
    vein = 0.0;
    return d;
  }
  if (form < 4.5) {
    // A compound leaf: leaflets in pairs on a rachis, one at the end.
    float d = sdSeg(q, vec2(0.0), vec2(0.0, 1.0)) - 0.007;
    vein = 1.0 - smoothstep(0.0, 0.012, d);
    for (int i = 0; i < 6; i++) {
      float fi = float(i);
      float ty = 0.15 + 0.125 * fi;
      float sz = 1.0 - 0.07 * fi;
      for (int s = 0; s < 2; s++) {
        float sg = s == 0 ? -1.0 : 1.0;
        vec2 c = vec2(sg * 0.14 * sz, ty + 0.06);
        float de = sdEllipse(q, c, vec2(0.15, 0.058) * sz, -sg * 0.95 + (sh.z - 0.5) * 0.3);
        d = min(d, de);
      }
    }
    d = min(d, sdEllipse(q, vec2(0.0, 0.93), vec2(0.05, 0.12), 1.5708));
    return d;
  }
  // A fan: ginkgo.
  vec2 v = q - vec2(0.0, 0.12);
  float r = length(v);
  float th = atan(v.x, v.y);
  float R = 0.86 * (1.0 + 0.05 * cos(th * 9.0 + sh.x * 5.0)) * (1.0 - 0.30 * exp(-th * th / 0.012));
  float d = max(r - R, (abs(th) - 1.05) * max(r, 0.1));
  vein = pow(abs(cos(th * 8.0)), 24.0) * step(0.06, r) * (1.0 - smoothstep(0.8, 1.0, r / R));
  d = min(d, sdSeg(q, vec2(0.0), vec2(0.0, 0.14)) - 0.010);
  return d;
}

// Coverage of the blade at this fragment, 0 outside the outline and 1 inside.
float treeLeafCoverage(out float vein, out float inner) {
  float d = leafOutline(vTreeB.xy, vTreeB.z, vTreeB.w, vTreeA, vein);
  float aa = max(fwidth(d), 1e-5);
  inner = clamp(-d / 0.08, 0.0, 1.0);
  return clamp(0.5 - d / aa, 0.0, 1.0);
}
`;

const FRAGMENT_SURFACE = /* glsl */ `
if (vTreeKind > 0.5) {
  if (vTreeKind < 1.5) {
    vec3 axis = normalize(vTreeA.xyz);
    float fissure = vTreeA.w;
    vec3 p = vTreeB.xyz;
    float along = dot(p, axis);
    vec3 perp = p - axis * along;
    float n1 = tNoise(perp * 16.0 + axis * along * 2.4);
    float n2 = tNoise(perp * 46.0 + axis * along * 6.0 + 7.1);
    float ridge = abs(n1 - 0.5) * 2.0;
    float groove = smoothstep(0.30 * fissure + 0.04, 0.0, ridge);
    float plate = tNoise(perp * 5.0 + axis * along * 0.8 + 3.3);
    vec3 col = vTreeColour.rgb * (0.78 + 0.34 * n2) * (0.88 + 0.24 * plate);
    col *= 1.0 - groove * (0.30 + 0.45 * fissure);
    diffuseColor.rgb = col;
  } else {
    float vein;
    float inner;
    float cover = treeLeafCoverage(vein, inner);
    vec3 col = vTreeColour.rgb;
    col = mix(col, col * 1.32 + vec3(0.035, 0.045, 0.0), vein * 0.6);
    col *= 0.88 + 0.12 * inner;
    diffuseColor.rgb = col;
    diffuseColor.a = cover;
  }
}
`;

const FRAGMENT_TRANSLUCENCY = /* glsl */ `
#if NUM_DIR_LIGHTS > 0
if (vTreeKind > 1.5) {
  vec3 toLight = directionalLights[0].direction;
  float through = max(0.0, dot(-normal, toLight));
  float seen = max(0.0, dot(normalize(vViewPosition), toLight));
  totalEmissiveRadiance += diffuseColor.rgb * directionalLights[0].color * through * (0.35 + 0.35 * seen) * (1.0 - 0.7 * vTreeColour.a);
}
#endif
`;

/**
 * The per-tree table, shared by every forest on the page: a tree's row is the
 * forest's base plus the tree's own index. It is one texture so that the passes
 * that draw trees (lit, shadow, occlusion) all read the same one.
 */
class TreeTableStore {
  readonly uniform: { value: THREE.DataTexture };
  data: Float32Array;
  capacity: number;
  /** Free runs of rows, as [start, length]. */
  private free: Array<[number, number]> = [];

  constructor(rows = 16) {
    this.capacity = rows * TABLE_PER_ROW;
    this.data = new Float32Array(TABLE_PER_ROW * TABLE_TEXELS * rows * 4);
    const texture = new THREE.DataTexture(
      this.data,
      TABLE_PER_ROW * TABLE_TEXELS,
      rows,
      THREE.RGBAFormat,
      THREE.FloatType,
    );
    texture.minFilter = THREE.NearestFilter;
    texture.magFilter = THREE.NearestFilter;
    texture.generateMipmaps = false;
    texture.needsUpdate = true;
    this.uniform = { value: texture };
    this.free.push([0, this.capacity]);
  }

  /** Reserve `n` consecutive trees; returns the first one's row. */
  allocate(n: number): number {
    for (;;) {
      const at = this.free.findIndex(([, length]) => length >= n);
      if (at >= 0) {
        const [start, length] = this.free[at];
        if (length === n) this.free.splice(at, 1);
        else this.free[at] = [start + n, length - n];
        return start;
      }
      this.grow(Math.max(n, this.capacity));
    }
  }

  release(start: number, n: number): void {
    this.free.push([start, n]);
    this.free.sort((a, b) => a[0] - b[0]);
    const merged: Array<[number, number]> = [];
    for (const run of this.free) {
      const last = merged[merged.length - 1];
      if (last && last[0] + last[1] === run[0]) last[1] += run[1];
      else merged.push([run[0], run[1]]);
    }
    this.free = merged;
  }

  private grow(extra: number): void {
    const rows = Math.ceil((this.capacity + extra) / TABLE_PER_ROW);
    const data = new Float32Array(TABLE_PER_ROW * TABLE_TEXELS * rows * 4);
    data.set(this.data);
    const old = this.capacity;
    this.data = data;
    this.capacity = rows * TABLE_PER_ROW;
    const texture = this.uniform.value;
    texture.dispose();
    texture.image = { data, width: TABLE_PER_ROW * TABLE_TEXELS, height: rows };
    texture.needsUpdate = true;
    this.release(old, this.capacity - old);
  }

  /** Write one tree's row. */
  put(id: number, texels: number[][]): void {
    const base = ((id >> 8) * TABLE_PER_ROW * TABLE_TEXELS + (id & (TABLE_PER_ROW - 1)) * TABLE_TEXELS) * 4;
    texels.forEach((texel, k) => this.data.set(texel, base + k * 4));
    this.uniform.value.needsUpdate = true;
  }
}

export const treeTable = new TreeTableStore();

type Compile = (shader: THREE.WebGLProgramParametersWithUniforms, renderer: THREE.WebGLRenderer) => void;

function patchVertex(source: string): string {
  let out = source.replace("#include <common>", `#include <common>\n${VERTEX_COMMON}`);
  if (out.includes("#include <beginnormal_vertex>")) {
    out = out.replace("#include <beginnormal_vertex>", VERTEX_NORMAL);
  }
  return out.replace("#include <begin_vertex>", VERTEX_POSITION);
}

/**
 * Teach a material to draw trees: any mesh whose geometry carries the tree
 * attributes is raised from them, and any other mesh is drawn as the material
 * always drew it. `mode` says which of the passes this material is.
 */
export function patchTreeMaterial(material: THREE.Material, mode: "lit" | "depth" | "normal"): void {
  const previous = material.onBeforeCompile as Compile | undefined;
  const key = `tree-${mode}`;
  const previousKey = material.customProgramCacheKey?.bind(material);
  material.customProgramCacheKey = () => `${previousKey ? previousKey() : ""}${key}`;
  material.onBeforeCompile = (shader, renderer) => {
    previous?.(shader, renderer);
    shader.uniforms.uTreeTable = treeTable.uniform;
    shader.vertexShader = patchVertex(shader.vertexShader);
    let fragment = shader.fragmentShader;
    fragment = fragment.replace("void main() {", `${FRAGMENT_COMMON}\nvoid main() {`);
    if (mode === "lit") {
      fragment = fragment.replace("#include <color_fragment>", `#include <color_fragment>\n${FRAGMENT_SURFACE}`);
      fragment = fragment.replace("#include <emissivemap_fragment>", `#include <emissivemap_fragment>\n${FRAGMENT_TRANSLUCENCY}`);
    } else if (mode === "depth") {
      fragment = fragment.replace(
        "#include <alphamap_fragment>",
        `#include <alphamap_fragment>\nif (vTreeKind > 1.5) { float tv; float ti; diffuseColor.a = treeLeafCoverage(tv, ti); }`,
      );
    } else {
      fragment = fragment.replace(
        "#include <normal_fragment_begin>",
        `#include <normal_fragment_begin>\nif (vTreeKind > 1.5) { float tv; float ti; if (treeLeafCoverage(tv, ti) < 0.5) discard; }`,
      );
    }
    shader.fragmentShader = fragment;
  };
  material.needsUpdate = true;
}

export interface TreeMaterials {
  wood: THREE.MeshStandardMaterial;
  leaf: THREE.MeshStandardMaterial;
  woodDepth: THREE.MeshDepthMaterial;
  leafDepth: THREE.MeshDepthMaterial;
  dispose(): void;
}

export function createTreeMaterials(): TreeMaterials {
  const wood = new THREE.MeshStandardMaterial({ color: 0xffffff, roughness: 0.95, metalness: 0 });
  patchTreeMaterial(wood, "lit");
  const leaf = new THREE.MeshStandardMaterial({
    color: 0xffffff,
    roughness: 0.62,
    metalness: 0,
    side: THREE.DoubleSide,
    alphaTest: 0.5,
    alphaToCoverage: true,
  });
  patchTreeMaterial(leaf, "lit");
  const woodDepth = new THREE.MeshDepthMaterial({ depthPacking: THREE.RGBADepthPacking });
  patchTreeMaterial(woodDepth, "depth");
  const leafDepth = new THREE.MeshDepthMaterial({ depthPacking: THREE.RGBADepthPacking, alphaTest: 0.5 });
  leafDepth.side = THREE.DoubleSide;
  patchTreeMaterial(leafDepth, "depth");
  return {
    wood,
    leaf,
    woodDepth,
    leafDepth,
    dispose() {
      wood.dispose();
      leaf.dispose();
      woodDepth.dispose();
      leafDepth.dispose();
    },
  };
}
