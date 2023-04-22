pub fn normalize_text_for_naist_jdic(input_text: &str) -> String {
    let yen_space = kana::yen2wide(&kana::space2wide(input_text)).replace("\\", "\u{a5}");
    let kana = kana::vsmark2full(&kana::combine(&kana::half2full(&yen_space)))
        .replace("\u{309b}", "")
        .replace("\u{309C}", "");
    let kana_ascii = kana::ascii2wide(&kana::yen2wide(&kana))
        .replace("－", "−")
        .replace("\u{FF5E}", "\u{301C}")
        .replace("｀", "‘")
        .replace("＂", "”")
        .replace("＇", "’");
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
