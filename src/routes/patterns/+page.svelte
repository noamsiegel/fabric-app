<script lang="ts">
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
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  import PatternsTable from "$lib/components/PatternsTable.svelte";

  let patterns = [
    { id: 1, name: "Small Talk", description: "General conversation starters" },
    {
      id: 2,
      name: "Technical Support",
      description: "Troubleshooting common issues",
    },
    // Add more patterns as needed
  ];

  let gitRepo = "";
  let gitFolder = "";

  // Load git settings on mount
  async function loadGitSettings() {
    try {
      gitRepo = await invoke("get_patterns_git_repo");
      gitFolder = await invoke("get_patterns_git_folder");
    } catch (err) {
      console.error("Failed to load git settings:", err);
    }
  }

  // Save git settings
  async function saveGitSettings() {
    try {
      await invoke("set_patterns_git_repo", { repoUrl: gitRepo });
      await invoke("set_patterns_git_folder", { folderPath: gitFolder });
    } catch (err) {
      console.error("Failed to save git settings:", err);
    }
  }

  onMount(async () => {
    await loadGitSettings();
  });
</script>

<div class="p-4">
  <h1 class="text-2xl font-bold mb-4">Pattern Browser</h1>

  <!-- Git Settings Card -->
  <Card class="mb-6">
    <CardHeader>
      <CardTitle>Git Pattern Settings</CardTitle>
      <CardDescription
        >Configure pattern loading from git repository</CardDescription
      >
    </CardHeader>
    <CardContent class="space-y-4">
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
      <Button variant="outline" on:click={loadGitSettings}>Load Settings</Button
      >
      <Button on:click={saveGitSettings}>Save Settings</Button>
    </CardFooter>
  </Card>

  <PatternsTable />
</div>
