// @typescript-eslint/no-unused-vars

import Decimal from "decimal.js"
import { DateOnly, ObjectId, File } from "@teodevgroup/teo"

export type ExistKeys<T> = {
    [key in keyof T]: T[key] extends false | undefined | null ? never : key
}[keyof T]

type HasSelect = {
    select: any
}

type HasInclude = {
    include: any
}

export type CheckSelectInclude<T, S, U> = T extends HasSelect
    ? U
    : T extends HasInclude
    ? U
    : S

export type SelectSubset<T, U> = U extends HasSelect
    ? {
        [K in ExistKeys<U['select']>]: K extends keyof T ? T[K] : never
    }
    : T

export type Enumerable<T> = T | Array<T>

export type Subset<T, U> = {
    [key in keyof T]: key extends keyof U ? T[key] : never
}

export type GetScalarType<T, O> = O extends object ? {
    [P in keyof T]: P extends keyof O
      ? O[P]
      : never
} : never

type Teo__Pick<T, K extends keyof T> = {
    [P in K]: T[P];
}

type PickEnumerable<T, K extends Enumerable<keyof T> | keyof T> = Teo__Pick<T, MaybeTupleToUnion<K>>

export type Boolean = True | False

export type True = 1

export type False = 0

export type Not<B extends Boolean> = {
  0: 1
  1: 0
}[B]

type NoExpand<T> = T extends unknown ? T : never;

type Key = string | number | symbol;
type AtBasic<O extends object, K extends Key> = K extends keyof O ? O[K] : never;
type AtStrict<O extends object, K extends Key> = O[K & keyof O];
type AtLoose<O extends object, K extends Key> = O extends unknown ? AtStrict<O, K> : never;
export type At<O extends object, K extends Key, strict extends Boolean = 1> = {
    1: AtStrict<O, K>;
    0: AtLoose<O, K>;
}[strict];

export type IntersectOf<U extends Union> = (
  U extends unknown ? (k: U) => void : never
) extends (k: infer I) => void
  ? I
  : never

export type Overwrite<O extends object, O1 extends object> = {
    [K in keyof O]: K extends keyof O1 ? O1[K] : O[K];
} & {};

type _Merge<U extends object> = IntersectOf<Overwrite<U, {
    [K in keyof U]-?: At<U, K>;
}>>;

export type ComputeRaw<A extends any> = A extends Function ? A : {
  [K in keyof A]: A[K];
} & {};

export type OptionalFlat<O> = {
  [K in keyof O]?: O[K];
} & {};

type _Record<K extends keyof any, T> = {
  [P in K]: T;
};

type AtLeast<O extends object, K extends string> = NoExpand<
  O extends unknown
  ? | (K extends keyof O ? { [P in K]: O[P] } & O : O)
    | {[P in keyof O as P extends K ? K : never]-?: O[P]} & O
  : never>;

type _Strict<U, _U = U> = U extends unknown ? U & OptionalFlat<_Record<Exclude<Keys<_U>, keyof U>, never>> : never;

export type Strict<U extends object> = ComputeRaw<_Strict<U>>;

export type Merge<U extends object> = ComputeRaw<_Merge<Strict<U>>>;

type ExcludeUnderscoreKeys<T extends string> = T extends `_${string}` ? never : T

export type Union = any

export type Extends<A1 extends any, A2 extends any> = [A1] extends [never]
  ? 0 // anything `never` is false
  : A1 extends A2
  ? 1
  : 0

export type Has<U extends Union, U1 extends Union> = Not<
  Extends<Exclude<U1, U>, U1>
>

export type Or<B1 extends Boolean, B2 extends Boolean> = {
  0: {
    0: 0
    1: 1
  }
  1: {
    0: 1
    1: 1
  }
}[B1][B2]

export type Keys<U extends Union> = U extends unknown ? keyof U : never

type Cast<A, B> = A extends B ? A : B;

type IsObject<T extends any> = T extends Array<any>
? False
: T extends Date
? False
: T extends Uint8Array
? False
: T extends object
? True
: False

type FieldPaths<
  T,
  U = Omit<T, '_avg' | '_sum' | '_count' | '_min' | '_max'>
> = IsObject<T> extends True ? U : T

type GetHavingFields<T> = {
  [K in keyof T]: Or<
    Or<Extends<'OR', K>, Extends<'AND', K>>,
    Extends<'NOT', K>
  > extends True
    ? // infer is only needed to not hit TS limit
      // based on the brilliant idea of Pierre-Antoine Mills
      // https://github.com/microsoft/TypeScript/issues/30188#issuecomment-478938437
      T[K] extends infer TK
      ? GetHavingFields<UnEnumerate<TK> extends object ? Merge<UnEnumerate<TK>> : never>
      : never
    : {} extends FieldPaths<T[K]>
    ? never
    : K
}[keyof T]

export type UnEnumerate<T extends unknown> = T extends Array<infer U> ? U : T

export type SubsetIntersection<T, U, K> = {
  [key in keyof T]: key extends keyof U ? T[key] : never
} & K

type _TupleToUnion<T> = T extends (infer E)[] ? E : never
type TupleToUnion<K extends readonly any[]> = _TupleToUnion<K>
type MaybeTupleToUnion<T> = T extends any[] ? TupleToUnion<T> : T

/**
 * **Status**
 *
 * This enum doesn't have a description.
 */
export type Status = "open" | "inProgress" | "pending" | "waitingForReview" | "done"

/**
 * **Container scalar fields**
 *
 * This synthesized enum doesn't have a description.
 */
export type ContainerScalarFields = "bool" | "boolArray" | "date" | "dateArray" | "dateTime" | "dateTimeArray" | "decimal" | "decimalArray" | "float32" | "float32Array" | "float64" | "float64Array" | "id" | "int32" | "int32Array" | "int64" | "int64Array" | "message" | "status" | "statusArray" | "string" | "stringArray"

/**
 * **Container serializable scalar fields**
 *
 * This synthesized enum doesn't have a description.
 */
export type ContainerSerializableScalarFields = "bool" | "boolArray" | "date" | "dateArray" | "dateTime" | "dateTimeArray" | "decimal" | "decimalArray" | "float32" | "float32Array" | "float64" | "float64Array" | "id" | "int32" | "int32Array" | "int64" | "int64Array" | "message" | "status" | "statusArray" | "string" | "stringArray"

/**
 * **Container relations**
 *
 * This synthesized enum doesn't have a description.
 */
export type ContainerRelations = undefined

/**
 * **Container direct relations**
 *
 * This synthesized enum doesn't have a description.
 */
export type ContainerDirectRelations = undefined

/**
 * **Container indirect relations**
 *
 * This synthesized enum doesn't have a description.
 */
export type ContainerIndirectRelations = undefined

/// ## Status
///
/// This enum doesn't have a description.
export const enum StatusEnumType {

    /// ### Open
    ///
    /// This enum member doesn't have a description.
    open = "open",

    /// ### In progress
    ///
    /// This enum member doesn't have a description.
    inProgress = "inProgress",

    /// ### Pending
    ///
    /// This enum member doesn't have a description.
    pending = "pending",

    /// ### Waiting for review
    ///
    /// This enum member doesn't have a description.
    waitingForReview = "waitingForReview",

    /// ### Done
    ///
    /// This enum member doesn't have a description.
    done = "done",
}

/// ## Container scalar fields
///
/// This synthesized enum doesn't have a description.
export const enum ContainerScalarFieldsEnumType {

    /// ### Bool
    ///
    /// This synthesized enum member doesn't have a description.
    bool = "bool",

    /// ### Bool array
    ///
    /// This synthesized enum member doesn't have a description.
    boolArray = "boolArray",

    /// ### Date
    ///
    /// This synthesized enum member doesn't have a description.
    date = "date",

    /// ### Date array
    ///
    /// This synthesized enum member doesn't have a description.
    dateArray = "dateArray",

    /// ### Date time
    ///
    /// This synthesized enum member doesn't have a description.
    dateTime = "dateTime",

    /// ### Date time array
    ///
    /// This synthesized enum member doesn't have a description.
    dateTimeArray = "dateTimeArray",

    /// ### Decimal
    ///
    /// This synthesized enum member doesn't have a description.
    decimal = "decimal",

    /// ### Decimal array
    ///
    /// This synthesized enum member doesn't have a description.
    decimalArray = "decimalArray",

    /// ### Float32
    ///
    /// This synthesized enum member doesn't have a description.
    float32 = "float32",

    /// ### Float32 array
    ///
    /// This synthesized enum member doesn't have a description.
    float32Array = "float32Array",

    /// ### Float64
    ///
    /// This synthesized enum member doesn't have a description.
    float64 = "float64",

    /// ### Float64 array
    ///
    /// This synthesized enum member doesn't have a description.
    float64Array = "float64Array",

    /// ### Id
    ///
    /// This synthesized enum member doesn't have a description.
    id = "id",

    /// ### Int32
    ///
    /// This synthesized enum member doesn't have a description.
    int32 = "int32",

    /// ### Int32 array
    ///
    /// This synthesized enum member doesn't have a description.
    int32Array = "int32Array",

    /// ### Int64
    ///
    /// This synthesized enum member doesn't have a description.
    int64 = "int64",

    /// ### Int64 array
    ///
    /// This synthesized enum member doesn't have a description.
    int64Array = "int64Array",

    /// ### Message
    ///
    /// This synthesized enum member doesn't have a description.
    message = "message",

    /// ### Status
    ///
    /// This synthesized enum member doesn't have a description.
    status = "status",

    /// ### Status array
    ///
    /// This synthesized enum member doesn't have a description.
    statusArray = "statusArray",

    /// ### String
    ///
    /// This synthesized enum member doesn't have a description.
    string = "string",

    /// ### String array
    ///
    /// This synthesized enum member doesn't have a description.
    stringArray = "stringArray",
}

