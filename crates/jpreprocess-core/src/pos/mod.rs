use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

mod doushi;
mod fukushi;
mod joshi;
mod keiyoushi;
mod kigou;
mod meishi;
mod settoushi;

pub use self::{doushi::*, fukushi::*, joshi::*, keiyoushi::*, kigou::*, meishi::*, settoushi::*};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("Tried to parse {string} (depth: {depth}), but failed in {kind}")]
pub struct POSParseError {
    depth: u8,
    string: String,
    kind: POSKind,
}
impl POSParseError {
    pub(crate) fn new(depth: u8, string: String, kind: POSKind) -> Self {
        Self {
            depth,
            string,
            kind,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum POSKind {
    POSMajor,
    Kigou,
    Keiyoushi,
    Joshi,
    Settoushi,
    Doushi,
    Fukushi,
    Meishi,
    KakuJoshi,
    KoyuMeishi,
    Person,
    Region,
    MeishiSetsubi,
    Daimeishi,
    MeishiHijiritsu,
}
impl Display for POSKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::POSMajor => "品詞",
            Self::Kigou => "記号",
            Self::Keiyoushi => "形容詞",
            Self::Joshi => "助詞",
            Self::Settoushi => "接頭詞",
            Self::Doushi => "動詞",
            Self::Fukushi => "副詞",
            Self::Meishi => "名詞",
            Self::KakuJoshi => "格助詞",
            Self::KoyuMeishi => "固有名詞",
            Self::Person => "人名（固有名詞）",
            Self::Region => "地域（固有名詞）",
            Self::MeishiSetsubi => "接尾（名詞）",
            Self::Daimeishi => "代名詞",
            Self::MeishiHijiritsu => "非自立（名詞）",
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize, Default)]
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
    #[default]
    Unknown,
}

impl POS {
    pub fn from_strs(g0: &str, g1: &str, g2: &str, g3: &str) -> Result<Self, POSParseError> {
        match g0 {
            "フィラー" => Ok(Self::Filler),
            "感動詞" => Ok(Self::Kandoushi),
            "記号" => Kigou::from_str(g1).map(Self::Kigou),
            "形容詞" => Keiyoushi::from_str(g1).map(Self::Keiyoushi),
            "助詞" => Joshi::from_strs(g1, g2).map(Self::Joshi),
            "助動詞" => Ok(Self::Jodoushi),
            "接続詞" => Ok(Self::Setsuzokushi),
            "接頭詞" => Settoushi::from_str(g1).map(Self::Settoushi),
            "動詞" => Doushi::from_str(g1).map(Self::Doushi),
            "副詞" => Fukushi::from_str(g1).map(Self::Fukushi),
            "名詞" => Meishi::from_strs(g1, g2, g3).map(Self::Meishi),
            "連体詞" => Ok(Self::Rentaishi),

            "その他" => Ok(Self::Others),

            "*" => Ok(Self::Unknown),

            _ => Err(POSParseError::new(0, g0.to_string(), POSKind::POSMajor)),
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

impl Display for POS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::Filler => "フィラー",
            Self::Kandoushi => "感動詞",
            Self::Kigou(_) => "記号",
            Self::Keiyoushi(_) => "形容詞",
            Self::Joshi(_) => "助詞",
            Self::Jodoushi => "助動詞",
            Self::Setsuzokushi => "接続詞",
            Self::Settoushi(_) => "接頭詞",
            Self::Doushi(_) => "動詞",
            Self::Fukushi(_) => "副詞",
            Self::Meishi(_) => "名詞",
            Self::Rentaishi => "連体詞",

            Self::Others => "その他",

            Self::Unknown => "*",
        })?;

        match &self {
            Self::Kigou(kigou) => write!(f, ",{}", kigou),
            Self::Keiyoushi(keiyoushi) => write!(f, ",{}", keiyoushi),
            Self::Joshi(joshi) => write!(f, ",{}", joshi),
            Self::Settoushi(settoushi) => write!(f, ",{}", settoushi),
            Self::Doushi(doushi) => write!(f, ",{}", doushi),
            Self::Fukushi(fukushi) => write!(f, ",{}", fukushi),
            Self::Meishi(meishi) => write!(f, ",{}", meishi),

            _ => f.write_str(",*,*,*"),
        }?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filler() {
        let pos = POS::from_strs("フィラー", "*", "*", "*").unwrap();
        assert!(matches!(pos, POS::Filler));
        assert_eq!(pos.to_string(), "フィラー,*,*,*")
    }

    #[test]
    fn joshi() {
        let pos = POS::from_strs("助詞", "副助詞／並立助詞／終助詞", "*", "*").unwrap();
        assert!(matches!(pos, POS::Joshi(Joshi::FukuHeiritsuShuJoshi)));
        assert_eq!(pos.to_string(), "助詞,副助詞／並立助詞／終助詞,*,*")
    }

    #[test]
    fn meishi() {
        let pos = POS::from_strs("名詞", "*", "*", "*").unwrap();
        assert!(matches!(pos, POS::Meishi(Meishi::None)));
        assert_eq!(pos.to_string(), "名詞,*,*,*")
    }

    #[test]
    fn koyumeishi() {
        let pos = POS::from_strs("名詞", "固有名詞", "人名", "姓").unwrap();
        assert!(matches!(
            pos,
            POS::Meishi(Meishi::KoyuMeishi(KoyuMeishi::Person(Person::Sei)))
        ));
        assert_eq!(pos.to_string(), "名詞,固有名詞,人名,姓")
    }
}
