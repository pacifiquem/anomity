<script lang="ts">
  import type { Room } from "../types/room"
  import { enhance } from "$app/forms"

  export let room: Room
  export let is_active: boolean
</script>

<li
  class={`py-2 bg-gray-300 px-2 rounded-sm flex justify-between ${
    is_active ? "bg-gray-500 text-gray-100" : ""
  }`}
>
  <a href="/{room.id}">{room.name}</a>
  <form
    method="POST"
    action="/?/deleteRoom"
    class="text-red-900 font-bold"
    use:enhance
  >
    <input type="hidden" name="roomId" value={room.id} />
    <button type="submit">x</button>
  </form>
</li>
