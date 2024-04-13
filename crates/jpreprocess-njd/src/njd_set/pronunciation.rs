use crate::NJD;

const QUESTION: &str = "？";
const DESU_STR: &str = "です";
const MASU_STR: &str = "ます";

use jpreprocess_core::{
    pos::*,
    pron,
    pronunciation::{MoraEnum, Pronunciation},
};

use jpreprocess_window::*;

pub fn njd_set_pronunciation(njd: &mut NJD) {
    for node in &mut njd.nodes {
        if node.get_pron().mora_size() == 0 {
            let pron = Pronunciation::parse(node.get_string(), 0).unwrap_or_default();
            let mora_size = pron.mora_size();

            /* if filler, overwrite pos */
            if mora_size != 0 {
                *node.get_pos_mut() = POS::Filler;
            }

            if pron.is_touten() {
                node.get_pos_mut().convert_to_kigou();
            }

            if pron.is_empty() {
                node.reset();
            } else {
                let read_string = pron.to_pure_string();
                node.set_pron(pron);
                node.set_read(&read_string);
            }
        }
    }

    njd.remove_silent_node();

    /* chain kana sequence */
    {
        let mut head_of_kana_filler_sequence_index: Option<usize> = None;
        for i in 0..njd.nodes.len() {
            let (head_of_kana_filler_sequence, node) = {
                let (a, b) = njd.nodes.split_at_mut(i);
                let head_of_kana_filler_sequence =
                    head_of_kana_filler_sequence_index.and_then(|i| a.get_mut(i));
                let node = b.get_mut(0).unwrap();
                (head_of_kana_filler_sequence, node)
            };
            if matches!(node.get_pos(), POS::Filler) {
                if Pronunciation::is_mora_convertable(node.get_string()) {
                    if let Some(seq) = head_of_kana_filler_sequence {
                        seq.transfer_from(node);
                    } else {
                        head_of_kana_filler_sequence_index = Some(i);
                    }
                } else {
                    head_of_kana_filler_sequence_index = None;
                }
            } else {
                head_of_kana_filler_sequence_index = None;
            }
        }
    }

    njd.remove_silent_node();

    {
        let mut iter = njd.iter_quint_mut();
        while let Some(quint) = iter.next() {
            let (node, next) = match Triple::from(quint) {
                Triple::First(node, next) => (node, next),
                Triple::Full(_, node, next) => (node, next),
                _ => continue,
            };
            if next.get_pron().mora_matches(MoraEnum::U)
                && matches!(next.get_pos(), POS::Jodoushi)
                && matches!(node.get_pos(), POS::Doushi(_) | POS::Jodoushi)
                && node.get_pron().mora_size() > 0
            {
                next.set_pron(pron!([Long], 0));
            }
            if matches!(node.get_pos(), POS::Jodoushi) && next.get_string() == QUESTION {
                match node.get_string() {
                    DESU_STR => node.set_pron(pron!([De, Su], 1)),
                    MASU_STR => node.set_pron(pron!([Ma, Su], 1)),
                    _ => (),
                }
            }
        }
    }
}
