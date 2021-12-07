import { getMagicString } from './utils'

export interface GenerateMapOptions {
  file?: string
  source?: string
  sourceRoot?: string
  includeContent?: boolean
}

export interface GenerateDecodedMapOptions extends GenerateMapOptions {}

export class MagicString {
  // TODO: fix type
  private _instance: any

  constructor(public originalStr: string) {
    const NativeMagicString = getMagicString()
    this._instance = new NativeMagicString(originalStr)
  }

  prepend(str: string): MagicString {
    this._instance.prepend(str)
    return this
  }

  append(str: string): MagicString {
    this._instance.append(str)
    return this
  }

  prependRight(index: number, str: string): MagicString {
    this._instance.prependRight(index, str)
    return this
  }

  prependLeft(index: number, str: string): MagicString {
    this._instance.prependLeft(index, str)
    return this
  }

  appendRight(index: number, str: string): MagicString {
    this._instance.appendRight(index, str)
    return this
  }

  appendLeft(index: number, str: string): MagicString {
    this._instance.appendLeft(index, str)
    return this
  }

  generateMap(options: GenerateMapOptions = {}) {
    const {
      file = '',
      source = '',
      sourceRoot = '',
      includeContent = false,
    } = options
    const toUrl = (): string => {
      return this._instance.toUrl(file, source, sourceRoot, includeContent)
    }

    const toJson = (): string => {
      return this._instance.toJson(file, source, sourceRoot, includeContent)
    }

    return {
      toUrl,
      toJson,
    }
  }

  // generateDecodedMap(options: GenerateDecodedMapOptions = {}) {
  // }

  toString(): string {
    return this._instance.toString()
  }
}
