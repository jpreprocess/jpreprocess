# jpreprocess

**NOTE:** まだ仕様が安定していませんのでご注意ください．

日本語文を解析し、音声合成エンジンに渡せる形式に変換します．

[OpenJTalk](http://open-jtalk.sourceforge.net/)のNJD及びJPCommon部分をRustで書き直したものです．

## 目標・方針

- OpenJTalkの構造をそのまま移すのではなく，できるだけ読みやすく，書きやすい構造に
- 独自の辞書形式により辞書ファイルのサイズを削減しつつ，従来の「すべての情報を文字列で持つ」辞書も使える
  - どちらもMecab辞書自体とは互換性がありませんが，Mecab辞書用のCSVファイルを使って辞書を生成できます．
- 一部のバグと思われる機能を除き，OpenJTalkと同じ出力が得られる
  - たとえば「特殊助動詞」や紛らわしい2,2,3桁区切りの数字の読み方は，OpenJTalkと異なります
- HTS Engineは実装しない

## Crates

### jpreprocess

中核部分です．日本語文を解析し，そのデータを音声合成エンジンに渡せる形に変換します．
解析結果の単語は，jpreprocess-coreで規定されるデータ構造で保持します．

### jpreprocess-core

発音，単語，品詞，JPCommon等のデータ構造と，それに関連する関数群，エラーを表現する構造を含みます．

### jpreprocess-dictionary

jpreprocess-dictionary-builderで生成される単語辞書をメモリ上に読み込み，単語を検索できるようにします．

### jpreprocess-dictionary-builder

元となる辞書はcsv形式なので，[Lindera](https://github.com/lindera-morphology/lindera)で高速に解析できるよう事前に辞書を生成します．
Linderaの[lindera-ipadic-builder](https://crates.io/crates/lindera-ipadic-builder)が元になっていますが，jpreprocess-dictionary-builderはさらに，事前に文字列をパースしJPreprocessで直接処理できる辞書(`jpreprocess.words`，`jpreprocess.wordsidx`)を生成します．

### jpreprocess-naist-jdic

Open JTalkに同梱されている辞書を用いて，JPreprocess/Lindera用の辞書を生成します．
このクレートはビルドに時間がかかります．

## Copyrights

This software includes source code from:

- [OpenJTalk](http://open-jtalk.sourceforge.net/).
  Copyright (c) 2008-2016  Nagoya Institute of Technology Department of Computer Science
- [Lindera](https://github.com/lindera-morphology/lindera).
  Copyright (c) 2019 by the project authors

## License

BSD-3-Clause
