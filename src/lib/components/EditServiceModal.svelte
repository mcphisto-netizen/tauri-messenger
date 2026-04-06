<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { updateService } from '../stores/services';
  import type { Service } from '../types';
  import 'emoji-picker-element';

  export let service: Service;
  const dispatch = createEventDispatcher();

  let name = service.name;
  let url = service.url;
  let icon = service.icon || '';
  let showEmojiPicker = false;

  function onEmojiSelect(event: any) {
    icon = event.detail.unicode;
    showEmojiPicker = false;
  }

  async function handleSubmit() {
    if (!name || !url) return;
    await updateService({
      ...service,
      name,
      url,
      icon: icon || null,
    });
    dispatch('close');
  }
</script>

<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
  <div class="bg-white dark:bg-gray-800 rounded-lg p-6 w-96 shadow-xl">
    <h2 class="text-xl font-bold mb-4 text-gray-900 dark:text-white">Editar Servicio</h2>
    
    <input
      bind:value={name}
      placeholder="Nombre"
      class="w-full border border-gray-300 dark:border-gray-600 rounded p-2 mb-3 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
    />
    <input
      bind:value={url}
      placeholder="URL (ej: https://web.whatsapp.com)"
      class="w-full border border-gray-300 dark:border-gray-600 rounded p-2 mb-3 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
    />
    
    <div class="relative mb-4">
      <input
        bind:value={icon}
        placeholder="Icono (emoji) o selecciona"
        class="w-full border border-gray-300 dark:border-gray-600 rounded p-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
      />
      <button
        type="button"
        on:click={() => showEmojiPicker = !showEmojiPicker}
        class="absolute right-2 top-2 text-gray-500 dark:text-gray-400"
      >😊</button>
      {#if showEmojiPicker}
        <div class="absolute z-50 bg-white dark:bg-gray-800 shadow-lg rounded mt-1" style="width: 300px;">
          <emoji-picker on:emoji-click={onEmojiSelect}></emoji-picker>
        </div>
      {/if}
    </div>

    <div class="flex justify-end gap-2">
      <button
        on:click={() => dispatch('close')}
        class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded hover:bg-gray-100 dark:hover:bg-gray-700 transition"
      >
        Cancelar
      </button>
      <button
        on:click={handleSubmit}
        class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition"
      >
        Guardar
      </button>
    </div>
  </div>
</div>