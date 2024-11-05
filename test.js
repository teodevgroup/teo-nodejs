"use strict";
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
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
var JSONMatchError = /** @class */ (function (_super) {
    __extends(JSONMatchError, _super);
    function JSONMatchError(path, found, reason) {
        return _super.call(this, "".concat(formatPath(path), ": ").concat(reason || "value is invalid", "\nFound value: ").concat(displayValue(found))) || this;
    }
    return JSONMatchError;
}(Error));
exports.JSONMatchError = JSONMatchError;
function matchJson(value, matcher) {
    return function () { return matchJsonValue(value, matcher); };
}
function matchJsonValue(value, matcher) {
    matchJsonValuePathed([], value, matcher);
}
function formatPath(path) {
    if (path.length === 0) {
        return '(root)';
    }
    if (path.length === 1) {
        return "".concat(path[0]);
    }
    var result = "".concat(path[0]);
    for (var i = 1; i < path.length; i++) {
        var current = path[i];
        if (typeof current === 'number') {
            result += "[".concat(current, "]");
        }
        else {
            result += ".".concat(current);
        }
    }
    return result;
}
function pathAppend(path, next) {
    var retval = __spreadArray([], path, true);
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
                throw new JSONMatchError(path, value, "expect ".concat(displayMatcher(matcher), ", found ").concat(displayValue(value)));
            }
        }
        else {
            throw new JSONMatchError(path, value, "expect ".concat(displayMatcher(matcher), ", found '").concat(displayValue(value), "'"));
        }
    }
    else if (typeof value === 'boolean') {
        if (matcher === Boolean) {
        }
        else if (typeof matcher === 'boolean') {
            if (value !== matcher) {
                throw new JSONMatchError(path, value, "expect ".concat(displayMatcher(matcher), ", found ").concat(displayValue(value)));
            }
        }
        else {
            throw new JSONMatchError(path, value, "expect ".concat(displayMatcher(matcher), ", found '").concat(displayValue(value), "'"));
        }
    }
    else if (typeof value === 'number') {
        if (matcher === Boolean) {
        }
        else if (typeof matcher === 'number') {
            if (value !== matcher) {
                throw new JSONMatchError(path, value, "expect ".concat(displayMatcher(matcher), ", found ").concat(displayValue(value)));
            }
        }
        else {
            throw new JSONMatchError(path, value, "expect ".concat(displayMatcher(matcher), ", found '").concat(displayValue(value), "'"));
        }
    }
    else if (value === null) {
        if (matcher !== null) {
            throw new JSONMatchError(path, value, "expect ".concat(displayMatcher(matcher), ", found ").concat(displayValue(value)));
        }
    }
    else if (Array.isArray(value)) {
        if (matcher === Array) {
        }
        else if (Array.isArray(matcher)) {
            matchArray(path, value, matcher);
        }
        else {
            throw new JSONMatchError(path, value, "expect ".concat(displayMatcher(matcher), ", found ").concat(displayValue(value)));
        }
    }
    else {
        if (typeof matcher === 'object' && !Array.isArray(matcher)) {
            matchObject(path, value, matcher, false);
        }
        else {
            throw new JSONMatchError(path, value, "expect ".concat(displayMatcher(matcher), ", found ").concat(displayValue(value)));
        }
    }
}
function matchObject(path, value, matcher, partial) {
    var objectKeys = Object.keys(value);
    var matcherKeys = Object.keys(matcher);
    if (!partial) {
        objectKeys.forEach(function (k) {
            if (!matcherKeys.includes(k)) {
                throw new JSONMatchError(path, value, "found extra key: ".concat(k));
            }
        });
    }
    matcherKeys.forEach(function (k) {
        if (!objectKeys.includes(k)) {
            throw new JSONMatchError(path, value, "missing key: ".concat(k));
        }
    });
    matcherKeys.forEach(function (k) {
        matchJsonValuePathed(pathAppend(path, k), value[k], matcher[k]);
    });
}
function matchArray(path, value, matcher) {
    if (value.length !== matcher.length) {
        throw new JSONMatchError(path, value, "array of wrong length");
    }
    for (var i = 0; i < value.length; i++) {
        matchJsonValuePathed(pathAppend(path, i), value[i], matcher[i]);
    }
}
function partial(matcher) {
    return function (path, value) {
        matchObject(path, value, matcher, true);
    };
}
var ignore = function () { };
exports.ignore = ignore;
function dateValue(date) {
    return function (path, value) {
        if (typeof value !== 'object') {
            throw new JSONMatchError(path, value, 'date value should be object');
        }
        var keys = Object.keys(value);
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
    return function (path, value) {
        if (typeof value !== 'object') {
            throw new JSONMatchError(path, value, 'date time value should be object');
        }
        var keys = Object.keys(value);
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
    return function (path, value) {
        if (typeof value !== 'object') {
            throw new JSONMatchError(path, value, 'decimal value should be object');
        }
        var keys = Object.keys(value);
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
    return function (path, value) {
        if (!value.endsWith(suffix)) {
            throw new JSONMatchError(path, value, "value doesn't end with '".concat(suffix, "'"));
        }
    };
}
function combine() {
    var matchers = [];
    for (var _i = 0; _i < arguments.length; _i++) {
        matchers[_i] = arguments[_i];
    }
    return function (path, value) {
        matchers.forEach(function (matcher) { return matcher(path, value); });
    };
}
function oneMatches(matcher) {
    return function (path, value) {
        if (!Array.isArray(value)) {
            throw new JSONMatchError(path, value, 'value is not array');
        }
        var len = value.length;
        for (var i = 0; i < len; i++) {
            try {
                matchJsonValuePathed(pathAppend(path, i), value[i], matcher);
                return;
            }
            catch (_) { }
        }
        throw new JSONMatchError(path, value, 'none of values matches matcher');
    };
}
