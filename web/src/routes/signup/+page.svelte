<script>
  import { enhance } from "$app/forms"
  import { goto } from "$app/navigation"
  import toast from "svelte-french-toast"
  import { user } from "$lib/auth"

  if ($user?.id) goto("/")
</script>

<svelte:head>
  <title>signup</title>
</svelte:head>

<div
  class="flex min-h-full flex-col justify-center py-12 sm:px-6 lg:px-8 h-screen"
>
  <div class="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
    <div class="px-4 py-8 shadow sm:rounded-lg sm:px-10">
      <h2 class="py-6 text-center text-xl font-medium text-gray-900">
        Sign up to your account
      </h2>
      <form
        class="space-y-6"
        action="?/signup"
        method="POST"
        use:enhance={() => {
          return async ({ result }) => {
            if (result.type === "error") toast.error(result.error.message)
            if (result.type === "redirect") goto(result.location)
          }
        }}
      >
        <div>
          <label
            for="username"
            class="block text-sm font-medium leading-6 text-gray-900"
            >username</label
          >
          <div class="mt-2">
            <input
              id="username"
              name="username"
              type="text"
              required
              class="block w-full rounded-md border-0 py-1.5 px-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            />
          </div>
        </div>

        <div>
          <label
            for="email"
            class="block text-sm font-medium leading-6 text-gray-900"
            >Email</label
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
            class="flex w-full justify-center rounded-md bg-indigo-500 px-3 py-2 text-sm font-semibold text-white shadow-sm focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
            >Sign up</button
          >
        </div>
        <div class="flex gap-1">
          <p>Already have an account?</p>
          <a href="/signin" class="text-indigo-500">Sign in</a>
        </div>
      </form>
    </div>
  </div>
</div>
