<script lang="ts">
  import { createTable, Render, Subscribe } from "svelte-headless-table";
  import {
    addSortBy,
    addTableFilter,
    addPagination,
  } from "svelte-headless-table/plugins";

  import * as Table from "$lib/components/ui/table";
  import { Input } from "$lib/components/ui/input";
  import { Button } from "./ui/button";

  import { ArrowUpDown } from "lucide-svelte";
  import { onMount } from "svelte";
  import { writable, type Writable } from "svelte/store";

  // tauri
  import { invoke } from "@tauri-apps/api/core";
  import { Edit } from "lucide-svelte";

  interface Secret {
    name: string;
    secret: string;
  }

  let secretsData: Writable<Secret[]> = writable([]);

  const table = createTable(secretsData, {
    sort: addSortBy({ disableMultiSort: true }),
    filter: addTableFilter(),
    page: addPagination(),
  });

  const columns = table.createColumns([
    table.column({
      header: "Name",
      accessor: "name",
    }),
    table.column({
      header: "Secret",
      accessor: "secret",
    }),
  ]);

  const { headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
    table.createViewModel(columns);
  const { filterValue } = pluginStates.filter;
  const { sortKeys } = pluginStates.sort;

  async function fetchSecrets() {
    try {
      const data = await invoke<Secret[]>("get_secrets");
      secretsData.set(data);
    } catch (err) {
      console.error("Failed to fetch secrets:", err);
    }
  }

  onMount(fetchSecrets);
</script>

<div class="flex items-center py-4">
  <Input
    placeholder="Filter secrets..."
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
                  <Button variant="ghost" on:click={props.sort.toggle}>
                    <Render of={cell.render()} />
                    <ArrowUpDown
                      class={`ml-2 h-4 w-4 ${$sortKeys[0]?.id === cell.id ? "text-foreground" : ""}`}
                    />
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
