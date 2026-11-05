import { invoke, isDesktop } from "./backend/invoke";
import { isConstrainedDevice } from "./platform";
import type { CityScene } from "./city/cityScene";
import type { GenerationResult, ProjectDocument, SimulationConfig } from "./types";

let previewUrls: string[] = [];

export async function generateTerrain(config: SimulationConfig): Promise<GenerationResult> {
  const result = await invoke<GenerationResult>("generate_terrain", { config });
  if (isConstrainedDevice) await moveAnalysisPreviewsToBlobs(result);
  return result;
}

/**
 * The sixteen analysis images arrive as PNG data URLs (about 1.4 MB of strings at
 * 256 and 5 MB at 512) that are only shown when the analysis tab is open. On a phone
 * they are moved into blobs, so the page keeps short URLs and the bytes live outside
 * the JS heap.
 */
async function moveAnalysisPreviewsToBlobs(result: GenerationResult): Promise<void> {
  for (const url of previewUrls) URL.revokeObjectURL(url);
  previewUrls = [];
  const previews = result.analysisPreviews as Record<string, string>;
  for (const key of Object.keys(previews)) {
    const value = previews[key];
    if (typeof value !== "string" || !value.startsWith("data:")) continue;
    try {
      const blob = await (await fetch(value)).blob();
      const url = URL.createObjectURL(blob);
      previewUrls.push(url);
      previews[key] = url;
    } catch {
      // Keep the data URL: slower to hold, still correct.
    }
  }
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
  if (!isDesktop()) {
    // The browser has no file system: the "path" is the download's file name.
    const { exportPng } = await import("./backend/webBackend");
    return exportPng(config, outputPath, outputSize);
  }
  return invoke<string>("export_terrain", { config, outputPath, outputSize });
}

export async function saveProject(project: ProjectDocument, outputPath: string): Promise<string> {
  if (!isDesktop()) {
    const { projectToFile } = await import("./backend/webBackend");
    return projectToFile(project, outputPath);
  }
  return invoke<string>("save_project", { project, outputPath });
}

export async function loadProject(inputPath: string): Promise<ProjectDocument> {
  if (!isDesktop()) {
    // `inputPath` is the token the browser file picker returned (see platform.ts).
    const { pickedFiles } = await import("./platform");
    const file = pickedFiles.get(inputPath);
    if (!file) throw new Error("the chosen project file is no longer available");
    const { projectFromBytes } = await import("./backend/webBackend");
    return projectFromBytes<ProjectDocument>(new Uint8Array(await file.arrayBuffer()));
  }
  return invoke<ProjectDocument>("load_project", { inputPath });
}
