import fs from 'node:fs'
import path from 'node:path'

const dir = 'apps/desktop/dist/assets'
const mapFile = fs.readdirSync(dir).find((file) => file.startsWith('index-') && file.endsWith('.js.map'))
if (!mapFile) {
  console.error('no map')
  process.exit(1)
}
const map = JSON.parse(fs.readFileSync(path.join(dir, mapFile), 'utf8'))
const index = map.sources.findIndex((source) => source.endsWith('main.ts'))
console.log(map.sources[index])
console.log(map.sourcesContent[index])
