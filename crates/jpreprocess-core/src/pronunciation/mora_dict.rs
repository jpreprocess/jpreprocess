use std::collections::HashMap;

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use once_cell::sync::Lazy;

use super::mora_enum::MoraEnum;

pub static MORA_STR_LIST: Lazy<Vec<&str>> = Lazy::new(|| {
    let mut result = Vec::with_capacity(1 + 158 + 158 + 52 + 4);
    result.push("ー");
    result.extend(MORA_KATAKANA.iter().map(|(from, _to)| from));
    result.extend(MORA_HIRAGANA.iter().map(|(from, _to)| from));
    result.extend(MORA_ALPHABET.iter().map(|(from, _to)| from));
    result.extend(MORA_IRREGULAR_KATAKANA.iter().map(|(from, _to)| from));
    result
});

pub static MORA_DICT_AHO_CORASICK: Lazy<AhoCorasick> = Lazy::new(|| {
    AhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostLongest)
        .build(MORA_STR_LIST.as_slice())
        .unwrap()
});

pub fn get_mora_enum(position: usize) -> Vec<MoraEnum> {
    match position {
        0 => vec![MoraEnum::Long],
        1..=158 => vec![MORA_KATAKANA[position - 1].1],
        159..=316 => vec![MORA_HIRAGANA[position - 159].1],
        317..=368 => MORA_ALPHABET[position - 317].1.to_vec(),
        369..=372 => vec![MORA_IRREGULAR_KATAKANA[position - 369].1],
        _ => unreachable!(),
    }
}

pub static INTO_STR: Lazy<HashMap<MoraEnum, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::from_iter(
        MORA_KATAKANA
            .iter()
            .map(|(katakana, mora_enum)| (*mora_enum, *katakana)),
    );
    map.extend(
        MORA_IRREGULAR_KATAKANA
            .iter()
            .map(|(katakana, mora_enum)| (*mora_enum, *katakana)),
    );
    map.insert(MoraEnum::Long, "ー");
    map
});

