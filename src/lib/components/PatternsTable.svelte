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

  // components
  import * as Table from "$lib/components/ui/table";
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { ArrowUpDown } from "lucide-svelte";
  import { Edit, Eye } from "lucide-svelte";

  // svelte
  import { onMount } from "svelte";
  import { writable, type Writable } from "svelte/store";

  // tauri
  import { invoke } from "@tauri-apps/api/core";

  interface Pattern {
    id: number;
    name: string;
  }

  let patternsData: Writable<Pattern[]> = writable([]);

  const table = createTable(patternsData, {
    sort: addSortBy({ disableMultiSort: true }),
    filter: addTableFilter(),
    page: addPagination({ initialPageSize: 10 }),
  });

  const columns = table.createColumns([
    table.column({
      header: "ID",
      accessor: "id",
      plugins: {
        sort: { disable: false },
      },
    }),
    table.column({
      header: "Name",
      accessor: "name",
      plugins: {
        sort: { disable: false },
      },
    }),
    // TODO add edit and view options
    // table.column({
    //   id: "actions",
    //   header: "Actions",
    //   accessor: ({ id }) => id, // Add accessor that returns a value we can use to identify the row
    //   cell: ({ row }) => {
    //     return createRender(Button, {
    //       variant: "ghost",
    //       class: "flex gap-2",
    //       children: [
    //         {
    //           component: Edit,
    //           props: {
    //             class: "h-4 w-4",
    //             onClick: () => handleEdit(row.data),
    //           },
    //         },
    //         {
    //           component: Eye,
    //           props: {
    //             class: "h-4 w-4",
    //             onClick: () => handleView(row.data),
    //           },
    //         },
    //       ],
    //     });
    //   },
    //   plugins: {
    //     sort: { disable: true },
    //   },
    // }),
  ]);

  const { headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
    table.createViewModel(columns);
  const { filterValue } = pluginStates.filter;
  const { sortKeys } = pluginStates.sort;
  const { pageIndex, hasNextPage, hasPreviousPage } = pluginStates.page;

  async function fetchPatterns() {
    try {
      const data: string[] = await invoke("get_patterns");
      const formattedPatterns: Pattern[] = data.map((pattern, index) => ({
        id: index + 1,
        name: formatPatternName(pattern),
      }));
      patternsData.set(formattedPatterns);
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

  function handleEdit(pattern: Pattern) {
    console.log("Edit pattern:", pattern);
    // TODO: Implement edit functionality
  }

  function handleView(pattern: Pattern) {
    console.log("View pattern:", pattern);
    // TODO: Implement view functionality
  }

  onMount(fetchPatterns);
</script>

<div class="flex items-center py-4">
  <Input
    placeholder="Filter patterns..."
    bind:value={$filterValue}
    class="max-w-sm"
  />
</div>

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
                  <Button variant="ghost" on:click={props.sort?.toggle}>
                    <Render of={cell.render()} />
                    {#if cell.id !== "actions"}
                      <ArrowUpDown
                        class={`ml-2 h-4 w-4 ${$sortKeys[0]?.id === cell.id ? "text-foreground" : ""}`}
                      />
                    {/if}
                  </Button>
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
                  {#if cell.id === "actions"}
                    <div class="flex gap-2">
                      <Button
                        variant="ghost"
                        size="icon"
                        on:click={() => handleEdit(row.data)}
                      >
                        <Edit class="h-4 w-4" />
                      </Button>
                      <Button
                        variant="ghost"
                        size="icon"
                        on:click={() => handleView(row.data)}
                      >
                        <Eye class="h-4 w-4" />
                      </Button>
                    </div>
                  {:else}
                    <Render of={cell.render()} />
                  {/if}
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
