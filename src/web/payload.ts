// Resolved by scripts/vite.single.config.ts: the world, the city scene and the
// tree grower, each gzip + base64, produced by scripts/build-single.mjs.
import payload from "virtual:single-payload";

async function gunzip(base64: string): Promise<Uint8Array> {
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i += 1) bytes[i] = binary.charCodeAt(i);
  const stream = new Blob([bytes]).stream().pipeThrough(new DecompressionStream("gzip"));
  return new Uint8Array(await new Response(stream).arrayBuffer());
}

export async function loadPayloads() {
  const [fixtureBytes, cityBytes] = await Promise.all([gunzip(payload.fixture), gunzip(payload.city)]);
  const decoder = new TextDecoder();
  return {
    fixture: JSON.parse(decoder.decode(fixtureBytes)),
    city: (([c]) => c)([JSON.parse(decoder.decode(cityBytes))].flat()),
  };
}
