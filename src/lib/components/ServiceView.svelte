<script lang="ts">
  import { activeServiceId, services } from '../stores/services';
  import { invoke } from '@tauri-apps/api/core';
  import { get } from 'svelte/store';

  let currentServiceId: string | null = null;

  $: if ($activeServiceId && $activeServiceId !== currentServiceId) {
    currentServiceId = $activeServiceId;
    const svc = get(services).find(s => s.id === currentServiceId);
    if (svc) {
      // Crear o restaurar la ventana del servicio
      invoke('restore_service', { serviceId: currentServiceId, url: svc.url });
    }
  }
</script>

<!-- Este componente no muestra nada visual; solo maneja la lógica de ventanas -->