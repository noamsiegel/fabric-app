<script lang="ts">
  import Table from "./table/table.svelte";
  import TextBox from "./edit/text-box.svelte";
  import * as Resizable from "$lib/components/ui/resizable";
</script>

<div class="p-4">
  <h1 class="text-2xl font-bold mb-4">Pattern Browser</h1>
  <Resizable.PaneGroup
    direction="horizontal"
    class="min-h-[600px] border rounded-lg gap-4"
  >
    <Resizable.Pane defaultSize={40} class="pl-4">
      <div class="h-full p-2">
        <Table />
      </div>
    </Resizable.Pane>
    <Resizable.Handle withHandle class="mx-2" />
    <Resizable.Pane defaultSize={60} class="pr-4">
      <div class="h-full p-2">
        <TextBox />
      </div>
    </Resizable.Pane>
  </Resizable.PaneGroup>
</div>