/// ## Container serializable scalar fields
///
/// This synthesized enum doesn't have a description.
export const enum ContainerSerializableScalarFieldsEnumType {

    /// ### Bool
    ///
    /// This synthesized enum member doesn't have a description.
    bool = "bool",

    /// ### Bool array
    ///
    /// This synthesized enum member doesn't have a description.
    boolArray = "boolArray",

    /// ### Date
    ///
    /// This synthesized enum member doesn't have a description.
    date = "date",

    /// ### Date array
    ///
    /// This synthesized enum member doesn't have a description.
    dateArray = "dateArray",

    /// ### Date time
    ///
    /// This synthesized enum member doesn't have a description.
    dateTime = "dateTime",

    /// ### Date time array
    ///
    /// This synthesized enum member doesn't have a description.
    dateTimeArray = "dateTimeArray",

    /// ### Decimal
    ///
    /// This synthesized enum member doesn't have a description.
    decimal = "decimal",

    /// ### Decimal array
    ///
    /// This synthesized enum member doesn't have a description.
    decimalArray = "decimalArray",

    /// ### Float32
    ///
    /// This synthesized enum member doesn't have a description.
    float32 = "float32",

    /// ### Float32 array
    ///
    /// This synthesized enum member doesn't have a description.
    float32Array = "float32Array",

    /// ### Float64
    ///
    /// This synthesized enum member doesn't have a description.
    float64 = "float64",

    /// ### Float64 array
    ///
    /// This synthesized enum member doesn't have a description.
    float64Array = "float64Array",

    /// ### Id
    ///
    /// This synthesized enum member doesn't have a description.
    id = "id",

    /// ### Int32
    ///
    /// This synthesized enum member doesn't have a description.
    int32 = "int32",

    /// ### Int32 array
    ///
    /// This synthesized enum member doesn't have a description.
    int32Array = "int32Array",

    /// ### Int64
    ///
    /// This synthesized enum member doesn't have a description.
    int64 = "int64",

    /// ### Int64 array
    ///
    /// This synthesized enum member doesn't have a description.
    int64Array = "int64Array",

    /// ### Message
    ///
    /// This synthesized enum member doesn't have a description.
    message = "message",

    /// ### Status
    ///
    /// This synthesized enum member doesn't have a description.
    status = "status",

    /// ### Status array
    ///
    /// This synthesized enum member doesn't have a description.
    statusArray = "statusArray",

    /// ### String
    ///
    /// This synthesized enum member doesn't have a description.
    string = "string",

    /// ### String array
    ///
    /// This synthesized enum member doesn't have a description.
    stringArray = "stringArray",
}

/// ## Container relations
///
/// This synthesized enum doesn't have a description.
export const enum ContainerRelationsEnumType {
}

/// ## Container direct relations
///
/// This synthesized enum doesn't have a description.
export const enum ContainerDirectRelationsEnumType {
}

/// ## Container indirect relations
///
/// This synthesized enum doesn't have a description.
export const enum ContainerIndirectRelationsEnumType {
}


/**
 * **Container select**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerSelect = {
    
    /**
     * **Bool**
     *
     * This synthesized field doesn't have a description.
     */
     bool?: boolean
    
    /**
     * **Bool Array**
     *
     * This synthesized field doesn't have a description.
     */
     boolArray?: boolean
    
    /**
     * **Date**
     *
     * This synthesized field doesn't have a description.
     */
     date?: boolean
    
    /**
     * **Date Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateArray?: boolean
    
    /**
     * **Date Time**
     *
     * This synthesized field doesn't have a description.
     */
     dateTime?: boolean
    
    /**
     * **Date Time Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateTimeArray?: boolean
    
    /**
     * **Decimal**
     *
     * This synthesized field doesn't have a description.
     */
     decimal?: boolean
    
    /**
     * **Decimal Array**
     *
     * This synthesized field doesn't have a description.
     */
     decimalArray?: boolean
    
    /**
     * **Float32**
     *
     * This synthesized field doesn't have a description.
     */
     float32?: boolean
    
    /**
     * **Float32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float32Array?: boolean
    
    /**
     * **Float64**
     *
     * This synthesized field doesn't have a description.
     */
     float64?: boolean
    
    /**
     * **Float64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float64Array?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
    /**
     * **Int32**
     *
     * This synthesized field doesn't have a description.
     */
     int32?: boolean
    
    /**
     * **Int32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int32Array?: boolean
    
    /**
     * **Int64**
     *
     * This synthesized field doesn't have a description.
     */
     int64?: boolean
    
    /**
     * **Int64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int64Array?: boolean
    
    /**
     * **Message**
     *
     * This synthesized field doesn't have a description.
     */
     message?: boolean
    
    /**
     * **Status**
     *
     * This synthesized field doesn't have a description.
     */
     status?: boolean
    
    /**
     * **Status Array**
     *
     * This synthesized field doesn't have a description.
     */
     statusArray?: boolean
    
    /**
     * **String**
     *
     * This synthesized field doesn't have a description.
     */
     string?: boolean
    
    /**
     * **String Array**
     *
     * This synthesized field doesn't have a description.
     */
     stringArray?: boolean
    
}


/**
 * **Container include**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerInclude = {
    
}


/**
 * **Container where input**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerWhereInput = {
    
    /**
     * **And**
     *
     * This synthesized field doesn't have a description.
     */
     AND?: ContainerWhereInput[]
    
    /**
     * **Not**
     *
     * This synthesized field doesn't have a description.
     */
     NOT?: ContainerWhereInput
    
    /**
     * **Or**
     *
     * This synthesized field doesn't have a description.
     */
     OR?: ContainerWhereInput[]
    
    /**
     * **Bool**
     *
     * This synthesized field doesn't have a description.
     */
     bool?: boolean | null | std.BoolNullableFilter
    
    /**
     * **Bool Array**
     *
     * This synthesized field doesn't have a description.
     */
     boolArray?: boolean[] | null | std.ArrayNullableFilter<boolean>
    
    /**
     * **Date**
     *
     * This synthesized field doesn't have a description.
     */
     date?: DateOnly | null | std.NullableFilter<DateOnly>
    
    /**
     * **Date Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateArray?: DateOnly[] | null | std.ArrayNullableFilter<DateOnly>
    
    /**
     * **Date Time**
     *
     * This synthesized field doesn't have a description.
     */
     dateTime?: Date | null | std.NullableFilter<Date>
    
    /**
     * **Date Time Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateTimeArray?: Date[] | null | std.ArrayNullableFilter<Date>
    
    /**
     * **Decimal**
     *
     * This synthesized field doesn't have a description.
     */
     decimal?: Decimal | null | std.NullableFilter<Decimal>
    
    /**
     * **Decimal Array**
     *
     * This synthesized field doesn't have a description.
     */
     decimalArray?: Decimal[] | null | std.ArrayNullableFilter<Decimal>
    
    /**
     * **Float32**
     *
     * This synthesized field doesn't have a description.
     */
     float32?: number | null | std.NullableFilter<number>
    
    /**
     * **Float32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float32Array?: number[] | null | std.ArrayNullableFilter<number>
    
    /**
     * **Float64**
     *
     * This synthesized field doesn't have a description.
     */
     float64?: number | null | std.NullableFilter<number>
    
    /**
     * **Float64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float64Array?: number[] | null | std.ArrayNullableFilter<number>
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number | std.Filter<number>
    
    /**
     * **Int32**
     *
     * This synthesized field doesn't have a description.
     */
     int32?: number | null | std.NullableFilter<number>
    
    /**
     * **Int32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int32Array?: number[] | null | std.ArrayNullableFilter<number>
    
    /**
     * **Int64**
     *
     * This synthesized field doesn't have a description.
     */
     int64?: number | null | std.NullableFilter<number>
    
    /**
     * **Int64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int64Array?: number[] | null | std.ArrayNullableFilter<number>
    
    /**
     * **Message**
     *
     * This synthesized field doesn't have a description.
     */
     message?: string | null | std.StringNullableFilter
    
    /**
     * **Status**
     *
     * This synthesized field doesn't have a description.
     */
     status?: Status | null | std.EnumNullableFilter<Status>
    
    /**
     * **Status Array**
     *
     * This synthesized field doesn't have a description.
     */
     statusArray?: Status[] | null | std.ArrayNullableFilter<Status>
    
    /**
     * **String**
     *
     * This synthesized field doesn't have a description.
     */
     string?: string | null | std.StringNullableFilter
    
    /**
     * **String Array**
     *
     * This synthesized field doesn't have a description.
     */
     stringArray?: string[] | null | std.ArrayNullableFilter<string>
    
}


/**
 * **Container where unique input**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerWhereUniqueInput = {
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id: number
    
}


/**
 * **Container scalar where with aggregates input**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerScalarWhereWithAggregatesInput = {
    
    /**
     * **And**
     *
     * This synthesized field doesn't have a description.
     */
     AND?: ContainerWhereInput[]
    
    /**
     * **Not**
     *
     * This synthesized field doesn't have a description.
     */
     NOT?: ContainerWhereInput
    
    /**
     * **Or**
     *
     * This synthesized field doesn't have a description.
     */
     OR?: ContainerWhereInput[]
    
    /**
     * **Bool**
     *
     * This synthesized field doesn't have a description.
     */
     bool?: boolean | null | std.BoolNullableWithAggregatesFilter
    
    /**
     * **Bool Array**
     *
     * This synthesized field doesn't have a description.
     */
     boolArray?: boolean[] | null | std.ArrayNullableWithAggregatesFilter<boolean>
    
    /**
     * **Date**
     *
     * This synthesized field doesn't have a description.
     */
     date?: DateOnly | null | std.NullableAggregatesFilter<DateOnly>
    
    /**
     * **Date Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateArray?: DateOnly[] | null | std.ArrayNullableWithAggregatesFilter<DateOnly>
    
    /**
     * **Date Time**
     *
     * This synthesized field doesn't have a description.
     */
     dateTime?: Date | null | std.NullableAggregatesFilter<Date>
    
    /**
     * **Date Time Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateTimeArray?: Date[] | null | std.ArrayNullableWithAggregatesFilter<Date>
    
    /**
     * **Decimal**
     *
     * This synthesized field doesn't have a description.
     */
     decimal?: Decimal | null | std.DecimalNullableWithAggregatesFilter<Decimal>
    
    /**
     * **Decimal Array**
     *
     * This synthesized field doesn't have a description.
     */
     decimalArray?: Decimal[] | null | std.ArrayNullableWithAggregatesFilter<Decimal>
    
    /**
     * **Float32**
     *
     * This synthesized field doesn't have a description.
     */
     float32?: number | null | std.FloatNumberNullableWithAggregatesFilter<number>
    
    /**
     * **Float32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float32Array?: number[] | null | std.ArrayNullableWithAggregatesFilter<number>
    
    /**
     * **Float64**
     *
     * This synthesized field doesn't have a description.
     */
     float64?: number | null | std.FloatNumberNullableWithAggregatesFilter<number>
    
    /**
     * **Float64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float64Array?: number[] | null | std.ArrayNullableWithAggregatesFilter<number>
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number | std.IntNumberWithAggregatesFilter<number>
    
    /**
     * **Int32**
     *
     * This synthesized field doesn't have a description.
     */
     int32?: number | null | std.IntNumberNullableWithAggregatesFilter<number>
    
    /**
     * **Int32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int32Array?: number[] | null | std.ArrayNullableWithAggregatesFilter<number>
    
    /**
     * **Int64**
     *
     * This synthesized field doesn't have a description.
     */
     int64?: number | null | std.IntNumberNullableWithAggregatesFilter<number>
    
    /**
     * **Int64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int64Array?: number[] | null | std.ArrayNullableWithAggregatesFilter<number>
    
    /**
     * **Message**
     *
     * This synthesized field doesn't have a description.
     */
     message?: string | null | std.StringNullableWithAggregatesFilter
    
    /**
     * **Status**
     *
     * This synthesized field doesn't have a description.
     */
     status?: Status | null | std.EnumNullableWithAggregatesFilter<Status>
    
    /**
     * **Status Array**
     *
     * This synthesized field doesn't have a description.
     */
     statusArray?: Status[] | null | std.ArrayNullableWithAggregatesFilter<Status>
    
    /**
     * **String**
     *
     * This synthesized field doesn't have a description.
     */
     string?: string | null | std.StringNullableWithAggregatesFilter
    
    /**
     * **String Array**
     *
     * This synthesized field doesn't have a description.
     */
     stringArray?: string[] | null | std.ArrayNullableWithAggregatesFilter<string>
    
}


