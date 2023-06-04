use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{error::JPreprocessErrorKind, JPreprocessError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 上二
pub enum UpperTwo {
    /// ダ行
    Da,
    /// ハ行
    Ha,
}

impl FromStr for UpperTwo {
    type Err = JPreprocessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ダ行" => Ok(Self::Da),
            "ハ行" => Ok(Self::Ha),
            _ => Err(JPreprocessErrorKind::CTypeParseError
                .with_error(anyhow::anyhow!("Parse failed in UpperTwo"))),
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
