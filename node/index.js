const { loadBinding } = require('@node-rs/helper')

/**
 * __dirname means load native addon from current dir
 * 'magic-string-rs' means native addon name is `magic-string-rs`
 * the first arguments was decided by `napi.name` field in `package.json`
 * the second arguments was decided by `name` field in `package.json`
 * loadBinding helper will load `magic-string-rs.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `magic-string-rs-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'magic-string-rs', 'magic-string-rs')