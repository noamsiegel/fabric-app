<script lang="ts">
  import {
    Settings,
    FileText,
    Database,
    Layers,
    MessageCircle,
  } from "lucide-svelte";
  import { page } from "$app/stores";
  import NavbarItem from "$lib/components/nav-item.svelte";
  import { settingsOpen } from "$lib/stores/settings";

  const navItems = [
    { href: "/chat", icon: MessageCircle },
    { href: "/patterns", icon: FileText },
    { href: "/models", icon: Database },
    { href: "/contexts", icon: Layers },
  ];
</script>

<div class="h-full w-[60px] border-r bg-background">
  <div class="space-y-4 py-4">
    <div class="flex flex-col items-center justify-center space-y-4">
      {#each navItems as { href, icon }}
        <NavbarItem {href} {icon} active={$page.url.pathname === href} />
      {/each}
      <button
        class="inline-flex items-center justify-center rounded-md w-10 h-10 hover:bg-accent hover:text-accent-foreground"
        onclick={() => settingsOpen.set(true)}
      >
        <Settings size={20} />
      </button>
    </div>
  </div>
</div>
