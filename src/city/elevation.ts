/**
 * The Chinese residential elevation material (`cnres/*`).
 *
 * The bake (`crates/city-scene/src/facades/cn_elevation.rs`) writes a surface
 * class into the texture's alpha channel, and this material draws each class
 * differently:
 *
 * * **paint** (alpha 255): the near-white weathering field times the wall's
 *   vertex colour, which is the building's render colour — so one tile serves
 *   a cream tower, a peach slab and an off-white high-rise;
 * * **fixed** (alpha 192): trim, frames and louvres, drawn as baked;
 * * **glass** (alpha 0–120, the window's id): a pane of glass with a room
 *   behind it.  The colour channels carry the texel's position in the opening
 *   (`u` across, `v` up) and the opening's width, so the shader can draw a
 *   curtain drawn from one side, a pair of curtains, a lowered blind or an
 *   empty dark room — a different one in every window, as in a photograph of
 *   any Chinese residential block — and each pane gets a slightly different
 *   tilt so its sky reflection differs from its neighbour's.
 */

import * as THREE from "three";

const GLASS_CHUNK = /* glsl */ `
float elevHash(vec3 p) {
  p = fract(p * vec3(0.1031, 0.1030, 0.0973));
  p += dot(p, p.yxz + 33.33);
  return fract((p.x + p.y) * p.z);
}
`;

const COLOR_CHUNK = /* glsl */ `
float elevGlass = 0.0;
float elevPaint = 0.0;
vec3 elevF0 = vec3(0.04);
vec3 elevEmit = vec3(0.0);
#ifdef USE_MAP
{
  float cls = sampledDiffuseColor.a * 255.0;
  elevPaint = smoothstep(220.0, 250.0, cls);
  elevGlass = 1.0 - smoothstep(128.0, 168.0, cls);
  vec3 tint = vec3(1.0);
  #if defined( USE_COLOR ) || defined( USE_COLOR_ALPHA )
    tint = vColor.rgb;
  #endif
  vec3 solid = mix(sampledDiffuseColor.rgb, sampledDiffuseColor.rgb * tint, elevPaint);

  // Which window this is: its id in the tile, which tile along and up the wall,
  // and which building (its paint colour).
  vec2 tileIx = floor(vMapUv);
  float wid = floor(cls + 0.5);
  vec3 seed = vec3(wid, tileIx.x * 7.13 + tileIx.y * 131.7, dot(tint, vec3(311.0, 173.0, 59.0)));
  float h1 = elevHash(seed);
  float h2 = elevHash(seed + 17.0);
  float h3 = elevHash(seed + 41.0);
  float h4 = elevHash(seed + 73.0);
  float h5 = elevHash(seed + 97.0);
  float u = sampledDiffuseColor.r;
  float v = sampledDiffuseColor.g;
  float widthM = max(sampledDiffuseColor.b * 5.0, 0.4);

#if defined( ELEV_SHOP )
  // A shop: a pale back wall, shelves of goods in a run of colours, panel
  // lights in the ceiling, and it lights itself — in daylight a shop interior
  // is about as bright as the shaded pavement outside it.  One in eight is
  // shut and dark.
  float col = floor(u * widthM / 0.45);
  float row = floor(v * 7.0);
  vec3 goods = vec3(
    elevHash(seed + col * 3.1 + row * 7.7),
    elevHash(seed + col * 5.3 + row * 1.9 + 3.0),
    elevHash(seed + col * 2.7 + row * 4.1 + 9.0));
  goods = mix(vec3(0.38), goods, 0.55);
  vec3 backWall = mix(vec3(0.55, 0.52, 0.46), vec3(0.62, 0.62, 0.60), h3);
  float shelf = step(0.06, v) * step(v, 0.74) * step(0.16, fract(v * 7.0)) * step(0.30, h2);
  vec3 inside = mix(backWall * 0.7, goods, shelf);
  inside = mix(inside, vec3(0.85), step(0.92, v) * step(fract(u * widthM / 1.5 + h4), 0.32));
  float lit = step(0.12, h1);
  elevEmit = inside * 0.42 * lit;
  inside = inside * 0.10 + vec3(0.008);
#elif defined( ELEV_OFFICE )
  // An office floor: a dark plate under a ceiling of lit panels, half the
  // floors behind venetian blinds; an opaque spandrel at the slab.
  vec3 inside = vec3(0.022, 0.024, 0.027) * (0.6 + 0.8 * h1);
  inside = mix(inside, vec3(0.09, 0.09, 0.09), step(0.86, v));
  float lamp = step(fract(u * widthM / 1.2 + h3), 0.24) * step(0.90, v) * step(h4, 0.8);
  inside += lamp * vec3(0.50, 0.52, 0.50);
  if (h2 < 0.45) {
    float f = 0.15 + 0.85 * h5;
    float slat = 0.75 + 0.25 * step(0.4, fract(v * 55.0));
    inside = mix(inside, vec3(0.42, 0.42, 0.40) * slat, step(1.0 - f, v));
  }
  if (sampledDiffuseColor.b > 0.97) inside = vec3(0.016, 0.017, 0.018);
  // Coated glass passes well under half the light.
  inside *= 0.42;
#else
  // The room: dark, a little warmer and lighter towards the ceiling, where the
  // window lights it.
  vec3 room = vec3(0.034, 0.031, 0.028) * (0.55 + 0.9 * h1) * mix(0.7, 1.25, v);
  vec3 inside = room;
  float folds = 0.86 + 0.14 * sin(u * widthM / 0.11 * 6.2831);
  if (h2 < 0.32) {
    // No curtain: the room, and now and then a lit lamp or a pale wall.
    inside = room * (h3 < 0.12 ? 3.5 : 1.0);
  } else if (h2 < 0.70) {
    // Sheer white curtains, drawn from one side.
    float f = 0.25 + 0.75 * h3;
    float covered = h4 < 0.5 ? step(u, f) : step(1.0 - f, u);
    inside = mix(room, vec3(0.50, 0.49, 0.45) * folds, covered * 0.82);
  } else if (h2 < 0.90) {
    // A pair of heavy curtains, partly open.
    vec3 cloth = h5 < 0.25 ? vec3(0.36, 0.28, 0.18)
      : h5 < 0.45 ? vec3(0.18, 0.21, 0.27)
      : h5 < 0.70 ? vec3(0.46, 0.42, 0.34)
      : h5 < 0.85 ? vec3(0.20, 0.13, 0.09)
      : vec3(0.27, 0.31, 0.25);
    float f = 0.30 + 0.70 * h3;
    float covered = step(u, f * 0.5) + step(1.0 - f * 0.5, u);
    inside = mix(room, cloth * folds, clamp(covered, 0.0, 1.0));
  } else {
    // A roller blind, lowered part way.
    float f = 0.2 + 0.8 * h3;
    inside = mix(room, vec3(0.46, 0.44, 0.40), step(1.0 - f, v));
  }
  // Two panes of float glass take a third of the light each way.
  inside *= 0.62;
#endif
  diffuseColor.rgb = diffuse * mix(solid, inside, elevGlass);
#ifdef ELEV_TINT_GLASS
  // A curtain wall's vertex colour is its coating, which is what it reflects.
  elevF0 = mix(vec3(0.04), tint, 0.30);
#elif defined( ELEV_OFFICE )
  elevF0 = vec3(0.10, 0.12, 0.13);
#endif
}
#endif
diffuseColor.a = 1.0;
`;

