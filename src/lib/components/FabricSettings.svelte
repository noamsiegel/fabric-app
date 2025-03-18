<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import { onMount } from 'svelte';
  import { Loader2, Save, Folder } from 'lucide-svelte';
  
  let customFabricPath = '';
  let customConfigPath = '';
  let saving = false;
  let message = '';
  let messageType: 'success' | 'error' | '' = '';
  
  onMount(async () => {
    try {
      // Try to load the current settings
      const binPath = await invoke('get_fabric_bin_path');
      if (binPath) {
        customFabricPath = binPath as string;
      }
      
      const configPath = await invoke('get_fabric_config_dir');
      if (configPath) {
        customConfigPath = configPath as string;
      }
    } catch (e) {
      console.error("Failed to load current paths:", e);
    }
  });
  
  async function browseFabricPath() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Executable',
          extensions: ['exe']
        }]
      });
      
      if (selected) {
        customFabricPath = selected as string;
      }
    } catch (e) {
      console.error("Error selecting file:", e);
    }
  }
  
  async function browseConfigPath() {
    try {
      const selected = await open({
        multiple: false,
        directory: true
      });
      
      if (selected) {
        customConfigPath = selected as string;
      }
    } catch (e) {
      console.error("Error selecting directory:", e);
    }
  }
  
  async function saveSettings() {
    saving = true;
    message = '';
    
    try {
      // Save paths to environment variables
      if (customFabricPath) {
        await invoke('update_secret', {
          key: 'FABRIC_BIN_PATH',
          value: customFabricPath
        });
      }
      
      if (customConfigPath) {
        await invoke('update_secret', {
          key: 'FABRIC_CONFIG_DIR',
          value: customConfigPath
        });
      }
      
      messageType = 'success';
      message = 'Settings saved successfully';
      
      // Verify installation
      const isInstalled = await invoke('check_fabric_installation');
      if (!isInstalled) {
        messageType = 'error';
        message = 'Settings saved, but Fabric installation could not be verified';
      }
    } catch (e) {
      messageType = 'error';
      message = `Error saving settings: ${e}`;
      console.error(e);
    } finally {
      saving = false;
    }
  }
</script>

<div class="fabric-settings">
  <h2 class="text-lg font-bold mb-4">Fabric Path Settings</h2>
  
  <div class="form-group mb-4">
    <label for="fabric-exe-path" class="block mb-1">Fabric Executable Path</label>
    <div class="flex">
      <input
        type="text"
        id="fabric-exe-path"
        bind:value={customFabricPath}
        placeholder="e.g., D:\fabric\fabric.exe"
        class="flex-grow p-2 border rounded"
      />
      <button 
        on:click={browseFabricPath} 
        class="ml-2 bg-blue-500 text-white p-2 rounded flex items-center"
      >
        <Folder size={18} />
      </button>
    </div>
    <small class="block mt-1 text-gray-500">Current path: {customFabricPath || 'Using default'}</small>
  </div>
  
  <div class="form-group mb-4">
    <label for="fabric-config-path" class="block mb-1">Fabric Config Directory</label>
    <div class="flex">
      <input
        type="text"
        id="fabric-config-path"
        bind:value={customConfigPath}
        placeholder="e.g., C:\Users\Colin\.config\fabric"
        class="flex-grow p-2 border rounded"
      />
      <button 
        on:click={browseConfigPath} 
        class="ml-2 bg-blue-500 text-white p-2 rounded flex items-center"
      >
        <Folder size={18} />
      </button>
    </div>
    <small class="block mt-1 text-gray-500">Current path: {customConfigPath || 'Using default'}</small>
  </div>
  
  <button 
    on:click={saveSettings} 
    class="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded flex items-center"
    disabled={saving}
  >
    {#if saving}
      <Loader2 class="animate-spin mr-2" size={18} />
      Saving...
    {:else}
      <Save size={18} class="mr-2" />
      Save Settings
    {/if}
  </button>
  
  {#if message}
    <div class={`mt-4 p-3 rounded ${messageType === 'success' ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'}`}>
      {message}
    </div>
  {/if}
</div>

<style>
  .fabric-settings {
    padding: 1.5rem;
    border-radius: 0.5rem;
    background-color: var(--background);
    border: 1px solid var(--border);
    margin-bottom: 1.5rem;
  }
</style>
