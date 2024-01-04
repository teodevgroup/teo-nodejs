const { readFileSync, writeFileSync } = require('fs')

function fixIndexJs(filename) {
  let content = readFileSync(filename).toString()

  let newContent = `
App.prototype.run = async function() {
  await this._prepare();
  return this._run();
}
globalThis.require = require
process.on('SIGINT', function() { process.exit(0) })
`
  if (!content.endsWith(newContent)) {
    content += newContent
  }
  writeFileSync(filename, content)
}

function fixIndexDTs(filename) {
  let content = readFileSync(filename).toString()
  content = content.replace("_run(): Promise<void>", `_run(): Promise<void>
  /** Run this app. */
  run(): Promise<void>`)
  writeFileSync(filename, content)
}


let filename = process.argv[process.argv.length - 1]
if (filename.endsWith('index.js')) {
  fixIndexJs(filename)
} else if (filename.endsWith('index.d.ts')) {
  fixIndexDTs(filename)
}
