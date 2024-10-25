<script lang="ts">
  import { page } from "$app/stores";
  import { Button } from "$lib/components/ui/button";
</script>

<div class="flex flex-col items-center justify-center h-screen">
  <h1 class="text-4xl font-bold mb-4">Error {$page.status}</h1>
  <p class="text-xl mb-8">{$page.error?.message}</p>
  <Button href="/">Go back home</Button>
</div>
