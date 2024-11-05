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

  // Add default constants
  const DEFAULT_TEMPERATURE = 0.7;
  const DEFAULT_PRESENCE_PENALTY = 0.0;
  const DEFAULT_TOP_P = 1.0;
  const DEFAULT_FREQUENCY_PENALTY = 0.0;

  export let temperature = 0.5;
  export let presencePenalty = 0.0;
  export let topP = 0.9;
  export let frequencyPenalty = 0.0;

  let isLoading = false;

  async function loadModelParameters() {
    try {
      temperature =
        parseFloat(await invoke("get_secret", { key: "TEMPERATURE" })) || 0.7;
      presencePenalty =
        parseFloat(await invoke("get_secret", { key: "PRESENCE_PENALTY" })) ||
        0;
      topP = parseFloat(await invoke("get_secret", { key: "TOP_P" })) || 1;
      frequencyPenalty =
        parseFloat(await invoke("get_secret", { key: "FREQUENCY_PENALTY" })) ||
        0;
    } catch (err) {
      console.error("Failed to load model parameters:", err);
    }
  }

  async function saveModelParameters() {
    try {
      await invoke("update_secret", {
        key: "TEMPERATURE",
        value: temperature.toString(),
      });
      await invoke("update_secret", {
        key: "PRESENCE_PENALTY",
        value: presencePenalty.toString(),
      });
      await invoke("update_secret", { key: "TOP_P", value: topP.toString() });
      await invoke("update_secret", {
        key: "FREQUENCY_PENALTY",
        value: frequencyPenalty.toString(),
      });
    } catch (err) {
      console.error("Failed to save model parameters:", err);
    }
  }

  async function resetModelParameters() {
    try {
      temperature = DEFAULT_TEMPERATURE;
      presencePenalty = DEFAULT_PRESENCE_PENALTY;
      topP = DEFAULT_TOP_P;
      frequencyPenalty = DEFAULT_FREQUENCY_PENALTY;

      await saveModelParameters();
    } catch (err) {
      console.error("Failed to reset model parameters:", err);
    }
  }

  onMount(async () => {
    await loadModelParameters();
  });
</script>

<Card>
  <CardHeader class="flex flex-row items-center justify-between">
    <CardTitle>Model Parameters</CardTitle>
    <Button
      variant="outline"
      on:click={resetModelParameters}
      disabled={isLoading}
    >
      {#if isLoading}
        <Loader2 class="mr-2 h-4 w-4 animate-spin" />
        Resetting...
      {:else}
        Reset
      {/if}
    </Button>
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
  <CardFooter class="flex justify-end gap-2"></CardFooter>
</Card>
