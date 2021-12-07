import { MagicString, GenerateDecodedMapOptions } from './dist'

let s = new MagicString('abc')

s.append('def').appendLeft(1, '123').appendRight(1, '456')

console.log(s.toString())
console.log(s.generateMap().toJson())
