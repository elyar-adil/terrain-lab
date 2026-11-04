/// <reference lib="webworker" />
/**
 * The Web Worker that owns the WebAssembly backend. Every command runs here, off
 * the page's main thread, so a generation that takes tens of seconds never freezes
 * the UI; progress is posted as it is reported.
 */
import { Command, WasmHost } from "./wasmHost";

type Request =
  | { id: number; kind: "init"; wasmUrl: string }
  | { id: number; kind: "call"; command: keyof typeof Command; request: unknown; format: "json" | "bytes" };

let host: WasmHost | null = null;

const post = (message: unknown, transfer: Transferable[] = []) =>
  (self as unknown as Worker).postMessage(message, transfer);

self.onmessage = async (event: MessageEvent<Request>) => {
  const message = event.data;
  try {
    if (message.kind === "init") {
      host = await WasmHost.instantiate(fetch(message.wasmUrl), (line) => post({ log: line }));
      post({ id: message.id, ok: true });
      return;
    }
    if (!host) throw new Error("the backend module is not loaded");
    const started = performance.now();
    const bytes = host.call(Command[message.command], message.request, (fraction, stage) =>
      post({ progress: { progress: fraction, stage } }),
    );
    const elapsedMs = performance.now() - started;
    if (message.format === "bytes") {
      post({ id: message.id, ok: true, bytes: bytes.buffer, elapsedMs }, [bytes.buffer]);
    } else {
      const value = JSON.parse(new TextDecoder().decode(bytes));
      post({ id: message.id, ok: true, value, elapsedMs });
    }
  } catch (error) {
    post({ id: message.id, ok: false, error: error instanceof Error ? error.message : String(error) });
  }
};
