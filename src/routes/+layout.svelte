<script>
  import "../app.css";
  import Settings from "./settings/+page.svelte";
  import { Toaster } from "$lib/components/ui/sonner";
  import Sidebar from "../lib/components/Sidebar.svelte";
</script>

<Toaster />

<div class="min-h-screen flex flex-col">
  <div class="flex flex-1">
    <Sidebar />
    <main class="flex-1">
      <slot />
    </main>
  </div>
</div>

<Settings />
