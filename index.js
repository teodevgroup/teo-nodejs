/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')
const { inspect } = require('util')
const customInspectSymbol = Symbol.for('nodejs.util.inspect.custom')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'teo.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./teo.android-arm64.node')
          } else {
            nativeBinding = require('@teodevgroup/teo-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'teo.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./teo.android-arm-eabi.node')
          } else {
            nativeBinding = require('@teodevgroup/teo-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'teo.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./teo.win32-x64-msvc.node')
          } else {
            nativeBinding = require('@teodevgroup/teo-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'teo.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./teo.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('@teodevgroup/teo-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'teo.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./teo.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('@teodevgroup/teo-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    localFileExisted = existsSync(join(__dirname, 'teo.darwin-universal.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./teo.darwin-universal.node')
      } else {
        nativeBinding = require('@teodevgroup/teo-darwin-universal')
      }
      break
    } catch {}
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'teo.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./teo.darwin-x64.node')
          } else {
            nativeBinding = require('@teodevgroup/teo-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'teo.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./teo.darwin-arm64.node')
          } else {
            nativeBinding = require('@teodevgroup/teo-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'teo.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./teo.freebsd-x64.node')
      } else {
        nativeBinding = require('@teodevgroup/teo-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'teo.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./teo.linux-x64-musl.node')
            } else {
              nativeBinding = require('@teodevgroup/teo-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'teo.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./teo.linux-x64-gnu.node')
            } else {
              nativeBinding = require('@teodevgroup/teo-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'teo.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./teo.linux-arm64-musl.node')
            } else {
              nativeBinding = require('@teodevgroup/teo-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'teo.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./teo.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('@teodevgroup/teo-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'teo.linux-arm-musleabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./teo.linux-arm-musleabihf.node')
            } else {
              nativeBinding = require('@teodevgroup/teo-linux-arm-musleabihf')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'teo.linux-arm-gnueabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./teo.linux-arm-gnueabihf.node')
            } else {
              nativeBinding = require('@teodevgroup/teo-linux-arm-gnueabihf')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'riscv64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'teo.linux-riscv64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./teo.linux-riscv64-musl.node')
            } else {
              nativeBinding = require('@teodevgroup/teo-linux-riscv64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'teo.linux-riscv64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./teo.linux-riscv64-gnu.node')
            } else {
              nativeBinding = require('@teodevgroup/teo-linux-riscv64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 's390x':
        localFileExisted = existsSync(
          join(__dirname, 'teo.linux-s390x-gnu.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./teo.linux-s390x-gnu.node')
          } else {
            nativeBinding = require('@teodevgroup/teo-linux-s390x-gnu')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { HandlerGroup, Model, Field, Property, Relation, Namespace, DateOnly, ObjectId, File, Range, OptionVariant, InterfaceEnumVariant, App, Expiration, Cookie, Cookies, Headers, HandlerMatch, Request, LocalObjects, LocalValues, Response, EnumMember, Enum, Pipeline, PipelineCtx, TestServer, TestRequest, TestResponse } = nativeBinding

module.exports.HandlerGroup = HandlerGroup
module.exports.Model = Model
module.exports.Field = Field
module.exports.Property = Property
module.exports.Relation = Relation
module.exports.Namespace = Namespace
module.exports.DateOnly = DateOnly
module.exports.ObjectId = ObjectId
module.exports.File = File
module.exports.Range = Range
module.exports.OptionVariant = OptionVariant
module.exports.InterfaceEnumVariant = InterfaceEnumVariant
module.exports.App = App
module.exports.Expiration = Expiration
module.exports.Cookie = Cookie
module.exports.Cookies = Cookies
module.exports.Headers = Headers
module.exports.HandlerMatch = HandlerMatch
module.exports.Request = Request
module.exports.LocalObjects = LocalObjects
module.exports.LocalValues = LocalValues
module.exports.Response = Response
module.exports.EnumMember = EnumMember
module.exports.Enum = Enum
module.exports.Pipeline = Pipeline
module.exports.PipelineCtx = PipelineCtx
module.exports.TestServer = TestServer
module.exports.TestRequest = TestRequest
module.exports.TestResponse = TestResponse

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
  return "ObjectId(\"" + this.toString() + "\")"
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
