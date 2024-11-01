<script>
  import "../app.css";
  import Settings from "./settings/+page.svelte";
  import { Toaster } from "$lib/components/ui/sonner";
  import Header from "./Header.svelte";
</script>

<Toaster />
<Header />
<Settings />

<main class="container mx-auto px-[3%] pt-6">
  <slot />
</main>
