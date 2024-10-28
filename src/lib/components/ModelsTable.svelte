<!-- ModelsTable.svelte -->
<script lang="ts">
  import { createTable, Render, Subscribe } from "svelte-headless-table";
  import {
    addSortBy,
    addTableFilter,
    addPagination,
  } from "svelte-headless-table/plugins";
  import * as Table from "$lib/components/ui/table";
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { ArrowUpDown } from "lucide-svelte";
  import { onMount } from "svelte";
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
  });

  const columns = table.createColumns([
    table.column({
      header: "ID",
      accessor: "id",
      plugins: {
        sort: { disable: true },
        filter: { exclude: true },
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
  const { filterValue } = pluginStates.filter;
  const { sortKeys } = pluginStates.sort;

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

  onMount(fetchModels);
</script>

<!-- 
TODO move search bar at the top
TODO make it so that there are checkboxes that determine the default model
TODO maybe make it so that users can no see some of the models depending on the keys they have
-->

<div class="rounded-md border">
  <Table.Root {...$tableAttrs}>
    <Table.Header>
      {#each $headerRows as headerRow}
        <Subscribe rowAttrs={headerRow.attrs()}>
          <Table.Row>
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
        <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
          <Table.Row {...rowAttrs}>
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

<div class="flex items-center py-4">
  <Input
    placeholder="Filter models..."
    bind:value={$filterValue}
    class="max-w-sm"
  />
</div>
