import { sveltekit } from '@sveltejs/kit/vite';
import UnoCSS from 'unocss/vite'
import { type ViteDevServer, defineConfig } from "vite"
import { Server } from "socket.io"

const webSocketServer = {
	name: 'webSocketServer',
	configureServer(server: ViteDevServer) {
		if(!server.httpServer) return 

		const io = new Server(server.httpServer)

		io.on("connection", (socket) => {
			socket.emit("message", "Client connected")
		})
	}
}

export default defineConfig({
	plugins: [
        UnoCSS(),
		sveltekit(),
		webSocketServer
	]
});
