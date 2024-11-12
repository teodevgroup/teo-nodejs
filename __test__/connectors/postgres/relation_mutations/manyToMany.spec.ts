import test from 'ava'
import { App, TestServer } from '../../../..'
import schemaPathArgs from '../../../helpers/schemaPathArgs'
import builtinReq from '../../../helpers/builtinReq'
import { ignore, matchJson, matchJsonValue, oneMatches } from '../../../../test'

const server = new TestServer(new App(schemaPathArgs(__filename, "schema.teo")))

test.before(async () => {
    await server.setup()
})

test.beforeEach(async () => {
    await server.reset()
})

test('create with nested create one', async (t) => {
    const _createRes = await builtinReq(server, "create", "Artist", {
        "create": {
            "name": "Taylor Swift",
            "songs": {
                "create": {
                    "name": "Love Story"
                }
            }
        },
    })
    const findManyRes = await builtinReq(server, "findMany", "Artist", {
        "include": {
            "songs": true
        }
    })
    t.notThrows(() => matchJsonValue(findManyRes, {
        "meta": {
            "count": 3
        },
        "data": oneMatches({
            "id": ignore,
            "name": "Taylor Swift",
            "songs": oneMatches({
                "id": ignore,
                "name": "Love Story"
            })
        })
    }))
})