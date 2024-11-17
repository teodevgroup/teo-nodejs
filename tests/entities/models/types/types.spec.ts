import test from 'ava'
import fs from 'fs'
import path from 'path'
import { FormDataEncoder } from 'form-data-encoder'
import { Readable } from 'stream'
import { TestRequest, TestServer } from '../../../..'
import loadApp from './app'
import { ignore, matchJsonValue } from '../../../../test'

const server = new TestServer(loadApp())

test.before(async () => {
    await server.setup()
})

test.beforeEach(async () => {
    await server.reset()
})

test('create object', async (t) => {
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

test('find many objects', async (t) => {
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
        "data": [],
    }))
})
