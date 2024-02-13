use phf::{phf_map, Map};

pub fn is_period(s: &str) -> bool {
    s == "．" || s == "・"
}

pub const TEN_FEATURE: &str = "．,名詞,接尾,助数詞,*,*,*,．,テン,テン,0/2,*,-1";
pub const ZERO1: &str = "〇";
pub const ZERO2: &str = "０";
pub const ZERO_BEFORE_DP: &str = "レー";
pub const TWO: &str = "二";
pub const TWO_BEFORE_DP: &str = "ニー";
pub const FIVE: &str = "五";
pub const FIVE_BEFORE_DP: &str = "ゴー";
pub const SIX: &str = "六";

pub const GATSU: &str = "月";
pub const NICHI: &str = "日";
pub const NICHIKAN: &str = "日間";

pub const ONE: &str = "一";
pub const TSUITACHI: &str = "一日,名詞,副詞可能,*,*,*,*,一日,ツイタチ,ツイタチ,4/4,*";

pub const FOUR: &str = "四";
pub const TEN: &str = "十";
pub const JUYOKKA: &str = "十四日,名詞,副詞可能,*,*,*,*,十四日,ジュウヨッカ,ジューヨッカ,1/5,*";
pub const JUYOKKAKAN: &str =
    "十四日間,名詞,副詞可能,*,*,*,*,十四日間,ジュウヨッカカン,ジューヨッカカン,5/7,*";
pub const NIJU: &str = "二十,名詞,副詞可能,*,*,*,*,二十,ニジュウ,ニジュー,1/3,*";
pub const YOKKA: &str = "四日,名詞,副詞可能,*,*,*,*,四日,ヨッカ,ヨッカ,0/3,*,0";
pub const YOKKAKAN: &str = "四日間,名詞,副詞可能,*,*,*,*,四日間,ヨッカカン,ヨッカカン,3/5,*,0";
pub const HATSUKA: &str = "二十日,名詞,副詞可能,*,*,*,*,二十日,ハツカ,ハツカ,0/3,*";
pub const HATSUKAKAN: &str = "二十日間,名詞,副詞可能,*,*,*,*,二十日間,ハツカカン,ハツカカン,3/5,*";

pub const DIGIT_NORMALIZE: Map<&'static str, &'static str> = phf_map! {
   "○" => "〇",
   "１" => "一",
   "２" => "二",
   "３" => "三",
   "４" => "四",
   "５" => "五",
   "６" => "六",
   "７" => "七",
   "８" => "八",
   "９" => "九",
   "一" => "一",
   "二" => "二",
   "三" => "三",
   "四" => "四",
   "五" => "五",
   "六" => "六",
   "七" => "七",
   "八" => "八",
   "九" => "九",
   "いち" => "一",
   "に" => "二",
   "さん" => "三",
   "よん" => "四",
   "ご" => "五",
   "ろく" => "六",
   "なな" => "七",
   "はち" => "八",
   "きゅう" => "九",
   "〇" => "〇",
   "０" => "０",
   "壱" => "一",
   "弐" => "二",
   "貳" => "二",
   "ニ" => "二",
   "参" => "三",
   "し" => "四",
   "しち" => "七",
   "く" => "九"
};
