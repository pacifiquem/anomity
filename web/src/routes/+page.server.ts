import { fail, redirect, type Actions } from "@sveltejs/kit"
import { BACKEND_BASE_URL } from "../utils/constants.js"

export const actions = {
	newRoom: async ({request, cookies}) => {
		const data = await request.formData()
		const name = data.get("name")
		const description = data.get("description")
		const sessionId = cookies.get("sessionId")

		if(!name || !description)  {
			return fail(400, {name, description, message: "All fields are required"})
		}

		const response = await fetch(`${BACKEND_BASE_URL}/rooms`, {
			body: JSON.stringify({name, description}),
			method: "POST",
			headers: {
				"Content-Type": "application/json",
				Authorization: `Bearer ${sessionId}`
			}
		})

		if(!response.ok)
			return fail(400, {name, description, message: "Invalid session"})

		const response_data = await response.json();

		redirect(303,`/${response_data.id}`)
	},

	deleteRoom: async ({request, cookies}) => {
		const formData = await request.formData()
		const roomId = formData.get("roomId")
		const sessionId = cookies.get("sessionId")

		if(!roomId) {
			return fail(400, {roomId, message: "All fields are required"})
		}

		const response = await fetch(`${BACKEND_BASE_URL}/rooms/${roomId}`, {
			method: "DELETE",
			headers: {
				"Content-Type": "application/json",
				Authorization: `Bearer ${sessionId}`
			}
		})

		if(!response.ok)
			return fail(400, {roomId, message: "Invalid session"})

		redirect(303, `/`)
	},
} satisfies Actions