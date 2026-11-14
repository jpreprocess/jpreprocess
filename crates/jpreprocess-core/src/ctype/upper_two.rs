use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 上二
pub enum UpperTwo {
    /// ダ行
    Da,
    /// ハ行
    Ha,
}
