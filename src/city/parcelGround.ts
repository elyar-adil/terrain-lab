/**
 * What the open ground of a large parcel is made of, by use, as one GLSL
 * function shared by the far form (`cityMass.ts`, where the kind rides in the
 * vertex seed) and the metre-scale scene (`cityScene.ts`, one material per kind).
 *
 * Real references, not decoration:
 *
 * - Concrete yard (industrial hardstanding, container and goods yards): jointed
 *   slabs in 5 x 5 m bays with a recessed joint, uneven slab tone, oil stains and
 *   wheel wear, and stacks of 20 ft / 40 ft ISO containers (2.44 m wide, 6.06 /
 *   12.19 m long) in the colours of the box lines.
 * - Car park (commercial, mixed use): 2.5 x 5.3 m bays on a 6 m aisle (JGJ 100
 *   parking design for a small car), white painted bay lines, parked cars at
 *   about 55 % occupancy.
 * - Asphalt lot (industrial and service yards): aged bituminous surface, oil
 *   stains, patch repairs, painted yellow lane lines.
 * - Plaza (civic, forecourt): 0.6 m granite paving, an expansion joint every 6 m,
 *   planting beds.
 * - Lawn (residential, park): mown in 6 m bands, patchy.
 *
 * Every pattern is anti-aliased by the pixel footprint `fw` (metres per pixel):
 * a joint a third of a pixel wide is a darker tone, not a dotted line.
 */

/** The kind codes. Vertex seeds 0..3 are the facade kinds of the mass shader. */
export const GROUND_KIND = {
  yard: 4,
  parking: 5,
  asphaltLot: 6,
  plaza: 7,
  lawn: 8,
  /** Car-park asphalt with no painted bays or cars: the detail scene lays those as geometry. */
  carParkBase: 9,
} as const;

