# jpreprocess

**NOTE:** このプロジェクトは開発中であり、まだほとんどの機能が完成していません。

日本語文を解析し、音声合成エンジンに渡せる形式に変換します。

[OpenJTalk](http://open-jtalk.sourceforge.net/)のNJD部分をRustに移植したものです。
手作業のため、多数のバグが残っていますので、安定するまでしばらくお待ちください。

## Crates

### jpreprocess-core

中核部分です．日本語文を解析し，そのデータを音声合成エンジンに渡せる形に変換します．
解析結果の単語は，jpreprocess-njdで規定されるデータ構造で保持します．

### jpreprocess-njd

単語を保持するデータ構造と，それに関連する関数群です．

### jpreprocess-dictionary

元となる辞書は文字列で単語の情報をもっているので，事前にパースして直接処理できる辞書(`jpreprocess.words`，`jpreprocess.wordsidx`)を生成します．
jpreprocess-njdの文字列からデータを生成する関数を使います．

### jpreprocess-naist-jdic

辞書を生成します．このクレートをビルドすると時間がかかります．

### lindera-ipadic-builder

[lindera-ipadic-builder](https://github.com/lindera-morphology/lindera/tree/main/lindera-ipadic-builder)を一部改変したものです．
そのため，このクレートだけライセンスがMITになっています．

具体的には，openjtalkに含まれる辞書のエンコーディングがUTF-8なので，受け入れるエンコーディングをEUC-JPからUTF-8に変更しています．

## Copyrights

Part of the source code of this project is from [OpenJTalk](http://open-jtalk.sourceforge.net/).

Copyright (c) 2008-2016  Nagoya Institute of Technology Department of Computer Science

## License

### lindera-ipadic-builder

MIT

### others

BSD-3-Clause
