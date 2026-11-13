use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum CFormParseError {
    #[error("Invalid CForm string: {0}")]
    InvalidString(String),
    #[error("Invalid CForm value: {0}")]
    InvalidValue(u8),
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize, Default)]
/// 活用形
pub enum CForm {
    /// ガル接続
    ConjunctionGaru,
    /// 音便基本形
    BasicEuphony,
    /// 仮定形
    Conditional,
    /// 仮定縮約１
    ConditionalContraction1,
    /// 仮定縮約２
    ConditionalContraction2,
    /// 基本形
    Basic,
    /// 基本形-促音便
    BasicDoubledConsonant,
    /// 現代基本形
    BasicModern,
    /// 体言接続
    TaigenConjunction,
    /// 体言接続特殊
    TaigenConjunctionSpecial,
    /// 体言接続特殊２
    TaigenConjunctionSpecial2,
    /// 文語基本形
    BasicOld,
    /// 未然ウ接続
    MizenConjunctionU,
    /// 未然ヌ接続
    MizenConjunctionNu,
    /// 未然レル接続
    MizenConjunctionReru,
    /// 未然形
    Mizen,
    /// 未然特殊
    MizenSpecial,
    /// 命令ｅ
    ImperativeE,
    /// 命令ｉ
    ImperativeI,
    /// 命令ｒｏ
    ImperativeRo,
    /// 命令ｙｏ
    ImperativeYo,
    /// 連用ゴザイ接続
    RenyouConjunctionGozai,
    /// 連用タ接続
    RenyouConjunctionTa,
    /// 連用テ接続
    RenyouConjunctionTe,
    /// 連用デ接続
    RenyouConjunctionDe,
    /// 連用ニ接続
    RenyouConjunctionNi,
    /// 連用形
    Renyou,

    /// \*
    #[default]
    None,
}

impl CForm {
    pub fn is_renyou(&self) -> bool {
        matches!(
            self,
            Self::RenyouConjunctionGozai
                | Self::RenyouConjunctionTa
                | Self::RenyouConjunctionTe
                | Self::RenyouConjunctionDe
                | Self::RenyouConjunctionNi
                | Self::Renyou
        )
    }

