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

impl Four {
    pub(crate) fn to_u8(&self) -> u8 {
        match self {
            Self::Ka => 0,
            Self::Ga => 1,
            Self::Sa => 2,
            Self::Ta => 3,
            Self::Ba => 4,
            Self::Ma => 5,
            Self::Ra => 6,
            Self::Ha => 7,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::Ka,
            1 => Self::Ga,
            2 => Self::Sa,
            3 => Self::Ta,
            4 => Self::Ba,
            5 => Self::Ma,
            6 => Self::Ra,
            7 => Self::Ha,
            _ => panic!("Invalid u8 value for Four: {}", n),
        }
    }
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
