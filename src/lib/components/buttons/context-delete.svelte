<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { invoke } from "@tauri-apps/api/core";
  import * as AlertDialog from "$lib/components/ui/alert-dialog";

  export let selectedTitle: string;
  export let content: string;
  export let onDelete: () => void = () => {};
  export let onContextDeleted: () => void;

  async function handleDelete() {
    if (!selectedTitle) return;

    try {
      await invoke("delete_context_file", {
        title: selectedTitle,
      });
      content = "";
      onDelete();
      onContextDeleted();
      console.log("Context deleted successfully");
    } catch (error) {
      console.error("Error deleting context:", error);
    }
  }
</script>

<AlertDialog.Root>
  <AlertDialog.Trigger>
    <Button variant="destructive" size="sm" disabled={!selectedTitle}>
      Delete
    </Button>
  </AlertDialog.Trigger>
  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Title>Delete Context</AlertDialog.Title>
      <AlertDialog.Description>
        Are you sure you want to delete "{selectedTitle}"? This action cannot be
        undone.
      </AlertDialog.Description>
    </AlertDialog.Header>
    <AlertDialog.Footer>
      <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
      <AlertDialog.Action onclick={handleDelete}>Delete</AlertDialog.Action>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
