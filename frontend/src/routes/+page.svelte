<script lang="ts">
  import { user } from "../lib/auth";
</script>

{#if $user}
  <h1>Welcome {$user.username}</h1>
{/if}
