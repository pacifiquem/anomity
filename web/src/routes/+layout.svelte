<script lang="ts">
  import "@unocss/reset/tailwind.css"
  import "uno.css"
  import { Toaster } from "svelte-french-toast"

  import { user } from "../lib/auth"
  import { page } from "$app/stores"
  import { BACKEND_BASE_URL } from "../utils/constants"

  export let data

  console.log({ data })

  let rooms = [
    {
      id: 1,
      name: "Room 1",
    },
    {
      id: 2,
      name: "Room 2",
    },
    {
      id: 3,
      name: "Room 3",
    },
  ]

  $: user.set(data.user)
  let roomFilterValue = ""
  let filteredRooms: typeof rooms = []

  function handleSearch() {
    filteredRooms = rooms.filter((room) =>
      room.name.toLowerCase().match(roomFilterValue.toLowerCase())
    )
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
          on:input={handleSearch}
        />
      </div>
      <ul class="py-4 flex flex-col gap-2">
        {#if filteredRooms.length}
          {#each filteredRooms as room}
            <li>
              <a
                class={`py-2 bg-gray-300 px-2 rounded-sm block ${
                  room.id == +$page.params.roomId ? "bg-gray-500" : ""
                }`}
                href="/{room.id}">{room.name}</a
              >
            </li>
          {/each}
        {:else}
          {#each rooms as room}
            <li>
              <a
                class={`py-2 bg-gray-300 px-2 rounded-sm block ${
                  room.id == +$page.params.roomId ? "bg-gray-500" : ""
                }`}
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
