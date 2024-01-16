use crate::limit::Limit;

use super::*;

#[derive(Clone, Debug)]
pub struct BreathGroup {
    pub accent_phrases: Vec<AccentPhrase>,
}

impl BreathGroup {
    pub fn new(accent_phrases: Vec<AccentPhrase>) -> Self {
        Self { accent_phrases }
    }

    pub fn to_h(&self) -> jlabel::BreathGroupPrevNext {
        jlabel::BreathGroupPrevNext {
            accent_phrase_count: Limit::M.ulimit(self.count_accent_phrase()),
            mora_count: Limit::L.ulimit(self.count_mora()),
        }
    }
    pub fn to_i(
        &self,
        breath_group_count_in_utterance: usize,
        breath_group_index_in_utterance: usize,
        accent_phrase_count_in_utterance: usize,
        accent_phrase_index_in_utterance: usize,
        mora_count_in_utterance: usize,
        mora_index_in_utterance: usize,
    ) -> jlabel::BreathGroupCurrent {
        jlabel::BreathGroupCurrent {
            accent_phrase_count: Limit::M.ulimit(self.count_accent_phrase()),
            mora_count: Limit::L.ulimit(self.count_mora()),
            breath_group_position_forward: Limit::S.ulimit(breath_group_index_in_utterance + 1),
            breath_group_position_backward: Limit::S
                .ulimit(breath_group_count_in_utterance - breath_group_index_in_utterance),
            accent_phrase_position_forward: Limit::M.ulimit(accent_phrase_index_in_utterance + 1),
            accent_phrase_position_backward: Limit::M
                .ulimit(accent_phrase_count_in_utterance - accent_phrase_index_in_utterance),
            mora_position_forward: Limit::LL.ulimit(mora_index_in_utterance + 1),
            mora_position_backward: Limit::LL
                .ulimit(mora_count_in_utterance - mora_index_in_utterance),
        }
    }
    pub fn to_j(&self) -> jlabel::BreathGroupPrevNext {
        jlabel::BreathGroupPrevNext {
            accent_phrase_count: Limit::M.ulimit(self.count_accent_phrase()),
            mora_count: Limit::L.ulimit(self.count_mora()),
        }
    }

    pub fn count_accent_phrase(&self) -> usize {
        self.accent_phrases.len()
    }
    pub fn count_mora(&self) -> usize {
        self.accent_phrases.iter().map(|ap| ap.count_mora()).sum()
    }
}