const MORA_KATAKANA: [(&str, MoraEnum); 158] = [
    ("ヴョ", MoraEnum::Vyo),
    ("ヴュ", MoraEnum::Vyu),
    ("ヴャ", MoraEnum::Vya),
    ("ヴォ", MoraEnum::Vo),
    ("ヴェ", MoraEnum::Ve),
    ("ヴィ", MoraEnum::Vi),
    ("ヴァ", MoraEnum::Va),
    ("ヴ", MoraEnum::Vu),
    ("ン", MoraEnum::N),
    ("ヲ", MoraEnum::Wo),
    ("ヱ", MoraEnum::We),
    ("ヰ", MoraEnum::Wi),
    ("ワ", MoraEnum::Wa),
    ("ロ", MoraEnum::Ro),
    ("レ", MoraEnum::Re),
    ("ル", MoraEnum::Ru),
    ("リョ", MoraEnum::Ryo),
    ("リュ", MoraEnum::Ryu),
    ("リャ", MoraEnum::Rya),
    ("リェ", MoraEnum::Rye),
    ("リ", MoraEnum::Ri),
    ("ラ", MoraEnum::Ra),
    ("ヨ", MoraEnum::Yo),
    ("ョ", MoraEnum::Xyo),
    ("ユ", MoraEnum::Yu),
    ("ュ", MoraEnum::Xyu),
    ("ヤ", MoraEnum::Ya),
    ("ャ", MoraEnum::Xya),
    ("モ", MoraEnum::Mo),
    ("メ", MoraEnum::Me),
    ("ム", MoraEnum::Mu),
    ("ミョ", MoraEnum::Myo),
    ("ミュ", MoraEnum::Myu),
    ("ミャ", MoraEnum::Mya),
    ("ミェ", MoraEnum::Mye),
    ("ミ", MoraEnum::Mi),
    ("マ", MoraEnum::Ma),
    ("ポ", MoraEnum::Po),
    ("ボ", MoraEnum::Bo),
    ("ホ", MoraEnum::Ho),
    ("ペ", MoraEnum::Pe),
    ("ベ", MoraEnum::Be),
    ("ヘ", MoraEnum::He),
    ("プ", MoraEnum::Pu),
    ("ブ", MoraEnum::Bu),
    ("フォ", MoraEnum::Fo),
    ("フェ", MoraEnum::Fe),
    ("フィ", MoraEnum::Fi),
    ("ファ", MoraEnum::Fa),
    ("フ", MoraEnum::Fu),
    ("ピョ", MoraEnum::Pyo),
    ("ピュ", MoraEnum::Pyu),
    ("ピャ", MoraEnum::Pya),
    ("ピェ", MoraEnum::Pye),
    ("ピ", MoraEnum::Pi),
    ("ビョ", MoraEnum::Byo),
    ("ビュ", MoraEnum::Byu),
    ("ビャ", MoraEnum::Bya),
    ("ビェ", MoraEnum::Bye),
    ("ビ", MoraEnum::Bi),
    ("ヒョ", MoraEnum::Hyo),
    ("ヒュ", MoraEnum::Hyu),
    ("ヒャ", MoraEnum::Hya),
    ("ヒェ", MoraEnum::Hye),
    ("ヒ", MoraEnum::Hi),
    ("パ", MoraEnum::Pa),
    ("バ", MoraEnum::Ba),
    ("ハ", MoraEnum::Ha),
    ("ノ", MoraEnum::No),
    ("ネ", MoraEnum::Ne),
    ("ヌ", MoraEnum::Nu),
    ("ニョ", MoraEnum::Nyo),
    ("ニュ", MoraEnum::Nyu),
    ("ニャ", MoraEnum::Nya),
    ("ニェ", MoraEnum::Nye),
    ("ニ", MoraEnum::Ni),
    ("ナ", MoraEnum::Na),
    ("ドゥ", MoraEnum::Dwu),
    ("ド", MoraEnum::Do),
    ("トゥ", MoraEnum::Twu),
    ("ト", MoraEnum::To),
    ("デョ", MoraEnum::Dho),
    ("デュ", MoraEnum::Dhu),
    ("デャ", MoraEnum::Dha),
    ("ディ", MoraEnum::Dhi),
    ("デ", MoraEnum::De),
    ("テョ", MoraEnum::Tho),
    ("テュ", MoraEnum::Thu),
    ("テャ", MoraEnum::Tha),
    ("ティ", MoraEnum::Thi),
    ("テ", MoraEnum::Te),
    ("ヅ", MoraEnum::Du),
    ("ツォ", MoraEnum::Tso),
    ("ツェ", MoraEnum::Tse),
    ("ツィ", MoraEnum::Tsi),
    ("ツァ", MoraEnum::Tsa),
    ("ツ", MoraEnum::Tsu),
    ("ッ", MoraEnum::Xtsu),
    ("ヂ", MoraEnum::Di),
    ("チョ", MoraEnum::Cho),
    ("チュ", MoraEnum::Chu),
    ("チャ", MoraEnum::Cha),
    ("チェ", MoraEnum::Che),
    ("チ", MoraEnum::Chi),
    ("ダ", MoraEnum::Da),
    ("タ", MoraEnum::Ta),
    ("ゾ", MoraEnum::Zo),
    ("ソ", MoraEnum::So),
    ("ゼ", MoraEnum::Ze),
    ("セ", MoraEnum::Se),
    ("ズィ", MoraEnum::Zwi),
    ("ズ", MoraEnum::Zu),
    ("スィ", MoraEnum::Swi),
    ("ス", MoraEnum::Su),
    ("ジョ", MoraEnum::Jo),
    ("ジュ", MoraEnum::Ju),
    ("ジャ", MoraEnum::Ja),
    ("ジェ", MoraEnum::Je),
    ("ジ", MoraEnum::Ji),
    ("ショ", MoraEnum::Sho),
    ("シュ", MoraEnum::Shu),
    ("シャ", MoraEnum::Sha),
    ("シェ", MoraEnum::She),
    ("シ", MoraEnum::Shi),
    ("ザ", MoraEnum::Za),
    ("サ", MoraEnum::Sa),
    ("ゴ", MoraEnum::Go),
    ("コ", MoraEnum::Ko),
    ("ゲ", MoraEnum::Ge),
    ("ケ", MoraEnum::Ke),
    ("グ", MoraEnum::Gu),
    ("ク", MoraEnum::Ku),
    ("ギョ", MoraEnum::Gyo),
    ("ギュ", MoraEnum::Gyu),
    ("ギャ", MoraEnum::Gya),
    ("ギェ", MoraEnum::Gye),
    ("ギ", MoraEnum::Gi),
    ("キョ", MoraEnum::Kyo),
    ("キュ", MoraEnum::Kyu),
    ("キャ", MoraEnum::Kya),
    ("キェ", MoraEnum::Kye),
    ("キ", MoraEnum::Ki),
    ("ガ", MoraEnum::Ga),
    ("カ", MoraEnum::Ka),
    ("オ", MoraEnum::O),
    ("ォ", MoraEnum::Xo),
    ("エ", MoraEnum::E),
    ("ェ", MoraEnum::Xe),
    ("ウォ", MoraEnum::Who),
    ("ウェ", MoraEnum::Whe),
    ("ウィ", MoraEnum::Whi),
    ("ウ", MoraEnum::U),
    ("ゥ", MoraEnum::Xu),
    ("イェ", MoraEnum::Ye),
    ("イ", MoraEnum::I),
    ("ィ", MoraEnum::Xi),
    ("ア", MoraEnum::A),
    ("ァ", MoraEnum::Xa),
];

