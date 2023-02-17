use phf::Set;

use jpreprocess_core::*;
use jpreprocess_core::pos::*;
use jpreprocess_njd::NJD;

pub mod rule;

#[derive(Clone, Copy, Debug)]
enum MoraFlag {
    Unknown,  //-1
    Voice,    // 0
    Unvoiced, // 1
}
#[derive(Debug)]
struct MoraState {
    pub mora: Option<&'static str>,
    pub nlink: Option<usize>,
    pub size: usize,
    pub flag: MoraFlag,
    pub midx: i32,
    pub atype: i32,
}
impl MoraState {
    pub fn get_nlink<'a>(&self, njd: &'a NJD) -> Option<&'a NJDNode> {
        self.nlink.map(|nlink| &njd.nodes[nlink])
    }
}
impl Default for MoraState {
    fn default() -> Self {
        Self {
            mora: None,
            nlink: None,
            size: 0,
            flag: MoraFlag::Unknown,
            midx: 0,
            atype: 0,
        }
    }
}

pub fn njd_set_unvoiced_vowel(njd: &mut NJD) {
    let mut mora_curr = MoraState::default();
    let mut mora_next = MoraState::default();
    let mut mora_nextnext = MoraState::default();

    let mut buff = "".to_string();

    for i in 0..njd.nodes.len() {
        buff.clear();
        let pron_len = match njd.nodes[i].get_pron() {
            Some(s) => s.len(),
            None => 0,
        };
        /* parse pronunciation */
        let mut index = 0;
        while index < pron_len {
            /* get mora information */
            if mora_curr.mora.is_none() {
                get_mora_information(njd, i, index, &mut mora_curr);
            }
            if mora_curr.mora.is_none() {
                eprintln!("WARNING: set_unvoiced_vowel() in unvoiced_vowel/mod.rs: Wrong pron.");
                panic!();
            }
            if mora_next.mora.is_none() {
                mora_next.midx = mora_curr.midx + 1;
                mora_next.atype = mora_curr.atype;
                get_mora_information(njd, i, index + mora_curr.size, &mut mora_next);
            }
            if mora_nextnext.mora.is_none() {
                mora_nextnext.midx = mora_next.midx + 1;
                mora_nextnext.atype = mora_next.atype;
                get_mora_information(
                    njd,
                    i,
                    index + mora_curr.size + mora_next.size,
                    &mut mora_nextnext,
                );
            }

            /* rule 1: look-ahead for 'masu' and 'desu' */
            if mora_next.mora.is_some()
                && mora_nextnext.mora.is_some()
                && mora_curr.nlink == mora_next.nlink
                && mora_next.nlink != mora_nextnext.nlink
            {
                let ng0 = mora_next
                    .get_nlink(njd)
                    .map(|node| node.get_pos().get_group0());
                match (mora_curr.mora, mora_next.mora, ng0) {
                    (
                        Some(rule::MA | rule::DE),
                        Some(rule::SU),
                        Some(Group0::Doushi | Group0::Jodoushi | Group0::Kandoushi),
                    ) => {
                        let nnpron = mora_nextnext
                            .get_nlink(njd)
                            .and_then(|node| node.get_pron());
                        mora_next.flag = match nnpron {
                            Some(rule::QUESTION | rule::CHOUON) => MoraFlag::Voice,
                            _ => MoraFlag::Unvoiced,
                        }
                    }
                    _ => (),
                }
            }

            /* rule 2: look-ahead for 'shi' */
            if matches!(mora_curr.flag, MoraFlag::Unknown | MoraFlag::Voice)
                && matches!(mora_next.flag, MoraFlag::Unknown)
                && matches!(mora_nextnext.flag, MoraFlag::Unknown | MoraFlag::Voice)
            {
                if let Some(nlink_next) = mora_next.get_nlink(njd) {
                    match (nlink_next.get_pos().get_group0(), nlink_next.get_pron()) {
                        (Group0::Doushi | Group0::Jodoushi | Group0::Joshi, Some(rule::SHI)) => {
                            mora_next.flag = if mora_next.atype == mora_next.midx + 1 {
                                /* rule 4 */
                                MoraFlag::Voice
                            } else {
                                /* rule 5 */
                                apply_unvoice_rule(&mora_next, &mora_nextnext)
                            };
                            if matches!(mora_next.flag, MoraFlag::Unvoiced) {
                                if matches!(mora_curr.flag, MoraFlag::Unknown) {
                                    mora_curr.flag = MoraFlag::Voice;
                                }
                                if matches!(mora_nextnext.flag, MoraFlag::Unknown) {
                                    mora_nextnext.flag = MoraFlag::Voice;
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }

            /* estimate unvoice */
            if matches!(mora_curr.flag, MoraFlag::Unknown) {
                mora_curr.flag = if matches!(
                    mora_curr
                        .get_nlink(njd)
                        .map(|node| node.get_pos().get_group0()),
                    Some(Group0::Filler)
                ) {
                    /* rule 0 */
                    MoraFlag::Voice
                } else if matches!(mora_next.flag, MoraFlag::Unvoiced) {
                    /* rule 3 */
                    MoraFlag::Voice
                } else if mora_curr.atype == mora_curr.midx + 1 {
                    /* rule 4 */
                    MoraFlag::Voice
                } else {
                    /* rule 5 */
                    apply_unvoice_rule(&mora_curr, &mora_next)
                }
            }
            match (mora_curr.flag, mora_next.flag) {
                (MoraFlag::Unvoiced, MoraFlag::Unknown) => {
                    mora_next.flag = MoraFlag::Voice;
                }
                _ => (),
            }

            /* store pronunciation */
            if let Some(mora) = mora_curr.mora {
                buff.push_str(mora);
            }
            if matches!(mora_curr.flag, MoraFlag::Unvoiced) {
                buff.push_str(rule::QUOTATION);
            }

            /* prepare next step */
            index += mora_curr.size;

            mora_curr = mora_next;
            mora_next = mora_nextnext;
            mora_nextnext = MoraState::default();
        }
        njd.nodes[i].set_pron(buff.as_str());
    }
}

fn get_mora_information(njd: &NJD, node_index: usize, index: usize, state: &mut MoraState) {
    let pron_len = match njd.nodes[node_index].get_pron() {
        Some(s) => s.len(),
        None => 0,
    };
    if index >= pron_len {
        if node_index < njd.nodes.len() - 1 {
            get_mora_information(njd, node_index + 1, index - pron_len, state);
        } else {
            *state = MoraState::default();
        }
        return;
    }

    let node = &njd.nodes[node_index];

    state.nlink = Some(node_index);

    /* reset mora index and accent type for new word */
    if index == 0 && matches!(node.get_chain_flag(), None | Some(false)) {
        state.midx = 0;
        state.atype = node.get_acc();
    }

    let pron = node.get_pron().unwrap();

    /* special symbol */
    match pron {
        rule::TOUTEN => {
            state.mora = Some(rule::TOUTEN);
            state.flag = MoraFlag::Voice;
            state.size = rule::TOUTEN.len();
            return;
        }
        rule::QUESTION => {
            state.mora = Some(rule::QUESTION);
            state.flag = MoraFlag::Voice;
            state.size = rule::QUESTION.len();
            return;
        }
        _ => (),
    }

    /* reset */
    state.mora = None;
    state.flag = MoraFlag::Unknown;
    state.size = 0;

    /* get mora */
    if let Some(mora) = search_list(&rule::MORA_LIST, &pron[index..pron.len()]) {
        state.mora = Some(mora.to_owned());
        state.size = mora.len();
    }

    /* get unvoice flag */
    if pron[index + state.size..pron.len()].starts_with(rule::QUOTATION) {
        state.flag = MoraFlag::Unvoiced;
        state.size += rule::QUOTATION.len();
    }
}

fn apply_unvoice_rule(state_curr: &MoraState, state_next: &MoraState) -> MoraFlag {
    let next_mora_opt = state_next.mora;

    if next_mora_opt.is_none() {
        return MoraFlag::Voice;
    }

    let curr_mora = state_curr.mora.unwrap();
    let next_mora = next_mora_opt.unwrap();

    if rule::CANDIDATE_LIST1.contains(curr_mora) {
        if search_list(&rule::NEXT_MORA_LIST1, next_mora).is_some() {
            return MoraFlag::Unvoiced;
        } else {
            return MoraFlag::Voice;
        }
    }

    if rule::CANDIDATE_LIST2.contains(curr_mora) {
        if search_list(&rule::NEXT_MORA_LIST2, next_mora).is_some() {
            return MoraFlag::Unvoiced;
        } else {
            return MoraFlag::Voice;
        }
    }

    if rule::CANDIDATE_LIST3.contains(curr_mora) {
        if search_list(&rule::NEXT_MORA_LIST3, next_mora).is_some() {
            return MoraFlag::Unvoiced;
        } else {
            return MoraFlag::Voice;
        }
    }

    MoraFlag::Unknown
}

fn search_list(set: &'static Set<&'static str>, key: &str) -> Option<&'static &'static str> {
    let mut chars = key.chars();
    let l1 = chars.next();
    let l2 = chars.next();
    match (l1, l2) {
        (Some(l1u), Some(l2u)) => {
            let s2: String = [l1u, l2u].iter().collect();
            set.get_key(&s2).or_else(|| set.get_key(&l1u.to_string()))
        }
        (Some(l1u), None) => set.get_key(&l1u.to_string()),
        _ => None,
    }
}
