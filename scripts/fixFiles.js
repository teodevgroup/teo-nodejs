const { readFileSync, writeFileSync } = require('fs')

let arg = process.argv[process.argv.length - 1];
if (arg === 'index.js') {
  let content = readFileSync('./index.js')
  content += "\nglobalThis.require = require\nprocess.on('SIGINT', function() { process.exit(0) })\n"
  writeFileSync("./index.js", content)
}
