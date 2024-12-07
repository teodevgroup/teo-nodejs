import { App, Request, Response } from "../../../.."
import schemaPathArgs from "../../../helpers/schemaPathArgs"
import { Support, SupportCreateInput, SupportFindManyArgs, Teo } from "./entities"

export default function loadApp() {
    const app = new App(schemaPathArgs(__filename, "schema.teo"))
    app.mainNamespace().defineModelHandlerGroup("Support", (group) => {
        group.defineHandler("myCreateObject", async (req: Request) => {
            const teo: Teo = req.teo
            const input: SupportCreateInput = req.bodyObject
            const object = await teo.support.createObject(input)
            await object.save()
            return Response.data(await object.toTeon())
        })
        group.defineHandler("myFindManyObjects", async (req: Request) => {
            const teo: Teo = req.teo
            const input: SupportFindManyArgs = req.bodyObject
            await (teo as any)._transaction(async (teo: Teo) => {
                const objects = await teo.support.findManyObjects(input)
                return objects
            })
            const objects = await teo.support.findManyObjects(input)
            const values = await Promise.all(objects.map(async object => await object.toTeon()))
            return Response.data(values)
        })
    })
    return app
}