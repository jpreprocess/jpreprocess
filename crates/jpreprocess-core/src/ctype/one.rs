use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{CTypeKind, CTypeParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 一段
pub enum One {
    /// 病メル
    Yameru,
    /// クレル
    Kureru,
    /// 得ル
    Eru,
    /// ル
    Ru,
    /// (Empty)
    None,
}

impl FromStr for One {
    type Err = CTypeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "病メル" => Ok(Self::Yameru),
            "クレル" => Ok(Self::Kureru),
            "得ル" => Ok(Self::Eru),
            "ル" => Ok(Self::Ru),
            "" => Ok(Self::None),
            _ => Err(CTypeParseError::new(s.to_string(), CTypeKind::One)),
        }
    }
}

impl Display for One {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::Yameru => "病メル",
            Self::Kureru => "クレル",
            Self::Eru => "得ル",
            Self::Ru => "ル",
            Self::None => "",
        })
    }
}
