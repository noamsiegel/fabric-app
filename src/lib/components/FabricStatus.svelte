<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { AlertCircle, CheckCircle, Loader2 } from 'lucide-svelte';

  let fabricInstalled = false;
  let loading = true;
  let error = null;

  async function checkFabricInstallation() {
    try {
      loading = true;
      fabricInstalled = await invoke('check_fabric_installation');
    } catch (e) {
      error = e;
      console.error("Error checking Fabric installation:", e);
    } finally {
      loading = false;
    }
  }

  async function installFabric() {
    try {
      loading = true;
      const result = await invoke('install_fabric');
      console.log("Install result:", result);
      await checkFabricInstallation();
    } catch (e) {
      error = e;
      console.error("Error installing Fabric:", e);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    checkFabricInstallation();
  });
</script>

<div class="fabric-status">
  {#if loading}
    <div class="status-indicator loading">
      <Loader2 class="animate-spin" />
      <span>Checking Fabric installation...</span>
    </div>
  {:else if fabricInstalled}
    <div class="status-indicator installed">
      <CheckCircle class="text-green-500" />
      <span>Fabric is installed</span>
    </div>
  {:else}
    <div class="status-indicator not-installed">
      <AlertCircle class="text-yellow-500" />
      <span>Fabric is not installed</span>
      <button 
        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
        on:click={installFabric}
      >
        Install Fabric
      </button>
    </div>
  {/if}
  
  {#if error}
    <div class="error bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mt-2">
      {error}
    </div>
  {/if}
</div>

<style>
  .fabric-status {
    padding: 1rem;
    margin-bottom: 1rem;
    border-radius: 0.5rem;
    background-color: var(--background);
    border: 1px solid var(--border);
  }
  
  .status-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  
  .error {
    margin-top: 1rem;
    padding: 0.5rem;
    border-radius: 0.25rem;
    background-color: rgba(254, 226, 226, 0.5);
    color: rgb(185, 28, 28);
  }
</style>
