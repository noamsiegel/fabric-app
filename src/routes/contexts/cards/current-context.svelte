<script lang="ts">
  import { onMount } from "svelte";
  import { Input } from "$lib/components/ui/input";
  import { invoke } from "@tauri-apps/api/core";
  import { currentContextStore } from "$lib/stores/context";
  import { Button } from "$lib/components/ui/button";

  async function loadCurrentContext() {
    try {
      const context = await invoke("get_secret", { key: "CURRENT_CONTEXT" });
      currentContextStore.set(context as string);
    } catch (err) {
      console.error("Failed to load current context:", err);
    }
  }

  onMount(async () => {
    await loadCurrentContext();
  });
</script>

<div class="space-y-2 mb-4">
  <div class="pb-2 pt-4">
    <h3 class="font-semibold text-lg">Current Context</h3>
  </div>
  <div class="flex gap-2">
    <Input
      id="currentContext"
      value={$currentContextStore || "No current context set"}
      readonly
      disabled={false}
      class="bg-white font-medium text-black"
    />
    <Button
      variant="secondary"
      size="default"
      on:click={async () => {
        await invoke("reset_secret", { key: "CURRENT_CONTEXT" });
        await loadCurrentContext();
      }}
    >
      Reset
    </Button>
  </div>
</div>
