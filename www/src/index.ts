import createController from './control'
import wasm, { Wasm } from './wasm'

function run(instance: Wasm) {
  const controller = createController(instance)
  const game = instance.Game.new()
  controller.subscribe('keydown', game.handle_key_down)
  controller.subscribe('keyup', game.handle_key_up)
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
