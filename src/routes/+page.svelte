<script lang="ts">
  // svelte stores
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { writable } from "svelte/store";
  import type { Writable } from "svelte/store";

  // svelte components
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Settings, Home, FileText, PlayCircle } from "lucide-svelte";
  import { Textarea } from "$lib/components/ui/textarea";

  // custom svelte components
  import PatternCombobox from "$lib/components/PatternCombobox.svelte";

  // fabric commands
  import {
    scrapeUrlAndRunPattern,
    scrapeQuestionAndRunPattern,
    clipboardContentsAndRunPattern,
  } from "$lib/fabricCommands";

  // state
  let isRunning = false;
  let urlToScrape = "";
  let questionToSearch = "";
  let patterns: string[] = [];
  let result: Promise<string> | null = null;
  let selected: Writable<{ value: string; label: string }> = writable({
    value: "",
    label: "",
  });

  function handlePatternSelect(event: CustomEvent<string>) {
    selected.set({ value: event.detail, label: event.detail });
    console.log("Selected pattern:", event.detail);
  }

  onMount(async () => {
    try {
      patterns = await invoke("get_patterns");
      const savedPattern = await invoke("get_selected_pattern");
      if (typeof savedPattern === "string" && savedPattern) {
        selected.set({ value: savedPattern, label: savedPattern });
      }
    } catch (error) {
      console.error("Error fetching patterns or selected pattern:", error);
    }
  });

  onMount(() => {
    const checkRunningStatus = async () => {
      isRunning = await invoke("get_is_running");
      setTimeout(checkRunningStatus, 1000); // Check every 1 second
    };
    checkRunningStatus();
  });

  $: {
    if ($selected.value) {
      invoke("set_selected_pattern", { pattern: $selected.value });
      console.log("Selected pattern:", $selected.value);
    }
  }
</script>

<div class="flex flex-col h-screen">
  <header class="flex justify-between items-center p-4 border-b">
    <div class="flex items-center">
      <!-- TODO add fabric logo -->
      <!-- <img src="/logo.png" alt="Fabric Logo" class="h-8 w-8 mr-2" /> -->
      <h1 class="text-2xl font-bold">Fabric</h1>
    </div>
    <nav class="flex space-x-4">
      <a href="/patterns" class="text-gray-600 hover:text-gray-900"
        ><FileText size={24} /></a
      >
      <a href="/settings" class="text-gray-600 hover:text-gray-900"
        ><Settings size={24} /></a
      >
    </nav>
  </header>

  <main class="flex-grow p-6">
    <div class="max-w-3xl mx-auto">
      <div class="mb-6">
        <Label for="pattern-select">Select Pattern</Label>
        <!-- TODO make this  auto refresh the drop down items -->
        <PatternCombobox
          {patterns}
          selectedPattern={$selected.value}
          on:select={handlePatternSelect}
        />
      </div>
      <div class="mb-6">
        <Label for="url-input">Enter URL to Scrape</Label>
        <div class="flex space-x-2">
          <Input
            id="url-input"
            type="text"
            bind:value={urlToScrape}
            placeholder="https://example.com"
          />
          <Button
            on:click={() => (result = scrapeUrlAndRunPattern(urlToScrape))}
            disabled={isRunning}
          >
            Scrape and Run Pattern
          </Button>
        </div>
      </div>

      <div class="mb-6">
        <Label for="question-input">Enter Question to Search</Label>
        <div class="flex space-x-2">
          <Input
            id="question-input"
            type="text"
            bind:value={questionToSearch}
            placeholder="What is the capital of France?"
          />
          <Button
            on:click={() =>
              (result = scrapeQuestionAndRunPattern(questionToSearch))}
            disabled={isRunning}
          >
            Search and Run Pattern
          </Button>
        </div>
      </div>

      <div class="mb-6">
        <Label for="clipboard-button">Run Pattern on Clipboard Contents</Label>
        <div class="flex space-x-2">
          <Button
            on:click={() => (result = clipboardContentsAndRunPattern())}
            disabled={isRunning}
          >
            Run Pattern on Clipboard
          </Button>
        </div>
      </div>

      {#if result}
        {#await result}
          <p>Loading...</p>
        {:then result}
          <div class="mt-4">
            <h3>Result:</h3>
            <Textarea value={result} readonly rows={30} class="w-full" />
          </div>
        {:catch error}
          <p>Error: {error.message}</p>
        {/await}
      {/if}
    </div>
  </main>
</div>
