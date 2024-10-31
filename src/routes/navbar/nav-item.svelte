<script lang="ts">
  import type { ComponentType } from "svelte";

  export let href: string;
  export let icon: ComponentType;
  export let active: boolean = false;
</script>

<div class="relative w-10 h-10 flex items-center justify-center">
  <a
    {href}
    class="text-gray-600 hover:text-gray-900 p-2 rounded-lg transition-colors absolute inset-0 flex items-center justify-center"
    class:bg-gray-900={active}
  >
    <svelte:component
      this={icon}
      size={24}
      class={active ? "text-white" : ""}
    />
  </a>
</div>
