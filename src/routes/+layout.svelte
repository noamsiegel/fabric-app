<script>
  import "../app.css";
  import BackButton from "../lib/components/BackButton.svelte";
</script>

<!-- <slot></slot>-->

<header>
  <BackButton />
</header>

<main>
  <slot />
</main>

<style>
  header {
    padding: 16px;
    background-color: #f8f8f8;
  }

  main {
    padding: 16px;
  }
</style>
