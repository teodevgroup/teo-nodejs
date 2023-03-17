/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export class App {
  /** Create a Teo app. */
  constructor()
  /** @internal */
  static withCli(cli: boolean): App
  /** Run this app. */
  run(): Promise<void>
  /** Register a named transformer. */
  transform(callback: (input: any) => any | Promise<any>): void
  /** Register a named validator. */
  validate(callback: (input: any) => boolean | string | undefined | null | Promise<boolean | string | undefined | null>): void
  /** Register a named callback. */
  callback(callback: (input: any) => void | Promise<void>): void
  compare<T>(callback: (oldValue: T, newValue: T) => boolean | string | undefined | null | Promise<boolean | string | undefined | null>): void
}
