import Decimal from "decimal.js";
import { DateOnly } from "..";
export type KeyPath = (string | number)[];
export type Matcher = (path: KeyPath, value: any) => void;
export declare class JSONMatchError extends Error {
    constructor(path: (string | number)[], found: any, reason?: string);
}
export declare function matchJson(value: any, matcher: any): () => void;
export declare function matchJsonValue(value: any, matcher: any): void;
export declare function formatPath(path: KeyPath): string;
export declare function pathAppend(path: KeyPath, next: string | number): KeyPath;
export declare function displayValue(value: any): string;
export declare function displayMatcher(matcher: any): string;
export declare function matchJsonValuePathed(path: KeyPath, value: any, matcher: any): void;
export declare function partial(matcher: any): Matcher;
export declare const ignore: Matcher;
export declare function dateValue(date: string | DateOnly): Matcher;
export declare function dateTimeValue(dateTime: string | Date | number): Matcher;
export declare function decimalValue(decimal: number | Decimal | string): Matcher;
export declare function objectIdValue(path: KeyPath, value: any): void;
export declare function endsWith(suffix: string): Matcher;
export declare function combine(...matchers: Matcher[]): Matcher;
export declare function oneMatches(matcher: any): Matcher;
