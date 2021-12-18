// export type { Control } from '../../wasm/pkg/wasm.js'

const PUBLIC_WASM_PATH = 'wasm/wasm_bg.wasm'

const wasm = import('../../wasm/pkg/wasm.js')

export type Wasm = typeof wasm extends Promise<infer T> ? T : never

export default wasm.then((loaded) => {
  console.log('loading...', PUBLIC_WASM_PATH)
  return loaded.default(PUBLIC_WASM_PATH) as unknown as Promise<Wasm>
})
