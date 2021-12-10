const { MagicString: MagicStringNative } = require('./binding')

module.exports.MagicString = class MagicString extends MagicStringNative {
  generateDecodedMap(options) {
    return JSON.parse(super.generateDecodedMap(options))
  }
}

module.exports.default = module.exports.MagicString