const SPECULAR_CHUNK = /* glsl */ `
#include <lights_physical_fragment>
material.specularColor = mix(material.specularColor, elevF0, elevGlass);
material.specularColorBlended = mix(material.specularColorBlended, elevF0, elevGlass);
`;

const EMISSIVE_CHUNK = /* glsl */ `
#include <emissivemap_fragment>
totalEmissiveRadiance += elevEmit * elevGlass;
`;

const ROUGHNESS_CHUNK = /* glsl */ `
#include <roughnessmap_fragment>
roughnessFactor = mix(mix(0.62, 0.90, elevPaint), 0.045, elevGlass);
`;

const NORMAL_CHUNK = /* glsl */ `
#include <normal_fragment_maps>
#ifdef USE_MAP
if (elevGlass > 0.0) {
  // Every pane is glazed a fraction of a degree off true, which is what makes
  // the sky in one window a different value from the sky in the next.
  float panes = max(1.0, floor(max(sampledDiffuseColor.b * 5.0, 0.4) / 0.85 + 0.5));
  float pane = floor(sampledDiffuseColor.r * panes);
  vec3 ps = vec3(floor(sampledDiffuseColor.a * 255.0 + 0.5), pane, floor(vMapUv.x) * 3.1 + floor(vMapUv.y) * 17.3);
  vec2 tilt = vec2(elevHash(ps + 5.0), elevHash(ps + 11.0)) - 0.5;
  normal = normalize(normal + elevGlass * vec3(tilt * 0.07, 0.0));
}
#endif
`;

/**
 * Which glass an elevation has: the float glass of a flat, the coated glass
 * of an office with an office floor behind it, a curtain wall whose vertex
 * colour is its coating, or a shop window with a lit shop behind it.
 */
export type ElevationGlass = "residential" | "office" | "curtain" | "shop";

/** Teach a `MeshStandardMaterial` with a `cnres/*` or `cncom/*` map the surface classes. */
export function patchElevationMaterial(
  material: THREE.MeshStandardMaterial,
  glass: ElevationGlass = "residential",
): void {
  const defines: Record<string, string> = {};
  if (glass === "office" || glass === "curtain") defines.ELEV_OFFICE = "";
  if (glass === "shop") defines.ELEV_SHOP = "";
  if (glass === "curtain") defines.ELEV_TINT_GLASS = "";
  material.defines = { ...(material.defines ?? {}), ...defines };
  material.onBeforeCompile = (shader) => {
    shader.fragmentShader = shader.fragmentShader
      .replace("void main() {", `${GLASS_CHUNK}\nvoid main() {`)
      .replace("#include <color_fragment>", COLOR_CHUNK)
      .replace("#include <roughnessmap_fragment>", ROUGHNESS_CHUNK)
      .replace("#include <lights_physical_fragment>", SPECULAR_CHUNK)
      .replace("#include <emissivemap_fragment>", EMISSIVE_CHUNK)
      .replace("#include <normal_fragment_maps>", NORMAL_CHUNK);
  };
  material.customProgramCacheKey = () => `cn-elevation-${glass}`;
  material.needsUpdate = true;
}
