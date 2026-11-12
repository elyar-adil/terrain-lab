/**
 * The materials of a generated car (`crates/city-scene/src/car.rs`), one per
 * part key. Shared by the city's material catalogue and the car harness, so
 * the car judged alone is the car the street draws.
 */
import * as THREE from "three";

/** The characters a plate is written with: provinces, then letters (no I or O), then digits. */
const PROVINCES = "京津沪渝冀豫云辽黑湘皖鲁新苏浙赣鄂桂甘晋蒙陕吉闽贵粤青藏川宁琼";
const LETTERS = "ABCDEFGHJKLMNPQRSTUVWXYZ";
const DIGITS = "0123456789";
const ATLAS_COLUMNS = 16;
const ATLAS_ROWS = 5;

/** Every plate character, white on black, one to a cell. */
function plateAtlas(): THREE.Texture {
  const cell = { width: 64, height: 112 };
  const canvas = document.createElement("canvas");
  canvas.width = cell.width * ATLAS_COLUMNS;
  canvas.height = cell.height * ATLAS_ROWS;
  const context = canvas.getContext("2d")!;
  context.fillStyle = "#000";
  context.fillRect(0, 0, canvas.width, canvas.height);
  context.fillStyle = "#fff";
  context.textAlign = "center";
  context.textBaseline = "middle";
  [...PROVINCES, ...LETTERS, ...DIGITS].forEach((glyph, index) => {
    const x = (index % ATLAS_COLUMNS) * cell.width + cell.width / 2;
    const y = Math.floor(index / ATLAS_COLUMNS) * cell.height + cell.height / 2;
    // Plate lettering is tall and narrow: drawn large, squeezed to the cell.
    context.save();
    context.translate(x, y + 4);
    const wide = index < PROVINCES.length;
    context.font = `bold ${wide ? 62 : 104}px "Noto Sans SC", "PingFang SC", "Microsoft YaHei", "DIN Alternate", "Arial Narrow", Arial, sans-serif`;
    context.scale(wide ? 0.98 : 0.8, wide ? 1.5 : 1);
    context.fillText(glyph, 0, 0);
    context.restore();
  });
  const texture = new THREE.CanvasTexture(canvas);
  texture.colorSpace = THREE.NoColorSpace;
  texture.anisotropy = 8;
  return texture;
}

/**
 * A number plate that writes itself. The generator gives every car a plate
 * whose instance tint is not a colour but a registration: `r` the number, `g`
 * blue plate or new-energy green, `b` the city that issued it. The shader
 * turns those into a province's character, a city's letter and five (or six)
 * letters and digits, on the right ground, inside the plate's white line.
 * Most cars carry the plates of the city they are parked in; some are from away.
 */
