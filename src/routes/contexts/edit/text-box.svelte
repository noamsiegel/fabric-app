<script lang="ts">
  import { Textarea } from "$lib/components/ui/textarea";
  import { Label } from "$lib/components/ui/label";
  export let content: string = "";
</script>

<div class="grid w-full gap-1.5">
  <Label for="message">Message</Label>
  <Textarea
    id="message"
    placeholder="Type your message here..."
    class="min-h-[100px] resize-none"
    bind:value={content}
  />
</div>
