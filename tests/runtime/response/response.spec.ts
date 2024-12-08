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

test('text response', async (t) => {
    const test_request = new TestRequest({
        method: 'GET',
        uri: '/textResponse',
    })
    const response = await server.process(test_request)
    t.is(response.bodyAsString(), "foo")
})

test('json response', async (t) => {
    const test_request = new TestRequest({
        method: 'GET',
        uri: '/jsonResponse',
    })
    const response = await server.process(test_request)
    t.deepEqual(response.bodyAsJson(), { 'foo': 'bar' })
})

test('file response', async (t) => {
    const test_request = new TestRequest({
        method: 'GET',
        uri: '/fileResponse',
    })
    const response = await server.process(test_request)
    t.deepEqual(response.body(), Buffer.from('foo'))
})

test('cookie in text response', async (t) => {
    const test_request = new TestRequest({
        method: 'GET',
        uri: '/textResponse',
    })
    const response = await server.process(test_request)
    t.is(response.headers.get('set-cookie'), 'foo=bar')
})

test('cookie in json response', async (t) => {
    const test_request = new TestRequest({
        method: 'GET',
        uri: '/jsonResponse',
    })
    const response = await server.process(test_request)
    t.is(response.headers.get('set-cookie'), 'foo=bar')
})

test('cookie in file response', async (t) => {
    const test_request = new TestRequest({
        method: 'GET',
        uri: '/fileResponse',
    })
    const response = await server.process(test_request)
    t.is(response.headers.get('set-cookie'), 'foo=bar')
})