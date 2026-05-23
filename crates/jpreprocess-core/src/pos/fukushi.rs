use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{POSKind, POSParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 副詞
pub enum Fukushi {
    /// \*
    None,
    /// 一般
    General,
    /// 助詞類接続
    JoshiruiSetsuzoku,
}

impl Fukushi {
    pub(crate) fn to_u8(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::General => 1,
            Self::JoshiruiSetsuzoku => 2,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::None,
            1 => Self::General,
            2 => Self::JoshiruiSetsuzoku,
            _ => panic!("Invalid u8 value for Fukushi: {}", n),
        }
    }
}

impl FromStr for Fukushi {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::None),
            "一般" => Ok(Self::General),
            "助詞類接続" => Ok(Self::JoshiruiSetsuzoku),
            _ => Err(POSParseError::new(1, s.to_string(), POSKind::Fukushi)),
        }
    }
}

impl Display for Fukushi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},*,*",
            match &self {
                Self::None => "*",
                Self::General => "一般",
                Self::JoshiruiSetsuzoku => "助詞類接続",
            },
        )
    }
}
