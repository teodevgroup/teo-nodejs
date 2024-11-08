import path from 'path'
import { App, Cookie, Request, Response } from "../../.."
import schemaPathArgs from "../../helpers/schemaPathArgs"

export default function loadApp() {
    const app = new App(schemaPathArgs(__filename, "schema.teo"))
    app.mainNamespace().defineHandler('textResponse', async (_request) => {
        const response = Response.string('foo', 'text/plain')
        response.addCookie(new Cookie("foo", "bar"))
        return response
    })
    app.mainNamespace().defineHandler('jsonResponse', async (_request) => {
        const response = Response.teon({ 'foo': 'bar' })
        response.addCookie(new Cookie('foo', 'bar'))
        return response
    })
    app.mainNamespace().defineHandler('fileResponse', async (_request) => {
        const response = Response.file(path.join(path.dirname(__filename), 'response.txt'))
        response.addCookie(new Cookie('foo', 'bar'))
        return response
    })
    return app
}