/**
 * **Container relation filter**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerRelationFilter = {
    
    /**
     * **Is**
     *
     * This synthesized field doesn't have a description.
     */
     is?: ContainerWhereInput
    
    /**
     * **Is Not**
     *
     * This synthesized field doesn't have a description.
     */
     isNot?: ContainerWhereInput
    
}


/**
 * **Container list relation filter**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerListRelationFilter = {
    
    /**
     * **Every**
     *
     * This synthesized field doesn't have a description.
     */
     every?: ContainerWhereInput
    
    /**
     * **None**
     *
     * This synthesized field doesn't have a description.
     */
     none?: ContainerWhereInput
    
    /**
     * **Some**
     *
     * This synthesized field doesn't have a description.
     */
     some?: ContainerWhereInput
    
}


/**
 * **Container order by input**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerOrderByInput = {
    
    /**
     * **Bool**
     *
     * This synthesized field doesn't have a description.
     */
     bool?: std.Sort
    
    /**
     * **Bool Array**
     *
     * This synthesized field doesn't have a description.
     */
     boolArray?: std.Sort
    
    /**
     * **Date**
     *
     * This synthesized field doesn't have a description.
     */
     date?: std.Sort
    
    /**
     * **Date Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateArray?: std.Sort
    
    /**
     * **Date Time**
     *
     * This synthesized field doesn't have a description.
     */
     dateTime?: std.Sort
    
    /**
     * **Date Time Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateTimeArray?: std.Sort
    
    /**
     * **Decimal**
     *
     * This synthesized field doesn't have a description.
     */
     decimal?: std.Sort
    
    /**
     * **Decimal Array**
     *
     * This synthesized field doesn't have a description.
     */
     decimalArray?: std.Sort
    
    /**
     * **Float32**
     *
     * This synthesized field doesn't have a description.
     */
     float32?: std.Sort
    
    /**
     * **Float32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float32Array?: std.Sort
    
    /**
     * **Float64**
     *
     * This synthesized field doesn't have a description.
     */
     float64?: std.Sort
    
    /**
     * **Float64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float64Array?: std.Sort
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: std.Sort
    
    /**
     * **Int32**
     *
     * This synthesized field doesn't have a description.
     */
     int32?: std.Sort
    
    /**
     * **Int32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int32Array?: std.Sort
    
    /**
     * **Int64**
     *
     * This synthesized field doesn't have a description.
     */
     int64?: std.Sort
    
    /**
     * **Int64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int64Array?: std.Sort
    
    /**
     * **Message**
     *
     * This synthesized field doesn't have a description.
     */
     message?: std.Sort
    
    /**
     * **Status**
     *
     * This synthesized field doesn't have a description.
     */
     status?: std.Sort
    
    /**
     * **Status Array**
     *
     * This synthesized field doesn't have a description.
     */
     statusArray?: std.Sort
    
    /**
     * **String**
     *
     * This synthesized field doesn't have a description.
     */
     string?: std.Sort
    
    /**
     * **String Array**
     *
     * This synthesized field doesn't have a description.
     */
     stringArray?: std.Sort
    
}


/**
 * **Container count aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerCountAggregateInputType = {
    
    /**
     * **All**
     *
     * This synthesized field doesn't have a description.
     */
     _all?: boolean
    
    /**
     * **Bool**
     *
     * This synthesized field doesn't have a description.
     */
     bool?: boolean
    
    /**
     * **Bool Array**
     *
     * This synthesized field doesn't have a description.
     */
     boolArray?: boolean
    
    /**
     * **Date**
     *
     * This synthesized field doesn't have a description.
     */
     date?: boolean
    
    /**
     * **Date Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateArray?: boolean
    
    /**
     * **Date Time**
     *
     * This synthesized field doesn't have a description.
     */
     dateTime?: boolean
    
    /**
     * **Date Time Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateTimeArray?: boolean
    
    /**
     * **Decimal**
     *
     * This synthesized field doesn't have a description.
     */
     decimal?: boolean
    
    /**
     * **Decimal Array**
     *
     * This synthesized field doesn't have a description.
     */
     decimalArray?: boolean
    
    /**
     * **Float32**
     *
     * This synthesized field doesn't have a description.
     */
     float32?: boolean
    
    /**
     * **Float32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float32Array?: boolean
    
    /**
     * **Float64**
     *
     * This synthesized field doesn't have a description.
     */
     float64?: boolean
    
    /**
     * **Float64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float64Array?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
    /**
     * **Int32**
     *
     * This synthesized field doesn't have a description.
     */
     int32?: boolean
    
    /**
     * **Int32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int32Array?: boolean
    
    /**
     * **Int64**
     *
     * This synthesized field doesn't have a description.
     */
     int64?: boolean
    
    /**
     * **Int64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int64Array?: boolean
    
    /**
     * **Message**
     *
     * This synthesized field doesn't have a description.
     */
     message?: boolean
    
    /**
     * **Status**
     *
     * This synthesized field doesn't have a description.
     */
     status?: boolean
    
    /**
     * **Status Array**
     *
     * This synthesized field doesn't have a description.
     */
     statusArray?: boolean
    
    /**
     * **String**
     *
     * This synthesized field doesn't have a description.
     */
     string?: boolean
    
    /**
     * **String Array**
     *
     * This synthesized field doesn't have a description.
     */
     stringArray?: boolean
    
}


/**
 * **Container sum aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerSumAggregateInputType = {
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
}


/**
 * **Container avg aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerAvgAggregateInputType = {
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
}


/**
 * **Container min aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerMinAggregateInputType = {
    
    /**
     * **Bool**
     *
     * This synthesized field doesn't have a description.
     */
     bool?: boolean
    
    /**
     * **Bool Array**
     *
     * This synthesized field doesn't have a description.
     */
     boolArray?: boolean
    
    /**
     * **Date**
     *
     * This synthesized field doesn't have a description.
     */
     date?: boolean
    
    /**
     * **Date Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateArray?: boolean
    
    /**
     * **Date Time**
     *
     * This synthesized field doesn't have a description.
     */
     dateTime?: boolean
    
    /**
     * **Date Time Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateTimeArray?: boolean
    
    /**
     * **Decimal**
     *
     * This synthesized field doesn't have a description.
     */
     decimal?: boolean
    
    /**
     * **Decimal Array**
     *
     * This synthesized field doesn't have a description.
     */
     decimalArray?: boolean
    
    /**
     * **Float32**
     *
     * This synthesized field doesn't have a description.
     */
     float32?: boolean
    
    /**
     * **Float32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float32Array?: boolean
    
    /**
     * **Float64**
     *
     * This synthesized field doesn't have a description.
     */
     float64?: boolean
    
    /**
     * **Float64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float64Array?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
    /**
     * **Int32**
     *
     * This synthesized field doesn't have a description.
     */
     int32?: boolean
    
    /**
     * **Int32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int32Array?: boolean
    
    /**
     * **Int64**
     *
     * This synthesized field doesn't have a description.
     */
     int64?: boolean
    
    /**
     * **Int64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int64Array?: boolean
    
    /**
     * **Message**
     *
     * This synthesized field doesn't have a description.
     */
     message?: boolean
    
    /**
     * **Status**
     *
     * This synthesized field doesn't have a description.
     */
     status?: boolean
    
    /**
     * **Status Array**
     *
     * This synthesized field doesn't have a description.
     */
     statusArray?: boolean
    
    /**
     * **String**
     *
     * This synthesized field doesn't have a description.
     */
     string?: boolean
    
    /**
     * **String Array**
     *
     * This synthesized field doesn't have a description.
     */
     stringArray?: boolean
    
}


/**
 * **Container max aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerMaxAggregateInputType = {
    
    /**
     * **Bool**
     *
     * This synthesized field doesn't have a description.
     */
     bool?: boolean
    
    /**
     * **Bool Array**
     *
     * This synthesized field doesn't have a description.
     */
     boolArray?: boolean
    
    /**
     * **Date**
     *
     * This synthesized field doesn't have a description.
     */
     date?: boolean
    
    /**
     * **Date Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateArray?: boolean
    
    /**
     * **Date Time**
     *
     * This synthesized field doesn't have a description.
     */
     dateTime?: boolean
    
    /**
     * **Date Time Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateTimeArray?: boolean
    
    /**
     * **Decimal**
     *
     * This synthesized field doesn't have a description.
     */
     decimal?: boolean
    
    /**
     * **Decimal Array**
     *
     * This synthesized field doesn't have a description.
     */
     decimalArray?: boolean
    
    /**
     * **Float32**
     *
     * This synthesized field doesn't have a description.
     */
     float32?: boolean
    
    /**
     * **Float32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float32Array?: boolean
    
    /**
     * **Float64**
     *
     * This synthesized field doesn't have a description.
     */
     float64?: boolean
    
    /**
     * **Float64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float64Array?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
    /**
     * **Int32**
     *
     * This synthesized field doesn't have a description.
     */
     int32?: boolean
    
    /**
     * **Int32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int32Array?: boolean
    
    /**
     * **Int64**
     *
     * This synthesized field doesn't have a description.
     */
     int64?: boolean
    
    /**
     * **Int64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int64Array?: boolean
    
    /**
     * **Message**
     *
     * This synthesized field doesn't have a description.
     */
     message?: boolean
    
    /**
     * **Status**
     *
     * This synthesized field doesn't have a description.
     */
     status?: boolean
    
    /**
     * **Status Array**
     *
     * This synthesized field doesn't have a description.
     */
     statusArray?: boolean
    
    /**
     * **String**
     *
     * This synthesized field doesn't have a description.
     */
     string?: boolean
    
    /**
     * **String Array**
     *
     * This synthesized field doesn't have a description.
     */
     stringArray?: boolean
    
}


