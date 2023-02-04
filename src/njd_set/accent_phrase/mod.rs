mod rule;

use crate::njd::pos::*;
use crate::njd::*;

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
    match (
        prev_pos.get_group0(),
        prev_pos.get_group1(),
        curr_pos.get_group0(),
        curr_pos.get_group1(),
    ) {
        //          /* Rule 18 */
        //          if (strcmp(NJDNode_get_pos_group1(node), NJD_SET_ACCENT_PHRASE_SETSUBI) == 0)
        //             NJDNode_set_chain_flag(node, 1);
        //       }
        //    }
        (_, _, _, Group1::Setsubi) => true,

        //          /* Rule 17 */
        //          if (strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_MEISHI) == 0
        //              && strcmp(NJDNode_get_pos_group3(node), NJD_SET_ACCENT_PHRASE_MEI) == 0)
        //             NJDNode_set_chain_flag(node, 0);
        (Group0::Meishi, _, _, _) if Group3::Mei == curr_pos.get_group3() => false,

        //          /* Rule 16 */
        //          if (strcmp(NJDNode_get_pos_group3(node->prev), NJD_SET_ACCENT_PHRASE_SEI) == 0
        //              && strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_MEISHI) == 0)
        //             NJDNode_set_chain_flag(node, 0);
        (_, _, Group0::Meishi, _) if Group3::Sei == prev_pos.get_group3() => false,

        //          /* Rule 15 */
        //          if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_SETTOUSHI) == 0)
        //             NJDNode_set_chain_flag(node, 0);
        (_, _, Group0::Settoushi, _) => false,

        //          /* Rule 14 */
        //          if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_KIGOU) == 0 ||
        //              strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_KIGOU) == 0)
        //             NJDNode_set_chain_flag(node, 0);
        (Group0::Kigou, _, _, _) => false,
        (_, _, Group0::Kigou, _) => false,

        //          /* Rule 13 */
        //          if (strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_MEISHI) == 0) {
        //             if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_DOUSHI) == 0 ||
        //                 strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_KEIYOUSHI) == 0 ||
        //                 strcmp(NJDNode_get_pos_group1(node), NJD_SET_ACCENT_PHRASE_KEIYOUDOUSHI_GOKAN) == 0)
        //                NJDNode_set_chain_flag(node, 0);
        //          }
        (Group0::Meishi, _, Group0::Doushi | Group0::Keiyoushi, _) => false,
        (Group0::Meishi, _, _, Group1::KeiyoudoushiGokan) => false,

        //          /* Rule 12 */
        //          if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_DOUSHI) == 0)
        //             if (strcmp(NJDNode_get_pos_group1(node), NJD_SET_ACCENT_PHRASE_HIJIRITSU) == 0) {
        //                if (strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_DOUSHI) == 0) {
        //                   if (strtopcmp(NJDNode_get_cform(node->prev), NJD_SET_ACCENT_PHRASE_RENYOU) != -1)
        //                      NJDNode_set_chain_flag(node, 1);
        //                } else if (strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_MEISHI) == 0) {
        //                   if (strcmp
        //                       (NJDNode_get_pos_group1(node->prev),
        //                        NJD_SET_ACCENT_PHRASE_SAHEN_SETSUZOKU) == 0)
        //                      NJDNode_set_chain_flag(node, 1);
        //                }
        //             }
        (Group0::Doushi, _, Group0::Doushi, Group1::Hijiritsu) if prev.is_renyou() => true,
        //(Group0::Meishi, Group1::SahenSetsuzoku, Group0::Doushi, Group1::Hijiritsu) => true,

        //          /* Rule 11 */
        //          if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_KEIYOUSHI) == 0)
        //             if (strcmp(NJDNode_get_pos_group1(node), NJD_SET_ACCENT_PHRASE_HIJIRITSU) == 0) {
        //                if (strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_DOUSHI) == 0) {
        //                   if (strtopcmp(NJDNode_get_cform(node->prev), NJD_SET_ACCENT_PHRASE_RENYOU) != -1)
        //                      NJDNode_set_chain_flag(node, 1);
        //                } else if (strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_KEIYOUSHI)
        //                           == 0) {
        //                   if (strtopcmp(NJDNode_get_cform(node->prev), NJD_SET_ACCENT_PHRASE_RENYOU) != -1)
        //                      NJDNode_set_chain_flag(node, 1);
        //                } else if (strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_JOSHI) == 0) {
        //                   if (strcmp
        //                       (NJDNode_get_pos_group1(node->prev),
        //                        NJD_SET_ACCENT_PHRASE_SETSUZOKUJOSHI) == 0) {
        //                      if (strcmp(NJDNode_get_string(node->prev), NJD_SET_ACCENT_PHRASE_TE) == 0)
        //                         NJDNode_set_chain_flag(node, 1);
        //                      else if (strcmp(NJDNode_get_string(node->prev), NJD_SET_ACCENT_PHRASE_DE) == 0)
        //                         NJDNode_set_chain_flag(node, 1);
        //                   }
        //                }
        //             }
        (Group0::Doushi, _, Group0::Keiyoushi, Group1::Hijiritsu) if prev.is_renyou() => true,
        (Group0::Keiyoushi, _, Group0::Keiyoushi, Group1::Hijiritsu) if prev.is_renyou() => true,
        (Group0::Joshi, Group1::Setsuzokujoshi, Group0::Keiyoushi, Group1::Hijiritsu)
            if match prev.get_string() {
                rule::TE | rule::DE => true,
                _ => false,
            } =>
        {
            true
        }

        //          /* Rule 10 */
        //          if (strcmp(NJDNode_get_pos_group1(node->prev), NJD_SET_ACCENT_PHRASE_SETSUBI) == 0)
        //             if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_MEISHI) == 0)
        //                NJDNode_set_chain_flag(node, 0);
        (_, Group1::Setsubi, Group0::Meishi, _) => false,

        //          /* Rule 09 */
        //          if (strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_JODOUSHI) == 0)
        //             if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_JODOUSHI) != 0 &&
        //                 strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_JOSHI) != 0)
        //                NJDNode_set_chain_flag(node, 0);
        //          if (strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_JOSHI) == 0)
        //             if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_JODOUSHI) != 0 &&
        //                 strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_JOSHI) != 0)
        //                NJDNode_set_chain_flag(node, 0);
        (Group0::Jodoushi | Group0::Joshi, _, _, _)
            if match curr_pos.get_group0() {
                Group0::Jodoushi | Group0::Joshi => false,
                _ => true,
            } =>
        {
            false
        }

        //          /* Rule 08 */
        //          if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_JODOUSHI) == 0)
        //             NJDNode_set_chain_flag(node, 1);
        //          if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_JOSHI) == 0)
        //             NJDNode_set_chain_flag(node, 1);
        (_, _, Group0::Jodoushi | Group0::Joshi, _) => true,
        //          /* Rule 07 */
        //          if (strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_MEISHI) == 0)
        //             if (strcmp(NJDNode_get_pos_group1(node->prev), NJD_SET_ACCENT_PHRASE_FUKUSHI_KANOU)
        //                 == 0)
        //                NJDNode_set_chain_flag(node, 0);
        //          if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_MEISHI) == 0)
        //             if (strcmp(NJDNode_get_pos_group1(node), NJD_SET_ACCENT_PHRASE_FUKUSHI_KANOU) == 0)
        //                NJDNode_set_chain_flag(node, 0);
        (Group0::Meishi, Group1::FukushiKanou, _, _) => false,
        (_, _, Group0::Meishi, Group1::FukushiKanou) => false,

        //          /* Rule 06 */
        //          if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_FUKUSHI) == 0
        //              || strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_FUKUSHI) == 0
        //              || strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_SETSUZOKUSHI) == 0
        //              || strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_SETSUZOKUSHI) == 0
        //              || strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_RENTAISHI) == 0
        //              || strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_RENTAISHI) == 0)
        //             NJDNode_set_chain_flag(node, 0);
        (Group0::Fukushi | Group0::Setsuzokushi | Group0::Rentaishi, _, _, _) => false,
        (_, _, Group0::Fukushi | Group0::Setsuzokushi | Group0::Rentaishi, _) => false,

        //          /* Rule 05 */
        //          if (strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_DOUSHI) == 0) {
        //             if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_KEIYOUSHI) == 0)
        //                NJDNode_set_chain_flag(node, 0);
        //             else if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_MEISHI) == 0)
        //                NJDNode_set_chain_flag(node, 0);
        //          }
        (Group0::Doushi, _, Group0::Keiyoushi | Group0::Meishi, _) => false,

        //          /* Rule 04 */
        //          if (strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_MEISHI) == 0)
        //             if (strcmp
        //                 (NJDNode_get_pos_group1(node->prev), NJD_SET_ACCENT_PHRASE_KEIYOUDOUSHI_GOKAN) == 0)
        //                if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_MEISHI) == 0)
        //                   NJDNode_set_chain_flag(node, 0);
        (Group0::Meishi, Group1::KeiyoudoushiGokan, Group0::Meishi, _) => false,

        //          /* Rule 03 */
        //          if (strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_KEIYOUSHI) == 0)
        //             if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_MEISHI) == 0)
        //                NJDNode_set_chain_flag(node, 0);
        (Group0::Keiyoushi, _, Group0::Meishi, _) => false,
        //          /* Rule 02 */
        //          if (strcmp(NJDNode_get_pos(node->prev), NJD_SET_ACCENT_PHRASE_MEISHI) == 0)
        //             if (strcmp(NJDNode_get_pos(node), NJD_SET_ACCENT_PHRASE_MEISHI) == 0)
        //                NJDNode_set_chain_flag(node, 1);
        (Group0::Meishi, _, Group0::Meishi, _) => true,

        //          /* Rule 01 */
        //          NJDNode_set_chain_flag(node, 1);
        _ => true,
    }
}
