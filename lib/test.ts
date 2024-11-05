import Decimal from "decimal.js"
import { DateOnly } from ".."

export type KeyPath = (string | number)[]
export type Matcher = (path: KeyPath, value: any) => void

export class JSONMatchError extends Error {
    constructor(path: (string | number)[], found: any, reason?: string) {
        super(`${formatPath(path)}: ${reason || "value is invalid"}\nFound value: ${displayValue(found)}`)
    }
}

export function matchJson(value: any, matcher: any) {
    return () => matchJsonValue(value, matcher)
}

export function matchJsonValue(value: any, matcher: any) {
    matchJsonValuePathed([], value, matcher)
}

export function formatPath(path: KeyPath): string {
    if (path.length === 0) {
        return '(root)'
    }
    if (path.length === 1) {
        return `${path[0]}`
    }
    let result = `${path[0]}`
    for (let i = 1; i < path.length; i++) {
        const current = path[i]
        if (typeof current === 'number') {
            result += `[${current}]`
        } else {
            result += `.${current}`
        }
    }
    return result
}

export function pathAppend(path: KeyPath, next: string | number): KeyPath {
    const retval = [...path]
    retval.push(next)
    return retval
}

export function displayValue(value: any): string {
    return JSON.stringify(value)
}

export function displayMatcher(matcher: any): string {
    if (matcher === String) {
        return 'string'
    } else if (matcher === Number) {
        return 'number'
    } else if (matcher === Boolean) {
        return 'boolean'
    } else if (matcher === Array) {
        return 'array'
    } else {
        if (Array.isArray(matcher)) {
            return JSON.stringify(matcher)
        } else {
            return JSON.stringify(matcher)
        }
    }
}

export function matchJsonValuePathed(path: KeyPath, value: any, matcher: any) {
    if (typeof matcher === 'function') {
        (matcher as Matcher)(path, value)
    } else if (typeof value === 'string') {
        if (matcher === String) {
        } else if (typeof matcher === 'string') {
            if (value !== matcher) {
                throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found ${displayValue(value)}`)
            }
        } else {
            throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found '${displayValue(value)}'`)
        }
    } else if (typeof value === 'boolean') {
        if (matcher === Boolean) {
        } else if (typeof matcher === 'boolean') {
            if (value !== matcher) {
                throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found ${displayValue(value)}`)
            }
        } else {
            throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found '${displayValue(value)}'`)
        }
    } else if (typeof value === 'number') {
        if (matcher === Boolean) {
        } else if (typeof matcher === 'number') {
            if (value !== matcher) {
                throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found ${displayValue(value)}`)
            }
        } else {
            throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found '${displayValue(value)}'`)
        }
    } else if (value === null) {
        if (matcher !== null) {
            throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found ${displayValue(value)}`)
        }
    } else if (Array.isArray(value)) {
        if (matcher === Array) {
        } else if (Array.isArray(matcher)) {
            matchArray(path, value, matcher)
        } else {
            throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found ${displayValue(value)}`)
        }
    } else {
        if (typeof matcher === 'object' && !Array.isArray(matcher)) {
            matchObject(path, value, matcher, false)
        } else {
            throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found ${displayValue(value)}`)
        }
    }
}

function matchObject(path: KeyPath, value: {[key: string]: any}, matcher: {[key: string]: any}, partial: boolean) {
    const objectKeys = Object.keys(value)
    const matcherKeys = Object.keys(matcher)
    if (!partial) {
        objectKeys.forEach((k) => {
            if (!matcherKeys.includes(k)) {
                throw new JSONMatchError(path, value, `found extra key: ${k}`)
            }
        })
    }
    matcherKeys.forEach((k) => {
        if (!objectKeys.includes(k)) {
            throw new JSONMatchError(path, value, `missing key: ${k}`)
        }
    })
    matcherKeys.forEach((k) => {
        matchJsonValuePathed(pathAppend(path, k), value[k], matcher[k])
    })
}

function matchArray(path: KeyPath, value: any[], matcher: any[]) {
    if (value.length !== matcher.length) {
        throw new JSONMatchError(path, value, "array of wrong length")
    }
    for (let i = 0; i < value.length; i++) {
        matchJsonValuePathed(pathAppend(path, i), value[i], matcher[i])
    }
}

export function partial(matcher: any): Matcher {
    return (path, value) => {
        matchObject(path, value, matcher, true)
    }
}

export const ignore: Matcher = () => { }

export function dateValue(date: string | DateOnly): Matcher {
    return (path, value) => {
        if (typeof value !== 'object') {
            throw new JSONMatchError(path, value, 'date value should be object')
        }
        const keys = Object.keys(value)
        if (keys.length !== 1) {
            throw new JSONMatchError(path, value, 'date object should have 1 key')
        }
        if (keys[0] !== '$date') {
            throw new JSONMatchError(path, value, 'date object should have 1 `$date` key')
        }
        if (value['$date'] !== date.toString()) {
            throw new JSONMatchError(path, value, 'value not equal')
        }
    }
}

export function dateTimeValue(dateTime: string | Date | number): Matcher {
    return (path: (string | number)[], value: any) => {
        if (typeof value !== 'object') {
            throw new JSONMatchError(path, value, 'date time value should be object')
        }
        const keys = Object.keys(value)
        if (keys.length !== 1) {
            throw new JSONMatchError(path, value, 'date time object should have 1 key')
        }
        if (keys[0] !== '$datetime') {
            throw new JSONMatchError(path, value, 'date time object should have 1 `$datetime` key')
        }
        if (value['$datetime'] !== (new Date(dateTime)).toISOString()) {
            throw new JSONMatchError(path, value, 'value not equal')
        }
    }
}

export function decimalValue(decimal: number | Decimal | string): Matcher {
    return (path: (string | number)[], value: any) => {
        if (typeof value !== 'object') {
            throw new JSONMatchError(path, value, 'decimal value should be object')
        }
        const keys = Object.keys(value)
        if (keys.length !== 1) {
            throw new JSONMatchError(path, value, 'decimal object should have 1 key')
        }
        if (keys[0] !== '$decimal') {
            throw new JSONMatchError(path, value, 'decimal object should have 1 `$decimal` key')
        }
        if (value['$decimal'] !== decimal.toString()) {
            throw new JSONMatchError(path, value, 'value not equal')
        }
    }
}

export function objectIdValue(path: KeyPath, value: any) {
    if (typeof value !== 'string') {
        throw new JSONMatchError(path, value, 'object id value should be string')
    }
    if (!/^[0-9a-f]{24}$/.test(value)) {
        throw new JSONMatchError(path, value, 'invalid object id value')
    }
}

export function endsWith(suffix: string): Matcher {
    return (path, value) => {
        if (!value.endsWith(suffix)) {
            throw new JSONMatchError(path, value, `value doesn't end with '${suffix}'`)
        }
    }
}

export function combine(...matchers: Matcher[]): Matcher {
    return (path, value) => {
        matchers.forEach((matcher) => matcher(path, value))
    }
}

export function oneMatches(matcher: any): Matcher {
    return (path, value) => {
        if (!Array.isArray(value)) {
            throw new JSONMatchError(path, value, 'value is not array')
        }
        const len = value.length
        for (let i = 0; i < len; i++) {
            try {
                matchJsonValuePathed(pathAppend(path, i), value[i], matcher)
                return
            } catch(_) { }
        }
        throw new JSONMatchError(path, value, 'none of values matches matcher')
    }
}
