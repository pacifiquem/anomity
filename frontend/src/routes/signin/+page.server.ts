import { redirect } from "@sveltejs/kit";
import { BACKEND_BASE_URL } from "../../utils/constants";
import type { Actions } from "./$types";

export const actions: Actions = {
	signin: async ({ request }) => {
		const form_data = await request.formData()
		const request_body = Object.fromEntries(form_data.entries())

		const signin_request = await fetch(`${BACKEND_BASE_URL}/users/signin`, {
			method: "POST",
			body: JSON.stringify(request_body),
			headers: {
				"Content-Type": "application/json"
			}
		})

		if (!signin_request.ok) {
			return {
				signin_error: (await signin_request.json()).message
			}
		}

		throw redirect(303, "/")
	}
};