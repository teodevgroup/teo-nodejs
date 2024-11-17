const fs = require('fs')
const { globSync } = require('glob')
const path = require('path')

globSync('tests/**/*.{teo,jpg,txt}').forEach((src) => {
    const dest = path.join('tests_build', src.replace(/^tests\/?\\?/, ''))
    const destDir = path.dirname(dest)
    if (!fs.existsSync(destDir)) {
        fs.mkdirSync(destDir, { recursive: true })
    }
    fs.copyFileSync(src, dest)
})
