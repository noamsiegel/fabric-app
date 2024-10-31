<script>
  import "../app.css";
  import Settings from "./settings/+page.svelte";
  import { Toaster } from "$lib/components/ui/sonner";
  import Header from "./Header.svelte";
</script>

<!-- todoist test 4 -->

<!-- TODO todoist test 3 -->
<!-- Issue URL: https://github.com/noamsiegel/fabric-app/issues/60 -->
<!-- Issue URL: https://github.com/noamsiegel/fabric-app/issues/58 -->
<!-- Issue URL: https://github.com/noamsiegel/fabric-app/issues/56 -->



<!-- TODO make it so that all front end pages have the same width margins -->
<!-- Issue URL: https://github.com/noamsiegel/fabric-app/issues/53 -->
<!-- Issue URL: https://github.com/noamsiegel/fabric-app/issues/52 -->
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
