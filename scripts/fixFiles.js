const { readFileSync, writeFileSync } = require('fs')

let content = readFileSync('./index.js')
content += "\nglobalThis.require = require\nprocess.on('SIGINT', function() { process.exit(0) })\n"
writeFileSync("./index.js", content)
