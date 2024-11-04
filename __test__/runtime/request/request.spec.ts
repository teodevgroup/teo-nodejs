import test from 'ava'
import fs from 'fs'
import path from 'path'
import { FormDataEncoder } from 'form-data-encoder'
import { Readable } from 'stream'
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

test('content type from header', async (t) => {
    const test_request = new TestRequest({
        method: 'POST',
        uri: '/?foo=bar',
        headers: {
            'content-type': 'application/json',
        },
        body: {},
    })
    const response = await server.process(test_request)
    t.is(response.bodyObject()['contentTypeFromHeader'], 'application/json')
})

test('content type', async (t) => {
    const test_request = new TestRequest({
        method: 'POST',
        uri: '/?foo=bar',
        headers: {
            'content-type': 'application/json',
        },
        body: {},
    })
    const response = await server.process(test_request)
    t.is(response.bodyObject()['contentType'], 'application/json')
})

test('method', async (t) => {
    const test_request = new TestRequest({
        method: 'POST',
        uri: '/?foo=bar',
        headers: {
            'content-type': 'application/json',
        },
        body: {},
    })
    const response = await server.process(test_request)
    t.is(response.bodyObject()['method'], 'POST')
})

test('captures', async (t) => {
    const test_request = new TestRequest({
        method: 'GET',
        uri: '/echo/foo',
    })
    const response = await server.process(test_request)
    t.is(response.body(), 'foo')
})

test('combined captures', async (t) => {
    const test_request = new TestRequest({
        method: 'GET',
        uri: '/echo/foo/bar/echo',
    })
    const response = await server.process(test_request)
    t.is(response.body(), 'foo/bar')
})

test('json body', async (t) => {
    const test_request = new TestRequest({
        method: 'PATCH',
        uri: '/echo/jsonBody',
        body: {
            name: 'foo',
            age: 1
        },
    })
    const response = await server.process(test_request)
    t.deepEqual(response.bodyObject(), {
        name: 'foo',
        age: 1
    })
})

test('form body', async (t) => {
    const form = new FormData()
    form.append('name', 'Shiranui Mai');
    form.append('avatar', await fs.openAsBlob(path.join(path.dirname(__filename), 'mai.jpg')), 'mai.jpg')
    const encoder = new FormDataEncoder(form)
    const readable = Readable.from(encoder.encode())
    const buffers = []
    for await (let chunk of readable) {
        buffers.push(chunk)
    }
    const buffer = Buffer.from(Buffer.concat(buffers))
    const test_request = new TestRequest({
        method: 'PATCH',
        uri: '/echo/formBody',
        headers: encoder.headers,
        body: buffer,
    })
    const response = await server.process(test_request)
    const responseBody = response.bodyObject()
    const responseKeys = Object.keys(responseBody)
    t.is(responseKeys.length, 2)
    t.assert(responseKeys.includes('name'))
    t.assert(responseKeys.includes('avatar'))
    t.is(responseBody['name'], 'Shiranui Mai')
    t.assert((responseBody['avatar'] as string).endsWith('.jpg'))
})

test('cookie', async (t) => {
    const test_request = new TestRequest({
        method: 'POST',
        uri: '/echo/cookie',
        headers: {
            'Cookie': 'a=b',
        },
        body: {},
    })
    const response = await server.process(test_request)
    t.deepEqual(response.bodyObject(), {
        "cookies": [
            { "name": "a", "value": "b" }
        ]
    })
})