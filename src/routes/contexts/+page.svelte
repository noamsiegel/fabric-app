<script lang="ts">
  import Table from "./table/table.svelte";
  import TextBox from "./edit/text-box.svelte";
  import * as Resizable from "$lib/components/ui/resizable";
  import { writable } from "svelte/store";

  const selectedContent = writable("");
  const selectedTitle = writable("");
  let refreshContexts: () => Promise<void>;
</script>

<!-- TODO add ability to change current context (both --context and .env)-->
<!-- TODO add ability to see current context (rust fn + .env)-->
<!-- TODO add ability to wipe context (maybe) -->

<div class="p-4">
  <h1 class="text-2xl font-bold mb-4">Context Browser</h1>
  <Resizable.PaneGroup
    direction="horizontal"
    class="min-h-[600px] border rounded-lg gap-4"
  >
    <Resizable.Pane defaultSize={40} class="pl-4">
      <div class="h-full p-2">
        <Table
          {selectedContent}
          {selectedTitle}
          onContextsChange={(fn) => (refreshContexts = fn)}
        />
      </div>
    </Resizable.Pane>
    <Resizable.Handle withHandle class="mx-2" />
    <Resizable.Pane defaultSize={60} class="pr-4">
      <div class="h-full p-2">
        <TextBox
          content={$selectedContent}
          selectedTitle={$selectedTitle}
          onContextDeleted={refreshContexts}
        />
      </div>
    </Resizable.Pane>
  </Resizable.PaneGroup>
</div>
