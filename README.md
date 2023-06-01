# jpreprocess

**NOTE:** まだ仕様が安定していませんのでご注意ください．

日本語文を解析し、音声合成エンジンに渡せる形式に変換します．

[OpenJTalk](http://open-jtalk.sourceforge.net/)の前処理部分(HTS Engine以外)をRustで書き直したものです．

## 目標・方針

- OpenJTalkの構造をそのまま移すのではなく，できるだけ読みやすく，書きやすい構造に
- 独自の辞書形式により辞書ファイルのサイズを削減しつつ，従来の「すべての情報を文字列で持つ」辞書も使える
  - どちらもMecab辞書自体とは互換性がありませんが，Mecab辞書用のCSVファイルを使って辞書を生成できます．
- 一部のバグと思われる機能を除き，OpenJTalkと全く同じ出力(JPCommon)を得ることができる
  - たとえば「特殊助動詞」や紛らわしい2,2,3桁区切りの数字の読み方は，OpenJTalkと異なります．
  - NJDレベルではOpenJTalkと違っていることがありますが，JPCommonでは同じになります．
  - 新しい機能の追加を排除するものではありませんが，
    オプションやバージョン等でOpenJTalkと同じ出力を得る手段が残るようにしたいと考えています．
- HTS Engineは実装しない
  - 荷が重い，かつ需要が少ないと考えられるので，少なくともこのリポジトリではHTS Engineは実装しません．

## Crates

### jpreprocess

主要なインターフェースです．
Lindera，jpreprocess-njd，jpreprocess-jpcommonなどのラッパーです．
解析結果の単語は，jpreprocess-coreで規定されるデータ構造で保持します．

例：

```rs
let config = JPreprocessDictionaryConfig::FileLindera(PathBuf::from("path_to_lindera_dictionary"));
let jpreprocess = JPreprocess::new(config).unwrap();
dbg!(jpreprocess.extract_fullcontext("日本語文を解析し、音声合成エンジンに渡せる形式に変換します．"))
```

### jpreprocess-core

発音，単語，品詞，JPCommon等のデータ構造と，それに関連する関数群，エラーを表現する構造を含みます．
なお，`pos`はPart Of Speechの頭字語で，「品詞」を表します．

### jpreprocess-dictionary

jpreprocess-dictionary-builderで生成される単語辞書をメモリ上に読み込み，単語を検索できるようにします．

### jpreprocess-dictionary-builder

元となる辞書はMecab同様のcsv形式ですが，[Lindera](https://github.com/lindera-morphology/lindera)で高速に解析できるよう，事前に専用の辞書を生成します．
Linderaの[lindera-ipadic-builder](https://crates.io/crates/lindera-ipadic-builder)が元になっていますが，
jpreprocess-dictionary-builderは文字列のパースも事前に行い，JPreprocessで直接処理できる辞書(`jpreprocess.words`，`jpreprocess.wordsidx`)を生成します．

### jpreprocess-naist-jdic

OpenJTalkに同梱されていた辞書を用いて，JPreprocess/Lindera用の辞書を生成します．
なお，このクレートはビルドに数分かかります．

### jpreprocess-njd

OpenJTalkでいうNJDNode，NJDの構造を定義し，NJDに対する変換処理を行います．

具体的には，数字の読み方を変換したり(たとえば「10,120」を「いちまんひゃくにじゅう」)，
アクセント位置を推定したりします．

### jpreprocess-jpcommon

OpenJTalkでいうJPCommonLabelの構造を定義し，NJDからJPCommon，さらにJPCommonから文字列への変換を行います．

### jpreprocess-window

jpreprocess-njdの変換処理で使われる，mutableなwindowを実装します．

## Copyrights

This software includes source code from:

- [OpenJTalk](http://open-jtalk.sourceforge.net/).
  Copyright (c) 2008-2016  Nagoya Institute of Technology Department of Computer Science
- [Lindera](https://github.com/lindera-morphology/lindera).
  Copyright (c) 2019 by the project authors

## License

BSD-3-Clause
