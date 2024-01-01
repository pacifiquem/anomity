import { BACKEND_BASE_URL } from "../utils/constants"

export const load = async ({ locals, cookies }) => {
	try {

		const sessionId = cookies.get("sessionId")
		//const user_request = await fetch(`${BACKEND_BASE_URL}/users/me`, {

		const rooms_request = await fetch(`${BACKEND_BASE_URL}/rooms`, {
			method: "GET",
			headers: {
				Authorization: `Bearer ${sessionId}`
			}
		})

		if(rooms_request.ok){
			return {
				rooms: await rooms_request.json(),
				user: locals.user
			}	
		}
	} catch (error) {
		console.log(error)
	}
}