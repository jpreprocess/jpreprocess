use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 形容詞
pub enum Keiyoushi {
    /// アウオ段
    Auo,
    /// イ段
    I,
    /// イイ
    Ii,
}
