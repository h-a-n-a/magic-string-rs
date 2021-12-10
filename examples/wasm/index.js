import init, { create } from '../../wasm/pkg/wasm.js'

await init()
console.log(create(123))
