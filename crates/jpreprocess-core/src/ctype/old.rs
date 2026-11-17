use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 文語
pub enum Old {
    /// ベシ
    Beshi,
    /// ゴトシ
    Gotoshi,
    /// ナリ
    Nari,
    /// マジ
    Maji,
    /// シム
    Shimu,
    /// キ
    Ki,
    /// ケリ
    Keri,
    /// ル
    Ru,
    /// リ
    Ri,
}
