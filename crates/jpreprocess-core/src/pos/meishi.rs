use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{POSKind, POSParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 名詞
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

    /// \*
    None,
}

impl Meishi {
    pub(crate) fn to_u8(&self) -> u8 {
        match self {
            Self::SahenSetsuzoku => 0,
            Self::NaiKeiyoushiGokan => 1,
            Self::General => 2,
            Self::QuoteStr => 3,
            Self::KeiyoudoushiGokan => 4,
            Self::KoyuMeishi(koyu_meishi) => 5 + koyu_meishi.to_u8(),
            Self::Kazu => 11,
            Self::Setsuzokushiteki => 12,
            Self::Setsubi(setsubi) => 13 + setsubi.to_u8(),
            Self::Daimeishi(daimeishi) => 22 + daimeishi.to_u8(),
            Self::DoushiHijiritsuteki => 24,
            Self::Special => 25,
            Self::Hijiritsu(meishi_hijiritsu) => 26 + meishi_hijiritsu.to_u8(),
            Self::FukushiKanou => 31,

            Self::None => 32,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::SahenSetsuzoku,
            1 => Self::NaiKeiyoushiGokan,
            2 => Self::General,
            3 => Self::QuoteStr,
            4 => Self::KeiyoudoushiGokan,
            5..=10 => Self::KoyuMeishi(KoyuMeishi::from_u8(n - 5)),
            11 => Self::Kazu,
            12 => Self::Setsuzokushiteki,
            13..=21 => Self::Setsubi(Setsubi::from_u8(n - 13)),
            22..=23 => Self::Daimeishi(Daimeishi::from_u8(n - 22)),
            24 => Self::DoushiHijiritsuteki,
            25 => Self::Special,
            26..=30 => Self::Hijiritsu(MeishiHijiritsu::from_u8(n - 26)),
            31 => Self::FukushiKanou,
            32 => Self::None,

            _ => panic!("Invalid u8 value for Meishi: {}", n),
        }
    }
}

impl Meishi {
    pub fn from_strs(g1: &str, g2: &str, g3: &str) -> Result<Self, POSParseError> {
        match g1 {
            "サ変接続" => Ok(Self::SahenSetsuzoku),
            "ナイ形容詞語幹" => Ok(Self::NaiKeiyoushiGokan),
            "一般" => Ok(Self::General),
            "引用文字列" => Ok(Self::QuoteStr),
            "形容動詞語幹" => Ok(Self::KeiyoudoushiGokan),
            "固有名詞" => KoyuMeishi::from_strs(g2, g3).map(Self::KoyuMeishi),
            "数" => Ok(Self::Kazu),
            "接続詞的" => Ok(Self::Setsuzokushiteki),
            "接尾" => Setsubi::from_str(g2).map(Self::Setsubi),
            "代名詞" => Daimeishi::from_str(g2).map(Self::Daimeishi),
            "動詞非自立的" => Ok(Self::DoushiHijiritsuteki),
            "特殊" => Ok(Self::Special),
            "非自立" => MeishiHijiritsu::from_str(g2).map(Self::Hijiritsu),
            "副詞可能" => Ok(Self::FukushiKanou),
            "*" => Ok(Self::None),

            _ => Err(POSParseError::new(1, g1.to_string(), POSKind::Meishi)),
        }
    }
}

impl Display for Meishi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::SahenSetsuzoku => "サ変接続",
            Self::NaiKeiyoushiGokan => "ナイ形容詞語幹",
            Self::General => "一般",
            Self::QuoteStr => "引用文字列",
            Self::KeiyoudoushiGokan => "形容動詞語幹",
            Self::KoyuMeishi(_) => "固有名詞",
            Self::Kazu => "数",
            Self::Setsuzokushiteki => "接続詞的",
            Self::Setsubi(_) => "接尾",
            Self::Daimeishi(_) => "代名詞",
            Self::DoushiHijiritsuteki => "動詞非自立的",
            Self::Special => "特殊",
            Self::Hijiritsu(_) => "非自立",
            Self::FukushiKanou => "副詞可能",

            Self::None => "*",
        })?;

        match &self {
            Self::KoyuMeishi(koyumeishi) => write!(f, ",{}", koyumeishi),
            Self::Setsubi(setsubi) => write!(f, ",{}", setsubi),
            Self::Daimeishi(daimeishi) => write!(f, ",{}", daimeishi),
            Self::Hijiritsu(meishi_hijiritsu) => write!(f, ",{}", meishi_hijiritsu),

            _ => f.write_str(",*,*"),
        }?;

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 固有名詞
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
    pub(crate) fn to_u8(&self) -> u8 {
        match self {
            Self::General => 0,
            Self::Person(Person::General) => 1,
            Self::Person(Person::Sei) => 2,
            Self::Person(Person::Mei) => 3,
            Self::Organization => 4,
            Self::Region(Region::General) => 5,
            Self::Region(Region::Country) => 6,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::General,
            1 => Self::Person(Person::General),
            2 => Self::Person(Person::Sei),
            3 => Self::Person(Person::Mei),
            4 => Self::Organization,
            5 => Self::Region(Region::General),
            6 => Self::Region(Region::Country),

            _ => panic!("Invalid u8 value for KoyuMeishi: {}", n),
        }
    }
}

