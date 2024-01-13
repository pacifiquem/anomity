<script lang="ts">
  import RoomItem from "./RoomItem.svelte"

  import "@unocss/reset/tailwind.css"
  import "uno.css"
  import { Toaster, toast } from "svelte-french-toast"

  import { page } from "$app/stores"
  import { enhance } from "$app/forms"
  import type { Room } from "../types/room"
  import { user } from "$lib/auth"
  import type { ActionData, PageData } from "./$types"

  export let data: PageData

  let roomFilterValue = ""
  let createRoomForm = false
  let filteredRooms: Room[] = []

  function handleSearch() {
    filteredRooms = data.rooms.filter((room: Room) =>
      room.name.toLowerCase().match(roomFilterValue.toLowerCase())
    )
  }

  $: user.set(data.user)

  export let form: ActionData

  if (form?.message) {
    toast.error(form.message)
  }
</script>

<svelte:head>
  <title>chat</title>
</svelte:head>

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
        <form
          class="flex flex-col gap-2 mt-2"
          method="POST"
          use:enhance
          action="/?/newRoom"
        >
          <input
            type="text"
            name="name"
            placeholder="Room name"
            required
            autocomplete="off"
            value={form?.name ?? ""}
            class="p-2 border border-gray-300 rounded-md"
          />
          <input
            type="text"
            name="description"
            required
            autocomplete="off"
            placeholder="Room description"
            class="p-2 border border-gray-300 rounded-md"
            value={form?.description ?? ""}
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
          {#each filteredRooms as room (room.id)}
            <RoomItem {room} is_active={room.id == $page.params.roomId} />
          {/each}
        {:else}
          {#each data.rooms as room (room.id)}
            <RoomItem {room} is_active={room.id == $page.params.roomId} />
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

{#if !$user}
  <slot />
{/if}

<Toaster />
