<script lang="ts">
  import { user } from "../lib/auth"

  let messages = ["Hello", "World"]
</script>

<svelte:head>
  <title>anomity</title>
</svelte:head>

{#if $user}
  <!--<h1 class="font-medium font-sans">Welcome {$user.username}</h1>-->
  <!--<ul>
    {#each messages as message}
      <li>{message}</li>
    {/each}
  </ul>
  <ul>
    {#each rooms as room}
      <li>
        <a href="/{room.id}">{room.name}</a>
      </li>
    {/each}
  </ul>-->
  <div class="px-8 py-2 font-sans">
    <p class="text-xl">Select a room to start chatting</p>
  </div>
{:else}
  <h2>
    <a href="/login">Login</a>
    <a href="/signup">Signup</a>
  </h2>
{/if}
