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

  // svelte components
  import * as Table from "$lib/components/ui/table";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";

  // svelte
  import { onMount } from "svelte";
  import { writable, type Writable } from "svelte/store";

  // table components
  import Actions from "./actions/model-actions.svelte";
  import RefreshButton from "$lib/components/buttons/model-refresh.svelte";

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
    table.column({
      accessor: ({ name }) => name,
      header: "",
      cell: ({ value }) => {
        return createRender(Actions, {
          name: value,
        });
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
          exclude: true,
        },
      },
    }),
  ]);

  const { headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
    table.createViewModel(columns);
  const { pageIndex, hasNextPage, hasPreviousPage } = pluginStates.page;
  const { filterValue } = pluginStates.filter;

  async function fetchModels() {
    try {
      const data: Model[] = await invoke("get_models");
      modelsData.set(data);
    } catch (err) {
      console.error("Failed to fetch models:", err);
    }
  }

  onMount(async () => {
    await fetchModels();
  });
</script>

<!-- 
  TODO maybe make it so that users can no see some of the models depending on the keys they have
  Issue URL: https://github.com/noamsiegel/fabric-app/issues/45
  -->
<!-- Add loading indicator and timing info -->
<div>
  <div class="flex items-center justify-between py-4">
    <div class="flex items-center gap-4">
      <Input
        class="max-w-sm"
        placeholder="Filter models..."
        type="text"
        bind:value={$filterValue}
      />
      <RefreshButton on:refreshComplete={fetchModels} />
    </div>
    <div class="flex items-center gap-4">
      <!-- TODO add vendor filter to models table -->
      <!-- <Select.Root
        type="single"
        value={selectedVendor}
        onValueChange={onVendorSelect}
      >
        <Select.Trigger>
          <span class="block truncate">
            {selectedVendor === "all" ? "All Vendors" : selectedVendor}
          </span>
        </Select.Trigger>
        <Select.Content>
          <Select.Item value="all">All Vendors</Select.Item>
          {#each vendors as vendor}
            <Select.Item value={vendor}>{vendor}</Select.Item>
          {/each}
        </Select.Content>
      </Select.Root> -->
    </div>
  </div>

  <!-- table -->
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

  <!-- pagination -->
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
