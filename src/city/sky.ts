/**
 * Sky, sun and atmosphere.
 *
 * A black sky and a wall of white fog are not subtle defects: they mean the scene
 * has no environment, so every surface is lit by ambient alone and every distant
 * object is fogged to the fog colour. Both come from the same omission, and both
 * are fixed the same way — build a real sky, light the scene with it, and make
 * the fog match what the sky actually looks like at the horizon.
 *
 * Ported from the reference project's `sky.js`, which gets three things right that
 * matter here:
 *
 * * The dome is a **shader**, not a colour. A vertical gradient plus a sun disc
 *   plus drifting cloud is what gives glass and metal something to reflect. With
 *   a flat clear colour, every reflective surface in a city renders as a dead
 *   grey and the whole image reads as untextured plastic.
 * * The same dome is **baked to a PMREM environment map**, so ambient light comes
 *   from the actual sky: blue from above, warm from the sun's side, and the
 *   correct colour temperature. This is most of what makes a render look lit
 *   rather than composited.
 * * The fog colour is the dome's own **horizon colour after tone mapping**, not a
 *   guess. Distant buildings then dissolve into the sky instead of into a
 *   different colour, which is what makes fog look like air.
 */

import * as THREE from "three";

/**
 * The one sun direction for the whole scene.
 *
 * It is a constant, not a per-city value, so the sky shader, the environment bake
 * and the shadow direction can never disagree. A renderer that moves the sun
 * without moving the sky produces a lit scene under a different sky, which reads
 * as wrong in a way that is hard to name and easy to see.
 */
export const SUN_DIR = new THREE.Vector3(-0.38, 0.74, -0.46).normalize();

/**
 * Fog colour: the sky's horizon *after* the tone curve, not its linear value.
 *
 * This is the single most important number for atmospheric perspective. Too light
 * and distant geometry turns into white paper; too dark and it turns into grey
 * concrete. It has to be the colour the horizon actually renders as.
 */
export const SKY_FOG = 0xbcc9d8;

const SKY_VERTEX = /* glsl */ `
  varying vec3 vDir;
  void main() {
    vDir = position;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
  }
`;

const SKY_FRAGMENT = /* glsl */ `
  varying vec3 vDir;
  uniform vec3 uSunDir;
  uniform float uTime;

  float hash(vec2 p) { return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453); }
  float vnoise(vec2 p) {
    vec2 i = floor(p), f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), f.x),
               mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), f.x), f.y);
  }
  float fbm(vec2 p) {
    float v = 0.0, a = 0.5;
    for (int i = 0; i < 6; i++) { v += a * vnoise(p); p = p * 2.07 + vec2(19.7, 7.3); a *= 0.5; }
    return v;
  }

  void main() {
    vec3 d = normalize(vDir);
    float y = d.y;

    // Zenith to horizon. The exponent shapes how fast the blue gives way near the
    // horizon, which is where a real sky is palest.
    vec3 zenith  = vec3(0.055, 0.185, 0.480);
    vec3 horizon = vec3(0.420, 0.545, 0.700);
    vec3 col = mix(horizon, zenith, pow(clamp(y, 0.0, 1.0), 0.48));

    float s = max(dot(d, uSunDir), 0.0);
    col += vec3(1.0, 0.86, 0.62) * pow(s, 6.0) * 0.13;   // broad forward scatter
    col += vec3(1.0, 0.95, 0.86) * pow(s, 900.0) * 6.0;  // the disc itself

    if (y > 0.010) {
      // Clouds projected onto the dome, so they converge at the horizon the way
      // real cloud does. Two octaves of FBM: the first warps, the second forms.
      vec2 uv = d.xz / (y + 0.14) * 4.5;
      vec2 drift = vec2(uTime * 0.0022, uTime * 0.0009);
      float warp = fbm(uv * 0.18 + drift);
      float c = fbm(uv * 0.34 + (warp - 0.5) * 2.4 + drift * 1.7);
      float cover = smoothstep(0.50, 0.78, c + 0.12 * pow(1.0 - y, 2.0));
      float fade = smoothstep(0.010, 0.09, y);
      float lit = smoothstep(0.42, 0.92, c);
      vec3 cloud = mix(vec3(0.50, 0.54, 0.61), vec3(0.97, 0.98, 1.02), lit);
      cloud += vec3(0.32, 0.25, 0.14) * pow(s, 3.0);   // sunlit side, warm
      col = mix(col, cloud, cover * fade * 0.92);
    }

    // Below the horizon, hand over to the fog colour so the two never seam.
    col = mix(col, horizon * vec3(1.03, 1.01, 0.98), smoothstep(0.040, -0.09, y));

    gl_FragColor = vec4(col, 1.0);
    #include <tonemapping_fragment>
    #include <colorspace_fragment>
  }
`;

export interface Sky {
  dome: THREE.Mesh;
  material: THREE.ShaderMaterial;
  update(elapsed: number): void;
  follow(camera: THREE.Camera): void;
  dispose(): void;
}

