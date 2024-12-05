import test from 'ava'
// import { TestRequest, TestServer } from '../../..'
// import loadApp from './app'
// import { dateTimeValue, dateValue, decimalValue, ignore, matchJsonValue, partial } from '../../../test'

// const server = new TestServer(loadApp())

// test.before(async () => {
//     await server.setup()
// })

// test.beforeEach(async () => {
//     await server.reset()
// })

// test('transform int32', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'int32': 1
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "int32": 10
//         }),
//     }))
// })

// test('transform int64', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'int64': 1
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "int64": 10
//         }),
//     }))
// })

// test('transform float32', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'float32': 1.0
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "float32": 10.0
//         }),
//     }))
// })

// test('transform float64', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'float64': 1.0
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "float64": 10.0
//         }),
//     }))
// })

// test('transform bool', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'bool': false
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "bool": true
//         }),
//     }))
// })

// test('transform string', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'string': 'Love'
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "string": '*Love*'
//         }),
//     }))
// })

// test('transform date', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'date': '2005-06-01'
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "date": dateValue('2005-06-02')
//         }),
//     }))
// })

// test('transform date time', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'dateTime': '2024-11-29T14:49:13.498Z'
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "dateTime": dateTimeValue('2024-11-30T14:49:13.498Z')
//         }),
//     }))
// })

// test('transform decimal', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'decimal': '1'
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "decimal": decimalValue('10')
//         }),
//     }))
// })

// test('transform status', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'status': 'open'
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "status": 'pending'
//         }),
//     }))
// })

// test('transform int32 array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'int32Array': [1, 1]
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "int32Array": [10, 10]
//         }),
//     }))
// })

// test('transform int64 array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'int64Array': [1, 1]
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "int64Array": [10, 10]
//         }),
//     }))
// })

// test('transform float32 array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'float32Array': [1.0, 1.0]
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "float32Array": [10.0, 10.0]
//         }),
//     }))
// })

// test('transform float64 array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'float64Array': [1.0, 1.0]
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "float64Array": [10.0, 10.0]
//         }),
//     }))
// })

// test('transform bool array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'boolArray': [false, false]
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "boolArray": [true, true]
//         }),
//     }))
// })

// test('transform string array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'stringArray': ['Love', 'Love']
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "stringArray": ['*Love*', '*Love*']
//         }),
//     }))
// })

// test('transform date array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'dateArray': ['2005-06-01', '2005-06-01']
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "dateArray": [dateValue('2005-06-02'), dateValue('2005-06-02')]
//         }),
//     }))
// })

// test('transform date time array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'dateTimeArray': ['2024-11-29T14:49:13.498Z', '2024-11-29T14:49:13.498Z']
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "dateTimeArray": [dateTimeValue('2024-11-30T14:49:13.498Z'), dateTimeValue('2024-11-30T14:49:13.498Z')]
//         }),
//     }))
// })

// test('transform decimal array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'decimalArray': ['1', '1']
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "decimalArray": [decimalValue('10'), decimalValue('10')]
//         }),
//     }))
// })

// test('transform status array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'statusArray': ['open', 'open']
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "statusArray": ['pending', 'pending']
//         }),
//     }))
// })

// test('alter int32', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'int32': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "int32": 5
//         }),
//     }))
// })

// test('alter int64', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'int64': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "int64": 5
//         }),
//     }))
// })

// test('alter float32', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'float32': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "float32": 5.5
//         }),
//     }))
// })

// test('alter float64', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'float64': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "float64": 5.5
//         }),
//     }))
// })

// test('alter bool', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'bool': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "bool": true
//         }),
//     }))
// })

// test('alter string', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'string': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "string": 'Flower'
//         }),
//     }))
// })

// test('alter date', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'date': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "date": dateValue('2003-06-23')
//         }),
//     }))
// })

// test('alter date time', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'dateTime': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "dateTime": dateTimeValue('2004-07-23T11:30:00.000Z')
//         }),
//     }))
// })

// test('alter decimal', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'decimal': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "decimal": decimalValue('5')
//         }),
//     }))
// })

// test('alter status', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'status': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "status": 'done'
//         }),
//     }))
// })

// test('alter int32 array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'int32Array': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "int32Array": [5, 5, 5, 5]
//         }),
//     }))
// })

