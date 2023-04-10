// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			user: {
				id: string;
				username: string;
				email: string;
				password: string;
				createdAt: string;
				updatedAt: string;
			}
		}
		// interface PageData {}
		// interface Platform {}
	}
}

export { };
