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

impl Special {
    pub(crate) fn to_u8(self) -> u8 {
        match self {
            Self::Nai => 0,
            Self::Tai => 1,
            Self::Ta => 2,
            Self::Da => 3,
            Self::Desu => 4,
            Self::Dosu => 5,
            Self::Ja => 6,
            Self::Masu => 7,
            Self::Nu => 8,
            Self::Ya => 9,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::Nai,
            1 => Self::Tai,
            2 => Self::Ta,
            3 => Self::Da,
            4 => Self::Desu,
            5 => Self::Dosu,
            6 => Self::Ja,
            7 => Self::Masu,
            8 => Self::Nu,
            9 => Self::Ya,
            _ => panic!("Invalid u8 value for Special: {}", n),
        }
    }
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
