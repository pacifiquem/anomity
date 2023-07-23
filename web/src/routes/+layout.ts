import type { User } from "../types/user"

export const load = async function ({ data }: { data: { user: User } }) {
	return {
		user: data.user
	}
}