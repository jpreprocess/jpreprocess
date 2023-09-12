use phf::{phf_map, phf_set, Map, Set};

/// Normalize input text
pub fn normalize_text_for_naist_jdic(input_text: &str) -> String {
    let (mut s, c) = input_text
        .chars()
        .map(|c| {
            if let Some(replacement) = HALFWIDTH.get(&c) {
                *replacement
            } else if '\u{0020}' < c && c < '\u{007f}' {
                char::from_u32((c as u32) + 0xfee0).unwrap()
            } else {
                c
            }
        })
        .fold(
            (String::with_capacity(input_text.len()), None),
            |(mut acc, prev), curr| {
                let semivoiced = SEMIVOICED_SOUND_MARK.contains(&curr);
                let voiced = VOICED_SOUND_MARK.contains(&curr);

                let combined = if semivoiced {
                    prev.and_then(|p| SEMIVOICED.get(&p))
                } else if voiced {
                    prev.and_then(|p| VOICED.get(&p))
                } else {
                    None
                };

                if let Some(combined) = combined {
                    acc.push(*combined);
                } else if let Some(prev_char) = prev {
                    acc.push(prev_char);
                }

                if semivoiced || voiced {
                    (acc, None)
                } else {
                    (acc, Some(curr))
                }
            },
        );

    if let Some(c) = c {
        s.push(c);
    }
    s
}

const HALFWIDTH: Map<char, char> = phf_map! {
    // Symbols
    ' ' => '\u{3000}', // 　 U+3000 Ideographic Space
    '\u{a5}' => '\u{FFE5}', // ￥ U+FFE5 Fullwidth Yen Sign
    '\\' => '\u{FFE5}', // ￥ U+FFE5 Fullwidth Yen Sign
    '-' => '\u{2212}', // − U+2212 MINUS SIGN
    '~' => '\u{301C}', // 〜 U+301C WAVE DASH
    '`' => '\u{2018}', // ‘ U+2018 LEFT SINGLE QUOTATION MARK
    '\"' => '\u{201D}', // ” U+201D RIGHT DOUBLE QUOTATION MARK
    '\'' => '\u{2019}', // ’ U+2019 RIGHT SINGLE QUOTATION MARK
    // Halfwidth japanese symbols
    '\u{FF61}' => '\u{3002}', // 。 U+3002 Ideographic Full Stop
    '\u{FF62}' => '\u{300C}', // 「 U+300C Left Corner Bracket Ideographic Full Stop
    '\u{FF63}' => '\u{300D}', // 」 U+300D Right Corner Bracket
    '\u{FF64}' => '\u{3001}', // 、 U+3001 Ideographic Comma
    '\u{FF65}' => '\u{30FB}', // ・ U+30FB Katakana Middle Dot
    // Katakana
    'ｦ' => 'ヲ',
    'ｧ' => 'ァ',
    'ｨ' => 'ィ',
    'ｩ' => 'ゥ',
    'ｪ' => 'ェ',
    'ｫ' => 'ォ',
    'ｬ' => 'ャ',
    'ｭ' => 'ュ',
    'ｮ' => 'ョ',
    'ｯ' => 'ッ',
    'ｰ' => 'ー',
    'ｱ' => 'ア',
    'ｲ' => 'イ',
    'ｳ' => 'ウ',
    'ｴ' => 'エ',
    'ｵ' => 'オ',
    'ｶ' => 'カ',
    'ｷ' => 'キ',
    'ｸ' => 'ク',
    'ｹ' => 'ケ',
    'ｺ' => 'コ',
    'ｻ' => 'サ',
    'ｼ' => 'シ',
    'ｽ' => 'ス',
    'ｾ' => 'セ',
    'ｿ' => 'ソ',
    'ﾀ' => 'タ',
    'ﾁ' => 'チ',
    'ﾂ' => 'ツ',
    'ﾃ' => 'テ',
    'ﾄ' => 'ト',
    'ﾅ' => 'ナ',
    'ﾆ' => 'ニ',
    'ﾇ' => 'ヌ',
    'ﾈ' => 'ネ',
    'ﾉ' => 'ノ',
    'ﾊ' => 'ハ',
    'ﾋ' => 'ヒ',
    'ﾌ' => 'フ',
    'ﾍ' => 'ヘ',
    'ﾎ' => 'ホ',
    'ﾏ' => 'マ',
    'ﾐ' => 'ミ',
    'ﾑ' => 'ム',
    'ﾒ' => 'メ',
    'ﾓ' => 'モ',
    'ﾔ' => 'ヤ',
    'ﾕ' => 'ユ',
    'ﾖ' => 'ヨ',
    'ﾗ' => 'ラ',
    'ﾘ' => 'リ',
    'ﾙ' => 'ル',
    'ﾚ' => 'レ',
    'ﾛ' => 'ロ',
    'ﾜ' => 'ワ',
    'ﾝ' => 'ン',
};

