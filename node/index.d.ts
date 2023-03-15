/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
/** Only for .d.ts type generation */
export interface DecodedMap {
  file?: string | undefined | null
  sources: Array<string | undefined | null>
  sourceRoot?: string | undefined | null
  sourcesContent: Array<string | undefined | null>
  names: Array<string>
  mappings: Array<Array<Array<number>>>
}
/** Only for .d.ts generation */
export interface GenerateDecodedMapOptions {
  file?: string | undefined | null
  sourceRoot?: string | undefined | null
  source?: string | undefined | null
  includeContent: boolean
  hires: boolean
}
/** Only for .d.ts generation */
export interface OverwriteOptions {
  contentOnly: boolean
}
export class MagicString {
  constructor(originalStr: string)
  append(input: string): this
  prepend(input: string): this
  appendLeft(index: number, input: string): this
  appendRight(index: number, input: string): this
  prependLeft(index: number, input: string): this
  prependRight(index: number, input: string): this
  overwrite(
    start: number,
    end: number,
    content: string,
    options?: OverwriteOptions,
  ): this
  trim(pattern?: string | undefined | null): this
  trimStart(pattern?: string | undefined | null): this
  trimEnd(pattern?: string | undefined | null): this
  trimLines(): this
  remove(start: number, end: number): this
  move(start: number, end: number, index: number): this
  isEmpty(): boolean
  generateMap(options?: Partial<GenerateDecodedMapOptions>): {
    toString: () => string
    toUrl: () => string
    toMap: () => {
      version: number
      file?: string
      sources: string[]
      sourcesContent: string[]
      names: string[]
      mappings: string
      sourceRoot?: string
    }
  }
  generateDecodedMap(options?: Partial<GenerateDecodedMapOptions>): DecodedMap
  toString(): string
  length(): number
}
