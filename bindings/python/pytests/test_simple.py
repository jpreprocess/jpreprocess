import jpreprocess
import pytest


@pytest.fixture(scope="module")
def jpre() -> jpreprocess.JPreprocess:
    return jpreprocess.jpreprocess(dictionary_version="latest")


def test_run_frontend(jpre: jpreprocess.JPreprocess) -> None:
    njd_features = jpre.run_frontend("本日は晴天なり")

    assert len(njd_features) == 4

    assert njd_features[0]["string"] == "本日"
    assert njd_features[0]["pos"] == "名詞"
    assert njd_features[0]["pos_group1"] == "副詞可能"
    assert njd_features[0]["pos_group2"] == "*"
    assert njd_features[0]["pos_group3"] == "*"
    assert njd_features[0]["ctype"] == "*"
    assert njd_features[0]["cform"] == "*"
    assert njd_features[0]["read"] == "ホンジツ"
    assert njd_features[0]["pron"] == "ホンジツ"
    assert njd_features[0]["acc"] == 1
    assert njd_features[0]["mora_size"] == 4
    assert njd_features[0]["chain_rule"] == "C1"
    assert njd_features[2]["chain_flag"] == 0

    assert njd_features[2]["string"] == "晴天"
    assert njd_features[2]["pos"] == "名詞"
    assert njd_features[2]["pos_group1"] == "一般"
    assert njd_features[2]["pos_group2"] == "*"
    assert njd_features[2]["pos_group3"] == "*"
    assert njd_features[2]["ctype"] == "*"
    assert njd_features[2]["cform"] == "*"
    assert njd_features[2]["read"] == "セイテン"
    assert njd_features[2]["pron"] == "セーテン"
    assert njd_features[2]["acc"] == 5
    assert njd_features[2]["mora_size"] == 4
    assert njd_features[2]["chain_rule"] == "C2"
    assert njd_features[2]["chain_flag"] == 0


def test_extract_fullcontext(jpre: jpreprocess.JPreprocess) -> None:
    fullcontext = jpre.extract_fullcontext("本日は晴天なり")

    assert len(fullcontext) == 21

    assert fullcontext[0] == r"xx^xx-sil+h=o/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:5_1%0_xx_xx/H:xx_xx/I:xx-xx@xx+xx&xx-xx|xx+xx/J:2_11/K:1+2-11"
    assert fullcontext[5] == r"N^j-i+ts=u/A:2+3+3/B:xx-xx_xx/C:02_xx+xx/D:24+xx_xx/E:xx_xx!xx_xx-xx/F:5_1#0_xx@1_2|1_11/G:6_5%0_xx_1/H:xx_xx/I:2-11@1+1&1-2|1+11/J:xx_xx/K:1+2-11"
    assert fullcontext[16] == r"e^N-n+a=r/A:0+5+2/B:02-xx_xx/C:10_6+2/D:xx+xx_xx/E:5_1!0_xx-1/F:6_5#0_xx@2_1|6_6/G:xx_xx%xx_xx_xx/H:xx_xx/I:2-11@1+1&1-2|1+11/J:xx_xx/K:1+2-11"


def test_g2p(jpre: jpreprocess.JPreprocess) -> None:
    assert jpre.g2p("本日は晴天なり", kana=False,
                    join=True) == "h o N j i ts u w a s e e t e N n a r i"
    assert jpre.g2p("本日は晴天なり", kana=False,
                    join=False) == ["h", "o", "N", "j", "i", "ts", "u", "w", "a", "s", "e", "e", "t", "e", "N", "n", "a", "r", "i"]
    assert jpre.g2p("本日は晴天なり", kana=True,
                    join=True) == "ホンジツワセーテンナリ"
    assert jpre.g2p("本日は晴天なり", kana=True,
                    join=False) == ["ホンジツ", "ワ", "セーテン", "ナリ"]

    assert jpre.g2p("おはようございます", kana=False,
                    join=True) == "o h a y o o g o z a i m a s U"
    assert jpre.g2p("おはようございます", kana=False,
                    join=False) == ["o", "h", "a", "y", "o", "o", "g", "o", "z", "a", "i", "m", "a", "s", "U"]
    assert jpre.g2p("おはようございます", kana=True,
                    join=True) == "オハヨーゴザイマス"
    assert jpre.g2p("おはようございます", kana=True,
                    join=False) == ["オハヨー", "ゴザイ", "マス"]
