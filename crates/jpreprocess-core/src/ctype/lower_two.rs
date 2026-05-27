use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{CTypeKind, CTypeParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 下二
pub enum LowerTwo {
    /// ア行
    A,
    /// カ行
    Ka,
    /// ガ行
    Ga,
    /// サ行
    Sa,
    /// ザ行
    Za,
    /// タ行
    Ta,
    /// ダ行
    Da,
    /// ナ行
    Na,
    /// ハ行
    Ha,
    /// バ行
    Ba,
    /// マ行
    Ma,
    /// ヤ行
    Ya,
    /// ラ行
    Ra,
    /// ワ行
    Wa,
    /// 得
    Get,
}

impl LowerTwo {
    pub(crate) fn to_u8(self) -> u8 {
        match self {
            Self::A => 0,
            Self::Ka => 1,
            Self::Ga => 2,
            Self::Sa => 3,
            Self::Za => 4,
            Self::Ta => 5,
            Self::Da => 6,
            Self::Na => 7,
            Self::Ha => 8,
            Self::Ba => 9,
            Self::Ma => 10,
            Self::Ya => 11,
            Self::Ra => 12,
            Self::Wa => 13,
            Self::Get => 14,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::A,
            1 => Self::Ka,
            2 => Self::Ga,
            3 => Self::Sa,
            4 => Self::Za,
            5 => Self::Ta,
            6 => Self::Da,
            7 => Self::Na,
            8 => Self::Ha,
            9 => Self::Ba,
            10 => Self::Ma,
            11 => Self::Ya,
            12 => Self::Ra,
            13 => Self::Wa,
            14 => Self::Get,
            _ => panic!("Invalid u8 value for LowerTwo: {}", n),
        }
    }
}

impl FromStr for LowerTwo {
    type Err = CTypeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ア行" => Ok(Self::A),
            "カ行" => Ok(Self::Ka),
            "ガ行" => Ok(Self::Ga),
            "サ行" => Ok(Self::Sa),
            "ザ行" => Ok(Self::Za),
            "タ行" => Ok(Self::Ta),
            "ダ行" => Ok(Self::Da),
            "ナ行" => Ok(Self::Na),
            "ハ行" => Ok(Self::Ha),
            "バ行" => Ok(Self::Ba),
            "マ行" => Ok(Self::Ma),
            "ヤ行" => Ok(Self::Ya),
            "ラ行" => Ok(Self::Ra),
            "ワ行" => Ok(Self::Wa),
            "得" => Ok(Self::Get),
            _ => Err(CTypeParseError::new(s.to_string(), CTypeKind::LowerTwo)),
        }
    }
}

impl Display for LowerTwo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::A => "ア行",
            Self::Ka => "カ行",
            Self::Ga => "ガ行",
            Self::Sa => "サ行",
            Self::Za => "ザ行",
            Self::Ta => "タ行",
            Self::Da => "ダ行",
            Self::Na => "ナ行",
            Self::Ha => "ハ行",
            Self::Ba => "バ行",
            Self::Ma => "マ行",
            Self::Ya => "ヤ行",
            Self::Ra => "ラ行",
            Self::Wa => "ワ行",
            Self::Get => "得",
        })
    }
}
