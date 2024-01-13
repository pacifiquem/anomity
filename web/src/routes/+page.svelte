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
  <section class="flex justify-center items-center flex-col">
    <div class="flex justify-center items-center gap-4 h-screen">
      <a
        href="/signin"
        class="bg-indigo-600 px-4 py-2 text-white rounded-sm border-2 border-indigo-500"
        >SignIn</a
      >
      <a href="/signup">Signup</a>
    </div>
  </section>
{/if}
