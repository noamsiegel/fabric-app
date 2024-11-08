<script lang="ts">
  import { Settings } from "lucide-svelte";
  import { settingsOpen } from "$lib/stores/settings";
  import FabricLogoPng from "$lib/static/icons/fabric-logo-png.png";
</script>

<header class="flex justify-between items-center p-4 border-b h-16">
  <div class="flex items-center">
    <a
      href="https://github.com/danielmiessler/fabric"
      class="flex items-center"
    >
      <img src={FabricLogoPng} alt="Fabric Logo" class="h-8 w-8 mr-2" />
    </a>
  </div>

  <div class="flex justify-end">
    <button
      class="text-gray-600 hover:text-gray-900"
      onclick={() => ($settingsOpen = true)}
    >
      <Settings size={24} />
    </button>
  </div>
</header>
