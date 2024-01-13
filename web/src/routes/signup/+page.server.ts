import type { Actions } from "./$types";
import { BACKEND_BASE_URL } from "../../utils/constants";
import { error, redirect } from "@sveltejs/kit";

export const ssr = false;

/** @type {import('./$types').Actions} */
export const actions = {
	signup: async ({ request, cookies }) => {
		const formData = await request.formData()
		
		const signup_response = await fetch(`${BACKEND_BASE_URL}/users`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify(Object.fromEntries(formData.entries())),
		})

		if (!signup_response.ok) {
			const response = await signup_response.json()
			error(400,{ message: response.message });
		}
			
		cookies.set("sessionId", await signup_response.text(), {
			path: "/"
		})


		redirect(301, "/");
	}
} satisfies Actions