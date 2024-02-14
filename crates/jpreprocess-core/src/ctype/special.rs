use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{CTypeKind, CTypeParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 特殊
pub enum Special {
    /// ナイ
    Nai,
    /// タイ
    Tai,
    /// タ
    Ta,
    /// ダ
    Da,
    /// デス
    Desu,
    /// ドス
    Dosu,
    /// ジャ
    Ja,
    /// マス
    Masu,
    /// ヌ
    Nu,
    /// ヤ
    Ya,
}

impl FromStr for Special {
    type Err = CTypeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ナイ" => Ok(Self::Nai),
            "タイ" => Ok(Self::Tai),
            "タ" => Ok(Self::Ta),
            "ダ" => Ok(Self::Da),
            "デス" => Ok(Self::Desu),
            "ドス" => Ok(Self::Dosu),
            "ジャ" => Ok(Self::Ja),
            "マス" => Ok(Self::Masu),
            "ヌ" => Ok(Self::Nu),
            "ヤ" => Ok(Self::Ya),
            _ => Err(CTypeParseError::new(s.to_string(), CTypeKind::Special)),
        }
    }
}

impl Display for Special {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::Nai => "ナイ",
            Self::Tai => "タイ",
            Self::Ta => "タ",
            Self::Da => "ダ",
            Self::Desu => "デス",
            Self::Dosu => "ドス",
            Self::Ja => "ジャ",
            Self::Masu => "マス",
            Self::Nu => "ヌ",
            Self::Ya => "ヤ",
        })
    }
}
