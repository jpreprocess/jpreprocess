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
