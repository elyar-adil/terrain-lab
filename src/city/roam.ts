/**
 * Free-roam camera: walk or fly, WASD plus mouse look, as an alternative to the
 * orbit controls.
 *
 * It works on the host's camera directly and never needs a target of its own:
 * every frame it reports `groundTarget`, the point of the terrain the camera is
 * looking at (capped to a distance proportional to the height above ground), so
 * the host's orbit-centred systems — near/far planes, detail streaming, shadow
 * frustum, LOD — keep working unchanged while the user roams.
 *
 * - **Fly**: the look direction is the travel direction. Speed scales with the
 *   height above ground, so the same keys cross a continent from orbit and
 *   inch along a facade at eye level. Mouse wheel scales it further.
 * - **Walk**: eye height above the ground, horizontal motion only, fixed pace.
 *
 * Keys: W A S D / arrows move, Shift speeds up, Space or E rises, Q or Ctrl
 * descends (fly), mouse drag looks, wheel changes speed.
 */

import * as THREE from "three";

export interface RoamHost {
  camera: THREE.PerspectiveCamera;
  dom: HTMLElement;
  /** Terrain height in scene units. */
  groundHeight(x: number, z: number): number;
  metresToScene: number;
  /** The world spans ±`halfExtent` in x and z, scene units. */
  halfExtent: number;
}

const EYE_HEIGHT_M = 1.7;
const WALK_SPEED_MS = 4.5;
const FLY_MIN_MS = 4;
const FLY_MAX_MS = 9000;
const FLY_PER_METRE_OF_HEIGHT = 0.9;

const MOVE_KEYS = new Set([
  "KeyW", "KeyA", "KeyS", "KeyD", "KeyQ", "KeyE", "Space",
  "ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight",
  "ShiftLeft", "ShiftRight", "ControlLeft", "ControlRight",
]);

export class Roam {
  active = false;
  walking = false;
  /** Where the camera is looking on the ground; valid while `active`. */
  readonly groundTarget = new THREE.Vector3();
  /** Current speed in metres per second, for a HUD. */
  speedMs = 0;
  onChange: (() => void) | null = null;

  private yaw = 0;
  private pitch = 0;
  private speedScale = 1;
  private readonly keys = new Set<string>();
  private dragging = false;
  private pointer = -1;
  private readonly forward = new THREE.Vector3();
  private readonly right = new THREE.Vector3();
  private readonly euler = new THREE.Euler(0, 0, 0, "YXZ");
  private readonly cleanup: Array<() => void> = [];

  constructor(private readonly host: RoamHost) {
    const listen = <K extends keyof WindowEventMap>(type: K, fn: (event: WindowEventMap[K]) => void) => {
      window.addEventListener(type, fn as EventListener);
      this.cleanup.push(() => window.removeEventListener(type, fn as EventListener));
    };
    listen("keydown", (event) => {
      if (!this.active || this.typing(event)) return;
      if (MOVE_KEYS.has(event.code)) {
        this.keys.add(event.code);
        event.preventDefault();
      }
    });
    listen("keyup", (event) => {
      this.keys.delete(event.code);
    });
    listen("blur", () => this.keys.clear());
    const dom = host.dom;
    const down = (event: PointerEvent) => {
      if (!this.active) return;
      this.dragging = true;
      this.pointer = event.pointerId;
      dom.setPointerCapture?.(event.pointerId);
    };
    const move = (event: PointerEvent) => {
      if (!this.active || !this.dragging || event.pointerId !== this.pointer) return;
      this.yaw -= event.movementX * 0.0026;
      this.pitch = THREE.MathUtils.clamp(this.pitch - event.movementY * 0.0026, -1.5, 1.5);
    };
    const up = (event: PointerEvent) => {
      if (event.pointerId !== this.pointer) return;
      this.dragging = false;
      this.pointer = -1;
      dom.releasePointerCapture?.(event.pointerId);
    };
    const wheel = (event: WheelEvent) => {
      if (!this.active) return;
      event.preventDefault();
      this.speedScale = THREE.MathUtils.clamp(this.speedScale * Math.exp(-event.deltaY * 0.0015), 0.05, 20);
    };
    dom.addEventListener("pointerdown", down);
    dom.addEventListener("pointermove", move);
    dom.addEventListener("pointerup", up);
    dom.addEventListener("pointercancel", up);
    dom.addEventListener("wheel", wheel, { passive: false });
    this.cleanup.push(() => {
      dom.removeEventListener("pointerdown", down);
      dom.removeEventListener("pointermove", move);
      dom.removeEventListener("pointerup", up);
      dom.removeEventListener("pointercancel", up);
      dom.removeEventListener("wheel", wheel);
    });
  }

  private typing(event: KeyboardEvent): boolean {
    const element = event.target as HTMLElement | null;
    return Boolean(element && /^(INPUT|SELECT|TEXTAREA)$/.test(element.tagName));
  }

  /** Take over the camera from its current pose. */
  enter(walking = false): void {
    const camera = this.host.camera;
    camera.up.set(0, 1, 0);
    camera.getWorldDirection(this.forward);
    this.yaw = Math.atan2(-this.forward.x, -this.forward.z);
    this.pitch = Math.asin(THREE.MathUtils.clamp(this.forward.y, -1, 1));
    this.active = true;
    this.walking = walking;
    this.keys.clear();
    if (walking) this.snapToEye();
    this.onChange?.();
  }

