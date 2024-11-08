<script>
  import { goto } from "$app/navigation";

  function goBack() {
    history.back();
  }
</script>

<button onclick={goBack}> Back </button>

<style>
  button {
    padding: 8px 16px;
    background-color: #f0f0f0;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
  }

  button:hover {
    background-color: #e0e0e0;
  }
</style>
