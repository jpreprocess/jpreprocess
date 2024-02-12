/*
  Rule 01 デフォルトはくっつける
  Rule 02 「名詞」の連続はくっつける
  Rule 03 「形容詞」の後に「名詞」がきたら別のアクセント句に
  Rule 04 「名詞,形容動詞語幹」の後に「名詞」がきたら別のアクセント句に
  Rule 05 「動詞」の後に「形容詞」or「名詞」がきたら別のアクセント句に
  Rule 06 「副詞」，「接続詞」，「連体詞」は単独のアクセント句に
  Rule 07 「名詞,副詞可能」（すべて，など）は単独のアクセント句に
  Rule 08 「助詞」or「助動詞」（付属語）は前にくっつける
  Rule 09 「助詞」or「助動詞」（付属語）の後の「助詞」，「助動詞」以外（自立語）は別のアクセント句に
  Rule 10 「*,接尾」の後の「名詞」は別のアクセント句に
  Rule 11 「形容詞,非自立」は「動詞,連用*」or「形容詞,連用*」or「助詞,接続助詞,て」or「助詞,接続助詞,で」に接続する場合に前にくっつける
  Rule 12 「動詞,非自立」は「動詞,連用*」or「名詞,サ変接続」に接続する場合に前にくっつける
  Rule 13 「名詞」の後に「動詞」or「形容詞」or「名詞,形容動詞語幹」がきたら別のアクセント句に
  Rule 14 「記号」は単独のアクセント句に
  Rule 15 「接頭詞」は単独のアクセント句に
  Rule 16 「*,*,*,姓」の後の「名詞」は別のアクセント句に
  Rule 17 「名詞」の後の「*,*,*,名」は別のアクセント句に
  Rule 18 「*,接尾」は前にくっつける
*/

pub const TE: &str = "て";
pub const DE: &str = "で";

use crate::{NJDNode, NJD};
use jpreprocess_core::pos::*;

use jpreprocess_window::*;

pub fn njd_set_accent_phrase(njd: &mut NJD) {
    if njd.nodes.is_empty() {
        return;
    }
    let mut iter = njd.iter_quint_mut();
    while let Some(quint) = iter.next() {
        let (prev, node) = match Double::from(quint) {
            Double::Full(p, c) => (p, c),
            _ => continue,
        };
        if node.get_chain_flag().is_none() {
            let chain: bool = chain_flag(prev, node);
            node.set_chain_flag(chain);
        }
    }
}

fn chain_flag(prev: &NJDNode, node: &NJDNode) -> bool {
    let prev_pos = prev.get_pos();
    let curr_pos = node.get_pos();
    match (prev_pos, curr_pos) {
        /* Rule 18 */
        (
            _,
            POS::Keiyoushi(Keiyoushi::Setsubi)
            | POS::Doushi(Doushi::Setsubi)
            | POS::Meishi(Meishi::Setsubi(_)),
        ) => true,
        /* Rule 17 */
        (POS::Meishi(_), POS::Meishi(Meishi::KoyuMeishi(KoyuMeishi::Person(Person::Mei)))) => false,
        /* Rule 16 */
        (POS::Meishi(Meishi::KoyuMeishi(KoyuMeishi::Person(Person::Sei))), POS::Meishi(_)) => false,
        /* Rule 15 */
        (_, POS::Settoushi(_)) => false,
        /* Rule 14 */
        (POS::Kigou(_), _) => false,
        (_, POS::Kigou(_)) => false,
        /* Rule 13 */
        (POS::Meishi(_), POS::Doushi(_)) => false,
        (POS::Meishi(_), POS::Keiyoushi(_)) => false,
        (POS::Meishi(_), POS::Meishi(Meishi::KeiyoudoushiGokan)) => false,
        /* Rule 12 */
        (POS::Doushi(_), POS::Doushi(Doushi::Hijiritsu)) if prev.is_renyou() => true,
        /* Rule 11 */
        (POS::Doushi(_), POS::Keiyoushi(Keiyoushi::Hijiritsu)) if prev.is_renyou() => true,
        (POS::Keiyoushi(_), POS::Keiyoushi(Keiyoushi::Hijiritsu)) if prev.is_renyou() => true,
        (POS::Joshi(Joshi::SetsuzokuJoshi), POS::Keiyoushi(Keiyoushi::Hijiritsu))
            if matches!(prev.get_string(), TE | DE) =>
        {
            true
        }
        /* Rule 10 */
        (
            POS::Keiyoushi(Keiyoushi::Setsubi)
            | POS::Doushi(Doushi::Setsubi)
            | POS::Meishi(Meishi::Setsubi(_)),
            POS::Meishi(_),
        ) => false,
        /* Rule 08 */
        (POS::Jodoushi | POS::Joshi(_), POS::Jodoushi | POS::Joshi(_)) => true,
        /* Rule 09 */
        (POS::Jodoushi | POS::Joshi(_), _) => false,
        /* Rule 08 */
        (_, POS::Jodoushi | POS::Joshi(_)) => true,
        /* Rule 07 */
        (POS::Meishi(Meishi::FukushiKanou), _) => false,
        (_, POS::Meishi(Meishi::FukushiKanou)) => false,
        /* Rule 06 */
        (POS::Fukushi(_) | POS::Setsuzokushi | POS::Rentaishi, _) => false,
        (_, POS::Fukushi(_) | POS::Setsuzokushi | POS::Rentaishi) => false,
        /* Rule 05 */
        (POS::Doushi(_), POS::Keiyoushi(_) | POS::Meishi(_)) => false,
        /* Rule 04 */
        (POS::Meishi(Meishi::KeiyoudoushiGokan), POS::Meishi(_)) => false,
        /* Rule 03 */
        (POS::Keiyoushi(_), POS::Meishi(_)) => false,
        /* Rule 02 */
        (POS::Meishi(_), POS::Meishi(_)) => true,
        /* Rule 01 */
        _ => true,
    }
}
