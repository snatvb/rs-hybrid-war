class Vec2d {
  constructor(x, y) {
    this.update(x, y)
  }

  update(x, y) {
    this.x = x
    this.y = y
  }

  to_string() {
    return `Vec2d(${this.x}, ${this.y})`
  }
}

class Size {
  constructor(width, height) {
    this.update(width, height)
  }

  update(width, height) {
    this.width = width
    this.height = height
  }
}

class Rect {
  constructor(position, size, color = '#000') {
    this.position = position
    this.size = size
    this.color = color
  }
}

function rand_range(min, max) {
  min = Math.ceil(min)
  max = Math.floor(max)
  return Math.floor(Math.random() * (max - min + 1)) + min
}

const mapRange = (count, fn) => {
  const result = []
  for (let i = 0; i < count; i++) {
    result.push(fn(i))
  }
  return result
}

function average(numbers) {
  return numbers.reduce((acc, n) => acc + n, 0) / numbers.length
}

function draw_rect(ctx, rect) {
  // ctx.fillStyle = rect.color
  ctx.fillRect(
    rect.position.x,
    rect.position.y,
    rect.size.width,
    rect.size.height,
  )
}

function rand_color() {
  let r = rand_range(0.0, 255.0)
  let g = rand_range(0.0, 255.0)
  let b = rand_range(0.0, 255.0)
  return `rgb(${r}, ${g}, ${b})`
}

function show_last_message(ctx, canvas, text) {
  ctx.clearRect(0.0, 0.0, canvas.width, canvas.height)
  ctx.fillStyle = '#000'
  ctx.font = '24px Arial'
  ctx.fillText(text, 12.0, canvas.height / 2.0 - 12.0)
}

function make_rand_rect(canvas) {
  let screen_width = canvas.width
  let screen_height = canvas.height
  let rSize = new Size(rand_range(10.0, 100.0), rand_range(10.0, 100.0))

  return new Rect(
    new Vec2d(
      rand_range(0.0, screen_width - rSize.width),
      rand_range(0.0, screen_height - rSize.height),
    ),
    rSize,
    rand_color(),
  )
}

const RECTS_COUNT = 2500
const ARRAY_ELEMENTS = 200000

export function arrays_bench() {
  let start_time_creating = performance.now()
  let list = mapRange(ARRAY_ELEMENTS, (i) => new Vec2d(i, i))
  let end_time_creating = performance.now()

  let start_time_editing = performance.now()
  list.forEach((v) => v.update(v.x * 2, v.y * 2))
  let end_time_editing = performance.now()

  console.log(
    `[JS] Array creating ${
      end_time_creating - start_time_creating
    }ms; Editing ${
      end_time_editing - start_time_editing
    }ms with ${ARRAY_ELEMENTS} elements`,
  )

  {
    let start_time = performance.now()
    let _ = list.reduce((acc, v) => acc + `${v.to_string()}, `, '')
    let end_time = performance.now()
    console.log(`[JS] Serialize time ${end_time - start_time}ms`)
  }
  {
    let start_time = performance.now()
    let sum = list.reduce((acc, v) => acc + v.x + v.y, 0)
    let end_time = performance.now()
    console.log(`[JS] Sum(${sum}) time ${end_time - start_time}ms`)
  }
}

export const run = () =>
  new Promise((resolve) => {
    const canvas = document.getElementById('canvas-js')
    const ctx = canvas.getContext('2d')

    /**
     * @type Rect[]
     */
    const rects = mapRange(2500, make_rand_rect)
    console.log('[JS] rects count:', RECTS_COUNT)

    let frame = 0
    /**
     * @type number[]
     */
    let times = []
    const tick = () => {
      if (times.length > 150) {
        let avg_ms = average(times)
        show_last_message(ctx, canvas, `Average time JS: ${avg_ms}`)
        resolve()
        return
      }

      frame++

      // ---- FRAME ----

      let screen_width = canvas.width
      let screen_height = canvas.height

      ctx.clearRect(0.0, 0.0, screen_width, screen_height)
      let start_time = performance.now()
      // ctx.fillStyle = rand_color()

      // for (let index = 0; index < RECTS_COUNT; index++) {
      //   draw_rect(ctx, make_rand_rect(canvas))
      // }

      rects.forEach((rect) => {
        rect.color = rand_color()
        rect.size.update(rand_range(10.0, 100.0), rand_range(10.0, 100.0))
        rect.position.update(
          rand_range(0.0, screen_width - rect.size.width),
          rand_range(0.0, screen_height - rect.size.height),
        )
        // draw_rect(ctx, rect)
      })

      let ms_time = performance.now() - start_time
      ctx.fillStyle = '#000'
      ctx.font = '12px Arial'
      ctx.fillText(`Frame: ${frame}`, 2.0, 14.0)
      times.push(ms_time)

      ctx.fillText(`ms: ${ms_time}`, 2.0, canvas.height - 14.0)

      requestAnimationFrame(tick)
    }

    requestAnimationFrame(tick)
  })
