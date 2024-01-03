//const { App, getModelClass } = require("./index.js")
const { App, Response } = require("./index.js")
const Decimal = require('decimal.js')
//const User = getModelClass('User')

const app = new App()

app.mainNamespace().defineMiddleware("myWrap1", (args) => {
    return async (req, next) => {
        console.log("1 before")
        const res = await next(req)
        console.log("1 after")
        return res
    }
});

app.mainNamespace().defineMiddleware("myWrap2", (args) => {
    return async (req, next) => {
        console.log("2 before")
        const res = await next(req)
        console.log("2 after")
        return res
    }
});

app.mainNamespace().definePipelineItem("replaceComWithIo", async (input, _, __, teo) => {
    var res = await teo.user.findMany({})
    console.log(res)
    return input.replace(/com$/, 'io')
})

app.mainNamespace().defineHandler("myHandler", async (_, teo) => {
    //throw new Error("Error!!")
    // console.log("a")
    var res = await teo.user.findMany({})
    console.log(res)
    return Response.data({"a":1,"b":2})
})

app.setup(async (teo) => {
    console.log("setup")
    let results = await teo.user.findMany({})
    console.log(results)
})

app.program("mycode", async (teo) => {
    console.log("mycode")
    let results = await teo.user.findMany({})
    console.log(results)
})

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
