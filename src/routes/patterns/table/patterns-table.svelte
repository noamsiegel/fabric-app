<script lang="ts">
  // svelte headless table
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
  import { onMount } from "svelte";
  import { writable, type Writable } from "svelte/store";

  // table components
  import Actions from "./actions.svelte";

  // tauri
  import { invoke } from "@tauri-apps/api/core";

  // stores
  import { defaultPatternStore } from "$lib/stores/pattern";

  interface Pattern {
    id: number;
    name: string;
  }

  let patternsData: Writable<Pattern[]> = writable([]);

  const table = createTable(patternsData, {
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
      accessor: ({ name }) => name,
      header: "",
      cell: ({ value }) => {
        return createRender(Actions, { defaultPattern: value });
      },
    }),
  ]);

  const { headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
    table.createViewModel(columns);
  const { hasNextPage, hasPreviousPage, pageIndex } = pluginStates.page;
  const { filterValue } = pluginStates.filter;

  async function fetchPatterns() {
    try {
      const data: string[] = await invoke("get_patterns");
      const formattedPatterns: Pattern[] = data.map((pattern, index) => ({
        id: index + 1,
        name: formatPatternName(pattern),
      }));
      patternsData.set(formattedPatterns);

      // Add this: Get and set default pattern when patterns are loaded
      const defaultPattern = (await invoke("get_default_pattern")) as string;
      defaultPatternStore.set(defaultPattern);
    } catch (err) {
      console.error("Failed to fetch patterns:", err);
    }
  }

  function formatPatternName(name: string): string {
    return name
      .split("_")
      .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
      .join(" ");
  }

  onMount(async () => {
    await fetchPatterns();
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
      onclick={() => ($pageIndex = $pageIndex - 1)}
      disabled={!$hasPreviousPage}>Previous</Button
    >
    <Button
      variant="outline"
      size="sm"
      disabled={!$hasNextPage}
      onclick={() => ($pageIndex = $pageIndex + 1)}>Next</Button
    >
  </div>
</div>
