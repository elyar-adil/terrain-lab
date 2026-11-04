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
float wxRain = 0.0;
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
  // Repainted patches (a rectangle of newer coat, a few % lighter and warmer),
  // pale lime efflorescence and damp darkening low on the wall.
  vec2 rpC = vec2(u / 4.5, vWxPos.y / 6.0);
  float rpH = wxHash(floor(rpC) + 11.0);
  vec2 rpF = fract(rpC);
  float rpIn = step(0.08, rpF.x) * step(rpF.x, 0.92) * step(0.1, rpF.y) * step(rpF.y, 0.9);
  float rp = step(0.84, rpH) * rpIn * wallness * (1.0 - 0.6 * smoothstep(0.15, 0.6, fw)) * uWxStrength;
  diffuseColor.rgb *= mix(vec3(1.0), vec3(1.07, 1.05, 0.99) + (rpH - 0.92) * vec3(0.4, 0.2, -0.2), rp);
  float lowZone = 1.0 - smoothstep(0.4, 2.2, vWxPos.y);
  float bloom = smoothstep(0.55, 0.8, wxNoise(vec2(u * 2.3, vWxPos.y * 3.1 + 7.0))) * lowZone * (0.4 + 0.6 * weather);
  diffuseColor.rgb = mix(diffuseColor.rgb, diffuseColor.rgb * 0.45 + vec3(0.5, 0.5, 0.47), bloom * 0.4 * wallness * uWxStrength * (1.0 - 0.5 * smoothstep(0.15, 0.6, fw)));
  diffuseColor.rgb *= 1.0 - 0.10 * lowZone * wallness * uWxStrength * smoothstep(0.35, 0.65, wxNoise(vec2(u * 0.8, 3.0)));
  wxRain = rain;
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
      .replace("#include <color_fragment>", FRAGMENT_MAIN)
      .replace("#include <roughnessmap_fragment>", "#include <roughnessmap_fragment>\nroughnessFactor = clamp(roughnessFactor - 0.10 * wxRain * uWxStrength, 0.3, 1.0);");
  };
  const key = material.customProgramCacheKey.bind(material);
  material.customProgramCacheKey = () => `${key()}|weathered`;
  material.needsUpdate = true;
}
