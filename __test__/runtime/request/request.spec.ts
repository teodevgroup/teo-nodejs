import test from 'ava'
import { TestRequest, TestServer } from '../../..'
import loadApp from './app'

const server = new TestServer(loadApp())

test.before(async () => {
    await server.setup()
})

test.beforeEach(async () => {
    await server.reset()
})

test('path', async (t) => {
    const test_request = new TestRequest({
        method: 'POST',
        uri: '/',
        body: {},
    })
    const response = await server.process(test_request)
    t.is(response.bodyObject()['path'], '/')
})

test('query', async (t) => {
    const test_request = new TestRequest({
        method: 'POST',
        uri: '/?foo=bar',
        body: {},
    })
    const response = await server.process(test_request)
    t.is(response.bodyObject()['query'], 'foo=bar')
})