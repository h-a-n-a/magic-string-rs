import test from "ava"

import { MagicString } from ".."

test("should generate a sourcemap",  t => {
  const s = new MagicString("abcdefghijkl")
  s.remove(3,9);

  const map = s.generateMap({
    file: "output.md",
    sourceRoot: "./",
    source: "input.md",
    includeContent: true
  })

  t.is(map.version, 3)
  t.is(map.file, "output.md")
  t.deepEqual(map.sources, ["input.md"])
  t.is(map.sourceRoot, "./")
  t.deepEqual(map.sourcesContent, ["abcdefghijkl"])
  t.is(map.mappings, "AAAA,GAAS")

  t.is(map.toString(), "{\"version\":3,\"mappings\":\"AAAA,GAAS\",\"names\":[],\"sources\":[\"input.md\"],\"sourcesContent\":[\"abcdefghijkl\"],\"file\":\"output.md\",\"sourceRoot\":\"./\"}")
  t.is(map.toUrl(), "data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJtYXBwaW5ncyI6IkFBQUEsR0FBUyIsIm5hbWVzIjpbXSwic291cmNlcyI6WyJpbnB1dC5tZCJdLCJzb3VyY2VzQ29udGVudCI6WyJhYmNkZWZnaGlqa2wiXSwiZmlsZSI6Im91dHB1dC5tZCIsInNvdXJjZVJvb3QiOiIuLyJ9")
})

test("should generate a correct sourcemap for prepend content when hires equals to false", t => {
  const s = new MagicString("x\nq")

  s.prepend("y\n");

  const map = s.generateMap({
    includeContent: true
  })

  t.is(map.mappings, ";AAAA;AACA")
  t.truthy(map.sourcesContent[0])
})

test("should correctly map inserted multi-lines content", t => {
  let s = new MagicString("function Foo () {}")

  s.overwrite(15, 16, "\n", {
    contentOnly: false
  })

  let map = s.generateMap({
    includeContent: true
  })

  t.is(map.toString(), "{\"version\":3,\"mappings\":\"AAAA;AAAgB\",\"names\":[],\"sources\":[null],\"sourcesContent\":[\"function Foo () {}\"],\"file\":null}")

  s = new MagicString("function Foo () {}")

  s.overwrite(
    15, 17, " {\n  console.log(\"bar\")\n",
    {
      contentOnly: false
    }
  )

  map = s.generateMap({
    includeContent: true
  })

  t.is(map.toString(), "{\"version\":3,\"mappings\":\"AAAA,eAAe;AAAA;AAAE\",\"names\":[],\"sources\":[null],\"sourcesContent\":[\"function Foo () {}\"],\"file\":null}")
})

test("should correctly map inserted content", t => {
  const s = new MagicString("function Foo () {}")

  s.overwrite(9, 12, "Bar", {
    contentOnly: false
  })

  const map = s.generateMap({
    includeContent: true
  })

  t.is(map.toString(), "{\"version\":3,\"mappings\":\"AAAA,SAAS,GAAG\",\"names\":[],\"sources\":[null],\"sourcesContent\":[\"function Foo () {}\"],\"file\":null}")
})

test("should correctly map to JSON", t => {
  const s = new MagicString("function Foo () {}")

  let map = s.generateMap({
    includeContent: true
  })

  t.is(map.sourcesContent[0], "function Foo () {}")
  t.is(map.file, null)
  t.is(map.sourceRoot, undefined)
  t.is(map.names.length, 0)
  t.is(map.sources[0], null)
  t.is(map.version, 3)
  t.is(map.mappings, "AAAA")
})

