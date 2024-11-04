const fs = require('fs')
const { globSync } = require('glob')
const path = require('path')

globSync('__test__/**/*.{teo,jpg,txt}').forEach((src) => {
    const dest = path.join('__test_build__', src.replace(/^__test__\/?\\?/, ''))
    const destDir = path.dirname(dest)
    if (!fs.existsSync(destDir)) {
        fs.mkdirSync(destDir, { recursive: true })
    }
    fs.copyFileSync(src, dest)
})
