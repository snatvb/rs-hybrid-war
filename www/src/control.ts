import { flow, pipe } from 'fp-ts/lib/function'
import * as O from 'fp-ts/lib/Option'

import { Control, Wasm } from './wasm'

export type OnControl = (control: Control) => void

type Subscribers = {
  keydown: OnControl[]
  keyup: OnControl[]
}

export default function createController({ Control }: Wasm) {
  const CONTROLLERS_MAPPING: Record<string, Control> = {
    KeyA: Control.Left,
    KeyD: Control.Right,
    KeyW: Control.Top,
    KeyS: Control.Bottom,
  }
  const getControl = (event: KeyboardEvent) =>
    O.fromNullable(CONTROLLERS_MAPPING[event.code])

  const subscribers: Subscribers = {
    keydown: [],
    keyup: [],
  }

  const handleKeyDown = flow(
    getControl,
    O.map((control) => {
      subscribers.keydown.forEach((subscriber) => subscriber(control))
    }),
  )

  const handleKeyUp = flow(
    getControl,
    O.map((control) => {
      subscribers.keyup.forEach((subscriber) => subscriber(control))
    }),
  )

  window.addEventListener('keydown', handleKeyDown)
  window.addEventListener('keyup', handleKeyUp)

  return {
    subscribe(type: keyof Subscribers, handler: OnControl) {
      subscribers[type].push(handler)
    },
    unsubscribe(type: keyof Subscribers, handler: OnControl) {
      const index = subscribers[type].indexOf(handler)
      if (index >= 0) {
        subscribers[type].splice(index, 1)
      }
    },
    dispose() {
      window.removeEventListener('keydown', handleKeyDown)
      window.removeEventListener('keyup', handleKeyUp)
    },
  }
}
