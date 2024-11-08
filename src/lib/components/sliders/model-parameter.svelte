<script lang="ts">
  import { Label } from "$lib/components/ui/label";
  import { Slider } from "$lib/components/ui/slider";
  import { createEventDispatcher } from "svelte";

  export let id: string;
  export let label: string;
  export let value: number;
  export let min: number = 0;
  export let max: number = 1;
  export let step: number = 0.1;

  const dispatch = createEventDispatcher();

  function handleValueChange(values: number[]) {
    dispatch("valueChange", values[0]);
  }
</script>

<div class="flex flex-col space-y-2">
  <Label for={id}>{label}: {value.toFixed(2)}</Label>
  <Slider
    {id}
    {min}
    {max}
    {step}
    value={[value]}
    onValueChange={handleValueChange}
  />
</div>
