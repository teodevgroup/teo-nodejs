import { TestRequest, TestServer } from "../.."

const builtinReq = async (server: TestServer, action: string, model: string, data: any) => {
    const request = new TestRequest({
        method: 'POST',
        uri: `/${model}/${action}`,
        body: data
    })
    const response = await server.process(request)
    return response.bodyAsJson()
}

export default builtinReq