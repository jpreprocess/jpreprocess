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
            accent: start_node.get_acc().try_into().unwrap(),
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

    pub fn to_e(&self, is_prev_pause: Option<bool>) -> String {
        let mora_count = self.count_mora();
        format!(
            "/E:{}_{}!{}_xx-{}",
            Limit::M.ulimit(mora_count),
            Limit::M.ulimit(if self.accent == 0 {
                mora_count
            } else {
                self.accent
            }),
            if self.is_interrogative { 1 } else { 0 },
            match is_prev_pause {
                Some(false) => "1",
                Some(true) => "0",
                None => "xx",
            }
        )
    }
    pub fn to_f(
        &self,
        accent_phrase_count_in_breath_group: usize,
        accent_phrase_index_in_breath_group: usize,
        mora_count_in_breath_group: usize,
        mora_index_in_breath_group: usize,
    ) -> String {
        let mora_count = self.count_mora();
        format!(
            "/F:{}_{}#{}_xx@{}_{}|{}_{}",
            Limit::M.ulimit(mora_count),
            Limit::M.ulimit(if self.accent == 0 {
                mora_count
            } else {
                self.accent
            }),
            if self.is_interrogative { 1 } else { 0 },
            Limit::M.ulimit(accent_phrase_index_in_breath_group + 1),
            Limit::M
                .ulimit(accent_phrase_count_in_breath_group - accent_phrase_index_in_breath_group),
            Limit::L.ulimit(mora_index_in_breath_group + 1),
            Limit::L.ulimit(mora_count_in_breath_group - mora_index_in_breath_group),
        )
    }
    pub fn to_g(&self, is_next_pause: Option<bool>) -> String {
        let mora_count = self.count_mora();
        format!(
            "/G:{}_{}%{}_xx_{}",
            Limit::M.ulimit(mora_count),
            Limit::M.ulimit(if self.accent == 0 {
                mora_count
            } else {
                self.accent
            }),
            if self.is_interrogative { 1 } else { 0 },
            match is_next_pause {
                Some(false) => "1",
                Some(true) => "0",
                None => "xx",
            }
        )
    }

    pub fn generate_mora_a(&self) -> Vec<String> {
        let mora_count = self.count_mora();
        let accent = if self.accent == 0 {
            mora_count
        } else {
            self.accent
        };
        (0..mora_count)
            .map(|mora_index| {
                format!(
                    "/A:{}+{}+{}",
                    Limit::M.ilimit(mora_index as isize - accent as isize + 1),
                    Limit::M.ulimit(mora_index + 1),
                    Limit::M.ulimit(mora_count - mora_index)
                )
            })
            .collect()
    }

    pub fn count_mora(&self) -> usize {
        self.words.iter().map(|word| word.count_mora()).sum()
    }
}
