<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { invoke } from "@tauri-apps/api/core";

  let homePath = "";
  let fabricConfigPath = "";
  let fabricBinPath = "";

  async function handleGetHomeDir() {
    try {
      const path = (await invoke("get_home_dir")) as string;
      homePath = path.toString();
    } catch (error) {
      console.error("Failed to get home directory:", error);
    }
  }

  async function handleGetFabricConfigDir() {
    try {
      const path = (await invoke("get_fabric_config_dir")) as string;
      fabricConfigPath = path.toString();
    } catch (error) {
      console.error("Failed to get fabric config directory:", error);
    }
  }

  async function handleGetFabricBinPath() {
    try {
      const path = (await invoke("get_fabric_bin_path")) as string;
      fabricBinPath = path.toString();
    } catch (error) {
      console.error("Failed to get fabric bin path:", error);
    }
  }
</script>

<Card.Root class="w-full max-w-2xl mx-auto p-6">
  <Card.Header>
    <Card.Title>Path Information</Card.Title>
    <Card.Description>View various system paths</Card.Description>
  </Card.Header>
  <Card.Content class="flex flex-col gap-4">
    <div>
      <Button variant="outline" onclick={handleGetHomeDir}
        >Get Home Directory</Button
      >
      {#if homePath}
        <p class="mt-2 text-sm text-muted-foreground">
          Home Directory: {homePath}
        </p>
      {/if}
    </div>

    <div>
      <Button variant="outline" onclick={handleGetFabricConfigDir}
        >Get Fabric Config Directory</Button
      >
      {#if fabricConfigPath}
        <p class="mt-2 text-sm text-muted-foreground">
          Fabric Config Directory: {fabricConfigPath}
        </p>
      {/if}
    </div>

    <div>
      <Button variant="outline" onclick={handleGetFabricBinPath}
        >Get Fabric Bin Path</Button
      >
      {#if fabricBinPath}
        <p class="mt-2 text-sm text-muted-foreground">
          Fabric Bin Path: {fabricBinPath}
        </p>
      {/if}
    </div>
  </Card.Content>
</Card.Root>
