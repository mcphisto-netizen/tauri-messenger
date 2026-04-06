import "clsx";
import { a3 as ensure_array_like, a4 as store_get, a5 as attr_class, e as escape_html, a6 as unsubscribe_stores } from "../../chunks/renderer.js";
import { w as writable } from "../../chunks/index.js";
import "@tauri-apps/plugin-sql";
import "emoji-picker-element";
import "@tauri-apps/api/core";
import "@tauri-apps/api/window";
const services = writable([]);
const activeServiceId = writable(null);
function Sidebar($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    var $$store_subs;
    $$renderer2.push(`<aside class="bg-gray-900 text-white flex flex-col h-screen w-full"><div class="p-4 font-bold text-lg border-b border-gray-700 flex justify-between items-center"><span>Tauri Messenger</span> <button class="text-gray-400 hover:text-white text-xl leading-none" title="Minimizar">─</button></div> <nav class="flex-1 overflow-y-auto"><!--[-->`);
    const each_array = ensure_array_like(store_get($$store_subs ??= {}, "$services", services));
    for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
      let service = each_array[$$index];
      $$renderer2.push(`<div${attr_class("p-3 cursor-pointer hover:bg-gray-700 flex items-center justify-between group", void 0, {
        "bg-gray-700": store_get($$store_subs ??= {}, "$activeServiceId", activeServiceId) === service.id
      })}><div class="flex items-center gap-2 truncate"><span class="text-xl">${escape_html(service.icon || "🌐")}</span> <span class="truncate">${escape_html(service.name)}</span></div> <div class="hidden group-hover:flex gap-1"><button class="text-xs bg-gray-600 hover:bg-gray-500 px-2 py-1 rounded" title="Editar">✏️</button> <button class="text-xs bg-red-600 hover:bg-red-500 px-2 py-1 rounded" title="Eliminar">🗑️</button></div></div>`);
    }
    $$renderer2.push(`<!--]--></nav> <button class="m-3 p-2 bg-blue-600 hover:bg-blue-700 rounded-lg transition">+ Agregar Servicio</button></aside> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]-->`);
    if ($$store_subs) unsubscribe_stores($$store_subs);
  });
}
function _page($$renderer) {
  $$renderer.push(`<main class="h-screen w-screen">`);
  Sidebar($$renderer);
  $$renderer.push(`<!----></main>`);
}
export {
  _page as default
};
