<script lang="ts">
  // svelte components
  import { Switch } from "$lib/components/ui/switch";
  import { Label } from "$lib/components/ui/label";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import { Button } from "$lib/components/ui/button";
  import { PlayCircle } from "lucide-svelte";
  import * as Drawer from "$lib/components/ui/drawer";
  import { settingsOpen } from "$lib/stores/settings";

  // tables
  // import ModelsTable from "$lib/components/ModelsTable.svelte";
  import SecretsTable from "./ApiKeysTable.svelte";
  import BaseUrlTable from "./BaseUrlTable.svelte";

  // tauri plugins
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let darkMode = false;
  let notifications = true;
  let isInstalling = false;
  let isInstalled = false;

  onMount(async () => {
    try {
      await invoke("get_fabric_dir");
      isInstalled = true;
    } catch (error) {
      isInstalled = false;
    }
  });
</script>

<Drawer.Root bind:open={$settingsOpen}>
  <Drawer.Content class="min-h-[50vh]">
    <div class="mx-auto w-full max-w-4xl p-4">
      <Tabs.Root value="general">
        <Tabs.List class="flex justify-center bg-transparent">
          <Tabs.Trigger class="data-[state=active]:bg-gray-100" value="general"
            >General</Tabs.Trigger
          >
          <Tabs.Trigger class="data-[state=active]:bg-gray-100" value="secrets"
            >API Keys</Tabs.Trigger
          >
          <Tabs.Trigger
            class="data-[state=active]:bg-gray-100"
            value="base-urls">Base URLs</Tabs.Trigger
          >
        </Tabs.List>
        <Tabs.Content value="general">
          <Button
            variant="outline"
            class="flex items-center gap-2"
            disabled={isInstalling}
            onclick={async () => {
              try {
                isInstalling = true;
                const result = await invoke("install_fabric");
                console.log("Installation successful:", result);
                isInstalled = true;
              } catch (error) {
                console.error("Installation failed:", error);
              } finally {
                isInstalling = false;
              }
            }}
          >
            <PlayCircle size={20} />
            {#if isInstalling}
              Installing...
            {:else}
              {isInstalled ? "Update Fabric" : "Install Fabric"}
            {/if}
          </Button>
          <div class="space-y-4 mt-4">
            <div class="flex items-center space-x-2">
              <Switch id="dark-mode" bind:checked={darkMode} />
              <Label for="dark-mode">Dark Mode</Label>
            </div>

            <div class="flex items-center space-x-2">
              <Switch id="notifications" bind:checked={notifications} />
              <Label for="notifications">Enable Notifications</Label>
            </div>
          </div>
        </Tabs.Content>
        <Tabs.Content value="secrets">
          <div class="mt-4">
            <SecretsTable />
          </div>
        </Tabs.Content>
        <Tabs.Content value="base-urls">
          <div class="mt-4">
            <BaseUrlTable />
          </div>
        </Tabs.Content>
      </Tabs.Root>
      <Drawer.Footer class="pt-2">
        <Drawer.Close>
          <Button variant="outline">Close</Button>
        </Drawer.Close>
      </Drawer.Footer>
    </div>
  </Drawer.Content>
</Drawer.Root>
