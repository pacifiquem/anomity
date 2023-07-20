<script lang="ts">
  import toast from "svelte-french-toast";
  import { enhance } from "$app/forms";
  import { goto } from "$app/navigation";
</script>

<svelte:head>
  <title>login</title>
</svelte:head>

<div
  class="flex min-h-full flex-col justify-center py-12 sm:px-6 lg:px-8 h-screen"
>
  <div class="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
    <div class="bg-white px-4 py-8 shadow sm:rounded-lg sm:px-10">
      <h2 class="py-6 text-center text-3xl font-extrabold text-gray-900">
        Sign in to your account
      </h2>
      <form
        class="space-y-6"
        action="?/login"
        method="POST"
        use:enhance={() => {
          return async ({ result }) => {
            if (result.type === "error") toast.error(result.error.message);

            if (result.type === "redirect") goto(result.location);
          };
        }}
      >
        <div>
          <label
            for="email"
            class="block text-sm font-medium leading-6 text-gray-900"
            >Email address</label
          >
          <div class="mt-2">
            <input
              id="email"
              name="email"
              type="email"
              autocomplete="email"
              required
              class="block w-full rounded-md border-0 py-1.5 px-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            />
          </div>
        </div>

        <div>
          <label
            for="password"
            class="block text-sm font-medium leading-6 text-gray-900"
            >Password</label
          >
          <div class="mt-2">
            <input
              id="password"
              name="password"
              type="password"
              autocomplete="current-password"
              required
              class="block w-full rounded-md border-0 py-1.5 px-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            />
          </div>
        </div>

        <div>
          <button
            type="submit"
            class="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
            >Sign in</button
          >
        </div>
      </form>
    </div>
  </div>
</div>
