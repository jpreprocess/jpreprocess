import { JPreprocess } from "example-wasm/example_wasm.js";
import { readFile } from "node:fs/promises";

(async () => {
  const jpreprocess = new JPreprocess({
    metadata: await readFile("min-dict/metadata.json", "utf-8").then((data) => JSON.parse(data)),
    dict_da: new Uint8Array(await readFile("min-dict/dict.da")),
    dict_vals: new Uint8Array(await readFile("min-dict/dict.vals")),
    cost_matrix: new Uint8Array(await readFile("min-dict/matrix.mtx")),
    char_definitions: new Uint8Array(await readFile("min-dict/char_def.bin")),
    unknown_dictionary: new Uint8Array(await readFile("min-dict/unk.bin")),
    words_idx_data: new Uint8Array(await readFile("min-dict/dict.wordsidx")),
    words_data: new Uint8Array(await readFile("min-dict/dict.words")),
  });

  const result = jpreprocess.run_frontend("音声合成エンジンに渡せる形式に変換します．");
  console.log(result);
})()
