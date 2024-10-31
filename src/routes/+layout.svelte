<script>
  import "../app.css";
  import Settings from "./settings/+page.svelte";
  import { Toaster } from "$lib/components/ui/sonner";
  import Header from "./Header.svelte";
</script>

<!-- Issue URL: https://github.com/noamsiegel/fabric-app/issues/80 -->
<!-- Issue URL: https://github.com/noamsiegel/fabric-app/issues/79 -->
<!-- Issue URL: https://github.com/noamsiegel/fabric-app/issues/43 -->
<Toaster />
<Header />
<Settings />

<main class="container mx-auto px-[3%] pt-6">
  <slot />
</main>
