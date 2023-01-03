use crate::njd_set::digit::lut_conversion::*;
use phf::{phf_map, phf_set};

pub const CONVERSION_TABLE: [(Keys, NumerativeLUT); 5] = [
    (NUMERATIVE_CLASS2B, CONV_TABLE2B),
    (NUMERATIVE_CLASS2C, CONV_TABLE2C),
    (NUMERATIVE_CLASS2D, CONV_TABLE2D),
    (NUMERATIVE_CLASS2E, CONV_TABLE2E),
    (NUMERATIVE_CLASS2F, CONV_TABLE2F),
];

const NUMERATIVE_CLASS2B: Keys = phf_set! {
    /* from paper */
    "分", "版", "敗", "発", "拍", "鉢", /* from dictionary */
    "波", "派", "泊", "犯", "班", "品", "分間", "分目", "片", "篇", "編", "辺", "遍", "歩", "報",
    "方",
};

const CONV_TABLE2B: NumerativeLUT = phf_map! {
   "一"=> 2,
   "三"=> 2,
   "四"=> 2,
   "六"=> 2,
   "八"=> 2,
   "十"=> 2,
   "百"=> 2,
   "千"=> 2,
   "万"=> 2,
   "何"=> 2,
};

const NUMERATIVE_CLASS2C: Keys = phf_set! {
    /* from paper */
    "本",
    "匹",
    "疋",
    "票",
    "俵",
    "箱",
    /* from dictionary */
    "本立て",
    "杯",
    "針",
    "柱",
};

const CONV_TABLE2C: NumerativeLUT = phf_map! {
   "一"=> 2,
   "三"=> 1,
   "六"=> 2,
   "八"=> 2,
   "十"=> 2,
   "百"=> 2,
   "千"=> 1,
   "万"=> 1,
   "何"=> 1,
};

const NUMERATIVE_CLASS2D: Keys = phf_set! {
   /* from paper */
   /* "羽", "把", *//* modified */
};

const CONV_TABLE2D: NumerativeLUT = phf_map! {
   "三"=>1,
   "六"=>2,
   "八"=>2,
   "十"=>2,
   "百"=>2,
   "千"=>1,
   "万"=>1,
   "何"=>1,
};

const NUMERATIVE_CLASS2E: Keys = phf_set! {
   /* from paper */
   "軒", "石", "足", "尺",
   /* from dictionary */
   "かけ", "重ね", "件", "勺",
};

const CONV_TABLE2E: NumerativeLUT = phf_map! {
   "三"=>1,
   "千"=>1,
   "万"=>1,
};

const NUMERATIVE_CLASS2F: Keys = phf_set! {/* from paper */ "階"};

const CONV_TABLE2F: NumerativeLUT = phf_map! {
   "三"=> 1,
};
