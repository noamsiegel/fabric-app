<script lang="ts">
  import {
    Card,
    CardContent,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { ModelSettingsManager } from "$lib/managers/ModelManager";
  import { defaultModelStore, defaultVendorStore } from "$lib/stores/models";
  import ModelVendors from "$lib/components/search-box/ModelVendors.svelte";

  const manager = new ModelSettingsManager();

  $effect(() => {
    Promise.all([
      manager.loadVendors(),
      manager.loadDefaultModel(),
      manager.loadDefaultVendor(),
    ]);
  });
</script>

<Card>
  <CardHeader class="flex flex-row items-center justify-between">
    <CardTitle>Default Model Settings</CardTitle>
    <Button variant="outline" onclick={() => manager.resetDefaults()}>
      Reset
    </Button>
  </CardHeader>
  <CardContent class="space-y-4">
    <div class="space-y-2">
      <label for="defaultModel">Default Model</label>
      <Input
        id="defaultModel"
        bind:value={$defaultModelStore}
        readonly={true}
        onchange={async () => {
          await manager.saveDefaultModel($defaultModelStore);
        }}
        placeholder="gpt-4-turbo-preview"
      />
    </div>
    <div class="space-y-2">
      <label for="defaultVendor">Default Vendor</label>
      <ModelVendors />
    </div>
  </CardContent>
</Card>