impl KoyuMeishi {
    pub fn from_strs(g2: &str, g3: &str) -> Result<Self, POSParseError> {
        match g2 {
            "一般" => Ok(Self::General),
            "人名" => Person::from_str(g3).map(Self::Person),
            "組織" => Ok(Self::Organization),
            "地域" => Region::from_str(g3).map(Self::Region),

            _ => Err(POSParseError::new(2, g2.to_string(), POSKind::KoyuMeishi)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 人名
pub enum Person {
    /// 一般
    General,
    /// 姓
    Sei,
    /// 名
    Mei,
}

impl Person {
    pub(crate) fn to_u8(&self) -> u8 {
        match self {
            Self::General => 0,
            Self::Sei => 1,
            Self::Mei => 2,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::General,
            1 => Self::Sei,
            2 => Self::Mei,
            _ => panic!("Invalid u8 value for Person: {}", n),
        }
    }
}

impl FromStr for Person {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "一般" => Ok(Self::General),
            "姓" => Ok(Self::Sei),
            "名" => Ok(Self::Mei),

            _ => Err(POSParseError::new(3, s.to_string(), POSKind::Person)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 地域
pub enum Region {
    /// 一般
    General,
    /// 国
    Country,
}

impl Region {
    pub(crate) fn to_u8(&self) -> u8 {
        match self {
            Self::General => 0,
            Self::Country => 1,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::General,
            1 => Self::Country,
            _ => panic!("Invalid u8 value for Region: {}", n),
        }
    }
}

impl FromStr for Region {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "一般" => Ok(Self::General),
            "国" => Ok(Self::Country),

            _ => Err(POSParseError::new(3, s.to_string(), POSKind::Region)),
        }
    }
}

impl Display for KoyuMeishi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::General => "一般,*",
            Self::Person(Person::General) => "人名,一般",
            Self::Person(Person::Sei) => "人名,姓",
            Self::Person(Person::Mei) => "人名,名",
            Self::Organization => "組織,*",
            Self::Region(Region::General) => "地域,一般",
            Self::Region(Region::Country) => "地域,国",
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 名詞・接尾
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

impl Setsubi {
    pub(crate) fn to_u8(&self) -> u8 {
        match self {
            Self::SahenSetsuzoku => 0,
            Self::General => 1,
            Self::KeiyoudoushiGokan => 2,
            Self::Josuushi => 3,
            Self::JodoushiGokan => 4,
            Self::Person => 5,
            Self::Region => 6,
            Self::Special => 7,
            Self::FukushiKanou => 8,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::SahenSetsuzoku,
            1 => Self::General,
            2 => Self::KeiyoudoushiGokan,
            3 => Self::Josuushi,
            4 => Self::JodoushiGokan,
            5 => Self::Person,
            6 => Self::Region,
            7 => Self::Special,
            8 => Self::FukushiKanou,
            _ => panic!("Invalid u8 value for Setsubi: {}", n),
        }
    }
}

impl FromStr for Setsubi {
    type Err = POSParseError;
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

            _ => Err(POSParseError::new(2, s.to_string(), POSKind::MeishiSetsubi)),
        }
    }
}

impl Display for Setsubi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},*",
            match &self {
                Self::SahenSetsuzoku => "サ変接続",
                Self::General => "一般",
                Self::KeiyoudoushiGokan => "形容動詞語幹",
                Self::Josuushi => "助数詞",
                Self::JodoushiGokan => "助動詞語幹",
                Self::Person => "人名",
                Self::Region => "地域",
                Self::Special => "特殊",
                Self::FukushiKanou => "副詞可能",
            },
        )
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 代名詞
pub enum Daimeishi {
    /// 一般
    General,
    /// 縮約
    Contraction,
}

impl Daimeishi {
    pub(crate) fn to_u8(&self) -> u8 {
        match self {
            Self::General => 0,
            Self::Contraction => 1,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::General,
            1 => Self::Contraction,
            _ => panic!("Invalid u8 value for Daimeishi: {}", n),
        }
    }
}

impl FromStr for Daimeishi {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "一般" => Ok(Self::General),
            "縮約" => Ok(Self::Contraction),

            _ => Err(POSParseError::new(2, s.to_string(), POSKind::Daimeishi)),
        }
    }
}

impl Display for Daimeishi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},*",
            match &self {
                Self::General => "一般",
                Self::Contraction => "縮約",
            },
        )
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 名詞・非自立
pub enum MeishiHijiritsu {
    /// 一般
    General,
    /// 形容動詞語幹
    KeiyoudoushiGokan,
    /// 助動詞語幹
    JodoushiGokan,
    /// 副詞可能
    FukushiKanou,
    /// \*
    None,
}

impl MeishiHijiritsu {
    pub(crate) fn to_u8(&self) -> u8 {
        match self {
            Self::General => 0,
            Self::KeiyoudoushiGokan => 1,
            Self::JodoushiGokan => 2,
            Self::FukushiKanou => 3,
            Self::None => 4,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::General,
            1 => Self::KeiyoudoushiGokan,
            2 => Self::JodoushiGokan,
            3 => Self::FukushiKanou,
            4 => Self::None,
            _ => panic!("Invalid u8 value for MeishiHijiritsu: {}", n),
        }
    }
}

impl FromStr for MeishiHijiritsu {
    type Err = POSParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "一般" => Ok(Self::General),
            "形容動詞語幹" => Ok(Self::KeiyoudoushiGokan),
            "助動詞語幹" => Ok(Self::JodoushiGokan),
            "副詞可能" => Ok(Self::FukushiKanou),
            "*" => Ok(Self::None),

            _ => Err(POSParseError::new(
                2,
                s.to_string(),
                POSKind::MeishiHijiritsu,
            )),
        }
    }
}

impl Display for MeishiHijiritsu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},*",
            match &self {
                Self::General => "一般",
                Self::KeiyoudoushiGokan => "形容動詞語幹",
                Self::JodoushiGokan => "助動詞語幹",
                Self::FukushiKanou => "副詞可能",
                Self::None => "*",
            },
        )
    }
}
