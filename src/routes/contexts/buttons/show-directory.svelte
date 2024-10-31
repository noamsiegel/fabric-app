<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { invoke } from "@tauri-apps/api/core";

  export let onDirectoryFound: (path: string) => void;

  async function getContextsDir() {
    try {
      const path = await invoke("get_contexts_dir");
      onDirectoryFound(path as string);
    } catch (error) {
      console.error("Failed to get contexts directory:", error);
    }
  }
</script>

<Button variant="outline" on:click={getContextsDir}
  >Show Contexts Directory</Button
>
