use serde::{Deserialize, Serialize};
use std::str::FromStr;

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

    /// *
    None,
}

impl FromStr for CType {
    type Err = JPreprocessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (major, _minor) = s
            .split_once("・")
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
