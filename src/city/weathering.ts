/**
 * Weathering for the plain painted surfaces of a building: the bays, balconies,
 * piers and parapets the Rust kit builds as geometry (`cnres.paint` and the
 * like), which carry a flat vertex colour and nothing else.
 *
 * It is the same model as the facade shader's (`facade.ts`): dirt washed down a
 * wall in vertical columns of their own strength, brown-grey so it takes more
 * blue than red off the paint, averaging to its mean once a column is finer
 * than a pixel. Without it the projections stayed showroom-clean against a
 * weathered wall, and the clean paint is most of what a tower's face shows.
 *
 * Positions are the mesh's own, which in a city layer are plan metres.
 */

import * as THREE from "three";

const VERTEX_PARS = /* glsl */ `
varying vec3 vWxPos;
varying vec3 vWxNormal;
`;

const FRAGMENT_PARS = /* glsl */ `
uniform float uWxStrength;
varying vec3 vWxPos;
varying vec3 vWxNormal;
float wxHash(vec2 p) {
  vec3 q = fract(vec3(p.xyx) * vec3(0.1031, 0.1030, 0.0973));
  q += dot(q, q.yxz + 33.33);
  return fract((q.x + q.y) * q.z);
}
float wxNoise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(wxHash(i), wxHash(i + vec2(1.0, 0.0)), f.x),
             mix(wxHash(i + vec2(0.0, 1.0)), wxHash(i + vec2(1.0, 1.0)), f.x), f.y);
}
`;

const FRAGMENT_MAIN = /* glsl */ `
#include <color_fragment>
{
  float wallness = 1.0 - smoothstep(0.35, 0.7, abs(vWxNormal.y));
  vec2 along = vec2(-vWxNormal.z, vWxNormal.x);
  along /= max(length(along), 1e-4);
  float u = dot(vWxPos.xz, along);
  float fw = max(fwidth(u), 1e-4);
  // How weathered this block is: a slow field across the city, so neighbours
  // differ but one building is of a piece.
  float weather = wxNoise(vWxPos.xz * 0.013);
  float colN = wxNoise(vec2(u * 1.9, vWxPos.y * 0.035));
  float colF = wxNoise(vec2(u * 7.0, vWxPos.y * 0.12));
  float rain = smoothstep(0.42, 0.85, colN) * mix(0.55 + 0.45 * colF, 0.75, smoothstep(0.04, 0.2, fw));
  rain = mix(rain, 0.18, smoothstep(0.15, 0.6, fw));
  float patchN = wxNoise(vec2(u * 0.23, vWxPos.y * 0.11)) - 0.5;
  diffuseColor.rgb *= 1.0 + 0.08 * patchN * wallness * uWxStrength;
  float dirt = clamp((0.10 + 0.20 * weather) * rain * uWxStrength, 0.0, 0.5) * wallness;
  diffuseColor.rgb *= vec3(1.0) - vec3(0.85, 0.92, 1.0) * dirt;
}
`;

/** Give `material` weathered paint; `strength` 1 is a rendered wall. */
export function patchWeathering(material: THREE.MeshStandardMaterial, strength = 1): void {
  const previous = material.onBeforeCompile;
  material.onBeforeCompile = (shader, renderer) => {
    previous?.call(material, shader, renderer);
    shader.uniforms.uWxStrength = { value: strength };
    shader.vertexShader = shader.vertexShader
      .replace("#include <common>", `#include <common>\n${VERTEX_PARS}`)
      .replace("#include <begin_vertex>", "#include <begin_vertex>\nvWxPos = position;\nvWxNormal = normal;");
    shader.fragmentShader = shader.fragmentShader
      .replace("#include <common>", `#include <common>\n${FRAGMENT_PARS}`)
      .replace("#include <color_fragment>", FRAGMENT_MAIN);
  };
  const key = material.customProgramCacheKey.bind(material);
  material.customProgramCacheKey = () => `${key()}|weathered`;
  material.needsUpdate = true;
}
