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
  constructor(message, code = 500) {
    super(message)
    this.code = code
  }
}
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
  constructor(message: string, code?: number)
  public title?: string
  public code?: number
  public errors?: { [key: string]: string }
  public prefixes?: string[]
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
