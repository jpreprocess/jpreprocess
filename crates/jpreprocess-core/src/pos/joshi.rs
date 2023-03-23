use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{error::JPreprocessErrorKind, JPreprocessError, JPreprocessResult};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Joshi {
    /// 格助詞
    KakuJoshi(KakuJoshi),
    /// 係助詞
    KakariJoshi,
    /// 終助詞
    ShuJoshi,
    /// 接続助詞
    SetsuzokuJoshi,
    /// 特殊
    Special,
    /// 副詞化
    Fukushika,
    /// 副助詞
    FukuJoshi,
    /// 副助詞/並立助詞/終助詞
    FukuHeiritsuShuJoshi,
    /// 並立助詞
    HeiritsuJoshi,
    /// 連体化
    Rentaika,
}

impl Joshi {
    pub fn from_strs(g1: &str, g2: &str) -> JPreprocessResult<Joshi> {
        match g1 {
            "格助詞" => KakuJoshi::from_str(g2).map(|kakujoshi| Self::KakuJoshi(kakujoshi)),
            "係助詞" => Ok(Self::KakariJoshi),
            "終助詞" => Ok(Self::ShuJoshi),
            "接続助詞" => Ok(Self::SetsuzokuJoshi),
            "特殊" => Ok(Self::Special),
            "副詞化" => Ok(Self::Fukushika),
            "副助詞" => Ok(Self::FukuJoshi),
            "副助詞／並立助詞／終助詞" => Ok(Self::FukuHeiritsuShuJoshi),
            "並立助詞" => Ok(Self::HeiritsuJoshi),
            "連体化" => Ok(Self::Rentaika),

            _ => Err(JPreprocessErrorKind::PartOfSpeechParseError
                .with_error(anyhow::anyhow!("Parse failed in Joshi"))),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum KakuJoshi {
    /// 一般
    General,
    /// 引用
    Quote,
    /// 連語
    Rengo,
}

impl FromStr for KakuJoshi {
    type Err = JPreprocessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "一般" => Ok(Self::General),
            "引用" => Ok(Self::Quote),
            "連語" => Ok(Self::Rengo),

            _ => Err(JPreprocessErrorKind::PartOfSpeechParseError
                .with_error(anyhow::anyhow!("Parse failed in KakuJoshi"))),
        }
    }
}
