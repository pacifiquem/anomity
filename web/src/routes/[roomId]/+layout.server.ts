import type { LayoutServerLoad } from '../$types';
import { BACKEND_BASE_URL } from '../../utils/constants';

export const load = (async ({cookies, params}) => {
	const sessionId = cookies.get('sessionId');
	const roomId = params.roomId;

	const messages_request = await fetch(`${BACKEND_BASE_URL}/rooms/messages/${roomId}`, {
		method: 'GET',
		headers: {
			"Authorization": `Bearer ${sessionId}`
		}
	})

	const messages = await messages_request.json();

    return {
		messages
	};
}) satisfies LayoutServerLoad;