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
}

macro_rules! cform_conversions {
    (@map $( $cform:expr => $value:literal, $string:literal ),* $(,)?) => {
        impl FromStr for CForm {
            type Err = CFormParseError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                const STRING_CFORM_CONVERSION_TABLE: phf::Map<&'static str, CForm> = phf::phf_map! {
                    $($string => $cform,)*
                };
                if let Some(cform) = STRING_CFORM_CONVERSION_TABLE.get(s) {
                    return Ok(*cform);
                }
                Err(CFormParseError::InvalidString(s.to_string()))
            }
        }
        impl CForm {
            pub(crate) fn from_u8(n: u8) -> Result<Self, CFormParseError> {
                const VALUE_CFORM_CONVERSION_TABLE: [CForm; 75] = {
                    let mut table = [CForm::None; 75];
                    $(
                        table[$value as usize] = $cform;
                    )*
                    table
                };
                if n as usize >= VALUE_CFORM_CONVERSION_TABLE.len() {
                    return Err(CFormParseError::InvalidValue(n));
                }
                Ok(VALUE_CFORM_CONVERSION_TABLE[n as usize])
            }
        }
    };
    (@match $( $cform:pat => $value:literal, $string:literal ),* $(,)?) => {
        impl CForm {
            pub(crate) fn to_u8(self) -> u8 {
                match self {
                    $($cform => $value,)*
                }
            }
        }
        impl Display for CForm {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(match self {
                    $($cform => $string,)*
                })
            }
        }
    };

    ($($tokens:tt)*) => {
        cform_conversions!(@map $($tokens)*);
        cform_conversions!(@match $($tokens)*);
    };
}

cform_conversions! {
    CForm::None => 0, "*",

    CForm::ConjunctionGaru => 1, "ガル接続",
    CForm::BasicEuphony => 2, "音便基本形",
    CForm::Conditional => 3, "仮定形",
    CForm::ConditionalContraction1 => 4, "仮定縮約１",
    CForm::ConditionalContraction2 => 5, "仮定縮約２",
    CForm::Basic => 6, "基本形",
    CForm::BasicDoubledConsonant => 7, "基本形-促音便",
    CForm::BasicModern => 8, "現代基本形",
    CForm::TaigenConjunction => 9, "体言接続",
    CForm::TaigenConjunctionSpecial => 10, "体言接続特殊",
    CForm::TaigenConjunctionSpecial2 => 11, "体言接続特殊２",
    CForm::BasicOld => 12, "文語基本形",
    CForm::MizenConjunctionU => 13, "未然ウ接続",
    CForm::MizenConjunctionNu => 14, "未然ヌ接続",
    CForm::MizenConjunctionReru => 15, "未然レル接続",
    CForm::Mizen => 16, "未然形",
    CForm::MizenSpecial => 17, "未然特殊",
    CForm::ImperativeE => 18, "命令ｅ",
    CForm::ImperativeI => 19, "命令ｉ",
    CForm::ImperativeRo => 20, "命令ｒｏ",
    CForm::ImperativeYo => 21, "命令ｙｏ",
    CForm::RenyouConjunctionGozai => 22, "連用ゴザイ接続",
    CForm::RenyouConjunctionTa => 23, "連用タ接続",
    CForm::RenyouConjunctionTe => 24, "連用テ接続",
    CForm::RenyouConjunctionDe => 25, "連用デ接続",
    CForm::RenyouConjunctionNi => 26, "連用ニ接続",
    CForm::Renyou => 27, "連用形",
}
