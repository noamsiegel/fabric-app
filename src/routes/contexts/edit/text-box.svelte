<script lang="ts">
  import { Textarea } from "$lib/components/ui/textarea";
  import { Label } from "$lib/components/ui/label";

  // buttons
  import DeleteContextButton from "../buttons/delete-context.svelte";
  import SaveContextButton from "../buttons/save-context.svelte";

  export let content: string = "";
  export let selectedTitle: string = "";
  export let onContextDeleted: () => void;
</script>

<div class="grid w-full gap-1.5">
  <div class="flex justify-between items-center">
    <Label for="message">{selectedTitle || "Message"}</Label>
    <div class="flex gap-2">
      <SaveContextButton {selectedTitle} {content} />
      <DeleteContextButton {selectedTitle} bind:content {onContextDeleted} />
    </div>
  </div>
  <Textarea
    id="message"
    placeholder="Type your message here..."
    class="min-h-[100px] resize-none"
    bind:value={content}
  />
</div>
