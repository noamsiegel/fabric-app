<script lang="ts">
  // Svelte
  import { onMount } from "svelte";

  // Svelte components
  import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { defaultPatternStore } from "$lib/stores/pattern";
  import { toast } from "svelte-sonner";

  // Tauri
  import { invoke } from "@tauri-apps/api/core";

  let gitRepo = "";
  let gitFolder = "";
  let defaultPattern = "";
  let isUpdating = false;

  defaultPatternStore.subscribe(async (newPattern) => {
    if (newPattern) {
      defaultPattern = newPattern;
    }
  });

  function handleInputClick() {
    toast.info("Set Default Pattern", {
      description:
        "To change the default pattern, click the star icon (⭐︎) next to a pattern in the table below.",
      duration: 4000,
    });
  }

  async function loadSettings() {
    try {
      gitRepo = await invoke("get_patterns_git_repo");
      gitFolder = await invoke("get_patterns_git_folder");
      const pattern = (await invoke("get_default_pattern")) as string;
      defaultPattern = pattern;
      defaultPattern = await invoke("get_default_pattern");
      defaultPatternStore.set(pattern); // Initialize the store
    } catch (err) {
      console.error("Failed to load git settings:", err);
    }
  }

  async function saveGitSettings() {
    try {
      await invoke("set_patterns_git_repo", { repoUrl: gitRepo });
      await invoke("set_patterns_git_folder", { folderPath: gitFolder });
    } catch (err) {
      console.error("Failed to save git settings:", err);
    }
  }

  async function handleUpdatePatterns() {
    if (isUpdating) return;

    isUpdating = true;
    try {
      console.log("Updating patterns...");
      const result = await invoke("update_patterns");
      console.log("Patterns updated:", result);
    } catch (error) {
      console.error("Failed to update patterns:", error);
    } finally {
      isUpdating = false;
    }
  }

  onMount(async () => {
    await loadSettings();
  });
</script>

<Card class="mb-6">
  <CardHeader class="flex flex-row items-center justify-between">
    <div>
      <CardTitle>Git Pattern Settings</CardTitle>
      <CardDescription
        >Configure pattern loading from git repository</CardDescription
      >
    </div>
    <Button
      variant="outline"
      on:click={handleUpdatePatterns}
      disabled={isUpdating}
    >
      {isUpdating ? "Updating..." : "Update Patterns"}
    </Button>
  </CardHeader>
  <CardContent class="space-y-4">
    <div class="space-y-2">
      <label for="defaultPattern">Default Pattern</label>
      <Input
        id="defaultPattern"
        value={defaultPattern}
        readonly
        disabled={false}
        class="bg-white font-medium text-black cursor-help"
        on:click={handleInputClick}
      />
    </div>
    <div class="space-y-2">
      <label for="gitRepo">Git Repository URL</label>
      <Input
        id="gitRepo"
        bind:value={gitRepo}
        placeholder="https://github.com/user/repo"
      />
    </div>
    <div class="space-y-2">
      <label for="gitFolder">Patterns Folder Path</label>
      <Input id="gitFolder" bind:value={gitFolder} placeholder="patterns/" />
    </div>
  </CardContent>
  <CardFooter class="flex gap-2">
    <Button on:click={saveGitSettings}>Save Settings</Button>
  </CardFooter>
</Card>
