<script lang="ts">
  import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Select from "$lib/components/ui/select";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { Loader2 } from "lucide-svelte";

  // stores
  import { defaultModelStore } from "$lib/stores/models";

  $: defaultModel = $defaultModelStore;

  let vendors: string[] = [];
  let defaultVendor: VendorOption = { value: "", label: "" };
  let isLoading = false;
  let loadTimes = {
    initial: 0,
    save: 0,
  };

  interface VendorOption {
    value: string;
    label: string;
  }

  async function loadVendors() {
    const startTime = performance.now();
    try {
      vendors = await invoke("get_vendors");
    } catch (err) {
      console.error("Failed to load vendors:", err);
    } finally {
      loadTimes.initial = performance.now() - startTime;
    }
  }
  async function loadDefaultSettings() {
    try {
      const model = await invoke("get_secret", { key: "DEFAULT_MODEL" });
      defaultModelStore.set(model as string);
      const vendor = (await invoke("get_secret", {
        key: "DEFAULT_VENDOR",
      })) as string;
      defaultVendor = { value: vendor, label: vendor };
    } catch (err) {
      console.error("Failed to load default settings:", err);
    }
  }

  async function saveDefaultSettings() {
    isLoading = true;
    const startTime = performance.now();

    try {
      await invoke("set_default_model", { model: defaultModel });
      defaultModelStore.set(defaultModel);
      await invoke("set_default_vendor", { vendor: defaultVendor.value });
    } catch (err) {
      console.error("Failed to save default settings:", err);
    } finally {
      loadTimes.save = performance.now() - startTime;
      isLoading = false;
    }
  }

  onMount(async () => {
    await Promise.all([loadVendors(), loadDefaultSettings()]);
  });
</script>

<Card>
  <CardHeader>
    <CardTitle>
      Default Model Settings
      {#if loadTimes.initial > 0}
        <span class="text-xs text-muted-foreground ml-2">
          Loaded in {loadTimes.initial.toFixed(2)}ms
        </span>
      {/if}
    </CardTitle>
    <CardDescription>Configure the default model to use</CardDescription>
  </CardHeader>
  <CardContent class="space-y-4">
    <div class="space-y-2">
      <label for="defaultModel">Default Model</label>
      <Input
        id="defaultModel"
        bind:value={defaultModel}
        placeholder="gpt-4-turbo-preview"
      />
    </div>
    <div class="space-y-2">
      <label for="defaultVendor">Default Vendor</label>
      <Select.Root bind:selected={defaultVendor}>
        <Select.Trigger class="w-full">
          <Select.Value placeholder="Select vendor" />
        </Select.Trigger>
        <Select.Content>
          {#each vendors as vendor}
            <Select.Item value={vendor}>{vendor}</Select.Item>
          {/each}
        </Select.Content>
      </Select.Root>
    </div>
  </CardContent>
  <CardFooter class="flex gap-2 items-center">
    {#if loadTimes.save > 0}
      <span class="text-xs text-muted-foreground">
        Last save took {loadTimes.save.toFixed(2)}ms
      </span>
    {/if}
    <Button on:click={saveDefaultSettings} disabled={isLoading}>
      {#if isLoading}
        <Loader2 class="mr-2 h-4 w-4 animate-spin" />
        Saving...
      {:else}
        Save Settings
      {/if}
    </Button>
  </CardFooter>
</Card>
