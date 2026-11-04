/**
 * The WebAssembly backend module (`crates/web-backend`) behind a typed, synchronous
 * call. It has no DOM dependency: the browser runs it inside a Web Worker
 * (`backendWorker.ts`), and the Node smoke test (`scripts/web-smoke.mjs`) runs the
 * same wire format. See docs/web-backend.md for the ABI.
 */

export const Command = {
  Generate: 0,
  CitySceneBin: 1,
  CityTileBin: 2,
  TrafficStep: 3,
  TrafficCapacities: 4,
  ExportPng: 5,
  ProjectToBytes: 6,
  ProjectFromBytes: 7,
  CitySceneJson: 8,
} as const;
export type CommandId = (typeof Command)[keyof typeof Command];

interface Exports {
  memory: WebAssembly.Memory;
  alloc(length: number): number;
  dealloc(pointer: number, length: number): void;
  result_ptr(): number;
  result_len(): number;
  result_free(): void;
  init(): void;
  call(command: number, pointer: number, length: number): number;
}

export type ProgressHandler = (fraction: number, stage: string) => void;

export class WasmHost {
  private constructor(
    private readonly exports: Exports,
    private readonly state: { progress: ProgressHandler; log: (line: string) => void },
  ) {}

  static async instantiate(
    source: BufferSource | Promise<Response>,
    log: (line: string) => void = (line) => console.error(line),
  ): Promise<WasmHost> {
    const state = { progress: (() => {}) as ProgressHandler, log };
    let memory: WebAssembly.Memory | undefined;
    const text = (pointer: number, length: number) =>
      new TextDecoder().decode(new Uint8Array(memory!.buffer, pointer, length));
    const imports = {
      env: {
        host_progress: (fraction: number, pointer: number, length: number) =>
          state.progress(fraction, text(pointer, length)),
        host_log: (pointer: number, length: number) => state.log(text(pointer, length)),
      },
    };
    const { instance } =
      source instanceof Promise
        ? await WebAssembly.instantiateStreaming(source, imports)
        : await WebAssembly.instantiate(source, imports);
    const exports = instance.exports as unknown as Exports;
    memory = exports.memory;
    exports.init();
    return new WasmHost(exports, state);
  }

  /** Run one command. Returns the raw result bytes (a copy); throws the module's error message. */
  call(command: CommandId, request: unknown, onProgress?: ProgressHandler): Uint8Array {
    const input =
      request instanceof Uint8Array ? request : new TextEncoder().encode(JSON.stringify(request ?? {}));
    // Pointers above 2 GiB come back from wasm as negative i32s.
    const pointer = this.exports.alloc(input.length) >>> 0;
    new Uint8Array(this.exports.memory.buffer, pointer, input.length).set(input);
    this.state.progress = onProgress ?? (() => {});
    let status: number;
    try {
      status = this.exports.call(command, pointer, input.length);
    } finally {
      this.state.progress = () => {};
      this.exports.dealloc(pointer, input.length);
    }
    // Copy out before freeing: memory growth detaches views and the next call overwrites.
    const bytes = new Uint8Array(this.exports.memory.buffer, this.exports.result_ptr() >>> 0, this.exports.result_len() >>> 0).slice();
    this.exports.result_free();
    if (status !== 0) throw new Error(new TextDecoder().decode(bytes));
    return bytes;
  }

  callJson<T>(command: CommandId, request: unknown, onProgress?: ProgressHandler): T {
    return JSON.parse(new TextDecoder().decode(this.call(command, request, onProgress))) as T;
  }
}
