use crate::pos::{
    Daimeishi, Doushi, Fukushi, Joshi, KakuJoshi, Keiyoushi, Kigou, KoyuMeishi, Meishi,
    MeishiHijiritsu, Person, Region, Setsubi, Settoushi,
};

use super::POS;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("Failed to parse part of speech (POS) from strings: {pos_strings:?}, search string: {search_string}")]
pub struct POSParseError {
    pos_strings: [String; 4],
    search_string: String,
}

impl POSParseError {
    pub(crate) fn new(pos_strings: [&str; 4], search_string: String) -> Self {
        Self {
            pos_strings: [
                pos_strings[0].to_string(),
                pos_strings[1].to_string(),
                pos_strings[2].to_string(),
                pos_strings[3].to_string(),
            ],
            search_string,
        }
    }
}

pub fn string_to_pos(pos_strings: [&str; 4]) -> Result<POS, POSParseError> {
    let last_pos_index = pos_strings
        .iter()
        .rposition(|s| !s.is_empty() && *s != "*")
        .unwrap_or(0);

    let search_string = pos_strings[..=last_pos_index].join(",");
    if let Some(pos) = STRING_POS_CONVERSION_TABLE.get(search_string.as_str()) {
        Ok(*pos)
    } else {
        Err(POSParseError::new(pos_strings, search_string))
    }
}

pub fn value_to_pos(pos_value: u8) -> Option<POS> {
    if pos_value as usize >= VALUE_POS_CONVERSION_TABLE.len() {
        return None;
    }
    Some(VALUE_POS_CONVERSION_TABLE[pos_value as usize])
}

pub fn pos_to_string(pos: &POS) -> String {
    if pos == &POS::Unknown {
        "*".to_string()
    } else {
        let (string, _) = get_by_pos(pos);
        let comma_count = string.chars().filter(|&c| c == ',').count();
        format!("{}{}", string, ",*".repeat(3 - comma_count))
    }
}

pub fn pos_to_value(pos: &POS) -> u8 {
    let (_, value) = get_by_pos(pos);
    value
}

macro_rules! pos_conversions {
    (@map_string $( $pos:expr => $value:literal, $string:literal ),* $(,)?) => {
        phf::phf_map! {
            $($string => $pos,)*
        }
    };
    (@map_value $( $pos:expr => $value:literal, $string:literal ),* $(,)?) => {
        {
            let mut table = [POS::Unknown; 75];
            $(
                table[$value as usize] = $pos;
            )*
            table
        }
    };
    (@match $target:expr, $( $pos:pat => $value:literal, $string:literal ),* $(,)?) => {
        match $target {
            $($pos => ($string, $value),)*
        }
    };

    ($($tokens:tt)*) => {
        const STRING_POS_CONVERSION_TABLE: phf::Map<&'static str, POS> = pos_conversions!(@map_string $($tokens)*);
        const VALUE_POS_CONVERSION_TABLE: [POS; 75] = pos_conversions!(@map_value $($tokens)*);
        fn get_by_pos(pos: &POS) -> (&'static str, u8) {
            pos_conversions!(@match pos, $($tokens)*)
        }
    };
}

