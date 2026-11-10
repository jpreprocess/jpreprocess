use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 動詞
pub enum Doushi {
    /// 自立
    Jiritsu,
    /// 接尾
    Setsubi,
    /// 非自立
    Hijiritsu,
}
