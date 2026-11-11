/**
 * What the lens and the sensor add after the scene is exposed.
 *
 * A render is perfectly even from corner to corner, has no noise in its shadows
 * and focuses every wavelength on the same pixel; no photograph is like that,
 * and the eye has learned the difference. This pass is the last one, in display
 * space, and each effect is kept at the level of a good lens, where it is felt
 * rather than seen: light falls off toward the corners, colours part by a
 * fraction of a pixel there, and a fine grain sits over the image, stronger in
 * the mid-tones than in the highlights.
 */
import { ShaderPass } from "three/examples/jsm/postprocessing/ShaderPass.js";

export interface LensPass {
  pass: ShaderPass;
  /** Advance the grain; call once a frame with seconds. */
  update(time: number): void;
}

export function createLensPass(): LensPass {
  const pass = new ShaderPass({
    name: "LensShader",
    uniforms: { tDiffuse: { value: null }, uTime: { value: 0 } },
    vertexShader: /* glsl */ `
      varying vec2 vUv;
      void main() {
        vUv = uv;
        gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
      }`,
    fragmentShader: /* glsl */ `
      uniform sampler2D tDiffuse;
      uniform float uTime;
      varying vec2 vUv;
      float grainHash(vec2 p) {
        vec3 q = fract(vec3(p.xyx) * 0.1031);
        q += dot(q, q.yzx + 33.33);
        return fract((q.x + q.y) * q.z);
      }
      void main() {
        vec2 fromCentre = vUv - 0.5;
        float r2 = dot(fromCentre, fromCentre);
        // Lateral chromatic aberration: red and blue magnified a hair differently.
        vec2 spread = fromCentre * r2 * 0.0045;
        vec3 colour = vec3(
          texture2D(tDiffuse, vUv + spread).r,
          texture2D(tDiffuse, vUv).g,
          texture2D(tDiffuse, vUv - spread).b
        );
        // The print: a film's shoulder and toe on top of the tone curve (deeper
        // shadows, highlights that keep rolling), and a little more colour.
        colour = clamp(colour, 0.0, 1.0);
        vec3 curved = colour * colour * (3.0 - 2.0 * colour);
        colour = mix(colour, curved, 0.26);
        float grey = dot(colour, vec3(0.2126, 0.7152, 0.0722));
        colour = mix(vec3(grey), colour, 1.10);
        // Natural vignetting, under half a stop at the corners.
        colour *= mix(1.0, 0.74, smoothstep(0.08, 0.62, r2));
        // Grain, re-drawn each frame; least in the highlights.
        float luma = dot(colour, vec3(0.2126, 0.7152, 0.0722));
        float grain = grainHash(gl_FragCoord.xy + fract(uTime) * 517.0) - 0.5;
        colour += grain * 0.030 * (1.0 - 0.7 * luma);
        gl_FragColor = vec4(colour, 1.0);
      }`,
  });
  return {
    pass,
    update(time: number) {
      pass.uniforms.uTime.value = time;
    },
  };
}
