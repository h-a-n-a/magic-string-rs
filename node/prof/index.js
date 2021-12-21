const b = require("benny")

const { MagicString: MagicStringRust } = require("../index");
const MagicString = require("magic-string")


const BANNER = `/*!
* Vue.js v2.6.14
* (c) 2014-2021 Evan You
* Released under the MIT License.
*/
`

// for (let i = 0; i < 100000; i++) {
//     const s = new MagicString(BANNER)
//     s.prepend(BANNER)
//     s.generateMap().toString()
// }

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