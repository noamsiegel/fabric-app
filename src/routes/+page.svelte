<script lang="ts">
  // svelte stores
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { writable, get } from "svelte/store";
  import type { Writable } from "svelte/store";

  // svelte components
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import * as Select from "$lib/components/ui/select";
  import { Settings, Home, FileText, PlayCircle } from "lucide-svelte";
  // tauri plugins
  import { Command } from "@tauri-apps/plugin-shell";
  // import { runPattern, scrapeUrl, searchQuestion } from "$lib/fabricCommands";

  // let selectedPattern = "";
  let selected: Writable<{ value: string; label: string }> = writable({
    value: "",
    label: "",
  });

  let urlToScrape = "";
  let questionToSearch = "";
  let patterns: string[] = [];
  let errorMessage: string = "";

  onMount(async () => {
    try {
      patterns = await invoke("get_patterns");
      const savedPattern = await invoke("get_selected_pattern");
      if (typeof savedPattern === "string" && savedPattern) {
        selected.set({ value: savedPattern, label: savedPattern });
        // alert(`Selected pattern: ${savedPattern}`);
      }
    } catch (error) {
      console.error("Error fetching patterns or selected pattern:", error);
      if (error instanceof Error) {
        errorMessage = error.message;
      } else {
        errorMessage = String(error);
      }
    }
  });

  $: {
    if ($selected.value) {
      invoke("set_selected_pattern", { pattern: $selected.value });
      console.log("Selected pattern:", $selected.value);
    }
  }

  async function runPattern() {
    if (!$selected.value) {
      alert("Please select a pattern first.");
      return;
    }
    try {
      console.log("Running pattern:", $selected.value);
      const result = await Command.create("fabric", [
        "--pattern",
        $selected.value,
      ]).execute();
      console.log("Test command result:", result);
      alert(`Command executed successfully. Output: ${result.stdout}`);
    } catch (error) {
      console.error("Error running test command:", error);
      if (error instanceof Error) {
        alert(`Error running command: ${error.message}`);
      } else {
        alert(`An unexpected error occurred: ${String(error)}`);
      }
    }
  }

  async function scrapeUrl() {
    try {
      console.log("Scraping URL:", urlToScrape);
      const result = await Command.create("fabric", [
        "-q",
        urlToScrape,
      ]).execute();
      console.log("Scrape command result:", result);
      // alert(`URL scraped successfully. Output: ${result.stdout}`);
    } catch (error) {
      console.error("Error scraping URL:", error);
      if (error instanceof Error) {
        alert(`Error scraping URL: ${error.message}`);
      } else {
        alert(`An unexpected error occurred: ${String(error)}`);
      }
    }
  }

  async function searchQuestion() {
    try {
      console.log("Searching question:", questionToSearch);
      const result = await Command.create("fabric", [
        "-q",
        questionToSearch,
      ]).execute();
      console.log("Search question command result:", result);
      alert(`Question searched successfully. Output: ${result.stdout}`);
    } catch (error) {
      console.error("Error searching question:", error);
      if (error instanceof Error) {
        alert(`Error searching question: ${error.message}`);
      } else {
        alert(`An unexpected error occurred: ${String(error)}`);
      }
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
      <a href="/" class="text-gray-600 hover:text-gray-900"
        ><Home size={24} /></a
      >
      <a href="/patterns" class="text-gray-600 hover:text-gray-900"
        ><FileText size={24} /></a
      >
      <a href="/sessions" class="text-gray-600 hover:text-gray-900"
        ><PlayCircle size={24} /></a
      >
      <a href="/settings" class="text-gray-600 hover:text-gray-900"
        ><Settings size={24} /></a
      >
    </nav>
  </header>

  <main class="flex-grow p-6">
    <div class="max-w-3xl mx-auto">
      <div class="mb-6">
        <Button on:click={runPattern}>Run Test Command</Button>
      </div>
      <div class="mb-6">
        <Label for="pattern-select">Select Pattern</Label>
        <Select.Root bind:selected={$selected}>
          <Select.Trigger id="pattern-select">
            <Select.Value placeholder="Choose a pattern" />
          </Select.Trigger>
          <Select.Content>
            {#each patterns as pattern}
              <Select.Item value={pattern}>{pattern}</Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
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
          <Button on:click={scrapeUrl}>Scrape URL</Button>
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
          <Button on:click={searchQuestion}>Search Question</Button>
        </div>
      </div>

      {#if $selected}
        <div class="flex space-x-4">
          <Button on:click={runPattern}>Run</Button>
          <Button variant="outline">Dry Run</Button>
          <Button variant="outline">Save Output</Button>
        </div>
      {/if}
    </div>
  </main>
</div>
