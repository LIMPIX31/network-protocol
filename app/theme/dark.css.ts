import { style } from '@vanilla-extract/css'

import { color } from './vars.css.ts'

export const darkMode = style({
	vars: {
		[color.background]: 'oklch(13.24% 0.0304 190.68)',
	},
})
