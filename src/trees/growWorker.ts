/// <reference lib="webworker" />
/**
 * Grows trees off the main thread. Each worker loads its own copy of the grower and
 * answers one request at a time; the result's buffers are handed back, not copied.
 */
import { TreeGrower, type TreeSpec } from "./wasm";

interface Request {
  id: number;
  spec: TreeSpec;
  lod: number;
}

let grower: Promise<TreeGrower> | null = null;

self.onmessage = async (event: MessageEvent<{ init: string } | Request>) => {
  const message = event.data;
  if ("init" in message) {
    grower = TreeGrower.load(message.init);
    return;
  }
  try {
    const g = await grower!;
    const data = g.grow(message.spec, message.lod);
    // The views all sit on one buffer: send it once.
    (self as unknown as Worker).postMessage({ id: message.id, data }, [data.segments.buffer]);
  } catch (error) {
    (self as unknown as Worker).postMessage({ id: message.id, error: String(error) });
  }
};
