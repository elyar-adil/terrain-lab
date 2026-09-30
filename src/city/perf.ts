/**
 * Lightweight performance counters, active only with `?perf=1`.
 *
 * Everything is a no-op otherwise, so call sites need no guards. Numbers are
 * published on `window.__PERF__` (for the headless harness) and drawn in a small
 * overlay. Nothing here allocates per frame beyond a few numbers.
 */

import type * as THREE from "three";

export interface PerfCounters {
  frames: number;
  /** Exponential moving average of wall-clock frame time, ms. */
  frameMs: number;
  /** Worst frame time (slowly decaying), ms. */
  worstFrameMs: number;
  /** Time spent inside the render call (CPU submit + software raster under SwiftShader), ms. */
  renderCpuMs: number;
  drawCalls: number;
  triangles: number;
  geometries: number;
  textures: number;
  /** Named timings: `jsonParseMs`, `cityBuildMs`, `cityBuildSliceMaxMs` ... */
  timings: Record<string, number>;
}

declare global {
  interface Window {
    __PERF__?: PerfCounters;
  }
}

export const perfEnabled: boolean = (() => {
  try {
    return typeof window !== "undefined" && new URLSearchParams(window.location.search).get("perf") === "1";
  } catch {
    return false;
  }
})();

const counters: PerfCounters = {
  frames: 0,
  frameMs: 0,
  worstFrameMs: 0,
  renderCpuMs: 0,
  drawCalls: 0,
  triangles: 0,
  geometries: 0,
  textures: 0,
  timings: {},
};
if (perfEnabled && typeof window !== "undefined") window.__PERF__ = counters;

/** Record a named timing (keeps the maximum for `*MaxMs` keys, else overwrites). */
export function perfTiming(name: string, ms: number): void {
  if (!perfEnabled) return;
  if (name.endsWith("MaxMs")) counters.timings[name] = Math.max(counters.timings[name] ?? 0, ms);
  else counters.timings[name] = ms;
}

/** Time `work` and record it under `name`. */
export function perfMeasure<T>(name: string, work: () => T): T {
  if (!perfEnabled) return work();
  const start = performance.now();
  try {
    return work();
  } finally {
    perfTiming(name, performance.now() - start);
  }
}

export interface FramePerf {
  /** Call immediately before rendering. */
  begin(): void;
  /** Call immediately after rendering. */
  end(): void;
  dispose(): void;
}

/** Instrument a renderer. `info.autoReset` is turned off so multi-pass frames sum. */
export function createFramePerf(renderer: THREE.WebGLRenderer): FramePerf {
  if (!perfEnabled) return { begin() {}, end() {}, dispose() {} };
  renderer.info.autoReset = false;
  const overlay = document.createElement("div");
  overlay.style.cssText =
    "position:fixed;left:8px;top:8px;z-index:99999;padding:4px 8px;background:rgba(0,0,0,.65);" +
    "color:#9f9;font:11px/1.35 ui-monospace,monospace;white-space:pre;pointer-events:none;border-radius:4px";
  document.body.appendChild(overlay);
  let last = 0;
  let begun = 0;
  return {
    begin() {
      renderer.info.reset();
      begun = performance.now();
    },
    end() {
      const now = performance.now();
      const cpu = now - begun;
      counters.renderCpuMs = counters.renderCpuMs === 0 ? cpu : counters.renderCpuMs * 0.9 + cpu * 0.1;
      if (last > 0) {
        const dt = now - last;
        counters.frameMs = counters.frameMs === 0 ? dt : counters.frameMs * 0.9 + dt * 0.1;
        counters.worstFrameMs = Math.max(counters.worstFrameMs * 0.995, dt);
      }
      last = now;
      counters.frames += 1;
      counters.drawCalls = renderer.info.render.calls;
      counters.triangles = renderer.info.render.triangles;
      counters.geometries = renderer.info.memory.geometries;
      counters.textures = renderer.info.memory.textures;
      if (counters.frames % 15 === 0) {
        const t = counters.timings;
        overlay.textContent =
          `frame ${counters.frameMs.toFixed(1)} ms (worst ${counters.worstFrameMs.toFixed(0)})  render ${counters.renderCpuMs.toFixed(1)}\n` +
          `draws ${counters.drawCalls}  tris ${(counters.triangles / 1000).toFixed(0)}k  geo ${counters.geometries}  tex ${counters.textures}\n` +
          Object.entries(t).map(([k, v]) => `${k} ${v.toFixed(0)}`).join("  ");
      }
    },
    dispose() {
      overlay.remove();
    },
  };
}
