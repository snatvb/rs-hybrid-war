import wasm, { Wasm } from './wasm'

function run(instance: Wasm) {
  try {
    console.clear()
    instance.run()
  } catch (e) {}
}

wasm.then(run)

const app = document.getElementById('app')
if (app) {
  app.innerHTML = `
  <div>
    RUST WASM <- VS -> JS
  </div>
  `
}
