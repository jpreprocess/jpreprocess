use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{error::JPreprocessErrorKind, JPreprocessError};

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

impl FromStr for Fukushi {
    type Err = JPreprocessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::None),
            "一般" => Ok(Self::General),
            "助詞類接続" => Ok(Self::JoshiruiSetsuzoku),
            _ => Err(JPreprocessErrorKind::PartOfSpeechParseError
                .with_error(anyhow::anyhow!("Parse failed in Fukushi"))),
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
