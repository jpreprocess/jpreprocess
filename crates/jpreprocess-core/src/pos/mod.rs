use serde::{Deserialize, Serialize};
use std::fmt::Display;

mod doushi;
mod fukushi;
mod joshi;
mod keiyoushi;
mod kigou;
mod meishi;
mod settoushi;

mod conversions;

pub use self::{doushi::*, fukushi::*, joshi::*, keiyoushi::*, kigou::*, meishi::*, settoushi::*};

#[derive(Debug, PartialEq, Eq)]
pub enum POSKind {
    POSMajor,
    Kigou,
    Keiyoushi,
    Joshi,
    Settoushi,
    Doushi,
    Fukushi,
    Meishi,
    KakuJoshi,
    KoyuMeishi,
    Person,
    Region,
    MeishiSetsubi,
    Daimeishi,
    MeishiHijiritsu,
}
impl Display for POSKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::POSMajor => "品詞",
            Self::Kigou => "記号",
            Self::Keiyoushi => "形容詞",
            Self::Joshi => "助詞",
            Self::Settoushi => "接頭詞",
            Self::Doushi => "動詞",
            Self::Fukushi => "副詞",
            Self::Meishi => "名詞",
            Self::KakuJoshi => "格助詞",
            Self::KoyuMeishi => "固有名詞",
            Self::Person => "人名（固有名詞）",
            Self::Region => "地域（固有名詞）",
            Self::MeishiSetsubi => "接尾（名詞）",
            Self::Daimeishi => "代名詞",
            Self::MeishiHijiritsu => "非自立（名詞）",
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize, Default)]
/// 品詞
pub enum POS {
    /// フィラー
    Filler,
    /// 感動詞
    Kandoushi,
    /// 記号
    Kigou(Kigou),
    /// 形容詞
    Keiyoushi(Keiyoushi),
    /// 助詞
    Joshi(Joshi),
    /// 助動詞
    Jodoushi,
    /// 接続詞
    Setsuzokushi,
    /// 接頭詞
    Settoushi(Settoushi),
    /// 動詞
    Doushi(Doushi),
    /// 副詞
    Fukushi(Fukushi),
    /// 名詞
    Meishi(Meishi),
    /// 連体詞
    Rentaishi,

    /// その他
    Others,

    /// 不明
    #[default]
    Unknown,
}

impl POS {
    pub fn from_strs(g0: &str, g1: &str, g2: &str, g3: &str) -> Self {
        conversions::string_to_pos([g0, g1, g2, g3])
    }

    pub fn is_kazu(&self) -> bool {
        matches!(self, Self::Kigou(Kigou::Kazu) | Self::Meishi(Meishi::Kazu))
    }

    pub fn convert_to_kigou(&mut self) {
        *self = match self {
            Self::Kigou(kigou) => Self::Kigou(*kigou),
            Self::Meishi(Meishi::Kazu) => Self::Kigou(Kigou::Kazu),
            Self::Fukushi(Fukushi::General) | Self::Meishi(Meishi::General) => {
                Self::Kigou(Kigou::General)
            }
            _ => Self::Kigou(Kigou::None),
        }
    }

    pub(crate) fn to_u8(self) -> u8 {
        conversions::pos_to_value(&self)
    }

    pub(crate) fn from_u8(n: u8) -> Self {
        conversions::value_to_pos(n).unwrap_or(Self::Unknown)
    }
}

impl Display for POS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&conversions::pos_to_string(self))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filler() {
        let pos = POS::from_strs("フィラー", "*", "*", "*");
        assert!(matches!(pos, POS::Filler));
        assert_eq!(pos.to_string(), "フィラー,*,*,*")
    }

    #[test]
    fn joshi() {
        let pos = POS::from_strs("助詞", "副助詞／並立助詞／終助詞", "*", "*");
        assert!(matches!(pos, POS::Joshi(Joshi::FukuHeiritsuShuJoshi)));
        assert_eq!(pos.to_string(), "助詞,副助詞／並立助詞／終助詞,*,*")
    }

    #[test]
    fn meishi() {
        let pos = POS::from_strs("名詞", "*", "*", "*");
        assert!(matches!(pos, POS::Meishi(Meishi::None)));
        assert_eq!(pos.to_string(), "名詞,*,*,*")
    }

    #[test]
    fn koyumeishi() {
        let pos = POS::from_strs("名詞", "固有名詞", "人名", "姓");
        assert!(matches!(
            pos,
            POS::Meishi(Meishi::KoyuMeishi(KoyuMeishi::Person(Person::Sei)))
        ));
        assert_eq!(pos.to_string(), "名詞,固有名詞,人名,姓")
    }
}
