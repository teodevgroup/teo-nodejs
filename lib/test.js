"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ignore = exports.JSONMatchError = void 0;
exports.matchJson = matchJson;
exports.matchJsonValue = matchJsonValue;
exports.formatPath = formatPath;
exports.pathAppend = pathAppend;
exports.displayValue = displayValue;
exports.displayMatcher = displayMatcher;
exports.matchJsonValuePathed = matchJsonValuePathed;
exports.partial = partial;
exports.dateValue = dateValue;
exports.dateTimeValue = dateTimeValue;
exports.decimalValue = decimalValue;
exports.objectIdValue = objectIdValue;
exports.endsWith = endsWith;
exports.combine = combine;
exports.oneMatches = oneMatches;
class JSONMatchError extends Error {
    constructor(path, found, reason) {
        super(`${formatPath(path)}: ${reason || "value is invalid"}\nFound value: ${displayValue(found)}`);
    }
}
exports.JSONMatchError = JSONMatchError;
function matchJson(value, matcher) {
    return () => matchJsonValue(value, matcher);
}
function matchJsonValue(value, matcher) {
    matchJsonValuePathed([], value, matcher);
}
function formatPath(path) {
    if (path.length === 0) {
        return '(root)';
    }
    if (path.length === 1) {
        return `${path[0]}`;
    }
    let result = `${path[0]}`;
    for (let i = 1; i < path.length; i++) {
        const current = path[i];
        if (typeof current === 'number') {
            result += `[${current}]`;
        }
        else {
            result += `.${current}`;
        }
    }
    return result;
}
function pathAppend(path, next) {
    const retval = [...path];
    retval.push(next);
    return retval;
}
function displayValue(value) {
    return JSON.stringify(value);
}
function displayMatcher(matcher) {
    if (matcher === String) {
        return 'string';
    }
    else if (matcher === Number) {
        return 'number';
    }
    else if (matcher === Boolean) {
        return 'boolean';
    }
    else if (matcher === Array) {
        return 'array';
    }
    else {
        if (Array.isArray(matcher)) {
            return JSON.stringify(matcher);
        }
        else {
            return JSON.stringify(matcher);
        }
    }
}
function matchJsonValuePathed(path, value, matcher) {
    if (typeof matcher === 'function') {
        matcher(path, value);
    }
    else if (typeof value === 'string') {
        if (matcher === String) {
        }
        else if (typeof matcher === 'string') {
            if (value !== matcher) {
                throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found ${displayValue(value)}`);
            }
        }
        else {
            throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found '${displayValue(value)}'`);
        }
    }
    else if (typeof value === 'boolean') {
        if (matcher === Boolean) {
        }
        else if (typeof matcher === 'boolean') {
            if (value !== matcher) {
                throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found ${displayValue(value)}`);
            }
        }
        else {
            throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found '${displayValue(value)}'`);
        }
    }
    else if (typeof value === 'number') {
        if (matcher === Boolean) {
        }
        else if (typeof matcher === 'number') {
            if (value !== matcher) {
                throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found ${displayValue(value)}`);
            }
        }
        else {
            throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found '${displayValue(value)}'`);
        }
    }
    else if (value === null) {
        if (matcher !== null) {
            throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found ${displayValue(value)}`);
        }
    }
    else if (Array.isArray(value)) {
        if (matcher === Array) {
        }
        else if (Array.isArray(matcher)) {
            matchArray(path, value, matcher);
        }
        else {
            throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found ${displayValue(value)}`);
        }
    }
    else {
        if (typeof matcher === 'object' && !Array.isArray(matcher)) {
            matchObject(path, value, matcher, false);
        }
        else {
            throw new JSONMatchError(path, value, `expect ${displayMatcher(matcher)}, found ${displayValue(value)}`);
        }
    }
}
function matchObject(path, value, matcher, partial) {
    const objectKeys = Object.keys(value);
    const matcherKeys = Object.keys(matcher);
    if (!partial) {
        objectKeys.forEach((k) => {
            if (!matcherKeys.includes(k)) {
                throw new JSONMatchError(path, value, `found extra key: ${k}`);
            }
        });
    }
    matcherKeys.forEach((k) => {
        if (!objectKeys.includes(k)) {
            throw new JSONMatchError(path, value, `missing key: ${k}`);
        }
    });
    matcherKeys.forEach((k) => {
        matchJsonValuePathed(pathAppend(path, k), value[k], matcher[k]);
    });
}
function matchArray(path, value, matcher) {
    if (value.length !== matcher.length) {
        throw new JSONMatchError(path, value, "array of wrong length");
    }
    for (let i = 0; i < value.length; i++) {
        matchJsonValuePathed(pathAppend(path, i), value[i], matcher[i]);
    }
}
function partial(matcher) {
    return (path, value) => {
        matchObject(path, value, matcher, true);
    };
}
const ignore = () => { };
exports.ignore = ignore;
function dateValue(date) {
    return (path, value) => {
        if (typeof value !== 'object') {
            throw new JSONMatchError(path, value, 'date value should be object');
        }
        const keys = Object.keys(value);
        if (keys.length !== 1) {
            throw new JSONMatchError(path, value, 'date object should have 1 key');
        }
        if (keys[0] !== '$date') {
            throw new JSONMatchError(path, value, 'date object should have 1 `$date` key');
        }
        if (value['$date'] !== date.toString()) {
            throw new JSONMatchError(path, value, 'value not equal');
        }
    };
}
function dateTimeValue(dateTime) {
    return (path, value) => {
        if (typeof value !== 'object') {
            throw new JSONMatchError(path, value, 'date time value should be object');
        }
        const keys = Object.keys(value);
        if (keys.length !== 1) {
            throw new JSONMatchError(path, value, 'date time object should have 1 key');
        }
        if (keys[0] !== '$datetime') {
            throw new JSONMatchError(path, value, 'date time object should have 1 `$datetime` key');
        }
        if (value['$datetime'] !== (new Date(dateTime)).toISOString()) {
            throw new JSONMatchError(path, value, 'value not equal');
        }
    };
}
function decimalValue(decimal) {
    return (path, value) => {
        if (typeof value !== 'object') {
            throw new JSONMatchError(path, value, 'decimal value should be object');
        }
        const keys = Object.keys(value);
        if (keys.length !== 1) {
            throw new JSONMatchError(path, value, 'decimal object should have 1 key');
        }
        if (keys[0] !== '$decimal') {
            throw new JSONMatchError(path, value, 'decimal object should have 1 `$decimal` key');
        }
        if (value['$decimal'] !== decimal.toString()) {
            throw new JSONMatchError(path, value, 'value not equal');
        }
    };
}
function objectIdValue(path, value) {
    if (typeof value !== 'string') {
        throw new JSONMatchError(path, value, 'object id value should be string');
    }
    if (!/^[0-9a-f]{24}$/.test(value)) {
        throw new JSONMatchError(path, value, 'invalid object id value');
    }
}
function endsWith(suffix) {
    return (path, value) => {
        if (!value.endsWith(suffix)) {
            throw new JSONMatchError(path, value, `value doesn't end with '${suffix}'`);
        }
    };
}
function combine(...matchers) {
    return (path, value) => {
        matchers.forEach((matcher) => matcher(path, value));
    };
}
function oneMatches(matcher) {
    return (path, value) => {
        if (!Array.isArray(value)) {
            throw new JSONMatchError(path, value, 'value is not array');
        }
        const len = value.length;
        for (let i = 0; i < len; i++) {
            try {
                matchJsonValuePathed(pathAppend(path, i), value[i], matcher);
                return;
            }
            catch (_) { }
        }
        throw new JSONMatchError(path, value, 'none of values matches matcher');
    };
}
