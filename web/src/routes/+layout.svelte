<script lang="ts">
  import "@unocss/reset/tailwind.css"
  import "uno.css"
  import { Toaster } from "svelte-french-toast"

  import { page } from "$app/stores"
  import type { Room } from "../types/room"
  import { user } from "$lib/auth"

  export let data

  let roomFilterValue = ""
  let createRoomForm = false
  let filteredRooms: Room[] = []

  function handleSearch() {
    filteredRooms = data.rooms.filter((room: Room) =>
      room.name.toLowerCase().match(roomFilterValue.toLowerCase())
    )
  }

  $: user.set(data.user)
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

      <button
        class="bg-black px-4 w-full py-2 rounded-md text-white mt-2 font-sans"
        class:bg-red-500={createRoomForm}
        on:click={() => (createRoomForm = !createRoomForm)}
        >{createRoomForm ? "Cancel" : "Create room"}</button
      >

      {#if createRoomForm}
        <!--{#if form}<p></p>-->
        <form
          class="flex flex-col gap-2 mt-2"
          method="post"
          action="?/new_room"
        >
          <input
            type="text"
            name="name"
            placeholder="Room name"
            class="p-2 border border-gray-300 rounded-md"
          />
          <input
            type="text"
            name="description"
            placeholder="Room description"
            class="p-2 border border-gray-300 rounded-md"
          />
          <button
            type="submit"
            class="bg-black px-4 w-full py-2 rounded-md text-white mt-2 font-sans"
          >
            Create
          </button>
        </form>
      {/if}

      <ul class="py-4 flex flex-col gap-2">
        {#if roomFilterValue.length}
          {#each filteredRooms as room}
            <li>
              <a
                class={`py-2 bg-gray-300 px-2 rounded-sm block ${
                  room.id == $page.params.roomId
                    ? "bg-gray-500 text-gray-100"
                    : ""
                }`}
                href="/{room.id}">{room.name}</a
              >
            </li>
          {/each}
        {:else}
          {#each data.rooms as room}
            <li>
              <a
                class={`py-2 bg-gray-300 px-2 rounded-sm block ${
                  room.id == +$page.params.roomId
                    ? "bg-gray-500 text-gray-100"
                    : ""
                }`}
                href="/{room.id}">{room.name}</a
              >
            </li>
          {/each}
        {/if}

        {#if (roomFilterValue.length > 0 && filteredRooms.length == 0) || data.rooms.length == 0}
          <p class="text-center">No rooms found</p>
        {/if}
      </ul>
    </div>
    <div class="w-300">
      <slot />
    </div>
  </section>
{/if}
<Toaster />
