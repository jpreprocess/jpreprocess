use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{CTypeKind, CTypeParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// カ変
pub enum KaIrregular {
    /// クル
    Katakana,
    /// 来ル
    Kanji,
}

impl FromStr for KaIrregular {
    type Err = CTypeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "クル" => Ok(Self::Katakana),
            "来ル" => Ok(Self::Kanji),
            _ => Err(CTypeParseError::new(s.to_string(), CTypeKind::KaIrregular)),
        }
    }
}

impl Display for KaIrregular {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::Katakana => "クル",
            Self::Kanji => "来ル",
        })
    }
}
