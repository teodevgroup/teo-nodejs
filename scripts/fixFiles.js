const { readFileSync, writeFileSync } = require('fs')

let content = readFileSync('./index.js').toString()

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
writeFileSync("./index.js", content)
