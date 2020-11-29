import { Client } from 'wasm-2'

const canvas = document.getElementById('rustCanvas')
const gl = canvas.getContext('webgl', { antialias: true })

const FPS_THROTTLE = 1000.0 / 240.0

const client = new Client()
const initialTime = Date.now()

let lastDrawTime = -1
const render = () => {
  window.requestAnimationFrame(render)
  const currTime = Date.now()
  if (currTime >= lastDrawTime + FPS_THROTTLE) {
    lastDrawTime = currTime
    if (
      window.innerHeight != canvas.height ||
      window.innerWidth != canvas.width
    ) {
      canvas.height = window.innerHeight
      canvas.style.height = window.innerHeight

      canvas.width = window.innerWidth
      canvas.style.width = window.innerWidth

      gl.viewport(0, 0, window.innerWidth, window.innerHeight)
    }
    let elapsedTime = currTime - initialTime
    client.update(elapsedTime, window.innerHeight, window.innerWidth)
    client.render()
  }
}
render()
