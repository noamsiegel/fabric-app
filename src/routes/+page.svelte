<script lang="ts">
  // svelte stores
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { writable, type Writable } from "svelte/store";

  // svelte components
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Textarea } from "$lib/components/ui/textarea";

  // custom svelte components
  import PatternCombobox from "$lib/components/PatternCombobox.svelte";

  // fabric commands
  import {
    scrapeUrlAndRunPattern,
    scrapeQuestionAndRunPattern,
  } from "$lib/fabricCommands";
  import { clipboard_contents_and_run_pattern } from "$lib/tauriCommands";

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
  <!-- <SettingsDrawer /> -->

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
            on:click={async () => {
              if (!$selected.value) {
                alert("Please select a pattern first");
                return;
              }
              try {
                result = scrapeUrlAndRunPattern(urlToScrape);
                console.log("Started scraping URL:", urlToScrape);
              } catch (error) {
                console.error("Error in scrape_url_and_run_pattern:", error);
              }
            }}
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
            on:click={() => (result = clipboard_contents_and_run_pattern())}
            disabled={isRunning}
          >
            Run Pattern on Clipboard
          </Button>
        </div>
      </div>

      {#if result}
        {#await result}
          <p>Loading...</p>
        {:then value}
          <div class="mt-4">
            <h3>Result:</h3>
            <Textarea {value} readonly rows={30} class="w-full" />
          </div>
        {:catch error}
          <p>Error: {error.message}</p>
          <pre>{JSON.stringify(error, null, 2)}</pre>
        {/await}
      {/if}
    </div>
  </main>
</div>
