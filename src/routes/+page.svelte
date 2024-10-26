<script lang="ts">
  // svelte stores
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { writable, get, derived } from "svelte/store";
  import type { Writable } from "svelte/store";

  // svelte components
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Settings, Home, FileText, PlayCircle } from "lucide-svelte";
  import { Textarea } from "$lib/components/ui/textarea";
  import {
    runPattern,
    scrapeUrl,
    searchQuestion,
    scrapeAndRunPattern,
  } from "$lib/fabricCommands";
  import PatternCombobox from "$lib/components/PatternCombobox.svelte";

  function handlePatternSelect(event: CustomEvent<string>) {
    selected.set({ value: event.detail, label: event.detail });
  }

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
      console.log("Patterns:", patterns);
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
  let scrapeAndRunResult: string = "";

  async function handleScrapeAndRunPattern(url: string) {
    try {
      scrapeAndRunResult = await scrapeAndRunPattern(url);
    } catch (error) {
      console.error("Error in scrapeAndRunPattern:", error);
      scrapeAndRunResult =
        "An error occurred while scraping and running the pattern.";
    }
  }

  $: {
    console.log("Patterns updated:", patterns);
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
        <Button on:click={runPattern}>Run Test Command</Button>
      </div>
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
            on:click={() =>
              urlToScrape && handleScrapeAndRunPattern(urlToScrape)}
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

      {#if scrapeAndRunResult}
        <div class="mt-6">
          <Label for="result-textarea">Scrape and Run Pattern Result</Label>
          <Textarea
            id="result-textarea"
            value={scrapeAndRunResult}
            rows={10}
            readonly
          />
        </div>
      {/if}
    </div>
  </main>
</div>
