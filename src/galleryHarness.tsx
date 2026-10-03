/**
 * The component gallery harness.
 *
 * Renders one row of the component gallery at a time with the same sky,
 * lighting, materials and post chain as the city viewer, so what a component
 * looks like here is what it looks like in the city. The audit drives it via
 * `?preset=<row>`; rows are `facade`, `ground`, `prototypes` and `all`.
 */

import { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import * as THREE from "three";

import {
  buildCityScene,
  createMaterials,
  type CityScene,
  foliageMaskTexture,
} from "./city/cityScene";
import { buildGallery } from "./city/gallery";
import { framingFromQuery } from "./city/cameraParam";
import { createPost } from "./city/post";
import {
  bakeSkyEnvironment,
  createFog,
  createLighting,
  createSky,
} from "./city/sky";
import "./styles.css";

declare global {
  interface Window {
    __RENDER_READY__?: boolean;
    __CITY_DIAGNOSTICS__?: Record<string, unknown>;
  }
}

const params = new URLSearchParams(window.location.search);
const requestedFps = Number(params.get("fps"));
const frameMs = 1000 / Math.min(30, Math.max(1, Number.isFinite(requestedFps) ? requestedFps : 6));
const originalRequestAnimationFrame = window.requestAnimationFrame.bind(window);
window.requestAnimationFrame = (callback: FrameRequestCallback) =>
  window.setTimeout(() => callback(performance.now()), frameMs);
window.cancelAnimationFrame = (handle: number) => window.clearTimeout(handle);

interface RowCamera {
  position: [number, number, number];
  target: [number, number, number];
  fov: number;
  radius: number;
}

const ROW_CAMERAS: Record<string, RowCamera> = {
  facade: { position: [90, 7, 30], target: [90, 6.5, 0], fov: 55, radius: 110 },
  ground: { position: [24, 2.4, 9], target: [24, 2.0, -20], fov: 55, radius: 60 },
  prototypes: { position: [180, 40, 60], target: [180, 6, -42], fov: 50, radius: 320 },
  all: { position: [120, 90, 120], target: [95, 4, -20], fov: 45, radius: 400 },
};

function Gallery() {
  const [scene, setScene] = useState<CityScene | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    // Overridable so component agents can gallery-render their own scratch
    // fixture without touching the shared city scene.
    const sceneUrl = params.get("scene") ?? "/city-scenes.json";
    fetch(sceneUrl)
      .then((response) => {
        if (!response.ok) throw new Error(`${response.status} ${response.statusText}`);
        return response.json();
      })
      .then((data: CityScene | CityScene[]) => {
        if (!cancelled) setScene(Array.isArray(data) ? data[0] : data);
      })
      .catch((thrown: Error) => {
        if (!cancelled) setError(`${sceneUrl}: ${thrown.message}`);
      });
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    if (!scene) return;
    const host = document.getElementById("gallery-viewport");
    if (!host) return;
    let disposed = false;
    let teardown: (() => void) | null = null;

    const row = params.get("preset") ?? "all";
    const cheap = params.get("cheap") === "1";

    const renderer = new THREE.WebGLRenderer({
      antialias: !cheap,
      powerPreference: "high-performance",
      preserveDrawingBuffer: true,
    });
    renderer.setPixelRatio(cheap ? 1 : Math.min(2, window.devicePixelRatio || 1));
    renderer.setSize(host.clientWidth || 1280, host.clientHeight || 800);
    renderer.outputColorSpace = THREE.SRGBColorSpace;
    renderer.toneMapping = THREE.ACESFilmicToneMapping;
    renderer.toneMappingExposure = 0.95;
    renderer.shadowMap.enabled = true;
    renderer.shadowMap.type = cheap ? THREE.PCFShadowMap : THREE.PCFSoftShadowMap;
    host.appendChild(renderer.domElement);

    const world = new THREE.Scene();
    const framing = framingFromQuery(ROW_CAMERAS[row] ?? ROW_CAMERAS.all);
    const camera = new THREE.PerspectiveCamera(
      framing.fov,
      (host.clientWidth || 1280) / (host.clientHeight || 800),
      0.5,
      12000,
    );
    camera.position.set(...framing.position);
    const lookAt = new THREE.Vector3(...framing.target);
    camera.lookAt(lookAt);
    world.fog = createFog(600);

    const sky = createSky();
    world.add(sky.dome);
    const lighting = createLighting();
    if (cheap) lighting.sun.shadow.mapSize.set(1024, 1024);
    world.add(lighting.sun, lighting.sun.target, lighting.hemisphere, lighting.fill);
    if (!cheap) {
      world.environment = bakeSkyEnvironment(renderer, sky.material);
    }
    world.environmentIntensity = 0.75;
    lighting.focus(lookAt, framing.radius);

    const materials = createMaterials(scene.textures);
    const city = buildCityScene(scene, materials);
    const gallery = buildGallery(materials, city);
    world.add(gallery.group);

    // A real ground so the prototypes sit on something lit like a street.
    const groundGeometry = new THREE.PlaneGeometry(2000, 2000);
    const groundUv = groundGeometry.getAttribute("uv") as THREE.BufferAttribute;
    for (let index = 0; index < groundUv.count; index += 1) {
      groundUv.setXY(index, groundUv.getX(index) * 2000, groundUv.getY(index) * 2000);
    }
    groundUv.needsUpdate = true;
    const ground = new THREE.Mesh(
      groundGeometry.rotateX(-Math.PI / 2),
      materials.get("sidewalk"),
    );
    ground.position.y = -0.02;
    ground.receiveShadow = true;
    world.add(ground);

    const post = cheap
      ? null
      : createPost(renderer, world, { camera, foliageMask: foliageMaskTexture(scene.textures) });

    const started = performance.now();
    let frame = 0;
    const tick = () => {
      if (disposed) return;
      sky.update((performance.now() - started) / 1000);
      frame += 1;
      if (post) post.render();
      else renderer.render(world, camera);
      if (frame === 3) {
        window.__CITY_DIAGNOSTICS__ = {
          row,
          draws: renderer.info.render.calls,
          triangles: renderer.info.render.triangles,
          missingMaterials: [...materials.missing],
          prototypes: gallery.prototypeKeys,
        };
        window.__RENDER_READY__ = true;
        document.documentElement.dataset.renderReady = "true";
      }
      originalRequestAnimationFrame(tick);
    };
    window.requestAnimationFrame(tick);

    teardown = () => {
      gallery.dispose();
      city.dispose();
      materials.dispose();
      post?.dispose();
      sky.dispose();
      lighting.dispose();
      renderer.dispose();
      renderer.domElement.remove();
    };

    return () => {
      disposed = true;
      teardown?.();
    };
  }, [scene]);

  if (error) {
    return (
      <pre style={{ color: "#ff6b6b", font: "13px ui-monospace, monospace", padding: 24 }}>
        {`could not load /city-scenes.json\n\n${error}`}
      </pre>
    );
  }
  return <div id="gallery-viewport" style={{ width: "100%", height: "100%" }} />;
}

createRoot(document.getElementById("gallery-root")!).render(<Gallery />);