function plateMaterial(): THREE.Material {
  const material = new THREE.MeshStandardMaterial({ color: 0xffffff, roughness: 0.4, map: plateAtlas() });
  material.onBeforeCompile = (shader) => {
    shader.fragmentShader = shader.fragmentShader
      .replace(
        "#include <map_pars_fragment>",
        /* glsl */ `#include <map_pars_fragment>
float plateHash(float n) { return fract(sin(n * 12.9898) * 43758.5453); }
float plateGlyph(float index, vec2 cell) {
  if (cell.x < 0.04 || cell.x > 0.96 || cell.y < 0.02 || cell.y > 0.98) return 0.0;
  float column = mod(index, ${ATLAS_COLUMNS}.0);
  float row = floor(index / ${ATLAS_COLUMNS}.0);
  return texture2D(map, vec2((column + cell.x) / ${ATLAS_COLUMNS}.0, 1.0 - (row + 1.0 - cell.y) / ${ATLAS_ROWS}.0)).r;
}`,
      )
      .replace(
        "#include <map_fragment>",
        /* glsl */ `
#ifdef USE_COLOR
  vec3 registration = vColor.rgb;
#else
  vec3 registration = vec3(0.37, 0.0, 0.21);
#endif
  // The number as a whole number: an interpolated tint is not exactly equal
  // from pixel to pixel, and a hash of it would be noise.
  float plateSeed = floor(registration.r * 8192.0 + 0.5);
  float plateGreen = step(0.5, registration.g);
  float plateCity = floor(registration.b * 1024.0 + 0.5) / 1024.0;
  vec2 plateAt = vMapUv;
  float plateCount = 7.0 + plateGreen;
  float plateGap = 0.06;
  float plateCell = (1.0 - 0.10 - plateGap) / plateCount;
  float plateX = plateAt.x - 0.05;
  bool plateInGap = plateX > 2.0 * plateCell && plateX < 2.0 * plateCell + plateGap;
  if (plateX > 2.0 * plateCell) plateX -= plateGap;
  float plateSlot = floor(plateX / plateCell);
  vec2 plateIn = vec2(fract(plateX / plateCell), (plateAt.y - 0.17) / 0.66);
  float plateLocal = step(plateHash(plateSeed + 0.31), 0.86);
  float plateProvince = floor(mix(plateHash(plateSeed + 0.57), plateCity, plateLocal) * ${PROVINCES.length}.0);
  float plateLetter = ${PROVINCES.length}.0 + floor(mix(plateHash(plateSeed + 0.73), fract(plateCity * 57.3), plateLocal) * ${LETTERS.length}.0);
  float plateRoll = plateHash(plateSeed + plateSlot * 0.113 + 0.07);
  float plateIndex = plateRoll < 0.3
    ? ${PROVINCES.length}.0 + floor(plateRoll / 0.3 * ${LETTERS.length}.0)
    : ${PROVINCES.length + LETTERS.length}.0 + floor((plateRoll - 0.3) / 0.7 * ${DIGITS.length}.0);
  if (plateSlot < 0.5) plateIndex = plateProvince;
  else if (plateSlot < 1.5) plateIndex = plateLetter;
  // A new-energy plate's third character says what kind: D electric, F other.
  else if (plateGreen > 0.5 && plateSlot < 2.5) plateIndex = ${PROVINCES.length}.0 + (plateHash(plateSeed + 0.91) < 0.7 ? 3.0 : 5.0);
  float plateInk = (plateInGap || plateSlot < 0.0 || plateSlot >= plateCount) ? 0.0 : plateGlyph(plateIndex, plateIn);
  float plateAspect = 3.2;
  vec2 plateDot = (plateAt - vec2(0.05 + 2.0 * plateCell + plateGap * 0.5, 0.5)) * vec2(plateAspect, 1.0);
  plateInk = max(plateInk, (1.0 - smoothstep(0.035, 0.05, length(plateDot))) * (1.0 - plateGreen));
  vec2 plateEdge = min(plateAt, 1.0 - plateAt) * vec2(plateAspect, 1.0);
  float plateRim = min(plateEdge.x, plateEdge.y);
  float plateLine = smoothstep(0.035, 0.045, plateRim) * (1.0 - smoothstep(0.075, 0.085, plateRim));
  vec3 plateGround = mix(
    vec3(0.010, 0.045, 0.36),
    mix(vec3(0.10, 0.50, 0.22), vec3(0.80, 0.90, 0.82), smoothstep(0.1, 0.9, plateAt.y)),
    plateGreen
  );
  vec3 plateText = mix(vec3(0.92), vec3(0.015), plateGreen);
  diffuseColor.rgb = mix(plateGround, plateText, max(plateInk, plateLine));`,
      )
      .replace("#include <color_fragment>", "");
  };
  return material;
}

