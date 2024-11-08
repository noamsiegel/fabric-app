<script lang="ts">
  // components
  import { Button } from "$lib/components/ui/button";
  import { toast } from "svelte-sonner";

  // icons
  import CircleCheck from "lucide-svelte/icons/circle-check";

  // tauri
  import { invoke } from "@tauri-apps/api/core";

  // stores
  import { currentContextStore } from "$lib/stores/context";

  // props
  export let name: string;

  async function setCurrentContext() {
    try {
      await invoke("set_current_context", { context: name });
      currentContextStore.set(name);
      toast.success("Current context updated", {
        description: `${name} has been set as the current context`,
        duration: 2000,
      });
    } catch (error) {
      toast.error("Failed to set current context", {
        description: "There was an error updating the current context",
      });
      console.error("Failed to set current context:", error);
    }
  }
</script>

<div class="flex gap-2">
  <Button
    variant="ghost"
    size="icon"
    class="h-8 w-8 p-0"
    onclick={setCurrentContext}
  >
    <CircleCheck class="h-4 w-4" />
    <span class="sr-only">Set {name} as current context</span>
  </Button>
</div>
