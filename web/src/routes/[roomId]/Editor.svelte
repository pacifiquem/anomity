<script lang="ts">
  //  import { user } from "../lib/auth"
  //  let messages = ["Hello", "World"]
  //  let _rooms = [
  //    {
  //      id: 1,
  //      name: "Room 1",
  //      messages: ["Hello", "World"],
  //    },
  //    {
  //      id: 2,
  //      name: "Room 2",
  //      messages: ["Hello", "World"],
  //    },
  //    {
  //      id: 3,
  //      name: "Room 3",
  //      messages: ["Hello", "World"],
  //    },
  //  ]
</script>

<form
  class="py-4 w-full"
  on:submit={(e) => {
    e.preventDefault()
  }}
>
  <div class="flex gap-3 w-full">
    <input
      type="text"
      class="border-gray border rounded-md px-2 w-full"
      placeholder="type message here..."
    />
    <input
      type="submit"
      value="send"
      class="bg-blue-500 py-2 px-4 rounded-md cursor-pointer w-auto text-white"
    />
  </div>
</form>
