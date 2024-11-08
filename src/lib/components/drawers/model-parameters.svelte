<script lang="ts">
  import * as Drawer from "$lib/components/ui/drawer";
  import { Button, buttonVariants } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";

  // Model parameters state
  let temperature = $state(0.7);
  let maxLength = $state(100);
  let topP = $state(0.9);
  let frequencyPenalty = $state(0.0);
  let presencePenalty = $state(0.0);

  // Handle form submission
  function handleSubmit() {
    // Add your submission logic here
    console.log({
      temperature,
      maxLength,
      topP,
      frequencyPenalty,
      presencePenalty,
    });
  }
</script>

<Drawer.Root>
  <Drawer.Trigger class={buttonVariants({ variant: "outline" })}>
    Model Parameters
  </Drawer.Trigger>
  <Drawer.Content>
    <div class="mx-auto w-full max-w-sm">
      <Drawer.Header>
        <Drawer.Title>Model Parameters</Drawer.Title>
        <Drawer.Description>
          Adjust the parameters for the AI model.
        </Drawer.Description>
      </Drawer.Header>

      <div class="p-4">
        <form class="space-y-4" on:submit|preventDefault={handleSubmit}>
          <div class="space-y-2">
            <Label for="temperature">Temperature</Label>
            <Input
              type="number"
              id="temperature"
              bind:value={temperature}
              min="0"
              max="2"
              step="0.1"
            />
            <p class="text-sm text-muted-foreground">
              Controls randomness: 0 is focused, 1 is balanced, 2 is creative
            </p>
          </div>

          <div class="space-y-2">
            <Label for="maxLength">Max Length</Label>
            <Input
              type="number"
              id="maxLength"
              bind:value={maxLength}
              min="1"
              max="4000"
            />
          </div>

          <div class="space-y-2">
            <Label for="topP">Top P</Label>
            <Input
              type="number"
              id="topP"
              bind:value={topP}
              min="0"
              max="1"
              step="0.1"
            />
          </div>

          <div class="space-y-2">
            <Label for="frequencyPenalty">Frequency Penalty</Label>
            <Input
              type="number"
              id="frequencyPenalty"
              bind:value={frequencyPenalty}
              min="-2"
              max="2"
              step="0.1"
            />
          </div>

          <div class="space-y-2">
            <Label for="presencePenalty">Presence Penalty</Label>
            <Input
              type="number"
              id="presencePenalty"
              bind:value={presencePenalty}
              min="-2"
              max="2"
              step="0.1"
            />
          </div>
        </form>
      </div>

      <Drawer.Footer>
        <Button onclick={handleSubmit}>Save Changes</Button>
        <Drawer.Close class={buttonVariants({ variant: "outline" })}>
          Cancel
        </Drawer.Close>
      </Drawer.Footer>
    </div>
  </Drawer.Content>
</Drawer.Root>
