import { App, Request, Response } from "../../.."
import schemaPathArgs from "../../helpers/schemaPathArgs"

export default function loadApp() {
    const app = new App(schemaPathArgs(__filename, "schema.teo"))
    app.mainNamespace().defineHandler("inspect", (request: Request) => {
        const contentType = request.headers.get("content-type")
        return Response.teon({
            "path": request.path,
            "query": request.query,
            "contentTypeFromHeader": contentType,
            "contentType": request.contentType,
            "method": request.method,
        })
    })
    app.mainNamespace().defineHandler("echo", (request: Request) => {
        const captures = request.captures
        const echo = captures["data"]
        return Response.string(echo, "text/plain")
    })
    app.mainNamespace().defineHandler("echoMore", (request: Request) => {
        const captures = request.captures
        const echo = captures["data"]
        return Response.string(echo, "text/plain")
    })
    app.mainNamespace().defineHandler("echoJsonBody", (request: Request) => {
        return Response.teon(request.bodyObject)
    })
    app.mainNamespace().defineHandler("echoFormBody", (request: Request) => {
        const filepath = request.bodyObject['avatar'].filepath
        return Response.teon({
            "name": request.bodyObject()['name'],
            "avatar": filepath
        })
    })
    app.mainNamespace().defineHandler("echoCookie", (request: Request) => {
        return Response.teon({
            "cookies": request.cookies.map((cookie) => ({
                "name": cookie.name(), 
                "value": cookie.value(),
            }))
        })
    })
    return app
}