<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
  } from "$lib/components/ui/select";
  import { Settings, Home, FileText, PlayCircle } from "lucide-svelte";
  import {
    Collapsible,
    CollapsibleContent,
    CollapsibleTrigger,
  } from "$lib/components/ui/collapsible";
  import { Command } from "@tauri-apps/plugin-shell";

  let selectedPattern = "";
  let variables = [];
  let showAdvancedSettings = false;

  function handlePatternChange(event) {
    selectedPattern = event.detail;
    // TODO: Fetch variables based on selected pattern
    variables = [
      { name: "Variable 1", value: "" },
      { name: "Variable 2", value: "" },
    ];
  }

  async function runPattern() {
    try {
      const result = await Command.create("fabric", [
        "--pattern",
        "summarize",
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
      <img src="/logo.png" alt="Fabric Logo" class="h-8 w-8 mr-2" />
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
        <Select onValueChange={handlePatternChange}>
          <SelectTrigger id="pattern-select">
            <SelectValue placeholder="Choose a pattern" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="pattern1">Pattern 1</SelectItem>
            <SelectItem value="pattern2">Pattern 2</SelectItem>
            <SelectItem value="pattern3">Pattern 3</SelectItem>
          </SelectContent>
        </Select>
      </div>

      {#if selectedPattern}
        <div class="mb-6">
          <h2 class="text-xl font-semibold mb-4">Variables</h2>
          {#each variables as variable}
            <div class="mb-4">
              <Label for={variable.name}>{variable.name}</Label>
              <Input id={variable.name} bind:value={variable.value} />
            </div>
          {/each}
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
