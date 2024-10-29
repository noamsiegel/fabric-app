<script lang="ts">
  import { Label } from "$lib/components/ui/label";
  import { Slider } from "$lib/components/ui/slider";
  import { invoke } from "@tauri-apps/api/core";
  import {
    Card,
    CardContent,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";

  export let temperature = 0.5;
  export let presencePenalty = 0.0;

  async function setTemperature(value: number) {
    await invoke("set_temperature", { value });
  }

  async function setPresencePenalty(value: number) {
    await invoke("set_presence_penalty", { value });
  }
</script>

<Card>
  <CardHeader>
    <CardTitle>Model Parameters</CardTitle>
  </CardHeader>
  <CardContent class="space-y-4">
    <div class="flex flex-col space-y-2">
      <Label for="temperature">Temperature: {temperature.toFixed(2)}</Label>
      <Slider
        id="temperature"
        min={0}
        max={1}
        step={0.1}
        value={[temperature]}
        onValueChange={(values) => {
          temperature = values[0];
          setTemperature(temperature);
        }}
      />
    </div>

    <div class="flex flex-col space-y-2">
      <Label for="presence-penalty"
        >Presence Penalty: {presencePenalty.toFixed(2)}</Label
      >
      <Slider
        id="presence-penalty"
        min={0}
        max={1}
        step={0.1}
        value={[presencePenalty]}
        onValueChange={(values) => {
          presencePenalty = values[0];
          setPresencePenalty(presencePenalty);
        }}
      />
    </div>
  </CardContent>
</Card>
