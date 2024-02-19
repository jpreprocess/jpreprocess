use jpreprocess_njd::NJDNode;

use crate::limit::Limit;

use super::*;

#[derive(Clone, Debug)]
pub struct AccentPhrase {
    accent: usize,
    is_interrogative: bool,
    pub words: Vec<Word>,
}

impl AccentPhrase {
    pub fn new(start_node: &NJDNode) -> Self {
        Self {
            accent: start_node.get_pron().accent(),
            is_interrogative: false,
            words: vec![start_node.into()],
        }
    }
    pub(super) fn push_node(&mut self, node: &NJDNode) {
        if !matches!(node.get_chain_flag(), Some(true)) {
            panic!("push_node of AccentPhrase should not be called unless chain flag is true");
        }
        self.words.push(node.into());
    }
    pub(super) fn set_interrogative(&mut self) {
        self.is_interrogative = true;
    }

    pub fn to_e(&self, is_prev_pause: Option<bool>) -> jlabel::AccentPhrasePrevNext {
        let mora_count = self.count_mora();
        jlabel::AccentPhrasePrevNext {
            mora_count: Limit::M.ulimit(mora_count),
            accent_position: Limit::M.ulimit(if self.accent == 0 {
                mora_count
            } else {
                self.accent
            }),
            is_interrogative: self.is_interrogative,
            is_pause_insertion: is_prev_pause,
        }
    }
    pub fn to_f(
        &self,
        accent_phrase_count_in_breath_group: usize,
        accent_phrase_index_in_breath_group: usize,
        mora_count_in_breath_group: usize,
        mora_index_in_breath_group: usize,
    ) -> jlabel::AccentPhraseCurrent {
        let mora_count = self.count_mora();
        jlabel::AccentPhraseCurrent {
            mora_count: Limit::M.ulimit(mora_count),
            accent_position: Limit::M.ulimit(if self.accent == 0 {
                mora_count
            } else {
                self.accent
            }),
            is_interrogative: self.is_interrogative,
            accent_phrase_position_forward: Limit::M
                .ulimit(accent_phrase_index_in_breath_group + 1),
            accent_phrase_position_backward: Limit::M
                .ulimit(accent_phrase_count_in_breath_group - accent_phrase_index_in_breath_group),
            mora_position_forward: Limit::L.ulimit(mora_index_in_breath_group + 1),
            mora_position_backward: Limit::L
                .ulimit(mora_count_in_breath_group - mora_index_in_breath_group),
        }
    }
    pub fn to_g(&self, is_next_pause: Option<bool>) -> jlabel::AccentPhrasePrevNext {
        let mora_count = self.count_mora();
        jlabel::AccentPhrasePrevNext {
            mora_count: Limit::M.ulimit(mora_count),
            accent_position: Limit::M.ulimit(if self.accent == 0 {
                mora_count
            } else {
                self.accent
            }),
            is_interrogative: self.is_interrogative,
            is_pause_insertion: is_next_pause,
        }
    }

    pub fn generate_mora_a(&self) -> Vec<jlabel::Mora> {
        let mora_count = self.count_mora();
        let accent = if self.accent == 0 {
            mora_count
        } else {
            self.accent
        };
        (0..mora_count)
            .map(|mora_index| jlabel::Mora {
                relative_accent_position: Limit::M
                    .ilimit(mora_index as isize - accent as isize + 1),
                position_forward: Limit::M.ulimit(mora_index + 1),
                position_backward: Limit::M.ulimit(mora_count - mora_index),
            })
            .collect()
    }

    pub fn count_mora(&self) -> usize {
        self.words.iter().map(|word| word.count_mora()).sum()
    }
}
