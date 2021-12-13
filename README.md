<br />

<h1 align="center">magic-string-rs</h1>

<p align="center">
<a href="https://github.com/h-a-n-a/magic-string-rs/actions/workflows/CI.yaml"><img src="https://github.com/h-a-n-a/magic-string-rs/actions/workflows/CI.yaml/badge.svg" alt="CI"></a>
<a href="https://crates.io/crates/magic_string"><img src="https://img.shields.io/crates/v/magic_string.svg?label=crates" alt="crates"></a>
<a href="https://www.npmjs.com/package/@napi-rs/magic-string"><img src="https://img.shields.io/npm/v/@napi-rs/magic-string?color=c95f8b&amp;label=NPM" alt="NPM version"></a>
</p>

<p align="center">
100% API compatible (port) <a href="https://github.com/Rich-Harris/magic-string">MagicString by Rich-Harris</a> implementation for Node and modern browsers, also, for rust, of course.
</p>

<br />

## 🔧 Installation

### Rust

Add it as a dependency in a Cargo project.

```toml
# Cargo.toml

[dependency]
magic_string = "x.x.x"
```

### Node

```bash
$ npm install @napi-rs/magic-string
```

Note: Web-Assembly is currently not supported, but it's on the plan.


## Benchmark

```
Running "toString" suite...
Progress: 100%

  MagicString:
    226 614 ops/s, ±0.08%   | slowest, 42.65% slower

  MagicStringRust:
    395 166 ops/s, ±1.83%   | fastest

Finished 2 cases!
  Fastest: MagicStringRust
  Slowest: MagicString
  
Running "generateDecodedMap" suite...
Progress: 100%

  MagicString:
    173 590 ops/s, ±0.55%   | slowest, 18.84% slower

  MagicStringRust:
    213 893 ops/s, ±0.75%   | fastest

Finished 2 cases!
  Fastest: MagicStringRust
  Slowest: MagicString
  
Running "generateMap.toString" suite...
Progress: 100%

  MagicString:
    141 658 ops/s, ±0.53%   | slowest, 31.48% slower

  MagicStringRust:
    206 744 ops/s, ±0.64%   | fastest

Finished 2 cases!
  Fastest: MagicStringRust
  Slowest: MagicString
```



## Supported APIs

- [x] generateMap
- [x] generateDecodedMap
- [x] toString
- [x] prepend
- [x] append
- [x] prependLeft
- [x] prependRight
- [x] appendLeft
- [x] appendRight
- [x] overwrite
- [x] trim
- [x] trimStart
- [x] trimEnd
- [x] trimLines
- [x] isEmpty
- [x] remove
- [ ] move
- [ ] indent
- [ ] addSourcemapLocation
- [ ] clone
- [ ] slice
- [ ] snip


## Credits

The original project [magic-string](https://github.com/Rich-Harris/magic-string) is really awesome, you should check it out and we made this project even furthur for better performance.

## License

MIT
