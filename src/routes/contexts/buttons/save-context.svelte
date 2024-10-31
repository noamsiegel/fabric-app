<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { invoke } from "@tauri-apps/api/core";

  export let selectedTitle: string;
  export let content: string;

  async function handleSave() {
    try {
      await invoke("save_context_file", {
        title: selectedTitle,
        content: content,
      });
      console.log("Context saved successfully");
    } catch (error) {
      console.error("Error saving context:", error);
    }
  }
</script>

<Button
  variant="outline"
  size="sm"
  on:click={handleSave}
  disabled={!selectedTitle}
>
  Save
</Button>
