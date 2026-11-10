use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 副詞
pub enum Fukushi {
    /// \*
    None,
    /// 一般
    General,
    /// 助詞類接続
    JoshiruiSetsuzoku,
}

