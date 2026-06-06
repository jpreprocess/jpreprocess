# jpreprocess

日本語音声合成の前処理を行うライブラリです．日本語文を解析し，フルコンテキストラベルを生成します．

[OpenJTalk](http://open-jtalk.sourceforge.net/)の前処理部分をRustで書き直したものです．なお，（前処理ではない）音声合成部分については，[jpreprocess/jbonsai](https://github.com/jpreprocess/jbonsai)にて再実装・最適化を行っています．

## 実装の概要

日本語の音声合成にはいくつかの手法がありますが，著名なライブラリである[OpenJTalk](http://open-jtalk.sourceforge.net/)では，以下の三段階で合成する方法がとられています．

1. 形態素解析：巨大な辞書を用いて，文章を単語ごとに分けます．さらに辞書には各単語の品詞，読み，アクセント等が書かれていますので，これらの情報を抜き出します．OpenJTalkでは形態素解析ライブラリMecabを用いています．
2. 文章の前処理：単語には前後の単語によって読みが変わるものがあります．数字の読み（10000→いちまん，など）は代表的ですが，他にもアクセント位置が移動したり，母音が無声化したりすることがあります．前処理ではこれらをルールベースで処理して反映します．
3. 音声合成：2.までで判明した読み・アクセント・品詞などの情報をもとに，音声波形を合成します．OpenJTalkではこの部分を同グループが開発した[hts_engine API](https://hts-engine.sourceforge.net)が担っています．

最近では，これをさらに発展させ，1.と2.はそのままMecab/OpenJTalkを用いつつ，音声合成部分のみを深層学習モデルに置き換えて性能を向上させることが行われています．

このような背景を踏まえ，本リポジトリでは1.と2.の部分に絞ってRustで再実装しています（ただし，1.は，その実質をRustで書かれた形態素解析ライブラリ[Lindera](https://github.com/lindera/lindera)に依存しています）．2.の出力は，[jlabel](https://github.com/jpreprocess/jlabel)を介してフルコンテキストラベルとして取り出すことができます．

残る3.の部分については，[jpreprocess/jbonsai](https://github.com/jpreprocess/jbonsai)にて再実装・最適化を行っています．したがって，jpreprocessとjbonsaiを組み合わせると，（広義）Open JTalk類似の音声合成を行うことができます．

### 目標

- （形態素解析部分）
  - Raspberry piなどメモリの限られた環境で使うことを考慮し，独自の辞書形式（jpreprocess辞書）によりメモリ上の辞書ファイルのサイズを削減する．
  - 一方，他の形態素解析エンジンとの互換性を維持するため，文字列（CSV）での入力も可能にする．
- （前処理部分）
  - OpenJTalkのコードをそのまま読み替えるのではなく，できるだけ読みやすく，書きやすい構造にする．
  - バグと思われる一部の挙動を除き，OpenJTalkと全く同じ出力（フルコンテキストラベル）を得ることができる．
    - たとえば「特殊助動詞」や紛らわしい2,2,3桁区切りの数字の読み方は，OpenJTalkと異なります．
    - ただし，OpenJTalkになかった機能を追加する場合もあります．その場合でも，OpenJTalkと同じ結果を得る手段が残るようにします．

## Crates

ドキュメンテーションについては，[docs.rs](https://docs.rs/jpreprocess/latest/jpreprocess/)を参照してください．

### jpreprocess

主要なインターフェースです．入力された日本語文の正規化を含みますが，それ以外の部分はLindera，jpreprocess-njd，jpreprocess-jpcommonなどのラッパーになっています．形態素解析の結果は，jpreprocess-coreで規定されるデータ構造で保持します．

例：

```rs
use jpreprocess::*;

let system = SystemDictionaryConfig::File(path).load()?;
let jpreprocess = JPreprocess::with_dictionaries(system, None);

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

### jpreprocess-core

発音，単語，品詞，JPCommon等のデータ構造と，それに関連する関数群，エラーを表現する構造を含みます．なお，`pos`はPart Of Speechの頭字語で，「品詞」を表します．

### jpreprocess-dictionary

単語辞書の作成と読み込みを担当します．

[Lindera](https://github.com/lindera/lindera)で形態素解析をするには，（Mecabと同様に）人間可読な辞書から，機械可読な辞書形式に変換する必要があります．

また，Linderaは日本語音声合成に特化した形態素解析ライブラリではないため，品詞等が文字列で出力され，メモリ上のデータサイズにいくらか無駄があります．jpreprocess-dictionaryでは，これらの単語の情報を事前にパースし，情報を圧縮したうえで，Linderaで解析可能な辞書として出力します（ここではjpreprocess辞書と呼びます）．

また，実行時には読み込んだ辞書がLinderaの辞書か，jpreprocess辞書かどうかを自動で判定し，jpreprocess辞書であれば必要に応じてjpreprocess-coreのデータ構造に展開します．

### jpreprocess-naist-jdic

OpenJTalkに同梱されていた辞書を用いて，JPreprocess用の辞書を生成します．jpreprocessクレートの`naist-jdic` featureのために使われます．

なお，`naist-jdic` featureを有効化してこのクレートを含めると，ビルドに数分かかります．

### jpreprocess-njd

OpenJTalkでいうNJDNode，NJDの構造を定義し，NJDに対する変換処理を行います．

具体的には，数字の読み方を変換したり(たとえば「10,120」を「いちまんひゃくにじゅう」)，アクセント位置を推定したりします．

### jpreprocess-jpcommon

OpenJTalkでいうJPCommonLabelの構造を定義し，NJDからJPCommon，さらにJPCommonからフルコンテキストラベルへの変換を行います．

### jpreprocess-window

jpreprocess-njdの変換処理で使われる，mutableなwindowを実装します．

## Copyrights

This software includes source code from:

- [OpenJTalk](http://open-jtalk.sourceforge.net/).
  Copyright (c) 2008-2016  Nagoya Institute of Technology Department of Computer Science
- [Lindera](https://github.com/lindera/lindera).
  Copyright (c) 2019 by the project authors

## License

BSD-3-Clause
