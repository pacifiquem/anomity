import type { Actions } from "./$types";
import { BACKEND_BASE_URL } from "../../utils/constants";
import { redirect } from "@sveltejs/kit";

/** @type {import('./$types').Actions} */
export const actions = {
	signup: async ({ request }) => {
		const formData = await request.formData()
		const data = Object.fromEntries(formData.entries())

		const signup_response = await fetch(`${BACKEND_BASE_URL}/users`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify(data),
		})

		if (!signup_response.ok)
			return {
				signup_error: (await signup_response.json()).message,
			}
		throw redirect(303, "/")
	}
} satisfies Actions