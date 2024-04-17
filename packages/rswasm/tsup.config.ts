import { defineConfig } from 'tsup'

export default defineConfig({
	entry: ['bin.ts'],
	platform: 'node',
	format: 'esm',
	clean: true,
})
