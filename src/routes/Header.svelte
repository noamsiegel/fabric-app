<script lang="ts">
  import { Settings, FileText, Database, Home } from "lucide-svelte";
  import { settingsOpen } from "$lib/stores/settings";
</script>

<header class="flex justify-between items-center p-4 border-b">
  <!-- Left section -->
  <div class="flex items-center w-1/4">
    <h1 class="text-2xl font-bold">Fabric</h1>
  </div>

  <!-- Center section -->
  <nav class="flex justify-center space-x-8 flex-1">
    <a href="/" class="text-gray-600 hover:text-gray-900"><Home size={24} /></a>
    <a href="/patterns" class="text-gray-600 hover:text-gray-900"
      ><FileText size={24} /></a
    >
    <a href="/models2" class="text-gray-600 hover:text-gray-900"
      ><Database size={24} /></a
    >
  </nav>

  <!-- Right section -->
  <div class="flex justify-end w-1/4">
    <button
      class="text-gray-600 hover:text-gray-900"
      on:click={() => ($settingsOpen = true)}
    >
      <Settings size={24} />
    </button>
  </div>
</header>
