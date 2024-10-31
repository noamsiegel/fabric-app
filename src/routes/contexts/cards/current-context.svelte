<script lang="ts">
  import { onMount } from "svelte";
  import { Input } from "$lib/components/ui/input";
  import { invoke } from "@tauri-apps/api/core";
  import { currentContextStore } from "$lib/stores/context";

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
  <label for="currentContext">Current Context</label>
  <Input
    id="currentContext"
    value={$currentContextStore}
    readonly
    disabled={false}
    class="bg-white font-medium text-black"
  />
</div>
