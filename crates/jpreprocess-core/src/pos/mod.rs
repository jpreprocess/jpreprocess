use serde::{Deserialize, Serialize};
use std::str::FromStr;

mod joshi;
mod meishi;
mod simple;

pub use joshi::*;
pub use meishi::*;
pub use simple::*;

use crate::{error::JPreprocessErrorKind, JPreprocessResult};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 品詞
pub enum POS {
    /// フィラー
    Filler,
    /// 感動詞
    Kandoushi,
    /// 記号
    Kigou(Kigou),
    /// 形容詞
    Keiyoushi(Keiyoushi),
    /// 助詞
    Joshi(Joshi),
    /// 助動詞
    Jodoushi,
    /// 接続詞
    Setsuzokushi,
    /// 接頭詞
    Settoushi(Settoushi),
    /// 動詞
    Doushi(Doushi),
    /// 副詞
    Fukushi(Fukushi),
    /// 名詞
    Meishi(Meishi),
    /// 連体詞
    Rentaishi,

    /// その他
    Others,

    /// 不明
    Unknown,
}

impl POS {
    pub fn from_strs(g0: &str, g1: &str, g2: &str, g3: &str) -> JPreprocessResult<Self> {
        match g0 {
            "フィラー" => Ok(Self::Filler),
            "感動詞" => Ok(Self::Kandoushi),
            "記号" => Kigou::from_str(g1).map(|kigou| Self::Kigou(kigou)),
            "形容詞" => Keiyoushi::from_str(g1).map(|keiyoushi| Self::Keiyoushi(keiyoushi)),
            "助詞" => Joshi::from_strs(g1, g2).map(|joshi| Self::Joshi(joshi)),
            "助動詞" => Ok(Self::Jodoushi),
            "接続詞" => Ok(Self::Setsuzokushi),
            "接頭詞" => Settoushi::from_str(g1).map(|settoushi| Self::Settoushi(settoushi)),
            "動詞" => Doushi::from_str(g1).map(|doushi| Self::Doushi(doushi)),
            "副詞" => Fukushi::from_str(g1).map(|fukushi| Self::Fukushi(fukushi)),
            "名詞" => Meishi::from_strs(g1, g2, g3).map(|meishi| Self::Meishi(meishi)),
            "連体詞" => Ok(Self::Rentaishi),

            "その他" => Ok(Self::Others),

            "*" => Ok(Self::Unknown),

            _ => Err(JPreprocessErrorKind::PartOfSpeechParseError
                .with_error(anyhow::anyhow!("Parse failed in POS"))),
        }
    }

    pub fn is_kazu(&self) -> bool {
        matches!(self, Self::Kigou(Kigou::Kazu) | Self::Meishi(Meishi::Kazu))
    }

    pub fn convert_to_kigou(&mut self) {
        *self = match self {
            Self::Kigou(kigou) => Self::Kigou(*kigou),
            Self::Meishi(Meishi::Kazu) => Self::Kigou(Kigou::Kazu),
            Self::Fukushi(Fukushi::General) | Self::Meishi(Meishi::General) => {
                Self::Kigou(Kigou::General)
            }
            _ => Self::Kigou(Kigou::None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_strs() {
        assert!(matches!(
            POS::from_strs("フィラー", "*", "*", "*"),
            Ok(POS::Filler)
        ));
        assert!(matches!(
            POS::from_strs("助詞", "副助詞／並立助詞／終助詞", "*", "*"),
            Ok(POS::Joshi(Joshi::FukuHeiritsuShuJoshi))
        ));
        assert!(matches!(
            POS::from_strs("名詞", "固有名詞", "人名", "姓"),
            Ok(POS::Meishi(Meishi::KoyuMeishi(KoyuMeishi::Person(
                Person::Sei
            ))))
        ));
    }
}
