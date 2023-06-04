use jpreprocess_core::ctype::CType;

pub fn ctype_to_id(ctype: &CType) -> Option<u8> {
    match ctype {
        // *:xx
        CType::None => None,
        // カ行変格:5
        CType::KaIrregular(_) => Some(5),
        // サ行変格:4
        CType::SaIrregular(_) => Some(4),
        // ラ行変格:6
        CType::RaIrregular => Some(6),
        // 一段:3
        CType::One(_) => Some(3),
        // 形容詞:7
        CType::Keiyoushi(_) => Some(7),
        // 五段:1
        CType::Five(_) => Some(1),
        // 四段:6
        CType::Four(_) => Some(6),
        // 助動詞:7
        CType::Special(_) => Some(7),
        // 二段:6
        CType::LowerTwo(_) => Some(6),
        CType::UpperTwo(_) => Some(6),
        // 不変化:6
        CType::NoConjugation => Some(6),
        // 文語助動詞:6
        CType::Old(_) => Some(6),
    }
}
