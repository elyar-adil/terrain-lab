/**
 * The car harness: one generated car, alone, close, under studio light.
 *
 * A car in a street is fifty pixels long; whether its lamps sit flush and its
 * tyres have tread can only be judged from a metre away. This page loads the
 * prototypes written by
 *   cargo run --release -p city-scene --example dump_cars -- public/audit-cars.json
 * and draws them with the city's own car materials.
 *
 *   car-harness.html?variant=2&view=front34&paint=9b2f26
 *
 * `scripts/car-shot.mjs` takes the standard set of views.
 */
import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { RoomEnvironment } from "three/examples/jsm/environments/RoomEnvironment.js";

import { carMaterials } from "./city/carMaterials";

interface Dump {
  variants: number;
  tints: Array<Record<string, [number, number, number]>>;
  meshes: Record<string, { p: number[]; n: number[]; i: number[]; uv?: number[] }>;
}

/** Camera position and target, per named view, in car lengths-ish metres. */
const VIEWS: Record<string, { from: [number, number, number]; to: [number, number, number]; fov?: number }> = {
  front34: { from: [5.2, 1.5, 3.6], to: [0.2, 0.62, 0] },
  rear34: { from: [-5.4, 1.6, 3.4], to: [-0.2, 0.62, 0] },
  side: { from: [0, 1.0, 7.4], to: [0, 0.7, 0] },
  front: { from: [7.2, 0.9, 0], to: [0, 0.7, 0] },
  rear: { from: [-7.2, 0.9, 0], to: [0, 0.7, 0] },
  top: { from: [0.01, 8.5, 0], to: [0, 0, 0] },
  headlamp: { from: [3.25, 0.95, 1.55], to: [2.15, 0.68, 0.62], fov: 30 },
  taillamp: { from: [-3.3, 1.05, 1.5], to: [-2.2, 0.82, 0.6], fov: 30 },
  wheel: { from: [1.9, 0.5, 2.1], to: [1.33, 0.32, 0.8], fov: 30 },
  mirror: { from: [1.9, 1.35, 2.0], to: [0.85, 1.0, 0.95], fov: 26 },
  mirrorback: { from: [-0.6, 1.25, 1.9], to: [0.85, 1.0, 0.95], fov: 26 },
  low: { from: [4.4, 0.35, 2.6], to: [0.4, 0.55, 0], fov: 35 },
};

