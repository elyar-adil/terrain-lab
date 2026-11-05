/**
 * The few things the app asks of its host besides the backend commands: progress
 * events and file pickers. On the desktop these are Tauri's event bus and native
 * dialogs; in a browser, progress comes from the backend worker, "save" resolves to
 * the file name the browser will download, and "open" is the browser's file picker.
 */
import { isDesktop } from "./backend/invoke";

type SaveOptions = { title?: string; defaultPath?: string; filters?: { name: string; extensions: string[] }[] };
type OpenOptions = SaveOptions & { multiple?: boolean; directory?: boolean };

export async function listen<T>(event: string, handler: (event: { payload: T }) => void): Promise<() => void> {
  if (isDesktop()) {
    const { listen: tauriListen } = await import("@tauri-apps/api/event");
    return tauriListen<T>(event, handler);
  }
  const { onProgress } = await import("./backend/webBackend");
  return onProgress((payload) => handler({ payload: payload as unknown as T }));
}

export async function save(options: SaveOptions): Promise<string | null> {
  if (isDesktop()) {
    const { save: tauriSave } = await import("@tauri-apps/plugin-dialog");
    return tauriSave(options);
  }
  return options.defaultPath ?? "download";
}

/** Files the user picked in the browser, by the token `open` returned for them. */
export const pickedFiles = new Map<string, File>();

export async function open(options: OpenOptions): Promise<string | null> {
  if (isDesktop()) {
    const { open: tauriOpen } = await import("@tauri-apps/plugin-dialog");
    const picked = await tauriOpen(options);
    return Array.isArray(picked) ? (picked[0] ?? null) : picked;
  }
  return new Promise<string | null>((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    const extensions = options.filters?.flatMap((filter) => filter.extensions.map((e) => `.${e}`)) ?? [];
    if (extensions.length) input.accept = extensions.join(",");
    input.onchange = () => {
      const file = input.files?.[0];
      if (!file) return resolve(null);
      const token = `browser-file:${file.name}`;
      pickedFiles.set(token, file);
      resolve(token);
    };
    input.oncancel = () => resolve(null);
    input.click();
  });
}

/**
 * Phones and tablets: a browser tab there is killed at roughly 1-1.5 GB, so the
 * browser build starts with a small grid and a light render path (no MSAA, no
 * ambient-occlusion pass, pixel ratio 1). `?mobile=0` / `?mobile=1` override.
 */
export const isConstrainedDevice: boolean = (() => {
  try {
    const forced = new URLSearchParams(window.location.search).get("mobile");
    if (forced === "0") return false;
    if (forced === "1") return true;
    const ua = navigator.userAgent;
    return /iPhone|iPad|iPod|Android/i.test(ua) || (/Macintosh/.test(ua) && navigator.maxTouchPoints > 1);
  } catch {
    return false;
  }
})();

export const LIGHT_MODE_NOTICE = "该设备内存有限，已使用轻量模式";

/**
 * A tab the system killed while the 3D scene was mounting cannot say so itself.
 * The scene writes a marker before it builds and clears it once frames are being
 * drawn; finding the marker on the next load means the last build was fatal, and
 * the page then starts in safe mode (no city silhouettes, no trees, no shadow
 * map) instead of repeating it. `?safe=0` clears it, `?safe=1` forces it.
 */
const MOUNT_MARKER = "terrainLab.mounting";
const SAFE_MARKER = "terrainLab.safeMode";
const storage = (): Storage | null => {
  try {
    return window.localStorage;
  } catch {
    return null;
  }
};

export const crashedLastTime: boolean = (() => {
  try {
    const store = storage();
    const forced = new URLSearchParams(window.location.search).get("safe");
    if (forced === "0") {
      store?.removeItem(SAFE_MARKER);
      store?.removeItem(MOUNT_MARKER);
      return false;
    }
    if (forced === "1") return true;
    if (!isConstrainedDevice || !store) return false;
    if (store.getItem(MOUNT_MARKER)) {
      store.removeItem(MOUNT_MARKER);
      store.setItem(SAFE_MARKER, "1");
    }
    return store.getItem(SAFE_MARKER) === "1";
  } catch {
    return false;
  }
})();

/** Phones start light; a phone whose tab died during a 3D mount also drops the heavy layers. */
export const isSafeMode: boolean = crashedLastTime;

export function markMountStarted(): void {
  if (isConstrainedDevice) storage()?.setItem(MOUNT_MARKER, String(Date.now()));
}

export function markMountFinished(): void {
  storage()?.removeItem(MOUNT_MARKER);
}
