/**
 * Grass as a surface with a pile, not a green plane.
 *
 * Two parts, both driven by position in the world (so a lawn is continuous across
 * every mesh it is cut into, and nothing repeats):
 *
 * - `applyGrassFur` teaches any lit material to be grass in the fragment shader:
 *   blades, clumps and patches at three scales, each faded out as it gets smaller
 *   than a pixel (so a far lawn averages to its mean colour instead of shimmering),
 *   a tilt of the normal that lights tufts differently, and the soft pale rim and
 *   back-lit glow that a pile of thin blades has when seen across the grain.
 * - `GrassShells` adds real volume near the camera: the ground mesh is drawn again
 *   as a stack of thin shells lifted along the normal, each cut to the blades tall
 *   enough to reach it. One instanced draw per ground mesh, and only for those near
 *   the camera.
 */

import * as THREE from "three";

type Compile = (shader: THREE.WebGLProgramParametersWithUniforms, renderer: THREE.WebGLRenderer) => void;

const COMMON = /* glsl */ `
varying vec3 vFurRel;
// Cell hashes are taken on wrapped coordinates: a lawn is thousands of metres from the
// origin and a hash of a million loses every bit it has. The noise is periodic over a
// few hundred cells, which nobody can see.
float furHash(vec2 p) {
  p = mod(p, vec2(251.0, 241.0));
  vec3 q = fract(vec3(p.xyx) * 0.1031);
  q += dot(q, q.yzx + 33.33);
  return fract((q.x + q.y) * q.z);
}
float furNoise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(furHash(i), furHash(i + vec2(1, 0)), f.x), mix(furHash(i + vec2(0, 1)), furHash(i + vec2(1, 1)), f.x), f.y);
}
`;

const FRAGMENT_COMMON = /* glsl */ `
${COMMON}
uniform vec3 uFurTip;
uniform vec3 uFurDry;
`;

/** The colour and lighting of the pile, inserted where the diffuse colour is final. */
const FRAGMENT_COLOUR = /* glsl */ `
{
  vec2 p = (vFurRel + cameraPosition).xz;
  // How big one pixel is on the ground, metres: detail below it must not be drawn.
  // The footprint of one pixel on the ground, metres. At a grazing angle it is long
  // one way and short the other; a layer must be gone when its feature is smaller
  // than the *long* side, or it smears into streaks along the view direction.
  vec2 fdx = dFdx(p);
  vec2 fdy = dFdy(p);
  float px = max(length(fdx), length(fdy));
  float blade = furNoise(p * 70.0);
  float blade2 = furNoise(p * 31.0 + 17.0);
  float clump = furNoise(p * 9.0 + 3.0);
  float lushNoise = furNoise(p * 0.55 + 11.0);
  float macro = furNoise(p * 0.07 + 5.0);
  float fBlade = 1.0 - smoothstep(0.003, 0.014, px);
  float fClump = 1.0 - smoothstep(0.02, 0.10, px);
  float fPatch = 1.0 - smoothstep(0.25, 1.0, px);
  float fMacro = 1.0 - smoothstep(3.0, 10.0, px);
  float fine = mix(0.5, 0.5 * (blade + blade2), fBlade);
  float lum = 0.62 + 0.76 * fine;
  lum *= mix(1.0, 0.78 + 0.44 * clump, fClump);
  lum *= mix(1.0, 0.86 + 0.28 * lushNoise, fPatch);
  // Lush to dry across the lawn, a metres-wide drift.
  vec3 col = diffuseColor.rgb * lum;
  col = mix(col, col * uFurDry, smoothstep(0.35, 0.8, macro) * 0.55 * fMacro);
  // Sun-bleached tips where the blade is tall.
  col = mix(col, col * uFurTip, smoothstep(0.55, 1.0, fine) * 0.5);
  diffuseColor.rgb = col;
}
`;

/** After the normal is known: tilt it by the clumps, and add the pile's own light. */
const FRAGMENT_LIGHT = /* glsl */ `
{
  vec2 p = (vFurRel + cameraPosition).xz;
  float px = max(length(dFdx(p)), length(dFdy(p)));
  float fc = 1.0 - smoothstep(0.02, 0.10, px);
  float e = 0.12;
  float c0 = furNoise(p * 9.0 + 3.0);
  vec2 g = vec2(furNoise(p * 9.0 + 3.0 + vec2(e, 0.0)) - c0, furNoise(p * 9.0 + 3.0 + vec2(0.0, e)) - c0) / e;
  vec3 tilt = vec3(-g.x, 0.0, -g.y) * 0.16 * fc;
  normal = normalize(normal + normalize(mat3(viewMatrix) * tilt + 1e-5) * length(tilt));
}
`;

