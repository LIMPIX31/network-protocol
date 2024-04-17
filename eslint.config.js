import base from '@ltos/config-eslint'

/** @type {import('eslint').Linter.FlatConfig[]} */
export default [
	...base,
	{
		ignores: ['target'],
	},
]
