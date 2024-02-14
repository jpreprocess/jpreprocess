use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{POSKind, POSParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 記号
pub enum Kigou {
    /// \*
    None,
    /// アルファベット
    Alphabet,
    /// 一般
    General,
    /// 括弧開
    KakkoOpen,
    /// 括弧閉
    KakkoClose,
    /// 句点
    Kuten,
    /// 空白
    Space,
    /// 数
    Kazu,
    /// 読点
    Touten,
}

impl FromStr for Kigou {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::None),
            "アルファベット" => Ok(Self::Alphabet),
            "一般" => Ok(Self::General),
            "括弧開" => Ok(Self::KakkoOpen),
            "括弧閉" => Ok(Self::KakkoClose),
            "句点" => Ok(Self::Kuten),
            "空白" => Ok(Self::Space),
            "数" => Ok(Self::Kazu),
            "読点" => Ok(Self::Touten),
            _ => Err(POSParseError::new(1, s.to_string(), POSKind::Kigou)),
        }
    }
}

impl Display for Kigou {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},*,*",
            match &self {
                Self::None => "*",
                Self::Alphabet => "アルファベット",
                Self::General => "一般",
                Self::KakkoOpen => "括弧開",
                Self::KakkoClose => "括弧閉",
                Self::Kuten => "句点",
                Self::Space => "空白",
                Self::Kazu => "数",
                Self::Touten => "読点",
            },
        )
    }
}
