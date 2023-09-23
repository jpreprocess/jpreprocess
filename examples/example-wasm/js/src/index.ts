import { JPreprocess } from "example-wasm/example_wasm.js";
import { readFile } from "node:fs/promises";

(async () => {
  const jpreprocess = new JPreprocess({
    dict_da: await readFile("min-dict/dict.da"),
    dict_vals: await readFile("min-dict/dict.vals"),
    cost_matrix: await readFile("min-dict/matrix.mtx"),
    char_definitions: await readFile("min-dict/char_def.bin"),
    unknown_dictionary: await readFile("min-dict/unk.bin"),
    words_idx_data: await readFile("min-dict/dict.wordsidx"),
    words_data: await readFile("min-dict/dict.words"),
  });

  const result = jpreprocess.run_frontend("音声合成エンジンに渡せる形式に変換します．");
  console.log(result);
})()
