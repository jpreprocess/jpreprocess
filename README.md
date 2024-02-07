# jpreprocess

日本語文を解析し，フルコンテキストラベルを生成します．

[OpenJTalk](http://open-jtalk.sourceforge.net/)の前処理部分(HTS Engine以外)をRustで書き直したものです．

## 目標・方針

- OpenJTalkの構造をそのまま移すのではなく，できるだけ読みやすく，書きやすい構造に
- 独自の辞書形式により辞書ファイルのサイズを削減しつつ，従来の「すべての情報を文字列で持つ」辞書も使える
  - どちらもMecab辞書自体とは互換性がありませんが，Mecab辞書の構築に使うのと同様のCSVファイルを使って辞書を生成できます．
- 一部のバグと思われる機能を除き，OpenJTalkと全く同じ出力（フルコンテキストラベル）を得ることができる
  - たとえば「特殊助動詞」や紛らわしい2,2,3桁区切りの数字の読み方は，OpenJTalkと異なります．
  - 新しい機能の追加を排除するものではありませんが，
    オプションやバージョン，feature等でOpenJTalkと同じ出力を得る手段が残るようにしたいと考えています．
- このリポジトリではHTS Engineは扱わない
  - フルコンテキストラベルの生成までをサポートしますが，その先はこのリポジトリの範囲外とします．
  - HTS EngineをRustで書き直すプロジェクトは[jpreprocess/jbonsai](https://github.com/jpreprocess/jbonsai)にあります．

## Crates

### jpreprocess

主要なインターフェースです．
Lindera，jpreprocess-njd，jpreprocess-jpcommonなどのラッパーです．
解析結果の単語は，jpreprocess-coreで規定されるデータ構造で保持します．

例：

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
  jpcommon_label[2],
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

### jpreprocess-core

発音，単語，品詞，JPCommon等のデータ構造と，それに関連する関数群，エラーを表現する構造を含みます．
なお，`pos`はPart Of Speechの頭字語で，「品詞」を表します．

### jpreprocess-dictionary

jpreprocess-dictionary-builderで生成される単語辞書をメモリ上に読み込み，単語を検索できるようにします．

この際，辞書の形式を自動で判別します．

### jpreprocess-dictionary-builder

元となる辞書はMecab同様のcsv形式ですが，[Lindera](https://github.com/lindera-morphology/lindera)で高速に解析できるよう，
事前に専用の辞書を生成する必要があります．

Linderaの[lindera-ipadic-builder](https://crates.io/crates/lindera-ipadic-builder)を元にして作られていますが，
jpreprocess-dictionary-builderは文字列のパースも事前に行い，JPreprocessで直接処理できる辞書（JPreprocess辞書）を生成できます．

### jpreprocess-naist-jdic

OpenJTalkに同梱されていた辞書を用いて，JPreprocess用の辞書を生成します．
jpreprocessクレートの`naist-jdic` featureのために使われます．

なお，`naist-jdic` featureを有効化してこのクレートを含めると，ビルドに数分かかります．

### jpreprocess-njd

OpenJTalkでいうNJDNode，NJDの構造を定義し，NJDに対する変換処理を行います．

具体的には，数字の読み方を変換したり(たとえば「10,120」を「いちまんひゃくにじゅう」)，
アクセント位置を推定したりします．

### jpreprocess-jpcommon

OpenJTalkでいうJPCommonLabelの構造を定義し，NJDからJPCommon，さらにJPCommonからフルコンテキストラベルへの変換を行います．

### jpreprocess-window

jpreprocess-njdの変換処理で使われる，mutableなwindowを実装します．

## Copyrights

This software includes source code from:

- [OpenJTalk](http://open-jtalk.sourceforge.net/).
  Copyright (c) 2008-2016  Nagoya Institute of Technology Department of Computer Science
- [Lindera](https://github.com/lindera-morphology/lindera).
  Copyright (c) 2019 by the project authors
- [Yada: Yet Another Double-Array](https://github.com/takuyaa/yada).

Although this repository has CODEOWNERS file,
it does not necessarily mean that the developers listed in codeowners file
have the copyright for all files in this repository.
Copyrights are listed in NOTICE or LICENSE files,
and CODEOWNERS file is just for code reviewing.

## License

BSD-3-Clause
