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
      ...options
    })

    const toString = () => super.toSourcemapString(sourcemap)
    const toUrl = () => super.toSourcemapUrl(sourcemap)

    return {
      toString,
      toUrl,
    }
  }
  toSourcemapString() {
    throw new Error("[magic-string] This is an internal API, you may refer to `generateMap`")
  }
  toSourcemapUrl() {
    throw new Error("[magic-string] This is an internal API, you may refer to `generateMap`")
  }
}

module.exports.default = module.exports.MagicString
