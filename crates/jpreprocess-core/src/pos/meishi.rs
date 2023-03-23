use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{error::JPreprocessErrorKind, JPreprocessError, JPreprocessResult};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Meishi {
    /// サ変接続
    SahenSetsuzoku,
    /// ナイ形容詞語幹
    NaiKeiyoushiGokan,
    /// 一般
    General,
    /// 引用文字列
    QuoteStr,
    /// 形容動詞語幹
    KeiyoudoushiGokan,
    /// 固有名詞
    KoyuMeishi(KoyuMeishi),
    /// 数
    Kazu,
    /// 接続詞的
    Setsuzokushiteki,
    /// 接尾
    Setsubi(Setsubi),
    /// 代名詞
    Daimeishi(Daimeishi),
    /// 動詞非自立的
    DoushiHijiritsuteki,
    /// 特殊
    Special,
    /// 非自立
    Hijiritsu(MeishiHijiritsu),
    /// 副詞可能
    FukushiKanou,

    /// *
    None,
}

impl Meishi {
    pub fn from_strs(g1: &str, g2: &str, g3: &str) -> JPreprocessResult<Self> {
        match g1 {
            "サ変接続" => Ok(Self::SahenSetsuzoku),
            "ナイ形容詞語幹" => Ok(Self::NaiKeiyoushiGokan),
            "一般" => Ok(Self::General),
            "引用文字列" => Ok(Self::QuoteStr),
            "形容動詞語幹" => Ok(Self::KeiyoudoushiGokan),
            "固有名詞" => {
                KoyuMeishi::from_strs(g2, g3).map(|koyumeishi| Self::KoyuMeishi(koyumeishi))
            }
            "数" => Ok(Self::Kazu),
            "接続詞的" => Ok(Self::Setsuzokushiteki),
            "接尾" => Setsubi::from_str(g2).map(|setsubi| Self::Setsubi(setsubi)),
            "代名詞" => Daimeishi::from_str(g2).map(|daimeishi| Self::Daimeishi(daimeishi)),
            "動詞非自立的" => Ok(Self::DoushiHijiritsuteki),
            "特殊" => Ok(Self::Special),
            "非自立" => {
                MeishiHijiritsu::from_str(g2).map(|hijiritsu| Self::Hijiritsu(hijiritsu))
            }
            "副詞可能" => Ok(Self::FukushiKanou),
            "*" => Ok(Self::None),

            _ => Err(JPreprocessErrorKind::PartOfSpeechParseError
                .with_error(anyhow::anyhow!("Parse failed in Meishi"))),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum KoyuMeishi {
    /// 一般
    General,
    /// 人名
    Person(Person),
    /// 組織
    Organization,
    /// 地域
    Region(Region),
}

impl KoyuMeishi {
    pub fn from_strs(g2: &str, g3: &str) -> JPreprocessResult<Self> {
        match g2 {
            "一般" => Ok(Self::General),
            "人名" => Person::from_str(g3).map(|person| Self::Person(person)),
            "組織" => Ok(Self::Organization),
            "地域" => Region::from_str(g3).map(|region| Self::Region(region)),

            _ => Err(JPreprocessErrorKind::PartOfSpeechParseError
                .with_error(anyhow::anyhow!("Parse failed in KoyuMeishi"))),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Person {
    /// 一般
    General,
    /// 姓
    Sei,
    /// 名
    Mei,
}

impl FromStr for Person {
    type Err = JPreprocessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "一般" => Ok(Self::General),
            "姓" => Ok(Self::Sei),
            "名" => Ok(Self::Mei),

            _ => Err(JPreprocessErrorKind::PartOfSpeechParseError
                .with_error(anyhow::anyhow!("Parse failed in Person"))),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Region {
    /// 一般
    General,
    /// 国
    Country,
}

impl FromStr for Region {
    type Err = JPreprocessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "一般" => Ok(Self::General),
            "国" => Ok(Self::Country),

            _ => Err(JPreprocessErrorKind::PartOfSpeechParseError
                .with_error(anyhow::anyhow!("Parse failed in Region"))),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Setsubi {
    /// サ変接続
    SahenSetsuzoku,
    /// 一般
    General,
    /// 形容動詞語幹
    KeiyoudoushiGokan,
    /// 助数詞
    Josuushi,
    /// 助動詞語幹
    JodoushiGokan,
    /// 人名
    Person,
    /// 地域
    Region,
    /// 特殊
    Special,
    /// 副詞可能
    FukushiKanou,
}

impl FromStr for Setsubi {
    type Err = JPreprocessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "サ変接続" => Ok(Self::SahenSetsuzoku),
            "一般" => Ok(Self::General),
            "形容動詞語幹" => Ok(Self::KeiyoudoushiGokan),
            "助数詞" => Ok(Self::Josuushi),
            "助動詞語幹" => Ok(Self::JodoushiGokan),
            "人名" => Ok(Self::Person),
            "地域" => Ok(Self::Region),
            "特殊" => Ok(Self::Special),
            "副詞可能" => Ok(Self::FukushiKanou),

            _ => Err(JPreprocessErrorKind::PartOfSpeechParseError
                .with_error(anyhow::anyhow!("Parse failed in Setsubi"))),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Daimeishi {
    /// 一般
    General,
    /// 縮約
    Contraction,
}

impl FromStr for Daimeishi {
    type Err = JPreprocessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "一般" => Ok(Self::General),
            "縮約" => Ok(Self::Contraction),

            _ => Err(JPreprocessErrorKind::PartOfSpeechParseError
                .with_error(anyhow::anyhow!("Parse failed in Daimeishi"))),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum MeishiHijiritsu {
    /// 一般
    General,
    /// 形容動詞語幹
    KeiyoudoushiGokan,
    /// 助動詞語幹
    JodoushiGokan,
    /// 副詞可能
    FukushiKanou,
    /// *
    None,
}

impl FromStr for MeishiHijiritsu {
    type Err = JPreprocessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "一般" => Ok(Self::General),
            "形容動詞語幹" => Ok(Self::KeiyoudoushiGokan),
            "助動詞語幹" => Ok(Self::JodoushiGokan),
            "副詞可能" => Ok(Self::FukushiKanou),
            "*" => Ok(Self::None),

            _ => Err(JPreprocessErrorKind::PartOfSpeechParseError
                .with_error(anyhow::anyhow!("Parse failed in MeishiHijiritsu"))),
        }
    }
}
