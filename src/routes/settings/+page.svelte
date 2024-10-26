<script lang="ts">
  // svelte components
  import { Button } from "$lib/components/ui/button";
  import { Switch } from "$lib/components/ui/switch";
  import { Label } from "$lib/components/ui/label";
  import { Slider } from "$lib/components/ui/slider";

  // svelte stores
  import { writable } from "svelte/store";
  import { onMount } from "svelte";

  // tauri plugins
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";

  let darkMode = false;
  let notifications = true;
  let fabricFolderPath = "";
  const fabricFolderStore = writable("");
  let patterns: string[] = [];

  async function selectFabricFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        defaultPath: fabricFolderPath || undefined,
      });

      if (selected) {
        fabricFolderPath = selected as string;
        await saveFabricFolderPath(fabricFolderPath);
      }
    } catch (err) {
      console.error("Failed to select folder:", err);
    }
  }

  async function saveFabricFolderPath(path: string) {
    try {
      // Set the fabric folder path in the Tauri app state
      await invoke("set_fabric_folder", { path });
      await invoke("set_patterns");
      // TODO make this into a list that the user can see and check off the ones that they do/don't want to use
      patterns = await invoke("get_patterns");
      console.log("Patterns:", patterns);

      // Update the Svelte store
      fabricFolderStore.set(path);
    } catch (err) {
      console.error("Failed to save fabric folder path:", err);
      alert(`Failed to save fabric folder path: ${err}`);
    }
  }

  async function loadFabricFolderPath() {
    try {
      // Get the fabric folder path from the Tauri app state
      const path = await invoke("get_fabric_folder");
      fabricFolderPath = path as string;
      fabricFolderStore.set(fabricFolderPath);
    } catch (err) {
      console.error("Failed to load fabric folder path:", err);
    }
  }

  let temperature = 0.5;
  let presencePenalty = 0.0;

  async function setTemperature(value: number) {
    await invoke("set_temperature", { value });
  }

  async function setPresencePenalty(value: number) {
    await invoke("set_presence_penalty", { value });
  }

  async function showModels() {
    try {
      const models: string[] = await invoke("get_models");
      await alert(`Available Models:\n\n${models.join("\n")}`);
    } catch (err) {
      console.error("Failed to fetch models:", err);
      await alert(`Failed to fetch models: ${err}`);
    }
  }

  // Load the fabric folder path when the component mounts
  onMount(() => {
    loadFabricFolderPath();
  });
</script>

<div class="p-4">
  <h1 class="text-2xl font-bold mb-4">Settings</h1>

  <div class="space-y-4">
    <div class="flex items-center space-x-2">
      <Switch id="dark-mode" bind:checked={darkMode} />
      <Label for="dark-mode">Dark Mode</Label>
    </div>

    <div class="flex items-center space-x-2">
      <Switch id="notifications" bind:checked={notifications} />
      <Label for="notifications">Enable Notifications</Label>
    </div>

    <div class="flex flex-col space-y-2">
      <Button on:click={selectFabricFolder}>Select Fabric Folder</Button>
      {#if fabricFolderPath}
        <p class="text-sm">Selected Fabric folder: {fabricFolderPath}</p>
      {:else}
        <p class="text-sm">No Fabric folder selected</p>
      {/if}
    </div>
  </div>
  <div class="flex flex-col space-y-2">
    <Label for="temperature">Temperature: {temperature.toFixed(2)}</Label>
    <Slider
      id="temperature"
      min={0}
      max={1}
      step={0.1}
      value={[temperature]}
      onValueChange={(values) => {
        temperature = values[0];
        setTemperature(temperature);
      }}
    />
  </div>

  <div class="flex flex-col space-y-2">
    <Label for="presence-penalty"
      >Presence Penalty: {presencePenalty.toFixed(2)}</Label
    >
    <Slider
      id="presence-penalty"
      min={0}
      max={1}
      step={0.1}
      value={[presencePenalty]}
      onValueChange={(values) => {
        presencePenalty = values[0];
        setPresencePenalty(presencePenalty);
      }}
    />
  </div>
  <Button on:click={showModels}>Show Available Models</Button>
</div>
