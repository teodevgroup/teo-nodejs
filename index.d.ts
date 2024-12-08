/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface CookieCreateArgs {
  name: string
  value: string
  httpOnly?: boolean
  secure?: boolean
  sameSite?: string
  partitioned?: boolean
  maxAge?: number
  path?: string
  domain?: string
  expires?: Expiration
}
export declare class HandlerGroup {
  defineHandler(name: string, callback: (request: Request) => Response | Promise<Response>): void
}
export declare class Model {
  setData(key: string, value: any): void
  data(key: string): any
}
export declare class Field {
  setData(key: string, value: any): void
  data(key: string): any
}
export declare class Property {
  setData(key: string, value: any): void
  data(key: string): any
}
export declare class Relation {
  setData(key: string, value: any): void
  data(key: string): any
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
  definePipelineItem(name: string, creator: (args: {[key: string]: any}) => (ctx: PipelineCtx) => any | Promise<any>): void
  defineTransformPipelineItem(name: string, creator: (args: {[key: string]: any}) => (ctx: PipelineCtx) => any | Promise<any>): void
  defineValidatorPipelineItem(name: string, creator: (args: {[key: string]: any}) => (ctx: PipelineCtx) => string | boolean | undefined | null | Promise<string | boolean | undefined | null>): void
  defineCallbackPipelineItem(name: string, creator: (args: {[key: string]: any}) => (ctx: PipelineCtx) => string | boolean | undefined | null | Promise<string | boolean | undefined | null>): void
  defineComparePipelineItem(name: string, creator: (args: {[key: string]: any}) => (oldValue: any, newValue: any, ctx: PipelineCtx) => void | Promise<void>): void
  definePipelineItemFunction(name: string, item: (ctx: PipelineCtx) => any | Promise<any>): void
  defineTransformPipelineItemFunction(name: string, item: (ctx: PipelineCtx) => any | Promise<any>): void
  defineValidatorPipelineItemFunction(name: string, item: (ctx: PipelineCtx) => string | boolean | undefined | null | Promise<string | boolean | undefined | null>): void
  defineCallbackPipelineItemFunction(name: string, item: (ctx: PipelineCtx) => void | Promise<void>): void
  defineComparePipelineItemFunction(name: string, item: (oldValue: any, newValue: any, ctx: PipelineCtx) => string | boolean | undefined | null | Promise<string | boolean | undefined | null>): void

