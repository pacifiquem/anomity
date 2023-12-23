import {defineConfig} from "unocss"
import extractorSvelte from "@unocss/extractor-svelte"
import presetIcons from "@unocss/preset-icons"
import presetUno from "@unocss/preset-uno"
//import presetReset from "@unocss/reset/tailwind.css"


export default defineConfig({
	extractors: [
		extractorSvelte()
	],
	presets: [
		presetUno(),
		presetIcons(),
	]
})