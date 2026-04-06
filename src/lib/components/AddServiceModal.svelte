<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { addService } from '../stores/services';
  import 'emoji-picker-element';

  const dispatch = createEventDispatcher();

  const predefinedServices = [
    { name: 'WhatsApp Web', url: 'https://web.whatsapp.com', icon: '💬' },
    { name: 'Telegram Web', url: 'https://web.telegram.org', icon: '✈️' },
    { name: 'Slack', url: 'https://app.slack.com', icon: '💼' },
    { name: 'Discord', url: 'https://discord.com/app', icon: '🎮' },
    { name: 'Messenger', url: 'https://www.messenger.com', icon: '💬' },
    { name: 'Google Chat', url: 'https://chat.google.com', icon: '📧' },
    { name: 'Twitter', url: 'https://twitter.com', icon: '🐦' },
    { name: 'Instagram', url: 'https://www.instagram.com', icon: '📸' },
  ];

  let selectedService = predefinedServices[0];
  let customName = '';
  let customUrl = '';
  let useCustom = false;
  let icon = '';
  let showEmojiPicker = false;

  $: if (!useCustom && selectedService) {
    icon = selectedService.icon;
  }

  function onEmojiSelect(event: any) {
    icon = event.detail.unicode;
    showEmojiPicker = false;
  }

  async function handleSubmit() {
    let name, url, finalIcon;
    if (useCustom) {
      name = customName;
      url = customUrl;
      finalIcon = icon || null;
    } else {
      name = selectedService.name;
      url = selectedService.url;
      finalIcon = selectedService.icon;
    }
    if (!name || !url) return;

    await addService({
      name,
      url,
      icon: finalIcon,
      order: Date.now(),
      custom_css: null,
      custom_js: null,
      user_agent: null,
      zoom: 1.0,
      notifications_enabled: true,
      is_active: true,
    });
    dispatch('close');
  }
</script>

<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
  <div class="bg-white dark:bg-gray-800 rounded-lg p-6 w-96 shadow-xl">
    <h2 class="text-xl font-bold mb-4 text-gray-900 dark:text-white">Agregar Servicio</h2>

    <label class="flex items-center gap-2 mb-3">
      <input type="radio" bind:group={useCustom} value={false} checked /> Servicio predefinido
    </label>
    <label class="flex items-center gap-2 mb-3">
      <input type="radio" bind:group={useCustom} value={true} /> Servicio personalizado
    </label>

    {#if !useCustom}
      <select bind:value={selectedService} class="w-full border border-gray-300 dark:border-gray-600 rounded p-2 mb-3 bg-white dark:bg-gray-700 text-gray-900 dark:text-white">
        {#each predefinedServices as s}
          <option value={s}>{s.name}</option>
        {/each}
      </select>
    {:else}
      <input bind:value={customName} placeholder="Nombre" class="w-full border border-gray-300 dark:border-gray-600 rounded p-2 mb-3 bg-white dark:bg-gray-700 text-gray-900 dark:text-white" />
      <input bind:value={customUrl} placeholder="URL completa (https://...)" class="w-full border border-gray-300 dark:border-gray-600 rounded p-2 mb-3 bg-white dark:bg-gray-700 text-gray-900 dark:text-white" />
    {/if}

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