export function createSky(): Sky {
  const material = new THREE.ShaderMaterial({
    uniforms: {
      uSunDir: { value: SUN_DIR.clone() },
      uTime: { value: 0 },
    },
    vertexShader: SKY_VERTEX,
    fragmentShader: SKY_FRAGMENT,
    side: THREE.BackSide,
    depthWrite: false,
    depthTest: false,
    fog: false,
  });
  const dome = new THREE.Mesh(new THREE.SphereGeometry(9000, 48, 28), material);
  dome.name = "sky";
  dome.frustumCulled = false;
  // Drawn before everything and without depth, so it is the background rather
  // than the farthest object.
  dome.renderOrder = -1000;
  return {
    dome,
    material,
    update(elapsed: number) {
      material.uniforms.uTime.value = elapsed;
    },
    follow(camera: THREE.Camera) {
      dome.position.copy(camera.position);
    },
    dispose() {
      dome.geometry.dispose();
      material.dispose();
    },
  };
}

/**
 * Bake the dome into an irradiance map.
 *
 * This is the difference between "lit" and "composited". With a sky-derived
 * environment, a white wall facing the sky is brighter than the same wall facing
 * the ground, a north-facing facade is cooler than a south-facing one, and every
 * reflective surface has something plausible in it. Without it, everything is lit
 * by one flat ambient term and the image looks like untextured plastic no matter
 * how good the textures are.
 */
export function bakeSkyEnvironment(
  renderer: THREE.WebGLRenderer,
  skyMaterial: THREE.ShaderMaterial,
): THREE.Texture {
  const pmrem = new THREE.PMREMGenerator(renderer);
  const scene = new THREE.Scene();
  // A small dome is enough: the environment map is a blur of the sky, so the
  // resolution that matters is the PMREM's, not the source geometry's.
  const clone = new THREE.Mesh(new THREE.SphereGeometry(100, 32, 20), skyMaterial);
  clone.frustumCulled = false;
  scene.add(clone);
  const texture = pmrem.fromScene(scene, 0.04).texture;
  clone.geometry.dispose();
  pmrem.dispose();
  return texture;
}

export interface Lighting {
  sun: THREE.DirectionalLight;
  hemisphere: THREE.HemisphereLight;
  fill: THREE.DirectionalLight;
  /**
   * Refit the shadow frustum around a point of interest.
   *
   * A shadow camera sized for a whole city gives roughly a metre per texel, which
   * is why a city-scale shadow looks like a grey smear. Fitting it to what the
   * camera is actually looking at is the difference between a readable shadow
   * under a kerb and no shadow at all.
   */
  focus(target: THREE.Vector3, radius: number): void;
  dispose(): void;
}

export function createLighting(): Lighting {
  // Sky above, bounced ground below. The ground colour is a desaturated olive
  // because that is what a real city bounces back up, and it is what stops the
  // undersides of balconies and canopies from going black.
  const hemisphere = new THREE.HemisphereLight(0xbfd6f2, 0x3e4034, 0.55);
  // Sunlight through a temperate sky is not white; it is warm, because the
  // atmosphere has taken the blue out of it on the way in.
  const sun = new THREE.DirectionalLight(0xfff0d4, 2.6);
  sun.position.copy(SUN_DIR).multiplyScalar(500);
  sun.castShadow = true;
  sun.shadow.mapSize.set(2048, 2048);
  sun.shadow.bias = -0.0006;
  // A normal bias rather than a depth bias: a depth bias detaches the shadow
  // from the contact point, which is visible as light leaking under every kerb.
  sun.shadow.normalBias = 0.035;
  sun.shadow.camera.near = 1;
  sun.shadow.camera.far = 2000;
  // A weak, cool bounce from the opposite side, standing in for skylight off a
  // pale road. Two directional lights plus an environment map is a standard
  // outdoor approximation and it is what keeps shadowed facades readable.
  const fill = new THREE.DirectionalLight(0xa8c4e8, 0.30);
  fill.position.set(180, 140, 220);

  const focus = (target: THREE.Vector3, radius: number) => {
    const extent = Math.max(40, Math.min(900, radius));
    sun.position.copy(SUN_DIR).multiplyScalar(extent * 2.2).add(target);
    sun.target.position.copy(target);
    sun.target.updateMatrixWorld();
    const camera = sun.shadow.camera;
    camera.left = -extent;
    camera.right = extent;
    camera.top = extent;
    camera.bottom = -extent;
    camera.near = extent * 0.4;
    camera.far = extent * 5.0;
    camera.updateProjectionMatrix();
  };

  return {
    sun,
    hemisphere,
    fill,
    focus,
    dispose() {
      sun.dispose();
      hemisphere.dispose();
      fill.dispose();
      sun.shadow.dispose();
    },
  };
}

/**
 * Atmospheric perspective.
 *
 * **Linear** fog, not exponential, and its range is derived from the scene's
 * extent rather than fixed. An exponential fog with a density tuned by eye is the
 * usual cause of "white dense fog": it does not stop, so the far half of the
 * image is fog colour regardless of how far away it is. Linear fog with a near
 * plane beyond the city and a far plane well beyond it leaves the near field
 * completely clear and only softens the true distance.
 */
export function createFog(extentMetres: number): THREE.Fog {
  const reach = Math.max(200, extentMetres);
  return new THREE.Fog(SKY_FOG, reach * 1.6, reach * 7.0);
}
