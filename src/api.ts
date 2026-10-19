import { invoke } from "@tauri-apps/api/core";
import type { CityScene } from "./city/cityScene";
import type { GenerationResult, ProjectDocument, SimulationConfig } from "./types";

export async function generateTerrain(config: SimulationConfig): Promise<GenerationResult> {
  return invoke<GenerationResult>("generate_terrain", { config });
}

/**
 * One city's finished scene, fetched when the metre-scale view is entered.
 *
 * Deliberately a separate call. A scene is roughly 160 MB of base64 vertex
 * buffers, so returning three of them from `generate_terrain` meant serialising
 * several hundred megabytes of JSON across the IPC boundary into a webview that
 * did not read it — which killed the webview, and read as "generate crashes the
 * app". Serialisation is the expensive half, not the build, so paying it once per
 * city actually shown is the difference between working and not.
 */
/**
 * One square tile of a city's detail. The desktop shell builds it on request; in the
 * web harness the tile is read from `/city-tiles/<index>-<column>-<row>.json`.
 */
export async function loadCityTile(index: number, column: number, row: number): Promise<CityScene> {
  try {
    const { decodeCityBinary } = await import("./city/cityLoader");
    const raw = await invoke<ArrayBuffer | number[]>("city_tile_bin", { index, column, row });
    const buffer = raw instanceof ArrayBuffer ? raw : new Uint8Array(raw).buffer;
    return decodeCityBinary(buffer);
  } catch (error) {
    const response = await fetch(`/city-tiles/${index}-${column}-${row}.json`);
    if (!response.ok) throw error;
    const [scene] = (await response.json()) as CityScene[];
    return scene;
  }
}

export async function fetchCityScene(index: number): Promise<CityScene> {
  return invoke<CityScene>("city_scene", { index });
}

export async function exportTerrain(
  config: SimulationConfig,
  outputPath: string,
  outputSize: number,
): Promise<string> {
  return invoke<string>("export_terrain", { config, outputPath, outputSize });
}

export async function saveProject(project: ProjectDocument, outputPath: string): Promise<string> {
  return invoke<string>("save_project", { project, outputPath });
}

export async function loadProject(inputPath: string): Promise<ProjectDocument> {
  return invoke<ProjectDocument>("load_project", { inputPath });
}
