/**
 * The materials of a generated car (`crates/city-scene/src/car.rs`), one per
 * part key. Shared by the city's material catalogue and the car harness, so
 * the car judged alone is the car the street draws.
 */
import * as THREE from "three";

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
      color: 0x2a3a3f, roughness: 0.04, metalness: 0.0, transparent: true, opacity: 0.4,
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
    // The plate's colour is its design's tint: blue, or a new-energy green.
    ["car/plate", standard({ roughness: 0.45 }, 0xffffff)],
    // Wheel wells and lamp housings: seen from outside through an opening, so drawn from inside.
    ["car/well", standard({ roughness: 0.9, side: THREE.DoubleSide }, 0x0c0c0d)],
    // The far model (beyond `CAR_NEAR_M`): plain paint, glass as a dark solid.
    ["car/far.body", standard({ roughness: 0.3, metalness: 0.35, envMapIntensity: 1.0 }, 0xffffff)],
    ["car/far.glass", standard({ roughness: 0.12, metalness: 0.5, envMapIntensity: 1.1 }, 0x1c2429)],
    ["car/far.wheel", standard({ roughness: 0.92 }, 0x6a6a70)],
  ];
}
