import * as THREE from "three";
import type { FarTreePayload } from "../types";

function bytes(base64: string): Uint8Array {
  const binary = atob(base64);
  const out = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i += 1) out[i] = binary.charCodeAt(i);
  return out;
}

function floats(base64: string): Float32Array {
  const raw = bytes(base64);
  return new Float32Array(raw.buffer, raw.byteOffset, raw.byteLength >> 2);
}

export interface FarTreeSet {
  /** One geometry per species at the requested LOD, baked in scene units at the species' own height. */
  geometries: THREE.BufferGeometry[];
  /** Shared vertex-coloured material (instance colour is a value/hue tint on top). */
  material: THREE.MeshStandardMaterial;
  dispose(): void;
}

/**
 * Far-LOD forest prototypes. The meshes come from Rust (`farTrees`, generated
 * from the city's species records); a payload without them (older fixtures)
 * falls back to one vertex-coloured cone so forests still draw.
 *
 * `lod` 0 is the near prototype (about 600 triangles, for the streamed forest
 * within a few hundred metres), 1 the mid one (about 110, the streamed forest
 * beyond that) and 2 the few-dozen-triangle one used for regional stands.
 */
export function buildFarTreeSet(
  payload: FarTreePayload[] | undefined,
  lod: number,
  metresToScene: number,
): FarTreeSet {
  const geometries: THREE.BufferGeometry[] = [];
  for (const proto of payload ?? []) {
    if (proto.lod !== lod) continue;
    const positions = floats(proto.positions);
    const scale = proto.heightMetres * metresToScene;
    const baked = new Float32Array(positions.length);
    for (let i = 0; i < positions.length; i += 1) baked[i] = positions[i] * scale;
    const indexBytes = bytes(proto.indices);
    const geometry = new THREE.BufferGeometry();
    geometry.setAttribute("position", new THREE.BufferAttribute(baked, 3));
    geometry.setAttribute("normal", new THREE.BufferAttribute(floats(proto.normals).slice(), 3));
    geometry.setAttribute("color", new THREE.BufferAttribute(floats(proto.colors).slice(), 3));
    geometry.setIndex(
      new THREE.BufferAttribute(
        new Uint16Array(indexBytes.buffer.slice(indexBytes.byteOffset, indexBytes.byteOffset + indexBytes.byteLength)),
        1,
      ),
    );
    geometry.computeBoundingSphere();
    geometries.push(geometry);
  }
  if (!geometries.length) {
    const cone = new THREE.ConeGeometry(3.8 * metresToScene, 18.0 * metresToScene, 5, 1);
    cone.translate(0, 9.0 * metresToScene, 0);
    const colours = new Float32Array(cone.attributes.position.count * 3);
    for (let i = 0; i < colours.length; i += 3) {
      colours[i] = 0.02;
      colours[i + 1] = 0.05;
      colours[i + 2] = 0.02;
    }
    cone.setAttribute("color", new THREE.BufferAttribute(colours, 3));
    geometries.push(cone);
  }
  const material = new THREE.MeshStandardMaterial({ vertexColors: true, roughness: 0.94, metalness: 0 });
  return {
    geometries,
    material,
    dispose() {
      for (const geometry of geometries) geometry.dispose();
      material.dispose();
    },
  };
}