const MORA_HIRAGANA: [(&str, MoraEnum); 158] = [
    ("ゔょ", MoraEnum::Vyo),
    ("ゔゅ", MoraEnum::Vyu),
    ("ゔゃ", MoraEnum::Vya),
    ("ゔぉ", MoraEnum::Vo),
    ("ゔぇ", MoraEnum::Ve),
    ("ゔぃ", MoraEnum::Vi),
    ("ゔぁ", MoraEnum::Va),
    ("ゔ", MoraEnum::Vu),
    ("ん", MoraEnum::N),
    ("を", MoraEnum::Wo),
    ("ゑ", MoraEnum::We),
    ("ゐ", MoraEnum::Wi),
    ("わ", MoraEnum::Wa),
    ("ろ", MoraEnum::Ro),
    ("れ", MoraEnum::Re),
    ("る", MoraEnum::Ru),
    ("りょ", MoraEnum::Ryo),
    ("りゅ", MoraEnum::Ryu),
    ("りゃ", MoraEnum::Rya),
    ("りぇ", MoraEnum::Rye),
    ("り", MoraEnum::Ri),
    ("ら", MoraEnum::Ra),
    ("よ", MoraEnum::Yo),
    ("ょ", MoraEnum::Xyo),
    ("ゆ", MoraEnum::Yu),
    ("ゅ", MoraEnum::Xyu),
    ("や", MoraEnum::Ya),
    ("ゃ", MoraEnum::Xya),
    ("も", MoraEnum::Mo),
    ("め", MoraEnum::Me),
    ("む", MoraEnum::Mu),
    ("みょ", MoraEnum::Myo),
    ("みゅ", MoraEnum::Myu),
    ("みゃ", MoraEnum::Mya),
    ("みぇ", MoraEnum::Mye),
    ("み", MoraEnum::Mi),
    ("ま", MoraEnum::Ma),
    ("ぽ", MoraEnum::Po),
    ("ぼ", MoraEnum::Bo),
    ("ほ", MoraEnum::Ho),
    ("ぺ", MoraEnum::Pe),
    ("べ", MoraEnum::Be),
    ("へ", MoraEnum::He),
    ("ぷ", MoraEnum::Pu),
    ("ぶ", MoraEnum::Bu),
    ("ふぉ", MoraEnum::Fo),
    ("ふぇ", MoraEnum::Fe),
    ("ふぃ", MoraEnum::Fi),
    ("ふぁ", MoraEnum::Fa),
    ("ふ", MoraEnum::Fu),
    ("ぴょ", MoraEnum::Pyo),
    ("ぴゅ", MoraEnum::Pyu),
    ("ぴゃ", MoraEnum::Pya),
    ("ぴぇ", MoraEnum::Pye),
    ("ぴ", MoraEnum::Pi),
    ("びょ", MoraEnum::Byo),
    ("びゅ", MoraEnum::Byu),
    ("びゃ", MoraEnum::Bya),
    ("びぇ", MoraEnum::Bye),
    ("び", MoraEnum::Bi),
    ("ひょ", MoraEnum::Hyo),
    ("ひゅ", MoraEnum::Hyu),
    ("ひゃ", MoraEnum::Hya),
    ("ひぇ", MoraEnum::Hye),
    ("ひ", MoraEnum::Hi),
    ("ぱ", MoraEnum::Pa),
    ("ば", MoraEnum::Ba),
    ("は", MoraEnum::Ha),
    ("の", MoraEnum::No),
    ("ね", MoraEnum::Ne),
    ("ぬ", MoraEnum::Nu),
    ("にょ", MoraEnum::Nyo),
    ("にゅ", MoraEnum::Nyu),
    ("にゃ", MoraEnum::Nya),
    ("にぇ", MoraEnum::Nye),
    ("に", MoraEnum::Ni),
    ("な", MoraEnum::Na),
    ("どぅ", MoraEnum::Dwu),
    ("ど", MoraEnum::Do),
    ("とぅ", MoraEnum::Twu),
    ("と", MoraEnum::To),
    ("でょ", MoraEnum::Dho),
    ("でゅ", MoraEnum::Dhu),
    ("でゃ", MoraEnum::Dha),
    ("でぃ", MoraEnum::Dhi),
    ("で", MoraEnum::De),
    ("てょ", MoraEnum::Tho),
    ("てゅ", MoraEnum::Thu),
    ("てゃ", MoraEnum::Tha),
    ("てぃ", MoraEnum::Thi),
    ("て", MoraEnum::Te),
    ("づ", MoraEnum::Du),
    ("つぉ", MoraEnum::Tso),
    ("つぇ", MoraEnum::Tse),
    ("つぃ", MoraEnum::Tsi),
    ("つぁ", MoraEnum::Tsa),
    ("つ", MoraEnum::Tsu),
    ("っ", MoraEnum::Xtsu),
    ("ぢ", MoraEnum::Di),
    ("ちょ", MoraEnum::Cho),
    ("ちゅ", MoraEnum::Chu),
    ("ちゃ", MoraEnum::Cha),
    ("ちぇ", MoraEnum::Che),
    ("ち", MoraEnum::Chi),
    ("だ", MoraEnum::Da),
    ("た", MoraEnum::Ta),
    ("ぞ", MoraEnum::Zo),
    ("そ", MoraEnum::So),
    ("ぜ", MoraEnum::Ze),
    ("せ", MoraEnum::Se),
    ("ずぃ", MoraEnum::Zwi),
    ("ず", MoraEnum::Zu),
    ("すぃ", MoraEnum::Swi),
    ("す", MoraEnum::Su),
    ("じょ", MoraEnum::Jo),
    ("じゅ", MoraEnum::Ju),
    ("じゃ", MoraEnum::Ja),
    ("じぇ", MoraEnum::Je),
    ("じ", MoraEnum::Ji),
    ("しょ", MoraEnum::Sho),
    ("しゅ", MoraEnum::Shu),
    ("しゃ", MoraEnum::Sha),
    ("しぇ", MoraEnum::She),
    ("し", MoraEnum::Shi),
    ("ざ", MoraEnum::Za),
    ("さ", MoraEnum::Sa),
    ("ご", MoraEnum::Go),
    ("こ", MoraEnum::Ko),
    ("げ", MoraEnum::Ge),
    ("け", MoraEnum::Ke),
    ("ぐ", MoraEnum::Gu),
    ("く", MoraEnum::Ku),
    ("ぎょ", MoraEnum::Gyo),
    ("ぎゅ", MoraEnum::Gyu),
    ("ぎゃ", MoraEnum::Gya),
    ("ぎぇ", MoraEnum::Gye),
    ("ぎ", MoraEnum::Gi),
    ("きょ", MoraEnum::Kyo),
    ("きゅ", MoraEnum::Kyu),
    ("きゃ", MoraEnum::Kya),
    ("きぇ", MoraEnum::Kye),
    ("き", MoraEnum::Ki),
    ("が", MoraEnum::Ga),
    ("か", MoraEnum::Ka),
    ("お", MoraEnum::O),
    ("ぉ", MoraEnum::Xo),
    ("え", MoraEnum::E),
    ("ぇ", MoraEnum::Xe),
    ("うぉ", MoraEnum::Who),
    ("うぇ", MoraEnum::Whe),
    ("うぃ", MoraEnum::Whi),
    ("う", MoraEnum::U),
    ("ぅ", MoraEnum::Xu),
    ("いぇ", MoraEnum::Ye),
    ("い", MoraEnum::I),
    ("ぃ", MoraEnum::Xi),
    ("あ", MoraEnum::A),
    ("ぁ", MoraEnum::Xa),
];

