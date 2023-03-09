mod rule;

use jpreprocess_core::pos::*;
use jpreprocess_core::*;
use jpreprocess_njd::NJD;

use crate::window::*;

pub fn njd_set_accent_phrase(njd: &mut NJD) {
    if njd.nodes.len() == 0 {
        return;
    }
    let mut iter = njd.iter_quint_mut();
    while let Some(quint) = iter.next() {
        let (prev, node) = match Double::from(quint) {
            Double::Full(p, c) => (p, c),
            _ => continue,
        };
        if node.get_chain_flag() == None {
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
            if matches!(prev.get_string(), rule::TE | rule::DE) =>
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