/**
 * **Container create input**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerCreateInput = {
    
    /**
     * **Bool**
     *
     * This synthesized field doesn't have a description.
     */
     bool?: boolean
    
    /**
     * **Bool Array**
     *
     * This synthesized field doesn't have a description.
     */
     boolArray?: boolean[]
    
    /**
     * **Date**
     *
     * This synthesized field doesn't have a description.
     */
     date?: DateOnly
    
    /**
     * **Date Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateArray?: DateOnly[]
    
    /**
     * **Date Time**
     *
     * This synthesized field doesn't have a description.
     */
     dateTime?: Date
    
    /**
     * **Date Time Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateTimeArray?: Date[]
    
    /**
     * **Decimal**
     *
     * This synthesized field doesn't have a description.
     */
     decimal?: Decimal
    
    /**
     * **Decimal Array**
     *
     * This synthesized field doesn't have a description.
     */
     decimalArray?: Decimal[]
    
    /**
     * **Float32**
     *
     * This synthesized field doesn't have a description.
     */
     float32?: number
    
    /**
     * **Float32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float32Array?: number[]
    
    /**
     * **Float64**
     *
     * This synthesized field doesn't have a description.
     */
     float64?: number
    
    /**
     * **Float64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float64Array?: number[]
    
    /**
     * **Int32**
     *
     * This synthesized field doesn't have a description.
     */
     int32?: number
    
    /**
     * **Int32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int32Array?: number[]
    
    /**
     * **Int64**
     *
     * This synthesized field doesn't have a description.
     */
     int64?: number
    
    /**
     * **Int64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int64Array?: number[]
    
    /**
     * **Status**
     *
     * This synthesized field doesn't have a description.
     */
     status?: Status
    
    /**
     * **Status Array**
     *
     * This synthesized field doesn't have a description.
     */
     statusArray?: Status[]
    
    /**
     * **String**
     *
     * This synthesized field doesn't have a description.
     */
     string?: string
    
    /**
     * **String Array**
     *
     * This synthesized field doesn't have a description.
     */
     stringArray?: string[]
    
}


/**
 * **Container update input**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerUpdateInput = {
    
    /**
     * **Bool**
     *
     * This synthesized field doesn't have a description.
     */
     bool?: boolean
    
    /**
     * **Bool Array**
     *
     * This synthesized field doesn't have a description.
     */
     boolArray?: boolean[]
    
    /**
     * **Date**
     *
     * This synthesized field doesn't have a description.
     */
     date?: DateOnly
    
    /**
     * **Date Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateArray?: DateOnly[]
    
    /**
     * **Date Time**
     *
     * This synthesized field doesn't have a description.
     */
     dateTime?: Date
    
    /**
     * **Date Time Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateTimeArray?: Date[]
    
    /**
     * **Decimal**
     *
     * This synthesized field doesn't have a description.
     */
     decimal?: Decimal
    
    /**
     * **Decimal Array**
     *
     * This synthesized field doesn't have a description.
     */
     decimalArray?: Decimal[]
    
    /**
     * **Float32**
     *
     * This synthesized field doesn't have a description.
     */
     float32?: number
    
    /**
     * **Float32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float32Array?: number[]
    
    /**
     * **Float64**
     *
     * This synthesized field doesn't have a description.
     */
     float64?: number
    
    /**
     * **Float64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float64Array?: number[]
    
    /**
     * **Int32**
     *
     * This synthesized field doesn't have a description.
     */
     int32?: number
    
    /**
     * **Int32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int32Array?: number[]
    
    /**
     * **Int64**
     *
     * This synthesized field doesn't have a description.
     */
     int64?: number
    
    /**
     * **Int64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int64Array?: number[]
    
    /**
     * **Status**
     *
     * This synthesized field doesn't have a description.
     */
     status?: Status
    
    /**
     * **Status Array**
     *
     * This synthesized field doesn't have a description.
     */
     statusArray?: Status[]
    
    /**
     * **String**
     *
     * This synthesized field doesn't have a description.
     */
     string?: string
    
    /**
     * **String Array**
     *
     * This synthesized field doesn't have a description.
     */
     stringArray?: string[]
    
}


/**
 * **Container create nested one input**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerCreateNestedOneInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: ContainerWhereUniqueInput
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: ContainerConnectOrCreateInput
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: ContainerCreateInput
    
}


/**
 * **Container create nested many input**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerCreateNestedManyInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: Enumerable<ContainerWhereUniqueInput>
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: Enumerable<ContainerConnectOrCreateInput>
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: Enumerable<ContainerCreateInput>
    
}


/**
 * **Container update nested one input**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerUpdateNestedOneInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: ContainerWhereUniqueInput
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: ContainerConnectOrCreateInput
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: ContainerCreateInput
    
    /**
     * **Delete**
     *
     * This synthesized field doesn't have a description.
     */
     delete?: boolean
    
    /**
     * **Disconnect**
     *
     * This synthesized field doesn't have a description.
     */
     disconnect?: boolean
    
    /**
     * **Set**
     *
     * This synthesized field doesn't have a description.
     */
     set?: ContainerWhereUniqueInput
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update?: ContainerUpdateInput
    
    /**
     * **Upsert**
     *
     * This synthesized field doesn't have a description.
     */
     upsert?: ContainerUpsertWithWhereUniqueInput
    
}


/**
 * **Container update nested many input**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerUpdateNestedManyInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: Enumerable<ContainerWhereUniqueInput>
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: Enumerable<ContainerConnectOrCreateInput>
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: Enumerable<ContainerCreateInput>
    
    /**
     * **Delete**
     *
     * This synthesized field doesn't have a description.
     */
     delete?: Enumerable<ContainerWhereUniqueInput>
    
    /**
     * **Delete Many**
     *
     * This synthesized field doesn't have a description.
     */
     deleteMany?: Enumerable<ContainerWhereInput>
    
    /**
     * **Disconnect**
     *
     * This synthesized field doesn't have a description.
     */
     disconnect?: Enumerable<ContainerWhereUniqueInput>
    
    /**
     * **Set**
     *
     * This synthesized field doesn't have a description.
     */
     set?: Enumerable<ContainerWhereUniqueInput>
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update?: Enumerable<ContainerUpdateWithWhereUniqueInput>
    
    /**
     * **Update Many**
     *
     * This synthesized field doesn't have a description.
     */
     updateMany?: Enumerable<ContainerUpdateManyWithWhereInput>
    
    /**
     * **Upsert**
     *
     * This synthesized field doesn't have a description.
     */
     upsert?: Enumerable<ContainerUpsertWithWhereUniqueInput>
    
}


/**
 * **Container connect or create input**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerConnectOrCreateInput = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: ContainerCreateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: ContainerWhereUniqueInput
    
}


/**
 * **Container update with where unique input**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerUpdateWithWhereUniqueInput = {
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: ContainerUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: ContainerWhereUniqueInput
    
}


/**
 * **Container upsert with where unique input**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerUpsertWithWhereUniqueInput = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: ContainerCreateInput
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: ContainerUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: ContainerWhereUniqueInput
    
}


/**
 * **Container update many with where input**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerUpdateManyWithWhereInput = {
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: ContainerUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: ContainerWhereInput
    
}


/**
 * **Container result**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerResult = {
    
    /**
     * **Bool**
     *
     * This synthesized field doesn't have a description.
     */
     bool?: boolean
    
    /**
     * **Bool Array**
     *
     * This synthesized field doesn't have a description.
     */
     boolArray?: boolean[]
    
    /**
     * **Date**
     *
     * This synthesized field doesn't have a description.
     */
     date?: DateOnly
    
    /**
     * **Date Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateArray?: DateOnly[]
    
    /**
     * **Date Time**
     *
     * This synthesized field doesn't have a description.
     */
     dateTime?: Date
    
    /**
     * **Date Time Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateTimeArray?: Date[]
    
    /**
     * **Decimal**
     *
     * This synthesized field doesn't have a description.
     */
     decimal?: Decimal
    
    /**
     * **Decimal Array**
     *
     * This synthesized field doesn't have a description.
     */
     decimalArray?: Decimal[]
    
    /**
     * **Float32**
     *
     * This synthesized field doesn't have a description.
     */
     float32?: number
    
    /**
     * **Float32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float32Array?: number[]
    
    /**
     * **Float64**
     *
     * This synthesized field doesn't have a description.
     */
     float64?: number
    
    /**
     * **Float64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float64Array?: number[]
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id: number
    
    /**
     * **Int32**
     *
     * This synthesized field doesn't have a description.
     */
     int32?: number
    
    /**
     * **Int32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int32Array?: number[]
    
    /**
     * **Int64**
     *
     * This synthesized field doesn't have a description.
     */
     int64?: number
    
    /**
     * **Int64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int64Array?: number[]
    
    /**
     * **Message**
     *
     * This synthesized field doesn't have a description.
     */
     message?: string
    
    /**
     * **Status**
     *
     * This synthesized field doesn't have a description.
     */
     status?: Status
    
    /**
     * **Status Array**
     *
     * This synthesized field doesn't have a description.
     */
     statusArray?: Status[]
    
    /**
     * **String**
     *
     * This synthesized field doesn't have a description.
     */
     string?: string
    
    /**
     * **String Array**
     *
     * This synthesized field doesn't have a description.
     */
     stringArray?: string[]
    
}
export type ContainerResultGetPayload<S extends boolean | null | undefined | ContainerArgs, U = keyof S> = S extends true
    ? ContainerResult
    : S extends undefined
        ? never
        : S extends ContainerArgs | ContainerFindManyArgs
            ? 'include' extends U
                ? SelectSubset<Container, S> & {
                    [P in ExistKeys<S['include']>]:
                    never
                }
                : SelectSubset<ContainerResult, S>
            : ContainerResult

export type GetContainerAggregateType<T extends ContainerAggregateArgs> = {
    [P in keyof T & keyof ContainerAggregateResult]: P extends '_count' | 'count'
  ? T[P] extends true
    ? number
    : GetScalarType<T[P], ContainerAggregateResult[P]>
  : GetScalarType<T[P], ContainerAggregateResult[P]>
}

export type GetContainerGroupByPayload<T extends ContainerGroupByArgs> =
  Array<
    PickEnumerable<ContainerGroupByResult, T['by']> &
      {
        [P in ((keyof T) & (keyof ContainerGroupByResult))]: P extends '_count'
          ? T[P] extends boolean
            ? number
            : GetScalarType<T[P], ContainerGroupByResult[P]>
          : GetScalarType<T[P], ContainerGroupByResult[P]>
      }
    >


