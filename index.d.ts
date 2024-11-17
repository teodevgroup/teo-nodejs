/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export declare class HandlerGroup {
  defineHandler(name: string, callback: (request: Request) => Response | Promise<Response>): void
}
export declare class Model {
  setData(key: string, value: unknown): void
  data(key: string): unknown
}
export declare class Field {
  setData(key: string, value: unknown): void
  data(key: string): unknown
}
export declare class Property {
  setData(key: string, value: unknown): void
  data(key: string): unknown
}
export declare class Relation {
  setData(key: string, value: unknown): void
  data(key: string): unknown
}
export declare class Namespace {
  isMain(): boolean
  isStd(): boolean
  path(): Array<string>
  namespace(name: string): Namespace | null
  namespaceOrCreate(name: string): Namespace
  namespaceAtPath(path: Array<string>): Namespace | null
  namespaceOrCreateAtPath(path: Array<string>): Namespace
  defineModelDecorator(name: string, body: (args: {[key: string]: any}, model: Model) => void): void
  defineModelFieldDecorator(name: string, body: (args: {[key: string]: any}, field: Field) => void): void
  defineModelRelationDecorator(name: string, body: (args: {[key: string]: any}, relation: Relation) => void): void
  defineModelPropertyDecorator(name: string, body: (args: {[key: string]: any}, property: Property) => void): void
  defineEnumDecorator(name: string, body: (args: {[key: string]: any}, e: Enum) => void): void
  defineEnumMemberDecorator(name: string, body: (args: {[key: string]: any}, member: EnumMember) => void): void
  definePipelineItem(name: string, body: (input: any, args: {[key: string]: any}, object: any, teo: any) => any | Promise<any>): void
  defineTransformPipelineItem(name: string, callback: (input: any, args: {[key: string]: any}, object: any, teo: any) => any | Promise<any>): void
  defineValidatorPipelineItem(name: string, callback: (input: any, args: {[key: string]: any}, object: any, teo: any) => boolean | string | undefined | null | Promise<boolean | string | undefined | null>): void
  /** Register a named callback. */
  defineCallbackPipelineItem(name: string, callback: (input: any, args: {[key: string]: any}, object: any, teo: any) => void | Promise<void>): void
  defineComparePipelineItem<T>(name: string, callback: (oldValue: T, newValue: T, args: {[key: string]: any}, object: any, teo: any) => boolean | string | undefined | null | Promise<boolean | string | undefined | null>): void
  defineHandler(name: string, callback: (request: Request) => Response | Promise<Response>): void
  defineHandlerGroup(name: string, callback: (group: HandlerGroup) => void): void
  defineModelHandlerGroup(name: string, callback: (group: HandlerGroup) => void): void
  defineRequestMiddleware(name: string, callback: (args: {[key: string]: any}) => (request: Request, next: (request: Request) => Promise<Response>) => Promise<Response> | Response): void
  defineHandlerMiddleware(name: string, callback: (args: {[key: string]: any}) => (request: Request, next: (request: Request) => Promise<Response>) => Promise<Response> | Response): void
}
export declare class DateOnly {
  toString(): string
  static fromString(string: string): unknown
}
export declare class ObjectId {
  toString(): string
  static fromString(string: string): unknown
}
/**
 * File
 * File only represent input file in form request.
 */
