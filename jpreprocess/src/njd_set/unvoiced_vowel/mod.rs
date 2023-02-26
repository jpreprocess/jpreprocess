use jpreprocess_core::pronounciation::{Mora, MoraEnum};
use phf::Set;

use jpreprocess_core::pos::*;
use jpreprocess_core::*;
use jpreprocess_njd::NJD;

use crate::window::{IterQuintMut, QuadForward};

pub mod rule;

#[derive(Debug)]
struct MoraState<'a> {
    pub mora: &'a mut Mora,
    pub node_index: usize,
    pub pos_group0: Group0,
    pub midx: i32,
    pub atype: i32,
}
impl<'a> MoraState<'a> {
    pub fn from_node(node_index: usize, node: &'a mut NJDNode) -> Vec<Self> {
        let acc = node.get_acc();
        let pos_group0 = node.get_pos().get_group0();
        let pron = node.get_pron_mut();
        pron.0
            .iter_mut()
            .enumerate()
            .map(|(i, mora)| Self {
                mora,
                node_index,
                pos_group0,
                midx: i.try_into().unwrap(),
                atype: acc,
            })
            .collect()
    }
}

pub fn njd_set_unvoiced_vowel(njd: &mut NJD) {
    let mut states: Vec<_> = njd
        .nodes
        .iter_mut()
        .enumerate()
        .map(|(i, node)| MoraState::from_node(i, node))
        .flatten()
        .collect();

    let mut iter = IterQuintMut::new(&mut states);
    while let Some(quint) = iter.next() {
        let (state_curr, mut state_next, mut state_nextnext) = match QuadForward::from(quint) {
            QuadForward::Single(curr) => (curr, None, None),
            QuadForward::Double(curr, next) => (curr, Some(next), None),
            QuadForward::Triple(curr, next, nextnext) => (curr, Some(next), Some(nextnext)),
            QuadForward::Full(curr, next, nextnext, _) => (curr, Some(next), Some(nextnext)),
        };

        /* rule 1: look-ahead for 'masu' and 'desu' */
        if let (Some(state_next), Some(state_nextnext)) =
            (state_next.as_mut(), state_nextnext.as_mut())
        {
            let index_ok = state_curr.node_index == state_next.node_index
                && state_next.node_index != state_nextnext.node_index;
            let pos_ok = matches!(
                state_next.pos_group0,
                Group0::Doushi | Group0::Jodoushi | Group0::Kandoushi
            );
            let mora_ok = matches!(
                (state_curr.mora.mora_enum, state_next.mora.mora_enum),
                (MoraEnum::Ma | MoraEnum::De, MoraEnum::Su)
            );
            if index_ok && pos_ok && mora_ok {
                state_next.mora.is_voiced = Some(match state_nextnext.mora.mora_enum {
                    MoraEnum::Question | MoraEnum::Long => true,
                    _ => false,
                });
            }
        }

        /* rule 2: look-ahead for shi */
        if let Some(state_next) = state_next.as_mut() {
            let is_voiced_ok = matches!(state_curr.mora.is_voiced, None | Some(true))
                && matches!(state_next.mora.is_voiced, None)
                && matches!(
                    state_nextnext.as_ref().and_then(|nn| nn.mora.is_voiced),
                    None | Some(true)
                );
            let pos_ok = matches!(
                state_next.pos_group0,
                Group0::Doushi | Group0::Jodoushi | Group0::Joshi
            );
            let mora_ok = matches!(state_next.mora.mora_enum, MoraEnum::Shi);
            if is_voiced_ok && pos_ok && mora_ok {
                state_next.mora.is_voiced = if state_next.atype == state_next.midx + 1 {
                    /* rule 4 */
                    Some(true)
                } else {
                    /* rule 5 */
                    apply_unvoice_rule(&state_curr.mora, Some(&state_next.mora))
                };
                if matches!(state_next.mora.is_voiced, Some(false)) {
                    state_curr.mora.is_voiced.get_or_insert(true);
                    state_nextnext.map(|nn| nn.mora.is_voiced.get_or_insert(true));
                }
            }
        }

        /* estimate unvoice */
        if state_curr.mora.is_voiced.is_none() {
            state_curr.mora.is_voiced = if matches!(state_curr.pos_group0, Group0::Filler) {
                /* rule 0 */
                Some(true)
            } else if matches!(
                state_next.as_ref().and_then(|n| n.mora.is_voiced),
                Some(false)
            ) {
                /* rule 3 */
                Some(true)
            } else if state_curr.atype == state_curr.midx + 1 {
                /* rule 4 */
                Some(true)
            } else {
                /* rule 5 */
                apply_unvoice_rule(&state_curr.mora, state_next.as_ref().map(|n| &*n.mora))
            };
        }

        if matches!(state_curr.mora.is_voiced, Some(false)) {
            state_next
                .as_mut()
                .map(|n| n.mora.is_voiced.get_or_insert(true));
        }
    }
}

fn apply_unvoice_rule(mora_curr: &Mora, mora_next: Option<&Mora>) -> Option<bool> {
    let Some(mora_next) = mora_next else {
        return Some(true);
    };

    match mora_curr.mora_enum {
        MoraEnum::Swi | MoraEnum::Su => {
            if search_list(&rule::NEXT_MORA_LIST1, &mora_next) {
                return Some(false);
            } else {
                return Some(true);
            }
        }
        MoraEnum::Fi | MoraEnum::Hi | MoraEnum::Fu => {
            if search_list(&rule::NEXT_MORA_LIST2, &mora_next) {
                return Some(false);
            } else {
                return Some(true);
            }
        }
        MoraEnum::Kyu
        | MoraEnum::Shu
        | MoraEnum::Chu
        | MoraEnum::Tsi
        | MoraEnum::Hyu
        | MoraEnum::Pyu
        | MoraEnum::Thu
        | MoraEnum::Twu
        | MoraEnum::Thi
        | MoraEnum::Ki
        | MoraEnum::Ku
        | MoraEnum::Shi
        | MoraEnum::Chi
        | MoraEnum::Tsu
        | MoraEnum::Pi
        | MoraEnum::Pu => {
            if search_list(&rule::NEXT_MORA_LIST3, &mora_next) {
                return Some(false);
            } else {
                return Some(true);
            }
        }
        _ => (),
    }

    None
}

fn search_list(set: &'static Set<&'static str>, key: &Mora) -> bool {
    set.contains(&key.to_string()[0..3])
}
