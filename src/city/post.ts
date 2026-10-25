/**
 * The post-processing chain: HDR render target with 4x MSAA, ground-truth
 * ambient occlusion, a restrained bloom, and one final pass that applies the
 * tone curve and the sRGB conversion.
 *
 * Ported from the reference project's `main.js`. The single most important
 * pass is GTAO: contact shadows where buildings meet the ground, where kerbs
 * meet the road, where trees meet the sky are not in the model and not in any
 * shadow map — without occlusion every object reads as floating, which is the
 * main thing that separates a render from a photograph.
 *
 * The chain is skipped entirely in the audit's cheap mode: on a software
 * rasteriser the extra passes multiply an already slow frame, and the audit
 * cares about geometry, materials and tone, all of which the direct path
 * still exercises.
 */

import * as THREE from "three";
import { EffectComposer } from "three/examples/jsm/postprocessing/EffectComposer.js";
import { GTAOPass } from "three/examples/jsm/postprocessing/GTAOPass.js";
import { OutputPass } from "three/examples/jsm/postprocessing/OutputPass.js";
import { RenderPass } from "three/examples/jsm/postprocessing/RenderPass.js";
import { ShaderPass } from "three/examples/jsm/postprocessing/ShaderPass.js";
import { UnrealBloomPass } from "three/examples/jsm/postprocessing/UnrealBloomPass.js";

import { patchTreeMaterial } from "../trees/shaders";

export interface Post {
  composer: EffectComposer;
  gtao: GTAOPass;
  setSize(width: number, height: number): void;
  render(): void;
  dispose(): void;
}

export interface PostOptions {
  camera: THREE.Camera;
  /**
   * A leaf-card alpha texture, used to punch the transparent corners of
   * foliage cards out of GTAO's normal/depth G-buffer. Without it the occluder
   * pass sees every card as a solid quad and each canopy self-occludes into a
   * black blob — the one visible artefact the raw pass numbers do not show.
   */
  foliageMask?: THREE.Texture | null;
}

export function createPost(
  renderer: THREE.WebGLRenderer,
  scene: THREE.Scene,
  options: PostOptions,
): Post {
  const size = new THREE.Vector2();
  renderer.getSize(size);
  const pixelRatio = renderer.getPixelRatio();

  // HDR half-float target with multisampling: the tone curve is applied once,
  // at the end, to linear data — an LDR chain would band the sky and clip the
  // sunlit paint before bloom ever saw it.
  const composer = new EffectComposer(
    renderer,
    new THREE.WebGLRenderTarget(Math.max(2, size.x), Math.max(2, size.y), {
      type: THREE.HalfFloatType,
      samples: 4,
    }),
  );
  composer.setPixelRatio(pixelRatio);
  composer.setSize(size.x, size.y);

  const renderPass = new RenderPass(scene, options.camera);
  const gtao = new GTAOPass(scene, options.camera, size.x, size.y);

  if (options.foliageMask) {
    // The G-buffer pass redraws the whole scene with one override
    // `MeshNormalMaterial`, so per-object leaf knowledge has to travel through
    // geometry: every foliage prototype carries an `aLeafCard` vertex
    // attribute (written by `buildCityScene`), and fragments whose card
    // texture is transparent are discarded from the G-buffer only.
    const material = gtao.normalMaterial;
    const mask = options.foliageMask;
    material.onBeforeCompile = (shader) => {
      shader.uniforms.foliageMask = { value: mask };
      shader.vertexShader = shader.vertexShader
        .replace(
          "#include <common>",
          "#include <common>\nattribute float aLeafCard;\nvarying vec2 vCardUv;\nvarying float vLeafCard;",
        )
        .replace(
          "#include <begin_vertex>",
          "#include <begin_vertex>\nvCardUv = uv; vLeafCard = aLeafCard;",
        );
      shader.fragmentShader = shader.fragmentShader
        .replace(
          "void main() {",
          "uniform sampler2D foliageMask;\nvarying vec2 vCardUv;\nvarying float vLeafCard;\nvoid main() {",
        )
        .replace(
          "#include <normal_fragment_begin>",
          "#include <normal_fragment_begin>\nif (vLeafCard > 0.5 && texture2D(foliageMask, vCardUv).a < 0.45) discard;",
        );
    };
  }

  // The occlusion pass draws the whole scene with one override material, so the
  // individually grown trees (which are raised in the vertex shader, not stored
  // as meshes) have to be taught to that material as well.
  patchTreeMaterial(gtao.normalMaterial, "normal");

  // Tuned on the reference project's street views: a radius under a metre so
  // occlusion is contact-scale rather than a global darkening, and a denoise
  // radius small enough to keep kerb lines crisp.
  gtao.output = GTAOPass.OUTPUT.Default;
  gtao.blendIntensity = 0.85;
  gtao.updateGtaoMaterial({
    radius: 0.8,
    distanceExponent: 1.1,
    thickness: 1.4,
    scale: 1.0,
    samples: 16,
  });
  gtao.updatePdMaterial({
    lumaPhi: 10,
    depthPhi: 2,
    normalPhi: 3,
    radius: 4,
    rings: 2,
    samples: 8,
  });

  // Bloom only catches what is brighter than the sky after the HDR clamp: the
  // sun disc, backlit signage, signal lenses. A higher threshold or strength
  // turns the whole skyline into fog.
  const bloom = new UnrealBloomPass(
    new THREE.Vector2(size.x / 2, size.y / 2),
    0.14,
    0.5,
    1.05,
  );

  composer.addPass(renderPass);
  composer.addPass(gtao);
  composer.addPass(sanitizePass());
  composer.addPass(bloom);
  composer.addPass(new OutputPass());

  return {
    composer,
    gtao,
    setSize(width: number, height: number) {
      composer.setSize(width, height);
      gtao.setSize(width, height);
    },
    render() {
      composer.render();
    },
    dispose() {
      gtao.dispose();
      bloom.dispose();
      composer.dispose();
    },
  };
}

/**
 * Replace non-finite pixels with black and clamp the rest, ahead of bloom.
 *
 * Bloom blurs the frame down a mip chain, so a single NaN or infinite pixel from
 * any material spreads to every pixel and the frame comes out solid white. The
 * clamp is far above anything a lit surface reaches; it only stops a stray spike.
 */
export function sanitizePass(): ShaderPass {
  return new ShaderPass({
    name: "SanitizeShader",
    uniforms: { tDiffuse: { value: null } },
    vertexShader: /* glsl */ `
      varying vec2 vUv;
      void main() {
        vUv = uv;
        gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
      }`,
    fragmentShader: /* glsl */ `
      uniform sampler2D tDiffuse;
      varying vec2 vUv;
      bool bad(float v) { return isnan(v) || isinf(v) || abs(v) > 1.0e4; }
      void main() {
        vec4 c = texture2D(tDiffuse, vUv);
        if (bad(c.r) || bad(c.g) || bad(c.b) || bad(c.a)) c = vec4(0.0, 0.0, 0.0, 1.0);
        gl_FragColor = vec4(min(c.rgb, vec3(64.0)), c.a);
      }`,
  });
}
