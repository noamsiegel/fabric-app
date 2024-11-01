<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { invoke } from "@tauri-apps/api/core";
  import {
    Card,
    CardContent,
    CardHeader,
    CardTitle,
    CardFooter,
  } from "$lib/components/ui/card";
  import { onMount } from "svelte";
  import SliderComponent from "./ParameterSlider.svelte";
  import { Loader2 } from "lucide-svelte";

  export let temperature = 0.5;
  export let presencePenalty = 0.0;
  export let topP = 0.9;
  export let frequencyPenalty = 0.0;
  let isLoading = false;
  let loadTimes = {
    initial: 0,
    save: 0,
  };

  async function loadModelParameters() {
    isLoading = true;
    const startTime = performance.now();

    try {
      temperature = await invoke("get_temperature");
      presencePenalty = await invoke("get_presence_penalty");
      topP = await invoke("get_top_p");
      frequencyPenalty = await invoke("get_frequency_penalty");
    } catch (err) {
      console.error("Failed to load model parameters:", err);
    } finally {
      loadTimes.initial = performance.now() - startTime;
      isLoading = false;
    }
  }

  async function saveModelParameters() {
    isLoading = true;
    const startTime = performance.now();

    try {
      await invoke("set_temperature", { value: temperature });
      await invoke("set_presence_penalty", { value: presencePenalty });
      await invoke("set_top_p", { value: topP });
      await invoke("set_frequency_penalty", { value: frequencyPenalty });
    } catch (err) {
      console.error("Failed to save model parameters:", err);
    } finally {
      loadTimes.save = performance.now() - startTime;
      isLoading = false;
    }
  }

  onMount(async () => {
    await loadModelParameters();
  });
</script>

<Card>
  <CardHeader class="flex justify-between w-full">
    <CardTitle class="flex-1 text-left">
      Model Parameters
      {#if loadTimes.initial > 0}
        <span class="text-xs text-muted-foreground ml-2">
          Loaded in {loadTimes.initial.toFixed(2)}ms
        </span>
      {/if}
    </CardTitle>
  </CardHeader>
  <CardContent class="space-y-8">
    <SliderComponent
      id="temperature"
      label="Temperature"
      bind:value={temperature}
      on:valueChange={async (e) => {
        temperature = e.detail;
        await saveModelParameters();
      }}
    />
    <SliderComponent
      id="presence-penalty"
      label="Presence Penalty"
      bind:value={presencePenalty}
      on:valueChange={async (e) => {
        presencePenalty = e.detail;
        await saveModelParameters();
      }}
    />
    <SliderComponent
      id="top-p"
      label="Top P"
      bind:value={topP}
      on:valueChange={async (e) => {
        topP = e.detail;
        await saveModelParameters();
      }}
    />
    <SliderComponent
      id="frequency-penalty"
      label="Frequency Penalty"
      bind:value={frequencyPenalty}
      on:valueChange={async (e) => {
        frequencyPenalty = e.detail;
        await saveModelParameters();
      }}
    />
  </CardContent>
  <CardFooter class="flex justify-end gap-2">
    <span class="text-xs text-muted-foreground">
      {#if loadTimes.save > 0}
        Last save took {loadTimes.save.toFixed(2)}ms
      {/if}
    </span>
    <Button on:click={saveModelParameters} disabled={isLoading}>
      {#if isLoading}
        <Loader2 class="mr-2 h-4 w-4 animate-spin" />
        Saving...
      {:else}
        Save Settings
      {/if}
    </Button>
  </CardFooter>
</Card>
