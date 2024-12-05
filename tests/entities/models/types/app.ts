import { App, Request, Response } from "../../../.."
import schemaPathArgs from "../../../helpers/schemaPathArgs"
import { SupportCreateInput, SupportFindManyArgs, Teo } from "./entities"

export default function loadApp() {
    const app = new App(schemaPathArgs(__filename, "schema.teo"))
    app.mainNamespace().defineModelHandlerGroup("Support", (group) => {
        group.defineHandler("myCreateObject", async (req: Request) => {
            console.log("here 1")
            const teo: Teo = req.teo
            console.log("here 2")
            const input: SupportCreateInput = req.bodyObject
            console.log("here 3")
            const object = await teo.support.createObject(input)
            console.log("here 4")
            await object.save()
            console.log("here 5")
            return Response.data(await object.toTeon())
        })
        group.defineHandler("myFindManyObjects", async (req: Request) => {
            console.log("there 1")
            const teo: Teo = req.teo
            console.log("there 2")
            const input: SupportFindManyArgs = req.bodyObject
            console.log("there 3")
            const objects = await teo.support.findManyObjects(input)
            console.log("there 4")
            const values = await Promise.all(objects.map(async object => await object.toTeon()))
            console.log("there 5")
            return Response.data(values)
        })
    })
    return app
}