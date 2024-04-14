/*
  無声子音: k ky s sh t ty ch ts h f hy p py
  Rule 0 フィラーは無声化しない
  Rule 1 助動詞の「です」と「ます」の「す」が無声化
  Rule 2 動詞，助動詞，助詞の「し」は無声化しやすい
  Rule 3 続けて無声化しない
  Rule 4 アクセント核で無声化しない
  Rule 5 無声子音(k ky s sh t ty ch ts h f hy p py)に囲まれた「i」と「u」が無声化
         例外：s->s, s->sh, f->f, f->h, f->hy, h->f, h->h, h->hy
*/

use jpreprocess_core::pronunciation::{
    phoneme::{Consonant, Vowel},
    Mora, MoraEnum,
};

use crate::NJD;
use jpreprocess_core::pos::*;

use jpreprocess_window::{IterQuintMut, QuadForward};

#[derive(Debug)]
struct MoraState<'a> {
    pub mora: &'a mut Mora,
    pub node_index: usize,
    pub pos: POS,
    pub is_voiced_flag: Option<bool>,
    pub midx: usize,
    pub atype: usize,
}

pub fn njd_set_unvoiced_vowel(njd: &mut NJD) {
    let mut states: Vec<MoraState> = Vec::new();

    let mut midx = 0;
    for (node_index, node) in njd.nodes.iter_mut().enumerate() {
        /* reset mora index for new word */
        if matches!(node.get_chain_flag(), None | Some(false)) {
            midx = 0;
        }

        let acc = node.get_pron().accent();
        let pos = node.get_pos().to_owned();
        let pron = node.get_pron_mut();

        for mora in pron.moras_mut() {
            states.push(MoraState {
                is_voiced_flag: if mora.is_voiced { None } else { Some(false) },
                mora,
                node_index,
                pos,
                midx,
                atype: acc,
            });
            midx += 1;
        }
    }

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
                state_next.pos,
                POS::Doushi(_) | POS::Jodoushi | POS::Kandoushi
            );
            let mora_ok = matches!(
                (state_curr.mora.mora_enum, state_next.mora.mora_enum),
                (MoraEnum::Ma | MoraEnum::De, MoraEnum::Su)
            );
            if index_ok && pos_ok && mora_ok {
                state_next.is_voiced_flag = Some(matches!(
                    state_nextnext.mora.mora_enum,
                    MoraEnum::Question | MoraEnum::Long
                ));
            }
        }

        /* rule 2: look-ahead for shi */
        if let Some(state_next) = state_next.as_mut() {
            let is_voiced_ok = matches!(state_curr.is_voiced_flag, None | Some(true))
                && state_next.is_voiced_flag.is_none()
                && matches!(
                    state_nextnext.as_ref().and_then(|nn| nn.is_voiced_flag),
                    None | Some(true)
                );
            let pos_ok = matches!(
                state_next.pos,
                POS::Doushi(_) | POS::Jodoushi | POS::Joshi(_)
            );
            let mora_ok = matches!(state_next.mora.mora_enum, MoraEnum::Shi);
            if is_voiced_ok && pos_ok && mora_ok {
                state_next.is_voiced_flag = if state_next.atype == state_next.midx + 1 {
                    /* rule 4 */
                    Some(true)
                } else {
                    /* rule 5 */
                    apply_unvoice_rule(state_curr.mora, Some(state_next.mora))
                };
                if matches!(state_next.is_voiced_flag, Some(false)) {
                    state_curr.is_voiced_flag.get_or_insert(true);
                    state_nextnext
                        .as_mut()
                        .map(|nn| nn.is_voiced_flag.get_or_insert(true));
                }
            }
        }

        /* estimate unvoice */
        if state_curr.is_voiced_flag.is_none() {
            state_curr.is_voiced_flag = if
            /* rule 0 */
            matches!(state_curr.pos, POS::Filler)  ||
                /* rule 3 */
                matches!(
                    state_next.as_ref().and_then(|n| n.is_voiced_flag),
                    Some(false)
                ) ||
                /* rule 4 */
                state_curr.atype == state_curr.midx + 1
            {
                Some(true)
            } else {
                /* rule 5 */
                apply_unvoice_rule(state_curr.mora, state_next.as_ref().map(|n| &*n.mora))
            };
        }

        if matches!(state_curr.is_voiced_flag, Some(false)) {
            state_next
                .as_mut()
                .map(|n| n.is_voiced_flag.get_or_insert(true));
        }

        state_curr.mora.is_voiced = state_curr.is_voiced_flag.unwrap_or(true);
    }
}

fn apply_unvoice_rule(mora_curr: &Mora, mora_next: Option<&Mora>) -> Option<bool> {
    let Some(mora_next) = mora_next else {
        return Some(true);
    };

    let (curr_consonant, curr_vowel) = mora_curr.phonemes();
    let (next_consonant, _) = mora_next.phonemes();

    if !matches!(
        curr_vowel,
        Some(Vowel::I | Vowel::U | Vowel::IUnvoiced | Vowel::UUnvoiced)
    ) {
        return None;
    }

    Some(match (curr_consonant, next_consonant) {
        (Some(Consonant::S), Some(Consonant::S | Consonant::Sh)) => true,
        (Some(Consonant::F | Consonant::H), Some(Consonant::F | Consonant::H | Consonant::Hy)) => {
            true
        }
        (
            Some(
                Consonant::K
                | Consonant::Ky
                | Consonant::S
                | Consonant::Sh
                | Consonant::T
                | Consonant::Ty
                | Consonant::Ch
                | Consonant::Ts
                | Consonant::H
                | Consonant::F
                | Consonant::Hy
                | Consonant::P
                | Consonant::Py,
            ),
            Some(
                Consonant::K
                | Consonant::Ky
                | Consonant::S
                | Consonant::Sh
                | Consonant::T
                | Consonant::Ty
                | Consonant::Ch
                | Consonant::Ts
                | Consonant::H
                | Consonant::F
                | Consonant::Hy
                | Consonant::P
                | Consonant::Py,
            ),
        ) => false,
        _ => true,
    })
}
