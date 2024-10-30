<script lang="ts">
  // svelte headless components
  import {
    createTable,
    Render,
    Subscribe,
    createRender,
  } from "svelte-headless-table";
  import {
    addSortBy,
    addTableFilter,
    addPagination,
  } from "svelte-headless-table/plugins";

  //svelte components
  import * as Table from "$lib/components/ui/table";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";

  // svelte
  import { onMount, onDestroy } from "svelte";
  import { writable, type Writable } from "svelte/store";
  import { browser } from "$app/environment";

  // table components
  //   import Actions from "./model-actions.svelte";

  // tauri
  import { invoke } from "@tauri-apps/api/core";

  interface Model {
    id: number;
    name: string;
    provider: string;
  }

  let modelsData: Writable<Model[]> = writable([]);

  const table = createTable(modelsData, {
    sort: addSortBy({ disableMultiSort: true }),
    filter: addTableFilter({
      fn: ({ filterValue, value }) =>
        value.toLowerCase().includes(filterValue.toLowerCase()),
    }),
    page: addPagination({ initialPageSize: 10 }),
  });

  const columns = table.createColumns([
    // table.column({
    //   header: "ID",
    //   accessor: "id",
    //   plugins: {
    //     sort: { disable: false },
    //     filter: {
    //       exclude: true,
    //     },
    //   },
    // }),
    table.column({
      accessor: ({ name }) => name,
      header: "",
      cell: ({ value }) => {
        return createRender(Actions, { name: value });
      },
    }),
    table.column({
      header: "Name",
      accessor: "name",
      plugins: {
        sort: { disable: false },
        filter: {
          exclude: false,
        },
      },
    }),
    table.column({
      header: "Provider",
      accessor: "provider",
      plugins: {
        sort: { disable: false },
        filter: {
          exclude: false,
        },
      },
    }),
  ]);

  const { headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
    table.createViewModel(columns);
  const { hasNextPage, hasPreviousPage, pageIndex } = pluginStates.page;
  const { filterValue } = pluginStates.filter;

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

  let mounted = false;

  onMount(async () => {
    if (browser && !mounted) {
      mounted = true;
      await fetchModels();
    }
  });

  onDestroy(() => {
    mounted = false;
  });
</script>

<div>
  <div class="flex items-center py-4">
    <Input
      class="max-w-sm"
      placeholder="Filter patterns..."
      type="text"
      bind:value={$filterValue}
    />
  </div>
  <div class="rounded-md border">
    <Table.Root {...$tableAttrs}>
      <Table.Header>
        {#each $headerRows as headerRow}
          <Subscribe rowAttrs={headerRow.attrs()}>
            <Table.Row>
              {#each headerRow.cells as cell (cell.id)}
                <Subscribe attrs={cell.attrs()} let:attrs props={cell.props()}>
                  <Table.Head {...attrs}>
                    <Render of={cell.render()} />
                  </Table.Head>
                </Subscribe>
              {/each}
            </Table.Row>
          </Subscribe>
        {/each}
      </Table.Header>
      <Table.Body {...$tableBodyAttrs}>
        {#each $pageRows as row (row.id)}
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
  <div class="flex items-center justify-end space-x-4 py-4">
    <Button
      variant="outline"
      size="sm"
      on:click={() => ($pageIndex = $pageIndex - 1)}
      disabled={!$hasPreviousPage}>Previous</Button
    >
    <Button
      variant="outline"
      size="sm"
      disabled={!$hasNextPage}
      on:click={() => ($pageIndex = $pageIndex + 1)}>Next</Button
    >
  </div>
</div>
