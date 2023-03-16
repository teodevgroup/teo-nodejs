const { App } = require("./index.js")

async function main() {
  const app = new App()
  app.transform("replaceComWithIo", function (input) {
    return input.replace(/com$/, 'io')
  })
  await app.run()
}

main()
