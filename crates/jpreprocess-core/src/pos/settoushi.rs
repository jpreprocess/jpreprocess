use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{POSKind, POSParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 接頭詞
pub enum Settoushi {
    /// 形容詞接続
    KeiyoushiSetsuzoku,
    /// 数接続
    SuuSetsuzoku,
    /// 動詞接続
    DoushiSetsuzoku,
    /// 名詞接続
    MeishiSetsuzoku,
}

impl Settoushi {
    pub(crate) fn to_u8(&self) -> u8 {
        match self {
            Self::KeiyoushiSetsuzoku => 0,
            Self::SuuSetsuzoku => 1,
            Self::DoushiSetsuzoku => 2,
            Self::MeishiSetsuzoku => 3,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::KeiyoushiSetsuzoku,
            1 => Self::SuuSetsuzoku,
            2 => Self::DoushiSetsuzoku,
            3 => Self::MeishiSetsuzoku,
            _ => panic!("Invalid u8 value for Settoushi: {}", n),
        }
    }
}

impl FromStr for Settoushi {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "形容詞接続" => Ok(Self::KeiyoushiSetsuzoku),
            "数接続" => Ok(Self::SuuSetsuzoku),
            "動詞接続" => Ok(Self::DoushiSetsuzoku),
            "名詞接続" => Ok(Self::MeishiSetsuzoku),
            _ => Err(POSParseError::new(1, s.to_string(), POSKind::Settoushi)),
        }
    }
}

impl Display for Settoushi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},*,*",
            match &self {
                Self::KeiyoushiSetsuzoku => "形容詞接続",
                Self::SuuSetsuzoku => "数接続",
                Self::DoushiSetsuzoku => "動詞接続",
                Self::MeishiSetsuzoku => "名詞接続",
            },
        )
    }
}
