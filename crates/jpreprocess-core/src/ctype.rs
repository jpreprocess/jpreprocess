use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

use crate::{error::JPreprocessErrorKind, JPreprocessError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 活用
pub enum CType {
    /// カ変
    KaIrregular,
    /// サ変
    SaIrregular,
    /// ラ変
    RaIrregular,
    /// 一段
    One,
    /// 下二
    LowerTwo,
    /// 形容詞
    Keiyoushi,
    /// 五段
    Five,
    /// 四段
    Four,
    /// 上二
    UpperTwo,
    /// 特殊
    Special,
    /// 不変化型
    NoConjugation,
    /// 文語
    Old,

    /// \*
    None,
}

impl FromStr for CType {
    type Err = JPreprocessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (major, _minor) = s
            .split_once('・')
            .map(|(major, minor)| (major, Some(minor)))
            .unwrap_or((s, None));
        match major {
            "カ変" => Ok(Self::KaIrregular),
            "サ変" => Ok(Self::SaIrregular),
            "ラ変" => Ok(Self::RaIrregular),
            "一段" => Ok(Self::One),
            "下二" => Ok(Self::LowerTwo),
            "形容詞" => Ok(Self::Keiyoushi),
            "五段" => Ok(Self::Five),
            "四段" => Ok(Self::Four),
            "上二" => Ok(Self::UpperTwo),
            "特殊" => Ok(Self::Special),
            "不変化型" => Ok(Self::NoConjugation),
            "文語" => Ok(Self::Old),
            "*" => Ok(Self::None),

            _ => Err(JPreprocessErrorKind::CTypeParseError
                .with_error(anyhow::anyhow!("Parse failed in CType major"))),
        }
    }
}

impl Display for CType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Current implementation loses the latter half of CType
        f.write_str(match &self {
            Self::KaIrregular => "カ変",
            Self::SaIrregular => "サ変",
            Self::RaIrregular => "ラ変",
            Self::One => "一段",
            Self::LowerTwo => "下二",
            Self::Keiyoushi => "形容詞",
            Self::Five => "五段",
            Self::Four => "四段",
            Self::UpperTwo => "上二",
            Self::Special => "特殊",
            Self::NoConjugation => "不変化型",
            Self::Old => "文語",

            Self::None => "*",
        })
    }
}
