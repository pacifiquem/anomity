<script lang="ts">
  import "@unocss/reset/tailwind.css"
  import "uno.css"
  import { Toaster } from "svelte-french-toast"

  import { user } from "../lib/auth"

  export let data

  let rooms = [
    {
      id: 1,
      name: "Room 1",
      messages: ["Hello", "World"],
    },
    {
      id: 2,
      name: "Room 2",
      messages: ["Hello", "World"],
    },
    {
      id: 3,
      name: "Room 3",
      messages: ["Hello", "World"],
    },
  ]

  $: user.set(data.user)
  let roomFilterValue = ""
  let filteredRooms: typeof rooms = []

  function handleSearch() {
    console.log(roomFilterValue)
    return (filteredRooms = rooms.filter((room) =>
      room.name.match(roomFilterValue.toLowerCase())
    ))
  }
</script>

{#if data.user}
  <section class="flex min-h-screen">
    <div class="flex-1 bg-gray-200 px-2 py-4">
      <div>
        <input
          type="text"
          placeholder="Search"
          class="w-full p-2 border border-gray-300 rounded-md"
          bind:value={roomFilterValue}
          on:change={handleSearch}
        />
      </div>
      <ul class="py-4 flex flex-col gap-2">
        <!--{#each rooms as room}
          <li>
            <a class="py-2 bg-gray-300 px-2 rounded-sm block" href="/{room.id}"
              >{room.name}</a
            >
          </li>
        {/each}-->

        {#if filteredRooms.length > 0}
          {#each filteredRooms as room}
            <li>
              <a
                class="py-2 bg-gray-300 px-2 rounded-sm block"
                href="/{room.id}">{room.name}</a
              >
            </li>
          {/each}
        {:else}
          {#each rooms as room}
            <li>
              <a
                class="py-2 bg-gray-300 px-2 rounded-sm block"
                href="/{room.id}">{room.name}</a
              >
            </li>
          {/each}
        {/if}
      </ul>
    </div>
    <div class="w-300">
      <slot />
    </div>
  </section>
{:else}
  <slot />
{/if}
<Toaster />
