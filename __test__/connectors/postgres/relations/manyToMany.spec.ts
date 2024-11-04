import test from 'ava'
import { App, TestServer } from '../../../..'
import schemaPathArgs from '../../../helpers/schemaPathArgs'
import builtinReq from '../../../helpers/builtinReq'

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
    t.deepEqual(findManyRes["meta"], { "count": 3 })
    t.assert(findManyRes["data"].find((item: any) => {
        return item.name === 'Taylor Swift' && item.songs[0].name === 'Love Story'
    }))
})