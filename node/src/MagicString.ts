import { getMagicString } from './utils'

export interface GenerateMapOptions {
  file?: string
  source?: string
  sourceRoot?: string
  includeContent?: boolean
}

interface NativeDecodedMap {
  // return empty string if not present
  file: string
  // return empty string if not present
  sourceRoot: string
  sources: string[]
  sourcesContent: string[]
  mappings: number[][][]
  names: string[]
}

interface DecodedMap {
  file: string
  sourceRoot: string
  sources: (string | null)[]
  sourcesContent?: (string | null)[]
  names: string[]
  mappings: number[][][]
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

  generateDecodedMap(options: GenerateDecodedMapOptions = {}): DecodedMap {
    const {
      file = '',
      source = '',
      sourceRoot = '',
      includeContent = false,
    } = options
    const decodedMap: NativeDecodedMap = this._instance.generateDecodedMap(
      file,
      source,
      sourceRoot,
      includeContent
    )

    decodedMap.sources = decodedMap.sources.map((source) => {
      return source.length ? source : null
    })
    decodedMap.sourcesContent = decodedMap.sourcesContent.map((content) => {
      return content.length ? content : null
    })

    return decodedMap
  }

  toString(): string {
    return this._instance.toString()
  }
}
