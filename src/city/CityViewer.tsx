/**
 * The city viewer.
 *
 * A standalone renderer for one `CityScene`, used to look at the city on its own
 * without the terrain payload. That is a deliberate division: the city scene is a
 * flat local frame, and everything in it is derived and finished, so a view of it
 * answers "is the city right" without a hundred megabytes of heightfield in the
 * way. It is also the harness the visual audit drives, which is why the camera is
 * addressable from the URL rather than only from the mouse — a defect has to be
 * reproducible to be fixed.
 */

import { useEffect, useRef, useState } from "react";
import * as THREE from "three";

import {
  type CityHandles,
  type CityScene,
  type MaterialCatalogue,
  buildCityScene,
  createMaterials,
  decodeFloats,
} from "./cityScene";
import {
  SUN_DIR,
  bakeSkyEnvironment,
  createFog,
  createLighting,
  createSky,
} from "./sky";

declare global {
  interface Window {
    __CITY_READY__?: boolean;
    __CITY_DIAGNOSTICS__?: Record<string, unknown>;
  }
}

/**
 * A named viewpoint.
 *
 * Each is chosen to make one class of defect *visible*. Framing a city from a
 * single hero angle hides half of what can be wrong, which is why the previous
 * attempts at this looked fine and were not.
 */
export interface CameraPreset {
  name: string;
  /** Where the camera is, in city-local metres. */
  position: [number, number, number];
  /** What it looks at, in city-local metres. */
  target: [number, number, number];
  fov: number;
  /** Shadow frustum radius, metres. */
  radius: number;
}

export const PRESETS: Record<string, CameraPreset> = {
  /** A street-level view down an avenue: the test for markings, kerbs, trees,
   *  shopfronts, traffic and sky all at once. */
  street: { name: "street", position: [-52, 6.5, 96], target: [24, 3.0, -10], fov: 55, radius: 120 },
  /** From a traffic-light height at a junction: the test for the box, the
   *  crossings, the stop lines, the signal heads and the guide markings. */
  junction: { name: "junction", position: [22, 9, 40], target: [-4, 1.2, -4], fov: 48, radius: 90 },
  /** Looking up a tower from its base: the test for the facade tile's storey
   *  rhythm, the podium, the balcony stack and the roofscape. */
  tower: { name: "tower", position: [70, 4, 74], target: [6, 42, 6], fov: 62, radius: 150 },
  /** A raised three-quarter view: the test for massing, roof forms, block
   *  structure and whether the skyline has any shape at all. */
  aerial: { name: "aerial", position: [430, 300, 430], target: [0, 12, 0], fov: 40, radius: 620 },
  /** The whole city from outside: the test for the extent, the silhouette
   *  against the sky, and the fog reading as air rather than as paper. */
  skyline: {
    name: "skyline",
    position: [-980, 190, 1180],
    target: [0, 40, 0],
    fov: 34,
    radius: 900,
  },
};

/** A plain ground plane, so the city is not floating in a void. */
function createGround(extent: number): THREE.Mesh {
  const size = Math.max(600, extent * 1.6);
  const geometry = new THREE.PlaneGeometry(size, size, 1, 1);
  geometry.rotateX(-Math.PI / 2);
  const material = new THREE.MeshStandardMaterial({
    color: 0x39412e,
    roughness: 0.96,
    metalness: 0,
  });
  const ground = new THREE.Mesh(geometry, material);
  ground.name = "ground";
  ground.receiveShadow = true;
  ground.position.y = -0.05;
  return ground;
}

function aspectOf(scene: CityScene): number {
  const [minX, minZ, maxX, maxZ] = scene.extentM;
  return Math.max(1, Math.max(maxX - minX, maxZ - minZ));
}

export interface CityViewerProps {
  scene: CityScene;
  preset?: string;
  onReady?: (diagnostics: Record<string, unknown>) => void;
}

