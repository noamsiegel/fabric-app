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

  export let temperature = 0.5;
  export let presencePenalty = 0.0;
  export let topP = 0.9;
  export let frequencyPenalty = 0.0;

  async function loadModelParameters() {
    try {
      temperature = await invoke("get_temperature");
      presencePenalty = await invoke("get_presence_penalty");
      topP = await invoke("get_top_p");
      frequencyPenalty = await invoke("get_frequency_penalty");
    } catch (err) {
      console.error("Failed to load model parameters:", err);
    }
  }

  async function saveModelParameters() {
    try {
      await invoke("set_temperature", { value: temperature });
      await invoke("set_presence_penalty", { value: presencePenalty });
      await invoke("set_top_p", { value: topP });
      await invoke("set_frequency_penalty", { value: frequencyPenalty });
    } catch (err) {
      console.error("Failed to save model parameters:", err);
    }
  }

  onMount(async () => {
    await loadModelParameters();
  });
</script>

<Card>
  <CardHeader class="flex justify-between w-full">
    <CardTitle class="flex-1 text-left">Model Parameters</CardTitle>
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
  <CardFooter class="flex justify-end">
    <Button on:click={saveModelParameters}>Save Settings</Button>
  </CardFooter>
</Card>