const SEMIVOICED_SOUND_MARK: Set<char> = phf_set! {
    '\u{309A}', // U+309A Combining Katakana-Hiragana Semi-Voiced Sound Mark
    '\u{309C}', // U+309C Katakana-Hiragana Semi-Voiced Sound Mark
    '\u{FF9F}', // U+FF9F Halfwidth Katakana Semi-Voiced Sound Mark
};
const SEMIVOICED: Map<char, char> = phf_map! {
    'ハ' => 'パ',
    'ヒ' => 'ピ',
    'フ' => 'プ',
    'ヘ' => 'ペ',
    'ホ' => 'ポ',
    'は' => 'ぱ',
    'ひ' => 'ぴ',
    'ふ' => 'ぷ',
    'へ' => 'ぺ',
    'ほ' => 'ぽ',
};

const VOICED_SOUND_MARK: Set<char> = phf_set! {
    '\u{3099}', // U+3099 Combining Katakana-Hiragana Voiced Sound Mark
    '\u{309B}', // U+309B Katakana-Hiragana Voiced Sound Mark
    '\u{FF9E}', // U+FF9E Halfwidth Katakana Voiced Sound Mark
};
const VOICED: Map<char, char> = phf_map! {
    'カ' => 'ガ',
    'キ' => 'ギ',
    'ク' => 'グ',
    'ケ' => 'ゲ',
    'コ' => 'ゴ',
    'サ' => 'ザ',
    'シ' => 'ジ',
    'ス' => 'ズ',
    'セ' => 'ゼ',
    'ソ' => 'ゾ',
    'タ' => 'ダ',
    'チ' => 'ヂ',
    'ツ' => 'ヅ',
    'テ' => 'デ',
    'ト' => 'ド',
    'ハ' => 'バ',
    'ヒ' => 'ビ',
    'フ' => 'ブ',
    'ヘ' => 'ベ',
    'ホ' => 'ボ',
    'ウ' => 'ヴ',
    'ワ' => 'ヷ',
    'ヰ' => 'ヸ',
    'ヱ' => 'ヹ',
    'ヲ' => 'ヺ',
    'ヽ' => 'ヾ',
    'か' => 'が',
    'き' => 'ぎ',
    'く' => 'ぐ',
    'け' => 'げ',
    'こ' => 'ご',
    'さ' => 'ざ',
    'し' => 'じ',
    'す' => 'ず',
    'せ' => 'ぜ',
    'そ' => 'ぞ',
    'た' => 'だ',
    'ち' => 'ぢ',
    'つ' => 'づ',
    'て' => 'で',
    'と' => 'ど',
    'は' => 'ば',
    'ひ' => 'び',
    'ふ' => 'ぶ',
    'へ' => 'べ',
    'ほ' => 'ぼ',
    'う' => 'ゔ',
};

#[cfg(test)]
mod tests {
    use crate::normalize_text_for_naist_jdic;

    #[test]
    fn ascii() {
        assert_eq!(
            normalize_text_for_naist_jdic(" !\"#$%&'()*+,-./"),
            "　！”＃＄％＆’（）＊＋，−．／"
        );
        assert_eq!(
            normalize_text_for_naist_jdic("0123456789"),
            "０１２３４５６７８９"
        );
        assert_eq!(normalize_text_for_naist_jdic(":;<=>?@"), "：；＜＝＞？＠");
        assert_eq!(
            normalize_text_for_naist_jdic("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            "ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ"
        );
        assert_eq!(normalize_text_for_naist_jdic("[\\]^_`"), "［￥］＾＿‘");
        assert_eq!(
            normalize_text_for_naist_jdic("abcdefghijklmnopqrstuvwxyz"),
            "ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ"
        );
        assert_eq!(normalize_text_for_naist_jdic("{|}~"), "｛｜｝〜");
    }

    #[test]
    fn kana() {
        assert_eq!(
            normalize_text_for_naist_jdic("ｳﾞｶﾞｷﾞｸﾞｹﾞｺﾞｻﾞｼﾞｽﾞｾﾞｿﾞﾀﾞﾁﾞﾂﾞﾃﾞﾄﾞﾊﾞﾋﾞﾌﾞﾍﾞﾎﾞﾊﾟﾋﾟﾌﾟﾍﾟﾎﾟ"),
            "ヴガギグゲゴザジズゼゾダヂヅデドバビブベボパピプペポ"
        );
        assert_eq!(normalize_text_for_naist_jdic("｡｢｣､･"), "。「」、・");
        assert_eq!(
            normalize_text_for_naist_jdic("ｦｧｨｩｪｫｬｭｮｯｰｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝ"),
            "ヲァィゥェォャュョッーアイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワン"
        );
    }

    #[test]
    fn diacritical() {
        assert_eq!(normalize_text_for_naist_jdic("ﾞﾟ"), "");
        assert_eq!(normalize_text_for_naist_jdic("あ゛"), "あ");
        assert_eq!(normalize_text_for_naist_jdic("あ゜"), "あ");
        assert_eq!(normalize_text_for_naist_jdic("は゛"), "ば");
        assert_eq!(normalize_text_for_naist_jdic("は゜"), "ぱ");
    }
}
