use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 下二
pub enum LowerTwo {
    /// ア行
    A,
    /// カ行
    Ka,
    /// ガ行
    Ga,
    /// サ行
    Sa,
    /// ザ行
    Za,
    /// タ行
    Ta,
    /// ダ行
    Da,
    /// ナ行
    Na,
    /// ハ行
    Ha,
    /// バ行
    Ba,
    /// マ行
    Ma,
    /// ヤ行
    Ya,
    /// ラ行
    Ra,
    /// ワ行
    Wa,
    /// 得
    Get,
}
