import { App, Request, Response } from "../../.."
import schemaPathArgs from "../../helpers/schemaPathArgs"

class NumberWrapper {
    constructor(public number: number) {
        this.number = number
    }
}

export default function loadApp() {
    const app = new App(schemaPathArgs(__filename, "schema.teo"))
    app.mainNamespace().defineHandler("inspect", (request: Request) => {
        const number: number = request.localValues.get("number")
        const numberWrapper: NumberWrapper = request.localObjects.get("number")
        return Response.teon({
            "numberFromValues": number,
            "numberFromObjects": numberWrapper.number
        })
    })
    app.mainNamespace().defineRequestMiddleware("requestOuter", () => {
        return async (request: Request, next: (request: Request) => Promise<Response>) => {
            request.localValues.set("number", 42)
            return await next(request)
        }
    })
    app.mainNamespace().defineRequestMiddleware("requestMiddle", () => {
        return async (request: Request, next: (request: Request) => Promise<Response>) => {
            request.localValues.set("number", request.localValues.get("number") * 2)
            return await next(request)
        }
    })
    app.mainNamespace().defineRequestMiddleware("requestInner", () => {
        return async (request: Request, next: (request: Request) => Promise<Response>) => {
            request.localValues.set("number", request.localValues.get("number") + 16)
            return await next(request)
        }
    })
    app.mainNamespace().defineHandlerMiddleware("handlerOuter", () => {
        return async (request: Request, next: (request: Request) => Promise<Response>) => {
            request.localObjects.set("number", new NumberWrapper(24))
            return await next(request)
        }
    })
    app.mainNamespace().defineHandlerMiddleware("handlerMiddle", () => {
        return async (request: Request, next: (request: Request) => Promise<Response>) => {
            const wrapper: NumberWrapper = request.localObjects.get("number")
            wrapper.number *= 4
            return await next(request)
        }
    })
    app.mainNamespace().defineHandlerMiddleware("handlerInner", () => {
        return async (request: Request, next: (request: Request) => Promise<Response>) => {
            const wrapper: NumberWrapper = request.localObjects.get("number")
            wrapper.number += 4
            return await next(request)
        }
    })
    return app
}