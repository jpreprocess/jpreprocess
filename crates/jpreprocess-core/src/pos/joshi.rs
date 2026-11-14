use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 助詞
pub enum Joshi {
    /// 格助詞
    KakuJoshi(KakuJoshi),
    /// 係助詞
    KakariJoshi,
    /// 終助詞
    ShuJoshi,
    /// 接続助詞
    SetsuzokuJoshi,
    /// 特殊
    Special,
    /// 副詞化
    Fukushika,
    /// 副助詞
    FukuJoshi,
    /// 副助詞/並立助詞/終助詞
    FukuHeiritsuShuJoshi,
    /// 並立助詞
    HeiritsuJoshi,
    /// 連体化
    Rentaika,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 格助詞
pub enum KakuJoshi {
    /// 一般
    General,
    /// 引用
    Quote,
    /// 連語
    Rengo,
}
