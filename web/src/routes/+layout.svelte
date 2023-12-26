<script>
  import "uno.css"
  import "@unocss/reset/tailwind.css"
  import { Toaster } from "svelte-french-toast"

  import { user } from "../lib/auth"

  export let data

  $: user.set(data.user)
</script>

<slot />
<Toaster />