async function main() {
  const params = new URLSearchParams(location.search);
  const variant = Number(params.get("variant") ?? 0);
  const viewName = params.get("view") ?? "front34";
  const far = params.has("far");
  const root = document.getElementById("render-root")!;
  const renderer = new THREE.WebGLRenderer({ antialias: true, preserveDrawingBuffer: true });
  renderer.setPixelRatio(Number(params.get("dpr") ?? 1));
  renderer.setSize(root.clientWidth, root.clientHeight);
  renderer.outputColorSpace = THREE.SRGBColorSpace;
  renderer.toneMapping = THREE.ACESFilmicToneMapping;
  renderer.toneMappingExposure = 0.8;
  renderer.shadowMap.enabled = true;
  renderer.shadowMap.type = THREE.PCFSoftShadowMap;
  root.appendChild(renderer.domElement);

  const scene = new THREE.Scene();
  scene.background = new THREE.Color(0xb9bcc0);
  const pmrem = new THREE.PMREMGenerator(renderer);
  scene.environment = pmrem.fromScene(new RoomEnvironment(), 0.03).texture;
  const sun = new THREE.DirectionalLight(0xfff4e2, 2.2);
  sun.position.set(4, 9, 5);
  sun.castShadow = true;
  sun.shadow.mapSize.set(4096, 4096);
  sun.shadow.camera.left = -4;
  sun.shadow.camera.right = 4;
  sun.shadow.camera.top = 4;
  sun.shadow.camera.bottom = -4;
  sun.shadow.bias = -0.0004;
  sun.shadow.normalBias = 0.01;
  scene.add(sun);
  const ground = new THREE.Mesh(
    new THREE.CircleGeometry(30, 64),
    new THREE.MeshStandardMaterial({ color: 0xb4b7bb, roughness: 0.9 }),
  );
  ground.rotation.x = -Math.PI / 2;
  ground.receiveShadow = true;
  scene.add(ground);

  const dump = (await (await fetch(`${import.meta.env.BASE_URL}audit-cars.json`)).json()) as Dump;
  const materials = new Map(carMaterials());
  const suffix = variant === 0 ? "" : `~${variant}`;
  const paint = params.get("paint");
  let triangles = 0;
  for (const [key, mesh] of Object.entries(dump.meshes)) {
    const tilde = key.indexOf("~");
    const plain = tilde >= 0 ? key.slice(0, tilde) : key;
    if ((tilde >= 0 ? key.slice(tilde) : "") !== suffix) continue;
    if (plain.startsWith("car/far.") !== far) continue;
    const material = materials.get(plain);
    if (!material) {
      console.error(`no material for ${plain}`);
      continue;
    }
    const part = plain.slice(4);
    const tint = dump.tints[variant]?.[part] ?? [1, 1, 1];
    // As the city draws it: one instance, its tint in the instance colour.
    const colour = new THREE.Color().setRGB(tint[0], tint[1], tint[2]);
    // A showroom colour of its own per design unless one is asked for.
    const showroom = ["8f9498", "1f4fa8", "c0221c", "eef0ee", "56634e", "15171a"];
    if (part === "body" || part === "far.body") colour.set(`#${paint ?? showroom[variant % showroom.length]}`);
    const geometry = new THREE.BufferGeometry();
    geometry.setAttribute("position", new THREE.Float32BufferAttribute(mesh.p, 3));
    geometry.setAttribute("normal", new THREE.Float32BufferAttribute(mesh.n, 3));
    if (mesh.uv && mesh.uv.length > 0) geometry.setAttribute("uv", new THREE.Float32BufferAttribute(mesh.uv, 2));
    geometry.setIndex(mesh.i);
    const object = new THREE.InstancedMesh(geometry, material, 1);
    object.setMatrixAt(0, new THREE.Matrix4());
    object.setColorAt(0, colour);
    object.frustumCulled = false;
    object.name = plain;
    object.castShadow = !material.transparent;
    object.receiveShadow = !material.transparent;
    object.renderOrder = material.transparent ? (plain === "car/glass" ? 1 : 2) : 0;
    scene.add(object);
    triangles += mesh.i.length / 3;
  }

  const triple = (name: string): [number, number, number] | undefined => {
    const parts = params.get(name)?.split(",").map(Number);
    return parts && parts.length === 3 ? [parts[0], parts[1], parts[2]] : undefined;
  };
  const named = VIEWS[viewName] ?? VIEWS.front34;
  // `from=x,y,z&to=x,y,z&fov=n` looks from anywhere.
  const view = {
    from: triple("from") ?? named.from,
    to: triple("to") ?? named.to,
    fov: params.has("fov") ? Number(params.get("fov")) : named.fov,
  };
  const camera = new THREE.PerspectiveCamera(view.fov ?? 28, root.clientWidth / root.clientHeight, 0.05, 200);
  camera.position.set(...view.from);
  const controls = new OrbitControls(camera, renderer.domElement);
  controls.target.set(...view.to);
  controls.update();
  const draw = () => renderer.render(scene, camera);
  controls.addEventListener("change", draw);
  window.addEventListener("resize", () => {
    renderer.setSize(root.clientWidth, root.clientHeight);
    camera.aspect = root.clientWidth / root.clientHeight;
    camera.updateProjectionMatrix();
    draw();
  });
  draw();
  (window as unknown as { __carReady: { triangles: number } }).__carReady = { triangles };
}

void main();
