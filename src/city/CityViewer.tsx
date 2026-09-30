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
  foliageMaskTexture,
} from "./cityScene";
import { createPost } from "./post";
import {
  SKY_FOG,
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
  /** A low oblique view across a district: street walls, roofs and lawns together. */
  district: { name: "district", position: [-150, 38, 210], target: [20, 14, -20], fov: 52, radius: 380 },
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
  // Far larger than the fog reach: the edge must dissolve into haze, never show as a square.
  const size = Math.max(24000, extent * 24);
  const geometry = new THREE.PlaneGeometry(size, size, 1, 1);
  geometry.rotateX(-Math.PI / 2);
  const material = new THREE.MeshStandardMaterial({
    color: 0x5f6d47,
    roughness: 0.96,
    metalness: 0,
  });
  const ground = new THREE.Mesh(geometry, material);
  ground.name = "ground";
  ground.receiveShadow = true;
  ground.position.y = -0.45; // well below the roadbed: at 100 m+ a 5 cm gap is lost in the depth buffer
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
      const base = PRESETS[preset] ?? PRESETS.street;
      // `?cam=x,y,z,tx,ty,tz[,fov]` overrides the framing, for audits and bug reports.
      const camParam = new URLSearchParams(window.location.search).get("cam");
      const cv = camParam ? camParam.split(",").map(Number) : [];
      const chosen: CameraPreset =
        cv.length >= 6 && cv.every((n) => Number.isFinite(n))
          ? { ...base, position: [cv[0], cv[1], cv[2]], target: [cv[3], cv[4], cv[5]], fov: cv[6] ?? base.fov }
          : base;
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
      /**
       * Size from the window, not from the container.
       *
       * The container is laid out by React *after* this effect runs, so reading
       * its box here gets a value that is about to change, and the canvas is then
       * resized out from under a frame that has already been composed. Measuring
       * the window instead removes the dependency entirely, and for a viewer
       * that always fills the page it is also simply the correct box.
       */
      const viewportSize = () => ({
        width: Math.max(1, window.innerWidth),
        height: Math.max(1, window.innerHeight),
      });
      const initial = viewportSize();
      renderer.setSize(initial.width, initial.height);
      // The GL context, for the diagnostics. A screenshot that fills only part of
      // the window is indistinguishable from a scene problem unless the drawing
      // buffer and the viewport can be read directly.
      const gl = renderer.getContext();
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
      const fogParam = new URLSearchParams(window.location.search).get("fog");
      // Fog is scaled by how far the camera stands from what it looks at: a
      // street view wants haze at city scale, a bird's-eye view must not fog the
      // very city it frames.
      {
        const fog = world.fog as THREE.Fog;
        const reach = Math.max(200, extent);
        const stand = Math.hypot(
          chosen.position[0] - chosen.target[0],
          chosen.position[1] - chosen.target[1],
          chosen.position[2] - chosen.target[2],
        );
        fog.near = reach * 0.45 + stand * 0.6;
        fog.far = reach * 1.3 + stand * 1.6;
      }
      if (fogParam) {
        const [n, f] = fogParam.split(",").map(Number);
        if (Number.isFinite(n) && Number.isFinite(f)) (world.fog as THREE.Fog).near = n, (world.fog as THREE.Fog).far = f;
      }
      /**
       * A background behind the sky dome, as insurance.
       *
       * The dome is a 9 km sphere drawn with `depthTest` off, so if it ever fails
       * to cover the frame — a camera outside it, a clipping problem, a lost
       * context — the result is opaque black, which reads as "the renderer is
       * broken" rather than "the sky is missing". Clearing to the horizon colour
       * makes that failure mode indistinguishable from a slightly flat sky.
       */
      world.background = new THREE.Color(SKY_FOG);

      const sky = createSky();
      world.add(sky.dome);

      /**
       * A bisect switch for the audit: `?hide=ground,sky,lamps,cars,trees`.
       *
       * A region of the frame that renders black while every buffer reports a
       * sane size, every vertex a plausible coordinate and every instance a
       * plausible transform, is being drawn by *something*. Rather than reason
       * about which, this removes candidates one at a time so a single run
       * identifies the culprit. It stays in the product because a renderer that
       * cannot be taken apart is a renderer that cannot be debugged.
       */
      const problems: string[] = [];
      const hidden = new Set(
        (new URLSearchParams(window.location.search).get("hide") ?? "")
          .split(",")
          .map((value) => value.trim())
          .filter(Boolean),
      );

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

      // Count the lamps first: an `InstancedMesh`'s capacity is fixed at
      // construction, and under-sizing it does not draw fewer lamps, it makes the
      // GPU read past the end of the instance buffer.
      let lampTotal = 0;
      for (const rig of scene.signals) lampTotal += rig.lamps.length;
      const materials: MaterialCatalogue = createMaterials(scene.textures, lampTotal);
      mark(`materials (${materials.textures.length} textures)`);
      const handles: CityHandles = buildCityScene(scene, materials);
      world.add(handles.group);
      if (hidden.has("ground")) mark("hiding the ground plane");
      else world.add(createGround(extent));
      if (hidden.has("sky")) {
        mark("hiding the sky dome");
        sky.dome.visible = false;
      }
      if (hidden.has("trees")) {
        mark("hiding instanced foliage");
        for (const [key, mesh] of handles.instanced) {
          if (key.startsWith("tree/")) mesh.visible = false;
        }
      }
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
        // Body colour per vehicle: mostly white, black, silver and grey, as on a
        // real Chinese road. Without it the whole fleet is one flat white.
        const paints = [0xf1f1ee, 0xf1f1ee, 0xf1f1ee, 0x17181a, 0x17181a, 0x17181a, 0x9ea3a8, 0x9ea3a8, 0x5b5e63, 0x8c1c1c, 0x24406f, 0x6b5a44];
        const paint = new THREE.Color();
        for (let i = 0; i < agentCount; i += 1) {
          fleet.setColorAt(i, paint.setHex(paints[(i * 7 + (i >> 2)) % paints.length]));
        }
        if (fleet.instanceColor) fleet.instanceColor.needsUpdate = true;
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
        const wanted = slots.length / 3;
        if (wanted > mesh.count + mesh.instanceMatrix.count) {
          // `setMatrixAt` past capacity is a silent no-op, so this is checked
          // rather than assumed. See `createMaterials`.
          problems.push(
            `${wanted} signal lamps of aspect ${aspect} exceed the mesh capacity of ` +
              `${mesh.instanceMatrix.count}`,
          );
        }
        mesh.count = wanted;
        for (let index = 0; index < wanted; index += 1) {
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
        initial.width / initial.height,
        // A far view has nothing within tens of metres, and the depth buffer's
        // resolution at 400 m is what makes coplanar ground plates flicker.
        chosen.position[1] > 100 ? 6 : 1.0,
        12000,
      );
      camera.position.set(...chosen.position);
      const lookAt = new THREE.Vector3(...chosen.target);
      camera.lookAt(lookAt);
      sky.follow(camera);
      lighting.focus(lookAt, chosen.radius);

      // The post chain (GTAO + bloom + ACES output) is where geometry stops
      // reading as floating cardboard. It is the most expensive part of the
      // frame after the environment bake, so the cheap audit path skips it.
      const post = cheap
        ? null
        : createPost(renderer, world, {
            camera,
            foliageMask: foliageMaskTexture(scene.textures),
          });
      mark(post ? "post" : "post skipped (cheap)");

      // --- loop ---------------------------------------------------------------
      const dummy = new THREE.Object3D();
      const started = performance.now();
      let frame = 0;
      /**
       * The frame number at the last resize.
       *
       * Readiness cannot simply be "three frames have been drawn": the container
       * is laid out after this effect runs, so the first frames are composed at a
       * fallback size and the canvas is resized afterwards. Reporting ready then
       * lets the audit screenshot a frame that predates the resize, and the
       * result looks like a scene that renders into a third of the frame.
       * Requiring two clean frames *after* the last resize is what makes the
       * screenshot match the window.
       */
      let sizedAt = 0;

      const drawFleet = (poses: CityScene["traffic"]["agents"]) => {
        if (!fleet) return;
        for (let index = 0; index < poses.length; index += 1) {
          const pose = poses[index];
          dummy.position.set(pose.x, pose.y, pose.z);
          /**
           * `heading` arrives as a three.js Y-Euler, derived in Rust as
           * `atan2(-z, x)` to match this renderer's own axis convention.
           *
           * It used to arrive as a compass bearing, and this applied
           * `-heading + PI/2` to convert. That compensation is now *wrong* — it
           * rotated every vehicle a further quarter turn, so traffic drove along
           * the kerb rather than in its lane. The conversion belongs in the layer
           * that knows the frame, and it is now there.
           */
          dummy.rotation.set(0, pose.heading, 0);
          dummy.scale.set(1, 1, 1);
          dummy.updateMatrix();
          fleet.setMatrixAt(index, dummy.matrix);
          if (fleetGlass) fleetGlass.setMatrixAt(index, dummy.matrix);
        }
        fleet.instanceMatrix.needsUpdate = true;
        if (fleetGlass) fleetGlass.instanceMatrix.needsUpdate = true;
      };
      drawFleet(scene.traffic.agents);

      const cores: THREE.Object3D[] = [];
      handles.group.traverse((object) => {
        if (object.userData.part === "mass") cores.push(object);
      });
      const coreDistance = camera.position.distanceTo(lookAt);
      for (const core of cores) core.visible = coreDistance > 260;

      const tick = () => {
        if (disposed) return;
        const elapsed = (performance.now() - started) / 1000;
        sky.update(elapsed);
        frame += 1;
        if (post) post.render();
        else renderer.render(world, camera);
        // The audit needs to know a frame has actually been composed, not that
        // the scene graph was populated. Under software WebGL those are seconds
        // apart, and reporting the second one produced blank screenshots.
        if (frame >= 3 && frame > sizedAt + 1 && !window.__CITY_READY__) {
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
            // A short UV buffer renders its whole group untextured. Naming the
            // material is the difference between "a wall looks wrong somewhere"
            // and a one-line fix in the scene layer.
            uvMismatch: [...materials.uvMismatch],
            vehicles: agentCount,
            signals: scene.signals.length,
            lamps: lampsByAspect.map((mesh) => mesh.count),
            exposure: renderer.toneMappingExposure,
            sun: SUN_DIR.toArray(),
            /**
             * Canvas geometry, so a screenshot that is not full-bleed can be
             * diagnosed rather than guessed at. A view that renders a third of
             * the frame and leaves the rest blank looks like a scene problem and
             * is a layout one.
             */
            canvas: {
              bufferWidth: renderer.domElement.width,
              bufferHeight: renderer.domElement.height,
              cssWidth: renderer.domElement.clientWidth,
              cssHeight: renderer.domElement.clientHeight,
              rect: (() => {
                const box = renderer.domElement.getBoundingClientRect();
                return [box.x, box.y, box.width, box.height].map((v) => Math.round(v));
              })(),
              drawingBuffer: [gl.drawingBufferWidth, gl.drawingBufferHeight],
              viewport: (() => {
                const v = new THREE.Vector4();
                renderer.getViewport(v);
                return [v.x, v.y, v.z, v.w];
              })(),
              setSize: renderer.getSize(new THREE.Vector2()).toArray(),
              hostWidth: host.clientWidth,
              hostHeight: host.clientHeight,
              windowWidth: window.innerWidth,
              windowHeight: window.innerHeight,
              pixelRatio: renderer.getPixelRatio(),
              devicePixelRatio: window.devicePixelRatio,
            },
          };
          window.__CITY_READY__ = true;
          document.documentElement.dataset.cityReady = "true";
          onReady?.(window.__CITY_DIAGNOSTICS__);
        }
        window.requestAnimationFrame(tick);
      };
      window.requestAnimationFrame(tick);

      const onResize = () => {
        const size = viewportSize();
        renderer.setSize(size.width, size.height);
        camera.aspect = size.width / size.height;
        camera.updateProjectionMatrix();
        post?.setSize(size.width, size.height);
        /**
         * Draw immediately at the new size.
         *
         * Resizing a WebGL canvas discards its drawing buffer. Until the next
         * animation frame the canvas holds whatever the compositor last had, and
         * under SwiftShader a screenshot taken in that window captures a frame
         * composed at the *previous* size — which is how a 1000 px viewport came
         * back with the right-hand 57% blank white. Rendering synchronously here
         * closes the window.
         */
        sizedAt = frame;
        if (post) post.render();
        else renderer.render(world, camera);
      };
      window.addEventListener("resize", onResize);
      /**
       * A `ResizeObserver`, not just the window `resize` event.
       *
       * The canvas is sized from its container, and the container is laid out by
       * React *after* this effect runs — so reading `clientWidth` once at mount
       * gets zero and falls through to a hard-coded 1280x800 that does not match
       * the window. That produced a screenshot with the right-hand third of the
       * frame empty and no error anywhere. The observer also fires on first
       * layout, which is the case that matters.
       */
      const observer = new ResizeObserver(onResize);
      observer.observe(host);

      teardown = () => {
        observer.disconnect();
        window.removeEventListener("resize", onResize);
        post?.dispose();
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
