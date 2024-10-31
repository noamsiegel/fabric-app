<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { invoke } from "@tauri-apps/api/core";

  let contextsPath: string = "";
  let contextsList: string = "";
  let newContextTitle: string = "";

  async function getContextsDir() {
    try {
      const path = await invoke("get_contexts_dir");
      contextsPath = path as string;
    } catch (error) {
      console.error("Failed to get contexts directory:", error);
    }
  }

  async function listContexts() {
    try {
      console.log("listing contexts");
      const contexts = await invoke("list_contexts");
      console.log("contexts:", contexts);
      contextsList = contexts as string;
      console.log("contextsList:", contextsList);
    } catch (error) {
      console.error("Failed to list contexts:", error);
    }
  }

  async function createContext() {
    if (!newContextTitle.trim()) {
      return;
    }
    try {
      const result = await invoke("create_context_file", {
        title: newContextTitle,
      });
      //  TODO make this show to the user
      console.log("Context created:", result);
      // Refresh the contexts list
      await listContexts();
      // Clear the input
      newContextTitle = "";
    } catch (error) {
      console.error("Failed to create context:", error);
    }
  }
</script>

<div class="flex flex-col gap-4">
  <div class="grid w-full max-w-sm items-center gap-1.5">
    <Label for="new-context">New Context</Label>
    <div class="flex w-full max-w-sm items-center space-x-2">
      <Input
        type="text"
        id="new-context"
        placeholder="Enter context title"
        bind:value={newContextTitle}
      />
      <Button on:click={createContext}>Create</Button>
    </div>
  </div>

  <div class="flex gap-2">
    <Button variant="outline" on:click={getContextsDir}
      >Show Contexts Directory</Button
    >
    <Button variant="outline" on:click={listContexts}>List All Contexts</Button>
  </div>

  {#if contextsPath}
    <p>Contexts Directory: {contextsPath}</p>
  {/if}

  {#if contextsList}
    <p>Available Contexts:</p>
    <pre>{contextsList}</pre>
  {/if}
</div>
