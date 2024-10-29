<script lang="ts">
  // svelte components
  import { Switch } from "$lib/components/ui/switch";
  import { Label } from "$lib/components/ui/label";
  import { Slider } from "$lib/components/ui/slider";
  import * as Tabs from "$lib/components/ui/tabs/index.js";

  // tables
  import ModelsTable from "$lib/components/ModelsTable.svelte";
  import PatternsTable from "$lib/components/PatternsTable.svelte";
  import SecretsTable from "$lib/components/ApiKeysTable.svelte";
  import BaseUrlTable from "$lib/components/BaseUrlTable.svelte";

  // tauri plugins
  import { invoke } from "@tauri-apps/api/core";

  let darkMode = false;
  let notifications = true;
  let temperature = 0.5;
  let presencePenalty = 0.0;

  async function setTemperature(value: number) {
    await invoke("set_temperature", { value });
  }

  async function setPresencePenalty(value: number) {
    await invoke("set_presence_penalty", { value });
  }
</script>

<div class="p-4">
  <h1 class="text-2xl font-bold mb-4">Settings</h1>

  <Tabs.Root value="general">
    <Tabs.List>
      <Tabs.Trigger value="general">General</Tabs.Trigger>
      <Tabs.Trigger value="models">Models</Tabs.Trigger>
      <Tabs.Trigger value="patterns">Patterns</Tabs.Trigger>
      <Tabs.Trigger value="secrets">API Keys</Tabs.Trigger>
      <Tabs.Trigger value="base-urls">Base URLs</Tabs.Trigger>
    </Tabs.List>
    <Tabs.Content value="general">
      <div class="space-y-4 mt-4">
        <div class="flex items-center space-x-2">
          <Switch id="dark-mode" bind:checked={darkMode} />
          <Label for="dark-mode">Dark Mode</Label>
        </div>

        <div class="flex items-center space-x-2">
          <Switch id="notifications" bind:checked={notifications} />
          <Label for="notifications">Enable Notifications</Label>
        </div>

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
      </div>
    </Tabs.Content>
    <Tabs.Content value="models">
      <div class="mt-4">
        <ModelsTable />
      </div>
    </Tabs.Content>
    <Tabs.Content value="patterns">
      <div class="mt-4">
        <PatternsTable />
      </div>
    </Tabs.Content>
    <Tabs.Content value="secrets">
      <div class="mt-4">
        <SecretsTable />
      </div>
    </Tabs.Content>
    <Tabs.Content value="base-urls">
      <div class="mt-4">
        <BaseUrlTable />
      </div>
    </Tabs.Content>
  </Tabs.Root>
</div>
