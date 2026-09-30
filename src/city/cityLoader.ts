/**
 * Getting a city scene from Rust into the renderer without a main-thread stall.
 *
 * The desktop app asks for the binary container (`city_scene_bin`): one raw
 * `ArrayBuffer` over IPC, a small JSON header, and typed-array *views* into the
 * buffer — no base64, no `JSON.parse` of a hundred-megabyte string, no copies.
 * The layout is defined by `city_scene::scene::encode_binary`.
 *
 * Scenes that arrive as JSON with base64 buffers (the audit fixtures) are still
 * accepted by every consumer; see `PayloadBuffer` in `cityScene.ts`.
 */

import type { CityScene, SceneInstances, SceneMesh, SceneTexture } from "./cityScene";
import { perfMeasure, perfTiming } from "./perf";

type Ref = [offset: number, bytes: number];

interface BinaryHeader extends Omit<CityScene, "meshes" | "instances" | "textures"> {
  meshes: Array<Omit<SceneMesh, "positions" | "normals" | "colors" | "uvs" | "indices"> & {
    positions: Ref;
    normals: Ref;
    colors?: Ref;
    uvs?: Ref;
    indices: Ref;
  }>;
  instances: Array<Omit<SceneInstances, "data"> & { data: Ref }>;
  textures: Array<Omit<SceneTexture, "data"> & { data: Ref }>;
}

/** Decode a `city_scene_bin` buffer. Views alias `buffer`; do not detach it. */
export function decodeCityBinary(buffer: ArrayBuffer): CityScene {
  const head = new DataView(buffer, 0, 8);
  const magic = String.fromCharCode(head.getUint8(0), head.getUint8(1), head.getUint8(2), head.getUint8(3));
  if (magic !== "CSB1") throw new Error(`not a city scene container (magic ${JSON.stringify(magic)})`);
  const headerLength = head.getUint32(4, true);
  const header = JSON.parse(
    new TextDecoder().decode(new Uint8Array(buffer, 8, headerLength)),
  ) as BinaryHeader;
  const base = Math.ceil((8 + headerLength) / 4) * 4;
  const f32 = ([offset, bytes]: Ref) => new Float32Array(buffer, base + offset, bytes / 4);
  const u32 = ([offset, bytes]: Ref) => new Uint32Array(buffer, base + offset, bytes / 4);
  const u8 = ([offset, bytes]: Ref) => new Uint8Array(buffer, base + offset, bytes);
  return {
    ...header,
    meshes: header.meshes.map((mesh) => ({
      ...mesh,
      positions: f32(mesh.positions),
      normals: u8(mesh.normals),
      colors: mesh.colors ? u8(mesh.colors) : undefined,
      uvs: mesh.uvs ? f32(mesh.uvs) : undefined,
      indices: u32(mesh.indices),
    })),
    instances: header.instances.map((list) => ({ ...list, data: f32(list.data) })),
    textures: header.textures.map((texture) => ({ ...texture, data: u8(texture.data) })),
  };
}

/** Fetch one city from the desktop shell and decode it. */
export async function loadCityScene(index: number): Promise<CityScene> {
  const { invoke } = await import("@tauri-apps/api/core");
  const started = performance.now();
  const raw = await invoke<ArrayBuffer | number[]>("city_scene_bin", { index });
  perfTiming("cityFetchMs", performance.now() - started);
  const buffer = raw instanceof ArrayBuffer ? raw : new Uint8Array(raw).buffer;
  return perfMeasure("cityDecodeMs", () => decodeCityBinary(buffer));
}
