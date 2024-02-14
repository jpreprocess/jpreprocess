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
        // Current implementation loses the latter half of CType
        let (major, minor) = match &self {
            Self::KaIrregular(minor) => ("カ変", minor.to_string()),
            Self::SaIrregular(minor) => ("サ変", minor.to_string()),
            Self::RaIrregular => ("ラ変", "".to_string()),
            Self::One(minor) => ("一段", minor.to_string()),
            Self::LowerTwo(minor) => ("下二", minor.to_string()),
            Self::Keiyoushi(minor) => ("形容詞", minor.to_string()),
            Self::Five(minor) => ("五段", minor.to_string()),
            Self::Four(minor) => ("四段", minor.to_string()),
            Self::UpperTwo(minor) => ("上二", minor.to_string()),
            Self::Special(minor) => ("特殊", minor.to_string()),
            Self::NoConjugation => ("不変化型", "".to_string()),
            Self::Old(minor) => ("文語", minor.to_string()),

            Self::None => ("*", "".to_string()),
        };

        if minor.is_empty() {
            write!(f, "{}", major)
        } else {
            write!(f, "{}・{}", major, minor)
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
