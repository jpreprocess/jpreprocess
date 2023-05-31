use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{error::JPreprocessErrorKind, JPreprocessError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
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

impl FromStr for Settoushi {
    type Err = JPreprocessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "形容詞接続" => Ok(Self::KeiyoushiSetsuzoku),
            "数接続" => Ok(Self::SuuSetsuzoku),
            "動詞接続" => Ok(Self::DoushiSetsuzoku),
            "名詞接続" => Ok(Self::MeishiSetsuzoku),
            _ => Err(JPreprocessErrorKind::PartOfSpeechParseError
                .with_error(anyhow::anyhow!("Parse failed in Settoushi"))),
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
