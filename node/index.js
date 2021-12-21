const { MagicString: MagicStringNative } = require('./binding')

module.exports.MagicString = class MagicString extends MagicStringNative {
  generateDecodedMap(options) {
    return JSON.parse(super.generateDecodedMap(options))
  }
  overwrite(start, end, content, options) {
    options = options || {
      contentOnly: false,
    }
    return super.overwrite(start, end, content, options)
  }
  generateMap(options) {
    const sourcemap = super.generateMap({
      file: null,
      source: null,
      sourceRoot: null,
      includeContent: false,
      hires: false,
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
  generateDecodedMap(options) {
    options = options || {
      file: null,
      source: null,
      sourceRoot: null,
      includeContent: false,
      hires: false,
      ...options,
    }

    return super.generateDecodedMap(options)
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

Object.assign(exports, '__esModule', {
  value: true,
})
module.exports.default = module.exports.MagicString
