use crate::ctype::{
    CType, Five, Four, KaIrregular, Keiyoushi, LowerTwo, Old, One, SaIrregular, Special, UpperTwo,
};

#[derive(Debug, thiserror::Error)]
pub enum CTypeParseError {
    #[error("Invalid CType string: {0}")]
    InvalidString(String),
    #[error("Invalid CType value: {0}")]
    InvalidValue(u8),
}

pub fn string_to_ctype(ctype_string: &str) -> Result<CType, CTypeParseError> {
    if let Some(ctype) = STRING_CTYPE_CONVERSION_TABLE.get(ctype_string) {
        return Ok(*ctype);
    }

    // "−" minus sign (U+2212) to "－" fullwidth hyphen-minus (U+FF0D)
    if let Some(ctype) =
        STRING_CTYPE_CONVERSION_TABLE.get(&ctype_string.replace("\u{2212}", "\u{FF0D}"))
    {
        return Ok(*ctype);
    }

    if ctype_string == "カ往促音便" {
        eprintln!("WARN: Unrecognized CType カ往促音便. Processed as カ行促音便.");
        return Ok(CType::Five(Five::KaDouble));
    }

    Err(CTypeParseError::InvalidString(ctype_string.to_string()))
}

pub fn value_to_ctype(ctype_value: u8) -> Result<CType, CTypeParseError> {
    if ctype_value as usize >= VALUE_CTYPE_CONVERSION_TABLE.len() {
        return Err(CTypeParseError::InvalidValue(ctype_value));
    }
    Ok(VALUE_CTYPE_CONVERSION_TABLE[ctype_value as usize])
}

pub fn ctype_to_string(ctype: &CType) -> String {
    let (string, _) = get_by_ctype(ctype);
    string.to_string()
}

pub fn ctype_to_value(ctype: &CType) -> u8 {
    let (_, value) = get_by_ctype(ctype);
    value
}

macro_rules! ctype_conversions {
    (@map $( $ctype:expr => $value:literal, $string:literal ),* $(,)?) => {
        const STRING_CTYPE_CONVERSION_TABLE: phf::Map<&'static str, CType> = phf::phf_map! {
            $($string => $ctype,)*
        };
        const VALUE_CTYPE_CONVERSION_TABLE: [CType; 75] = {
            let mut table = [CType::None; 75];
            $(
                table[$value as usize] = $ctype;
            )*
            table
        };
    };
    (@match $( $ctype:pat => $value:literal, $string:literal ),* $(,)?) => {
        fn get_by_ctype(ctype: &CType) -> (&'static str, u8) {
            match ctype {
                $($ctype => ($string, $value),)*
            }
        }
    };

    ($($tokens:tt)*) => {
        ctype_conversions!(@map $($tokens)*);
        ctype_conversions!(@match $($tokens)*);
    };
}