/**
 * **Container count aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerCountAggregateResult = {
    
    /**
     * **All**
     *
     * This synthesized field doesn't have a description.
     */
     _all?: number
    
    /**
     * **Bool**
     *
     * This synthesized field doesn't have a description.
     */
     bool?: number
    
    /**
     * **Bool Array**
     *
     * This synthesized field doesn't have a description.
     */
     boolArray?: number
    
    /**
     * **Date**
     *
     * This synthesized field doesn't have a description.
     */
     date?: number
    
    /**
     * **Date Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateArray?: number
    
    /**
     * **Date Time**
     *
     * This synthesized field doesn't have a description.
     */
     dateTime?: number
    
    /**
     * **Date Time Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateTimeArray?: number
    
    /**
     * **Decimal**
     *
     * This synthesized field doesn't have a description.
     */
     decimal?: number
    
    /**
     * **Decimal Array**
     *
     * This synthesized field doesn't have a description.
     */
     decimalArray?: number
    
    /**
     * **Float32**
     *
     * This synthesized field doesn't have a description.
     */
     float32?: number
    
    /**
     * **Float32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float32Array?: number
    
    /**
     * **Float64**
     *
     * This synthesized field doesn't have a description.
     */
     float64?: number
    
    /**
     * **Float64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float64Array?: number
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Int32**
     *
     * This synthesized field doesn't have a description.
     */
     int32?: number
    
    /**
     * **Int32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int32Array?: number
    
    /**
     * **Int64**
     *
     * This synthesized field doesn't have a description.
     */
     int64?: number
    
    /**
     * **Int64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int64Array?: number
    
    /**
     * **Message**
     *
     * This synthesized field doesn't have a description.
     */
     message?: number
    
    /**
     * **Status**
     *
     * This synthesized field doesn't have a description.
     */
     status?: number
    
    /**
     * **Status Array**
     *
     * This synthesized field doesn't have a description.
     */
     statusArray?: number
    
    /**
     * **String**
     *
     * This synthesized field doesn't have a description.
     */
     string?: number
    
    /**
     * **String Array**
     *
     * This synthesized field doesn't have a description.
     */
     stringArray?: number
    
}


/**
 * **Container sum aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerSumAggregateResult = {
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
}


/**
 * **Container avg aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerAvgAggregateResult = {
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
}


/**
 * **Container min aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerMinAggregateResult = {
    
    /**
     * **Bool**
     *
     * This synthesized field doesn't have a description.
     */
     bool?: boolean
    
    /**
     * **Bool Array**
     *
     * This synthesized field doesn't have a description.
     */
     boolArray?: boolean[]
    
    /**
     * **Date**
     *
     * This synthesized field doesn't have a description.
     */
     date?: DateOnly
    
    /**
     * **Date Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateArray?: DateOnly[]
    
    /**
     * **Date Time**
     *
     * This synthesized field doesn't have a description.
     */
     dateTime?: Date
    
    /**
     * **Date Time Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateTimeArray?: Date[]
    
    /**
     * **Decimal**
     *
     * This synthesized field doesn't have a description.
     */
     decimal?: Decimal
    
    /**
     * **Decimal Array**
     *
     * This synthesized field doesn't have a description.
     */
     decimalArray?: Decimal[]
    
    /**
     * **Float32**
     *
     * This synthesized field doesn't have a description.
     */
     float32?: number
    
    /**
     * **Float32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float32Array?: number[]
    
    /**
     * **Float64**
     *
     * This synthesized field doesn't have a description.
     */
     float64?: number
    
    /**
     * **Float64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float64Array?: number[]
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Int32**
     *
     * This synthesized field doesn't have a description.
     */
     int32?: number
    
    /**
     * **Int32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int32Array?: number[]
    
    /**
     * **Int64**
     *
     * This synthesized field doesn't have a description.
     */
     int64?: number
    
    /**
     * **Int64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int64Array?: number[]
    
    /**
     * **Message**
     *
     * This synthesized field doesn't have a description.
     */
     message?: string
    
    /**
     * **Status**
     *
     * This synthesized field doesn't have a description.
     */
     status?: Status
    
    /**
     * **Status Array**
     *
     * This synthesized field doesn't have a description.
     */
     statusArray?: Status[]
    
    /**
     * **String**
     *
     * This synthesized field doesn't have a description.
     */
     string?: string
    
    /**
     * **String Array**
     *
     * This synthesized field doesn't have a description.
     */
     stringArray?: string[]
    
}


/**
 * **Container max aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerMaxAggregateResult = {
    
    /**
     * **Bool**
     *
     * This synthesized field doesn't have a description.
     */
     bool?: boolean
    
    /**
     * **Bool Array**
     *
     * This synthesized field doesn't have a description.
     */
     boolArray?: boolean[]
    
    /**
     * **Date**
     *
     * This synthesized field doesn't have a description.
     */
     date?: DateOnly
    
    /**
     * **Date Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateArray?: DateOnly[]
    
    /**
     * **Date Time**
     *
     * This synthesized field doesn't have a description.
     */
     dateTime?: Date
    
    /**
     * **Date Time Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateTimeArray?: Date[]
    
    /**
     * **Decimal**
     *
     * This synthesized field doesn't have a description.
     */
     decimal?: Decimal
    
    /**
     * **Decimal Array**
     *
     * This synthesized field doesn't have a description.
     */
     decimalArray?: Decimal[]
    
    /**
     * **Float32**
     *
     * This synthesized field doesn't have a description.
     */
     float32?: number
    
    /**
     * **Float32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float32Array?: number[]
    
    /**
     * **Float64**
     *
     * This synthesized field doesn't have a description.
     */
     float64?: number
    
    /**
     * **Float64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float64Array?: number[]
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Int32**
     *
     * This synthesized field doesn't have a description.
     */
     int32?: number
    
    /**
     * **Int32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int32Array?: number[]
    
    /**
     * **Int64**
     *
     * This synthesized field doesn't have a description.
     */
     int64?: number
    
    /**
     * **Int64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int64Array?: number[]
    
    /**
     * **Message**
     *
     * This synthesized field doesn't have a description.
     */
     message?: string
    
    /**
     * **Status**
     *
     * This synthesized field doesn't have a description.
     */
     status?: Status
    
    /**
     * **Status Array**
     *
     * This synthesized field doesn't have a description.
     */
     statusArray?: Status[]
    
    /**
     * **String**
     *
     * This synthesized field doesn't have a description.
     */
     string?: string
    
    /**
     * **String Array**
     *
     * This synthesized field doesn't have a description.
     */
     stringArray?: string[]
    
}


/**
 * **Container aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerAggregateResult = {
    
    /**
     * **Avg**
     *
     * This synthesized field doesn't have a description.
     */
     _avg?: ContainerAvgAggregateResult
    
    /**
     * **Count**
     *
     * This synthesized field doesn't have a description.
     */
     _count?: ContainerCountAggregateResult
    
    /**
     * **Max**
     *
     * This synthesized field doesn't have a description.
     */
     _max?: ContainerMaxAggregateResult
    
    /**
     * **Min**
     *
     * This synthesized field doesn't have a description.
     */
     _min?: ContainerMinAggregateResult
    
    /**
     * **Sum**
     *
     * This synthesized field doesn't have a description.
     */
     _sum?: ContainerSumAggregateResult
    
}


/**
 * **Container group by result**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerGroupByResult = {
    
    /**
     * **Avg**
     *
     * This synthesized field doesn't have a description.
     */
     _avg?: ContainerAvgAggregateResult
    
    /**
     * **Count**
     *
     * This synthesized field doesn't have a description.
     */
     _count?: ContainerCountAggregateResult
    
    /**
     * **Max**
     *
     * This synthesized field doesn't have a description.
     */
     _max?: ContainerMaxAggregateResult
    
    /**
     * **Min**
     *
     * This synthesized field doesn't have a description.
     */
     _min?: ContainerMinAggregateResult
    
    /**
     * **Sum**
     *
     * This synthesized field doesn't have a description.
     */
     _sum?: ContainerSumAggregateResult
    
    /**
     * **Bool**
     *
     * This synthesized field doesn't have a description.
     */
     bool?: boolean
    
    /**
     * **Bool Array**
     *
     * This synthesized field doesn't have a description.
     */
     boolArray?: boolean[]
    
    /**
     * **Date**
     *
     * This synthesized field doesn't have a description.
     */
     date?: DateOnly
    
    /**
     * **Date Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateArray?: DateOnly[]
    
    /**
     * **Date Time**
     *
     * This synthesized field doesn't have a description.
     */
     dateTime?: Date
    
    /**
     * **Date Time Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateTimeArray?: Date[]
    
    /**
     * **Decimal**
     *
     * This synthesized field doesn't have a description.
     */
     decimal?: Decimal
    
    /**
     * **Decimal Array**
     *
     * This synthesized field doesn't have a description.
     */
     decimalArray?: Decimal[]
    
    /**
     * **Float32**
     *
     * This synthesized field doesn't have a description.
     */
     float32?: number
    
    /**
     * **Float32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float32Array?: number[]
    
    /**
     * **Float64**
     *
     * This synthesized field doesn't have a description.
     */
     float64?: number
    
    /**
     * **Float64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float64Array?: number[]
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Int32**
     *
     * This synthesized field doesn't have a description.
     */
     int32?: number
    
    /**
     * **Int32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int32Array?: number[]
    
    /**
     * **Int64**
     *
     * This synthesized field doesn't have a description.
     */
     int64?: number
    
    /**
     * **Int64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int64Array?: number[]
    
    /**
     * **Message**
     *
     * This synthesized field doesn't have a description.
     */
     message?: string
    
    /**
     * **Status**
     *
     * This synthesized field doesn't have a description.
     */
     status?: Status
    
    /**
     * **Status Array**
     *
     * This synthesized field doesn't have a description.
     */
     statusArray?: Status[]
    
    /**
     * **String**
     *
     * This synthesized field doesn't have a description.
     */
     string?: string
    
    /**
     * **String Array**
     *
     * This synthesized field doesn't have a description.
     */
     stringArray?: string[]
    
}


/**
 * **Container args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: ContainerInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: ContainerSelect
    
}


/**
 * **Container find many args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerFindManyArgs = {
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: ContainerWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: ContainerSerializableScalarFields
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: ContainerInclude
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<ContainerOrderByInput>
    
    /**
     * **Page Number**
     *
     * This synthesized field doesn't have a description.
     */
     pageNumber?: number
    
    /**
     * **Page Size**
     *
     * This synthesized field doesn't have a description.
     */
     pageSize?: number
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: ContainerSelect
    
    /**
     * **Skip**
     *
     * This synthesized field doesn't have a description.
     */
     skip?: number
    
    /**
     * **Take**
     *
     * This synthesized field doesn't have a description.
     */
     take?: number
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where?: ContainerWhereInput
    
}


