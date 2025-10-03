import { JPreprocess } from "example-wasm/example_wasm.js";
import { readFile } from "node:fs/promises";

function assertEq<T extends string>(arg1: string, arg2: T): asserts arg1 is T {
  if (arg1 !== arg2) {
    throw new Error(`Assert failed: "${arg1}" !== "${arg2}"`);
  }
}

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

  const njd = jpreprocess.run_frontend("音声合成エンジンに渡せる形式に変換します．");
  assertEq(njd[2], "エンジン,名詞,一般,*,*,*,*,エンジン,エンジン,エンジン,1/4,C1,1");

  const jpcommon = jpreprocess.make_label(njd);
  assertEq(jpcommon[2], "sil^o-N+s=e/A:-7+2+12/B:xx-xx_xx/C:02_xx+xx/D:03+xx_xx/E:xx_xx!xx_xx-xx/F:13_9#0_xx@1_5|1_29/G:4_4%0_xx_1/H:xx_xx/I:5-29@1+1&1-5|1+29/J:xx_xx/K:1+5-29");
})()
