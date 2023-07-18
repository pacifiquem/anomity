import { error, redirect } from "@sveltejs/kit";
import { BACKEND_BASE_URL } from "../../utils/constants";
import type { Actions } from "./$types";

export const actions: Actions = {
  login: async ({ request }) => {
    const form_data = await request.formData();
    const request_body = Object.fromEntries(form_data.entries());

    const login_request = await fetch(`${BACKEND_BASE_URL}/users/login`, {
      method: "POST",
      body: JSON.stringify(request_body),
      headers: {
        "Content-Type": "application/json",
      },
    });

    if (!login_request.ok) {
      throw error(400, {
        message: (await login_request.json()).message,
      });
    }

	console.log(login_request.headers);
	console.log("login_request", await login_request.json());

    throw redirect(301, "/");
  },
};
