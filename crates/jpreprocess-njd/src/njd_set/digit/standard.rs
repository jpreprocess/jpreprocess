use phf::{phf_map, Map};

use crate::NJDNode;

pub fn is_period(s: &str) -> bool {
    s == "．" || s == "・"
}

pub fn normalize_digit(node: &mut NJDNode) -> bool {
    if node.get_string() != "*" && node.get_pos().is_kazu() {
        if let Some(replace) = DIGIT_NORMALIZE.get(node.get_string()) {
            node.replace_string(replace);
            return true;
        }
    }
    false
}

// NUMERAL_LIST1 in OpenJTalk
const DIGIT_NORMALIZE: Map<&'static str, &'static str> = phf_map! {
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
