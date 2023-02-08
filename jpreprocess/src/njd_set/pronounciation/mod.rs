use jpreprocess_njd::NJD;

mod rule;

use jpreprocess_njd::pos::*;

use crate::window::*;

pub fn njd_set_pronunciation(njd: &mut NJD) {
    for node in &mut njd.nodes {
        if node.get_mora_size() == 0 {
            node.unset_read();
            node.unset_pron();
            /* if the word is kana, set them as filler */
            {
                let mut read_add = "".to_string();
                let mut mora_size_delta = 0;

                {
                    let mut chars = node.get_string().chars();
                    let mut lprev = chars.next();
                    loop {
                        let lcurr = chars.next();
                        let (l1, l2) = {
                            if let Some(lp) = lprev {
                                if let Some(lc) = lcurr {
                                    (lp.to_string(), [lp, lc].iter().collect::<String>())
                                } else {
                                    (lp.to_string(), "".to_string())
                                }
                            } else {
                                break;
                            }
                        };
                        if let Some((read, mora_size)) = rule::LIST.get(&l2) {
                            read_add.push_str(read);
                            mora_size_delta += mora_size;
                            lprev = chars.next();
                            continue;
                        }
                        if let Some((read, mora_size)) = rule::LIST.get(&l1) {
                            read_add.push_str(read);
                            mora_size_delta += mora_size;
                        }
                        lprev = lcurr;
                    }
                }

                if !read_add.is_empty() {
                    node.set_read(read_add.as_str());
                    node.set_pron(read_add.as_str());
                }
                node.add_mora_size(mora_size_delta);

                /* if filler, overwrite pos */
                if node.get_mora_size() != 0 {
                    *node.get_pos_mut() = PartOfSpeech::new([rule::FILLER, "", "", ""]);
                }
                node.ensure_orig();
            }
            /* if known symbol, set the pronunciation */
            if node.get_pron().is_none() {
                if let Some(conv) = rule::SYMBOL_LIST.get(node.get_string()) {
                    node.set_read(conv);
                    node.set_pron(conv);
                }
            }
            /* if the word is not kana, set pause symbol */
            if node.get_pron().is_none() {
                node.set_read(rule::TOUTEN);
                node.set_pron(rule::TOUTEN);
                node.get_pos_mut().set_group0(rule::KIGOU);
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
            if matches!(node.get_pos().get_group0(), Group0::Filler) {
                if rule::LIST.get(node.get_string()).is_some() {
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
            if matches!(next.get_pron(),Some(pron) if pron==rule::U)
                && matches!(next.get_pos().get_group0(), Group0::Jodoushi)
                && matches!(
                    node.get_pos().get_group0(),
                    Group0::Doushi | Group0::Jodoushi
                )
                && node.get_mora_size() > 0
            {
                next.set_pron(rule::CHOUON);
            }
            if matches!(node.get_pos().get_group0(), Group0::Jodoushi)
                && next.get_string() == rule::QUESTION
            {
                match node.get_string() {
                    rule::DESU_STR => node.set_pron(rule::DESU_PRON),
                    rule::MASU_STR => node.set_pron(rule::MASU_PRON),
                    _ => (),
                }
            }
        }
    }
}
