/**
 * A light probe at the eye: what is actually around the camera, as the scene's
 * environment.
 *
 * The baked sky is the right ambient light for an open hillside and the wrong one
 * for a street. Between buildings half the sky is hidden, and what replaces it
 * is the buildings themselves: the sunlit wall across the road that lights the
 * shaded one, the pale paving under an awning, the dark of a tree overhead. And
 * glass shows the street it faces, not an empty sky. Both come from one thing: a
 * small cube map of the scene, taken where the eye is, filtered and used as the
 * environment. Because the probe is lit by the previous probe, light it has
 * captured bounces again: a cheap, single-point global illumination.
 *
 * One face is drawn per frame, so a capture costs a seventh of a frame for seven
 * frames instead of a visible hitch.
 */
import * as THREE from "three";

export interface StreetProbe {
  /**
   * Call once a frame, before the main render. Returns the environment to use:
   * the probe's when `wanted` and one has been captured, else null.
   */
  update(wanted: boolean, eye: THREE.Vector3, moveThreshold: number, near: number, far: number, now: number): THREE.Texture | null;
  dispose(): void;
}

export function createStreetProbe(renderer: THREE.WebGLRenderer, scene: THREE.Scene, size = 256): StreetProbe {
  const target = new THREE.WebGLCubeRenderTarget(size, { type: THREE.HalfFloatType, generateMipmaps: false });
  const cube = new THREE.CubeCamera(0.01, 30, target);
  cube.coordinateSystem = renderer.coordinateSystem;
  cube.updateCoordinateSystem();
  const pmrem = new THREE.PMREMGenerator(renderer);
  let filtered: THREE.WebGLRenderTarget | null = null;
  let face = -1; // -1: idle; 0..5: the face to draw next; 6: filter
  const capturedAt = new THREE.Vector3(Number.NaN, 0, 0);
  let capturedTime = -Infinity;

  return {
    update(wanted, eye, moveThreshold, near, far, now) {
      if (!wanted) {
        face = -1;
        capturedAt.x = Number.NaN;
        return null;
      }
      if (face < 0) {
        const first = Number.isNaN(capturedAt.x);
        // Again after a move, and every few seconds: tiles keep arriving and the
        // light captured last time has not yet bounced.
        if (first || capturedAt.distanceTo(eye) > moveThreshold || now - capturedTime > 5) {
          face = 0;
          cube.position.copy(eye);
          for (const child of cube.children as THREE.PerspectiveCamera[]) {
            child.near = near;
            child.far = far;
            child.updateProjectionMatrix();
          }
          cube.updateMatrixWorld(true);
        }
      }
      if (face >= 0 && face < 6) {
        const previousTarget = renderer.getRenderTarget();
        const previousFace = renderer.getActiveCubeFace();
        const previousMip = renderer.getActiveMipmapLevel();
        const xr = renderer.xr.enabled;
        renderer.xr.enabled = false;
        renderer.setRenderTarget(target, face);
        renderer.render(scene, cube.children[face] as THREE.PerspectiveCamera);
        renderer.setRenderTarget(previousTarget, previousFace, previousMip);
        renderer.xr.enabled = xr;
        face += 1;
      } else if (face === 6) {
        const next = pmrem.fromCubemap(target.texture);
        filtered?.dispose();
        filtered = next;
        capturedAt.copy(cube.position);
        capturedTime = now;
        face = -1;
      }
      return filtered ? filtered.texture : null;
    },
    dispose() {
      filtered?.dispose();
      target.dispose();
      pmrem.dispose();
    },
  };
}
