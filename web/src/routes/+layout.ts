import type { User } from "../types/user"
import { BACKEND_BASE_URL } from "../utils/constants"

export const load = async function ({ data }: { data: { user: User } }) {
	try {
		const rooms_request = await fetch(`${BACKEND_BASE_URL}/rooms`, {
			method: "GET",
			headers: {
				Authorization: `Bearer ${data.user}`
			}
		})

		const rooms_response = await rooms_request.json()
	} catch (error) {
		//console.log(error)
	}

	return {
		user: data.user
	}
}