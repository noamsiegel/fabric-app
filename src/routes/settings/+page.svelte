<script lang="ts">
  // svelte components
  import { Button } from "$lib/components/ui/button";
  import { Switch } from "$lib/components/ui/switch";
  import { Label } from "$lib/components/ui/label";
  import { Slider } from "$lib/components/ui/slider";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import ModelsTable from "$lib/components/ModelsTable.svelte";
  import PatternsTable from "$lib/components/PatternsTable.svelte";
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
  let temperature = 0.5;
  let presencePenalty = 0.0;
  let models: { id: number; name: string; provider: string }[] = [];
  let fabricDirExists = false;
  let patternFolders: string[] = [];

  async function checkFabricDir() {
    try {
      fabricDirExists = await invoke("get_fabric_dir");
      console.log("home directory exists:", fabricDirExists);
    } catch (err) {
      console.error("Failed to check home directory:", err);
    }
  }

  async function getPatternFolders() {
    try {
      patternFolders = await invoke("get_pattern_folders");
      console.log("Pattern folders:", patternFolders);
    } catch (err) {
      console.error("Failed to get pattern folders:", err);
    }
  }

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

  async function setTemperature(value: number) {
    await invoke("set_temperature", { value });
  }

  async function setPresencePenalty(value: number) {
    await invoke("set_presence_penalty", { value });
  }

  async function showModels() {
    try {
      const modelList: string[] = await invoke("get_models");
      models = modelList.map((model, index) => {
        const [provider, ...nameParts] = model.split(":");
        return {
          id: index + 1,
          name: nameParts.join(":").trim(),
          provider: provider.trim(),
        };
      });
    } catch (err) {
      console.error("Failed to fetch models:", err);
    }
  }

  // Load the fabric folder path when the component mounts
  onMount(() => {
    loadFabricFolderPath();
    showModels();
    checkFabricDir(); // Add initial check
  });
</script>

<div class="p-4">
  <div class="flex flex-col space-y-2">
    <Button variant="secondary" on:click={getPatternFolders}>
      Get Pattern Folders
    </Button>
    {#if patternFolders.length > 0}
      <div class="mt-2">
        <h3 class="text-sm font-medium mb-2">Pattern Folders:</h3>
        <ul class="list-disc list-inside">
          {#each patternFolders as folder}
            <li class="text-sm">{folder}</li>
          {/each}
        </ul>
      </div>
    {:else}
      <p class="text-sm text-gray-500">No pattern folders found</p>
    {/if}
  </div>

  <h1 class="text-2xl font-bold mb-4">Settings</h1>

  <Tabs.Root value="general">
    <Tabs.List>
      <Tabs.Trigger value="general">General</Tabs.Trigger>
      <Tabs.Trigger value="models">Models</Tabs.Trigger>
      <Tabs.Trigger value="patterns">Patterns</Tabs.Trigger>
    </Tabs.List>
    <Tabs.Content value="general">
      <div class="space-y-4 mt-4">
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
          <Button variant="secondary" on:click={checkFabricDir}>
            Check Fabric Directory
          </Button>
          <p class="text-sm">
            Fabric directory {fabricDirExists ? "exists" : "does not exist"}
          </p>
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
      </div>
    </Tabs.Content>
    <Tabs.Content value="models">
      <div class="mt-4">
        <ModelsTable />
      </div>
    </Tabs.Content>
    <Tabs.Content value="patterns">
      <div class="mt-4">
        <PatternsTable />
      </div>
    </Tabs.Content>
  </Tabs.Root>
</div>
