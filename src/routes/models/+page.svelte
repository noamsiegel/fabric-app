<script lang="ts">
  import ModelsTable from "./ModelsTable.svelte";
  import DefaultModelSettings from "./DefaultModelSettings.svelte";
  import DefaultModelParameters from "./ModelParameters.svelte";
</script>

<div class="container py-8">
  <h1 class="text-3xl font-bold mb-6">Models</h1>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
    <DefaultModelSettings />
    <DefaultModelParameters />
  </div>
  <ModelsTable />
</div>
