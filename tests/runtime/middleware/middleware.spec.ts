import test from 'ava'
import { TestRequest, TestServer } from '../../..'
import loadApp from './app'
import { matchJsonValue } from '../../../test'

const server = new TestServer(loadApp())

test.before(async () => {
    await server.setup()
})

test.beforeEach(async () => {
    await server.reset()
})

test('middleware and request locals', async (t) => {
    const test_request = new TestRequest({
        method: 'POST',
        uri: '/',
        body: {},
    })
    const response = await server.process(test_request)
    t.notThrows(() => matchJsonValue(response, {
        "data": {
            "numberFromValues": 100,
            "numberFromObjects": 100,
        },
    }))
})
