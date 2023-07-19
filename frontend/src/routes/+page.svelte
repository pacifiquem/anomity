<script lang="ts">
  import { page } from "$app/stores";
</script>

{#if $page.data.user}
  <h1>Welcome</h1>
{/if}
