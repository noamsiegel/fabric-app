<script lang="ts">
  import { Textarea } from "$lib/components/ui/textarea";
  import { Label } from "$lib/components/ui/label";
  import { Button } from "$lib/components/ui/button";
  import { invoke } from "@tauri-apps/api/core";

  export let content: string = "";
  export let selectedTitle: string = "";

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

<!-- TODO add ability to save context -->
<!-- TODO make this show the context name -->

<div class="grid w-full gap-1.5">
  <div class="flex justify-between items-center">
    <Label for="message">{selectedTitle || "Message"}</Label>
    <Button
      variant="outline"
      size="sm"
      on:click={handleSave}
      disabled={!selectedTitle}
    >
      Save
    </Button>
  </div>
  <Textarea
    id="message"
    placeholder="Type your message here..."
    class="min-h-[100px] resize-none"
    bind:value={content}
  />
</div>
