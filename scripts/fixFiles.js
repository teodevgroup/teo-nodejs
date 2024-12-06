const { readFileSync, writeFileSync } = require('fs')

function fixIndexJs(filename) {
  let content = readFileSync(filename).toString()

  let newContent = `
App.prototype.run = async function() {
  await this._prepare();
  return this._run();
}
HandlerMatch.prototype[customInspectSymbol] = function(_, inspectOptions) {
  return "HandlerMatch " + inspect({
    "path": this.path,
    "handlerName": this.handlerName,
    "captures": this.captures,
  }, inspectOptions)
}
Request.prototype[customInspectSymbol] = function(_, inspectOptions) {
  return "Request " + inspect({
    "method": this.method,
    "path": this.path,
    "query": this.query,
    "contentType": this.contentType,
    "headers": this.headers,
    "bodyObject": this.bodyObject,
    "teo": this.teo,
    "handlerMatch": this.handlerMatch,
  }, inspectOptions)
}
Response.prototype[customInspectSymbol] = function(_, inspectOptions) {
  return "Response " + inspect({
    "code": this.code(),
    "headers": this.headers(),
  }, inspectOptions)
}
Headers.prototype[customInspectSymbol] = function(_, inspectOptions) {
  let object = {}
  for (let k of this.keys()) {
    object[k] = this.get(k)
  }
  return "Headers " + inspect(object, inspectOptions)
}
DateOnly.prototype[customInspectSymbol] = function(_, inspectOptions) {
  return this.toString()
}
ObjectId.prototype[customInspectSymbol] = function(_, inspectOptions) {
  return "ObjectId(\\\"" + this.toString() + "\\\")"
}
Namespace.prototype.definePipelineItem = function(name, creator) {
  this._definePipelineItem(name, function(e, args) {
    if (e != null) {
      throw e
    }
    const item = creator(args)
    return function (e, ctx) {
      if (e != null) {
        throw e
      }
      return item(ctx)
    }
  })
}
Namespace.prototype.defineTransformPipelineItem = Namespace.prototype.definePipelineItem
Namespace.prototype.defineValidatorPipelineItem = function(name, creator) {
  this._defineValidatorPipelineItem(name, function(e, args) {
    if (e != null) {
      throw e
    }
    const item = creator(args)
    return function (e, ctx) {
      if (e != null) {
        throw e
      }
      return item(ctx)
    }
  })
}
Namespace.prototype.defineCallbackPipelineItem = function(name, creator) {
  this._defineCallbackPipelineItem(name, function(e, args) {
    if (e != null) {
      throw e
    }
    const item = creator(args)
    return function (e, ctx) {
      if (e != null) {
        throw e
      }
      return item(ctx)
    }
  })
}
Namespace.prototype.defineComparePipelineItem = function(name, creator) {
  this._defineComparePipelineItem(name, function(e, args) {
    if (e != null) {
      throw e
    }
    const item = creator(args)
    return function (e, oldValue, newValue, ctx) {
      if (e != null) {
        throw e
      }
      return item(oldValue, newValue, ctx)
    }
  })
}
Namespace.prototype.definePipelineItemFunction = function(name, item) {
  this.definePipelineItem(name, () => item)
}
Namespace.prototype.defineTransformPipelineItemFunction = Namespace.prototype.definePipelineItemFunction
Namespace.prototype.defineValidatorPipelineItemFunction = function(name, item) {
  this.defineValidatorPipelineItem(name, () => item)
}
Namespace.prototype.defineCallbackPipelineItemFunction = function(name, item) {
  this.defineCallbackPipelineItem(name, () => item)
}
Namespace.prototype.defineComparePipelineItemFunction = function(name, item) {
  this.defineComparePipelineItem(name, () => item)
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
TestServer.prototype.setup = async function() {
  await this._setup_0()
  this._setup_1()
}
Object.defineProperty(Response.prototype, 'text', {
  get() {
      return this.getText()
  }
})
Object.defineProperty(Response.prototype, 'teon', {
  get() {
      return this.getTeon()
  }
})
Object.defineProperty(Response.prototype, 'file', {
  get() {
      return this.getFile()
  }
})
App.prototype.setup = function(callback) {
  this._setup(function(e, teo) {
    if (e != null) {
      throw e
    }
    return callback(teo)
  })
}
App.prototype.program = function(callback) {
  this._program(function(e, teo) {
    if (e != null) {
      throw e
    }
    return callback(teo)
  })
}
Namespace.prototype.defineModelDecorator = function(name, callback) {
  this._defineModelDecorator(function(e, arg) {
    if (e != null) {
      throw e
    }
    return callback(arg)
  })
}
Namespace.prototype.defineModelFieldDecorator = function(name, callback) {
  this._defineModelFieldDecorator(function(e, arg) {
    if (e != null) {
      throw e
    }
    return callback(arg)
  })
}
Namespace.prototype.defineModelRelationDecorator = function(name, callback) {
  this._defineModelRelationDecorator(function(e, arg) {
    if (e != null) {
      throw e
    }
    return callback(arg)
  })
}
Namespace.prototype.defineModelPropertyDecorator = function(name, callback) {
  this._defineModelPropertyDecorator(function(e, arg) {
    if (e != null) {
      throw e
    }
    return callback(arg)
  })
}
Namespace.prototype.defineEnumDecorator = function(name, callback) {
  this._defineEnumDecorator(function(e, arg) {
    if (e != null) {
      throw e
    }
    return callback(arg)
  })
}
Namespace.prototype.defineEnumMemberDecorator = function(name, callback) {
  this._defineEnumMemberDecorator(function(e, arg) {
    if (e != null) {
      throw e
    }
    return callback(arg)
  })
}
Namespace.prototype.defineHandlerGroup = function(name, callback) {
  this._defineHandlerGroup(function(e, arg) {
    if (e != null) {
      throw e
    }
    return callback(arg)
  })
}
Namespace.prototype.defineModelHandlerGroup = function(name, callback) {
  this._defineModelHandlerGroup(function(e, arg) {
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
    return new TeoError(this.errors ? this.errorMessage : prefix + ': ' + this.errorMessage, this.code, this.errors ? Object.fromEntries(
      Object.entries(this.errors).map(([key, value]) => [key, prefix + ": " + value])
    ) : null)
  }
  pathPrefixed(prefix) {
    return new TeoError(this.errorMessage, this.code, this.errors ? Object.fromEntries(
      Object.entries(this.errors).map(([key, value]) => [prefix + "." + key, value])
    ) : null)
  }
  mapPath(mapper) {
    return new TeoError(this.errorMessage, this.code, this.errors ? Object.fromEntries(
      Object.entries(this.errors).map(([key, value]) => [mapper(key), value])
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
  run(): Promise<void>`)
  .replace(`  getText(): string | null
  getTeon(): any | null
  getFile(): string | null`, `  get text(): string | null
  get teon(): any | null
  get file(): string | null`)
  .replace("map(callback: (cookie: Cookie) => T): T[]", "map<T>(callback: (cookie: Cookie) => T): T[]")
  .replace("_setup", "setup")
  .replace("_program", "program")
  .replace("_defineModelDecorator", "defineModelDecorator")
  .replace("_defineModelFieldDecorator", "defineModelFieldDecorator")
  .replace("_defineModelRelationDecorator", "defineModelRelationDecorator")
  .replace("_defineModelPropertyDecorator", "defineModelPropertyDecorator")
  .replace("_defineEnumDecorator", "defineEnumDecorator")
  .replace("_defineEnumMemberDecorator", "defineEnumMemberDecorator")
  .replaceAll("_defineHandler", "defineHandler")
  .replaceAll("_definePipelineItem", "definePipelineItem")
  .replaceAll("_defineTransformPipelineItem", "defineTransformPipelineItem")
  .replaceAll("_defineValidatorPipelineItem", "defineValidatorPipelineItem")
  .replaceAll("_defineCallbackPipelineItem", "defineCallbackPipelineItem")
  .replaceAll("_defineComparePipelineItem", "defineComparePipelineItem")
  .replace("  defineComparePipelineItem(name: string, creator: (args: {[key: string]: any}) => (oldValue: any, newValue: any, ctx: PipelineCtx) => void | Promise<void>): void", `  defineComparePipelineItem(name: string, creator: (args: {[key: string]: any}) => (oldValue: any, newValue: any, ctx: PipelineCtx) => void | Promise<void>): void
  definePipelineItemFunction(name: string, item: (ctx: PipelineCtx) => any | Promise<any>): void
  defineTransformPipelineItemFunction(name: string, item: (ctx: PipelineCtx) => any | Promise<any>): void
  defineValidatorPipelineItemFunction(name: string, item: (ctx: PipelineCtx) => string | boolean | undefined | null | Promise<string | boolean | undefined | null>): void
  defineCallbackPipelineItemFunction(name: string, item: (ctx: PipelineCtx) => void | Promise<void>): void
  defineComparePipelineItemFunction(name: string, item: (oldValue: any, newValue: any, ctx: PipelineCtx) => string | boolean | undefined | null | Promise<string | boolean | undefined | null>): void
`)
  .replace("_setup_1(): void", `_setup_1(): void
    /** Setup the server. */
    setup(): Promise<void>`)
  content += `export class TeoError extends Error {
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
`
  writeFileSync(filename, content)
}


let filename = process.argv[process.argv.length - 1]
if (filename.endsWith('index.js')) {
  fixIndexJs(filename)
} else if (filename.endsWith('index.d.ts')) {
  fixIndexDTs(filename)
}
