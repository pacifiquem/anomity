<script lang="ts">
  import type { PageData } from "./$types"
  import { browser } from "$app/environment"
  import { page } from "$app/stores"
  import { io } from "socket.io-client"

  export let data: PageData
  export let message = ""

  let messages = data.messages

  const socket = io("ws://localhost:8090/api/ws/room")

  if (browser) {
    //let socket = new WebSocket("ws://localhost:8090/api/ws/room")
    //socket.addEventListener("open", (event) => {
    //  socket.send(
    //    JSON.stringify({
    //      roomId: $page.params.roomId,
    //      userId: data.user?.id,
    //    })
    //  )
    //})
    //socket.addEventListener("message", (event) => {
    //  console.log("event", event)
    //  const message = JSON.parse(event.data)
    //  messages = [...messages, message]
    //})
  }

  //  function sendMessage(message: string) {
  //    socket.send(
  //      JSON.stringify({
  //        roomId: $page.params.roomId,
  //        userId: data.user?.id,
  //        message,
  //      })
  //    )
  //  }
</script>

<section class="flex flex-col justify-end px-2 h-screen">
  <div>
    <pre>
		{JSON.stringify(data.messages, null, 2)}
	</pre>
  </div>
  <form
    class="py-4 w-full"
    on:submit={(e) => {
      e.preventDefault()
      console.log({ message })
    }}
  >
    <div class="flex gap-3 w-full">
      <input
        type="text"
        class="border-gray border rounded-md px-2 w-full"
        placeholder="type message here..."
        bind:value={message}
      />
      <button
        type="submit"
        class="bg-blue-500 py-2 px-4 rounded-md cursor-pointer w-auto text-white"
        >submit</button
      >
    </div>
  </form>
</section>
