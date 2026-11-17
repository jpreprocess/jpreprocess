use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// カ変
pub enum KaIrregular {
    /// クル
    Katakana,
    /// 来ル
    Kanji,
}
