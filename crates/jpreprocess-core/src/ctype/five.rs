use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{CTypeKind, CTypeParseError};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 五段
pub enum Five {
    /// カ行イ音便
    KaI,
    /// カ行促音便
    KaDouble,
    /// カ行促音便ユク
    KaDoubleYuku,
    /// ガ行
    Ga,
    /// サ行
    Sa,
    /// タ行
    Ta,
    /// ナ行
    Na,
    /// バ行
    Ba,
    /// マ行
    Ma,
    /// ラ行
    Ra,
    /// ラ行アル
    RaAru,
    /// ラ行特殊
    RaSpecial,
    /// ワ行ウ音便
    WaU,
    /// ワ行促音便
    WaDouble,
}

impl Five {
    pub(crate) fn to_u8(&self) -> u8 {
        match self {
            Self::KaI => 0,
            Self::KaDouble => 1,
            Self::KaDoubleYuku => 2,
            Self::Ga => 3,
            Self::Sa => 4,
            Self::Ta => 5,
            Self::Na => 6,
            Self::Ba => 7,
            Self::Ma => 8,
            Self::Ra => 9,
            Self::RaAru => 10,
            Self::RaSpecial => 11,
            Self::WaU => 12,
            Self::WaDouble => 13,
        }
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::KaI,
            1 => Self::KaDouble,
            2 => Self::KaDoubleYuku,
            3 => Self::Ga,
            4 => Self::Sa,
            5 => Self::Ta,
            6 => Self::Na,
            7 => Self::Ba,
            8 => Self::Ma,
            9 => Self::Ra,
            10 => Self::RaAru,
            11 => Self::RaSpecial,
            12 => Self::WaU,
            13 => Self::WaDouble,
            _ => panic!("Invalid u8 value for Five: {}", n),
        }
    }
}

impl FromStr for Five {
    type Err = CTypeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "カ行イ音便" => Ok(Self::KaI),
            "カ行促音便" => Ok(Self::KaDouble),
            "カ行促音便ユク" => Ok(Self::KaDoubleYuku),
            "ガ行" => Ok(Self::Ga),
            "サ行" => Ok(Self::Sa),
            "タ行" => Ok(Self::Ta),
            "ナ行" => Ok(Self::Na),
            "バ行" => Ok(Self::Ba),
            "マ行" => Ok(Self::Ma),
            "ラ行" => Ok(Self::Ra),
            "ラ行アル" => Ok(Self::RaAru),
            "ラ行特殊" => Ok(Self::RaSpecial),
            "ワ行ウ音便" => Ok(Self::WaU),
            "ワ行促音便" => Ok(Self::WaDouble),

            "カ往促音便" => {
                eprintln!("WARN: Unrecognized CType {}. Processed as カ行促音便.", s);
                Ok(Self::KaDouble)
            }

            _ => Err(CTypeParseError::new(s.to_string(), CTypeKind::Five)),
        }
    }
}

impl Display for Five {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::KaI => "カ行イ音便",
            Self::KaDouble => "カ行促音便",
            Self::KaDoubleYuku => "カ行促音便ユク",
            Self::Ga => "ガ行",
            Self::Sa => "サ行",
            Self::Ta => "タ行",
            Self::Na => "ナ行",
            Self::Ba => "バ行",
            Self::Ma => "マ行",
            Self::Ra => "ラ行",
            Self::RaAru => "ラ行アル",
            Self::RaSpecial => "ラ行特殊",
            Self::WaU => "ワ行ウ音便",
            Self::WaDouble => "ワ行促音便",
        })
    }
}
