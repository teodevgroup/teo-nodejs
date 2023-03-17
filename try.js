const { App } = require("./index.js")
const Decimal = require('decimal.js')

async function main() {
  const app = new App()
  app.transform("replaceComWithIo", async function (input) {
    return input.replace(/com$/, 'io')
  })
  app.transform("10times", async function (input) {
    return Decimal((input || Decimal(5)) * 10)
  })
  app.validate("oddLength", (s) => s.length % 2 == 0)
  app.callback("print", (v) => console.log(v))
  await app.run()
}

main()
