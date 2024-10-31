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
  import { onMount } from "svelte";
  import { writable, type Writable } from "svelte/store";

  // buttons
  import CreateContext from "../buttons/create-context.svelte";
  export let selectedContent: Writable<string>;
  export let selectedTitle: Writable<string>;
  export let onContextsChange: (fn: () => Promise<void>) => void;

  // tauri
  import { invoke } from "@tauri-apps/api/core";

  interface Context {
    name: string;
  }

  let contextsData: Writable<Context[]> = writable([]);

  const table = createTable(contextsData, {
    sort: addSortBy({ disableMultiSort: true }),
    filter: addTableFilter({
      fn: ({ filterValue, value }) =>
        value.toLowerCase().includes(filterValue.toLowerCase()),
    }),
    page: addPagination({ initialPageSize: 10 }),
  });

  // clicking row -> puts context markdown in textbox
  const handleRowClick = async (title: string) => {
    try {
      const content = await invoke("read_context_file", { title });
      if (typeof content === "string") {
        selectedContent.set(content);
        selectedTitle.set(title);
        console.log("Context file contents:", content);
      }
    } catch (error) {
      console.error("Error reading context file:", error);
    }
  };

  const columns = table.createColumns([
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
    // table.column({
    //   accessor: ({ name }) => name,
    //   header: "",
    //   cell: ({ value }) => {
    //     return createRender(Actions, { name: value });
    //   },
    // }),
  ]);

  $: console.log("contextsData value:", $contextsData);
  $: console.log("pageRows value:", $pageRows);

  const { headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
    table.createViewModel(columns);
  const { hasNextPage, hasPreviousPage, pageIndex } = pluginStates.page;
  const { filterValue } = pluginStates.filter;

  function formatContextName(name: string): string {
    return name.replace(".md", "");
  }

  async function fetchContexts() {
    try {
      const data = await invoke("list_contexts");
      if (typeof data === "string") {
        const contextsList = data
          .split("\n")
          .filter((context) => context.trim());

        const formattedContexts: Context[] = contextsList.map((context) => ({
          name: formatContextName(context),
        }));

        console.log("formattedContexts:", formattedContexts);
        contextsData.set(formattedContexts);
      } else {
        console.error("Invalid data format received:", data);
        contextsData.set([]);
      }
    } catch (err) {
      console.error("Failed to fetch contexts:", err);
    }
  }

  onMount(async () => {
    await fetchContexts();
    onContextsChange(fetchContexts);
    console.log("Table rows:", $pageRows);
  });
</script>

<div>
  <div class="flex items-center justify-between py-4">
    <Input
      class="max-w-sm"
      placeholder="Filter contexts..."
      type="text"
      bind:value={$filterValue}
    />
    <CreateContext onContextCreated={fetchContexts} />
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
            <Table.Row
              {...rowAttrs}
              on:click={() => handleRowClick(row.original.name)}
              class="cursor-pointer"
            >
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
