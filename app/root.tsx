import init      from "@crate/cursor"
import { setup } from "@crate/cursor"
import { Port }  from "@crate/cursor"

await init()
setup()
const port = Port.new(0)
const offer = await port.offer()
console.log(offer)

export function Root() {
	return <h1>Hello, world</h1>
}
