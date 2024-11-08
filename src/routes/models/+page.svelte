<script lang="ts">
  // Update imports to explicitly include .svelte extension
  import ModelSettings from "$lib/components/cards/model-settings.svelte";
  import Table from "../../lib/components/tables/models.svelte";
</script>

<!-- TODO load faster by storing models in file and having a refresh button -->

<div>
  <h1 class="text-3xl font-bold mb-6">Models</h1>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
    <ModelSettings />
    <!-- <ModelParameters /> -->
  </div>
  <Table />
</div>