const MORA_ALPHABET: [(&str, &[MoraEnum]); 52] = [
    //Small Letters
    ("ｚ", &[MoraEnum::Zwi, MoraEnum::Long]), // ズィー 2
    ("ｙ", &[MoraEnum::Wa, MoraEnum::I]),     // ワイ 2
    (
        "ｘ",
        &[MoraEnum::E, MoraEnum::Xtsu, MoraEnum::Ku, MoraEnum::Su],
    ), // エックス 4
    (
        "ｗ",
        &[MoraEnum::Da, MoraEnum::Bu, MoraEnum::Ryu, MoraEnum::Long],
    ), // ダブリュー 4
    ("ｖ", &[MoraEnum::Bu, MoraEnum::I]),     // ブイ 2
    ("ｕ", &[MoraEnum::Yu, MoraEnum::Long]),  // ユー 2
    ("ｔ", &[MoraEnum::Thi, MoraEnum::Long]), // ティー 2
    ("ｓ", &[MoraEnum::E, MoraEnum::Su]),     // エス 2
    ("ｒ", &[MoraEnum::A, MoraEnum::Long, MoraEnum::Ru]), // アール 3
    ("ｑ", &[MoraEnum::Kyu, MoraEnum::Long]), // キュー 2
    ("ｐ", &[MoraEnum::Pi, MoraEnum::Long]),  // ピー 2
    ("ｏ", &[MoraEnum::O, MoraEnum::Long]),   // オー 2
    ("ｎ", &[MoraEnum::E, MoraEnum::Nu]),     // エヌ 2
    ("ｍ", &[MoraEnum::E, MoraEnum::Mu]),     // エム 2
    ("ｌ", &[MoraEnum::E, MoraEnum::Ru]),     // エル 2
    ("ｋ", &[MoraEnum::Ke, MoraEnum::Long]),  // ケー 2
    ("ｊ", &[MoraEnum::Je, MoraEnum::Long]),  // ジェー 2
    ("ｉ", &[MoraEnum::A, MoraEnum::I]),      // アイ 2
    ("ｈ", &[MoraEnum::E, MoraEnum::I, MoraEnum::Chi]), // エイチ 3
    ("ｇ", &[MoraEnum::Ji, MoraEnum::Long]),  // ジー 2
    ("ｆ", &[MoraEnum::E, MoraEnum::Fu]),     // エフ 2
    ("ｅ", &[MoraEnum::I, MoraEnum::Long]),   // イー 2
    ("ｄ", &[MoraEnum::Dhi, MoraEnum::Long]), // ディー 2
    ("ｃ", &[MoraEnum::Shi, MoraEnum::Long]), // シー 2
    ("ｂ", &[MoraEnum::Bi, MoraEnum::Long]),  // ビー 2
    ("ａ", &[MoraEnum::E, MoraEnum::Long]),   // エー 2
    // Capital Letters
    ("Ｚ", &[MoraEnum::Zwi, MoraEnum::Long]), // ズィー 2
    ("Ｙ", &[MoraEnum::Wa, MoraEnum::I]),     // ワイ 2
    (
        "Ｘ",
        &[MoraEnum::E, MoraEnum::Xtsu, MoraEnum::Ku, MoraEnum::Su],
    ), // エックス 4
    (
        "Ｗ",
        &[MoraEnum::Da, MoraEnum::Bu, MoraEnum::Ryu, MoraEnum::Long],
    ), // ダブリュー 4
    ("Ｖ", &[MoraEnum::Bu, MoraEnum::I]),     // ブイ 2
    ("Ｕ", &[MoraEnum::Yu, MoraEnum::Long]),  // ユー 2
    ("Ｔ", &[MoraEnum::Thi, MoraEnum::Long]), // ティー 2
    ("Ｓ", &[MoraEnum::E, MoraEnum::Su]),     // エス 2
    ("Ｒ", &[MoraEnum::A, MoraEnum::Long, MoraEnum::Ru]), // アール 3
    ("Ｑ", &[MoraEnum::Kyu, MoraEnum::Long]), // キュー 2
    ("Ｐ", &[MoraEnum::Pi, MoraEnum::Long]),  // ピー 2
    ("Ｏ", &[MoraEnum::O, MoraEnum::Long]),   // オー 2
    ("Ｎ", &[MoraEnum::E, MoraEnum::Nu]),     // エヌ 2
    ("Ｍ", &[MoraEnum::E, MoraEnum::Mu]),     // エム 2
    ("Ｌ", &[MoraEnum::E, MoraEnum::Ru]),     // エル 2
    ("Ｋ", &[MoraEnum::Ke, MoraEnum::Long]),  // ケー 2
    ("Ｊ", &[MoraEnum::Je, MoraEnum::Long]),  // ジェー 2
    ("Ｉ", &[MoraEnum::A, MoraEnum::I]),      // アイ 2
    ("Ｈ", &[MoraEnum::E, MoraEnum::I, MoraEnum::Chi]), // エイチ 3
    ("Ｇ", &[MoraEnum::Ji, MoraEnum::Long]),  // ジー 2
    ("Ｆ", &[MoraEnum::E, MoraEnum::Fu]),     // エフ 2
    ("Ｅ", &[MoraEnum::I, MoraEnum::Long]),   // イー 2
    ("Ｄ", &[MoraEnum::Dhi, MoraEnum::Long]), // ディー 2
    ("Ｃ", &[MoraEnum::Shi, MoraEnum::Long]), // シー 2
    ("Ｂ", &[MoraEnum::Bi, MoraEnum::Long]),  // ビー 2
    ("Ａ", &[MoraEnum::E, MoraEnum::Long]),   // エー 2
];

