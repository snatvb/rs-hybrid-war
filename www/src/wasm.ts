export type { Control } from '../../wasm/pkg/wasm.js'

const wasm = import('../../wasm/pkg/wasm.js')

export type Wasm = typeof wasm extends Promise<infer T> ? T : never

export default wasm
