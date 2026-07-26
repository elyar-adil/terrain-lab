import { invoke } from "@tauri-apps/api/core";
import type { GenerationResult, ProjectDocument, SimulationConfig } from "./types";

export async function generateTerrain(config: SimulationConfig): Promise<GenerationResult> {
  return invoke<GenerationResult>("generate_terrain", { config });
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