export const GROUND_GLSL = /* glsl */ `
float gHash(vec2 p) { return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453); }
float gNoise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(gHash(i), gHash(i + vec2(1.0, 0.0)), f.x),
             mix(gHash(i + vec2(0.0, 1.0)), gHash(i + vec2(1.0, 1.0)), f.x), f.y);
}
// Coverage of a line w metres wide every period metres, fading to its mean
// share once the line is under a pixel.
float gLine(float c, float period, float w, float fw) {
  float d = abs(fract(c / period + 0.5) - 0.5) * period;
  float cov = clamp((w * 0.5 + fw * 0.5 - d) / max(fw, 1e-4), 0.0, 1.0);
  float mean = min(1.0, w / period);
  return mix(cov, mean, smoothstep(0.3 * period, 0.7 * period, fw));
}
float gBox(vec2 p, vec2 lo, vec2 hi) {
  vec2 s = step(lo, p) * step(p, hi);
  return s.x * s.y;
}
vec3 gContainer(float h) {
  if (h < 0.18) return vec3(0.46, 0.17, 0.12);
  if (h < 0.34) return vec3(0.12, 0.22, 0.38);
  if (h < 0.48) return vec3(0.16, 0.33, 0.24);
  if (h < 0.60) return vec3(0.62, 0.36, 0.10);
  if (h < 0.74) return vec3(0.50, 0.52, 0.52);
  if (h < 0.86) return vec3(0.40, 0.30, 0.20);
  return vec3(0.70, 0.68, 0.62);
}
vec3 groundSurface(float kind, vec3 base, vec2 p, float fw, float seed) {
  vec2 q = p + seed * 97.0;
  float low = gNoise(q / 31.0);
  float mid = gNoise(q / 6.3 + 7.0);
  float fine = gNoise(q / 0.45);
  float grain = 1.0 - smoothstep(0.15, 0.6, fw);
  vec3 c = base * (0.88 + 0.24 * low) * (0.94 + 0.12 * mid);
  c *= mix(1.0, 0.92 + 0.16 * fine, grain);
  if (kind < 4.5) {
    // Concrete yard: slab tone, joints, wheel wear, oil, container stacks.
    vec2 cell = floor(q / 5.0);
    float tone = 0.92 + 0.16 * gHash(cell + 3.0);
    float joint = max(gLine(q.x, 5.0, 0.03, fw), gLine(q.y, 5.0, 0.03, fw));
    c *= tone * (1.0 - 0.5 * joint);
    float lane = smoothstep(0.35, 0.5, gNoise(vec2(q.x / 55.0, q.y / 2.2)));
    c *= 1.0 - 0.10 * lane * (1.0 - smoothstep(0.5, 3.0, fw));
    float oil = smoothstep(0.62, 0.9, gNoise(q / 2.6 + 11.0));
    c *= 1.0 - 0.14 * oil * (1.0 - smoothstep(0.8, 3.0, fw));
    vec2 blockSize = vec2(42.0, 30.0);
    vec2 block = floor(q / blockSize);
    vec2 local = q - block * blockSize;
    float stacked = step(gHash(block + seed * 13.0), 0.33);
    if (stacked > 0.5) {
      vec2 rel = vec2((local.x - 8.0) / 12.19, (local.y - 7.0) / 2.59);
      float inside = gBox(rel, vec2(0.0, 0.0), vec2(2.0, 6.0));
      vec2 box = floor(rel);
      float h = gHash(box + block * 7.0);
      vec2 inBox = fract(rel);
      float gap = 1.0 - gBox(inBox, vec2(0.012, 0.03), vec2(0.988, 0.97));
      vec3 box3 = gContainer(h) * (0.85 + 0.3 * gHash(box + 5.0));
      // Corrugation ribs along the side, 0.26 m pitch.
      box3 *= 1.0 - 0.10 * grain * step(0.5, fract(q.y / 0.26));
      box3 *= 1.0 - 0.25 * gap;
      float showBoxes = 1.0 - smoothstep(0.4, 2.4, fw);
      vec3 meanC = mix(c, vec3(0.42, 0.34, 0.28), 0.09);
      c = mix(meanC, mix(c, box3, inside), showBoxes);
    }
  } else if (kind < 5.5) {
    // (Far form only: the metre-scale scene uses carParkBase.)
    // Car park: 16.6 m double-loaded module along z, 2.5 m bays along x.
    float v = mod(q.y, 16.6);
    float inStall = 1.0 - step(5.3, v) * (1.0 - step(11.3, v));
    float depth = v < 5.3 ? v : 16.6 - v;
    float bayX = q.x / 2.5;
    float paint = gLine(q.x, 2.5, 0.1, fw) * inStall;
    float endA = 1.0 - smoothstep(0.05, 0.05 + max(fw, 0.02), abs(v - 5.3));
    float endB = 1.0 - smoothstep(0.05, 0.05 + max(fw, 0.02), abs(v - 11.3));
    paint = max(paint, max(endA, endB) * 0.9);
    c *= 1.0 - 0.10 * (1.0 - inStall) * gNoise(vec2(q.x / 9.0, q.y / 2.0));
    float row = floor(q.y / 16.6) + (v > 8.3 ? 0.5 : 0.0);
    float oil = smoothstep(0.62, 0.78, gNoise(vec2(floor(bayX), row) * 3.1 + q / 1.6)) * inStall;
    c *= 1.0 - 0.28 * oil * (1.0 - smoothstep(0.5, 3.0, fw));
    float sid = gHash(vec2(floor(bayX), row) + seed * 5.0);
    float occupied = step(sid, 0.55) * inStall;
    float across = fract(bayX);
    float car = occupied * gBox(vec2(across, depth), vec2(0.14, 0.45), vec2(0.86, 4.85));
    float tint = gHash(vec2(floor(bayX), row) + 9.0);
    vec3 paintC = tint < 0.30 ? vec3(0.62) : tint < 0.50 ? vec3(0.05) : tint < 0.72 ? vec3(0.32, 0.33, 0.35) : tint < 0.84 ? vec3(0.42, 0.10, 0.08) : vec3(0.10, 0.16, 0.30);
    float glass = gBox(vec2(across, depth), vec2(0.2, 1.3), vec2(0.8, 3.6));
    vec3 carC = mix(paintC, vec3(0.05, 0.06, 0.07), glass * 0.65);
    float showCar = 1.0 - smoothstep(0.5, 2.2, fw);
    vec3 meanCar = mix(c, vec3(0.32), 0.19 * inStall);
    c = mix(meanCar, mix(c, carC, car), showCar);
    c = mix(c, vec3(0.78, 0.77, 0.70), paint * 0.8);
  } else if (kind < 6.5) {
    // Asphalt lot: oil, patches, cracks, painted lane lines.
    // Oil drips collect in rounded stains, soft at the edge, darkest at the core.
    float oil = smoothstep(0.58, 0.95, gNoise(q / 3.1 + 31.0)) * (0.6 + 0.4 * gNoise(q / 0.5));
    c *= 1.0 - 0.20 * oil * (1.0 - smoothstep(0.8, 4.0, fw));
    vec2 pc = floor(q / 14.0);
    float repair = step(gHash(pc + 41.0), 0.14) * gBox(fract(q / 14.0), vec2(0.15, 0.25), vec2(0.85, 0.7));
    c *= 1.0 - 0.18 * repair;
    float crack = smoothstep(0.46, 0.5, gNoise(vec2(q.x / 0.9, q.y / 14.0) + 21.0));
    c *= 1.0 - 0.06 * crack * grain;
    float lane = max(gLine(q.x + 4.0, 28.0, 0.12, fw), gLine(q.y + 9.0, 28.0, 0.12, fw)) * step(0.35, gHash(floor(q / 28.0) + seed));
    c = mix(c, vec3(0.62, 0.52, 0.18), lane * 0.55);
  } else if (kind < 7.5) {
    // Plaza: granite pavers, expansion joints, planting beds.
    vec2 pv = floor(q / 0.6);
    float paver = 0.95 + 0.10 * gHash(pv);
    float joints = max(gLine(q.x, 0.6, 0.012, fw), gLine(q.y, 0.6, 0.012, fw));
    float expansion = max(gLine(q.x, 6.0, 0.04, fw), gLine(q.y, 6.0, 0.04, fw));
    c *= paver * (1.0 - 0.18 * joints) * (1.0 - 0.28 * expansion);
    vec2 block = floor(q / vec2(34.0, 26.0));
    vec2 local = q - block * vec2(34.0, 26.0);
    float bed = step(gHash(block + seed * 3.0), 0.30) * gBox(local, vec2(6.0, 9.0), vec2(26.0, 14.0));
    vec3 green = vec3(0.17, 0.27, 0.11) * (0.8 + 0.4 * gNoise(q / 0.7));
    c = mix(c, green, bed);
  } else if (kind > 8.5) {
    // Car-park base: worn asphalt, tyre-polished aisles and oil stains. Bays and
    // cars are real geometry (buildings/parking.rs), so none are painted here.
    float aisle = smoothstep(0.55, 0.8, gNoise(vec2(q.x / 40.0, q.y / 3.0) + 5.0));
    c *= 1.0 - 0.07 * aisle * (1.0 - smoothstep(0.5, 3.0, fw));
    float oil = smoothstep(0.62, 0.9, gNoise(q / 2.2 + 17.0)) * (0.6 + 0.4 * gNoise(q / 0.5));
    c *= 1.0 - 0.18 * oil * (1.0 - smoothstep(0.8, 4.0, fw));
    vec2 pc = floor(q / 9.0);
    float repair = step(gHash(pc + 23.0), 0.10) * gBox(fract(q / 9.0), vec2(0.2, 0.2), vec2(0.8, 0.75));
    c *= 1.0 - 0.12 * repair;
  } else {
    // Lawn: mown bands and patchiness.
    float band = step(0.5, fract(q.x / 6.0));
    c *= 1.0 + 0.05 * (band - 0.5) * (1.0 - smoothstep(1.0, 4.0, fw));
    c *= 0.9 + 0.2 * gNoise(q / 2.4 + 3.0);
  }
  return c;
}
`;

/**
 * Make a material draw a parcel ground kind from its local plan position in
 * metres (the tile group is scaled to metres, so `position.xz` is metres on the
 * ground). The material's own colour is the base the pattern modulates.
 */
export function applyGroundKind<T extends import("three").MeshStandardMaterial>(material: T, kind: number): T {
  const previous = material.onBeforeCompile;
  material.onBeforeCompile = (shader, renderer) => {
    previous?.call(material, shader, renderer);
    shader.vertexShader = shader.vertexShader
      .replace("#include <common>", "#include <common>\nvarying vec2 vGroundP;")
      .replace("#include <begin_vertex>", "#include <begin_vertex>\nvGroundP = position.xz;");
    shader.fragmentShader = shader.fragmentShader
      .replace("#include <common>", "#include <common>\nvarying vec2 vGroundP;\n" + GROUND_GLSL)
      .replace(
        "#include <color_fragment>",
        `#include <color_fragment>
        diffuseColor.rgb = groundSurface(${kind.toFixed(1)}, diffuseColor.rgb, vGroundP, length(fwidth(vGroundP)), 0.0);`,
      );
  };
  material.customProgramCacheKey = () => `ground-${kind}`;
  return material;
}
