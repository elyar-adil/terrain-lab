/**
 * A city as a layer that can be mounted into any three.js scene.
 *
 * The standalone `CityViewer` owns its own renderer, sky and lighting. The
 * world view must not: the city has to live in the *terrain's* scene, lit by
 * the terrain's sun and fogged by the terrain's haze, so that zooming from the
 * whole map down to a street is one continuous camera move. This module builds
 * everything that belongs to the city itself — geometry, instanced trees and
 * lamps, the traffic fleet — and leaves lights, sky, fog and post to the host.
 *
 * Building is resumable (`beginCityLayer`): the host calls `step` with a few
 * milliseconds a frame and mounts the group only when it reports done.
 */

import * as THREE from "three";

import { CityLod, LOD } from "./cityLod";
import { UniqueForest, preloadTreeGrower, treeGrowerIfReady } from "../trees";
import { plantFromInstances } from "../trees/plant";
import { perfTiming } from "./perf";
import {
  type CityBuild,
  type CityHandles,
  type CityScene,
  type MaterialCatalogue,
  beginCityBuild,
  createMaterials,
} from "./cityScene";

export interface CityLayer {
  /** City-local metres. The host scales, rotates and positions this group. */
  group: THREE.Group;
  scene: CityScene;
  problems: string[];
  /**
   * Advance animated surfaces (water) to the host's clock, in seconds, and
   * distance- and view-cull. `camera` is the host's camera (the group's
   * transform must be current); without it, LOD is skipped. Returns true when
   * shadow casters changed and the host should refresh its shadow map.
   */
  update(seconds: number, cameraDistanceM?: number, camera?: THREE.Camera): boolean;
  /** What the LOD currently keeps, for the perf overlay. */
  lodStats(): { hiddenStatics: number; instances: number };
  dispose(): void;
}

/** A city layer under construction; see `beginCityLayer`. */
export interface CityLayerBuild {
  /** Null until `done`. */
  readonly layer: CityLayer | null;
  readonly done: boolean;
  readonly progress: number;
  step(budgetMs: number): boolean;
  cancel(): void;
}

// Start fetching the tree grower as soon as a city could be wanted; by the time a
// scene has downloaded it is ready. Without it the city keeps its prototype trees.
void preloadTreeGrower();

const PAINTS = [
  0xf1f1ee, 0xf1f1ee, 0xf1f1ee, 0x17181a, 0x17181a, 0x17181a,
  0x9ea3a8, 0x9ea3a8, 0x5b5e63, 0x8c1c1c, 0x24406f, 0x6b5a44,
];

/** Build a whole layer at once (blocks). Hosts with a frame loop use `beginCityLayer`. */
export function createCityLayer(scene: CityScene): CityLayer {
  const build = beginCityLayer(scene);
  build.step(Number.POSITIVE_INFINITY);
  return build.layer as CityLayer;
}

/**
 * Build a layer a few milliseconds per call: materials, then geometry, then the
 * layer itself. Nothing in here mounts the group, so a host that waits for
 * `done` never draws a half-built city.
 */
export function beginCityLayer(scene: CityScene): CityLayerBuild {
  let stage: 0 | 1 | 2 = 0;
  let materials: MaterialCatalogue | null = null;
  let build: CityBuild | null = null;
  let layer: CityLayer | null = null;
  let cancelled = false;
  let cpu = 0;

  return {
    get layer() {
      return layer;
    },
    get done() {
      return layer !== null;
    },
    get progress() {
      if (layer) return 1;
      if (stage === 0) return 0;
      return 0.05 + 0.9 * (build?.progress ?? 0);
    },
    step(budgetMs: number): boolean {
      if (layer || cancelled) return layer !== null;
      const sliceStart = performance.now();
      const deadline = sliceStart + budgetMs;
      if (stage === 0) {
        let lampTotal = 0;
        for (const rig of scene.signals) lampTotal += rig.lamps.length;
        materials = createMaterials(scene.textures, lampTotal);
        perfTiming("cityMaterialsMs", performance.now() - sliceStart);
        stage = 1;
        build = beginCityBuild(scene, materials, { chunkCellM: 320, chunkMinTriangles: 8000, lod: true });
      }
      if (stage === 1 && build) {
        if (build.step(Math.max(0, deadline - performance.now()))) stage = 2;
      }
      if (stage === 2 && build && materials) {
        layer = finishLayer(scene, materials, build.handles);
      }
      const slice = performance.now() - sliceStart;
      cpu += slice;
      perfTiming("cityBuildSliceMaxMs", slice);
      perfTiming("cityBuildCpuMs", cpu);
      return layer !== null;
    },
    cancel() {
      if (cancelled || layer) return;
      cancelled = true;
      build?.cancel();
      materials?.dispose();
    },
  };
}

