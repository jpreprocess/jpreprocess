use crate::njd_set::digit::lut_conversion::*;
use phf::{phf_map, phf_set};

pub const digit_conversion_table: [(Keys, DigitLUT); 2] = [
    (numeral_list8, numeral_list9),
    (numeral_list10, numeral_list11),
];
pub const numerative_conversion_table: [(Keys, NumerativeLUT); 1] =
    [(numeral_list6, numeral_list7)];

pub const numeral_list4: Keys = phf_set! {
  "一", "二", "三", "四", "五", "六", "七", "八", "九", "何", "幾", "数",
};

pub const numeral_list5: Keys = phf_set! {
 "十", "百", "千", "万", "億", "兆", "京", "垓",
 "𥝱",
 "穣", "溝", "澗", "正", "載", "極",
 "恒河沙", "阿僧祇", "那由他", "不可思議", "無量大数",
};

const numeral_list6: Keys = phf_set! {"百", "千"};

const numeral_list7: NumerativeLUT = phf_map! {
 "三"=>1,
 "六"=>2,
 "八"=>2,
 "何"=>1,
};

const numeral_list8: Keys = phf_set! {"百"};

const numeral_list9: DigitLUT = phf_map! {
 "六"=> ("ロッ", 0, 2),
 "八"=>("ハッ", 0, 2),
};

const numeral_list10: Keys = phf_set! {"千", "兆"};

const numeral_list11: DigitLUT = phf_map! {
 "一"=> ("イッ", 0, 2),
 "八"=> ("ハッ", 0, 2),
 "十"=> ("ジュッ", 1, 2),
};
