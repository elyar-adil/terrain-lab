import { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";

import { CityViewer } from "./city/CityViewer";
import type { CityScene } from "./city/cityScene";
import { preloadTreeGrower } from "./trees";
import "./styles.css";

declare global {
  interface Window {
    __RENDER_READY__?: boolean;
  }
}

/**
 * The city audit harness.
 *
 * The product renderer animates at display refresh rate. Under SwiftShader, the
 * software rasteriser the audit runs on, letting it run free turns a three-frame
 * smoke test into hundreds of expensive frames, so the loop is throttled. The
 * simulation clock still advances in real elapsed time, so the traffic and signal
 * animation are still verified rather than frozen.
 */
const params = new URLSearchParams(window.location.search);
const requestedFps = Number(params.get("fps"));
const framesPerSecond = Number.isFinite(requestedFps)
  ? Math.min(30, Math.max(1, requestedFps))
  : 6;
const frameMs = 1000 / framesPerSecond;
window.requestAnimationFrame = (callback: FrameRequestCallback) =>
  window.setTimeout(() => callback(performance.now()), frameMs);
window.cancelAnimationFrame = (handle: number) => window.clearTimeout(handle);

function Harness() {
  const [scenes, setScenes] = useState<CityScene[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const city = Number(params.get("city") ?? "0");
  const preset = params.get("preset") ?? "street";

  useEffect(() => {
    let cancelled = false;
    // The fixture path is overridable so a component agent can render its own
    // scratch scene (`?scene=audit-trees.json`) without clobbering the shared
    // city fixture another audit may be reading.
    const sceneUrl = params.get("scene") ?? "/city-scenes.json";
    // The tree grower is loaded first, so the city is built with its trees.
    preloadTreeGrower()
      .then(() => fetch(sceneUrl))
      .then((response) => {
        if (!response.ok) throw new Error(`${response.status} ${response.statusText}`);
        return response.json();
      })
      .then((data: CityScene[]) => {
        if (cancelled) return;
        setScenes(Array.isArray(data) ? data : [data]);
      })
      .catch((thrown: Error) => {
        if (!cancelled) setError(`${sceneUrl}: ${thrown.message}`);
      });
    return () => {
      cancelled = true;
    };
  }, []);

  if (error) {
    return (
      <pre style={{ color: "#ff6b6b", font: "13px ui-monospace, monospace", padding: 24 }}>
        {`could not load /city-scenes.json\n\n${error}\n\ngenerate it with:\n  cargo run --release -p wind-water-terrain-lab --example tiny_city`}
      </pre>
    );
  }

  const scene = scenes?.[Math.max(0, Math.min(scenes.length - 1, city))];
  if (!scene) return null;

  return (
    <CityViewer
      scene={scene}
      preset={preset}
      onReady={() => {
        // The audit waits on the renderer's own flag rather than a fixed delay,
        // because under software rendering a fixed delay captures a frame that
        // has not been composed yet.
        window.__RENDER_READY__ = true;
        document.documentElement.dataset.renderReady = "true";
      }}
    />
  );
}

// StrictMode mounts effects twice in development, which duplicates every WebGL
// allocation here and adds no coverage to a screenshot.
createRoot(document.getElementById("render-root")!).render(<Harness />);
