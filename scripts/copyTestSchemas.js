const fs = require('fs')
const { globSync } = require('glob')
const path = require('path')

globSync('__test__/**/*.teo').forEach((src) => {
    dest = path.join('__test_build__', src.replace(/^__test__\/?\\?/, ''))
    fs.copyFileSync(src, dest)
})
