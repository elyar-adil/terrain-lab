/**
 * The component gallery: every facade design, every ground-floor bake, every
 * instanced prototype, laid out alone and rendered with the real pipeline.
 *
 * A whole-city screenshot can only tell you *that* something is wrong. This
 * page tells you *which* component is wrong, because each one is visible on
 * its own, at framing distance, with nothing occluding it — the visual
 * equivalent of a unit test. The row contents come from the real payload and
 * the real material catalogue, so what the gallery shows is what the city
 * renders, not a parallel implementation of it.
 *
 * Driven by `gallery.html?preset=<row>` and audited with
 * `node scripts/city-audit.mjs --page gallery.html --preset facade`.
 */

import * as THREE from "three";

import {
  type CityHandles,
  type CityScene,
  type MaterialCatalogue,
} from "./cityScene";

/** One test quad with metre-unit UVs — the contract every baked tile assumes. */
function metreQuad(width: number, height: number): THREE.BufferGeometry {
  const geometry = new THREE.PlaneGeometry(width, height);
  // Tagged, so the gallery's dispose frees only the geometry it built and
  // never the city prototypes it shares.
  geometry.userData.own = true;
  const uv = geometry.getAttribute("uv") as THREE.BufferAttribute;
  for (let index = 0; index < uv.count; index += 1) {
    // PlaneGeometry UVs are 0..1; the materials' `repeat` expects metres.
    uv.setXY(index, uv.getX(index) * width, uv.getY(index) * height);
  }
  uv.needsUpdate = true;
  return geometry;
}

function wall(width: number, height: number, x: number, material: THREE.Material): THREE.Mesh {
  const mesh = new THREE.Mesh(metreQuad(width, height), material);
  mesh.position.set(x, height / 2, 0);
  return mesh;
}

export interface GalleryHandles {
  group: THREE.Group;
  /** Key at each prototype slot, so a defect can be named in the report. */
  prototypeKeys: string[];
  dispose(): void;
}

export function buildGallery(materials: MaterialCatalogue, city: CityHandles): GalleryHandles {
  const group = new THREE.Group();

  // --- row 1: the 24 facade designs ----------------------------------------
  // Each wall is exactly two tiles wide (6 m) and one tile tall (12.8 m, four
  // storeys): enough to show pier rhythm and per-floor window variety without
  // the row turning into a street.
  let x = 0;
  for (let index = 0; index < 24; index += 1) {
    const key = `facade/${index.toString().padStart(2, "0")}`;
    group.add(wall(6, 12.8, x, materials.get(key)));
    x += 8.5;
  }

  // --- row 2: ground floors, roof, and the ground family --------------------
  // Ground-floor bays at true scale (4.2 × 4.5 m), then roof and the asphalt /
  // paving / grass / paint textures as 8 m pads.
  const groundRow = new THREE.Group();
  x = 0;
  for (const kind of ["shop", "lobby", "home"]) {
    groundRow.add(wall(4.2, 4.5, x, materials.get(`ground/${kind}`)));
    x += 6.5;
  }
  for (const key of ["roof", "asphalt", "sidewalk", "block.ground", "marking.crosswalk"]) {
    groundRow.add(wall(8, 4.5, x, materials.get(key)));
    x += 10.5;
  }
  groundRow.position.z = -20;
  group.add(groundRow);

  // --- row 3: every instanced prototype, one instance each ------------------
  // Lamps, poles, signs, the gantry, shelters, cars, and every tree species —
  // exactly the geometries the city instances thousands of times, shown once
  // each. Geometry and material are shared with the city handles, which keep
  // ownership; only the grid and its single-instance meshes are ours.
  const prototypeKeys = [...city.prototypes.keys()].sort();
  const prototypeRow = new THREE.Group();
  const slot = new THREE.Object3D();
  prototypeKeys.forEach((key, index) => {
    const prototype = city.prototypes.get(key)!;
    const mesh = new THREE.InstancedMesh(prototype.geometry, prototype.material, 1);
    mesh.name = key;
    mesh.castShadow = city.instanced.get(key)?.castShadow ?? true;
    mesh.frustumCulled = false;
    slot.position.set(index * 14, 0, 0);
    slot.updateMatrix();
    mesh.setMatrixAt(0, slot.matrix);
    prototypeRow.add(mesh);
  });
  prototypeRow.position.z = -42;
  group.add(prototypeRow);

  return {
    group,
    prototypeKeys,
    dispose() {
      // The metreQuad geometries on the test walls are the gallery's own; the
      // prototype geometry and materials stay owned by the city handles.
      group.traverse((object) => {
        if (object instanceof THREE.Mesh && object.geometry.userData.own) {
          object.geometry.dispose();
        }
      });
      group.clear();
    },
  };
}

export type { CityScene };