/**
 * **Container find first args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerFindFirstArgs = {
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: ContainerWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: ContainerSerializableScalarFields
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: ContainerInclude
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<ContainerOrderByInput>
    
    /**
     * **Page Number**
     *
     * This synthesized field doesn't have a description.
     */
     pageNumber?: number
    
    /**
     * **Page Size**
     *
     * This synthesized field doesn't have a description.
     */
     pageSize?: number
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: ContainerSelect
    
    /**
     * **Skip**
     *
     * This synthesized field doesn't have a description.
     */
     skip?: number
    
    /**
     * **Take**
     *
     * This synthesized field doesn't have a description.
     */
     take?: number
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where?: ContainerWhereInput
    
}


/**
 * **Container find unique args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerFindUniqueArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: ContainerInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: ContainerSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: ContainerWhereUniqueInput
    
}


/**
 * **Container create args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerCreateArgs = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: ContainerCreateInput
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: ContainerInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: ContainerSelect
    
}


/**
 * **Container update args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerUpdateArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: ContainerInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: ContainerSelect
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: ContainerUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: ContainerWhereUniqueInput
    
}


/**
 * **Container upsert args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerUpsertArgs = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: ContainerCreateInput
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: ContainerInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: ContainerSelect
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: ContainerUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: ContainerWhereUniqueInput
    
}


/**
 * **Container copy args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerCopyArgs = {
    
    /**
     * **Copy**
     *
     * This synthesized field doesn't have a description.
     */
     copy: ContainerUpdateInput
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: ContainerInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: ContainerSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: ContainerWhereUniqueInput
    
}


/**
 * **Container delete args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerDeleteArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: ContainerInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: ContainerSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: ContainerWhereUniqueInput
    
}


/**
 * **Container create many args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerCreateManyArgs = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: Enumerable<ContainerCreateInput>
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: ContainerInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: ContainerSelect
    
}


/**
 * **Container update many args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerUpdateManyArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: ContainerInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: ContainerSelect
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: ContainerUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: ContainerWhereInput
    
}


/**
 * **Container delete many args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerDeleteManyArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: ContainerInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: ContainerSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: ContainerWhereInput
    
}


/**
 * **Container copy many args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerCopyManyArgs = {
    
    /**
     * **Copy**
     *
     * This synthesized field doesn't have a description.
     */
     copy: ContainerUpdateInput
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: ContainerInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: ContainerSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: ContainerWhereInput
    
}


/**
 * **Container count args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerCountArgs = {
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: ContainerWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: ContainerSerializableScalarFields
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<ContainerOrderByInput>
    
    /**
     * **Page Number**
     *
     * This synthesized field doesn't have a description.
     */
     pageNumber?: number
    
    /**
     * **Page Size**
     *
     * This synthesized field doesn't have a description.
     */
     pageSize?: number
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: ContainerCountAggregateInputType
    
    /**
     * **Skip**
     *
     * This synthesized field doesn't have a description.
     */
     skip?: number
    
    /**
     * **Take**
     *
     * This synthesized field doesn't have a description.
     */
     take?: number
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where?: ContainerWhereInput
    
}


/**
 * **Container aggregate args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerAggregateArgs = {
    
    /**
     * **Avg**
     *
     * This synthesized field doesn't have a description.
     */
     _avg?: ContainerAvgAggregateInputType
    
    /**
     * **Count**
     *
     * This synthesized field doesn't have a description.
     */
     _count?: ContainerCountAggregateInputType
    
    /**
     * **Max**
     *
     * This synthesized field doesn't have a description.
     */
     _max?: ContainerMaxAggregateInputType
    
    /**
     * **Min**
     *
     * This synthesized field doesn't have a description.
     */
     _min?: ContainerMinAggregateInputType
    
    /**
     * **Sum**
     *
     * This synthesized field doesn't have a description.
     */
     _sum?: ContainerSumAggregateInputType
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: ContainerWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: ContainerSerializableScalarFields
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<ContainerOrderByInput>
    
    /**
     * **Page Number**
     *
     * This synthesized field doesn't have a description.
     */
     pageNumber?: number
    
    /**
     * **Page Size**
     *
     * This synthesized field doesn't have a description.
     */
     pageSize?: number
    
    /**
     * **Skip**
     *
     * This synthesized field doesn't have a description.
     */
     skip?: number
    
    /**
     * **Take**
     *
     * This synthesized field doesn't have a description.
     */
     take?: number
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where?: ContainerWhereInput
    
}


/**
 * **Container group by args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerGroupByArgs = {
    
    /**
     * **Avg**
     *
     * This synthesized field doesn't have a description.
     */
     _avg?: ContainerAvgAggregateInputType
    
    /**
     * **Count**
     *
     * This synthesized field doesn't have a description.
     */
     _count?: ContainerCountAggregateInputType
    
    /**
     * **Max**
     *
     * This synthesized field doesn't have a description.
     */
     _max?: ContainerMaxAggregateInputType
    
    /**
     * **Min**
     *
     * This synthesized field doesn't have a description.
     */
     _min?: ContainerMinAggregateInputType
    
    /**
     * **Sum**
     *
     * This synthesized field doesn't have a description.
     */
     _sum?: ContainerSumAggregateInputType
    
    /**
     * **By**
     *
     * This synthesized field doesn't have a description.
     */
     by: Enumerable<ContainerSerializableScalarFields>
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: ContainerWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: ContainerSerializableScalarFields
    
    /**
     * **Having**
     *
     * This synthesized field doesn't have a description.
     */
     having?: ContainerScalarWhereWithAggregatesInput
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<ContainerOrderByInput>
    
    /**
     * **Page Number**
     *
     * This synthesized field doesn't have a description.
     */
     pageNumber?: number
    
    /**
     * **Page Size**
     *
     * This synthesized field doesn't have a description.
     */
     pageSize?: number
    
    /**
     * **Skip**
     *
     * This synthesized field doesn't have a description.
     */
     skip?: number
    
    /**
     * **Take**
     *
     * This synthesized field doesn't have a description.
     */
     take?: number
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where?: ContainerWhereInput
    
}


/**
 * **Container scalar update input**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerScalarUpdateInput = {
    
    /**
     * **Bool**
     *
     * This synthesized field doesn't have a description.
     */
     bool?: boolean
    
    /**
     * **Bool Array**
     *
     * This synthesized field doesn't have a description.
     */
     boolArray?: boolean[]
    
    /**
     * **Date**
     *
     * This synthesized field doesn't have a description.
     */
     date?: DateOnly
    
    /**
     * **Date Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateArray?: DateOnly[]
    
    /**
     * **Date Time**
     *
     * This synthesized field doesn't have a description.
     */
     dateTime?: Date
    
    /**
     * **Date Time Array**
     *
     * This synthesized field doesn't have a description.
     */
     dateTimeArray?: Date[]
    
    /**
     * **Decimal**
     *
     * This synthesized field doesn't have a description.
     */
     decimal?: Decimal
    
    /**
     * **Decimal Array**
     *
     * This synthesized field doesn't have a description.
     */
     decimalArray?: Decimal[]
    
    /**
     * **Float32**
     *
     * This synthesized field doesn't have a description.
     */
     float32?: number
    
    /**
     * **Float32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float32Array?: number[]
    
    /**
     * **Float64**
     *
     * This synthesized field doesn't have a description.
     */
     float64?: number
    
    /**
     * **Float64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     float64Array?: number[]
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Int32**
     *
     * This synthesized field doesn't have a description.
     */
     int32?: number
    
    /**
     * **Int32 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int32Array?: number[]
    
    /**
     * **Int64**
     *
     * This synthesized field doesn't have a description.
     */
     int64?: number
    
    /**
     * **Int64 Array**
     *
     * This synthesized field doesn't have a description.
     */
     int64Array?: number[]
    
    /**
     * **Message**
     *
     * This synthesized field doesn't have a description.
     */
     message?: string
    
    /**
     * **Status**
     *
     * This synthesized field doesn't have a description.
     */
     status?: Status
    
    /**
     * **Status Array**
     *
     * This synthesized field doesn't have a description.
     */
     statusArray?: Status[]
    
    /**
     * **String**
     *
     * This synthesized field doesn't have a description.
     */
     string?: string
    
    /**
     * **String Array**
     *
     * This synthesized field doesn't have a description.
     */
     stringArray?: string[]
    
}


/**
 * **Container sign in checker ids**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerSignInCheckerIds = {
    
}


/**
 * **Container sign in checker companions**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerSignInCheckerCompanions = {
    
}


/**
 * **Container sign in input**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerSignInInput = {
    
    /**
     * **Credentials**
     *
     * This synthesized field doesn't have a description.
     */
     credentials: ContainerSignInArgs
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: ContainerInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: ContainerSelect
    
}


/**
 * **Container sign in args**
 *
 * This synthesized interface doesn't have a description
 */
export type ContainerSignInArgs = {
    
}



export namespace std {


    /**
     * **Sort Order**
     *
     * Represents the sort order
     */
    export type Sort = "asc" | "desc"

    /**
     * **String Match Mode**
     *
     * Whether the string query is case sensitive or not
     */
    export type StringMatchMode = "default" | "caseInsensitive"

    /// ## Sort Order
    ///
    /// Represents the sort order
    export const enum SortEnumType {

        /// ### Asc
        ///
        /// This enum member doesn't have a description.
        asc = "asc",

        /// ### Desc
        ///
        /// This enum member doesn't have a description.
        desc = "desc",
    }

    /// ## String Match Mode
    ///
    /// Whether the string query is case sensitive or not
    export const enum StringMatchModeEnumType {

        /// ### Default
        ///
        /// This enum member doesn't have a description.
        default = "default",

        /// ### Case insensitive
        ///
        /// This enum member doesn't have a description.
        caseInsensitive = "caseInsensitive",
    }


    /**
     * **Empty**
     *
     * The empty interface
     */
    export type Empty = {
        
    }


    /**
     * **Data**
     *
     * This interface is common for action output
     */
    export type Data<T> = {
        
        /**
         * **Data**
         *
         * This interface field doesn't have a description.
         */
         data: T
        
    }


    /**
     * **Data and Meta**
     *
     * This interface is common for action output with meta information
     */
    export type DataMeta<T, U> = {
        
        /**
         * **Data**
         *
         * This interface field doesn't have a description.
         */
         data: T
        
        /**
         * **Meta**
         *
         * This interface field doesn't have a description.
         */
         meta: U
        
    }


    /**
     * **Paging info**
     *
     * This interface doesn't have a description.
     */
    export type PagingInfo = {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         count: number
        
        /**
         * **Number of pages**
         *
         * This interface field doesn't have a description.
         */
         numberOfPages?: number
        
    }


