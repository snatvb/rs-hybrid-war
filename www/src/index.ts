import wasm, { Wasm } from './wasm'

function run(instance: Wasm) {
  try {
    console.clear()
    instance.run()
  } catch (e) {}
}

wasm.then(run)
