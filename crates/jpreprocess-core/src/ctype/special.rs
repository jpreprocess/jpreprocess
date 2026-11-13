use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 特殊
pub enum Special {
    /// ナイ
    Nai,
    /// タイ
    Tai,
    /// タ
    Ta,
    /// ダ
    Da,
    /// デス
    Desu,
    /// ドス
    Dosu,
    /// ジャ
    Ja,
    /// マス
    Masu,
    /// ヌ
    Nu,
    /// ヤ
    Ya,
}
