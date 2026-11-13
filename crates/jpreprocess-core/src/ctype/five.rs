use serde::{Deserialize, Serialize};

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
