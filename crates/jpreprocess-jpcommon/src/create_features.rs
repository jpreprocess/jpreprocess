use jpreprocess_njd::NJDNode;
use jpreprocess_window::{Double, IterQuintMut, Triple};

use crate::{
    limit::Limit,
    word_attr::{cform_to_id, ctype_to_id, pos_to_id},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Delimiters {
    breath_group_start: bool,
    breath_group_end: bool,
    accent_phrase_start: bool,
    accent_phrase_end: bool,

    is_interrogative: bool,

    is_pau: bool,
    is_unique_pau: bool,
}

impl Delimiters {
    fn scan_delims(nodes: &[NJDNode]) -> Vec<Self> {
        let mut node_iter = nodes.iter();

        let mut delims = vec![Self::default(); nodes.len()];
        if let Some(first) = delims.first_mut() {
            *first = Delimiters {
                breath_group_start: true,
                breath_group_end: false,
                accent_phrase_start: true,
                accent_phrase_end: false,

                is_interrogative: false,

                is_pau: false,
                is_unique_pau: false,
            };
        }

        let mut delims_iter = IterQuintMut::new(&mut delims);

        while let (Some(node), Some(quint)) = (node_iter.next(), delims_iter.next()) {
            let (mut prev, curr, mut next) = match Triple::from(quint) {
                Triple::Single(curr) => (None, curr, None),
                Triple::First(curr, next) => (None, curr, Some(next)),
                Triple::Last(prev, curr) => (Some(prev), curr, None),
                Triple::Full(prev, curr, next) => (Some(prev), curr, Some(next)),
            };

            // "、" (読点; Touten) and "？" (Question mark) indicate the border of breath groups.
            if node.get_pron().is_touten() || node.get_pron().is_question() {
                curr.is_pau = true;
                curr.is_unique_pau = true;
                curr.is_interrogative = node.get_pron().is_question();

                if curr.breath_group_start {
                    curr.breath_group_start = false;
                    curr.accent_phrase_start = false;
                    curr.is_unique_pau = false;
                } else if let Some(prev) = prev.as_mut() {
                    prev.breath_group_end = true;
                    prev.accent_phrase_end = true;
                }
                if let Some(next) = next.as_mut() {
                    next.breath_group_start = true;
                    next.accent_phrase_start = true;
                }
                continue;
            }

            // Words without `Some(true)` chain flag indicates the end of accent phrase.
            if node.get_chain_flag() != Some(true) {
                curr.accent_phrase_end = true;
                if let Some(next) = next.as_mut() {
                    next.accent_phrase_start = true;
                }
            }

            // Lastly, if the last word is not touten or question mark,
            // it is the end of breath group and accent phrase.
            if next.is_none() {
                curr.breath_group_end = true;
                curr.accent_phrase_end = true;
            }
        }

        delims
    }
}

#[derive(Debug, Clone)]
struct Position {
    mora_in_accent_phrase: usize,
    mora_in_breath_group: usize,
    mora_in_utterance: usize,
    accent_phrase_in_breath_group: usize,
    accent_phrase_in_utterance: usize,
    breath_group_in_utterance: usize,

    mora_size: usize,
}

impl Position {
    fn new() -> Self {
        Self {
            mora_in_accent_phrase: 1,
            mora_in_breath_group: 1,
            mora_in_utterance: 1,
            accent_phrase_in_breath_group: 1,
            accent_phrase_in_utterance: 1,
            breath_group_in_utterance: 1,

            mora_size: 0,
        }
    }
    fn increment_mora(&mut self, mora_size: usize) {
        self.mora_in_accent_phrase += mora_size;
        self.mora_in_breath_group += mora_size;
        self.mora_in_utterance += mora_size;

        self.mora_size = mora_size;
    }
    fn accent_phrase_start(&mut self) {
        self.mora_in_accent_phrase = 1;
    }
    fn accent_phrase_end(&mut self) {
        self.accent_phrase_in_breath_group += 1;
        self.accent_phrase_in_utterance += 1;
    }
    fn breath_group_start(&mut self) {
        self.mora_in_breath_group = 1;
        self.accent_phrase_in_breath_group = 1;
    }
    fn breath_group_end(&mut self) {
        self.breath_group_in_utterance += 1;
    }
}

fn calculate_positions<'a>(
    iter: impl 'a + Iterator<Item = (&'a Delimiters, &'a usize)>,
) -> impl 'a + Iterator<Item = Position> {
    let mut accumulator = Position::new();

    iter.map(move |(delim, &mora_size)| {
        if delim.breath_group_start {
            accumulator.breath_group_start();
        }
        if delim.accent_phrase_start {
            accumulator.accent_phrase_start();
        }

        if !delim.is_pau {
            accumulator.increment_mora(mora_size);
        }

        let pos = accumulator.clone();

        if delim.accent_phrase_end {
            accumulator.accent_phrase_end();
        }
        if delim.breath_group_end {
            accumulator.breath_group_end();
        }

        pos
    })
}

