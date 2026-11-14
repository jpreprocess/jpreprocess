use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 接頭詞
pub enum Settoushi {
    /// 形容詞接続
    KeiyoushiSetsuzoku,
    /// 数接続
    SuuSetsuzoku,
    /// 動詞接続
    DoushiSetsuzoku,
    /// 名詞接続
    MeishiSetsuzoku,
}
