import b from 'benny'
import MagicString from 'magic-string'

import { MagicString as MagicStringRust } from '../node'

const BANNER = `/*!
* Vue.js v2.6.14
* (c) 2014-2021 Evan You
* Released under the MIT License.
*/
`

const EXPORT_STATEMENT = `export default foo`

b.suite(
    'overwrite',
    b.add('MagicString', () => {
        const m = new MagicString(`export const foo = 'bar'`)
        m.overwrite(13, 16, "bar")
    }),
    b.add('MagicStringRust', () => {
        const m = new MagicStringRust(`export const foo = 'bar'`)
        m.overwrite(13, 16, "bar")
    }),
    b.cycle(),
    b.complete(),
)

b.suite(
    'prepend|append',
    b.add('MagicString', () => {
        const m = new MagicString(`export const foo = 'bar'`)
        m.prepend(BANNER)
        m.append(EXPORT_STATEMENT)
    }),
    b.add('MagicStringRust', () => {
        const m = new MagicStringRust(`export const foo = 'bar'`)
        m.prepend(BANNER)
        m.append(EXPORT_STATEMENT)
    }),
    b.cycle(),
    b.complete(),
)

b.suite(
    'add banner#toString',
    b.add('MagicString', () => {
        const m = new MagicString(`export const foo = 'bar'`)
        m.prepend(BANNER)
        m.toString()
    }),
    b.add('MagicStringRust', () => {
        const m = new MagicStringRust(`export const foo = 'bar'`)
        m.prepend(BANNER)
        m.toString()
    }),
    b.cycle(),
    b.complete(),
    )

b.suite(
  'add banner#generateDecodedMap',
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

b.suite(
    'add banner#generateMapHires',
    b.add('MagicString', () => {
        const m = new MagicString(`export const foo = 'bar'`)
        m.prepend(BANNER)
        m.generateMap({
            hires: true
        })
    }),
    b.add('MagicStringRust', () => {
        const m = new MagicStringRust(`export const foo = 'bar'`)
        m.prepend(BANNER)
        m.generateMap({
            hires: true
        }).toMap()
    }),
    b.cycle(),
    b.complete(),
)

b.suite(
    'add banner#generateMap',
    b.add('MagicString', () => {
        const m = new MagicString(`export const foo = 'bar'`)
        m.prepend(BANNER)
        m.generateMap()
    }),
    b.add('MagicStringRust', () => {
        const m = new MagicStringRust(`export const foo = 'bar'`)
        m.prepend(BANNER)
        m.generateMap().toMap()
    }),
    b.cycle(),
    b.complete(),
)

b.suite(
    'add banner#generateMap.toString',
    b.add('MagicString', () => {
        const m = new MagicString(`export const foo = 'bar'`)
        m.prepend(BANNER)
        m.generateMap().toString()
    }),
    b.add('MagicStringRust', () => {
        const m = new MagicStringRust(`export const foo = 'bar'`)
        m.prepend(BANNER)
        m.generateMap().toString()
    }),
    b.cycle(),
    b.complete(),
)

b.suite(
    'add banner#generateMapHires.toString',
    b.add('MagicString', () => {
        const m = new MagicString(`export const foo = 'bar'`)
        m.prepend(BANNER)
        m.generateMap({
            hires: true
        }).toString()
    }),
    b.add('MagicStringRust', () => {
        const m = new MagicStringRust(`export const foo = 'bar'`)
        m.prepend(BANNER)
        m.generateMap({
            hires: true
        }).toString()
    }),
    b.cycle(),
    b.complete(),
)

b.suite(
    'add banner#generateMap.toUrl',
    b.add('MagicString', () => {
        const m = new MagicString(`export const foo = 'bar'`)
        m.prepend(BANNER)
        m.generateMap().toUrl()
    }),
    b.add('MagicStringRust', () => {
        const m = new MagicStringRust(`export const foo = 'bar'`)
        m.prepend(BANNER)
        m.generateMap().toUrl()
    }),
    b.cycle(),
    b.complete(),
)

b.suite(
    'clone',
    b.add('MagicString', () => {
        const m = new MagicString(`export const foo = 'bar'`)
        m.clone()
    }),
    b.add('MagicStringRust', () => {
        const m = new MagicStringRust(`export const foo = 'bar'`)
        m.clone()
    }),
    b.cycle(),
    b.complete(),
)

b.suite(
    'snip',
    b.add('MagicString', () => {
        const m = new MagicString(`export const foo = 'bar'`)
        m.snip(3, 9)
    }),
    b.add('MagicStringRust', () => {
        const m = new MagicStringRust(`export const foo = 'bar'`)
        m.snip(3, 9)
    }),
    b.cycle(),
    b.complete(),
)