export function CityViewer({ scene, preset = "street", onReady }: CityViewerProps) {
  const mount = useRef<HTMLDivElement | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const host = mount.current;
    if (!host) return;
    let disposed = false;
    let teardown: (() => void) | null = null;

    try {
      const chosen = PRESETS[preset] ?? PRESETS.street;
      const extent = aspectOf(scene);
      // `?cheap=1` trades the environment bake and the shadow resolution for
      // speed. The headless audit runs on SwiftShader, where a PMREM bake and a
      // 2k shadow map cost tens of seconds each, so an iteration loop that keeps
      // them is an iteration loop nobody runs. Framing, geometry, materials and
      // tone are all still exercised; only the two slowest effects are reduced.
      const cheap = new URLSearchParams(window.location.search).get("cheap") === "1";
      const mark = (stage: string) => {
        if (cheap) console.log(`[city] ${stage}`);
      };
      mark("begin");

      const renderer = new THREE.WebGLRenderer({
        antialias: !cheap,
        // A city is mostly high-frequency detail: thin lane lines, railings,
        // wires, leaf cards. Without this the whole image crawls, and no amount
        // of texture quality survives it.
        powerPreference: "high-performance",
        preserveDrawingBuffer: true,
      });
      renderer.setPixelRatio(cheap ? 1 : Math.min(2, window.devicePixelRatio || 1));
      renderer.setSize(host.clientWidth || 1280, host.clientHeight || 800);
      renderer.outputColorSpace = THREE.SRGBColorSpace;
      // ACES is the filmic curve. It rolls highlights off instead of clipping
      // them, which is the difference between a sunlit white render and a
      // blown-out white hole.
      renderer.toneMapping = THREE.ACESFilmicToneMapping;
      // Slightly under 1: the palette is physical albedo, and physical albedo
      // under a correct sun needs a little headroom, not a boost.
      renderer.toneMappingExposure = 0.95;
      renderer.shadowMap.enabled = true;
      renderer.shadowMap.type = cheap ? THREE.PCFShadowMap : THREE.PCFSoftShadowMap;
      host.appendChild(renderer.domElement);
      mark("renderer");

      const world = new THREE.Scene();
      world.fog = createFog(extent);

      const sky = createSky();
      world.add(sky.dome);

      const lighting = createLighting();
      if (cheap) lighting.sun.shadow.mapSize.set(1024, 1024);
      world.add(lighting.sun, lighting.sun.target, lighting.hemisphere, lighting.fill);
      mark("lights");

      // The environment bake is the single biggest contributor to a render
      // looking lit rather than composited — and, on a software rasteriser, the
      // single most expensive thing in the frame.
      if (cheap) {
        world.environmentIntensity = 0.75;
      } else {
        const environment = bakeSkyEnvironment(renderer, sky.material);
        world.environment = environment;
        world.environmentIntensity = 0.75;
      }
      mark("environment");

      const materials: MaterialCatalogue = createMaterials(scene.textures);
      mark(`materials (${materials.textures.length} textures)`);
      const handles: CityHandles = buildCityScene(scene, materials);
      world.add(handles.group);
      world.add(createGround(extent));
      mark("geometry");

      // --- traffic and signals ------------------------------------------------
      // The moving fleet is one instanced mesh per part, positioned every frame
      // from the poses Rust hands over. The poses arrive already on the lane
      // graph, so the renderer never computes a route.
      const agentCount = scene.traffic.agents.length;
      const bodyMaterial = materials.get("car/body");
      const glassMaterial = materials.get("car/glass");
      const fleet = handles.carBody
        ? new THREE.InstancedMesh(handles.carBody, bodyMaterial, Math.max(1, agentCount))
        : null;
      const fleetGlass = handles.carGlass
        ? new THREE.InstancedMesh(handles.carGlass, glassMaterial, Math.max(1, agentCount))
        : null;
      if (fleet) {
        fleet.count = agentCount;
        fleet.castShadow = true;
        fleet.receiveShadow = true;
        fleet.frustumCulled = false;
        fleet.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
        world.add(fleet);
      }
      if (fleetGlass) {
        fleetGlass.count = agentCount;
        fleetGlass.castShadow = false;
        fleetGlass.frustumCulled = false;
        fleetGlass.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
        world.add(fleetGlass);
      }

      // Signal lenses: one instanced sphere per aspect, holding every lamp of
      // that aspect in the city. A phase change is a colour write on one
      // material, so it costs nothing.
      const lampsByAspect = handles.lamps;
      const lampSlots: number[][] = [[], [], []];
      for (const rig of scene.signals) {
        for (const lamp of rig.lamps) {
          const slot = lampSlots[lamp.aspect] ?? lampSlots[0];
          slot.push(lamp.position[0], lamp.position[1], lamp.position[2]);
        }
      }
      const lampDummy = new THREE.Object3D();
      for (let aspect = 0; aspect < lampsByAspect.length; aspect += 1) {
        const mesh = lampsByAspect[aspect];
        const slots = lampSlots[aspect];
        mesh.count = slots.length / 3;
        for (let index = 0; index < slots.length / 3; index += 1) {
          lampDummy.position.set(slots[index * 3], slots[index * 3 + 1], slots[index * 3 + 2]);
          lampDummy.updateMatrix();
          mesh.setMatrixAt(index, lampDummy.matrix);
        }
        mesh.instanceMatrix.needsUpdate = true;
        world.add(mesh);
      }

      const applyAspects = (aspects: number[]) => {
        for (let rig = 0; rig < scene.signals.length; rig += 1) {
          const aspect = aspects[rig] ?? 0;
          const material = lampsByAspect[aspect]?.material as
            | THREE.MeshStandardMaterial
            | undefined;
          if (!material) continue;
          // A lit lamp is a light source; an unlit one is just its own dark
          // colour. Driving both from one number is why signals used to be
          // either invisible or painted on.
          material.emissiveIntensity = aspect === 3 ? 0 : 3.2;
        }
      };
      applyAspects(scene.traffic.aspects);

      // --- camera -------------------------------------------------------------
      const camera = new THREE.PerspectiveCamera(
        chosen.fov,
        (host.clientWidth || 1280) / (host.clientHeight || 800),
        0.5,
        12000,
      );
      camera.position.set(...chosen.position);
      const lookAt = new THREE.Vector3(...chosen.target);
      camera.lookAt(lookAt);
      sky.follow(camera);
      lighting.focus(lookAt, chosen.radius);

      // --- loop ---------------------------------------------------------------
      const dummy = new THREE.Object3D();
      const started = performance.now();
      let frame = 0;

      const drawFleet = (poses: CityScene["traffic"]["agents"]) => {
        if (!fleet) return;
        for (let index = 0; index < poses.length; index += 1) {
          const pose = poses[index];
          dummy.position.set(pose.x, pose.y, pose.z);
          // The pose heading is a compass bearing in the local frame; the car
          // prototype is authored facing +X, hence the quarter turn.
          dummy.rotation.set(0, -pose.heading + Math.PI / 2, 0);
          dummy.scale.set(1, 1, 1);
          dummy.updateMatrix();
          fleet.setMatrixAt(index, dummy.matrix);
          if (fleetGlass) fleetGlass.setMatrixAt(index, dummy.matrix);
        }
        fleet.instanceMatrix.needsUpdate = true;
        if (fleetGlass) fleetGlass.instanceMatrix.needsUpdate = true;
      };
      drawFleet(scene.traffic.agents);

      const tick = () => {
        if (disposed) return;
        const elapsed = (performance.now() - started) / 1000;
        sky.update(elapsed);
        frame += 1;
        renderer.render(world, camera);
        // The audit needs to know a frame has actually been composed, not that
        // the scene graph was populated. Under software WebGL those are seconds
        // apart, and reporting the second one produced blank screenshots.
        if (frame === 3 && !window.__CITY_READY__) {
          mark("first frames");
          window.__CITY_DIAGNOSTICS__ = {
            preset,
            cheap,
            draws: renderer.info.render.calls,
            triangles: renderer.info.render.triangles,
            textures: renderer.info.memory.textures,
            geometries: renderer.info.memory.geometries,
            programs: renderer.info.programs?.length ?? 0,
            missingMaterials: [...materials.missing],
            vehicles: agentCount,
            signals: scene.signals.length,
            lamps: lampsByAspect.map((mesh) => mesh.count),
            exposure: renderer.toneMappingExposure,
            sun: SUN_DIR.toArray(),
          };
          window.__CITY_READY__ = true;
          document.documentElement.dataset.cityReady = "true";
          onReady?.(window.__CITY_DIAGNOSTICS__);
        }
        window.requestAnimationFrame(tick);
      };
      window.requestAnimationFrame(tick);

      const onResize = () => {
        if (!host.clientWidth || !host.clientHeight) return;
        renderer.setSize(host.clientWidth, host.clientHeight);
        camera.aspect = host.clientWidth / host.clientHeight;
        camera.updateProjectionMatrix();
      };
      window.addEventListener("resize", onResize);

      teardown = () => {
        window.removeEventListener("resize", onResize);
        handles.dispose();
        materials.dispose();
        sky.dispose();
        lighting.dispose();
        renderer.dispose();
        renderer.domElement.remove();
      };
    } catch (thrown) {
      setError(thrown instanceof Error ? thrown.message : String(thrown));
    }

    return () => {
      disposed = true;
      teardown?.();
    };
  }, [scene, preset, onReady]);

  if (error) {
    return (
      <div style={{ color: "#ff6b6b", font: "13px ui-monospace, monospace", padding: 24 }}>
        city viewer failed: {error}
      </div>
    );
  }
  return <div ref={mount} style={{ width: "100%", height: "100%" }} />;
}

/** Decode helper re-exported so the harness can read a scene without the viewer. */
export { decodeFloats };
