/**
 * Renders grown trees alone, with the city's sky, light and post chain, so the
 * trees can be judged without a city around them. Driven by the query string:
 *
 *   species=xiang-zhang | all    which species (all: one of each, in a grid)
 *   n=1                          how many of the species, in a row
 *   h=12                         height, metres
 *   seed=1                       first tree's seed
 *   season=0.5                   day of the year, 0..1
 *   cam=x,y,z,tx,ty,tz           camera and target
 *   fov=45  post=0|1  shadows=0|1
 */

import * as THREE from "three";

import { bakeSkyEnvironment, createFog, createLighting, createSky } from "./city/sky";
import { createPost } from "./city/post";
import { TreeGrower, UniqueForest, type PlantedTree } from "./trees";

declare global {
  interface Window {
    __RENDER_READY__?: boolean;
    __TREE_STATS__?: Record<string, unknown>;
  }
}

const params = new URLSearchParams(window.location.search);
const num = (key: string, fallback: number) => {
  const raw = params.get(key);
  return raw === null || Number.isNaN(Number(raw)) ? fallback : Number(raw);
};

async function main() {
  const grower = await TreeGrower.load();
  const wanted = params.get("species") ?? "xiang-zhang";
  // `species=all` draws every species; `from`/`to` pick a slice of the catalogue.
  const everyone = grower.speciesKeys.map((_, i) => i).slice(num("from", 0), num("to", 99));
  const species = wanted === "all" ? everyone : [Math.max(0, grower.speciesIndex(wanted))];
  const count = Math.max(1, Math.floor(num("n", 1)));
  const height = num("h", 12);
  const seed0 = num("seed", 1);
  const spacing = num("gap", height * 0.9);

  const trees: PlantedTree[] = [];
  const columns = wanted === "all" ? 6 : count;
  let index = 0;
  for (const sp of species) {
    for (let k = 0; k < count; k += 1) {
      const slot = wanted === "all" ? index : k;
      const col = slot % columns;
      const row = Math.floor(slot / columns);
      trees.push({
        x: (col - (columns - 1) / 2) * spacing,
        y: 0,
        z: -row * spacing,
        species: sp,
        height: height * (0.85 + 0.3 * (Math.abs(Math.sin((seed0 + index) * 12.9898) * 43758.5453) % 1)),
        seed: seed0 + index,
        openness: num("open", 0.85),
        age: num("age", 0.8),
        health: num("health", 1),
        lift: 0,
      });
      index += 1;
    }
  }

  const width = window.innerWidth;
  const heightPx = window.innerHeight;
  const renderer = new THREE.WebGLRenderer({ antialias: params.get("post") !== "1", preserveDrawingBuffer: true });
  renderer.setPixelRatio(1);
  renderer.setSize(width, heightPx);
  renderer.outputColorSpace = THREE.SRGBColorSpace;
  renderer.toneMapping = THREE.ACESFilmicToneMapping;
  renderer.toneMappingExposure = 0.95;
  renderer.shadowMap.enabled = params.get("shadows") !== "0";
  renderer.shadowMap.type = THREE.PCFSoftShadowMap;
  document.body.appendChild(renderer.domElement);

  const world = new THREE.Scene();
  world.fog = createFog(900);
  const sky = createSky();
  world.add(sky.dome);
  const lighting = createLighting();
  world.add(lighting.sun, lighting.sun.target, lighting.hemisphere, lighting.fill);
  world.environment = bakeSkyEnvironment(renderer, sky.material);
  world.environmentIntensity = 0.75;

  const ground = new THREE.Mesh(
    new THREE.PlaneGeometry(4000, 4000).rotateX(-Math.PI / 2),
    new THREE.MeshStandardMaterial({ color: 0x5b6b3a, roughness: 1 }),
  );
  ground.receiveShadow = true;
  world.add(ground);

  const cam = (params.get("cam") ?? "").split(",").map(Number);
  const camera = new THREE.PerspectiveCamera(num("fov", 45), width / heightPx, 0.1, 20000);
  const centreZ = -((Math.ceil(trees.length / columns) - 1) * spacing) / 2;
  const position = cam.length >= 3 && cam.every((x) => !Number.isNaN(x))
    ? new THREE.Vector3(cam[0], cam[1], cam[2])
    : new THREE.Vector3(0, height * 0.6, centreZ + height * 3.2 + (columns - 1) * spacing * 0.6);
  const target = cam.length >= 6 ? new THREE.Vector3(cam[3], cam[4], cam[5]) : new THREE.Vector3(0, height * 0.45, centreZ);
  camera.position.copy(position);
  camera.lookAt(target);
  lighting.focus(target, Math.max(60, height * 6 + columns * spacing));

  const forest = new UniqueForest(grower, trees, { season: num("season", 0.5), farM: 5000 });
  world.add(forest.group);

  const post = params.get("post") === "1" ? createPost(renderer, world, { camera }) : null;

  let frames = 0;
  let settled = 0;
  const tick = () => {
    forest.update(camera.position, 40);
    sky.follow(camera);
    if (post) post.render();
    else renderer.render(world, camera);
    frames += 1;
    if (forest.pending === 0) settled += 1;
    if (settled === 3) {
      window.__TREE_STATS__ = {
        ...forest.stats,
        planted: trees.length,
        draws: renderer.info.render.calls,
        triangles: renderer.info.render.triangles,
        frames,
      };
      window.__RENDER_READY__ = true;
      document.documentElement.dataset.renderReady = "true";
    }
    // Once settled, stop: a software rasteriser should not keep drawing frames
    // while the screenshot waits for the clock.
    if (settled < 3) setTimeout(tick, 30);
  };
  tick();
}

main().catch((error) => {
  document.body.innerHTML = `<pre style="color:#f66;padding:20px">${String(error?.stack ?? error)}</pre>`;
  throw error;
});
