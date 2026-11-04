/**
 * Dev-only automation of the real desktop app (see docs/real-app-verification.md).
 *
 * The Rust side reads `TERRAIN_LAB_AUTOMATION` and hands the document to
 * `automation_spec`; without it (every normal run, and the browser/harness pages,
 * where there is no Tauri bridge) `loadAutomation` resolves to null and nothing in
 * the app changes.
 */
import { invoke } from "@tauri-apps/api/core";

import type { SimulationConfig } from "./types";

export type ShotCamera = {
  /** City index: `pos`/`target` are city-local metres (x, height above ground, z). Absent: world km (x, height above ground in metres, y). */
  city?: number;
  pos: [number, number, number];
  target: [number, number, number];
  /** "walk" (eye height 1.7 m), "fly" (free roam, any height), "orbit" (default: orbit controls with the target on the ground). */
  mode?: "walk" | "fly" | "orbit";
};

export interface Automation {
  /** Overrides applied on top of the app's default simulation config. */
  config?: Partial<SimulationConfig>;
  /** Presentation: "3d" (default), "satellite", or "analysis" with `layer`. */
  view?: "3d" | "satellite" | "analysis";
  layer?: string;
  /** Jump to a city the way the "jump to city" menu does (0 = world view, 1 = first city...). */
  focus?: { option: number; pitchDeg?: number; azimuthDeg?: number; spanKm?: number };
  camera?: ShotCamera;
  /** Rendered frames to wait after the requested view is settled. */
  settleFrames?: number;
  /** Give up waiting for city tiles after this many seconds and report ready anyway. */
  tileTimeoutSec?: number;
  /** Milliseconds of tile building per frame (default 6); raise it under software GL, where one frame takes seconds. */
  tileBudgetMs?: number;
  /** Metres around the camera in which city tiles are streamed (default 600). */
  tileLoadM?: number;
  /** Close the app this long after reporting ready (ms); unset keeps it open. */
  exitAfterReadyMs?: number;
}

export async function loadAutomation(): Promise<Automation | null> {
  try {
    return (await invoke<Automation | null>("automation_spec")) ?? null;
  } catch (error) {
    if (typeof window !== "undefined" && "__TAURI_INTERNALS__" in window) {
      console.error("[automation]", error);
    }
    return null;
  }
}

export function reportAutomation(event: string, detail: unknown = {}): void {
  invoke("automation_event", { event, detail }).catch(() => undefined);
}

/** Mirror console output and uncaught errors into the automation log. */
export function forwardConsole(): void {
  for (const level of ["log", "info", "warn", "error"] as const) {
    const original = console[level].bind(console);
    console[level] = (...args: unknown[]) => {
      original(...args);
      reportAutomation("console", { level, text: args.map((arg) => (arg instanceof Error ? arg.stack ?? arg.message : typeof arg === "string" ? arg : JSON.stringify(arg))).join(" ").slice(0, 2000) });
    };
  }
  window.addEventListener("error", (event) => reportAutomation("console", { level: "uncaught", text: `${event.message} @ ${event.filename}:${event.lineno}` }));
  window.addEventListener("unhandledrejection", (event) => reportAutomation("console", { level: "unhandledrejection", text: String(event.reason) }));
}
