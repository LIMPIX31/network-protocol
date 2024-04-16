import { globalStyle } from "@vanilla-extract/css"

import { preflight }   from './layers.css.ts'
import { color }       from './vars.css.ts'

globalStyle('*, *::before, *::after', {
	'@layer': {
		[preflight]: {
			boxSizing: 'border-box',
		},
	},
})

globalStyle('*', {
	'@layer': {
		[preflight]: {
			margin: 0,
			padding: 0,
			font: 'inherit',
		},
	},
})

globalStyle('body', {
	'@layer': {
		[preflight]: {
			minHeight: '100vh',
			backgroundColor: color.background,
		},
	},
})

globalStyle('img, picture, svg, video', {
	'@layer': {
		[preflight]: {
			display: 'block',
			maxWidth: '100%',
		},
	},
})

globalStyle('img', {
	'@layer': {
		[preflight]: {
			fontStyle: 'italic',
			backgroundRepeat: 'no-repeat',
			backgroundSize: 'cover',
			shapeMargin: '1rem',
			maxWidth: '100%',
			height: 'auto',
			verticalAlign: 'middle',
		},
	},
})
