const { MagicString: MagicStringNative } = require('./binding')

module.exports.MagicString = class MagicString extends MagicStringNative {
  generateDecodedMap(options) {
    return JSON.parse(super.generateDecodedMap(options))
  }
  generateMap(options) {
    const sourcemap = super.generateMap({
      file: null,
      source: null,
      sourceRoot: null,
      includeContent: false,
      ...options,
    })

    const str = super.toSourcemapString(sourcemap)
    const obj = JSON.parse(str)

    const toString = () => str
    const toUrl = () => super.toSourcemapUrl(sourcemap)

    Object.defineProperty(obj, 'toString', {
      value: toString,
    })
    Object.defineProperty(obj, 'toUrl', {
      value: toUrl,
    })

    return obj
  }
  toSourcemapString() {
    throw new Error(
      '[magic-string] This is an internal API, you may refer to `generateMap`',
    )
  }
  toSourcemapUrl() {
    throw new Error(
      '[magic-string] This is an internal API, you may refer to `generateMap`',
    )
  }
}

module.exports.default = module.exports.MagicString
