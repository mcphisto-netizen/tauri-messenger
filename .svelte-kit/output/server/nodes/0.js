

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export const universal = {
  "ssr": false
};
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.CwVQ_LJ2.js","_app/immutable/chunks/DJxAduVi.js","_app/immutable/chunks/B61nKZNH.js","_app/immutable/chunks/O52Vl81L.js"];
export const stylesheets = ["_app/immutable/assets/0.UkNGv_H1.css"];
export const fonts = [];
