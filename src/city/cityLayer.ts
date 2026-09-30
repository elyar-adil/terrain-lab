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
  update(seconds: number): void;
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

  // Ground apron under the city: the terrain is levelled beneath it, and this
  // grass plate sits between the roadbed and that levelled ground so that the
  // satellite texture never shows through between blocks.
  const [minX, minZ, maxX, maxZ] = scene.extentM;
  const pad = 60;
  const apron = new THREE.Mesh(
    new THREE.PlaneGeometry(maxX - minX + pad * 2, maxZ - minZ + pad * 2),
    new THREE.MeshStandardMaterial({ color: 0x5f6d47, roughness: 1, metalness: 0 }),
  );
  apron.rotation.x = -Math.PI / 2;
  apron.position.set((minX + maxX) / 2, -0.45, (minZ + maxZ) / 2);
  apron.receiveShadow = true;
  apron.name = "apron";
  group.add(apron);

  const agentCount = scene.traffic.agents.length;
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

  return {
    group,
    scene,
    problems,
    update(seconds: number) {
      const clock = materials.get("water").userData.uTime as { value: number } | undefined;
      if (clock) clock.value = seconds;
    },
    dispose() {
      group.removeFromParent();
      for (const mesh of owned) mesh.dispose();
      apron.geometry.dispose();
      (apron.material as THREE.Material).dispose();
      handles.dispose();
      materials.dispose();
    },
  };
}
