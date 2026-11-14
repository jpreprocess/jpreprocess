use crate::ctype::{
    CType, Five, Four, KaIrregular, Keiyoushi, LowerTwo, Old, One, SaIrregular, Special, UpperTwo,
};

#[derive(Debug, thiserror::Error)]
pub enum CTypeParseError {
    #[error("Invalid CType string: {0}")]
    InvalidString(String),
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

pub fn ctype_to_string(ctype: &CType) -> String {
    let string = get_by_ctype(ctype);
    string.to_string()
}

macro_rules! ctype_conversions {
    (@map $( $ctype:expr => $string:literal ),* $(,)?) => {
        const STRING_CTYPE_CONVERSION_TABLE: phf::Map<&'static str, CType> = phf::phf_map! {
            $($string => $ctype,)*
        };
    };
    (@match $( $ctype:pat => $string:literal ),* $(,)?) => {
        fn get_by_ctype(ctype: &CType) -> &'static str {
            match ctype {
                $($ctype => $string,)*
            }
        }
    };

    ($($tokens:tt)*) => {
        ctype_conversions!(@map $($tokens)*);
        ctype_conversions!(@match $($tokens)*);
    };
}

ctype_conversions! {
    CType::KaIrregular(KaIrregular::Katakana) => "カ変・クル",
    CType::KaIrregular(KaIrregular::Kanji) => "カ変・来ル",
    CType::SaIrregular(SaIrregular::Alone) => "サ変・スル",
    CType::SaIrregular(SaIrregular::ConjugationSuru) => "サ変・－スル",
    CType::SaIrregular(SaIrregular::ConjugationZuru) => "サ変・－ズル",
    CType::RaIrregular => "ラ変",
    CType::One(One::Yameru) => "一段・病メル",
    CType::One(One::Kureru) => "一段・クレル",
    CType::One(One::Eru) => "一段・得ル",
    CType::One(One::Ru) => "一段・ル",
    CType::One(One::None) => "一段",
    CType::LowerTwo(LowerTwo::A) => "下二・ア行",
    CType::LowerTwo(LowerTwo::Ka) => "下二・カ行",
    CType::LowerTwo(LowerTwo::Ga) => "下二・ガ行",
    CType::LowerTwo(LowerTwo::Sa) => "下二・サ行",
    CType::LowerTwo(LowerTwo::Za) => "下二・ザ行",
    CType::LowerTwo(LowerTwo::Ta) => "下二・タ行",
    CType::LowerTwo(LowerTwo::Da) => "下二・ダ行",
    CType::LowerTwo(LowerTwo::Na) => "下二・ナ行",
    CType::LowerTwo(LowerTwo::Ha) => "下二・ハ行",
    CType::LowerTwo(LowerTwo::Ba) => "下二・バ行",
    CType::LowerTwo(LowerTwo::Ma) => "下二・マ行",
    CType::LowerTwo(LowerTwo::Ya) => "下二・ヤ行",
    CType::LowerTwo(LowerTwo::Ra) => "下二・ラ行",
    CType::LowerTwo(LowerTwo::Wa) => "下二・ワ行",
    CType::LowerTwo(LowerTwo::Get) => "下二・得",
    CType::Keiyoushi(Keiyoushi::Auo) => "形容詞・アウオ段",
    CType::Keiyoushi(Keiyoushi::I) => "形容詞・イ段",
    CType::Keiyoushi(Keiyoushi::Ii) => "形容詞・イイ",
    CType::Five(Five::KaI) => "五段・カ行イ音便",
    CType::Five(Five::KaDouble) => "五段・カ行促音便",
    CType::Five(Five::KaDoubleYuku) => "五段・カ行促音便ユク",
    CType::Five(Five::Ga) => "五段・ガ行",
    CType::Five(Five::Sa) => "五段・サ行",
    CType::Five(Five::Ta) => "五段・タ行",
    CType::Five(Five::Na) => "五段・ナ行",
    CType::Five(Five::Ba) => "五段・バ行",
    CType::Five(Five::Ma) => "五段・マ行",
    CType::Five(Five::Ra) => "五段・ラ行",
    CType::Five(Five::RaAru) => "五段・ラ行アル",
    CType::Five(Five::RaSpecial) => "五段・ラ行特殊",
    CType::Five(Five::WaU) => "五段・ワ行ウ音便",
    CType::Five(Five::WaDouble) => "五段・ワ行促音便",
    CType::Four(Four::Ka) => "四段・カ行",
    CType::Four(Four::Ga) => "四段・ガ行",
    CType::Four(Four::Sa) => "四段・サ行",
    CType::Four(Four::Ta) => "四段・タ行",
    CType::Four(Four::Ba) => "四段・バ行",
    CType::Four(Four::Ma) => "四段・マ行",
    CType::Four(Four::Ra) => "四段・ラ行",
    CType::Four(Four::Ha) => "四段・ハ行",
    CType::UpperTwo(UpperTwo::Da) => "上二・ダ行",
    CType::UpperTwo(UpperTwo::Ha) => "上二・ハ行",
    CType::Special(Special::Nai) => "特殊・ナイ",
    CType::Special(Special::Tai) => "特殊・タイ",
    CType::Special(Special::Ta) => "特殊・タ",
    CType::Special(Special::Da) => "特殊・ダ",
    CType::Special(Special::Desu) => "特殊・デス",
    CType::Special(Special::Dosu) => "特殊・ドス",
    CType::Special(Special::Ja) => "特殊・ジャ",
    CType::Special(Special::Masu) => "特殊・マス",
    CType::Special(Special::Nu) => "特殊・ヌ",
    CType::Special(Special::Ya) => "特殊・ヤ",
    CType::NoConjugation => "不変化型",
    CType::Old(Old::Beshi) => "文語・ベシ",
    CType::Old(Old::Gotoshi) => "文語・ゴトシ",
    CType::Old(Old::Nari) => "文語・ナリ",
    CType::Old(Old::Maji) => "文語・マジ",
    CType::Old(Old::Shimu) => "文語・シム",
    CType::Old(Old::Ki) => "文語・キ",
    CType::Old(Old::Keri) => "文語・ケリ",
    CType::Old(Old::Ru) => "文語・ル",
    CType::Old(Old::Ri) => "文語・リ",
    CType::None => "*",
}
