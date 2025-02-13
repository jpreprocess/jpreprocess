//! Set pronunciation based on various clues.

use crate::NJD;

use jpreprocess_core::{
    pos::*,
    pron,
    pronunciation::{MoraEnum, Pronunciation},
};

use jpreprocess_window::*;

pub fn njd_set_pronunciation(njd: &mut NJD) {
    {
        let nodes = std::mem::take(&mut njd.nodes);
        for node in nodes {
            if node.get_pron().mora_size() != 0 {
                njd.nodes.push(node);
                continue;
            }

            let prons = Pronunciation::parse_mora_str(node.get_string());
            if prons.is_empty() {
                continue;
            }
            for (range, moras) in prons {
                let string = &node.get_string()[range.clone()];
                let mut node = node.clone();
                node.replace_string(string);

                let pron = Pronunciation::new(moras, 0);

                let mora_size = pron.mora_size();
                if mora_size == 0 {
                    if pron.is_touten() {
                        node.get_pos_mut().convert_to_kigou();
                    }
                } else {
                    *node.get_pos_mut() = POS::Filler;
                }
                if pron.is_empty() {
                    node.reset();
                } else {
                    let read_string = pron.to_pure_string();
                    node.set_pron(pron);
                    node.set_read(&read_string);
                    njd.nodes.push(node);
                }
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
            if matches!(node.get_pos(), POS::Jodoushi) && next.get_string() == "？" {
                match node.get_string() {
                    "です" => node.set_pron(pron!([De, Su], 1)),
                    "ます" => node.set_pron(pron!([Ma, Su], 1)),
                    _ => (),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pronunciation::njd_set_pronunciation, NJD};

    #[test]
    fn barry_payne() {
        let mut njd: NJD = [
            "バリー・ペーン,名詞,*,*,*,*,*,バリー・ペーン,*,,0/0,*,-1",
            "は,名詞,*,*,*,*,*,は,*,,0/0,*,-1",
        ]
        .into_iter()
        .collect();

        njd_set_pronunciation(&mut njd);

        assert_eq!(njd.nodes[0].get_pron().mora_size(), 3);
        assert_eq!(njd.nodes[1].get_pron().mora_size(), 0);
        assert_eq!(njd.nodes[2].get_pron().mora_size(), 3);
        assert_eq!(njd.nodes[3].get_pron().mora_size(), 1);
    }
}