fn words_to_labels_precursor(
    delims: &[Delimiters],
    nodes: &[NJDNode],
    positions_forward: &[Position],
    positions_backward: &[Position],
) -> Vec<jlabel::Label> {
    let mut u = None;
    let mut bg = None;
    let mut ap = None;

    delims
        .iter()
        .zip(nodes)
        .zip(
            positions_forward
                .iter()
                .zip(positions_backward.iter().rev()),
        )
        .filter_map(|((delim, node), (forward, backward))| {
            if u.is_none() {
                u = Some(jlabel::Utterance {
                    breath_group_count: Limit::S.ulimit(backward.breath_group_in_utterance),
                    accent_phrase_count: Limit::M.ulimit(backward.accent_phrase_in_utterance),
                    mora_count: Limit::LL.ulimit(backward.mora_in_utterance),
                })
            }

            if delim.breath_group_start {
                let accent_phrase_count = backward.accent_phrase_in_breath_group;
                let mora_count = backward.mora_in_breath_group + forward.mora_size;

                bg = Some(jlabel::BreathGroupCurrent {
                    accent_phrase_count: Limit::M.ulimit(accent_phrase_count),
                    mora_count: Limit::L.ulimit(mora_count),
                    breath_group_position_forward: Limit::S
                        .ulimit(forward.breath_group_in_utterance),
                    breath_group_position_backward: Limit::S
                        .ulimit(backward.breath_group_in_utterance),
                    accent_phrase_position_forward: Limit::S
                        .ulimit(forward.accent_phrase_in_utterance),
                    accent_phrase_position_backward: Limit::S
                        .ulimit(backward.accent_phrase_in_utterance),
                    mora_position_forward: Limit::S.ulimit(forward.mora_in_utterance),
                    mora_position_backward: Limit::S.ulimit(backward.mora_in_utterance),
                });
            }

            if delim.accent_phrase_start {
                let mora_count = backward.mora_in_accent_phrase + forward.mora_size;

                ap = Some(jlabel::AccentPhraseCurrent {
                    mora_count: Limit::M.ulimit(mora_count),
                    accent_position: Limit::M.ulimit(if node.get_pron().accent() == 0 {
                        mora_count
                    } else {
                        node.get_pron().accent()
                    }),
                    is_interrogative: delim.is_interrogative,
                    accent_phrase_position_forward: Limit::M
                        .ulimit(forward.accent_phrase_in_breath_group),
                    accent_phrase_position_backward: Limit::M
                        .ulimit(backward.accent_phrase_in_breath_group),
                    mora_position_forward: Limit::L.ulimit(forward.mora_in_breath_group),
                    mora_position_backward: Limit::L.ulimit(forward.mora_in_breath_group),
                });
            }

            if delim.is_pau {
                None
            } else {
                let word = jlabel::Word {
                    pos: pos_to_id(node.get_pos()),
                    ctype: ctype_to_id(node.get_ctype()),
                    cform: cform_to_id(node.get_cform()),
                };

                Some(jlabel::Label {
                    phoneme: jlabel::Phoneme {
                        p2: None,
                        p1: None,
                        c: None,
                        n1: None,
                        n2: None,
                    },
                    mora: None,
                    word_prev: None,
                    word_curr: Some(word),
                    word_next: None,
                    accent_phrase_prev: None,
                    accent_phrase_curr: ap.clone(),
                    accent_phrase_next: None,
                    breath_group_prev: None,
                    breath_group_curr: bg.clone(),
                    breath_group_next: None,
                    utterance: u.clone().unwrap(),
                })
            }
        })
        .collect()
}

fn fill_prevnext(labels: &mut [jlabel::Label]) {
    let mut iter = IterQuintMut::new(labels);

    fn ap_curr_to_prevnext(curr: &jlabel::AccentPhraseCurrent) -> jlabel::AccentPhrasePrevNext {
        jlabel::AccentPhrasePrevNext {
            mora_count: curr.mora_count,
            accent_position: curr.accent_position,
            is_interrogative: curr.is_interrogative,
            // To be filled later
            is_pause_insertion: None,
        }
    }
    fn bg_curr_to_prevnext(curr: &jlabel::BreathGroupCurrent) -> jlabel::BreathGroupPrevNext {
        jlabel::BreathGroupPrevNext {
            accent_phrase_count: curr.accent_phrase_count,
            mora_count: curr.mora_count,
        }
    }

    // FIXME: only use prev/next when the current position is start/end
    while let Some(quint) = iter.next() {
        let (prev, curr, next) = match Triple::from(quint) {
            Triple::Single(curr) => (None, curr, None),
            Triple::First(curr, next) => (None, curr, Some(next)),
            Triple::Last(prev, curr) => (Some(prev), curr, None),
            Triple::Full(prev, curr, next) => (Some(prev), curr, Some(next)),
        };

        curr.word_prev = prev.as_ref().and_then(|prev| prev.word_curr.clone());
        curr.accent_phrase_prev = prev
            .as_ref()
            .and_then(|prev| prev.accent_phrase_curr.as_ref())
            .map(ap_curr_to_prevnext);
        curr.breath_group_prev = prev
            .as_ref()
            .and_then(|prev| prev.breath_group_curr.as_ref())
            .map(bg_curr_to_prevnext);

        curr.word_next = next.as_ref().and_then(|next| next.word_curr.clone());
        curr.accent_phrase_next = next
            .as_ref()
            .and_then(|next| next.accent_phrase_curr.as_ref())
            .map(ap_curr_to_prevnext);
        curr.breath_group_next = next
            .as_ref()
            .and_then(|next| next.breath_group_curr.as_ref())
            .map(bg_curr_to_prevnext);
    }
}

fn construct_word_labels(nodes: &[NJDNode]) {
    let word_mora_size = nodes
        .iter()
        .map(|node| node.get_pron().mora_size())
        .collect::<Vec<_>>();

    let delims = Delimiters::scan_delims(nodes);

    // Forward scan
    let positions_forward =
        calculate_positions(delims.iter().zip(&word_mora_size)).collect::<Vec<_>>();
    // Backward scan
    let positions_backward =
        calculate_positions(delims.iter().zip(&word_mora_size).rev()).collect::<Vec<_>>();

    let mut labels = words_to_labels_precursor(&delims, nodes, &positions_forward, &positions_backward);
    fill_prevnext(&mut labels);


}