  defineHandler(name: string, callback: (request: Request) => Response | Promise<Response>): void
  defineHandlerGroup(name: string, callback: (group: HandlerGroup) => void): void
  defineModelHandlerGroup(name: string, callback: (group: HandlerGroup) => void): void
  defineRequestMiddleware(name: string, creator: (args: {[key: string]: any}) => (request: Request, next: (request: Request) => Promise<Response>) => Promise<Response> | Response): void
  defineHandlerMiddleware(name: string, creator: (args: {[key: string]: any}) => (request: Request, next: (request: Request) => Promise<Response>) => Promise<Response> | Response): void
}
export declare class DateOnly {
  constructor(string: string)
  toString(): string
}
export declare class ObjectId {
  constructor(value: string)
  toString(): string
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
  get upperbond(): number
  get lowerbond(): number
  get isClosed(): boolean
  get isOpen(): boolean
}
export declare class OptionVariant { }
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
  get main(): Namespace
}
export declare class Expiration {
  static sessionExpiration(): Expiration
  static datetimeExpiration(datetime: Date): Expiration
  get isSession(): boolean
  get isDatetime(): boolean
  get datetime(): Date | null
}
export declare class Cookie {
  constructor(args: CookieCreateArgs)
  get name(): string
  set name(name: string)
  get value(): string
  set value(value: string)
  get valueTrimmed(): string
  get httpOnly(): boolean | null
  set httpOnly(httpOnly: boolean | null | undefined)
  get secure(): boolean | null
  set secure(secure: boolean | null | undefined)
  get sameSite(): "strict" | "lax" | "none"
  set sameSite(sameSite: "strict" | "lax" | "none")
  get partitioned(): boolean | null
  set partitioned(partitioned: boolean | null | undefined)
  get maxAge(): number | null
  set maxAge(maxAge: number | null | undefined)
  get path(): string | null
  set path(path: string | null | undefined)
  get domain(): string | null
  set domain(domain: string | null | undefined)
  get expires(): Expiration | null
  set expires(expires: Expiration | null | undefined)
}
export declare class Cookies {
  constructor(cookies?: Array<Cookie> | undefined | null)
  get(key: string): Cookie | null
  has(key: string): boolean
  push(cookie: Cookie): void
  clear(): void
  get length(): number
  map<T>(callback: (cookie: Cookie) => T): T[]
}
export declare class Headers {
  append(key: string, value: string): void
  set(key: string, value: string): void
  get(key: string): string | null
  getAll(key: string): Array<string>
  delete(key: string): void
  keys(): Array<string>
  values(): Array<string>
  get length(): number
  has(key: string): boolean
}
export declare class HandlerMatch {
  get path(): Array<string>
  get handlerName(): string
  get captures(): {[key: string]: string} | any
}
export declare class Request {
  get version(): string
  set version(value: string)
  get method(): string
  set method(value: string)
  get uri(): string
  set uri(value: string)
  get scheme(): string | null
  get host(): string | null
  get path(): string
  get query(): string | null
  get contentType(): string | null
  get headers(): Headers
  set headers(headers: Headers)
  get cookies(): Cookies
  set cookies(cookies: Cookies)
  get handlerMatch(): HandlerMatch
  get captures(): {[key: string]: string} | any
  get bodyObject(): any
  set bodyObject(value: any)
  get teo(): any
  get localValues(): LocalValues
  get localObjects(): LocalObjects
}
export declare class LocalObjects {
  set(key: string, value: any): void
  get(key: string): any
  has(key: string): boolean
  remove(key: string): void
  clear(): void
}
export declare class LocalValues {
  set(key: string, value: any): void
  get(key: string): any
  has(key: string): boolean
  remove(key: string): void
  clear(): void
}
export declare class Response {
  static empty(): Response
  static string(content: string, contentType: string): Response
  static teon(value: any): Response
  static html(content: string): Response
  static data(value: any): Response
  static dataMeta(data: any, meta: any): Response
  static file(path: string): Response
  static sendFile(base: string, path: string): Response
  static redirect(path: string): Response
  set code(code: number)
  get code(): number
  get headers(): Headers
  set headers(headers: Headers)
  get isFile(): boolean
  get isText(): boolean
  get isEmpty(): boolean
  get isTeon(): boolean
  get text(): string | null
  get teon(): any | null
  get file(): string | null
  get cookies(): Cookies
  set cookies(cookies: Cookies)
}
export declare class EnumMember {
  setData(key: string, value: any): void
  data(key: string): any
}
export declare class Enum {
  setData(key: string, value: any): void
  data(key: string): any
}
export declare class Pipeline {
  get length(): number
}
export declare class PipelineCtx {
  get value(): any
  get object(): any
  get path(): object
  get teo(): object
  get request(): Request | null
}
export declare class TestServer {
  constructor(app: App)
  /** @internal */
  _setup_0(): Promise<void>
  /** @internal */
  _setup_1(): void
    /** Setup the server. */
    setup(): Promise<void>
  reset(): Promise<void>
  process(request: TestRequest): Promise<TestResponse>
}
export declare class TestRequest {
  constructor(props: { method?: string, uri: string, headers?: { [key: string]: string }, body?: any, cookies?: Cookie[] })
  get method(): string
  set method(method: string)
  get uri(): string
  set uri(uri: string)
  get headers(): Headers
  set headers(headers: Headers)
  insertHeader(key: string, value: string): this
  appendHeader(key: string, value: string): this
  get body(): Buffer
  set body(body: Buffer)
  get cookies(): Cookies
  set cookies(cookies: Cookies)
}
export declare class TestResponse {
  get status(): number
  get version(): string
  body(): Buffer
  bodyAsString(): string
  bodyAsJson(): any
  get headers(): Headers
  get cookies(): Cookies
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
