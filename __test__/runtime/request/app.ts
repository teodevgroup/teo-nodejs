import { App, RequestCtx, Response } from "../../.."
import schemaPathArgs from "../../helpers/schemaPathArgs"

export default function loadApp() {
    const app = new App(schemaPathArgs(__filename, "schema.teo"))
    app.mainNamespace().defineHandler("inspect", (ctx: RequestCtx) => {
        const contentType = ctx.request().header("content-type")
        return Response.teon({
            "path": ctx.request().path(),
            "queryString": ctx.request().queryString(),
            "contentTypeFromHeader": contentType,
            "contentType": ctx.request().contentType(),
            "method": ctx.request().method(),
        })
    })
    app.mainNamespace().defineHandler("echo", (ctx: RequestCtx) => {
        const captures = ctx.handlerMatch().captures()
        const echo = captures["data"]
        return Response.string(echo, "text/plain")
    })
    app.mainNamespace().defineHandler("echoMore", (ctx: RequestCtx) => {
        const captures = ctx.handlerMatch().captures()
        const echo = captures["data"]
        return Response.string(echo, "text/plain")
    })
    app.mainNamespace().defineHandler("echoJsonBody", (ctx: RequestCtx) => {
        return Response.teon(ctx.body())
    })
    app.mainNamespace().defineHandler("echoFormBody", (ctx: RequestCtx) => {
        const filepath = ctx.body()['avatar'].filepath
        return Response.teon({
            "name": ctx.body()['name'],
            "avatar": filepath
        })
    })
    app.mainNamespace().defineHandler("echoCookie", (ctx: RequestCtx) => {
        return Response.teon({
            "cookies": ctx.request().cookies().map((cookie) => ({
                "name": cookie.name(), 
                "value": cookie.value(),
            }))
        })
    })
    return app
}