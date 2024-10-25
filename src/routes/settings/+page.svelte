<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Switch } from "$lib/components/ui/switch";
  import { Label } from "$lib/components/ui/label";
  import { open } from "@tauri-apps/plugin-dialog";
  import { appConfigDir, join } from "@tauri-apps/api/path";
  import { writeTextFile, readTextFile, exists } from "@tauri-apps/plugin-fs";

  let darkMode = false;
  let notifications = true;
  let fabricFolderPath = "";

  async function selectFabricFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        defaultPath: fabricFolderPath || undefined,
      });

      if (selected) {
        fabricFolderPath = selected as string;
        await saveFabricFolderPath();
      }
    } catch (err) {
      console.error("Failed to select folder:", err);
    }
  }

  async function saveFabricFolderPath() {
    try {
      const configDir = await appConfigDir();
      const configPath = await join(configDir, "fabric_config.json");

      // Ensure the config directory exists
      if (!(await exists(configDir))) {
        await createDir(configDir, { recursive: true });
      }

      await writeTextFile(configPath, JSON.stringify({ fabricFolderPath }));
    } catch (err) {
      console.error("Failed to save fabric folder path:", err);
    }
  }

  async function loadFabricFolderPath() {
    try {
      const configDir = await appConfigDir();
      const configPath = await join(configDir, "fabric_config.json");

      if (await exists(configPath)) {
        const configContent = await readTextFile(configPath);
        const config = JSON.parse(configContent);
        fabricFolderPath = config.fabricFolderPath || "";
      }
    } catch (err) {
      console.error("Failed to load fabric folder path:", err);
    }
  }

  // Load the fabric folder path when the component mounts
  loadFabricFolderPath();
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
      {/if}
    </div>
  </div>
</div>