pos_conversions! {
    // ---- 単独 ----
    POS::Filler => 0, "フィラー",
    POS::Kandoushi => 1, "感動詞",
    POS::Jodoushi => 2, "助動詞",
    POS::Setsuzokushi => 3, "接続詞",
    POS::Rentaishi => 4, "連体詞",
    POS::Others => 5, "その他",
    POS::Unknown => 6, "",

    // ---- 記号 ----
    POS::Kigou(Kigou::None) => 7, "記号",
    POS::Kigou(Kigou::Alphabet) => 8, "記号,アルファベット",
    POS::Kigou(Kigou::General) => 9, "記号,一般",
    POS::Kigou(Kigou::KakkoOpen) => 10, "記号,括弧開",
    POS::Kigou(Kigou::KakkoClose) => 11, "記号,括弧閉",
    POS::Kigou(Kigou::Kuten) => 12, "記号,句点",
    POS::Kigou(Kigou::Space) => 13, "記号,空白",
    POS::Kigou(Kigou::Kazu) => 14, "記号,数",
    POS::Kigou(Kigou::Touten) => 15, "記号,読点",

    // ---- 形容詞 ----
    POS::Keiyoushi(Keiyoushi::Jiritsu) => 16, "形容詞,自立",
    POS::Keiyoushi(Keiyoushi::Setsubi) => 17, "形容詞,接尾",
    POS::Keiyoushi(Keiyoushi::Hijiritsu) => 18, "形容詞,非自立",

    // ---- 助詞 ----
    POS::Joshi(Joshi::KakuJoshi(KakuJoshi::General)) => 19, "助詞,格助詞,一般",
    POS::Joshi(Joshi::KakuJoshi(KakuJoshi::Quote)) => 20, "助詞,格助詞,引用",
    POS::Joshi(Joshi::KakuJoshi(KakuJoshi::Rengo)) => 21, "助詞,格助詞,連語",
    POS::Joshi(Joshi::KakariJoshi) => 22, "助詞,係助詞",
    POS::Joshi(Joshi::ShuJoshi) => 23, "助詞,終助詞",
    POS::Joshi(Joshi::SetsuzokuJoshi) => 24, "助詞,接続助詞",
    POS::Joshi(Joshi::Special) => 25, "助詞,特殊",
    POS::Joshi(Joshi::Fukushika) => 26, "助詞,副詞化",
    POS::Joshi(Joshi::FukuJoshi) => 27, "助詞,副助詞",
    POS::Joshi(Joshi::FukuHeiritsuShuJoshi) => 28, "助詞,副助詞／並立助詞／終助詞",
    POS::Joshi(Joshi::HeiritsuJoshi) => 29, "助詞,並立助詞",
    POS::Joshi(Joshi::Rentaika) => 30, "助詞,連体化",

    // ---- 接頭詞 ----
    POS::Settoushi(Settoushi::KeiyoushiSetsuzoku) => 31, "接頭詞,形容詞接続",
    POS::Settoushi(Settoushi::SuuSetsuzoku) => 32, "接頭詞,数接続",
    POS::Settoushi(Settoushi::DoushiSetsuzoku) => 33, "接頭詞,動詞接続",
    POS::Settoushi(Settoushi::MeishiSetsuzoku) => 34, "接頭詞,名詞接続",

    // ---- 動詞 ----
    POS::Doushi(Doushi::Jiritsu) => 35, "動詞,自立",
    POS::Doushi(Doushi::Setsubi) => 36, "動詞,接尾",
    POS::Doushi(Doushi::Hijiritsu) => 37, "動詞,非自立",

    // ---- 副詞 ----
    POS::Fukushi(Fukushi::None) => 38, "副詞",
    POS::Fukushi(Fukushi::General) => 39, "副詞,一般",
    POS::Fukushi(Fukushi::JoshiruiSetsuzoku) => 40, "副詞,助詞類接続",

    // ---- 名詞（直下） ----
    POS::Meishi(Meishi::SahenSetsuzoku) => 41, "名詞,サ変接続",
    POS::Meishi(Meishi::NaiKeiyoushiGokan) => 42, "名詞,ナイ形容詞語幹",
    POS::Meishi(Meishi::General) => 43, "名詞,一般",
    POS::Meishi(Meishi::QuoteStr) => 44, "名詞,引用文字列",
    POS::Meishi(Meishi::KeiyoudoushiGokan) => 45, "名詞,形容動詞語幹",
    POS::Meishi(Meishi::Kazu) => 46, "名詞,数",
    POS::Meishi(Meishi::Setsuzokushiteki) => 47, "名詞,接続詞的",
    POS::Meishi(Meishi::DoushiHijiritsuteki) => 48, "名詞,動詞非自立的",
    POS::Meishi(Meishi::Special) => 49, "名詞,特殊",
    POS::Meishi(Meishi::FukushiKanou) => 50, "名詞,副詞可能",
    POS::Meishi(Meishi::None) => 51, "名詞",

    // ---- 名詞・固有名詞 ----
    POS::Meishi(Meishi::KoyuMeishi(KoyuMeishi::General)) => 52, "名詞,固有名詞,一般",
    POS::Meishi(Meishi::KoyuMeishi(KoyuMeishi::Person(Person::General))) => 53, "名詞,固有名詞,人名,一般",
    POS::Meishi(Meishi::KoyuMeishi(KoyuMeishi::Person(Person::Sei))) => 54, "名詞,固有名詞,人名,姓",
    POS::Meishi(Meishi::KoyuMeishi(KoyuMeishi::Person(Person::Mei))) => 55, "名詞,固有名詞,人名,名",
    POS::Meishi(Meishi::KoyuMeishi(KoyuMeishi::Organization)) => 56, "名詞,固有名詞,組織",
    POS::Meishi(Meishi::KoyuMeishi(KoyuMeishi::Region(Region::General))) => 57, "名詞,固有名詞,地域,一般",
    POS::Meishi(Meishi::KoyuMeishi(KoyuMeishi::Region(Region::Country))) => 58, "名詞,固有名詞,地域,国",

    // ---- 名詞・接尾 ----
    POS::Meishi(Meishi::Setsubi(Setsubi::SahenSetsuzoku)) => 59, "名詞,接尾,サ変接続",
    POS::Meishi(Meishi::Setsubi(Setsubi::General)) => 60, "名詞,接尾,一般",
    POS::Meishi(Meishi::Setsubi(Setsubi::KeiyoudoushiGokan)) => 61, "名詞,接尾,形容動詞語幹",
    POS::Meishi(Meishi::Setsubi(Setsubi::Josuushi)) => 62, "名詞,接尾,助数詞",
    POS::Meishi(Meishi::Setsubi(Setsubi::JodoushiGokan)) => 63, "名詞,接尾,助動詞語幹",
    POS::Meishi(Meishi::Setsubi(Setsubi::Person)) => 64, "名詞,接尾,人名",
    POS::Meishi(Meishi::Setsubi(Setsubi::Region)) => 65, "名詞,接尾,地域",
    POS::Meishi(Meishi::Setsubi(Setsubi::Special)) => 66, "名詞,接尾,特殊",
    POS::Meishi(Meishi::Setsubi(Setsubi::FukushiKanou)) => 67, "名詞,接尾,副詞可能",

    // ---- 名詞・代名詞 ----
    POS::Meishi(Meishi::Daimeishi(Daimeishi::General)) => 68, "名詞,代名詞,一般",
    POS::Meishi(Meishi::Daimeishi(Daimeishi::Contraction)) => 69, "名詞,代名詞,縮約",

    // ---- 名詞・非自立 ----
    POS::Meishi(Meishi::Hijiritsu(MeishiHijiritsu::General)) => 70, "名詞,非自立,一般",
    POS::Meishi(Meishi::Hijiritsu(MeishiHijiritsu::KeiyoudoushiGokan)) => 71, "名詞,非自立,形容動詞語幹",
    POS::Meishi(Meishi::Hijiritsu(MeishiHijiritsu::JodoushiGokan)) => 72, "名詞,非自立,助動詞語幹",
    POS::Meishi(Meishi::Hijiritsu(MeishiHijiritsu::FukushiKanou)) => 73, "名詞,非自立,副詞可能",
    POS::Meishi(Meishi::Hijiritsu(MeishiHijiritsu::None)) => 74, "名詞,非自立",
}
