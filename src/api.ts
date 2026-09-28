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
