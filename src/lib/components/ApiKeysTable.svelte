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
  import * as Dialog from "$lib/components/ui/dialog";
  import { Label } from "$lib/components/ui/label";

  import { ArrowUpDown, Pencil } from "lucide-svelte";
  import { onMount } from "svelte";
  import { writable, type Writable } from "svelte/store";

  // tauri
  import { invoke } from "@tauri-apps/api/core";

  interface Secret {
    name: string;
    secret: string;
  }

  let editedValue = "";
  let secretsData: Writable<Secret[]> = writable([]);
  let dialogOpen = false;

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
      header: "API Key",
      accessor: "secret",
    }),
  ]);

  async function handleEdit(row: any) {
    const secret = {
      name: row.cells[0].value,
      secret: editedValue,
    };

    try {
      await invoke("update_secret", {
        key: secret.name,
        value: secret.secret,
      });

      secretsData.update((secrets) =>
        secrets.map((s) =>
          s.name === secret.name ? { ...s, secret: secret.secret } : s
        )
      );
      dialogOpen = false;
    } catch (err) {
      console.error("Failed to update secret:", err);
    }
  }

  const { headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
    table.createViewModel(columns);
  const { filterValue } = pluginStates.filter;
  const { sortKeys } = pluginStates.sort;

  async function fetchApiKeys() {
    try {
      const data = await invoke<Secret[]>("get_api_keys");
      secretsData.set(data);
    } catch (err) {
      console.error("Failed to fetch secrets:", err);
    }
  }

  onMount(fetchApiKeys);
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
            <Table.Head>Actions</Table.Head>
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
            <!-- Button in each row -->
            <Table.Cell>
              <Dialog.Root bind:open={dialogOpen}>
                <Dialog.Trigger asChild let:builder>
                  <Button variant="outline" builders={[builder]}>
                    <Pencil class="h-4 w-4 mr-2" />
                    Edit
                  </Button>
                </Dialog.Trigger>
                <Dialog.Content class="sm:max-w-[425px]">
                  <Dialog.Header>
                    <Dialog.Title>Edit API Key</Dialog.Title>
                    <Dialog.Description>
                      Update the API key value. Click save when you're done.
                    </Dialog.Description>
                  </Dialog.Header>
                  <div class="grid gap-4 py-4">
                    <div class="grid grid-cols-4 items-center gap-4">
                      <Label class="text-right">Name</Label>
                      <div class="col-span-3">
                        {row.cells[0].value}
                      </div>
                    </div>
                    <div class="grid grid-cols-4 items-center gap-4">
                      <Label for="secret" class="text-right">API Key</Label>
                      <Input
                        id="secret"
                        bind:value={editedValue}
                        class="col-span-3"
                        placeholder="Enter new API key value"
                      />
                    </div>
                  </div>
                  <Dialog.Footer>
                    <Dialog.Close asChild let:builder>
                      <Button variant="outline" builders={[builder]}
                        >Cancel</Button
                      >
                    </Dialog.Close>
                    <Button on:click={() => handleEdit(row)}
                      >Save changes</Button
                    >
                  </Dialog.Footer>
                </Dialog.Content>
              </Dialog.Root>
            </Table.Cell>
          </Table.Row>
        </Subscribe>
      {/each}
    </Table.Body>
  </Table.Root>
</div>
