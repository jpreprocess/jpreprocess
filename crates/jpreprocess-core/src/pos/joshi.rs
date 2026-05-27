use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{POSKind, POSParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 助詞
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
    pub(crate) fn to_u8(self) -> u8 {
        match self {
            Self::KakuJoshi(KakuJoshi::General) => 0,
            Self::KakuJoshi(KakuJoshi::Quote) => 1,
            Self::KakuJoshi(KakuJoshi::Rengo) => 2,
            Self::KakariJoshi => 3,
            Self::ShuJoshi => 4,
            Self::SetsuzokuJoshi => 5,
            Self::Special => 6,
            Self::Fukushika => 7,
            Self::FukuJoshi => 8,
            Self::FukuHeiritsuShuJoshi => 9,
            Self::HeiritsuJoshi => 10,
            Self::Rentaika => 11,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::KakuJoshi(KakuJoshi::General),
            1 => Self::KakuJoshi(KakuJoshi::Quote),
            2 => Self::KakuJoshi(KakuJoshi::Rengo),
            3 => Self::KakariJoshi,
            4 => Self::ShuJoshi,
            5 => Self::SetsuzokuJoshi,
            6 => Self::Special,
            7 => Self::Fukushika,
            8 => Self::FukuJoshi,
            9 => Self::FukuHeiritsuShuJoshi,
            10 => Self::HeiritsuJoshi,
            11 => Self::Rentaika,
            _ => panic!("Invalid u8 value for Joshi: {}", n),
        }
    }
}

impl Joshi {
    pub fn from_strs(g1: &str, g2: &str) -> Result<Joshi, POSParseError> {
        match g1 {
            "格助詞" => KakuJoshi::from_str(g2).map(Self::KakuJoshi),
            "係助詞" => Ok(Self::KakariJoshi),
            "終助詞" => Ok(Self::ShuJoshi),
            "接続助詞" => Ok(Self::SetsuzokuJoshi),
            "特殊" => Ok(Self::Special),
            "副詞化" => Ok(Self::Fukushika),
            "副助詞" => Ok(Self::FukuJoshi),
            "副助詞／並立助詞／終助詞" => Ok(Self::FukuHeiritsuShuJoshi),
            "並立助詞" => Ok(Self::HeiritsuJoshi),
            "連体化" => Ok(Self::Rentaika),

            _ => Err(POSParseError::new(1, g1.to_string(), POSKind::Joshi)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 格助詞
pub enum KakuJoshi {
    /// 一般
    General,
    /// 引用
    Quote,
    /// 連語
    Rengo,
}

impl FromStr for KakuJoshi {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "一般" => Ok(Self::General),
            "引用" => Ok(Self::Quote),
            "連語" => Ok(Self::Rengo),

            _ => Err(POSParseError::new(2, s.to_string(), POSKind::KakuJoshi)),
        }
    }
}

impl Display for Joshi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::KakuJoshi(KakuJoshi::General) => "格助詞,一般,*",
            Self::KakuJoshi(KakuJoshi::Quote) => "格助詞,引用,*",
            Self::KakuJoshi(KakuJoshi::Rengo) => "格助詞,連語,*",
            Self::KakariJoshi => "係助詞,*,*",
            Self::ShuJoshi => "終助詞,*,*",
            Self::SetsuzokuJoshi => "接続助詞,*,*",
            Self::Special => "特殊,*,*",
            Self::Fukushika => "副詞化,*,*",
            Self::FukuJoshi => "副助詞,*,*",
            Self::FukuHeiritsuShuJoshi => "副助詞／並立助詞／終助詞,*,*",
            Self::HeiritsuJoshi => "並立助詞,*,*",
            Self::Rentaika => "連体化,*,*",
        })
    }
}
