import { redirect, type Handle } from "@sveltejs/kit";
import { BACKEND_BASE_URL } from "./utils/constants";

const protected_routes = ["/"]

export const handle: Handle = async ({ event, resolve }) => {
	const session = event.cookies.get("sessionId")

	try {
		const user_request = await fetch(`${BACKEND_BASE_URL}/users/me`, {
			headers: {
				Authorization: `Bearer ${session}`
			}
		})
	
	
		if (user_request.ok) {
			const user_response = await user_request.json()
	
			if (protected_routes.includes(event.url.pathname) && !user_response.id) {
				redirect(303, "/signin");
			}
	
			event.locals.user = user_response
		}
	} catch (error) {
		if(session) {
			event.cookies.delete("sessionId", { path: "/" })

			redirect(303, "/signin");
		}
	}
	


	return resolve(event)
}