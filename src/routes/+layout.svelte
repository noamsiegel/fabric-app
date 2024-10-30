<script>
  import "../app.css";
  import BackButton from "../lib/components/BackButton.svelte";
  import Header from "./Header.svelte";
  import { Toaster } from "$lib/components/ui/sonner";
  import SettingsDrawer from "./settings/+page.svelte";
  import { settingsOpen } from "$lib/stores/settings";
</script>

<Toaster />

<div class="flex flex-col h-screen">
  <Header />
  <main class="flex-grow">
    <slot />
  </main>
</div>

<SettingsDrawer />

<main>
  <slot />
</main>

<style>
  header {
    padding: 16px;
    background-color: #f8f8f8;
  }

  main {
    padding: 16px;
  }
</style>
