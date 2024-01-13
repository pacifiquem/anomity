import { BACKEND_BASE_URL } from "../utils/constants"
import {error} from "@sveltejs/kit"

export const load = async ({ locals, cookies }) => {
	try {
		const sessionId = cookies.get("sessionId")
		
		const rooms_request = await fetch(`${BACKEND_BASE_URL}/rooms`, {
			method: "GET",
			headers: {
				Authorization: `Bearer ${sessionId}`
			}
		})

		if(rooms_request.ok){
			const rooms = await rooms_request.json()

			return {
				rooms,
				user: locals.user
			}	
		}

		return {
			rooms: [],
			user: locals.user
		}
	} catch (msg) {
		throw error(400, "Invalid session")	
	}
}