// test('alter int64 array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'int64Array': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "int64Array": [5, 5, 5, 5]
//         }),
//     }))
// })

// test('alter float32 array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'float32Array': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "float32Array": [5.5, 5.5, 5.5, 5.5]
//         }),
//     }))
// })

// test('alter float64 array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'float64Array': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "float64Array": [5.5, 5.5, 5.5, 5.5]
//         }),
//     }))
// })

// test('alter bool array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'boolArray': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "boolArray": [true, true, true, true]
//         }),
//     }))
// })

// test('alter string array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'stringArray': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "stringArray": ['Flower', 'Flower', 'Flower', 'Flower']
//         }),
//     }))
// })

// test('alter date array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'dateArray': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "dateArray": [dateValue('2003-06-23'), dateValue('2003-06-23')]
//         }),
//     }))
// })

// test('alter date time array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'dateTimeArray': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "dateTimeArray": [dateTimeValue('2004-07-23T11:30:00.000Z'), dateTimeValue('2004-07-23T11:30:00.000Z')]
//         }),
//     }))
// })

// test('alter decimal array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'decimalArray': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "decimalArray": [decimalValue('5'), decimalValue('5'), decimalValue('5'), decimalValue('5')]
//         }),
//     }))
// })

// test('alter status array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'statusArray': null
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "statusArray": ['done', 'done', 'done', 'done']
//         }),
//     }))
// })

// test('callback int32', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'int32': 1
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "1"
//         }),
//     }))
// })

// test('callback int64', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'int64': 1
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "1"
//         }),
//     }))
// })

// test('callback float32', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'float32': 1.0
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "1.0"
//         }),
//     }))
// })

// test('callback float64', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'float64': 1.0
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "1.0"
//         }),
//     }))
// })

// test('callback bool', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'bool': false
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "False"
//         }),
//     }))
// })

// test('callback string', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'string': 'Love'
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "Love"
//         }),
//     }))
// })

// test('callback date', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'date': '2005-06-01'
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "2005-06-01"
//         }),
//     }))
// })

// test('callback date time', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'dateTime': '2024-11-29T14:49:13.498Z'
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "2024-11-29 14:49:13.498000+00:00"
//         }),
//     }))
// })

// test('callback decimal', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'decimal': '1'
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "1"
//         }),
//     }))
// })

// test('callback status', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'status': 'open'
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "open"
//         }),
//     }))
// })

// test('callback int32 array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'int32Array': [1, 1]
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "[1, 1]"
//         }),
//     }))
// })

// test('callback int64 array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'int64Array': [1, 1]
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "[1, 1]"
//         }),
//     }))
// })

// test('callback float32 array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'float32Array': [1.0, 1.0]
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "[1.0, 1.0]"
//         }),
//     }))
// })

// test('callback float64 array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'float64Array': [1.0, 1.0]
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "[1.0, 1.0]"
//         }),
//     }))
// })

// test('callback bool array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'boolArray': [false, false]
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "[False, False]"
//         }),
//     }))
// })

// test('callback string array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'stringArray': ['Love', 'Love']
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "[Love, Love]"
//         }),
//     }))
// })

// test('callback date array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'dateArray': ['2005-06-01', '2005-06-01']
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "[2005-06-01, 2005-06-01]"
//         }),
//     }))
// })

// test('callback date time array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'dateTimeArray': ['2024-11-29T14:49:13.498Z', '2024-11-29T14:49:13.498Z']
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "[2024-11-29 14:49:13.498000+00:00, 2024-11-29 14:49:13.498000+00:00]"
//         }),
//     }))
// })

// test('callback decimal array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'decimalArray': ['1', '1']
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "[1, 1]"
//         }),
//     }))
// })

// test('callback status array', async (t) => {
//     const test_request = new TestRequest({
//         method: 'POST',
//         uri: '/Container/create',
//         body: {
//             'create': {
//                 'statusArray': ['open', 'open']
//             }
//         }
//     })
//     const response = await server.process(test_request)
//     t.notThrows(() => matchJsonValue(response.bodyAsJson(), {
//         "data": partial({
//             "id": ignore,
//             "message": "[open, open]"
//         }),
//     }))
// })

test.serial("placeholder", (t) => {
    t.is(true, true)
})
