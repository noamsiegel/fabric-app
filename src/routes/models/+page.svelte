<script lang="ts">
  // Update imports to explicitly include .svelte extension
  import ModelSettings from "./cards/ModelSettings.svelte";
  import ModelParameters from "./cards/ModelParameters.svelte";
  import Table from "./table/table.svelte";
</script>

<!-- TODO load faster by storing models in file and having a refresh button -->
<!-- Issue URL: https://github.com/noamsiegel/fabric-app/issues/90 -->

<div class="container py-8">
  <h1 class="text-3xl font-bold mb-6">Models</h1>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
    <ModelSettings />

    <ModelParameters />
  </div>
  <Table />
</div>
