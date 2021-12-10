import b from 'benny'
import MagicString from 'magic-string'

import { MagicString as MagicStringRust } from '../node'

const BANNER = `/*!
* Vue.js v2.6.14
* (c) 2014-2021 Evan You
* Released under the MIT License.
*/
`

b.suite(
  'add banner',
  b.add('MagicString', () => {
    const m = new MagicString(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.generateDecodedMap()
  }),
  b.add('MagicStringRust', () => {
    const m = new MagicStringRust(`export const foo = 'bar'`)
    m.prepend(BANNER)
    m.generateDecodedMap()
  }),
  b.cycle(),
  b.complete(),
)