    /**
     * **Response error**
     *
     * This interface doesn't have a description.
     */
    export type ResponseError = {
        
        /**
         * **Type**
         *
         * This interface field doesn't have a description.
         */
         type: string
        
        /**
         * **Message**
         *
         * This interface field doesn't have a description.
         */
         message: string
        
        /**
         * **Fields**
         *
         * This interface field doesn't have a description.
         */
         fields: {[key: string]: string} | null
        
    }


    /**
     * **Bool filter**
     *
     * This interface doesn't have a description.
     */
    export type BoolFilter = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: boolean
        
        /**
         * **Not**
         *
         * This interface field doesn't have a description.
         */
         not?: boolean | std.BoolFilter
        
    }


    /**
     * **Bool nullable filter**
     *
     * This interface doesn't have a description.
     */
    export type BoolNullableFilter = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: boolean | null
        
        /**
         * **Not**
         *
         * This interface field doesn't have a description.
         */
         not?: boolean | null | std.BoolNullableFilter
        
    }


    /**
     * **Filter**
     *
     * This interface doesn't have a description.
     */
    export type Filter<T> = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: T
        
        /**
         * **In**
         *
         * This interface field doesn't have a description.
         */
         in?: T[]
        
        /**
         * **Not in**
         *
         * This interface field doesn't have a description.
         */
         notIn?: T[]
        
        /**
         * **Lt**
         *
         * This interface field doesn't have a description.
         */
         lt?: T
        
        /**
         * **Lte**
         *
         * This interface field doesn't have a description.
         */
         lte?: T
        
        /**
         * **Gt**
         *
         * This interface field doesn't have a description.
         */
         gt?: T
        
        /**
         * **Gte**
         *
         * This interface field doesn't have a description.
         */
         gte?: T
        
        /**
         * **Not**
         *
         * This interface field doesn't have a description.
         */
         not?: T | std.Filter<T>
        
    }


    /**
     * **Nullable filter**
     *
     * This interface doesn't have a description.
     */
    export type NullableFilter<T> = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: T | null
        
        /**
         * **In**
         *
         * This interface field doesn't have a description.
         */
         in?: (T | null)[]
        
        /**
         * **Not in**
         *
         * This interface field doesn't have a description.
         */
         notIn?: (T | null)[]
        
        /**
         * **Lt**
         *
         * This interface field doesn't have a description.
         */
         lt?: T
        
        /**
         * **Lte**
         *
         * This interface field doesn't have a description.
         */
         lte?: T
        
        /**
         * **Gt**
         *
         * This interface field doesn't have a description.
         */
         gt?: T
        
        /**
         * **Gte**
         *
         * This interface field doesn't have a description.
         */
         gte?: T
        
        /**
         * **Not**
         *
         * This interface field doesn't have a description.
         */
         not?: T | null | std.NullableFilter<T>
        
    }


    /**
     * **String filter**
     *
     * This interface doesn't have a description.
     */
    export type StringFilter = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: string
        
        /**
         * **In**
         *
         * This interface field doesn't have a description.
         */
         in?: string[]
        
        /**
         * **Not in**
         *
         * This interface field doesn't have a description.
         */
         notIn?: string[]
        
        /**
         * **Lt**
         *
         * This interface field doesn't have a description.
         */
         lt?: string
        
        /**
         * **Lte**
         *
         * This interface field doesn't have a description.
         */
         lte?: string
        
        /**
         * **Gt**
         *
         * This interface field doesn't have a description.
         */
         gt?: string
        
        /**
         * **Gte**
         *
         * This interface field doesn't have a description.
         */
         gte?: string
        
        /**
         * **Contains**
         *
         * This interface field doesn't have a description.
         */
         contains?: string
        
        /**
         * **Starts with**
         *
         * This interface field doesn't have a description.
         */
         startsWith?: string
        
        /**
         * **Ends with**
         *
         * This interface field doesn't have a description.
         */
         endsWith?: string
        
        /**
         * **Matches**
         *
         * This interface field doesn't have a description.
         */
         matches?: string
        
        /**
         * **Mode**
         *
         * This interface field doesn't have a description.
         */
         mode?: std.StringMatchMode
        
        /**
         * **Not**
         *
         * This interface field doesn't have a description.
         */
         not?: string | std.StringFilter
        
    }


    /**
     * **String nullable filter**
     *
     * This interface doesn't have a description.
     */
    export type StringNullableFilter = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: string | null
        
        /**
         * **In**
         *
         * This interface field doesn't have a description.
         */
         in?: (string | null)[]
        
        /**
         * **Not in**
         *
         * This interface field doesn't have a description.
         */
         notIn?: (string | null)[]
        
        /**
         * **Lt**
         *
         * This interface field doesn't have a description.
         */
         lt?: string
        
        /**
         * **Lte**
         *
         * This interface field doesn't have a description.
         */
         lte?: string
        
        /**
         * **Gt**
         *
         * This interface field doesn't have a description.
         */
         gt?: string
        
        /**
         * **Gte**
         *
         * This interface field doesn't have a description.
         */
         gte?: string
        
        /**
         * **Contains**
         *
         * This interface field doesn't have a description.
         */
         contains?: string
        
        /**
         * **Starts with**
         *
         * This interface field doesn't have a description.
         */
         startsWith?: string
        
        /**
         * **Ends with**
         *
         * This interface field doesn't have a description.
         */
         endsWith?: string
        
        /**
         * **Matches**
         *
         * This interface field doesn't have a description.
         */
         matches?: string
        
        /**
         * **Mode**
         *
         * This interface field doesn't have a description.
         */
         mode?: std.StringMatchMode
        
        /**
         * **Not**
         *
         * This interface field doesn't have a description.
         */
         not?: string | null | std.StringNullableFilter
        
    }


    /**
     * **Enum filter**
     *
     * This interface doesn't have a description.
     */
    export type EnumFilter<T> = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: T
        
        /**
         * **In**
         *
         * This interface field doesn't have a description.
         */
         in?: T[]
        
        /**
         * **Not in**
         *
         * This interface field doesn't have a description.
         */
         notIn?: T[]
        
        /**
         * **Not**
         *
         * This interface field doesn't have a description.
         */
         not?: T | std.EnumFilter<T>
        
    }


    /**
     * **Enum nullable filter**
     *
     * This interface doesn't have a description.
     */
    export type EnumNullableFilter<T> = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: T | null
        
        /**
         * **In**
         *
         * This interface field doesn't have a description.
         */
         in?: (T | null)[]
        
        /**
         * **Not in**
         *
         * This interface field doesn't have a description.
         */
         notIn?: (T | null)[]
        
        /**
         * **Not**
         *
         * This interface field doesn't have a description.
         */
         not?: T | null | std.EnumNullableFilter<T>
        
    }


    /**
     * **Array filter**
     *
     * This interface doesn't have a description.
     */
    export type ArrayFilter<T> = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: T[]
        
        /**
         * **Has**
         *
         * This interface field doesn't have a description.
         */
         has?: T
        
        /**
         * **Has some**
         *
         * This interface field doesn't have a description.
         */
         hasSome?: T[]
        
        /**
         * **Has every**
         *
         * This interface field doesn't have a description.
         */
         hasEvery?: T[]
        
        /**
         * **Is empty**
         *
         * This interface field doesn't have a description.
         */
         isEmpty?: boolean
        
        /**
         * **Length**
         *
         * This interface field doesn't have a description.
         */
         length?: number
        
    }


    /**
     * **Array nullable filter**
     *
     * This interface doesn't have a description.
     */
    export type ArrayNullableFilter<T> = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: T[] | null
        
        /**
         * **Has**
         *
         * This interface field doesn't have a description.
         */
         has?: T
        
        /**
         * **Has some**
         *
         * This interface field doesn't have a description.
         */
         hasSome?: T[]
        
        /**
         * **Has every**
         *
         * This interface field doesn't have a description.
         */
         hasEvery?: T[]
        
        /**
         * **Is empty**
         *
         * This interface field doesn't have a description.
         */
         isEmpty?: boolean
        
        /**
         * **Length**
         *
         * This interface field doesn't have a description.
         */
         length?: number
        
    }


    /**
     * **Bool with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type BoolWithAggregatesFilter = std.BoolFilter & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.BoolFilter
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.BoolFilter
        
    }


    /**
     * **Bool nullable with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type BoolNullableWithAggregatesFilter = std.BoolNullableFilter & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.BoolNullableFilter
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.BoolNullableFilter
        
    }


    /**
     * **Int number with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type IntNumberWithAggregatesFilter<T> = std.Filter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.Filter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.Filter<T>
        
        /**
         * **Avg**
         *
         * This interface field doesn't have a description.
         */
         _avg?: std.Filter<number>
        
        /**
         * **Sum**
         *
         * This interface field doesn't have a description.
         */
         _sum?: std.Filter<number>
        
    }


    /**
     * **Int number nullable with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type IntNumberNullableWithAggregatesFilter<T> = std.NullableFilter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.NullableFilter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.NullableFilter<T>
        
        /**
         * **Avg**
         *
         * This interface field doesn't have a description.
         */
         _avg?: std.NullableFilter<number>
        
        /**
         * **Sum**
         *
         * This interface field doesn't have a description.
         */
         _sum?: std.NullableFilter<number>
        
    }


    /**
     * **Float number with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type FloatNumberWithAggregatesFilter<T> = std.Filter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.Filter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.Filter<T>
        
        /**
         * **Avg**
         *
         * This interface field doesn't have a description.
         */
         _avg?: std.Filter<number>
        
        /**
         * **Sum**
         *
         * This interface field doesn't have a description.
         */
         _sum?: std.Filter<number>
        
    }


    /**
     * **Float number nullable with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type FloatNumberNullableWithAggregatesFilter<T> = std.NullableFilter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.NullableFilter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.NullableFilter<T>
        
        /**
         * **Avg**
         *
         * This interface field doesn't have a description.
         */
         _avg?: std.NullableFilter<number>
        
        /**
         * **Sum**
         *
         * This interface field doesn't have a description.
         */
         _sum?: std.NullableFilter<number>
        
    }


    /**
     * **Decimal with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type DecimalWithAggregatesFilter = std.Filter<Decimal> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.Filter<Decimal>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.Filter<Decimal>
        
        /**
         * **Avg**
         *
         * This interface field doesn't have a description.
         */
         _avg?: std.Filter<Decimal>
        
        /**
         * **Sum**
         *
         * This interface field doesn't have a description.
         */
         _sum?: std.Filter<Decimal>
        
    }


    /**
     * **Decimal nullable with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type DecimalNullableWithAggregatesFilter<T> = std.NullableFilter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.NullableFilter<Decimal>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.NullableFilter<Decimal>
        
        /**
         * **Avg**
         *
         * This interface field doesn't have a description.
         */
         _avg?: std.NullableFilter<Decimal>
        
        /**
         * **Sum**
         *
         * This interface field doesn't have a description.
         */
         _sum?: std.NullableFilter<Decimal>
        
    }


    /**
     * **Aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type AggregatesFilter<T> = std.Filter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.Filter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.Filter<T>
        
    }


    /**
     * **Nullable aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type NullableAggregatesFilter<T> = std.NullableFilter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.NullableFilter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.NullableFilter<T>
        
    }


    /**
     * **String with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type StringWithAggregatesFilter = std.StringFilter & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.StringFilter
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.StringFilter
        
    }


    /**
     * **String nullable with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type StringNullableWithAggregatesFilter = std.StringNullableFilter & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.StringNullableFilter
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.StringNullableFilter
        
    }


    /**
     * **Enum with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type EnumWithAggregatesFilter<T> = std.EnumFilter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.EnumFilter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.EnumFilter<T>
        
    }


    /**
     * **Enum nullable with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type EnumNullableWithAggregatesFilter<T> = std.EnumNullableFilter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.EnumNullableFilter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.EnumNullableFilter<T>
        
    }


    /**
     * **Array with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type ArrayWithAggregatesFilter<T> = std.ArrayFilter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.ArrayFilter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.ArrayFilter<T>
        
    }


    /**
     * **Array nullable with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type ArrayNullableWithAggregatesFilter<T> = std.ArrayNullableFilter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.ArrayNullableFilter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.ArrayNullableFilter<T>
        
    }


    /**
     * **Number atomic update operation input**
     *
     * This interface doesn't have a description.
     */
    export type NumberAtomicUpdateOperationInput<T> = {
        
        /**
         * **Increment**
         *
         * This interface field doesn't have a description.
         */
         increment?: T
        
        /**
         * **Decrement**
         *
         * This interface field doesn't have a description.
         */
         decrement?: T
        
        /**
         * **Multiply**
         *
         * This interface field doesn't have a description.
         */
         multiply?: T
        
        /**
         * **Divide**
         *
         * This interface field doesn't have a description.
         */
         divide?: T
        
    }


    /**
     * **Array atomic update operation input**
     *
     * This interface doesn't have a description.
     */
    export type ArrayAtomicUpdateOperationInput<T> = {
        
        /**
         * **Push**
         *
         * This interface field doesn't have a description.
         */
         push?: T
        
    }



    export namespace admin {










        declare class AdminNamespace {

        }

    }

    export namespace bcrypt {










        declare class BcryptNamespace {

        }

    }

    export namespace identity {



        /**
         * **Token info**
         *
         * This interface doesn't have a description.
         */
        export type TokenInfo = {
            
            /**
             * **Token**
             *
             * This interface field doesn't have a description.
             */
             token: string
            
        }









        declare class IdentityNamespace {

        }

    }










}






