<!-- ModelsTable.svelte -->
<script lang="ts">
  // svelte headless table
  import { createTable, Render, Subscribe } from "svelte-headless-table";
  import {
    addSortBy,
    addTableFilter,
    addPagination,
    addSelectedRows,
  } from "svelte-headless-table/plugins";
  // components
  import * as Table from "$lib/components/ui/table";
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { ArrowUpDown } from "lucide-svelte";
  import { onMount } from "svelte";
  import { Checkbox } from "$lib/components/ui/checkbox";
  // tauri
  import { invoke } from "@tauri-apps/api/core";
  import { writable, type Writable } from "svelte/store";

  interface Model {
    id: number;
    name: string;
    provider: string;
  }

  let modelsData: Writable<Model[]> = writable([]);

  const table = createTable(modelsData, {
    sort: addSortBy({ disableMultiSort: true }),
    filter: addTableFilter(),
    page: addPagination(),
    select: addSelectedRows({
      initialSelectedDataIds: {},
    }),
  });

  const columns = table.createColumns([
    table.column({
      header: "ID",
      accessor: "id",
      plugins: {
        sort: { disable: false },
        filter: { exclude: true },
        select: {},
      },
    }),
    table.column({
      header: "Name",
      accessor: "name",
    }),
    table.column({
      header: "Provider",
      accessor: "provider",
    }),
  ]);

  const { headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
    table.createViewModel(columns);
  const { selectedDataIds } = pluginStates.select;
  const { filterValue } = pluginStates.filter;
  const { sortKeys } = pluginStates.sort;
  const { pageIndex, hasNextPage, hasPreviousPage } = pluginStates.page; // Add pagination states

  async function fetchModels() {
    try {
      const data: string[] = await invoke("get_models");
      const formattedModels: Model[] = [];
      let currentProvider = "";
      let lineIndex = 0;

      for (const line of data) {
        const trimmedLine = line.trim();
        lineIndex++;

        // Skip empty lines and the "Available models:" header
        if (!trimmedLine || trimmedLine === "Available models:") {
          continue;
        }

        // Check if the line is a provider name
        if (["Anthropic", "Groq", "Gemini", "OpenAI"].includes(trimmedLine)) {
          currentProvider = trimmedLine;
          continue;
        }

        // Regex to match lines like "[ID]\tmodel-name"
        const modelRegex = /^\[(\d+)\]\s+(.*)$/;
        const match = modelRegex.exec(trimmedLine);

        if (match) {
          const id = parseInt(match[1], 10);
          const name = match[2].trim();

          formattedModels.push({
            id: id,
            name: name,
            provider: currentProvider,
          });
        } else {
          console.warn(
            `Line ${lineIndex} does not match model pattern: ${trimmedLine}`
          );
        }
      }

      modelsData.set(formattedModels);
    } catch (err) {
      console.error("Failed to fetch models:", err);
    }
  }

  async function handleSetDefaultModel() {
    try {
      console.log("Raw selected data:", $selectedDataIds);

      const selectedIds = Object.entries($selectedDataIds)
        .filter(([_, selected]) => selected)
        .map(([id]) => parseInt(id) + 1);

      console.log("Selected IDs:", selectedIds[0]);
      console.log("Raw selected data:", $selectedDataIds);
      console.log("Selected IDs (full array):", selectedIds);
      console.log("Selected ID type:", typeof selectedIds[0]);
      console.log("Selected ID value:", selectedIds[0]);

      if (selectedIds.length !== 1) {
        alert("Please select exactly one model");
        return;
      }

      // Updated parameter name to match Rust function
      await invoke("set_default_model", { modelId: selectedIds[0] });
    } catch (err) {
      console.error("Failed to set default model:", err);
    }
  }

  onMount(fetchModels);
</script>

<!-- 
TODO maybe make it so that users can no see some of the models depending on the keys they have
-->

<div class="flex items-center py-4">
  <Input
    placeholder="Filter models..."
    bind:value={$filterValue}
    class="max-w-sm"
  />
  <Button on:click={handleSetDefaultModel}>Set Default Model</Button>
</div>

<div class="rounded-md border">
  <Table.Root {...$tableAttrs}>
    <Table.Header>
      {#each $headerRows as headerRow}
        <Subscribe rowAttrs={headerRow.attrs()}>
          <Table.Row>
            <Table.Head class="w-[10px]"></Table.Head>
            {#each headerRow.cells as cell (cell.id)}
              <Subscribe
                attrs={cell.attrs()}
                let:attrs
                props={cell.props()}
                let:props
              >
                <Table.Head {...attrs}>
                  {#if cell.id === "name" || cell.id === "provider"}
                    <Button variant="ghost" on:click={props.sort.toggle}>
                      <Render of={cell.render()} />
                      <ArrowUpDown
                        class={`ml-2 h-4 w-4 ${$sortKeys[0]?.id === cell.id ? "text-foreground" : ""}`}
                      />
                    </Button>
                  {:else}
                    <Render of={cell.render()} />
                  {/if}
                </Table.Head>
              </Subscribe>
            {/each}
          </Table.Row>
        </Subscribe>
      {/each}
    </Table.Header>
    <Table.Body {...$tableBodyAttrs}>
      {#each $pageRows as row}
        <Subscribe
          rowAttrs={row.attrs()}
          let:rowAttrs
          rowProps={row.props()}
          let:rowProps
        >
          <Table.Row {...rowAttrs}>
            <Table.Cell>
              <Checkbox
                checked={rowProps.select.selected}
                onCheckedChange={() => {
                  const state = pluginStates.select.getRowState(row);
                  state.isSelected.set(!rowProps.select.selected);
                }}
              />
            </Table.Cell>
            {#each row.cells as cell (cell.id)}
              <Subscribe attrs={cell.attrs()} let:attrs>
                <Table.Cell {...attrs}>
                  <Render of={cell.render()} />
                </Table.Cell>
              </Subscribe>
            {/each}
          </Table.Row>
        </Subscribe>
      {/each}
    </Table.Body>
  </Table.Root>
</div>

<div class="flex items-center justify-end space-x-2 py-4">
  <Button
    variant="outline"
    size="sm"
    on:click={() => ($pageIndex = $pageIndex - 1)}
    disabled={!$hasPreviousPage}
  >
    Previous
  </Button>
  <Button
    variant="outline"
    size="sm"
    disabled={!$hasNextPage}
    on:click={() => ($pageIndex = $pageIndex + 1)}
  >
    Next
  </Button>
</div>
