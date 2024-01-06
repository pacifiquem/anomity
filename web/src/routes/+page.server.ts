import { fail, redirect } from "@sveltejs/kit"
import { env } from "$env/dynamic/private"
import { BACKEND_BASE_URL } from "../utils/constants.js"

/** @type {import('./$types').Actions} */
export const actions = {
	new_room: async ({request, cookies}) => {
		const data = await request.formData()
		const name = data.get("name")
		const description = data.get("description")
		const sessionId = cookies.get("sessionId")

		if(!name || !description)  
			return fail(400, { message: "All fields are required"})

			console.log(env.API_BASE_URL)

		const response = await fetch(`${BACKEND_BASE_URL}/rooms`, {
			body: JSON.stringify({name, description}),
			method: "POST",
			headers: {
				"Content-Type": "application/json",
				Authorization: `Bearer ${sessionId}`
			}
		})

		if(!response.ok)
			return fail(400, {message: "Invalid session"})

		const response_data = await response.json();

		redirect(303,`/rooms/${response_data.id}`)
	},
}