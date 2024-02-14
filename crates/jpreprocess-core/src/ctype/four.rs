use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{CTypeKind, CTypeParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 四段
pub enum Four {
    /// カ行
    Ka,
    /// ガ行
    Ga,
    /// サ行
    Sa,
    /// タ行
    Ta,
    /// バ行
    Ba,
    /// マ行
    Ma,
    /// ラ行
    Ra,
    /// ハ行
    Ha,
}

impl FromStr for Four {
    type Err = CTypeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "カ行" => Ok(Self::Ka),
            "ガ行" => Ok(Self::Ga),
            "サ行" => Ok(Self::Sa),
            "タ行" => Ok(Self::Ta),
            "バ行" => Ok(Self::Ba),
            "マ行" => Ok(Self::Ma),
            "ラ行" => Ok(Self::Ra),
            "ハ行" => Ok(Self::Ha),
            _ => Err(CTypeParseError::new(s.to_string(), CTypeKind::Four)),
        }
    }
}

impl Display for Four {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::Ka => "カ行",
            Self::Ga => "ガ行",
            Self::Sa => "サ行",
            Self::Ta => "タ行",
            Self::Ba => "バ行",
            Self::Ma => "マ行",
            Self::Ra => "ラ行",
            Self::Ha => "ハ行",
        })
    }
}
