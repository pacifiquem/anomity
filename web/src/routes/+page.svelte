<script lang="ts">
  import { user } from "../lib/auth"
</script>

<svelte:head>
  <title>anomity</title>
</svelte:head>

{#if $user}
  <div class="px-8 py-2 font-sans">
    <p class="text-xl">Select a room to start chatting</p>
  </div>
{:else}
  <h2>
    <a href="/signin">Login</a>
    <a href="/signup">Signup</a>
  </h2>
{/if}
