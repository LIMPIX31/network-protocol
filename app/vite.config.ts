import { defineConfig }         from 'vite'

import { fileURLToPath }        from 'node:url'

import { vanillaExtractPlugin } from '@vanilla-extract/vite-plugin'
import reactPlugin              from '@vitejs/plugin-react'

export default defineConfig({
	plugins: [reactPlugin(), vanillaExtractPlugin()],
	resolve: {
		alias: [
			{
				find: /^&/,
				replacement: fileURLToPath(new URL('./', import.meta.url)),
			},
		],
	},
})
