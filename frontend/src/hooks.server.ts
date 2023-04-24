import { redirect, type Handle } from "@sveltejs/kit";
import { BACKEND_BASE_URL } from "./utils/constants";

const protected_routes = ["/"]

export const handle: Handle = async ({ event, resolve }) => {
	const session = event.cookies.get("session")

	if (protected_routes.includes(event.url.pathname) && !session) {
		throw redirect(303, "/signin")
	}

	if (!session) {
		return await resolve(event)
	}

	const user = await fetch(`${BACKEND_BASE_URL}/users/me`)

	event.locals.user = await user.json()

	return await resolve(event)
}