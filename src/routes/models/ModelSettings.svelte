<script lang="ts">
  // svelte components
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

  // svelte
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";

  // Tauri
  import { invoke } from "@tauri-apps/api/core";

  let defaultModel: string = "";
  let vendors: string[] = [];
  let defaultVendor: VendorOption = { value: "", label: "" };
  let isUpdating = false;

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
      const model = (await invoke("get_secret", {
        key: "DEFAULT_MODEL",
      })) as string;
      defaultModel = model;
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
      isUpdating = true;
      await invoke("update_secret", {
        key: "DEFAULT_MODEL",
        value: defaultModel,
      });
      await invoke("update_secret", {
        key: "DEFAULT_VENDOR",
        value: defaultVendor.value,
      });
      toast.success("Settings saved successfully");
    } catch (err) {
      console.error("Failed to save default settings:", err);
      toast.error("Failed to save settings");
    } finally {
      isUpdating = false;
    }
  }

  function handleInputClick() {
    toast.info("Set Default Model", {
      description:
        "To change the default model, click the star icon (⭐︎) next to a model name in the table below.",
      duration: 4000,
    });
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
        value={defaultModel}
        readonly
        disabled={false}
        class="bg-white font-medium text-black cursor-help"
        placeholder="gpt-4-turbo-preview"
        on:click={handleInputClick}
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
