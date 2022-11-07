<br />

<h1 align="center">magic-string-rs</h1>

<p align="center">
<a href="https://github.com/h-a-n-a/magic-string-rs/actions/workflows/CI.yaml"><img src="https://github.com/h-a-n-a/magic-string-rs/actions/workflows/CI.yaml/badge.svg" alt="CI"></a>
<a href="https://crates.io/crates/magic_string"><img src="https://img.shields.io/crates/v/magic_string.svg?label=crates" alt="crates"></a>
<a href="https://www.npmjs.com/package/@napi-rs/magic-string"><img src="https://img.shields.io/npm/v/@napi-rs/magic-string?color=c95f8b&amp;label=NPM" alt="NPM version"></a>
</p>

<p align="center">
<a href="https://github.com/Rich-Harris/magic-string">MagicString</a> port for Node and modern browsers, also, for rust, of course.
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

## Performance

### Hardware info

```
Model Name:	MacBook Pro
Model Identifier:	MacBookPro17,1
Chip:	Apple M1
Total Number of Cores:	8 (4 performance and 4 efficiency)
Memory:	16 GB
```

### Benchmark

```
Running "overwrite" suite...
Progress: 100%

  MagicString:
    238 584 ops/s, ±0.34%   | slowest, 50.7% slower

  MagicStringRust:
    483 950 ops/s, ±2.13%   | fastest

Finished 2 cases!
  Fastest: MagicStringRust
  Slowest: MagicString

Running "prepend|append" suite...
Progress: 100%

  MagicString:
    290 244 ops/s, ±1.35%   | slowest, 48.35% slower

  MagicStringRust:
    561 981 ops/s, ±6.71%   | fastest

Finished 2 cases!
  Fastest: MagicStringRust
  Slowest: MagicString

Running "add banner#toString" suite...
Progress: 100%

  MagicString:
    301 467 ops/s, ±0.29%   | slowest, 37.66% slower

  MagicStringRust:
    483 586 ops/s, ±5.50%   | fastest

Finished 2 cases!
  Fastest: MagicStringRust
  Slowest: MagicString

Running "add banner#generateDecodedMap" suite...
Progress: 100%

  MagicString:
    233 702 ops/s, ±0.76%   | fastest

  MagicStringRust:
    229 899 ops/s, ±2.68%   | slowest, 1.63% slower

Finished 2 cases!
  Fastest: MagicString
  Slowest: MagicStringRust

Running "add banner#generateMapHires" suite...
Progress: 100%

  MagicString:
    177 783 ops/s, ±1.84%   | fastest

  MagicStringRust:
    90 780 ops/s, ±1.00%    | slowest, 48.94% slower

Finished 2 cases!
  Fastest: MagicString
  Slowest: MagicStringRust

Running "add banner#generateMap" suite...
Progress: 100%

  MagicString:
    227 594 ops/s, ±0.68%   | slowest, 0.42% slow
er

  MagicStringRust:
    228 545 ops/s, ±0.82%   | fastest

Finished 2 cases!
  Fastest: MagicStringRust
  Slowest: MagicString

Running "add banner#generateMap.toString" suite...
Progress: 100%

  MagicString:
    201 272 ops/s, ±0.47%   | slowest, 21.86% slower

  MagicStringRust:
    257 577 ops/s, ±2.38%   | fastest

Finished 2 cases!
  Fastest: MagicStringRust
  Slowest: MagicString

Running "add banner#generateMapHires.toString" suite...
Progress: 100%

  MagicString:
    157 685 ops/s, ±0.18%   | fastest

  MagicStringRust:
    95 510 ops/s, ±1.00%    | slowest, 39.43% slower

Finished 2 cases!
  Fastest: MagicString
  Slowest: MagicStringRust

Running "add banner#generateMap.toUrl" suite...
Progress: 100%

  MagicString:
    182 161 ops/s, ±0.65%   | slowest, 25.04% slower

  MagicStringRust:
    243 019 ops/s, ±0.98%   | fastest

Finished 2 cases!
  Fastest: MagicStringRust
  Slowest: MagicString
```

## 📃 Documentation

[doc.rs](https://docs.rs/magic_string/latest/magic_string)

## Supported APIs

- [x] generateMap: Note that there is a huge overhead for rust for implementing the same API in Node, for more detail please refer to [this](./node/index.d.ts)
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
