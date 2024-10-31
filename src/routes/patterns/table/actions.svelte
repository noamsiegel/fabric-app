<script lang="ts">
  // components
  import { Button } from "$lib/components/ui/button";
  import { toast } from "svelte-sonner";

  // icons
  import Edit from "lucide-svelte/icons/pen";
  import Eye from "lucide-svelte/icons/eye";
  import Star from "lucide-svelte/icons/star";

  // stores
  import { defaultPatternStore } from "$lib/stores/pattern";

  // tauri
  import { invoke } from "@tauri-apps/api/core";

  // props
  export let name: string;

  async function setDefaultPattern() {
    try {
      await invoke("set_default_pattern", { pattern: name });
      defaultPatternStore.set(name);
      toast.success("Default pattern updated", {
        description: `${name} has been set as the default pattern`,
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
  <Button variant="ghost" size="icon" class="h-8 w-8 p-0">
    <Edit class="h-4 w-4" />
    <span class="sr-only">Edit {name}</span>
  </Button>

  <Button variant="ghost" size="icon" class="h-8 w-8 p-0">
    <Eye class="h-4 w-4" />
    <span class="sr-only">View {name}</span>
  </Button>

  <Button
    variant="ghost"
    size="icon"
    class="h-8 w-8 p-0"
    on:click={setDefaultPattern}
  >
    <Star class="h-4 w-4" />
    <span class="sr-only">Set {name} as default</span>
  </Button>
</div>
