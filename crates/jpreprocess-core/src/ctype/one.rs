use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 一段
pub enum One {
    /// 病メル
    Yameru,
    /// クレル
    Kureru,
    /// 得ル
    Eru,
    /// ル
    Ru,
    /// (Empty)
    None,
}
