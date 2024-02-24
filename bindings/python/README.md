# jpreprocess (python binding)

Japanese text preprocessor for Text-to-Speech application.

This is a python binding of jpreprocess, which is written in Rust.
The rust library is published in [crates.io](https://crates.io/crates/jpreprocess).

JPreprocess (the base code written in Rust) is a rewrite of [OpenJTalk](http://open-jtalk.sourceforge.net/).

## Usage

Unlike [pyopenjtalk](https://pypi.org/project/pyopenjtalk/), this package does not include support of marine and TTS.

Currently, this package is for text processing only.

### Run text processing frontend

```python
import jpreprocess

j = jpreprocess.jpreprocess()
njd_features = j.run_frontend("本日は晴天なり")

assert njd_features[0].get("string") == "本日"
assert njd_features[0].get("pos") == "名詞"
```

### Extract full-context label

```python
import jpreprocess

j = jpreprocess.jpreprocess()
fullcontext = j.extract_fullcontext("本日は晴天なり")

assert len(fullcontext) == 21
assert fullcontext[0] == r"xx^xx-sil+h=o/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:5_1%0_xx_xx/H:xx_xx/I:xx-xx@xx+xx&xx-xx|xx+xx/J:2_11/K:1+2-11"
```

### Grapheme-to-phoeneme (G2P)

```python
import jpreprocess

j = jpreprocess.jpreprocess()

assert j.g2p("おはようございます") == "o h a y o o g o z a i m a s U"
assert j.g2p("おはようございます", kana=True) == "オハヨーゴザイマス"
```

## Copyrights

Please see [README.md](https://github.com/jpreprocess/jpreprocess/blob/main/README.md).

## License

BSD-3-Clause