const FRAGMENT_EMISSIVE = /* glsl */ `
#if NUM_DIR_LIGHTS > 0
{
  vec3 L = directionalLights[0].direction;
  vec3 V = normalize(vViewPosition);
  float ndv = clamp(dot(normalize(vNormal), V), 0.0, 1.0);
  // A pile of thin blades is lighter across the grain...
  float rim = pow(1.0 - ndv, 3.0);
  // ...and glows when the sun is behind it.
  float back = pow(max(dot(-V, L), 0.0), 3.0) * 0.5 + 0.25;
  vec3 pile = diffuseColor.rgb * directionalLights[0].color;
  totalEmissiveRadiance += pile * (rim * 0.55 * back + 0.05);
}
#endif
`;

function patchVertexPosition(source: string): string {
  return source
    .replace("#include <common>", `#include <common>\nvarying vec3 vFurRel;`)
    .replace(
      "#include <project_vertex>",
      `#include <project_vertex>\n#ifdef USE_INSTANCING\n  vFurRel = (modelMatrix * instanceMatrix * vec4(transformed, 1.0)).xyz - cameraPosition;\n#else\n  vFurRel = (modelMatrix * vec4(transformed, 1.0)).xyz - cameraPosition;\n#endif`,
    );
}

/** Make a lit material grass. Chains with whatever patch it already carries. */
export function applyGrassFur(material: THREE.MeshStandardMaterial, tip = 0xd8cf8a, dry = 0xb7ad6a): THREE.MeshStandardMaterial {
  const previous = material.onBeforeCompile as Compile | undefined;
  const previousKey = material.customProgramCacheKey?.bind(material);
  material.customProgramCacheKey = () => `${previousKey ? previousKey() : ""}|fur`;
  const tipColour = new THREE.Color(tip);
  const dryColour = new THREE.Color(dry);
  // The tint multiplies the base colour: normalise so "no change" is white.
  tipColour.multiplyScalar(1 / 0.85);
  dryColour.multiplyScalar(1 / 0.72);
  material.onBeforeCompile = (shader, renderer) => {
    previous?.(shader, renderer);
    shader.uniforms.uFurTip = { value: tipColour };
    shader.uniforms.uFurDry = { value: dryColour };
    shader.vertexShader = patchVertexPosition(shader.vertexShader);
    shader.fragmentShader = shader.fragmentShader
      .replace("void main() {", `${FRAGMENT_COMMON}\nvoid main() {`)
      .replace("#include <color_fragment>", `#include <color_fragment>\n${FRAGMENT_COLOUR}`)
      .replace("#include <normal_fragment_maps>", `#include <normal_fragment_maps>\n${FRAGMENT_LIGHT}`)
      .replace("#include <emissivemap_fragment>", `#include <emissivemap_fragment>\n${FRAGMENT_EMISSIVE}`);
  };
  material.roughness = 1;
  material.needsUpdate = true;
  return material;
}

// --- shells -------------------------------------------------------------------

const SHELL_COUNT = 16;
const SHELL_HEIGHT_M = 0.11;

function shellMaterial(_base: THREE.MeshStandardMaterial): THREE.MeshStandardMaterial {
  const material = new THREE.MeshStandardMaterial({
    // No texture: blades are coloured by their own height and cell, so the shells
    // do not depend on a mesh having metre UVs (the open-ground carpet has none).
    color: 0x6b7c47,
    roughness: 1,
    metalness: 0,
  });
  material.customProgramCacheKey = () => "grass-shell";
  material.onBeforeCompile = (shader) => {
    shader.uniforms.uShells = { value: SHELL_COUNT };
    shader.uniforms.uShellHeight = { value: SHELL_HEIGHT_M };
    shader.vertexShader = shader.vertexShader
      .replace(
        "#include <common>",
        `#include <common>
varying vec3 vFurRel;
varying float vShell;
uniform float uShells;
uniform float uShellHeight;`,
      )
      .replace(
        "#include <begin_vertex>",
        `#include <begin_vertex>
float shell = float(gl_InstanceID + 1) / uShells;
vShell = shell;
transformed += normalize(normal) * (uShellHeight * shell);`,
      )
      .replace(
        "#include <project_vertex>",
        `#include <project_vertex>
vFurRel = (modelMatrix * vec4(transformed, 1.0)).xyz - cameraPosition;`,
      );
    shader.fragmentShader = shader.fragmentShader
      .replace(
        "void main() {",
        `${COMMON}
varying float vShell;
void main() {
  {
    vec2 p = (vFurRel + cameraPosition).xz;
    float px = max(length(dFdx(p)), length(dFdy(p)));
    // Blades a centimetre across, one per cell; the cell's own height decides how
    // many shells it reaches, and its width tapers toward the tip.
    float density = 90.0;
    vec2 cell = floor(p * density);
    vec2 local = fract(p * density) - 0.5;
    float h = 0.35 + 0.65 * furHash(cell);
    vec2 offset = (vec2(furHash(cell + 7.0), furHash(cell + 13.0)) - 0.5) * 0.45;
    float r = length(local - offset);
    float width = 0.46 * (1.0 - vShell / max(h, 0.05));
    // Fade to nothing as blades get thinner than a pixel: the base lawn takes over.
    float far = smoothstep(0.012, 0.05, px);
    // The blades thin out with distance and are gone at the edge of the carpet, so the
    // pile melts into the lawn the surface shader draws beyond it.
    float edge = smoothstep(14.0, 34.0, length(vFurRel.xz));
    if (vShell > h * (1.0 - edge) || r > width || far > 0.97) discard;
  }`,
      )
      .replace(
        "#include <color_fragment>",
        `#include <color_fragment>
{
  float h = 0.35 + 0.65 * furHash(floor((vFurRel + cameraPosition).xz * 90.0));
  float t = vShell / max(h, 0.05);
  // Dark and damp at the root, bleached at the tip, one blade to the next.
  float tone = 0.55 + 0.9 * furHash(floor((vFurRel + cameraPosition).xz * 90.0) + 3.0);
  diffuseColor.rgb *= mix(0.62, 1.25, vShell) * (0.72 + 0.4 * tone);
  diffuseColor.rgb = mix(diffuseColor.rgb, diffuseColor.rgb * vec3(1.25, 1.18, 0.75), t * 0.45);
}`,
      );
  };
  return material;
}

