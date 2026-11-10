
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 記号
pub enum Kigou {
    /// \*
    None,
    /// アルファベット
    Alphabet,
    /// 一般
    General,
    /// 括弧開
    KakkoOpen,
    /// 括弧閉
    KakkoClose,
    /// 句点
    Kuten,
    /// 空白
    Space,
    /// 数
    Kazu,
    /// 読点
    Touten,
}
