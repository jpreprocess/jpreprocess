use super::*;
use jpreprocess_core::pron;
use phf::{phf_map, phf_set};

pub const CONVERSION_TABLE: [(Keys, DigitLUT); 11] = [
    (NUMERATIVE_CLASS1B, CONV_TABLE1B),
    (NUMERATIVE_CLASS1C1, CONV_TABLE1C1),
    (NUMERATIVE_CLASS1C2, CONV_TABLE1C2),
    (NUMERATIVE_CLASS1D, CONV_TABLE1D),
    (NUMERATIVE_CLASS1E, CONV_TABLE1E),
    (NUMERATIVE_CLASS1F, CONV_TABLE1F),
    (NUMERATIVE_CLASS1G, CONV_TABLE1G),
    (NUMERATIVE_CLASS1H, CONV_TABLE1H),
    (NUMERATIVE_CLASS1I, CONV_TABLE1I),
    (NUMERATIVE_CLASS1J, CONV_TABLE1J),
    (NUMERATIVE_CLASS1K, CONV_TABLE1K),
];

const NUMERATIVE_CLASS1B: Keys = phf_set! {
   /* from paper */
   "年" /* ねん */ , "円",
   /* from dictionary */
   "年間", "年生", "年代", "年度", "年版", "年余", "年来", "えん",
};
const CONV_TABLE1B: DigitLUT = phf_map! {
    "四" => pron!([Yo], 0),
};

const NUMERATIVE_CLASS1C1: Keys = phf_set! {
   /* from paper */
   "人",
   /* from dictionary */
   "人月", "人前", "人組",
};
const CONV_TABLE1C1: DigitLUT = phf_map! {
    "四" => pron!([Yo], 0),
    "七" => pron!([Shi, Chi], 1),
};

const NUMERATIVE_CLASS1C2: Keys = phf_set! {
   /* from paper */
   "時", "時間",
   /* from dictionary */
   "時限", "時半",
};
const CONV_TABLE1C2: DigitLUT = phf_map! {
    "四" => pron!([Yo], 0),
    "七" => pron!([Shi, Chi], 1),
    "九" => pron!([Ku], 0),
};

const NUMERATIVE_CLASS1D: Keys = phf_set! {
    /* from paper */
    "日", /* にち */
    /* from dictionary */
    "日間",
};
const CONV_TABLE1D: DigitLUT = phf_map! {
   /* "四", "ヨッ", "1", "2", *//* modified */
    "七" => pron!([Shi, Chi], 1),
    "九" => pron!([Ku], 0),
};

const NUMERATIVE_CLASS1E: Keys = phf_set! {/* from paper */ "月" /* がつ */};
const CONV_TABLE1E: DigitLUT = phf_map! {
   "四" => pron!([Shi], 0),
   "七" => pron!([Shi, Chi], 1),
   "九" => pron!([Ku], 0),
};

const NUMERATIVE_CLASS1F: Keys = phf_set! {
   /* from paper */
   /* "羽", "把", *//* modified */
};
const CONV_TABLE1F: DigitLUT = phf_map! {
    "六" => pron!([Ro, Xtsu], 1),
    "八" => pron!([Ha, Xtsu], 1),
    "十" => pron!([Ju, Xtsu], 1),
    "百" => pron!([Hya, Xtsu], 1),
};

const NUMERATIVE_CLASS1G: Keys = phf_set! {
   /* from paper */
   "個", "階", "分" /* ふん */ , "発", "本", "鉢", "口", "切れ", "箱",
   /* from dictionary */
   "か月", "か国", "か所", "か条", "か村", "か年", "カ月", "カ国", "カ寺", "カ所", "カ条", "カ村",
   "カ店", "カ年", "ケ月", "ケ国", "ケ所", "ケ条", "ケ村", "ケ年", "ヵ月", "ヵ国", "ヵ所",
   "ヵ条", "ヵ村", "ヵ年", "ヶ月", "ヶ国", "ヶ所", "ヶ条", "ヶ村", "ヶ年", "個月", "個口",
   "個国", "個条", "個年", "箇月", "箇国", "箇所", "箇条", "箇年", "かけ", "くだり", "けた",
   "価", "課", "画", "回", "回忌", "回生", "回戦", "回線", "回分", "海里", "カイリ", "浬", "角",
   "株", "冠", "巻", "缶", "貫", "貫目", "間", "基", "期", "期生", "機", "気圧", "季", "騎",
   "客", "脚", "球", "級", "橋", "局", "曲", "極", "重ね", "斤", "金", "句", "区", "躯", "計",
   "桁", "ケタ", "校", "港", /* "行", */ "項", "組", "件", "軒", "言", "戸", "湖", "光年", "石",
   "ぴき", "ぺん", "波", "派", "敗", "杯", "拍", "泊", "版", "犯", "班", "匹", "疋", "筆", "俵",
   "票", "品", "分間", "分目", "片", "篇", "編", "辺", "遍", "歩", "報", "方",
   "法", "本立て", "頭身",
};
const CONV_TABLE1G: DigitLUT = phf_map! {
   "一" => pron!([I, Xtsu], 1),
   "六" => pron!([Ro, Xtsu], 1),
   "八" => pron!([Ha, Xtsu], 1),
   "十" => pron!([Ju, Xtsu], 1),
   "百" => pron!([Hya, Xtsu], 1),
};