ctype_conversions! {
    CType::KaIrregular(KaIrregular::Katakana) => 0, "カ変・クル",
    CType::KaIrregular(KaIrregular::Kanji) => 1, "カ変・来ル",
    CType::SaIrregular(SaIrregular::Alone) => 2, "サ変・スル",
    CType::SaIrregular(SaIrregular::ConjugationSuru) => 3, "サ変・－スル",
    CType::SaIrregular(SaIrregular::ConjugationZuru) => 4, "サ変・－ズル",
    CType::RaIrregular => 5, "ラ変",
    CType::One(One::Yameru) => 6, "一段・病メル",
    CType::One(One::Kureru) => 7, "一段・クレル",
    CType::One(One::Eru) => 8, "一段・得ル",
    CType::One(One::Ru) => 9, "一段・ル",
    CType::One(One::None) => 10, "一段",
    CType::LowerTwo(LowerTwo::A) => 11, "下二・ア行",
    CType::LowerTwo(LowerTwo::Ka) => 12, "下二・カ行",
    CType::LowerTwo(LowerTwo::Ga) => 13, "下二・ガ行",
    CType::LowerTwo(LowerTwo::Sa) => 14, "下二・サ行",
    CType::LowerTwo(LowerTwo::Za) => 15, "下二・ザ行",
    CType::LowerTwo(LowerTwo::Ta) => 16, "下二・タ行",
    CType::LowerTwo(LowerTwo::Da) => 17, "下二・ダ行",
    CType::LowerTwo(LowerTwo::Na) => 18, "下二・ナ行",
    CType::LowerTwo(LowerTwo::Ha) => 19, "下二・ハ行",
    CType::LowerTwo(LowerTwo::Ba) => 20, "下二・バ行",
    CType::LowerTwo(LowerTwo::Ma) => 21, "下二・マ行",
    CType::LowerTwo(LowerTwo::Ya) => 22, "下二・ヤ行",
    CType::LowerTwo(LowerTwo::Ra) => 23, "下二・ラ行",
    CType::LowerTwo(LowerTwo::Wa) => 24, "下二・ワ行",
    CType::LowerTwo(LowerTwo::Get) => 25, "下二・得",
    CType::Keiyoushi(Keiyoushi::Auo) => 26, "形容詞・アウオ段",
    CType::Keiyoushi(Keiyoushi::I) => 27, "形容詞・イ段",
    CType::Keiyoushi(Keiyoushi::Ii) => 28, "形容詞・イイ",
    CType::Five(Five::KaI) => 29, "五段・カ行イ音便",
    CType::Five(Five::KaDouble) => 30, "五段・カ行促音便",
    CType::Five(Five::KaDoubleYuku) => 31, "五段・カ行促音便ユク",
    CType::Five(Five::Ga) => 32, "五段・ガ行",
    CType::Five(Five::Sa) => 33, "五段・サ行",
    CType::Five(Five::Ta) => 34, "五段・タ行",
    CType::Five(Five::Na) => 35, "五段・ナ行",
    CType::Five(Five::Ba) => 36, "五段・バ行",
    CType::Five(Five::Ma) => 37, "五段・マ行",
    CType::Five(Five::Ra) => 38, "五段・ラ行",
    CType::Five(Five::RaAru) => 39, "五段・ラ行アル",
    CType::Five(Five::RaSpecial) => 40, "五段・ラ行特殊",
    CType::Five(Five::WaU) => 41, "五段・ワ行ウ音便",
    CType::Five(Five::WaDouble) => 42, "五段・ワ行促音便",
    CType::Four(Four::Ka) => 43, "四段・カ行",
    CType::Four(Four::Ga) => 44, "四段・ガ行",
    CType::Four(Four::Sa) => 45, "四段・サ行",
    CType::Four(Four::Ta) => 46, "四段・タ行",
    CType::Four(Four::Ba) => 47, "四段・バ行",
    CType::Four(Four::Ma) => 48, "四段・マ行",
    CType::Four(Four::Ra) => 49, "四段・ラ行",
    CType::Four(Four::Ha) => 50, "四段・ハ行",
    CType::UpperTwo(UpperTwo::Da) => 51, "上二・ダ行",
    CType::UpperTwo(UpperTwo::Ha) => 52, "上二・ハ行",
    CType::Special(Special::Nai) => 53, "特殊・ナイ",
    CType::Special(Special::Tai) => 54, "特殊・タイ",
    CType::Special(Special::Ta) => 55, "特殊・タ",
    CType::Special(Special::Da) => 56, "特殊・ダ",
    CType::Special(Special::Desu) => 57, "特殊・デス",
    CType::Special(Special::Dosu) => 58, "特殊・ドス",
    CType::Special(Special::Ja) => 59, "特殊・ジャ",
    CType::Special(Special::Masu) => 60, "特殊・マス",
    CType::Special(Special::Nu) => 61, "特殊・ヌ",
    CType::Special(Special::Ya) => 62, "特殊・ヤ",
    CType::NoConjugation => 63, "不変化型",
    CType::Old(Old::Beshi) => 64, "文語・ベシ",
    CType::Old(Old::Gotoshi) => 65, "文語・ゴトシ",
    CType::Old(Old::Nari) => 66, "文語・ナリ",
    CType::Old(Old::Maji) => 67, "文語・マジ",
    CType::Old(Old::Shimu) => 68, "文語・シム",
    CType::Old(Old::Ki) => 69, "文語・キ",
    CType::Old(Old::Keri) => 70, "文語・ケリ",
    CType::Old(Old::Ru) => 71, "文語・ル",
    CType::Old(Old::Ri) => 72, "文語・リ",
    CType::None => 73, "*",
}
