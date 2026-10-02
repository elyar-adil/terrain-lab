/**
 * Anti-tiling for the ground materials.
 *
 * Asphalt, paving and grass are small textures repeated every few metres, and a
 * repeated texture reads as one the moment a camera can see more than a few
 * tiles: the same dark dots on every square of lawn, the same grain on every
 * metre of road. The fix has two parts, both in the fragment shader so they cost
 * no texture memory:
 *
 *  - the texture is sampled twice, the second time rotated and at another scale,
 *    and the two are blended by low-frequency noise, so no two neighbouring tiles
 *    are the same picture; the contrast the blend loses is put back;
 *  - each kind of ground then gets large-scale variation of its own that no tile
 *    can carry: asphalt gets wear and patched repairs, grass gets lush and dry
 *    patches and bare earth, paving gets slab-to-slab tone and stains.
 *
 * Texture coordinates here are in tiles (the renderer sets `repeat` to one tile
 * per `tile_metres`), so noise scales are expressed in tiles too.
 */

import * as THREE from "three";

export type GroundKind = "asphalt" | "paving" | "grass";

const COMMON = /* glsl */ `
float atHash(vec2 p) { p = fract(p * vec2(123.34, 456.21)); p += dot(p, p + 45.32); return fract(p.x * p.y); }
float atNoise(vec2 p) {
  vec2 i = floor(p), f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(atHash(i), atHash(i + vec2(1.0, 0.0)), f.x), mix(atHash(i + vec2(0.0, 1.0)), atHash(i + vec2(1.0, 1.0)), f.x), f.y);
}
// Two samples of the same texture, the second rotated and rescaled, blended by
// noise so that neighbouring tiles differ. \`restore\` puts back the contrast the
// blend loses (it peaks where both samples are equally weighted).
vec4 antiTileSample(sampler2D t, vec2 uv, float restore) {
  vec4 a = texture2D(t, uv);
  vec4 b = texture2D(t, mat2(0.8, -0.6, 0.6, 0.8) * uv * 0.61 + vec2(0.37, 0.11));
  float w = smoothstep(0.30, 0.70, atNoise(uv * 0.17 + 3.1));
  vec4 c = mix(a, b, w);
  // The contrast is restored around the texture's own mean (its top mip level),
  // not around mid grey: ground textures are dark, and stretching them about 0.5
  // would push their colours out of range.
  vec4 mean = textureLod(t, vec2(0.5), 20.0);
  float boost = 1.0 + restore * (1.0 - abs(2.0 * w - 1.0));
  return vec4(max(mean.rgb + (c.rgb - mean.rgb) * boost, vec3(0.0)), c.a);
}
`;

const TONE: Record<GroundKind, string> = {
  asphalt: /* glsl */ `
vec3 antiTone(vec2 uv) {
  float lo = atNoise(uv * 0.06 + 9.0);
  float mid = atNoise(uv * 0.35);
  float tone = 0.78 + 0.44 * lo + 0.14 * (mid - 0.5);
  // Patched repairs: straight-edged rectangles of newer, blacker surface.
  vec2 cell = floor(uv * 0.5);
  vec2 f = fract(uv * 0.5);
  float inside = step(0.15, f.x) * step(f.x, 0.85) * step(0.20, f.y) * step(f.y, 0.70);
  tone *= 1.0 - 0.30 * step(0.90, atHash(cell)) * inside;
  // Sealed cracks: a few long, thin, darker lines.
  float crack = 1.0 - smoothstep(0.0, 0.035, abs(atNoise(uv * vec2(0.5, 0.045) + 4.0) - 0.5));
  tone *= 1.0 - 0.22 * crack * step(0.55, atNoise(uv * 0.08));
  return vec3(tone * 0.98, tone, tone * 1.02);
}`,
  grass: /* glsl */ `
vec3 antiTone(vec2 uv) {
  float lo = atNoise(uv * 0.045 + 2.0);
  float mid = atNoise(uv * 0.22);
  vec3 lush = vec3(0.90, 1.07, 0.86);
  vec3 dry = vec3(1.22, 1.05, 0.66);
  vec3 c = mix(lush, dry, smoothstep(0.45, 0.80, lo));
  float bare = smoothstep(0.76, 0.90, atNoise(uv * 0.09 + 7.0));
  c = mix(c, vec3(1.40, 1.02, 0.72), bare * 0.55);
  return c * (0.84 + 0.32 * mid);
}`,
  paving: /* glsl */ `
vec3 antiTone(vec2 uv) {
  // Slab to slab: each 2 m slab is a slightly different batch of concrete.
  float slab = atHash(floor(uv * 2.0));
  float lo = atNoise(uv * 0.07 + 5.0);
  float stain = smoothstep(0.62, 0.85, atNoise(uv * 0.28 + 1.7));
  float tone = (0.90 + 0.18 * slab) * (0.86 + 0.28 * lo) * (1.0 - 0.18 * stain);
  return vec3(tone * 1.01, tone, tone * 0.98);
}`,
};

/**
 * Patch a ground material in place. Idempotent per material: the program is
 * keyed by kind so two kinds never share a compiled shader.
 */
export function applyAntiTiling<T extends THREE.MeshStandardMaterial>(material: T, kind: GroundKind): T {
  const restore = kind === "grass" ? 0.30 : 0.22;
  material.onBeforeCompile = (shader) => {
    shader.fragmentShader = shader.fragmentShader
      .replace("#include <common>", `#include <common>\n${COMMON}\n${TONE[kind]}`)
      .replace(
        "#include <map_fragment>",
        THREE.ShaderChunk.map_fragment
          .replace("texture2D( map, vMapUv )", `antiTileSample( map, vMapUv, ${restore.toFixed(2)} )`)
          .replace("diffuseColor *= sampledDiffuseColor;", "diffuseColor *= sampledDiffuseColor;\n\tdiffuseColor.rgb *= antiTone(vMapUv);"),
      )
      .replace(
        "#include <normal_fragment_maps>",
        THREE.ShaderChunk.normal_fragment_maps.split("texture2D( normalMap, vNormalMapUv )").join("antiTileSample( normalMap, vNormalMapUv, 0.0 )"),
      );
  };
  material.customProgramCacheKey = () => `antitile-${kind}`;
  material.needsUpdate = true;
  return material;
}
