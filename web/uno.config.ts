import {defineConfig} from "unocss"
import extractorSvelte from "@unocss/extractor-svelte"
import presetIcons from "@unocss/preset-icons"
import presetUno from "@unocss/preset-uno"
import presetWebFonts from '@unocss/preset-web-fonts'

export default defineConfig({
	extractors: [
		extractorSvelte()
	],
	presets: [
		presetUno(),
		presetIcons(),
		presetWebFonts({
			provider: "google",
			fonts: {
				sans: "DM Sans"
			}
		})
	]
})