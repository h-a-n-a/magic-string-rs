import assert from "assert"
import { MagicString } from ".."

describe("should generate a sourcemap",  () => {
  const s = new MagicString("abcdefghijkl")
  s.remove(3,9);

  const map = s.generateMap({
    file: "output.md",
    sourceRoot: "./",
    source: "input.md",
    includeContent: true
  })

  assert.equal(map.version, 3)
  assert.equal(map.file, "output.md")
  assert.deepEqual(map.sources, ["input.md"])
  assert.equal(map.sourceRoot, "./")
  assert.deepEqual(map.sourcesContent, ["abcdefghijkl"])
  assert.equal(map.mappings, "AAAA,GAAS")

  assert.equal(map.toString(), "{\"version\":3,\"mappings\":\"AAAA,GAAS\",\"names\":[],\"sources\":[\"input.md\"],\"sourcesContent\":[\"abcdefghijkl\"],\"file\":\"output.md\",\"sourceRoot\":\"./\"}")
  assert.equal(map.toUrl(), "data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJtYXBwaW5ncyI6IkFBQUEsR0FBUyIsIm5hbWVzIjpbXSwic291cmNlcyI6WyJpbnB1dC5tZCJdLCJzb3VyY2VzQ29udGVudCI6WyJhYmNkZWZnaGlqa2wiXSwiZmlsZSI6Im91dHB1dC5tZCIsInNvdXJjZVJvb3QiOiIuLyJ9")
})

describe("should generate a correct sourcemap for prepend content when hires equals to false", () =>  {
  const s = new MagicString("x\nq")

  s.prepend("y\n");

  const map = s.generateMap({
    includeContent: true
  })

  assert.equal(map.mappings, ";AAAA;AACA")
  assert.ok(map.sourcesContent[0])
})

describe("should correctly map inserted multi-lines content", () =>  {
  let s = new MagicString("function Foo () {}")

  s.overwrite(15, 16, "\n", {
    contentOnly: false
  })

  let map = s.generateMap({
    includeContent: true
  })

  assert.equal(map.toString(), "{\"version\":3,\"mappings\":\"AAAA;AAAgB\",\"names\":[],\"sources\":[null],\"sourcesContent\":[\"function Foo () {}\"],\"file\":null}")

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

  assert.equal(map.toString(), "{\"version\":3,\"mappings\":\"AAAA,eAAe;AAAA;AAAE\",\"names\":[],\"sources\":[null],\"sourcesContent\":[\"function Foo () {}\"],\"file\":null}")
})

describe("should correctly map inserted content", () =>  {
  const s = new MagicString("function Foo () {}")

  s.overwrite(9, 12, "Bar", {
    contentOnly: false
  })

  const map = s.generateMap({
    includeContent: true
  })

  assert.equal(map.toString(), "{\"version\":3,\"mappings\":\"AAAA,SAAS,GAAG\",\"names\":[],\"sources\":[null],\"sourcesContent\":[\"function Foo () {}\"],\"file\":null}")
})

describe("should correctly map to JSON", () =>  {
  const s = new MagicString("function Foo () {}")

  let map = s.generateMap({
    includeContent: true
  })

  assert.equal(map.sourcesContent[0], "function Foo () {}")
  assert.equal(map.file, null)
  assert.equal(map.sourceRoot, undefined)
  assert.equal(map.names.length, 0)
  assert.equal(map.sources[0], null)
  assert.equal(map.version, 3)
  assert.equal(map.mappings, "AAAA")
})

