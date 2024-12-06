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
    app.mainNamespace().defineTransformPipelineItemFunction("transformDate", (ctx) => {
        const dateOnly: DateOnly = ctx.value
        const date = new Date(dateOnly.toString())
        date.setDate(date.getDate() + 1)
        return date.toISOString().slice(0, 10)
    })
    app.mainNamespace().defineTransformPipelineItemFunction("transformDateTime", (ctx) => {
        const date: Date = ctx.value
        date.setDate(date.getDate() + 1)
        return date
    })
    app.mainNamespace().defineTransformPipelineItemFunction("transformDecimal", (ctx) => new Decimal(ctx.value).times(10))
    app.mainNamespace().defineTransformPipelineItemFunction("transformStatus", (ctx) => {
        const status: Status = ctx.value
        switch (status) {
            case 'open': return 'pending'
            case 'pending': return 'inProgress'
            case 'inProgress': return 'waitingForReview'
            case 'waitingForReview': return 'done'
            case 'done': return 'open'
            default: throw new Error(`unknown status ${status}`)
        }
    })
    app.mainNamespace().defineTransformPipelineItemFunction("transformInt32Array", (ctx) => ctx.value.map((v: number) => v * 10))
    app.mainNamespace().defineTransformPipelineItemFunction("transformInt64Array", (ctx) => ctx.value.map((v: number) => v * 10))
    app.mainNamespace().defineTransformPipelineItemFunction("transformFloat32Array", (ctx) => ctx.value.map((v: number) => v * 10.0))
    app.mainNamespace().defineTransformPipelineItemFunction("transformFloat64Array", (ctx) => ctx.value.map((v: number) => v * 10.0))
    app.mainNamespace().defineTransformPipelineItemFunction("transformBoolArray", (ctx) => ctx.value.map((v: boolean) => !v))
    app.mainNamespace().defineTransformPipelineItemFunction("transformStringArray", (ctx) => ctx.value.map((v: string) => `*${v}*`))
    app.mainNamespace().defineTransformPipelineItemFunction("transformDateArray", (ctx) => ctx.value.map((dateOnly: DateOnly) => {
        const date = new Date(dateOnly.toString())
        date.setDate(date.getDate() + 1)
        return date.toISOString().slice(0, 10)
    }))
    app.mainNamespace().defineTransformPipelineItemFunction("transformDateTimeArray", (ctx) => ctx.value.map((date: Date) => {
        date.setDate(date.getDate() + 1)
        return date
    }))
    app.mainNamespace().defineTransformPipelineItemFunction("transformDecimalArray", (ctx) => ctx.value.map((v: Decimal) => v.times(10)))
    app.mainNamespace().defineTransformPipelineItemFunction("transformStatusArray", (ctx) => {
        const mapper = (value: Status): Status => {
            switch (value) {
                case 'open': return 'pending'
                case 'pending': return 'inProgress'
                case 'inProgress': return 'waitingForReview'
                case 'waitingForReview': return 'done'
                case 'done': return 'open'
                default: throw new Error(`unknown status ${value}`)
            }
        }
        return ctx.value.map(mapper)
    })
    app.main.defineTransformPipelineItem("alterInt32", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterInt64", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterFloat32", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterFloat64", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterBool", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterString", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterDate", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterDateTime", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterDecimal", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterStatus", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterInt32Array", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterInt64Array", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterFloat32Array", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterFloat64Array", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterBoolArray", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterStringArray", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterDateArray", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterDateTimeArray", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterDecimalArray", ({ to }) => () => to)
    app.main.defineTransformPipelineItem("alterStatusArray", ({ to }) => () => to)
    app.main.defineValidatorPipelineItemFunction("validateInt32", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateInt64", (ctx) => null)
    app.main.defineValidatorPipelineItemFunction("validateFloat32", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateFloat64", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateBool", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateString", (ctx) => ctx.value.length > 1 ? null : "string is too short, expect length > 1")
    app.main.defineValidatorPipelineItemFunction("validateDate", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateDateTime", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateDecimal", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateStatus", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateInt32Array", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateInt64Array", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateFloat32Array", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateFloat64Array", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateBoolArray", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateStringArray", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateDateArray", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateDateTimeArray", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateDecimalArray", (ctx) => true)
    app.main.defineValidatorPipelineItemFunction("validateStatusArray", (ctx) => true)
    app.main.defineCallbackPipelineItemFunction("int32Callback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("int64Callback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("float32Callback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("float64Callback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("boolCallback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("stringCallback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("dateCallback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("dateTimeCallback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("decimalCallback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("statusCallback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("int32ArrayCallback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("int64ArrayCallback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("float32ArrayCallback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("float64ArrayCallback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("boolArrayCallback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("stringArrayCallback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("dateArrayCallback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("dateTimeArrayCallback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("decimalArrayCallback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineCallbackPipelineItemFunction("statusArrayCallback", (ctx) => { ctx.object.message = `${ctx.value}` })
    app.main.defineComparePipelineItemFunction("compareInt32", (oldValue: number, newValue: number) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareInt64", (oldValue: number, newValue: number) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareFloat32", (oldValue: number, newValue: number) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareFloat64", (oldValue: number, newValue: number) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareBool", (oldValue: boolean, newValue: boolean) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareString", (oldValue: string, newValue: string) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareDate", (oldValue: DateOnly, newValue: DateOnly) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareDateTime", (oldValue: Date, newValue: Date) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareDecimal", (oldValue: Decimal, newValue: Decimal) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareStatus", (oldValue: Status, newValue: Status) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareInt32Array", (oldValue: number[], newValue: number[]) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareInt64Array", (oldValue: number[], newValue: number[]) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareFloat32Array", (oldValue: number[], newValue: number[]) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareFloat64Array", (oldValue: number[], newValue: number[]) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareBoolArray", (oldValue: boolean[], newValue: boolean[]) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareStringArray", (oldValue: string[], newValue: string[]) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareDateArray", (oldValue: DateOnly[], newValue: DateOnly[]) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareDateTimeArray", (oldValue: Date[], newValue: Date[]) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareDecimalArray", (oldValue: Decimal[], newValue: Decimal[]) => oldValue !== newValue)
    app.main.defineComparePipelineItemFunction("compareStatusArray", (oldValue: Status[], newValue: Status[]) => oldValue !== newValue)
    return app
}
