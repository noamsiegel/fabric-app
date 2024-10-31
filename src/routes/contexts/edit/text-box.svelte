<script lang="ts">
  import { Textarea } from "$lib/components/ui/textarea";

  // buttons
  import DeleteContextButton from "../buttons/delete-context.svelte";
  import SaveContextButton from "../buttons/save-context.svelte";

  export let content: string = "";
  export let selectedTitle: string = "";
  export let onContextDeleted: () => void;
</script>

<div class="grid w-full gap-1.5">
  <div class="flex justify-between items-center">
    <div class="pb-2 pt-4">
      <h3 class="font-semibold text-lg">{selectedTitle || "Context Name"}</h3>
    </div>
    <div class="flex gap-2 justify-end pb-2 pt-4">
      <SaveContextButton {selectedTitle} {content} />
      <DeleteContextButton {selectedTitle} bind:content {onContextDeleted} />
    </div>
  </div>
  <Textarea
    id="context"
    placeholder="Type your context here..."
    class="min-h-[100px] resize-none"
    bind:value={content}
  />
</div>