  /** Hand back to the orbit controls; the caller re-targets them from `groundTarget`. */
  exit(): void {
    if (!this.active) return;
    this.active = false;
    this.dragging = false;
    this.keys.clear();
    this.onChange?.();
  }

  setWalking(walking: boolean): void {
    if (this.walking === walking) return;
    this.walking = walking;
    if (walking) {
      // Look level enough to see where you are walking.
      this.pitch = Math.max(this.pitch, -0.35);
      this.snapToEye();
    }
    this.onChange?.();
  }

  private snapToEye(): void {
    const { camera, metresToScene } = this.host;
    camera.position.y =
      this.host.groundHeight(camera.position.x, camera.position.z) + EYE_HEIGHT_M * metresToScene;
  }

  /** Advance one frame. Allocation-free. */
  update(deltaSeconds: number): void {
    if (!this.active) return;
    const { camera, metresToScene, halfExtent } = this.host;
    const dt = Math.min(deltaSeconds, 0.1);
    this.euler.set(this.pitch, this.yaw, 0);
    camera.quaternion.setFromEuler(this.euler);

    const groundHere = this.host.groundHeight(camera.position.x, camera.position.z);
    const heightM = Math.max(0, (camera.position.y - groundHere) / metresToScene);
    const boost = this.keys.has("ShiftLeft") || this.keys.has("ShiftRight") ? 4 : 1;
    const base = this.walking
      ? WALK_SPEED_MS
      : THREE.MathUtils.clamp(heightM * FLY_PER_METRE_OF_HEIGHT, FLY_MIN_MS, FLY_MAX_MS);
    this.speedMs = base * boost * this.speedScale;
    const step = this.speedMs * metresToScene * dt;

    const k = this.keys;
    const ahead = (k.has("KeyW") || k.has("ArrowUp") ? 1 : 0) - (k.has("KeyS") || k.has("ArrowDown") ? 1 : 0);
    const side = (k.has("KeyD") || k.has("ArrowRight") ? 1 : 0) - (k.has("KeyA") || k.has("ArrowLeft") ? 1 : 0);
    const lift = (k.has("Space") || k.has("KeyE") ? 1 : 0) - (k.has("KeyQ") || k.has("ControlLeft") || k.has("ControlRight") ? 1 : 0);

    if (this.walking) {
      this.forward.set(-Math.sin(this.yaw), 0, -Math.cos(this.yaw));
    } else {
      camera.getWorldDirection(this.forward);
    }
    this.right.set(-this.forward.z, 0, this.forward.x);
    if (this.right.lengthSq() > 1e-9) this.right.normalize();
    camera.position.addScaledVector(this.forward, ahead * step);
    camera.position.addScaledVector(this.right, side * step);
    if (!this.walking) camera.position.y += lift * step;

    const limit = halfExtent * 0.998;
    camera.position.x = THREE.MathUtils.clamp(camera.position.x, -limit, limit);
    camera.position.z = THREE.MathUtils.clamp(camera.position.z, -limit, limit);
    const ground = this.host.groundHeight(camera.position.x, camera.position.z);
    const eye = EYE_HEIGHT_M * metresToScene;
    if (this.walking) {
      // Follow the ground smoothly so steps over a levelled apron do not pop.
      const wanted = ground + eye;
      camera.position.y += (wanted - camera.position.y) * Math.min(1, dt * 14);
    } else if (camera.position.y < ground + eye) {
      camera.position.y = ground + eye;
    }
    camera.updateMatrixWorld();
    this.updateGroundTarget(ground, heightM);
  }

  /** The terrain point under the view centre, capped near the camera. */
  private updateGroundTarget(groundHere: number, heightM: number): void {
    const { camera, metresToScene } = this.host;
    camera.getWorldDirection(this.forward);
    const capM = Math.max(30, heightM * 14);
    const cap = capM * metresToScene;
    let hit = cap;
    if (this.forward.y < -1e-4) {
      // March down the view ray; the terrain is a height field, so the first
      // sample below it brackets the hit. Steps grow with distance.
      let t = 0;
      let step = Math.max(2 * metresToScene, cap / 64);
      let previous = 0;
      for (let i = 0; i < 96 && t < cap; i += 1) {
        t += step;
        const x = camera.position.x + this.forward.x * t;
        const z = camera.position.z + this.forward.z * t;
        const y = camera.position.y + this.forward.y * t;
        if (y <= this.host.groundHeight(x, z)) {
          // Refine between the last two samples.
          let lo = previous;
          let hi = t;
          for (let j = 0; j < 6; j += 1) {
            const mid = (lo + hi) * 0.5;
            const mx = camera.position.x + this.forward.x * mid;
            const mz = camera.position.z + this.forward.z * mid;
            const my = camera.position.y + this.forward.y * mid;
            if (my <= this.host.groundHeight(mx, mz)) hi = mid;
            else lo = mid;
          }
          hit = Math.min(cap, hi);
          break;
        }
        previous = t;
        step *= 1.08;
      }
    }
    void groundHere;
    this.groundTarget.copy(camera.position).addScaledVector(this.forward, Math.max(hit, 1.2 * metresToScene));
  }

  dispose(): void {
    for (const off of this.cleanup) off();
    this.cleanup.length = 0;
  }
}
