<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Button } from "$lib/components/ui/button";
  import { createEventDispatcher } from "svelte";

  export let disabled = false;
  const dispatch = createEventDispatcher();
  let loading = false;

  async function handleRefresh() {
    try {
      loading = true;
      await invoke("refresh_models");
      dispatch("refreshComplete");
    } catch (err) {
      console.error("Failed to refresh models:", err);
    } finally {
      loading = false;
    }
  }
</script>

<Button
  variant="outline"
  size="sm"
  on:click={handleRefresh}
  disabled={loading || disabled}
>
  {loading ? "Refreshing..." : "Refresh Models"}
</Button>
