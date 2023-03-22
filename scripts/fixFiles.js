const { readFileSync, writeFileSync } = require('fs')

let content = readFileSync('./index.js').toString()

let newContent = "\nglobalThis.require = require\nprocess.on('SIGINT', function() { process.exit(0) })\n"
if (!content.endsWith(newContent)) {
  content += newContent
}
writeFileSync("./index.js", content)
