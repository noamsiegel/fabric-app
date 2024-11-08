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
  import { Eye, EyeOff } from "lucide-svelte"; // Add these imports

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
  let visibleSecrets: Writable<Set<string>> = writable(new Set());

  const table = createTable(secretsData, {
    sort: addSortBy({ disableMultiSort: true }),
    filter: addTableFilter(),
    page: addPagination(),
  });

  const columns = table.createColumns([
    table.column({
      header: "Name",
      accessor: "name",
      cell: ({ value }) => formatApiKeyName(value),
    }),
    table.column({
      header: "API Key",
      accessor: "secret",
    }),
  ]);

  // Add this near other state declarations
  let editingRow: any = null;

  // Update handleEdit function
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
          s.name === secret.name ? { ...s, secret: secret.secret } : s,
        ),
      );

      // Reset state
      dialogOpen = false;
      editingRow = null;
      editedValue = "";
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
      console.log("API keys:", data);
      secretsData.set(data);
    } catch (err) {
      console.error("Failed to fetch secrets:", err);
    }
  }

  function formatApiKeyName(name: string): string {
    return name
      .replace("_API_KEY", "") // Remove '_api_key' from the string
      .split("_")
      .map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase()) // Make rest of letters lowercase
      .join(" ");
  }

  function toggleSecretVisibility(name: string) {
    visibleSecrets.update((set) => {
      const newSet = new Set(set);
      if (newSet.has(name)) {
        newSet.delete(name);
      } else {
        newSet.add(name);
      }
      return newSet;
    });
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
                  <Button
                    variant="ghost"
                    class="justify-start w-full"
                    onclick={props.sort.toggle}
                  >
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
                <Table.Cell {...attrs} class="text-left">
                  {#if cell.id === "secret"}
                    <div class="flex items-left gap-2">
                      <span class="font-mono">
                        {$visibleSecrets.has(row.cells[0].value)
                          ? cell.value
                          : "••••••••••••••••"}
                      </span>
                      <Button
                        variant="ghost"
                        size="icon"
                        class="h-8 w-8"
                        onclick={() =>
                          toggleSecretVisibility(row.cells[0].value)}
                      >
                        {#if $visibleSecrets.has(row.cells[0].value)}
                          <EyeOff class="h-4 w-4" />
                        {:else}
                          <Eye class="h-4 w-4" />
                        {/if}
                        <span class="sr-only">
                          {$visibleSecrets.has(row.cells[0].value)
                            ? "Hide"
                            : "Show"} API key
                        </span>
                      </Button>
                    </div>
                  {:else}
                    <Render of={cell.render()} />
                  {/if}
                </Table.Cell>
              </Subscribe>
            {/each}
            <!-- Button in each row -->
            <Table.Cell>
              <Dialog.Root
                bind:open={dialogOpen}
                onOpenChange={(open) => {
                  if (open) {
                    editingRow = row;
                    editedValue = row.cells[1].value;
                  } else {
                    editingRow = null;
                    editedValue = "";
                  }
                }}
              >
                <Dialog.Trigger>
                  <Button variant="outline">
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
                        {editingRow?.cells[0].value ?? ""}
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
                    <Button
                      variant="outline"
                      onclick={() => (dialogOpen = false)}
                    >
                      Cancel
                    </Button>
                    <Button onclick={() => handleEdit(editingRow)}>
                      Save changes
                    </Button>
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
