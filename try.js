//const { App, getModelClass } = require("./index.js")
const { App, Response } = require("./index.js")
const Decimal = require('decimal.js')
//const User = getModelClass('User')

const app = new App()

app.mainNamespace().definePipelineItem("replaceComWithIo", (input) => input.replace(/com$/, 'io'))

app.mainNamespace().defineHandler("myHandler", () => {
    var res = Response.html("<h1>Hello, Teo!</h1>")
    console.log(res)
    return res
})

// app.transform("replaceComWithIo", async function (input) {
//   return input.replace(/com$/, 'io')
// })
//
// app.transform("10times", async function (input) {
//   return Decimal((input || Decimal(5)) * 10)
// })
//
// app.validate("oddLength", (s) => s.length % 2 == 0)
//
// app.callback("print", async (v) => {
//   let user = await User.create({ email: `${v}@gmail.com` });
//   await user.save()
//   user.email = user.email.replace("gmail.com", "outlook.com");
//   console.log(user.id, user, user.email)
// })

app.run()