interface Tracked {
  object: THREE.Mesh;
  box: THREE.Box3;
  shells: THREE.InstancedMesh | null;
}

/** Real blades near the camera, on every grass mesh within reach of it. */
export class GrassShells {
  private readonly tracked: Tracked[] = [];
  private readonly material: THREE.MeshStandardMaterial;
  private readonly identity = new THREE.Matrix4();

  constructor(
    grass: THREE.Mesh[],
    base: THREE.MeshStandardMaterial,
    private readonly reachM = 36,
  ) {
    this.material = shellMaterial(base);
    for (const object of grass) {
      const box = object.geometry.boundingBox ?? (object.geometry.computeBoundingBox(), object.geometry.boundingBox!);
      this.tracked.push({ object, box, shells: null });
    }
  }

  private carpetMesh: THREE.InstancedMesh | null = null;

  /**
   * A square of blades that follows the camera over open ground, where there is no
   * lawn mesh to put shells on: the ground itself is one enormous quad.
   */
  carpet(y: number, parent: THREE.Object3D, halfM = 40): void {
    const geometry = new THREE.PlaneGeometry(halfM * 2, halfM * 2, 1, 1);
    geometry.rotateX(-Math.PI / 2);
    const mesh = new THREE.InstancedMesh(geometry, this.material, SHELL_COUNT);
    for (let i = 0; i < SHELL_COUNT; i += 1) mesh.setMatrixAt(i, this.identity);
    mesh.instanceMatrix.needsUpdate = true;
    mesh.position.y = y;
    mesh.castShadow = false;
    mesh.receiveShadow = true;
    mesh.frustumCulled = false;
    parent.add(mesh);
    this.carpetMesh = mesh;
  }

  /** @param camera the camera in the grass meshes' own frame (city-local metres) */
  update(camera: THREE.Vector3): void {
    if (this.carpetMesh) {
      this.carpetMesh.position.x = Math.round(camera.x);
      this.carpetMesh.position.z = Math.round(camera.z);
    }
    for (const entry of this.tracked) {
      const near = entry.object.visible && entry.box.distanceToPoint(camera) < this.reachM;
      if (near && !entry.shells) {
        const shells = new THREE.InstancedMesh(entry.object.geometry, this.material, SHELL_COUNT);
        for (let i = 0; i < SHELL_COUNT; i += 1) shells.setMatrixAt(i, this.identity);
        shells.instanceMatrix.needsUpdate = true;
        shells.castShadow = false;
        shells.receiveShadow = true;
        shells.frustumCulled = false;
        shells.position.copy(entry.object.position);
        shells.quaternion.copy(entry.object.quaternion);
        shells.scale.copy(entry.object.scale);
        entry.object.parent?.add(shells);
        entry.shells = shells;
      }
      if (entry.shells) entry.shells.visible = near;
    }
  }

  dispose(): void {
    if (this.carpetMesh) {
      this.carpetMesh.removeFromParent();
      this.carpetMesh.geometry.dispose();
      this.carpetMesh.dispose();
    }
    for (const entry of this.tracked) {
      if (entry.shells) {
        entry.shells.removeFromParent();
        entry.shells.dispose();
      }
    }
    this.material.dispose();
  }
}