export class ContainerModel {
    findManyObjects(query: ContainerFindManyArgs): Promise<Container[]>
    findUniqueObject(query: ContainerFindUniqueArgs): Promise<Container | null>
    findFirstObject(query: ContainerFindManyArgs): Promise<Container | null>
    createObject(input?: ContainerCreateInput): Promise<Container>
    count(input?: ContainerCountArgs): Promise<number>
    aggregate<T extends ContainerAggregateArgs>(input?: Subset<T, ContainerAggregateArgs>): Promise<GetContainerAggregateType<T>>
    groupBy<T extends ContainerGroupByArgs,
      HasSelectOrTake extends Or<
        Extends<'skip', Keys<T>>,
        Extends<'take', Keys<T>>
      >,
      OrderByArg extends True extends HasSelectOrTake
        ? { orderBy: ContainerGroupByArgs['orderBy'] }
        : { orderBy?: ContainerGroupByArgs['orderBy'] },
      OrderFields extends ExcludeUnderscoreKeys<Keys<MaybeTupleToUnion<T['orderBy']>>>,
      ByFields extends MaybeTupleToUnion<T['by']>,
      ByValid extends Has<ByFields, OrderFields>,
      HavingFields extends GetHavingFields<T['having']>,
      HavingValid extends Has<ByFields, HavingFields>,
      ByEmpty extends T['by'] extends never[] ? True : False,
      InputErrors extends ByEmpty extends True
      ? `Error: "by" must not be empty.`
      : HavingValid extends False
      ? {
          [P in HavingFields]: P extends ByFields
            ? never
            : P extends string
            ? `Error: Field "${P}" used in "having" needs to be provided in "by".`
            : [
                Error,
                'Field ',
                P,
                ` in "having" needs to be provided in "by"`,
              ]
        }[HavingFields]
      : 'take' extends Keys<T>
      ? 'orderBy' extends Keys<T>
        ? ByValid extends True
          ? {}
          : {
              [P in OrderFields]: P extends ByFields
                ? never
                : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
            }[OrderFields]
        : 'Error: If you provide "take", you also need to provide "orderBy"'
      : 'skip' extends Keys<T>
      ? 'orderBy' extends Keys<T>
        ? ByValid extends True
          ? {}
          : {
              [P in OrderFields]: P extends ByFields
                ? never
                : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
            }[OrderFields]
        : 'Error: If you provide "skip", you also need to provide "orderBy"'
      : ByValid extends True
      ? {}
      : {
          [P in OrderFields]: P extends ByFields
            ? never
            : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
        }[OrderFields]>(input: SubsetIntersection<T, ContainerGroupByArgs, OrderByArg> & InputErrors): Promise<{} extends InputErrors ? GetContainerGroupByPayload<T> : InputErrors>
    
    sql<T>(sql: string): Promise<T[]>
    
}

export class Container {
    get isNew(): boolean
    get isModified(): boolean
    set(input: ContainerUpdateInput): Promise<void>
    update(input: ContainerScalarUpdateInput): Promise<void>
    save(): Promise<void>
    delete(): Promise<void>
    toTeon(): Promise<ContainerResult>
    /// ## Id
    ///
    /// This field doesn't have a description.
    get id(): number

    /// ## Id
    ///
    /// This field doesn't have a description.
    set id(newValue: number)
    /// ## Int32
    ///
    /// This field doesn't have a description.
    get int32(): number | null

    /// ## Int32
    ///
    /// This field doesn't have a description.
    set int32(newValue: number | null)
    /// ## Int64
    ///
    /// This field doesn't have a description.
    get int64(): number | null

    /// ## Int64
    ///
    /// This field doesn't have a description.
    set int64(newValue: number | null)
    /// ## Float32
    ///
    /// This field doesn't have a description.
    get float32(): number | null

    /// ## Float32
    ///
    /// This field doesn't have a description.
    set float32(newValue: number | null)
    /// ## Float64
    ///
    /// This field doesn't have a description.
    get float64(): number | null

    /// ## Float64
    ///
    /// This field doesn't have a description.
    set float64(newValue: number | null)
    /// ## Bool
    ///
    /// This field doesn't have a description.
    get bool(): boolean | null

    /// ## Bool
    ///
    /// This field doesn't have a description.
    set bool(newValue: boolean | null)
    /// ## String
    ///
    /// This field doesn't have a description.
    get string(): string | null

    /// ## String
    ///
    /// This field doesn't have a description.
    set string(newValue: string | null)
    /// ## Date
    ///
    /// This field doesn't have a description.
    get date(): DateOnly | null

    /// ## Date
    ///
    /// This field doesn't have a description.
    set date(newValue: DateOnly | null)
    /// ## Date time
    ///
    /// This field doesn't have a description.
    get dateTime(): Date | null

    /// ## Date time
    ///
    /// This field doesn't have a description.
    set dateTime(newValue: Date | null)
    /// ## Decimal
    ///
    /// This field doesn't have a description.
    get decimal(): Decimal | null

    /// ## Decimal
    ///
    /// This field doesn't have a description.
    set decimal(newValue: Decimal | null)
    /// ## Status
    ///
    /// This field doesn't have a description.
    get status(): Status | null

    /// ## Status
    ///
    /// This field doesn't have a description.
    set status(newValue: Status | null)
    /// ## Int32 array
    ///
    /// This field doesn't have a description.
    get int32Array(): number[] | null

    /// ## Int32 array
    ///
    /// This field doesn't have a description.
    set int32Array(newValue: number[] | null)
    /// ## Int64 array
    ///
    /// This field doesn't have a description.
    get int64Array(): number[] | null

    /// ## Int64 array
    ///
    /// This field doesn't have a description.
    set int64Array(newValue: number[] | null)
    /// ## Float32 array
    ///
    /// This field doesn't have a description.
    get float32Array(): number[] | null

    /// ## Float32 array
    ///
    /// This field doesn't have a description.
    set float32Array(newValue: number[] | null)
    /// ## Float64 array
    ///
    /// This field doesn't have a description.
    get float64Array(): number[] | null

    /// ## Float64 array
    ///
    /// This field doesn't have a description.
    set float64Array(newValue: number[] | null)
    /// ## Bool array
    ///
    /// This field doesn't have a description.
    get boolArray(): boolean[] | null

    /// ## Bool array
    ///
    /// This field doesn't have a description.
    set boolArray(newValue: boolean[] | null)
    /// ## String array
    ///
    /// This field doesn't have a description.
    get stringArray(): string[] | null

    /// ## String array
    ///
    /// This field doesn't have a description.
    set stringArray(newValue: string[] | null)
    /// ## Date array
    ///
    /// This field doesn't have a description.
    get dateArray(): DateOnly[] | null

    /// ## Date array
    ///
    /// This field doesn't have a description.
    set dateArray(newValue: DateOnly[] | null)
    /// ## Date time array
    ///
    /// This field doesn't have a description.
    get dateTimeArray(): Date[] | null

    /// ## Date time array
    ///
    /// This field doesn't have a description.
    set dateTimeArray(newValue: Date[] | null)
    /// ## Decimal array
    ///
    /// This field doesn't have a description.
    get decimalArray(): Decimal[] | null

    /// ## Decimal array
    ///
    /// This field doesn't have a description.
    set decimalArray(newValue: Decimal[] | null)
    /// ## Status array
    ///
    /// This field doesn't have a description.
    get statusArray(): Status[] | null

    /// ## Status array
    ///
    /// This field doesn't have a description.
    set statusArray(newValue: Status[] | null)
    /// ## Message
    ///
    /// This field doesn't have a description.
    get message(): string | null

    /// ## Message
    ///
    /// This field doesn't have a description.
    set message(newValue: string | null)
}



declare class Teo {

    transaction(callback: (teo: Teo) => Promise<void>): Promise<void>

    get container(): ContainerModel
}
