<!-- ModelsTable.svelte -->
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
  import * as Select from "$lib/components/ui/select";

  // svelte
  import { onMount } from "svelte";
  import { writable, type Writable } from "svelte/store";
  import { Loader2 } from "lucide-svelte";

  // table components
  import Actions from "./actions.svelte";
  import RefreshButton from "../buttons/refresh-models.svelte";

  // tauri
  import { invoke } from "@tauri-apps/api/core";

  interface Model {
    id: number;
    name: string;
    provider: string;
  }
  let originalModelsData: Model[] = [];
  let modelsData: Writable<Model[]> = writable([]);
  let selectedVendor = { value: "all", label: "All Vendors" };

  // loading
  let isLoading = false;
  let loadTimes = {
    initial: 0,
    filter: 0,
  };

  const table = createTable(modelsData, {
    sort: addSortBy({ disableMultiSort: true }),
    filter: addTableFilter({
      fn: ({ filterValue, value }) =>
        value.toLowerCase().includes(filterValue.toLowerCase()),
    }),
    page: addPagination({ initialPageSize: 10 }),
  });

  // Add this to track unique vendors and filter data
  $: vendors = [...new Set($modelsData.map((model) => model.provider))];
  $: {
    const filteredData = originalModelsData.filter(
      (model) =>
        selectedVendor.value === "all" ||
        model.provider === selectedVendor.value
    );
    modelsData.set(filteredData);
  }

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
    isLoading = true;
    const startTime = performance.now();

    try {
      const formattedModels: Model[] = await invoke("get_models");
      originalModelsData = formattedModels;
      modelsData.set(formattedModels);
    } catch (err) {
      console.error("Failed to fetch models:", err);
    } finally {
      loadTimes.initial = performance.now() - startTime;
      isLoading = false;
    }
  }

  // Track filter timing
  $: {
    const startTime = performance.now();
    const filteredData = originalModelsData.filter(
      (model) =>
        selectedVendor.value === "all" ||
        model.provider === selectedVendor.value
    );
    modelsData.set(filteredData);
    loadTimes.filter = performance.now() - startTime;
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
      <RefreshButton on:refreshComplete={fetchModels} disabled={isLoading} />
      {#if loadTimes.initial > 0}
        <span class="text-xs text-muted-foreground">
          Loaded in {loadTimes.initial.toFixed(2)}ms
        </span>
      {/if}
    </div>
    <div class="flex items-center gap-4">
      {#if loadTimes.filter > 0}
        <span class="text-xs text-muted-foreground">
          Filtered in {loadTimes.filter.toFixed(2)}ms
        </span>
      {/if}
      <Select.Root bind:selected={selectedVendor}>
        <Select.Trigger class="w-[180px]">
          <Select.Value placeholder="Select Vendor" />
        </Select.Trigger>
        <Select.Content>
          <Select.Item value="all">All Vendors</Select.Item>
          {#each vendors as vendor}
            <Select.Item value={vendor}>{vendor}</Select.Item>
          {/each}
        </Select.Content>
      </Select.Root>
    </div>
  </div>

  <!-- table -->
  <div class="rounded-md border">
    {#if isLoading}
      <div
        class="absolute inset-0 flex items-center justify-center bg-background/50 z-50"
      >
        <Loader2 class="h-8 w-8 animate-spin" />
      </div>
    {/if}
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
