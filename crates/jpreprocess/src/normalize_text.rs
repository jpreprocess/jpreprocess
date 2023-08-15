/// Normalize input text
pub fn normalize_text_for_naist_jdic(input_text: &str) -> String {
    let yen_space = kana::yen2wide(&kana::space2wide(input_text).replace('\\', "\u{00A5}"));
    let kana = kana::vsmark2full(&kana::combine(&kana::half2full(&yen_space)))
        .replace(['\u{309B}', '\u{309C}'], "");
    let kana_ascii = kana::ascii2wide(
        &kana
            .replace('-', "\u{2212}") // − U+2212 MINUS SIGN
            .replace('~', "\u{301C}") // 〜 U+301C WAVE DASH
            .replace('`', "\u{2018}") // ‘ U+2018 LEFT SINGLE QUOTATION MARK
            .replace('\"', "\u{201D}") // ” U+201D RIGHT DOUBLE QUOTATION MARK
            .replace('\'', "\u{2019}"), // ’ U+2019 RIGHT SINGLE QUOTATION MARK
    );
    kana::space2wide(&kana_ascii)
}

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
