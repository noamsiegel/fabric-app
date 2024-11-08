<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Dialog from "$lib/components/ui/dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { Plus } from "lucide-svelte"; // Add this import for the plus icon

  export let newContextTitle: string = "";
  export let onContextCreated: () => Promise<void>;
  let open = false;

  async function createContext() {
    if (!newContextTitle.trim()) {
      return;
    }
    try {
      const result = await invoke("create_context_file", {
        title: newContextTitle,
      });
      console.log("Context created:", result);
      await onContextCreated();
      open = false;
      newContextTitle = ""; // Reset input after successful creation
    } catch (error) {
      console.error("Failed to create context:", error);
    }
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Trigger>
    <Button variant="outline" size="icon">
      <Plus class="h-4 w-4" />
      <span class="sr-only">Create Context</span>
    </Button>
  </Dialog.Trigger>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Create Context</Dialog.Title>
      <Dialog.Description>
        Enter a title for your new context.
      </Dialog.Description>
    </Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="grid gap-2">
        <Input
          id="title"
          bind:value={newContextTitle}
          placeholder="Enter context title..."
        />
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (open = false)}>Cancel</Button>
      <Button onclick={createContext}>Create</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
