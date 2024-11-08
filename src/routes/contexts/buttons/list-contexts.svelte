<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { invoke } from "@tauri-apps/api/core";

  export let onContextsListed: (contexts: string) => void;

  async function listContexts() {
    try {
      console.log("listing contexts");
      const contexts = await invoke("list_contexts");
      console.log("contexts:", contexts);
      onContextsListed(contexts as string);
    } catch (error) {
      console.error("Failed to list contexts:", error);
    }
  }
</script>

<Button variant="outline" onclick={listContexts}>List All Contexts</Button>
