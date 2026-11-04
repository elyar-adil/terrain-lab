/**
 * Tree LOD harness: the same species drawn two ways, side by side, at one
 * distance. For each species the left tree is the detail prototype the city
 * instances (bark, leaf cards, canopy mass), the right one the far-form crown
 * (`treeCrown.ts`) the mass layer draws. Both stand on one lit ground under the
 * city's sky and sun, and the camera is telephoto: it sits `d` metres away and
 * narrows its field of view so the row fills the picture, which gives the
 * shading and size relationship of that distance at a size one can read.
 *
 *   ?d=300                   camera distance, metres
 *   ?species=huai-shu,...    which species (default: a street mix)
 *   ?scene=/def.json         the scene the detail prototypes come from
 *   ?eye=1.7                 camera height; default a third of a tree
 *   ?w=1400&h=520            picture size
 *   ?mode=both|near|far      draw both rows, or only one
 */

import * as THREE from "three";

import { buildCityScene, createMaterials, type CityScene } from "./city/cityScene";
import { createFog, createLighting, createSky } from "./city/sky";
import { drawCrown, speciesSize } from "./city/treeCrown";

declare global {
  interface Window {
    __RENDER_READY__?: boolean;
    __CITY_DIAGNOSTICS__?: Record<string, unknown>;
  }
}

const params = new URLSearchParams(window.location.search);
const num = (k: string, f: number) => (Number.isFinite(Number(params.get(k))) && params.get(k) !== null ? Number(params.get(k)) : f);

function b64Floats(data: string): Float32Array {
  const bin = atob(data);
  const bytes = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i += 1) bytes[i] = bin.charCodeAt(i);
  return new Float32Array(bytes.buffer);
}

function floatsB64(values: number[]): string {
  const bytes = new Uint8Array(new Float32Array(values).buffer);
  let s = "";
  for (let i = 0; i < bytes.length; i += 1) s += String.fromCharCode(bytes[i]);
  return btoa(s);
}

