use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

mod five;
mod four;
mod ka_irregular;
mod keiyoushi;
mod lower_two;
mod old;
mod one;
mod sa_irregular;
mod special;
mod upper_two;

pub use five::*;
pub use four::*;
pub use ka_irregular::*;
pub use keiyoushi::*;
pub use lower_two::*;
pub use old::*;
pub use one::*;
pub use sa_irregular::*;
pub use special::*;
pub use upper_two::*;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("Tried to parse {string}, but failed in {kind}")]
pub struct CTypeParseError {
    string: String,
    kind: CTypeKind,
}
impl CTypeParseError {
    pub(crate) fn new(string: String, kind: CTypeKind) -> Self {
        Self { string, kind }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CTypeKind {
    CTypeMajor,
    Five,
    Four,
    KaIrregular,
    Keiyoushi,
    LowerTwo,
    Old,
    One,
    SaIrregular,
    Special,
    UpperTwo,
}
impl Display for CTypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::CTypeMajor => "活用形",
            Self::Five => "五段",
            Self::Four => "四段",
            Self::KaIrregular => "カ変",
            Self::Keiyoushi => "形容詞",
            Self::LowerTwo => "下二",
            Self::Old => "文語",
            Self::One => "一段",
            Self::SaIrregular => "サ変",
            Self::Special => "特殊",
            Self::UpperTwo => "上二",
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize, Default)]
/// 活用
pub enum CType {
    /// カ変
    KaIrregular(KaIrregular),
    /// サ変
    SaIrregular(SaIrregular),
    /// ラ変
    RaIrregular,
    /// 一段
    One(One),
    /// 下二
    LowerTwo(LowerTwo),
    /// 形容詞
    Keiyoushi(Keiyoushi),
    /// 五段
    Five(Five),
    /// 四段
    Four(Four),
    /// 上二
    UpperTwo(UpperTwo),
    /// 特殊
    Special(Special),
    /// 不変化型
    NoConjugation,
    /// 文語
    Old(Old),

    /// \*
    #[default]
    None,
}

impl CType {
    pub(crate) fn to_u8(self) -> u8 {
        let (major, minor) = match self {
            Self::KaIrregular(minor) => (0, minor.to_u8()),
            Self::SaIrregular(minor) => (1, minor.to_u8()),
            Self::RaIrregular => (2, 0),
            Self::One(minor) => (3, minor.to_u8()),
            Self::LowerTwo(minor) => (4, minor.to_u8()),
            Self::Keiyoushi(minor) => (5, minor.to_u8()),
            Self::Five(minor) => (6, minor.to_u8()),
            Self::Four(minor) => (7, minor.to_u8()),
            Self::UpperTwo(minor) => (8, minor.to_u8()),
            Self::Special(minor) => (9, minor.to_u8()),
            Self::NoConjugation => (10, 0),
            Self::Old(minor) => (11, minor.to_u8()),

            Self::None => (12, 0),
        };
        assert!(major < 16, "Major type must be less than 16");
        assert!(minor < 16, "Minor type must be less than 16");

        (major << 4) + minor
    }
    pub(crate) fn from_u8(n: u8) -> Self {
        let major = n >> 4;
        let minor = n & 0x0F;

        match major {
            0 => Self::KaIrregular(KaIrregular::from_u8(minor)),
            1 => Self::SaIrregular(SaIrregular::from_u8(minor)),
            2 => Self::RaIrregular,
            3 => Self::One(One::from_u8(minor)),
            4 => Self::LowerTwo(LowerTwo::from_u8(minor)),
            5 => Self::Keiyoushi(Keiyoushi::from_u8(minor)),
            6 => Self::Five(Five::from_u8(minor)),
            7 => Self::Four(Four::from_u8(minor)),
            8 => Self::UpperTwo(UpperTwo::from_u8(minor)),
            9 => Self::Special(Special::from_u8(minor)),
            10 => Self::NoConjugation,
            11 => Self::Old(Old::from_u8(minor)),
            12 => Self::None,

            _ => panic!("Invalid u8 value for CType: {}", major),
        }
    }
}

impl FromStr for CType {
    type Err = CTypeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (major, minor) = s.split_once('・').unwrap_or((s, ""));
        match major {
            "カ変" => Ok(Self::KaIrregular(KaIrregular::from_str(minor)?)),
            "サ変" => Ok(Self::SaIrregular(SaIrregular::from_str(minor)?)),
            "ラ変" => Ok(Self::RaIrregular),
            "一段" => Ok(Self::One(One::from_str(minor)?)),
            "下二" => Ok(Self::LowerTwo(LowerTwo::from_str(minor)?)),
            "形容詞" => Ok(Self::Keiyoushi(Keiyoushi::from_str(minor)?)),
            "五段" => Ok(Self::Five(Five::from_str(minor)?)),
            "四段" => Ok(Self::Four(Four::from_str(minor)?)),
            "上二" => Ok(Self::UpperTwo(UpperTwo::from_str(minor)?)),
            "特殊" => Ok(Self::Special(Special::from_str(minor)?)),
            "不変化型" => Ok(Self::NoConjugation),
            "文語" => Ok(Self::Old(Old::from_str(minor)?)),
            "*" => Ok(Self::None),

            _ => Err(CTypeParseError::new(s.to_string(), CTypeKind::CTypeMajor)),
        }
    }
}

impl Display for CType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::KaIrregular(_) => "カ変",
            Self::SaIrregular(_) => "サ変",
            Self::RaIrregular => "ラ変",
            Self::One(_) => "一段",
            Self::LowerTwo(_) => "下二",
            Self::Keiyoushi(_) => "形容詞",
            Self::Five(_) => "五段",
            Self::Four(_) => "四段",
            Self::UpperTwo(_) => "上二",
            Self::Special(_) => "特殊",
            Self::NoConjugation => "不変化型",
            Self::Old(_) => "文語",

            Self::None => "*",
        })?;

        match &self {
            Self::One(One::None) => Ok(()),

            Self::KaIrregular(minor) => write!(f, "・{}", minor),
            Self::SaIrregular(minor) => write!(f, "・{}", minor),
            Self::One(minor) => write!(f, "・{}", minor),
            Self::LowerTwo(minor) => write!(f, "・{}", minor),
            Self::Keiyoushi(minor) => write!(f, "・{}", minor),
            Self::Five(minor) => write!(f, "・{}", minor),
            Self::Four(minor) => write!(f, "・{}", minor),
            Self::UpperTwo(minor) => write!(f, "・{}", minor),
            Self::Special(minor) => write!(f, "・{}", minor),
            Self::Old(minor) => write!(f, "・{}", minor),

            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none() {
        let ctype = CType::from_str("*").unwrap();
        assert!(matches!(ctype, CType::None));
        assert_eq!(ctype.to_string(), "*")
    }

    #[test]
    fn ra_irregular() {
        let ctype = CType::from_str("ラ変").unwrap();
        assert!(matches!(ctype, CType::RaIrregular));
        assert_eq!(ctype.to_string(), "ラ変")
    }

    #[test]
    fn lower_two() {
        let ctype = CType::from_str("下二・ア行").unwrap();
        assert!(matches!(ctype, CType::LowerTwo(LowerTwo::A)));
        assert_eq!(ctype.to_string(), "下二・ア行")
    }

    #[test]
    fn one_empty() {
        let ctype = CType::from_str("一段").unwrap();
        assert!(matches!(ctype, CType::One(One::None)));
        assert_eq!(ctype.to_string(), "一段")
    }
}
