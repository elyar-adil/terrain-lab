/// <reference types="vite/client" />

declare module "virtual:single-payload" {
  const payload: { fixture: string; city: string };
  export default payload;
}