export declare class File {
  filepath: string
  contentType?: string
  filename: string
  filenameExt?: string
}
export declare class Range {
  upperbond(): number
  lowerbond(): number
  isClosed(): boolean
  isOpen(): boolean
}
export declare class OptionVariant { }
export declare class Pipeline { }
export declare class InterfaceEnumVariant { }
export declare class App {
  /** Create a Teo app. */
  constructor(argv?: Array<string> | undefined | null)
  /** @internal */
  static withCli(cli: boolean, argv?: Array<string> | undefined | null): App
  /** @internal */
  _prepare(): Promise<void>
  /** @internal */
  _run(): Promise<void>
  /** Run this app. */
  run(): Promise<void>
  /** Run before server is started. */
  setup(callback: (ctx: any) => void | Promise<void>): void
  /** Define a custom program. */
  program(name: string, desc: string | undefined, callback: (ctx: any) => void | Promise<void>): void
  mainNamespace(): Namespace
}
export declare class HandlerMatch {
  path(): Array<string>
  handlerName(): string
  captures(): {[key: string]: string}
}
export declare class Request {
  version(): string
  method(): string
  uri(): string
  scheme(): string | null
  host(): string | null
  path(): string
  query(): string | null
  contentType(): string | null
  containsHeader(name: string): boolean
  headerValue(name: string): string | null
  headerValues(name: string): Array<string>
  headerKeys(): string[]
  headersLength(): number
  cookie(name: string): Cookie | null
  cookies(): Array<Cookie>
  handlerMatch(): HandlerMatch
  captures(): {[key: string]: string}
  bodyObject(): any
  setBodyObject(value: unknown): void
  teo(): any
}
export declare class Expiration {
  static createSession(): Expiration
  static createDatetime(datetime: Date): Expiration
  isSession(): boolean
  isDatetime(): boolean
  datetime(): Date | null
}
export declare class Cookie {
  constructor(name: string, value: string)
  name(): string
  value(): string
  expires(): Expiration | null
  maxAge(): number | null
  domain(): string | null
  path(): string | null
  secure(): boolean | null
  httpOnly(): boolean | null
  setSameSite(sameSite: "strict" | "lax" | "none"): void
  sameSite(): "strict" | "lax" | "none"
  toString(): string
  setMaxAge(maxAge: number): void
  setExpires(expires: Expiration): void
  setDomain(domain: string): void
  setPath(path: string): void
  setSecure(secure: boolean): void
  setHttpOnly(httpOnly: boolean): void
  setName(name: string): void
  setValue(value: string): void
  makeRemoval(): void
  makePermanent(): void
  static fromString(string: string): Cookie
}
export declare class ReadWriteHeaderMap {
  keys(): Array<string>
  len(): number
  containsKey(key: string): boolean
  get(key: string): string | null
  set(key: string, value: string): void
}
export declare class Response {
  static empty(): Response
  static string(content: string, contentType: string): Response
  static teon(value: unknown): Response
  static html(content: string): Response
  static data(value: unknown): Response
  static dataMeta(data: unknown, meta: unknown): Response
  static file(path: string): Response
  static redirect(path: string): Response
  setCode(code: number): void
  code(): number
  headers(): ReadWriteHeaderMap
  isFile(): boolean
  isText(): boolean
  isEmpty(): boolean
  isTeon(): boolean
  getText(): string | null
  getTeon(): unknown
  getFile(): string | null
  addCookie(cookie: Cookie): void
  cookies(): Array<Cookie>
}
export declare class EnumMember {
  setData(key: string, value: unknown): void
  data(key: string): unknown
}
export declare class Enum {
  setData(key: string, value: unknown): void
  data(key: string): unknown
}
export declare class TestServer {
  constructor(app: App)
  setup(): Promise<void>
  reset(): Promise<void>
  process(request: TestRequest): Promise<TestResponse>
}
export declare class TestRequest {
  constructor(props: { method?: string, uri: string, headers?: { [key: string]: string }, body?: any })
  method(): string
  setMethod(method: string): void
  uri(): string
  setUri(uri: string): void
  insertHeader(key: string, value: string): void
  appendHeader(key: string, value: string): void
  body(): Buffer
  setBody(body: Buffer): void
}
export declare class TestResponse {
  status(): number
  version(): string
  body(): Buffer
  bodyAsString(): string
  bodyAsJson(): any
  containsHeader(name: string): boolean
  headerValue(name: string): string | null
  headerValues(name: string): Array<string>
  headerKeys(): string[]
  headersLength(): number
}
export class TeoError extends Error {
  constructor(message: string, code?: number, errors?: { [key: string]: string } | null)
  public get code(): number
  public get errorMessage(): string
  public get errors(): { [key: string]: string } | null
  public messagePrefixed(prefix: string): TeoError
  public pathPrefixed(prefix: string): TeoError
  public mapPath(mapper: (string) => string): TeoError
  public static notFound(message?: string): TeoError
  public static invalidRequest(message?: string): TeoError
  public static internalServerError(message?: string): TeoError
  public static unauthorized(message?: string): TeoError  
}
