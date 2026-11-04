/**
 * The browser's stand-in for the desktop shell's Tauri commands: the same command
 * names and argument shapes, answered by the WebAssembly backend running in a
 * dedicated Web Worker. Nothing is fetched from a server and nothing is
 * pre-generated; every world and city is computed on this machine.
 */
import type { Command } from "./wasmHost";

type Progress = { stage: string; progress: number };
type Reply =
  | { id: number; ok: true; value?: unknown; bytes?: ArrayBuffer; elapsedMs?: number }
  | { id: number; ok: false; error: string };

/** What a command returns and in which form; mirrors src-tauri/src/lib.rs. */
const COMMANDS: Record<string, { command: keyof typeof Command; format: "json" | "bytes"; args?: (a: any) => unknown }> = {
  generate_terrain: { command: "Generate", format: "json", args: (a) => a.config },
  city_scene: { command: "CitySceneJson", format: "json" },
  city_scene_bin: { command: "CitySceneBin", format: "bytes" },
  city_tile_bin: { command: "CityTileBin", format: "bytes" },
  traffic_step: { command: "TrafficStep", format: "json" },
  traffic_capacities: { command: "TrafficCapacities", format: "json" },
};

const progressListeners = new Set<(event: Progress) => void>();

export function onProgress(listener: (event: Progress) => void): () => void {
  progressListeners.add(listener);
  return () => progressListeners.delete(listener);
}

let worker: Worker | null = null;
let ready: Promise<void> | null = null;
let nextId = 1;
const pending = new Map<number, { resolve: (reply: Reply) => void; reject: (error: Error) => void }>();

function spawn(): Promise<void> {
  worker = new Worker(new URL("./backendWorker.ts", import.meta.url), { type: "module" });
  worker.onmessage = (event: MessageEvent<any>) => {
    const message = event.data;
    if (message.progress) progressListeners.forEach((listener) => listener(message.progress));
    else if (message.log) console.error("[backend]", message.log);
    else {
      const entry = pending.get(message.id);
      if (!entry) return;
      pending.delete(message.id);
      entry.resolve(message as Reply);
    }
  };
  worker.onerror = (event) => {
    const error = new Error(event.message || "the backend worker failed");
    pending.forEach((entry) => entry.reject(error));
    pending.clear();
    reset();
  };
  const wasmUrl = new URL(`${import.meta.env.BASE_URL}web_backend.wasm`, location.href).href;
  return send({ kind: "init", wasmUrl }).then(() => undefined);
}

function reset() {
  worker?.terminate();
  worker = null;
  ready = null;
}

function send(message: Record<string, unknown>, transfer: Transferable[] = []): Promise<Reply> {
  return new Promise<Reply>((resolve, reject) => {
    const id = nextId++;
    pending.set(id, {
      resolve: (reply) => (reply.ok ? resolve(reply) : reject(new Error(reply.error))),
      reject,
    });
    worker!.postMessage({ id, ...message }, transfer);
  });
}

/** After a trap the module's state is unusable (locks may be held); start over on the next call. */
const isTrap = (error: Error) => /unreachable|out of bounds|memory access|stack/i.test(error.message);

async function run(name: string, args: Record<string, any> = {}): Promise<{ value?: any; bytes?: ArrayBuffer }> {
  const spec = COMMANDS[name];
  if (!spec) throw new Error(`the browser build has no command "${name}"`);
  if (!ready) ready = spawn();
  try {
    await ready;
    const reply = (await send({
      kind: "call",
      command: spec.command,
      request: spec.args ? spec.args(args) : args,
      format: spec.format,
    })) as Extract<Reply, { ok: true }>;
    if (name === "generate_terrain" && reply.value && reply.elapsedMs !== undefined) {
      // The module has no clock of its own; report the worker's measurement.
      (reply.value as { elapsedMs: number }).elapsedMs = Math.round(reply.elapsedMs);
    }
    return { value: reply.value, bytes: reply.bytes };
  } catch (error) {
    if (error instanceof Error && isTrap(error)) reset();
    throw error;
  }
}

/** The Tauri `invoke` of the browser build. */
export async function webInvoke<T>(name: string, args: Record<string, any> = {}): Promise<T> {
  const { value, bytes } = await run(name, args);
  return (spec(name).format === "bytes" ? bytes : value) as T;
}

const spec = (name: string) => COMMANDS[name];

/** Render the satellite export at `size` px and hand it to the browser as a download. */
export async function exportPng(config: unknown, fileName: string, size: number): Promise<string> {
  const bytes = await rawCall("ExportPng", { config, size });
  download(new Blob([bytes], { type: "image/png" }), fileName);
  return fileName;
}

export async function projectToFile(project: unknown, fileName: string): Promise<string> {
  const bytes = await rawCall("ProjectToBytes", project);
  download(new Blob([bytes], { type: "application/json" }), fileName);
  return fileName;
}

export async function projectFromBytes<T>(bytes: Uint8Array): Promise<T> {
  const copy = bytes.slice();
  if (!ready) ready = spawn();
  await ready;
  const reply = (await send(
    { kind: "call", command: "ProjectFromBytes", request: copy, format: "json" },
    [copy.buffer],
  )) as Extract<Reply, { ok: true }>;
  return reply.value as T;
}

async function rawCall(command: keyof typeof Command, request: unknown): Promise<ArrayBuffer> {
  if (!ready) ready = spawn();
  await ready;
  const reply = (await send({ kind: "call", command, request, format: "bytes" })) as Extract<Reply, { ok: true }>;
  return reply.bytes!;
}

function download(blob: Blob, fileName: string) {
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = fileName;
  document.body.appendChild(link);
  link.click();
  link.remove();
  setTimeout(() => URL.revokeObjectURL(url), 10_000);
}
