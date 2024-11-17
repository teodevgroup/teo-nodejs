import test from 'ava'
import { TestRequest, TestServer } from '../../../..'
import loadApp from './app'
import { dateValue, ignore, matchJsonValue } from '../../../../test'

const server = new TestServer(loadApp())

test.before(async () => {
    await server.setup()
})

test.beforeEach(async () => {
    await server.reset()
})

test.serial('create object', async (t) => {
    const test_request = new TestRequest({
        method: 'POST',
        uri: '/Support/myCreateObject',
        body: {
            "int32": 1,
        },
    })
    const response = await server.process(test_request)
    t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
        "data": {
            "id": ignore,
            "int32": 1
        },
    }))
})

test.serial('find many objects', async (t) => {
    // create one
    const test_request_c = new TestRequest({
        method: 'POST',
        uri: '/Support/myCreateObject',
        body: {
            "date": "2005-12-25",
        },
    })
    await server.process(test_request_c)
    // find this one
    const test_request = new TestRequest({
        method: 'POST',
        uri: '/Support/myFindManyObjects',
        body: {
            "orderBy": {
                "id": "asc"
            }
        },
    })
    const response = await server.process(test_request)
    t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
        "data": [{
            "id": ignore,
            "date": dateValue("2005-12-25"),
        }],
    }))
})
