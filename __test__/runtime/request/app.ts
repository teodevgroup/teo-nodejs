import { App } from "../../.."
import schemaPathArgs from "../../helpers/schemaPathArgs"

export default function loadApp() {
    const app = new App(schemaPathArgs(__filename, "schema.teo"))
    
    return app
}