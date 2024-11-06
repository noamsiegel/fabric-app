<script lang="ts">
  import {
    Settings,
    FileText,
    Database,
    Home,
    Layers,
    MessageCircle,
  } from "lucide-svelte";
  import { page } from "$app/stores";
  import NavbarItem from "../../routes/navbar/nav-item.svelte";
  import FabricLogoPng from "$lib/static/icons/fabric-logo-png.png";

  const navItems = [
    { href: "/", icon: Home, label: "Home" },
    { href: "/patterns", icon: FileText, label: "Patterns" },
    { href: "/models", icon: Database, label: "Models" },
    { href: "/contexts", icon: Layers, label: "Contexts" },
    { href: "/chat", icon: MessageCircle, label: "Chat" },
  ];
</script>

<aside class="w-64 h-screen fixed left-0 border-r">
  <!-- Logo section -->
  <div class="p-4 border-b">
    <a
      href="https://github.com/danielmiessler/fabric"
      class="flex items-center"
    >
      <img src={FabricLogoPng} alt="Fabric Logo" class="h-8 w-8 mr-2" />
    </a>
  </div>

  <!-- Navigation section -->
  <nav class="flex flex-col p-4 space-y-2">
    {#each navItems as { href, icon, label }}
      <NavbarItem {href} {icon} active={$page.url.pathname === href}>
        <span class="ml-3">{label}</span>
      </NavbarItem>
    {/each}
  </nav>

  <!-- Settings button at bottom -->
  <div class="absolute bottom-0 w-full p-4 border-t">
    <button
      class="flex items-center w-full px-3 py-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-md"
    >
      <Settings size={20} />
      <span class="ml-3">Settings</span>
    </button>
  </div>
</aside>
