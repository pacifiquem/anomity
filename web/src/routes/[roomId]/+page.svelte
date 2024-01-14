<script lang="ts">
  import Editor from "./Editor.svelte"
  import type { PageData } from "./$types"
  import { browser } from "$app/environment"
  import { page } from "$app/stores"

  export let data: PageData

  let messages = data.messages

  if (browser) {
    var socket = new WebSocket("ws://localhost:8090/api/ws/room")

    socket.addEventListener("open", (event) => {
      socket.send(
        JSON.stringify({
          roomId: $page.params.roomId,
          userId: data.user?.id,
        })
      )
    })

    socket.addEventListener("message", (event) => {
      console.log("event", event)
      const message = JSON.parse(event.data)

      messages = [...messages, message]
    })
  }
</script>

<section class="flex flex-col justify-end px-2 h-screen">
  <div>
    <pre>
		{JSON.stringify(data.messages, null, 2)}
	</pre>
  </div>
  <Editor />
</section>
