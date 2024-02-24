# jpreprocess

Japanese text preprocessor for Text-to-Speech application.

This project is a rewrite of [OpenJTalk](http://open-jtalk.sourceforge.net/) in Rust language.

## Usage

Put the following in Cargo.toml

```toml
[dependencies]
jpreprocess = "0.8.0"
```

It may be necessary to add
[jpreprocess-njd](https://crates.io/crates/jpreprocess-njd/) and/or
[jpreprocess-jpcommon](https://crates.io/crates/jpreprocess-jpcommon/)
if you want control over how njd and jpcommon are processed.

## Example

In this example, jpreprocess takes a [lindera](https://crates.io/crates/lindera-tokenizer/) dictionary and
preprocesses a text into jpcommon labels.

```rs
use jpreprocess::*;

let config = JPreprocessConfig {
     dictionary: SystemDictionaryConfig::File(path),
     user_dictionary: None,
 };
let jpreprocess = JPreprocess::from_config(config)?;

let jpcommon_label = jpreprocess
    .extract_fullcontext("日本語文を解析し、音声合成エンジンに渡せる形式に変換します．")?;
assert_eq!(
  jpcommon_label[2].to_string(),
  concat!(
      "sil^n-i+h=o",
      "/A:-3+1+7",
      "/B:xx-xx_xx",
      "/C:02_xx+xx",
      "/D:02+xx_xx",
      "/E:xx_xx!xx_xx-xx",
      "/F:7_4#0_xx@1_3|1_12",
      "/G:4_4%0_xx_1",
      "/H:xx_xx",
      "/I:3-12@1+2&1-8|1+41",
      "/J:5_29",
      "/K:2+8-41"
  )
);
```

Other examples can be found at [GitHub](https://github.com/jpreprocess/jpreprocess/tree/main/examples).

## Copyrights

This software includes source code from:

- [OpenJTalk](http://open-jtalk.sourceforge.net/).
  Copyright (c) 2008-2016  Nagoya Institute of Technology Department of Computer Science
- [Lindera](https://github.com/lindera-morphology/lindera).
  Copyright (c) 2019 by the project authors

## License

BSD-3-Clause

## API Reference

- [jpreprocess](https://docs.rs/jpreprocess)
