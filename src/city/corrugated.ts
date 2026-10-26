/**
 * Profiled steel sheet: the cladding of every shed in an industrial park, and the
 * slats of a roller shutter.
 *
 * The ribs are drawn in the shading normal, not in geometry: a trapezoidal
 * profile every `period` metres along the sheet, which is what makes cladding
 * read as metal and not as a painted box. The UV the wall carries is in metres
 * (`MeshBuilder::wall_uv`), along the wall for vertical ribs (`axis` 0) or up it
 * for the horizontal slats of a shutter (`axis` 1). Once a rib is finer than a
 * pixel the normal is left flat, so a far shed does not shimmer.
 */

import * as THREE from "three";

export function patchCorrugation(
  material: THREE.MeshStandardMaterial,
  period: number,
  depth: number,
  axis: 0 | 1,
): void {
  material.defines = { ...(material.defines ?? {}), USE_UV: "" };
  material.onBeforeCompile = (shader) => {
    shader.uniforms.uRibPeriod = { value: period };
    shader.uniforms.uRibDepth = { value: depth };
    shader.fragmentShader = shader.fragmentShader
      .replace("#include <common>", "#include <common>\nuniform float uRibPeriod;\nuniform float uRibDepth;")
      .replace(
        "#include <normal_fragment_maps>",
        `#include <normal_fragment_maps>
        {
          float t = vUv[${axis}] / uRibPeriod;
          float w = fwidth(t);
          // A trapezoid: flat crown, sloping web, flat pan. The slope of the
          // profile is what tilts the normal.
          float f = fract(t);
          float slope = (smoothstep(0.15, 0.25, f) - smoothstep(0.55, 0.65, f)) * 2.0 - 1.0;
          float web = 1.0 - smoothstep(0.0, 0.08, abs(abs(f - 0.4) - 0.2));
          float tilt = slope * web * uRibDepth * (1.0 - smoothstep(0.25, 0.6, w));
          // The tangent along the UV axis, from screen derivatives.
          vec3 dp1 = dFdx(-vViewPosition);
          vec3 dp2 = dFdy(-vViewPosition);
          vec2 duv1 = dFdx(vUv);
          vec2 duv2 = dFdy(vUv);
          vec3 tangent = ${axis === 0 ? "dp1 * duv2.y - dp2 * duv1.y" : "dp2 * duv1.x - dp1 * duv2.x"};
          float tl = length(tangent);
          if (tl > 1e-6) normal = normalize(normal + (tangent / tl) * tilt);
        }`,
      );
  };
  const key = material.customProgramCacheKey.bind(material);
  material.customProgramCacheKey = () => `${key()}|ribs${axis}`;
  material.needsUpdate = true;
}
