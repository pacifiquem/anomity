import { redirect, type Handle } from "@sveltejs/kit";

const protected_routes = ["/"]

export const handle: Handle = async ({ event, resolve }) => {
	const session = event.cookies.get("session")

	if (protected_routes.includes(event.url.pathname) && !session) {
		throw redirect(303, "/signin")
	}

	if (!session) {
		return await resolve(event)
	}

	return await resolve(event)
}