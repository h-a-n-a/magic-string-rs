const { MagicString: MagicStringNative } = require('./binding')

const nativeOverwrite = MagicStringNative.prototype.overwrite
MagicStringNative.prototype.overwrite = function overwrite(start, end, content, options) {
  options = {
    contentOnly: false,
    ...options
  }
  nativeOverwrite.call(this, start, end, content, options)
  return this
}

MagicStringNative.prototype.generateMap = function generateMap(options) {
  options = {
    file: null,
    source: null,
    sourceRoot: null,
    includeContent: false,
    hires: false,
    ...options,
  }

  const toString = () => this.toSourcemapString(options)
  const toUrl = () => this.toSourcemapUrl(options)
  const toMap = () => JSON.parse(toString(options))

  return {
    toString,
    toUrl,
    toMap,
  }
}

const nativeGenerateDecodedMap = MagicStringNative.prototype.generateDecodedMap
MagicStringNative.prototype.generateDecodedMap = function generateDecodedMap(options) {
  options = {
    file: null,
    source: null,
    sourceRoot: null,
    includeContent: false,
    hires: false,
    ...options,
  }

  return JSON.parse(nativeGenerateDecodedMap.call(this, options))
}

module.exports.MagicString = class MagicString extends MagicStringNative {
}

Object.assign(exports, '__esModule', {
  value: true,
})
module.exports.default = module.exports.MagicString
