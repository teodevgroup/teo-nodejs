import Decimal from 'decimal.js'
import { App, DateOnly } from "../../.."
import schemaPathArgs from "../../helpers/schemaPathArgs"
import { Container, Status } from "./entities"

export default function loadApp() {
    const app = new App(schemaPathArgs(__filename, "schema.teo"))
    app.mainNamespace().defineTransformPipelineItemFunction("transformInt32", (ctx) => ctx.value * 10)
    app.mainNamespace().defineTransformPipelineItemFunction("transformInt64", (ctx) => ctx.value * 10)
    app.mainNamespace().defineTransformPipelineItemFunction("transformFloat32", (ctx) => ctx.value * 10.0)
    app.mainNamespace().defineTransformPipelineItemFunction("transformFloat64", (ctx) => ctx.value * 10.0)
    app.mainNamespace().defineTransformPipelineItemFunction("transformBool", (ctx) => !ctx.value)
    app.mainNamespace().defineTransformPipelineItemFunction("transformString", (ctx) => `*${ctx.value}*`)
    app.mainNamespace().defineTransformPipelineItemFunction("transformDate", (ctx) => ctx.value.addDays(1))
    app.mainNamespace().defineTransformPipelineItemFunction("transformDateTime", (ctx) => ctx.value.addDays(1))
    return app
}
