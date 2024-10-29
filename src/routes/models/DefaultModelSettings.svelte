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

  let defaultModel = "";
  let vendors: string[] = [];
  let defaultVendor: VendorOption = { value: "", label: "" };

  interface VendorOption {
    value: string;
    label: string;
  }

  async function loadVendors() {
    try {
      vendors = await invoke("get_vendors");
    } catch (err) {
      console.error("Failed to load vendors:", err);
    }
  }

  async function loadDefaultSettings() {
    try {
      defaultModel = await invoke("get_secret", { key: "DEFAULT_MODEL" });
      const vendor = (await invoke("get_secret", {
        key: "DEFAULT_VENDOR",
      })) as string;
      defaultVendor = { value: vendor, label: vendor };
    } catch (err) {
      console.error("Failed to load default settings:", err);
    }
  }

  async function saveDefaultSettings() {
    try {
      await invoke("set_default_model", { model: defaultModel });
      await invoke("set_default_vendor", { vendor: defaultVendor.value });
    } catch (err) {
      console.error("Failed to save default settings:", err);
    }
  }

  onMount(async () => {
    await Promise.all([loadVendors(), loadDefaultSettings()]);
  });
</script>

<Card>
  <CardHeader>
    <CardTitle>Default Model Settings</CardTitle>
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
  <CardFooter class="flex gap-2">
    <Button on:click={saveDefaultSettings}>Save Settings</Button>
  </CardFooter>
</Card>
