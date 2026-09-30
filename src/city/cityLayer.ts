/**
 * A city as a layer that can be mounted into any three.js scene.
 *
 * The standalone `CityViewer` owns its own renderer, sky and lighting. The
 * world view must not: the city has to live in the *terrain's* scene, lit by
 * the terrain's sun and fogged by the terrain's haze, so that zooming from the
 * whole map down to a street is one continuous camera move. This module builds
 * everything that belongs to the city itself — geometry, instanced trees and
 * lamps, the traffic fleet — and leaves lights, sky, fog and post to the host.
 */

import * as THREE from "three";

import {
  type CityHandles,
  type CityScene,
  type MaterialCatalogue,
  buildCityScene,
  createMaterials,
} from "./cityScene";

export interface CityLayer {
  /** City-local metres. The host scales, rotates and positions this group. */
  group: THREE.Group;
  scene: CityScene;
  problems: string[];
  /** Advance animated surfaces (water) to the host's clock, in seconds. */
  update(seconds: number, cameraDistanceM?: number): void;
  dispose(): void;
}

const PAINTS = [
  0xf1f1ee, 0xf1f1ee, 0xf1f1ee, 0x17181a, 0x17181a, 0x17181a,
  0x9ea3a8, 0x9ea3a8, 0x5b5e63, 0x8c1c1c, 0x24406f, 0x6b5a44,
];

export function createCityLayer(scene: CityScene): CityLayer {
  const problems: string[] = [];
  let lampTotal = 0;
  for (const rig of scene.signals) lampTotal += rig.lamps.length;
  const materials: MaterialCatalogue = createMaterials(scene.textures, lampTotal);
  const handles: CityHandles = buildCityScene(scene, materials);
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

  const cores: THREE.Object3D[] = [];
  group.traverse((object) => {
    if (object.userData.part === "mass") cores.push(object);
  });

  return {
    group,
    scene,
    problems,
    update(seconds: number, cameraDistanceM = 0) {
      // The solid core inside each crown exists only so that, from far above, a
      // forest reads as a dense canopy instead of scattered cards. Up close the
      // real branches and leaf clumps carry the crown, so the core is dithered
      // out (alphaHash cross-fade) between 200 m and 300 m and hidden inside.
      const fade = Math.min(1, Math.max(0, (cameraDistanceM - 200) / 100));
      const coreOpacity = fade * fade * (3 - 2 * fade);
      for (const mesh of cores) {
        const visible = coreOpacity > 0.01;
        if (mesh.visible !== visible) mesh.visible = visible;
        const material = (mesh as THREE.Mesh).material as THREE.Material | undefined;
        if (material && material.opacity !== coreOpacity) material.opacity = coreOpacity;
      }
      const clock = materials.get("water").userData.uTime as { value: number } | undefined;
      if (clock) clock.value = seconds;
    },
    dispose() {
      group.removeFromParent();
      for (const mesh of owned) mesh.dispose();
      handles.dispose();
      materials.dispose();
    },
  };
}
