<script lang="ts">
  // components
  import { Button } from "$lib/components/ui/button";
  import { toast } from "svelte-sonner";

  // stores
  import { defaultModelStore } from "$lib/stores/models";
  import { Loader2 } from "lucide-svelte";

  // icons
  import Star from "lucide-svelte/icons/star";

  // tauri
  import { invoke } from "@tauri-apps/api/core";

  // props
  export let name: string;

  let isLoading = false;
  let loadTime = 0;

  async function setDefaultModel() {
    isLoading = true;
    const startTime = performance.now();

    try {
      await invoke("update_secret", {
        key: "DEFAULT_MODEL",
        value: name,
      });
      defaultModelStore.set(name);
      toast.success("Default model updated", {
        description: `${name} has been set as the default model (${loadTime.toFixed(2)}ms)`,
        duration: 2000,
      });
    } catch (error) {
      toast.error("Failed to set default pattern", {
        description: "There was an error updating the default pattern",
      });
      console.error("Failed to set default pattern:", error);
    } finally {
      loadTime = performance.now() - startTime;
      isLoading = false;
    }
  }
</script>

<div class="flex gap-2">
  <Button
    variant="ghost"
    size="icon"
    class="h-8 w-8 p-0"
    on:click={setDefaultModel}
    disabled={isLoading}
  >
    {#if isLoading}
      <Loader2 class="h-4 w-4 animate-spin" />
    {:else}
      <Star class="h-4 w-4" />
    {/if}
    <span class="sr-only">Set {name} as default</span>
  </Button>
</div>
