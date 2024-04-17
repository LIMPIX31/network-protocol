import init from '@crate/hello'

await init()

export function Root() {
	return <h1>Hello, world</h1>
}
