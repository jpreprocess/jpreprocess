use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{CTypeKind, CTypeParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 上二
pub enum UpperTwo {
    /// ダ行
    Da,
    /// ハ行
    Ha,
}

impl UpperTwo {
    pub(crate) fn to_u8(self) -> u8 {
        match self {
            Self::Da => 0,
            Self::Ha => 1,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::Da,
            1 => Self::Ha,
            _ => panic!("Invalid u8 value for UpperTwo: {}", n),
        }
    }
}

impl FromStr for UpperTwo {
    type Err = CTypeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ダ行" => Ok(Self::Da),
            "ハ行" => Ok(Self::Ha),
            _ => Err(CTypeParseError::new(s.to_string(), CTypeKind::UpperTwo)),
        }
    }
}

impl Display for UpperTwo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::Da => "ダ行",
            Self::Ha => "ハ行",
        })
    }
}
