use crate::njd_set::digit::lut_conversion::*;
use phf::{phf_map, phf_set};

pub const conversion_table: [(Keys, NumerativeLUT); 5] = [
    (numerative_class2b, conv_table2b),
    (numerative_class2c, conv_table2c),
    (numerative_class2d, conv_table2d),
    (numerative_class2e, conv_table2e),
    (numerative_class2f, conv_table2f),
];

const numerative_class2b: Keys = phf_set! {
    /* from paper */
    "分", "版", "敗", "発", "拍", "鉢", /* from dictionary */
    "波", "派", "泊", "犯", "班", "品", "分間", "分目", "片", "篇", "編", "辺", "遍", "歩", "報",
    "方",
};

const conv_table2b: NumerativeLUT = phf_map! {
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

const numerative_class2c: Keys = phf_set! {
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

const conv_table2c: NumerativeLUT = phf_map! {
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

const numerative_class2d: Keys = phf_set! {
   /* from paper */
   /* "羽", "把", *//* modified */
};

const conv_table2d: NumerativeLUT = phf_map! {
   "三"=>1,
   "六"=>2,
   "八"=>2,
   "十"=>2,
   "百"=>2,
   "千"=>1,
   "万"=>1,
   "何"=>1,
};

const numerative_class2e: Keys = phf_set! {
   /* from paper */
   "軒", "石", "足", "尺",
   /* from dictionary */
   "かけ", "重ね", "件", "勺",
};

const conv_table2e: NumerativeLUT = phf_map! {
   "三"=>1,
   "千"=>1,
   "万"=>1,
};

const numerative_class2f: Keys = phf_set! {/* from paper */ "階"};

const conv_table2f: NumerativeLUT = phf_map! {
   "三"=> 1,
};
