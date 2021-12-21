const { MagicString: MagicStringNative } = require('./binding')

module.exports.MagicString = class MagicString extends MagicStringNative {
  overwrite(start, end, content, options) {
    options = {
      contentOnly: false,
      ...options
    }
    return super.overwrite(start, end, content, options)
  }
  generateMap(options) {
    options = {
      file: null,
      source: null,
      sourceRoot: null,
      includeContent: false,
      hires: false,
      ...options,
    }

    const toString = () => super.toSourcemapString(options)
    const toUrl = () => super.toSourcemapUrl(options)
    const toMap = () => JSON.parse(toString(options))

    return {
      toString,
      toUrl,
      toMap,
    }
  }
  generateDecodedMap(options) {
    options = {
      file: null,
      source: null,
      sourceRoot: null,
      includeContent: false,
      hires: false,
      ...options,
    }

    return JSON.parse(super.generateDecodedMap(options))
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
