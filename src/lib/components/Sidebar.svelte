<script lang="ts">
  import { services, activeServiceId, loadServices, deleteService } from '../stores/services';
  import AddServiceModal from './AddServiceModal.svelte';
  import EditServiceModal from './EditServiceModal.svelte';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  let showAddModal = false;
  let showEditModal = false;
  let editingService: any = null;

  onMount(() => {
    loadServices();
  });

  async function handleDelete(id: string) {
    if (confirm('¿Eliminar este servicio?')) {
      await deleteService(id);
    }
  }

  function handleEdit(service: any) {
    editingService = service;
    showEditModal = true;
  }

  async function openService(serviceId: string, url: string) {
    activeServiceId.set(serviceId);
    await invoke('restore_service', { serviceId, url });
    // Opcional: minimizar ventana principal al abrir un servicio
    // await getCurrentWindow().minimize();
  }
</script>

<aside class="bg-gray-900 text-white flex flex-col h-screen w-full">
  <div class="p-4 font-bold text-lg border-b border-gray-700 flex justify-between items-center">
    <span>Tauri Messenger</span>
    <button
      on:click={() => getCurrentWindow().minimize()}
      class="text-gray-400 hover:text-white text-xl leading-none"
      title="Minimizar"
    >─</button>
  </div>
  <nav class="flex-1 overflow-y-auto">
    {#each $services as service}
      <div
        class="p-3 cursor-pointer hover:bg-gray-700 flex items-center justify-between group"
        class:bg-gray-700={$activeServiceId === service.id}
        on:click={() => openService(service.id, service.url)}
      >
        <div class="flex items-center gap-2 truncate">
          <span class="text-xl">{service.icon || '🌐'}</span>
          <span class="truncate">{service.name}</span>
        </div>
        <div class="hidden group-hover:flex gap-1">
          <button
            on:click={(e) => { e.stopPropagation(); handleEdit(service); }}
            class="text-xs bg-gray-600 hover:bg-gray-500 px-2 py-1 rounded"
            title="Editar"
          >✏️</button>
          <button
            on:click={(e) => { e.stopPropagation(); handleDelete(service.id); }}
            class="text-xs bg-red-600 hover:bg-red-500 px-2 py-1 rounded"
            title="Eliminar"
          >🗑️</button>
        </div>
      </div>
    {/each}
  </nav>
  <button
    class="m-3 p-2 bg-blue-600 hover:bg-blue-700 rounded-lg transition"
    on:click={() => showAddModal = true}
  >
    + Agregar Servicio
  </button>
</aside>

{#if showAddModal}
  <AddServiceModal on:close={() => showAddModal = false} />
{/if}

{#if showEditModal && editingService}
  <EditServiceModal
    service={editingService}
    on:close={() => { showEditModal = false; editingService = null; }}
  />
{/if}