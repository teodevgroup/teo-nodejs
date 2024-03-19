const { readFileSync, writeFileSync } = require('fs')

function fixIndexJs(filename) {
  let content = readFileSync(filename).toString()

  let newContent = `
App.prototype.run = async function() {
  await this._prepare();
  return this._run();
}
ReadOnlyHeaderMap.prototype[customInspectSymbol] = function(_, inspectOptions) {
  let object = {}
  for (let k of this.keys()) {
    object[k] = this.get(k)
  }
  return "ReadOnlyHeaderMap " + inspect(object, inspectOptions)
}
HandlerMatch.prototype[customInspectSymbol] = function(_, inspectOptions) {
  return "HandlerMatch " + inspect({
    "path": this.path(),
    "handlerName": this.handlerName(),
    "captures": this.captures(),
  }, inspectOptions)
}
RequestCtx.prototype[customInspectSymbol] = function(_, inspectOptions) {
  return "RequestCtx " + inspect({
    "request": this.request(),
    "body": this.body(),
    "teo": this.teo(),
    "handlerMatch": this.handlerMatch(),
  }, inspectOptions)
}
Request.prototype[customInspectSymbol] = function(_, inspectOptions) {
  return "Request " + inspect({
    "method": this.method(),
    "path": this.path(),
    "queryString": this.queryString(),
    "contentType": this.contentType(),
    "headers": this.headers(),
  }, inspectOptions)
}
Response.prototype[customInspectSymbol] = function(_, inspectOptions) {
  return "Response " + inspect({
    "code": this.code(),
    "headers": this.headers(),
  }, inspectOptions)
}
ReadWriteHeaderMap.prototype[customInspectSymbol] = function(_, inspectOptions) {
  let object = {}
  for (let k of this.keys()) {
    object[k] = this.get(k)
  }
  return "ReadWriteHeaderMap " + inspect(object, inspectOptions)
}
DateOnly.prototype[customInspectSymbol] = function(_, inspectOptions) {
  return this.toString()
}
ObjectId.prototype[customInspectSymbol] = function(_, inspectOptions) {
  return "ObjectId(\\\"" + this.toString() + "\\\")"
}
Namespace.prototype.defineHandler = function(name, callback) {
  this._defineHandler(name, function(e, arg) {
    if (e != null) {
      throw e
    }  
    return callback(arg)
  })
}
HandlerGroup.prototype.defineHandler = function(name, callback) {
  this._defineHandler(name, function(e, arg) {
    if (e != null) {
      throw e
    }  
    return callback(arg)
  })
}
class TeoError extends Error {
  constructor(message, code = 500, errors = null) {
    super("")
    this.name = "TeoError"
    this._code = code
    this._errorMessage = message
    this._errors = errors
    this.message = this.buildMessage()
  }
  buildMessage() {
    return JSON.stringify({code: this.code, message: this.errorMessage, errors: this.errors })
  }
  set code(newValue) { 
    this._code = newValue 
    this.message = this.buildMessage()
  }
  get code() { return this._code }
  set errorMessage(newValue) {
    this._errorMessage = newValue
    this.message = this.buildMessage()
  } 
  get errorMessage() { return this._errorMessage }
  set errors(newValue) {
    this._errors = newValue
    this.message = this.buildMessage()
  }
  get errors() { return this._errors }
  messagePrefixed(prefix) {
    return new TeoError(this.code, this.errors ? this.errorMessage : prefix + ': ' + this.errorMessage, this.errors ? Object.fromEntries(
      Object.entries(this.errors).map(([key, value]) => [key, prefix + ": " + value)])
    ) : null)
  }
  pathPrefixed(prefix) {
    return new TeoError(this.code, this.errorMessage, this.errors ? Object.fromEntries(
      Object.entries(this.errors).map(([key, value]) => [prefix + "." + key, value)])
    ) : null)
  }
  mapPath(mapper) {
    return new TeoError(this.code, this.errorMessage, this.errors ? Object.fromEntries(
      Object.entries(this.errors).map(([key, value]) => [mapper(key), value)])
    ) : null)    
  }
}
TeoError.notFound = (message = "not found") => new TeoError(message, 404)
TeoError.invalidRequest = (message = "value is invalid") => new TeoError(message, 400)
TeoError.internalServerError = (message = "internal server error") => new TeoError(message, 500)
TeoError.unauthorized = (message = "unauthorized") => new TeoError(message, 401)
module.exports.TeoError = TeoError

globalThis.require = require
process.on('SIGINT', function() { process.exit(0) })
`
  content += newContent
  content = content.replace("const { join } = require('path')", `const { join } = require('path')
const { inspect } = require('util')
const customInspectSymbol = Symbol.for('nodejs.util.inspect.custom')`)
  writeFileSync(filename, content)
}

function fixIndexDTs(filename) {
  let content = readFileSync(filename).toString()
  content = content.replace("_run(): Promise<void>", `_run(): Promise<void>
  /** Run this app. */
  run(): Promise<void>`).replaceAll("_defineHandler", "defineHandler")
  content += `export class TeoError extends Error {
  constructor(message: string, code: number = 500, errors: { [key: string]: string } | null = null)
  public get code(): number
  public get errorMessage(): string
  public get errors(): { [key: string]: string } | null
  public messagePrefixed(prefix: string): TeoError
  public pathPrefixed(prefix: string): TeoError
  public mapPath(mapper: (string) => string): TeoError
  static public notFound(message: string = "not found"): TeoError
  static public invalidRequest(message: string = "value is invalid"): TeoError
  static public internalServerError(message: string = "internal server error"): TeoError
  static public unauthorized(message: string = "unauthorized"): TeoError  
}
`
  writeFileSync(filename, content)
}


let filename = process.argv[process.argv.length - 1]
if (filename.endsWith('index.js')) {
  fixIndexJs(filename)
} else if (filename.endsWith('index.d.ts')) {
  fixIndexDTs(filename)
}
