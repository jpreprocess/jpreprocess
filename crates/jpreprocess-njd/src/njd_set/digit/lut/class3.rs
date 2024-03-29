use super::*;
use jpreprocess_core::pron;
use phf::phf_map;

type Class3Keys = Map<&'static str, &'static [&'static str]>;

pub const CONVERSION_TABLE: [(Class3Keys, DigitLUT); 1] = [(NUMERATIVE_CLASS3, CONV_TABLE3)];

const NUMERATIVE_CLASS3: Map<&str, &[&str]> = phf_map! {
    /* from paper */
    "棟" => &["ムネ"],
    /* from dictionary */
    "かけ" => &["カケ"],
    "くだり" => &["クダリ"],
    "けた" => &["ケタ"],
    "すじ" => &["スジ"],
    "そろい" => &["ソロイ"],
    "たび" => &["タビ"],
    "つかみ" => &["ツカミ"],
    "つがい" => &["ツガイ"],
    "つまみ" => &["ツマミ"],
    "とおり" => &["トオリ"],
    "ところ" => &["トコロ"],
    "とせ" => &["トセ"],
    "まわり" => &["マワリ"],
    "シーズン" => &["シーズン"],
    "セット" => &["セット"],
    "握り" => &["ニギリ"],
    "回り" => &["マワリ"],
    "株" => &["カブ"],
    "竿" => &["サオ"],
    "筋" => &["スジ"],
    "桁" => &["ケタ"],
    "ケタ" => &["ケタ"],
    "月" => &["ツキ"],
    "言" => &["コト"],
    "口" => &["クチ"],
    "差し" => &["サシ"],
    "皿" => &["サラ"],
    "山" => &["ヤマ"],
    "勺" => &["シャク"],
    "尺" => &["シャク"],
    "重ね" => &["カサネ", "ガサネ"],
    "振り" => &["フリ"],
    "針" => &["ハリ"],
    "切れ" => &["キレ"],
    "束" => &["タバ"],
    "続き" => &["ツヅキ"],
    "揃" => &["ソロイ"],
    "袋" => &["フクロ"],
    "柱" => &["ハシラ"],
    "張り" => &["ハリ"],
    "通り" => &["トオリ"],
    "掴み" => &["ツカミ"],
    "坪" => &["ツボ"],
    "箱" => &["ハコ"],
    "鉢" => &["ハチ"],
    "晩" => &["バン"],
    "品" => &["シナ"],
    "瓶" => &["ビン"],
    "分け" => &["ワケ"],
    "幕" => &["マク"],
    "夜" => &["ヤ", "ヨ"],
    "粒" => &["ツブ"],
    "枠" => &["ワク"],
    "棹" => &["サオ"],
    "つ折" => &["ツオリ"],
    "つ折り" => &["ツオリ"],
    "つぶ" => &["ツブ"],
    "とき" => &["トキ"],
};

const CONV_TABLE3: DigitLUT = phf_map! {
    "一" => pron!([Hi, To], 0),
    "二" => pron!([Fu, Ta], 0),
   /* "三", "ミ", "1", "1", *//* modified */
};
