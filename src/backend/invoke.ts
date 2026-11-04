/**
 * One `invoke` for both hosts. The desktop shell answers through Tauri's IPC; a
 * plain browser answers from the WebAssembly backend (see webBackend.ts).
 */
export const isDesktop = (): boolean => typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export async function invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (isDesktop()) {
    const { invoke: tauriInvoke } = await import("@tauri-apps/api/core");
    return tauriInvoke<T>(command, args);
  }
  const { webInvoke } = await import("./webBackend");
  return webInvoke<T>(command, args);
}
