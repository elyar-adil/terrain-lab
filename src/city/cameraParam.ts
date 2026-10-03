/**
 * `?cam=x,y,z,tx,ty,tz[,fov]` — a camera override shared by every harness, so any
 * component can be inspected from any angle and a bad frame can be reported as a
 * URL. Returns `base` unchanged when the parameter is absent or malformed.
 */
export interface Framing {
  position: [number, number, number];
  target: [number, number, number];
  fov: number;
}

export function framingFromQuery<T extends Framing>(base: T, search = window.location.search): T {
  const raw = new URLSearchParams(search).get("cam");
  const v = raw ? raw.split(",").map(Number) : [];
  if (v.length < 6 || !v.every((n) => Number.isFinite(n))) return base;
  return {
    ...base,
    position: [v[0], v[1], v[2]],
    target: [v[3], v[4], v[5]],
    fov: v[6] ?? base.fov,
  };
}
