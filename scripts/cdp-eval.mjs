import { readFileSync } from 'node:fs'

const response = await fetch('http://127.0.0.1:9222/json')
const pages = await response.json()
const page = pages.find((item) => !item.url.includes('#/hud')) ?? pages.find((item) => item.url.includes('index.html')) ?? pages[0]
if (!page) {
  console.error('no page')
  process.exit(1)
}

const socket = new WebSocket(page.webSocketDebuggerUrl)
await new Promise((resolve) => socket.addEventListener('open', resolve, { once: true }))

let nextId = 1
function send(method, params) {
  const id = nextId++
  return new Promise((resolve) => {
    const handler = (event) => {
      const message = JSON.parse(event.data)
      if (message.id === id) {
        socket.removeEventListener('message', handler)
        resolve(message)
      }
    }
    socket.addEventListener('message', handler)
    socket.send(JSON.stringify({ id, method, params }))
  })
}

const expression = process.argv[2]?.endsWith('.js')
  ? readFileSync(process.argv[2], 'utf8')
  : (process.argv[2] ?? 'document.body.innerText')
const result = await send('Runtime.evaluate', {
  expression,
  returnByValue: true,
  awaitPromise: true,
})
console.log(JSON.stringify(result.result ?? result, null, 2))
socket.close()
