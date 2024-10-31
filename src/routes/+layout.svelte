<script>
  import "../app.css";
  import Settings from "./settings/+page.svelte";
  import { Toaster } from "$lib/components/ui/sonner";
  import Header from "./Header.svelte";
</script>

<!-- TODO make it so that all front end pages have the same width margins -->
<!-- Issue URL: https://github.com/noamsiegel/fabric-app/issues/79 -->
<!-- Issue URL: https://github.com/noamsiegel/fabric-app/issues/43 -->
<Toaster />
<Header />
<Settings />

<main>
  <slot />
</main>

<style>
  main {
    padding: 16px;
  }
</style>
