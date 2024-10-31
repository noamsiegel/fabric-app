<script lang="ts">
  import Table from "./table/table.svelte";
  import TextBox from "./edit/text-box.svelte";
  import * as Resizable from "$lib/components/ui/resizable";
  import { writable } from "svelte/store";

  const selectedContent = writable("");
  const selectedTitle = writable("");
  let refreshContexts: () => Promise<void>;
</script>

<div>
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
