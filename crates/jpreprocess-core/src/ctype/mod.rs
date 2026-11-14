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

mod conversions;

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

pub use conversions::CTypeParseError;

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
        conversions::ctype_to_value(&self)
    }
    pub(crate) fn from_u8(n: u8) -> Result<Self, CTypeParseError> {
        conversions::value_to_ctype(n)
    }
}

impl FromStr for CType {
    type Err = CTypeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        conversions::string_to_ctype(s)
    }
}

impl Display for CType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&conversions::ctype_to_string(self))
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
