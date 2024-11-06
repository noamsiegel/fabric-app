<script>
  import "../app.css";
  // import Settings from "./settings/+page.svelte";
  import { Toaster } from "$lib/components/ui/sonner";
  // import Header from "./Header.svelte";
  import Sidebar from "./Sidebar.svelte";
</script>

<Toaster />
<!-- <Header /> -->
<!-- <Settings /> -->

<!-- <main class="container mx-auto px-[3%] pt-6"> -->
<!-- <slot></slot> -->
<!-- </main> -->
<div class="min-h-screen flex flex-col">
  <!-- <Header /> -->
  <div class="flex flex-1">
    <Sidebar />
    <main class="flex-1">
      <slot />
    </main>
  </div>
</div>
