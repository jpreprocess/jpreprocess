use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{error::JPreprocessErrorKind, JPreprocessError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Kigou {
    /// *
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
    type Err = JPreprocessError;
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
            _ => Err(JPreprocessErrorKind::PartOfSpeechParseError
                .with_error(anyhow::anyhow!("Parse failed in Kigou"))),
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

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Keiyoushi {
    /// 自立
    Jiritsu,
    /// 接尾
    Setsubi,
    /// 非自立
    Hijiritsu,
}

impl FromStr for Keiyoushi {
    type Err = JPreprocessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "自立" => Ok(Self::Jiritsu),
            "接尾" => Ok(Self::Setsubi),
            "非自立" => Ok(Self::Hijiritsu),
            _ => Err(JPreprocessErrorKind::PartOfSpeechParseError
                .with_error(anyhow::anyhow!("Parse failed in Keiyoushi"))),
        }
    }
}

impl Display for Keiyoushi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},*,*",
            match &self {
                Self::Jiritsu => "自立",
                Self::Setsubi => "接尾",
                Self::Hijiritsu => "非自立",
            },
        )
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Doushi {
    /// 自立
    Jiritsu,
    /// 接尾
    Setsubi,
    /// 非自立
    Hijiritsu,
}

impl FromStr for Doushi {
    type Err = JPreprocessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "自立" => Ok(Self::Jiritsu),
            "接尾" => Ok(Self::Setsubi),
            "非自立" => Ok(Self::Hijiritsu),
            _ => Err(JPreprocessErrorKind::PartOfSpeechParseError
                .with_error(anyhow::anyhow!("Parse failed in Doushi"))),
        }
    }
}

impl Display for Doushi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},*,*",
            match &self {
                Self::Jiritsu => "自立",
                Self::Setsubi => "接尾",
                Self::Hijiritsu => "非自立",
            },
        )
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Fukushi {
    /// *
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
