<script lang="ts">
  // components
  import { Button } from "$lib/components/ui/button";
  import { toast } from "svelte-sonner";

  // stores
  import { defaultModelStore } from "$lib/stores/models";

  // icons
  import Star from "lucide-svelte/icons/star";

  // tauri
  import { invoke } from "@tauri-apps/api/core";

  // props
  export let name: string;

  async function setDefaultModel() {
    try {
      await invoke("set_default_model", { model: name });
      defaultModelStore.set(name);
      toast.success("Default model updated", {
        description: `${name} has been set as the default model`,
        duration: 2000,
      });
    } catch (error) {
      toast.error("Failed to set default pattern", {
        description: "There was an error updating the default pattern",
      });
      console.error("Failed to set default pattern:", error);
    }
  }
</script>

<div class="flex gap-2">
  <Button
    variant="ghost"
    size="icon"
    class="h-8 w-8 p-0"
    on:click={setDefaultModel}
  >
    <Star class="h-4 w-4" />
    <span class="sr-only">Set {name} as default</span>
  </Button>
</div>
