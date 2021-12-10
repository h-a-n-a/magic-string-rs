/* eslint-disable */

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
}
export class MagicString {
  constructor(originalStr: string)
  append(input: string): this
  prepend(input: string): this
  appendLeft(index: number, input: string): this
  appendRight(index: number, input: string): this
  prependLeft(index: number, input: string): this
  prependRight(index: number, input: string): this
  toJsonSourcemap(
    options?: GenerateDecodedMapOptions | undefined | null,
  ): string
  toUrlSourcemap(options?: GenerateDecodedMapOptions | undefined | null): string
  generateDecodedMap(
    options?: GenerateDecodedMapOptions | undefined | null,
  ): DecodedMap
  toString(): string
}