    pub(crate) fn to_u8(self) -> u8 {
        match self {
            Self::ConjunctionGaru => 0,
            Self::BasicEuphony => 1,
            Self::Conditional => 2,
            Self::ConditionalContraction1 => 3,
            Self::ConditionalContraction2 => 4,
            Self::Basic => 5,
            Self::BasicDoubledConsonant => 6,
            Self::BasicModern => 7,
            Self::TaigenConjunction => 8,
            Self::TaigenConjunctionSpecial => 9,
            Self::TaigenConjunctionSpecial2 => 10,
            Self::BasicOld => 11,
            Self::MizenConjunctionU => 12,
            Self::MizenConjunctionNu => 13,
            Self::MizenConjunctionReru => 14,
            Self::Mizen => 15,
            Self::MizenSpecial => 16,
            Self::ImperativeE => 17,
            Self::ImperativeI => 18,
            Self::ImperativeRo => 19,
            Self::ImperativeYo => 20,
            Self::RenyouConjunctionGozai => 21,
            Self::RenyouConjunctionTa => 22,
            Self::RenyouConjunctionTe => 23,
            Self::RenyouConjunctionDe => 24,
            Self::RenyouConjunctionNi => 25,
            Self::Renyou => 26,

            Self::None => 27,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Result<Self, CFormParseError> {
        match n {
            0 => Ok(Self::ConjunctionGaru),
            1 => Ok(Self::BasicEuphony),
            2 => Ok(Self::Conditional),
            3 => Ok(Self::ConditionalContraction1),
            4 => Ok(Self::ConditionalContraction2),
            5 => Ok(Self::Basic),
            6 => Ok(Self::BasicDoubledConsonant),
            7 => Ok(Self::BasicModern),
            8 => Ok(Self::TaigenConjunction),
            9 => Ok(Self::TaigenConjunctionSpecial),
            10 => Ok(Self::TaigenConjunctionSpecial2),
            11 => Ok(Self::BasicOld),
            12 => Ok(Self::MizenConjunctionU),
            13 => Ok(Self::MizenConjunctionNu),
            14 => Ok(Self::MizenConjunctionReru),
            15 => Ok(Self::Mizen),
            16 => Ok(Self::MizenSpecial),
            17 => Ok(Self::ImperativeE),
            18 => Ok(Self::ImperativeI),
            19 => Ok(Self::ImperativeRo),
            20 => Ok(Self::ImperativeYo),
            21 => Ok(Self::RenyouConjunctionGozai),
            22 => Ok(Self::RenyouConjunctionTa),
            23 => Ok(Self::RenyouConjunctionTe),
            24 => Ok(Self::RenyouConjunctionDe),
            25 => Ok(Self::RenyouConjunctionNi),
            26 => Ok(Self::Renyou),

            27 => Ok(Self::None),

            _ => Err(CFormParseError::InvalidValue(n)),
        }
    }
}

impl FromStr for CForm {
    type Err = CFormParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ガル接続" => Ok(Self::ConjunctionGaru),
            "音便基本形" => Ok(Self::BasicEuphony),
            "仮定形" => Ok(Self::Conditional),
            "仮定縮約１" => Ok(Self::ConditionalContraction1),
            "仮定縮約２" => Ok(Self::ConditionalContraction2),
            "基本形" => Ok(Self::Basic),
            "基本形-促音便" => Ok(Self::BasicDoubledConsonant),
            "現代基本形" => Ok(Self::BasicModern),
            "体言接続" => Ok(Self::TaigenConjunction),
            "体言接続特殊" => Ok(Self::TaigenConjunctionSpecial),
            "体言接続特殊２" => Ok(Self::TaigenConjunctionSpecial2),
            "文語基本形" => Ok(Self::BasicOld),
            "未然ウ接続" => Ok(Self::MizenConjunctionU),
            "未然ヌ接続" => Ok(Self::MizenConjunctionNu),
            "未然レル接続" => Ok(Self::MizenConjunctionReru),
            "未然形" => Ok(Self::Mizen),
            "未然特殊" => Ok(Self::MizenSpecial),
            "命令ｅ" => Ok(Self::ImperativeE),
            "命令ｉ" => Ok(Self::ImperativeI),
            "命令ｒｏ" => Ok(Self::ImperativeRo),
            "命令ｙｏ" => Ok(Self::ImperativeYo),
            "連用ゴザイ接続" => Ok(Self::RenyouConjunctionGozai),
            "連用タ接続" => Ok(Self::RenyouConjunctionTa),
            "連用テ接続" => Ok(Self::RenyouConjunctionTe),
            "連用デ接続" => Ok(Self::RenyouConjunctionDe),
            "連用ニ接続" => Ok(Self::RenyouConjunctionNi),
            "連用形" => Ok(Self::Renyou),

            "*" => Ok(Self::None),

            _ => Err(CFormParseError::InvalidString(s.to_string())),
        }
    }
}

impl Display for CForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::ConjunctionGaru => "ガル接続",
            Self::BasicEuphony => "音便基本形",
            Self::Conditional => "仮定形",
            Self::ConditionalContraction1 => "仮定縮約１",
            Self::ConditionalContraction2 => "仮定縮約２",
            Self::Basic => "基本形",
            Self::BasicDoubledConsonant => "基本形-促音便",
            Self::BasicModern => "現代基本形",
            Self::TaigenConjunction => "体言接続",
            Self::TaigenConjunctionSpecial => "体言接続特殊",
            Self::TaigenConjunctionSpecial2 => "体言接続特殊２",
            Self::BasicOld => "文語基本形",
            Self::MizenConjunctionU => "未然ウ接続",
            Self::MizenConjunctionNu => "未然ヌ接続",
            Self::MizenConjunctionReru => "未然レル接続",
            Self::Mizen => "未然形",
            Self::MizenSpecial => "未然特殊",
            Self::ImperativeE => "命令ｅ",
            Self::ImperativeI => "命令ｉ",
            Self::ImperativeRo => "命令ｒｏ",
            Self::ImperativeYo => "命令ｙｏ",
            Self::RenyouConjunctionGozai => "連用ゴザイ接続",
            Self::RenyouConjunctionTa => "連用タ接続",
            Self::RenyouConjunctionTe => "連用テ接続",
            Self::RenyouConjunctionDe => "連用デ接続",
            Self::RenyouConjunctionNi => "連用ニ接続",
            Self::Renyou => "連用形",

            Self::None => "*",
        })
    }
}
