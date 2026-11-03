/**
 * Bark for the grown trees: the prototype carries a smooth vertex colour and no
 * texture, so up close a trunk was a faceted tube with blocky flecks. The
 * fragment adds what bark is: long fissures that run with the trunk, between
 * flat ridges that catch the light, finer cracking across the ridges, and
 * patches of pale lichen and dark damp, all in the tree's own metres so a
 * thick old trunk has coarse plates and a sapling's is fine.
 *
 * The pattern runs round the trunk by arc length and up it by height, and is
 * worked out from the prototype's own position, so it costs no texture and
 * every instance of a prototype shares it (the instance tint still varies).
 * Everything finer than a pixel fades to the mean, so a trunk at 100 m is
 * plain brown, not noise.
 */

import * as THREE from "three";

const VERTEX_PARS = /* glsl */ `
varying vec3 vBarkPos;
`;

const FRAGMENT_PARS = /* glsl */ `
varying vec3 vBarkPos;
float bkHash(vec2 p) {
  vec3 q = fract(vec3(p.xyx) * vec3(0.1031, 0.1030, 0.0973));
  q += dot(q, q.yxz + 33.33);
  return fract((q.x + q.y) * q.z);
}
float bkNoise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(bkHash(i), bkHash(i + vec2(1.0, 0.0)), f.x),
             mix(bkHash(i + vec2(0.0, 1.0)), bkHash(i + vec2(1.0, 1.0)), f.x), f.y);
}
`;

const FRAGMENT_MAIN = /* glsl */ `
#include <color_fragment>
{
  // Round the trunk by arc length, up it by height.
  float r = max(length(vBarkPos.xz), 0.04);
  vec2 q = vec2(atan(vBarkPos.x, vBarkPos.z) * r, vBarkPos.y);
  float fw = max(length(fwidth(q)), 1e-4);
  // Fissures: ridged noise stretched along the trunk, one plate every ~6 cm.
  float plate = bkNoise(q * vec2(16.0, 2.6) + 3.7);
  float ridge = 1.0 - abs(plate * 2.0 - 1.0);
  float fissure = smoothstep(0.55, 0.95, ridge);
  float crack = bkNoise(q * vec2(34.0, 15.0) + 11.0);
  float detail = 1.0 - smoothstep(0.02, 0.12, fw);
  float coarse = 1.0 - smoothstep(0.004, 0.02, fw);
  float shade = 1.0 - 0.42 * fissure * detail - 0.12 * (crack - 0.5) * detail;
  // Pale lichen and damp: slow patches over the whole trunk.
  float patch = bkNoise(q * vec2(2.2, 1.1) + 29.0);
  vec3 lichen = vec3(1.12, 1.14, 1.0);
  vec3 damp = vec3(0.82, 0.80, 0.78);
  vec3 tone = mix(vec3(1.0), lichen, smoothstep(0.62, 0.85, patch) * 0.5);
  tone = mix(tone, damp, smoothstep(0.30, 0.12, patch) * 0.45);
  diffuseColor.rgb *= mix(1.0, shade, 0.55 + 0.45 * coarse) * tone;
}
`;

/** Give `material` fissured, lichened bark. */
export function patchBark<T extends THREE.MeshStandardMaterial>(material: T): T {
  const previous = material.onBeforeCompile;
  material.onBeforeCompile = (shader, renderer) => {
    previous?.call(material, shader, renderer);
    shader.vertexShader = shader.vertexShader
      .replace("#include <common>", `#include <common>\n${VERTEX_PARS}`)
      .replace("#include <begin_vertex>", "#include <begin_vertex>\nvBarkPos = position;");
    shader.fragmentShader = shader.fragmentShader
      .replace("#include <common>", `#include <common>\n${FRAGMENT_PARS}`)
      .replace("#include <color_fragment>", FRAGMENT_MAIN);
  };
  const key = material.customProgramCacheKey.bind(material);
  material.customProgramCacheKey = () => `${key()}|bark`;
  material.needsUpdate = true;
  return material;
}
