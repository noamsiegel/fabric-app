<script lang="ts">
  import { Settings, FileText, Database, Home, Layers } from "lucide-svelte";
  import { settingsOpen } from "$lib/stores/settings";
  import { page } from "$app/stores";
  import NavbarItem from "./navbar/nav-item.svelte";

  const navItems = [
    { href: "/", icon: Home },
    { href: "/patterns", icon: FileText },
    { href: "/models", icon: Database },
    { href: "/contexts", icon: Layers },
  ];
</script>

<!-- TODO make sure that the position of the icons does not shift on page change -->
<!-- Issue URL: https://github.com/noamsiegel/fabric-app/issues/97 -->

<header class="flex justify-between items-center p-4 border-b h-16">
  <!-- Left section with fixed width -->
  <div class="flex items-center w-[100px]">
    <h1 class="text-2xl font-bold">Fabric</h1>
  </div>

  <!-- Center section -->
  <nav class="flex justify-center space-x-8">
    {#each navItems as { href, icon }}
      <NavbarItem {href} {icon} active={$page.url.pathname === href} />
    {/each}
  </nav>

  <!-- Right section with fixed width -->
  <div class="flex justify-end w-[100px]">
    <button
      class="text-gray-600 hover:text-gray-900"
      on:click={() => ($settingsOpen = true)}
    >
      <Settings size={24} />
    </button>
  </div>
</header>
