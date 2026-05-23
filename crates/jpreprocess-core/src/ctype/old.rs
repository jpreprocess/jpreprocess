use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{CTypeKind, CTypeParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 文語
pub enum Old {
    /// ベシ
    Beshi,
    /// ゴトシ
    Gotoshi,
    /// ナリ
    Nari,
    /// マジ
    Maji,
    /// シム
    Shimu,
    /// キ
    Ki,
    /// ケリ
    Keri,
    /// ル
    Ru,
    /// リ
    Ri,
}

impl Old {
    pub(crate) fn to_u8(&self) -> u8 {
        match self {
            Self::Beshi => 0,
            Self::Gotoshi => 1,
            Self::Nari => 2,
            Self::Maji => 3,
            Self::Shimu => 4,
            Self::Ki => 5,
            Self::Keri => 6,
            Self::Ru => 7,
            Self::Ri => 8,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::Beshi,
            1 => Self::Gotoshi,
            2 => Self::Nari,
            3 => Self::Maji,
            4 => Self::Shimu,
            5 => Self::Ki,
            6 => Self::Keri,
            7 => Self::Ru,
            8 => Self::Ri,
            _ => panic!("Invalid u8 value for Old: {}", n),
        }
    }
}

impl FromStr for Old {
    type Err = CTypeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ベシ" => Ok(Self::Beshi),
            "ゴトシ" => Ok(Self::Gotoshi),
            "ナリ" => Ok(Self::Nari),
            "マジ" => Ok(Self::Maji),
            "シム" => Ok(Self::Shimu),
            "キ" => Ok(Self::Ki),
            "ケリ" => Ok(Self::Keri),
            "ル" => Ok(Self::Ru),
            "リ" => Ok(Self::Ri),
            _ => Err(CTypeParseError::new(s.to_string(), CTypeKind::Old)),
        }
    }
}

impl Display for Old {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::Beshi => "ベシ",
            Self::Gotoshi => "ゴトシ",
            Self::Nari => "ナリ",
            Self::Maji => "マジ",
            Self::Shimu => "シム",
            Self::Ki => "キ",
            Self::Keri => "ケリ",
            Self::Ru => "ル",
            Self::Ri => "リ",
        })
    }
}
