<script lang="ts">
  import { user } from "../lib/auth"

  let messages = ["Hello", "World"]
</script>

{#if $user}
  <h1 class="font-medium font-sans">Welcome {$user.username}</h1>

  <ul>
    {#each messages as message}
      <li>{message}</li>
    {/each}
  </ul>

  <form
    class="h-[20rem] border-blue w-[20rem]"
    on:submit={(e) => {
      //@ts-ignore
      messages = [...messages, e.target[0].value]

      e.preventDefault()
    }}
  >
    <div class="flex gap-3">
      <textarea class="resize-none p-2 border-gray"></textarea>
      <input
        type="submit"
        value="Submit"
        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded border-none cursor-pointer"
      />
    </div>
  </form>
{:else}
  <h2>
    <a href="/login">Login</a>
    <a href="/signup">Signup</a>
  </h2>
{/if}