async function main() {
  const width = num("w", 1400), height = num("h", 520);
  const d = num("d", 300);
  const mode = params.get("mode") ?? "both";
  const species = (params.get("species") ?? "huai-shu,wu-tong,xiang-zhang,yin-xing,yang-shu,xue-song,liu-shu,rong-shu").split(",");
  const data = await (await fetch(params.get("scene") ?? "/def.json")).json();
  const scene: CityScene = Array.isArray(data) ? data[0] : data;

  // Place one detail tree per species, at the prototype's own real size.
  const pitch = num("pitch", 24);
  const heights = new Map<string, number>();
  for (const list of scene.instances as Array<{ key: string; count: number; data: string }>) {
    const m = /^tree\/([^/]+)\/0$/.exec(list.key);
    if (!m) {
      if (list.key.startsWith("tree/")) { list.count = 0; list.data = ""; }
      continue;
    }
    const i = species.indexOf(m[1]);
    if (i < 0 || list.count === 0) { list.count = 0; list.data = ""; continue; }
    const f = b64Floats(list.data);
    const scale = f[4];
    heights.set(m[1], scale);
    const x = (i - (species.length - 1) / 2) * pitch - pitch * 0.23;
    list.count = mode === "far" ? 0 : 1;
    list.data = floatsB64([x, 0, 0, 0.7, scale, scale, scale, 1, 1, 1]);
  }
  // Every other instance list (cars, lamps, hedges) is cleared: this is a tree view.
  for (const list of scene.instances as Array<{ key: string; count: number; data: string }>) {
    if (!list.key.startsWith("tree/")) { list.count = 0; list.data = ""; }
  }

  const renderer = new THREE.WebGLRenderer({ antialias: true, preserveDrawingBuffer: true });
  renderer.setSize(width, height);
  renderer.outputColorSpace = THREE.SRGBColorSpace;
  renderer.toneMapping = THREE.ACESFilmicToneMapping;
  renderer.toneMappingExposure = 0.95;
  renderer.shadowMap.enabled = true;
  renderer.shadowMap.type = THREE.PCFShadowMap;
  document.getElementById("viewport")!.appendChild(renderer.domElement);

  const world = new THREE.Scene();
  world.fog = createFog();
  const sky = createSky();
  world.add(sky.dome);
  const lighting = createLighting();
  lighting.sun.shadow.mapSize.set(2048, 2048);
  world.add(lighting.sun, lighting.sun.target, lighting.hemisphere, lighting.fill);
  world.environmentIntensity = 0.75;

  const materials = createMaterials(scene.textures);
  const city = buildCityScene(scene, materials);
  // A tree view: the city's streets and buildings are not part of it.
  if (params.get("statics") !== "1") for (const mesh of city.statics) mesh.visible = false;
  world.add(city.group);

  // The far crowns, in the layer's own vertex-colour material.
  const positions: number[] = [], normals: number[] = [], colors: number[] = [];
  const colour = new THREE.Color();
  const sink = {
    tri(a: number[], b: number[], c: number[], normal: number[], hex: number, shade = 1) {
      const ux = b[0] - a[0], uy = b[1] - a[1], uz = b[2] - a[2];
      const vx = c[0] - a[0], vy = c[1] - a[1], vz = c[2] - a[2];
      const nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
      const flip = nx * normal[0] + ny * normal[1] + nz * normal[2] < 0;
      const pts = flip ? [a, c, b] : [a, b, c];
      colour.setHex(hex);
      for (const p of pts) {
        positions.push(p[0], p[1], p[2]);
        normals.push(normal[0], normal[1], normal[2]);
        colors.push(colour.r * shade, colour.g * shade, colour.b * shade);
      }
    },
  };
  Object.assign(sink, {
    triSmooth(a: number[], b: number[], c: number[], na: number[], nb: number[], nc: number[], hex: number, shade = 1) {
      const ux = b[0] - a[0], uy = b[1] - a[1], uz = b[2] - a[2];
      const vx = c[0] - a[0], vy = c[1] - a[1], vz = c[2] - a[2];
      const nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
      const flip = nx * (na[0] + nb[0] + nc[0]) + ny * (na[1] + nb[1] + nc[1]) + nz * (na[2] + nb[2] + nc[2]) < 0;
      const pts = flip ? [a, c, b] : [a, b, c];
      const ns = flip ? [na, nc, nb] : [na, nb, nc];
      colour.setHex(hex);
      for (let i = 0; i < 3; i += 1) {
        positions.push(pts[i][0], pts[i][1], pts[i][2]);
        normals.push(ns[i][0], ns[i][1], ns[i][2]);
        colors.push(colour.r * shade, colour.g * shade, colour.b * shade);
      }
    },
  });
  if (mode !== "near") {
    species.forEach((key, i) => {
      const x = (i - (species.length - 1) / 2) * pitch + pitch * 0.23;
      const size = speciesSize(key);
      const h = heights.get(key) ?? size.height;
      drawCrown(sink, x, 0, 0, h, size.crown, key, 0.37 + i * 0.11);
    });
  }
  const farGeometry = new THREE.BufferGeometry();
  farGeometry.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
  farGeometry.setAttribute("normal", new THREE.Float32BufferAttribute(normals, 3));
  farGeometry.setAttribute("color", new THREE.Float32BufferAttribute(colors, 3));
  const far = new THREE.Mesh(
    farGeometry,
    new THREE.MeshStandardMaterial({ vertexColors: true, roughness: 1, metalness: 0, emissive: 0x16240f, emissiveIntensity: 0.5 }),
  );
  far.castShadow = true;
  far.receiveShadow = true;
  world.add(far);

  const ground = new THREE.Mesh(new THREE.PlaneGeometry(4000, 4000).rotateX(-Math.PI / 2), new THREE.MeshStandardMaterial({ color: 0x5b6b3c, roughness: 1 }));
  ground.position.y = -0.02;
  ground.receiveShadow = true;
  world.add(ground);

  // Telephoto framing: the row fills the picture whatever the distance.
  const row = species.length * pitch + 12;
  const eye = num("eye", 4);
  const target = new THREE.Vector3(0, num("look", 8), 0);
  const aspect = width / height;
  const vfov = 2 * THREE.MathUtils.RAD2DEG * Math.atan(Math.max(row / aspect, 26) / 2 / d);
  const camera = new THREE.PerspectiveCamera(vfov, aspect, 0.5, 12000);
  camera.position.set(0, eye, d);
  camera.lookAt(target);
  lighting.focus(target, Math.max(row, 60));

  let frame = 0;
  const tick = () => {
    sky.update(0);
    renderer.render(world, camera);
    frame += 1;
    if (frame === 3) {
      window.__CITY_DIAGNOSTICS__ = {
        d, mode, species,
        draws: renderer.info.render.calls,
        triangles: renderer.info.render.triangles,
        farTriangles: positions.length / 9,
        missingMaterials: [...materials.missing],
      };
      window.__RENDER_READY__ = true;
    }
    window.setTimeout(tick, 200);
  };
  tick();
}

main().catch((error) => {
  document.body.innerText = String(error?.stack ?? error);
});