export function carMaterials(): Array<[string, THREE.Material]> {
  const standard = (options: THREE.MeshStandardMaterialParameters, colour: number) =>
    new THREE.MeshStandardMaterial({ color: colour, ...options });
  return [
    // `car/body` deliberately has no per-vertex colours: the car is one shared
    // prototype instanced 250 times, and its colour is the *instance* tint. Asking
    // for `vertexColors` with no `color` attribute makes WebGL supply the default
    // generic attribute, which is (0,0,0,1) — so every car renders pure black, and
    // the draw-call counter is perfectly happy about it.
    // Paint is a metallic base under a clear coat: the lacquer is what gives a
    // car its second, sharper reflection.
    ["car/body", new THREE.MeshPhysicalMaterial({ color: 0xffffff, roughness: 0.34, metalness: 0.4, clearcoat: 1.0, clearcoatRoughness: 0.06, envMapIntensity: 1.0 })],
    // Glazing you can see through: the seats and the wheel behind it are modelled.
    ["car/glass", new THREE.MeshPhysicalMaterial({
      color: 0x1a2428, roughness: 0.03, metalness: 0.0, transparent: true, opacity: 0.8,
      side: THREE.DoubleSide, depthWrite: false, envMapIntensity: 1.3,
    })],
    // Tyre rubber, and the rim's paint (tinted per design: silver, gunmetal, black).
    ["car/wheel", standard({ roughness: 0.9 }, 0x6a6a70)],
    ["car/rim", new THREE.MeshPhysicalMaterial({ color: 0xffffff, roughness: 0.3, metalness: 0.85, clearcoat: 0.6, clearcoatRoughness: 0.12, envMapIntensity: 1.1 })],
    ["car/trim", standard({ roughness: 0.55 }, 0x151617)],
    ["car/chrome", standard({ roughness: 0.14, metalness: 1.0, envMapIntensity: 1.15 }, 0xd4d8dc)],
    // Lenses: the clear cover of a headlamp, the red of the tail. They
    // are pieces of the body's own skin, so they write no depth and the housing
    // behind them shows through.
    ["car/lens", new THREE.MeshPhysicalMaterial({
      color: 0xdfe7ea, roughness: 0.03, metalness: 0.0, transparent: true, opacity: 0.16,
      depthWrite: false, envMapIntensity: 1.6, clearcoat: 1.0, clearcoatRoughness: 0.02,
    })],
    ["car/lens.red", new THREE.MeshPhysicalMaterial({
      color: 0x8c0a0c, roughness: 0.05, metalness: 0.0, transparent: true, opacity: 0.62,
      depthWrite: false, envMapIntensity: 1.4, clearcoat: 1.0, clearcoatRoughness: 0.03,
    })],
    // What is lit behind the lenses: daytime running lights are on by day, and
    // a tail lamp's LEDs glow a little under any light.
    ["car/lamp.front", standard({ roughness: 0.25, emissive: 0xf2f6ff, emissiveIntensity: 1.6 }, 0xf4f7fa)],
    ["car/lamp.rear", standard({ roughness: 0.25, emissive: 0xff1a12, emissiveIntensity: 0.9 }, 0xb0100c)],
    ["car/lamp.amber", standard({ roughness: 0.22, emissive: 0xff7a00, emissiveIntensity: 0.25 }, 0xe07a10)],
    ["car/interior", standard({ roughness: 0.95, side: THREE.DoubleSide }, 0xffffff)],
    ["car/plate", plateMaterial()],
    // Wheel wells and lamp housings: seen from outside through an opening, so drawn from inside.
    ["car/well", standard({ roughness: 0.9, side: THREE.DoubleSide }, 0x0c0c0d)],
    // The far model (beyond `CAR_NEAR_M`): plain paint, glass as a dark solid.
    ["car/far.body", standard({ roughness: 0.3, metalness: 0.35, envMapIntensity: 1.0 }, 0xffffff)],
    ["car/far.glass", standard({ roughness: 0.12, metalness: 0.5, envMapIntensity: 1.1 }, 0x1c2429)],
    ["car/far.wheel", standard({ roughness: 0.92 }, 0x6a6a70)],
  ];
}
