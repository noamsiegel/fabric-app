<script lang="ts">
  import {
    Card,
    CardContent,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Select from "$lib/components/ui/select"; 
  import { onMount } from "svelte";
  import {
    ModelSettingsManager,
    selectedVendor,
  } from "$lib/managers/ModelSettings";
  import { defaultModelStore } from "$lib/stores/models";

  const manager = new ModelSettingsManager();

  onMount(async () => {
    await Promise.all([
      manager.loadVendors(),
      manager.loadDefaultModel(),
      manager.loadDefaultVendor(),
    ]);
  });
</script>

<Card>
  <CardHeader class="flex flex-row items-center justify-between">
    <CardTitle>Default Model Settings</CardTitle>
    <Button variant="outline" on:click={() => manager.resetDefaults()}
      >Reset</Button
    >
  </CardHeader>
  <CardContent class="space-y-4">
    <div class="space-y-2">
      <label for="defaultModel">Default Model</label>
      <Input
        id="defaultModel"
        bind:value={$defaultModelStore}
        readonly={true}
        on:change={async () => {
          await manager.saveDefaultModel($defaultModelStore);
        }}
        placeholder="gpt-4-turbo-preview"
      />
    </div>
    <div class="space-y-2">
      <label for="defaultVendor">Default Vendor</label>
      <Select.Root
        bind:selected={$selectedVendor}
        onSelectedChange={(value) => {
          if (value?.value) {
            manager.saveDefaultVendor(value.value);
          }
        }}
      >
        <Select.Trigger class="w-full">
          <Select.Value placeholder="Select vendor" />
        </Select.Trigger>
        <Select.Content>
          {#each manager.vendors as vendor}
            <Select.Item value={vendor}>
              {vendor}
            </Select.Item>
          {/each}
        </Select.Content>
      </Select.Root>
    </div>
  </CardContent>
</Card>
