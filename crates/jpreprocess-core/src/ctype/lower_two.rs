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
