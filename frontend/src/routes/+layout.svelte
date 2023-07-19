<script>
  import { Toaster } from "svelte-french-toast";
  import "../app.css";

  import { user } from "../lib/auth";

  export let data;

  $: user.set(data.user);
</script>

<slot />
<Toaster />
