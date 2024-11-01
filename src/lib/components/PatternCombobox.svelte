<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { tick } from "svelte";
  import Check from "lucide-svelte/icons/check";
  import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import * as Command from "$lib/components/ui/command/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { cn } from "$lib/utils.js";

  export let patterns: string[] = [];
  export let selectedPattern: string = "";

  let open = false;
  let searchTerm = "";
  let filteredPatterns: string[] = [];

  $: {
    filteredPatterns = patterns.filter((pattern) =>
      pattern.toLowerCase().includes(searchTerm.toLowerCase())
    );
    // console.log("Search term updated:", searchTerm);
    // console.log("Filtered patterns:", filteredPatterns);
  }

  const dispatch = createEventDispatcher();

  function closeAndFocusTrigger(triggerId: string) {
    open = false;
    searchTerm = ""; // Reset search term when closing
    tick().then(() => {
      document.getElementById(triggerId)?.focus();
    });
  }

  function handleSelect(value: string, triggerId: string) {
    // selectedPattern = value;
    // closeAndFocusTrigger(triggerId);
    // dispatch("select", value);
    selectedPattern = value;
    dispatch("select", value);
    // Only close if the selection wasn't made during a search
    closeAndFocusTrigger(triggerId);
  }
</script>

<Popover.Root bind:open let:ids>
  <Popover.Trigger asChild let:builder>
    <Button
      builders={[builder]}
      variant="outline"
      role="combobox"
      aria-expanded={open}
      class="w-[800px] justify-between"
    >
      {selectedPattern || "Select pattern..."}
      <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
    </Button>
  </Popover.Trigger>
  <Popover.Content class="w-[700px] p-0">
    <Command.Root>
      <Command.Input
        placeholder="Search pattern..."
        class="h-9"
        bind:value={searchTerm}
      />
      <Command.Empty>No pattern found.</Command.Empty>
      <Command.Group>
        {#each filteredPatterns as pattern (pattern)}
          <Command.Item
            value={pattern}
            onSelect={() => handleSelect(pattern, ids.trigger)}
          >
            <Check
              class={cn(
                "mr-2 h-4 w-4",
                selectedPattern !== pattern && "text-transparent"
              )}
            />
            {pattern}
          </Command.Item>
        {/each}
      </Command.Group>
    </Command.Root>
  </Popover.Content>
</Popover.Root>
