use crate::njd_set::digit::lut_conversion::*;
use phf::{phf_map, phf_set};

pub const conversion_table: [(Keys, DigitLUT); 11] = [
    (numerative_class1b, conv_table1b),
    (numerative_class1c1, conv_table1c1),
    (numerative_class1c2, conv_table1c2),
    (numerative_class1d, conv_table1d),
    (numerative_class1e, conv_table1e),
    (numerative_class1f, conv_table1f),
    (numerative_class1g, conv_table1g),
    (numerative_class1h, conv_table1h),
    (numerative_class1i, conv_table1i),
    (numerative_class1j, conv_table1j),
    (numerative_class1k, conv_table1k),
];

const numerative_class1b: Keys = phf_set! {
   /* from paper */
   "年" /* ねん */ , "円",
   /* from dictionary */
   "年間", "年生", "年代", "年度", "年版", "年余", "年来", "えん",
};
const numerative_class1c1: Keys = phf_set! {
   /* from paper */
   "人",
   /* from dictionary */
   "人月", "人前", "人組",
};
const numerative_class1c2: Keys = phf_set! {
   /* from paper */
   "時", "時間",
   /* from dictionary */
   "時限", "時半",
};
const numerative_class1d: Keys = phf_set! {
    /* from paper */
    "日", /* にち */
    /* from dictionary */
    "日間",
};
const numerative_class1e: Keys = phf_set! {/* from paper */ "月" /* がつ */};
const numerative_class1f: Keys = phf_set! {
   /* from paper */
   /* "羽", "把", *//* modified */
};
const numerative_class1g: Keys = phf_set! {
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
const numerative_class1h: Keys = phf_set! {
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
const numerative_class1i: Keys = phf_set! {
   /* from paper */
   "キロ", "カロリー",
   /* from dictionary */
   "ｃａｌ", "ｋｂ", "ｋｇ", "ｋｌ", "ｋｍ", "ｋｔ", "ｋｗ", "ｋグラム", "ｋバイト", "ｋヘルツ",
   "ｋメートル", "ｋリットル", "ｋワット", "カナダドル", "カラット", "ガロン", "キュリー",
   "キロカロリー", "キログラム", "キロトン", "キロバイト", "キロヘルツ", "キロメートル",
   "キロリットル", "キロワット", "キロワット時", "クラス", "クローナ", "クローネ", "グァラニ",
   "ケース", "コース", "粁",
};
const numerative_class1j: Keys = phf_set! {
   /* from paper */
   "トン",
   /* from dictionary */
   "ｔ", "タル", "テラ", "トライ",
};
const numerative_class1k: Keys = phf_set! {
   /* from paper */
   "房" /* ふさ */ , "柱", "％", "ポンド",
   /* from dictionary */
   "ｐａ", "ｐｐｍ", "パーセント", "パーミル", "パスカル", "パック", "パット", "ピーピーエム",
   "ピコ", "ページ", "頁", "ペア", "ペセタ", "ペソ", "ペニー", "ペニヒ", "ペンス", "ポイント",
   "振り", "針", "袋", "張り", "平米", "平方キロ", "平方キロメートル", "平方センチメートル",
   "平方メートル", "品目",
};

const conv_table1b: DigitLUT = phf_map! {
   "四"=> ("ヨ", 0, 1),
};
const conv_table1c1: DigitLUT = phf_map! {
   "四"=> ("ヨ", 0, 1),
   "七"=>("シチ", 1, 2),
};
const conv_table1c2: DigitLUT = phf_map! {
   "四"=>("ヨ", 0, 1),
   "七"=> ("シチ", 1, 2),
   "九"=> ("ク", 0, 1),
};
const conv_table1d: DigitLUT = phf_map! {
   /* "四", "ヨッ", "1", "2", *//* modified */
   "七"=> ("シチ", 1, 2),
   "九"=> ("ク", 0, 1),
};

const conv_table1e: DigitLUT = phf_map! {
   "四"=>("シ", 0, 1),
   "七"=>("シチ", 1, 2),
   "九"=>("ク", 0, 1),
};
const conv_table1f: DigitLUT = phf_map! {
   "六"=> ("ロッ", 1, 2),
   "八"=> ("ハッ", 1, 2),
   "十"=> ("ジュッ", 1, 2),
   "百"=> ("ヒャッ", 1, 2),
};
const conv_table1g: DigitLUT = phf_map! {
   "一"=> ("イッ", 1, 2),
   "六"=> ("ロッ", 1, 2),
   "八"=> ("ハッ", 1, 2),
   "十"=> ("ジュッ", 1, 2),
   "百"=> ("ヒャッ", 1, 2),
};
const conv_table1h: DigitLUT = phf_map! {
   "一"=> ("イッ", 1, 2),
   "八"=>("ハッ", 1, 2),
   "十"=> ("ジュッ", 1, 2),
};

const conv_table1i: DigitLUT = phf_map! {
   "六"=> ("ロッ", 1, 2),
   "十"=> ("ジュッ", 1, 2),
   "百"=> ("ヒャッ", 1, 2),
};

const conv_table1j: DigitLUT = phf_map! {
   "一"=> ("イッ", 1, 2),
   "十"=> ("ジュッ", 1, 2),
};
const conv_table1k: DigitLUT = phf_map! {
   "十"=>("ジュッ", 1, 2),
};