const NUMERATIVE_CLASS1H: Keys = phf_set! {
   /* from paper */
   "．", "・", "才", "頭", "着", "足", "尺", "坪", "通り", "センチ", "シーシー",
   /* from dictionary */
   "ＣＣ", "ｃｃ", "ｃｍ", "サイクル", "サンチーム", "シーズン", "シート", "シリング",
   "シンガポールドル", "スイスフラン", "スウェーデンクローネ", "スクレ", "セット", "セント",
   "ソル", "ゾーン", "糎", "竿", "差", "差し", "歳", "歳児", "作", "冊", "刷", "皿", "棹",
   "艘", "子", "視", "式", "失", "室", "射", "社", "勺", "種", "首", "周", "周忌", "周年", "州",
   "週", "週間", "集", "宿", "所", "勝", "升", "床", "章", "色", "食", "親等", "進",
   "進数", "品", "すじ", "そう", "そろい", "筋", "数", "寸", "世", "隻", "席", "石", "節", "戦",
   "線", "選", "銭", "層", "相", "揃", "たび", "つかみ", "つがい", "つぶ", "つまみ", "つ折",
   "つ折り", "とおり", "とき", "ところ", "とせ", "玉", "月", "手", "束", "続き", "体", "対",
   "卓", "樽", "反", "丁", "丁目", "鳥", "通", "掴み", "艇", "滴", "店", "転", "点", "斗", "棟",
   "盗", "灯", "等", "等席", "等地", "等分", "答", "得", "噸", "粒", "種類", "歳馬", "世紀",
   "車種",
};
const CONV_TABLE1H: DigitLUT = phf_map! {
    "一" => pron!([I, Xtsu], 1),
    "八" => pron!([Ha, Xtsu], 1),
    "十" => pron!([Ju, Xtsu], 1),
};

const NUMERATIVE_CLASS1I: Keys = phf_set! {
   /* from paper */
   "キロ", "カロリー",
   /* from dictionary */
   "ｃａｌ", "ｋｂ", "ｋｇ", "ｋｌ", "ｋｍ", "ｋｔ", "ｋｗ", "ｋグラム", "ｋバイト", "ｋヘルツ",
   "ｋメートル", "ｋリットル", "ｋワット", "カナダドル", "カラット", "ガロン", "キュリー",
   "キロカロリー", "キログラム", "キロトン", "キロバイト", "キロヘルツ", "キロメートル",
   "キロリットル", "キロワット", "キロワット時", "クラス", "クローナ", "クローネ", "グァラニ",
   "ケース", "コース", "粁",
};
const CONV_TABLE1I: DigitLUT = phf_map! {
    "六" => pron!([Ro, Xtsu], 1),
    "十" => pron!([Ju, Xtsu], 1),
    "百" => pron!([Hya, Xtsu], 1),
};

const NUMERATIVE_CLASS1J: Keys = phf_set! {
   /* from paper */
   "トン",
   /* from dictionary */
   "ｔ", "タル", "テラ", "トライ",
};
const CONV_TABLE1J: DigitLUT = phf_map! {
    "一" => pron!([I, Xtsu], 1),
    "十" => pron!([Ju, Xtsu], 1),
};

const NUMERATIVE_CLASS1K: Keys = phf_set! {
   /* from paper */
   "房" /* ふさ */ , "柱", "％", "ポンド",
   /* from dictionary */
   "ｐａ", "ｐｐｍ", "パーセント", "パーミル", "パスカル", "パック", "パット", "ピーピーエム",
   "ピコ", "ページ", "頁", "ペア", "ペセタ", "ペソ", "ペニー", "ペニヒ", "ペンス", "ポイント",
   "振り", "針", "袋", "張り", "平米", "平方キロ", "平方キロメートル", "平方センチメートル",
   "平方メートル", "品目",
};
const CONV_TABLE1K: DigitLUT = phf_map! {
    "十" => pron!([Ju, Xtsu], 1),
};