function finishLayer(
  scene: CityScene,
  materials: MaterialCatalogue,
  handles: CityHandles,
): CityLayer {
  const problems: string[] = [];
  const group = new THREE.Group();
  group.name = "city-layer";
  group.add(handles.group);

  // The vehicle models are not yet believable, so no cars are drawn at all.
  const agentCount = 0;
  const dummy = new THREE.Object3D();
  const owned: THREE.InstancedMesh[] = [];
  if (handles.carBody && agentCount > 0) {
    const body = new THREE.InstancedMesh(handles.carBody, materials.get("car/body"), agentCount);
    body.castShadow = true;
    body.receiveShadow = true;
    body.frustumCulled = false;
    const paint = new THREE.Color();
    for (let i = 0; i < agentCount; i += 1) {
      body.setColorAt(i, paint.setHex(PAINTS[(i * 7 + (i >> 2)) % PAINTS.length]));
    }
    const glass = handles.carGlass
      ? new THREE.InstancedMesh(handles.carGlass, materials.get("car/glass"), agentCount)
      : null;
    if (glass) glass.frustumCulled = false;
    scene.traffic.agents.forEach((pose, index) => {
      dummy.position.set(pose.x, pose.y, pose.z);
      dummy.rotation.set(0, pose.heading, 0);
      dummy.updateMatrix();
      body.setMatrixAt(index, dummy.matrix);
      glass?.setMatrixAt(index, dummy.matrix);
    });
    group.add(body);
    owned.push(body);
    if (glass) {
      group.add(glass);
      owned.push(glass);
    }
  }

  const slots: number[][] = [[], [], []];
  for (const rig of scene.signals) {
    for (const lamp of rig.lamps) {
      (slots[lamp.aspect] ?? slots[0]).push(...lamp.position);
    }
  }
  handles.lamps.forEach((mesh, aspect) => {
    const list = slots[aspect] ?? [];
    const wanted = list.length / 3;
    if (wanted > mesh.instanceMatrix.count) problems.push(`lamps of aspect ${aspect} exceed capacity`);
    mesh.count = Math.min(wanted, mesh.instanceMatrix.count);
    for (let i = 0; i < mesh.count; i += 1) {
      dummy.position.set(list[i * 3], list[i * 3 + 1], list[i * 3 + 2]);
      dummy.rotation.set(0, 0, 0);
      dummy.updateMatrix();
      mesh.setMatrixAt(i, dummy.matrix);
    }
    mesh.instanceMatrix.needsUpdate = true;
    const material = mesh.material as THREE.MeshStandardMaterial;
    material.emissiveIntensity = aspect === 3 ? 0 : 3.2;
    group.add(mesh);
  });

  // Every tree grown for itself: replace the prototype trees by a forest that
  // grows each one from its own seed, finer the nearer the camera is.
  let forest: UniqueForest | null = null;
  const grower = treeGrowerIfReady();
  if (grower) {
    const planting = plantFromInstances(handles.lodSets, grower, scene.seed);
    if (planting.trees.length > 0) {
      forest = new UniqueForest(grower, planting.trees);
      group.add(forest.group);
      for (const mesh of planting.replaced) mesh.removeFromParent();
      const gone = new Set(planting.sets);
      handles.lodSets = handles.lodSets.filter((set) => !gone.has(set));
    }
  }

  const lod = new CityLod(handles);
  const inverse = new THREE.Matrix4();
  const viewProjection = new THREE.Matrix4();
  const frustum = new THREE.Frustum();
  const local = new THREE.Vector3();
  const forward = new THREE.Vector3();

  return {
    group,
    scene,
    problems,
    update(seconds: number, cameraDistanceM = 0, camera?: THREE.Camera): boolean {
      const clock = materials.get("water").userData.uTime as { value: number } | undefined;
      if (clock) clock.value = seconds;
      if (!camera) return false;
      camera.updateMatrixWorld();
      // The group's scale is metres-to-scene, so its local frame is metres and
      // every LOD distance is a real distance.
      inverse.copy(group.matrixWorld).invert();
      local.setFromMatrixPosition(camera.matrixWorld).applyMatrix4(inverse);
      camera.getWorldDirection(forward).transformDirection(inverse);
      // view-projection of the camera, applied to city-local points
      viewProjection.copy(camera.matrixWorld).invert();
      viewProjection.premultiply(camera.projectionMatrix).multiply(group.matrixWorld);
      frustum.setFromProjectionMatrix(viewProjection);
      let changed = lod.update(local, forward, frustum, cameraDistanceM);
      if (forest) {
        forest.setCasting(cameraDistanceM < LOD.shadowTrees);
        changed = forest.update(local, 6) || changed;
      }
      return changed;
    },
    lodStats() {
      return { hiddenStatics: lod.hiddenStatics, instances: lod.keptInstances };
    },
    dispose() {
      group.removeFromParent();
      forest?.dispose();
      for (const mesh of owned) mesh.dispose();
      handles.dispose();
      materials.dispose();
    },
  };
}
