<script lang="ts">
  // svelte stores
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  // svelte components
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import * as Select from "$lib/components/ui/select";
  import { Settings, Home, FileText, PlayCircle } from "lucide-svelte";
  import {
    Collapsible,
    CollapsibleContent,
    CollapsibleTrigger,
  } from "$lib/components/ui/collapsible";
  // tauri plugins
  import { Command } from "@tauri-apps/plugin-shell";

  // let selectedPattern = "";
  let selectedPattern = { value: "", label: "" };
  let showAdvancedSettings = false;

  let patterns: string[] = [];
  let errorMessage: string = "";

  onMount(async () => {
    try {
      patterns = await invoke("get_patterns");
    } catch (error) {
      console.error("Error fetching patterns:", error);
      if (error instanceof Error) {
        errorMessage = error.message;
      } else {
        errorMessage = String(error);
      }
    }
  });

  onMount(async () => {
    // Fetch patterns based on the fabric folder path
    patterns = await invoke("get_patterns");
  });

  async function runPattern() {
    try {
      console.log("Running pattern:", selectedPattern);
      const result = await Command.create("fabric", [
        "--pattern",
        selectedPattern.value,
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
        <Select.Root bind:selected={selectedPattern}>
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

      {#if selectedPattern}
        <div class="mb-6">
          <h2 class="text-xl font-semibold mb-4">Variables</h2>
          <!-- TODO: Add variables fields here -->
        </div>

        <Collapsible>
          <CollapsibleTrigger class="flex items-center mb-4">
            <Button variant="outline">
              {showAdvancedSettings ? "Hide" : "Show"} Advanced Settings
            </Button>
          </CollapsibleTrigger>
          <CollapsibleContent>
            <div class="mb-6">
              <h2 class="text-xl font-semibold mb-4">Advanced Settings</h2>
              <!-- Add advanced settings fields here -->
            </div>
          </CollapsibleContent>
        </Collapsible>

        <div class="flex space-x-4">
          <Button on:click={runPattern}>Run</Button>
          <Button variant="outline">Dry Run</Button>
          <Button variant="outline">Save Output</Button>
        </div>
      {/if}
    </div>
  </main>
</div>
