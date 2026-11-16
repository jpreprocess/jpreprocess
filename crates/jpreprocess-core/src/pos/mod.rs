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
