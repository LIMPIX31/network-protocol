import type { FC } from 'react'

function apply<T, R>(target: T, fn: (t: T) => R) {
	return fn(target)
}

type Provider = (Component: FC) => FC

const providers: Provider[] = []

export function withProviders(component: FC) {
	return providers.reduce(apply, component)
}