const MORA_IRREGULAR_KATAKANA: [(&str, MoraEnum); 4] = [
    ("グヮ", MoraEnum::Gwa),
    ("クヮ", MoraEnum::Kwa),
    ("ヮ", MoraEnum::Xwa),
    ("ヶ", MoraEnum::Xke),
];

#[cfg(test)]
mod tests {
    use crate::pronunciation::MoraEnum;

    use super::{get_mora_enum, MORA_STR_LIST};

    #[test]
    fn long() {
        let found = MORA_STR_LIST.iter().position(|l| *l == "ー").unwrap();
        assert_eq!(get_mora_enum(found).as_slice(), [MoraEnum::Long]);
    }
    #[test]
    fn katakana() {
        let found = MORA_STR_LIST.iter().position(|l| *l == "ヴョ").unwrap();
        assert_eq!(get_mora_enum(found).as_slice(), [MoraEnum::Vyo]);
    }
    #[test]
    fn hiragana() {
        let found = MORA_STR_LIST.iter().position(|l| *l == "ぁ").unwrap();
        assert_eq!(get_mora_enum(found).as_slice(), [MoraEnum::Xa]);
    }
    #[test]
    fn alphabet() {
        let found = MORA_STR_LIST.iter().position(|l| *l == "ｘ").unwrap();
        assert_eq!(
            get_mora_enum(found).as_slice(),
            [MoraEnum::E, MoraEnum::Xtsu, MoraEnum::Ku, MoraEnum::Su]
        );
    }
    #[test]
    fn katakana_irregular1() {
        let found = MORA_STR_LIST.iter().position(|l| *l == "グヮ").unwrap();
        assert_eq!(get_mora_enum(found).as_slice(), [MoraEnum::Gwa]);
    }
    #[test]
    fn katakana_irregular2() {
        let found = MORA_STR_LIST.iter().position(|l| *l == "ヶ").unwrap();
        assert_eq!(get_mora_enum(found).as_slice(), [MoraEnum::Xke]);
    }
}
