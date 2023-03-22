use crate::jpcommon::feature::limit::Limit;

use super::*;

pub struct BreathGroup {
    pub accent_phrases: Vec<AccentPhrase>,
}

impl BreathGroup {
    pub fn new(accent_phrases: Vec<AccentPhrase>) -> Self {
        Self { accent_phrases }
    }

    pub fn to_h(&self) -> String {
        format!(
            "/H:{}_{}",
            Limit::M.ulimit(self.count_accent_phrase()),
            Limit::L.ulimit(self.count_mora())
        )
    }
    pub fn to_i(
        &self,
        breath_group_count_in_utterance: usize,
        breath_group_index_in_utterance: usize,
        accent_phrase_count_in_utterance: usize,
        accent_phrase_index_in_utterance: usize,
        mora_count_in_utterance: usize,
        mora_index_in_utterance: usize,
    ) -> String {
        format!(
            "/I:{}-{}@{}+{}&{}-{}|{}+{}",
            Limit::M.ulimit(self.count_accent_phrase()),
            Limit::L.ulimit(self.count_mora()),
            Limit::S.ulimit(breath_group_index_in_utterance + 1),
            Limit::S.ulimit(breath_group_count_in_utterance - breath_group_index_in_utterance),
            Limit::M.ulimit(accent_phrase_index_in_utterance + 1),
            Limit::M.ulimit(accent_phrase_count_in_utterance - accent_phrase_index_in_utterance),
            Limit::LL.ulimit(mora_index_in_utterance + 1),
            Limit::LL.ulimit(mora_count_in_utterance - mora_index_in_utterance),
        )
    }
    pub fn to_j(&self) -> String {
        format!(
            "/J:{}_{}",
            Limit::M.ulimit(self.count_accent_phrase()),
            Limit::L.ulimit(self.count_mora())
        )
    }

    pub fn count_accent_phrase(&self) -> usize {
        self.accent_phrases.len()
    }
    pub fn count_mora(&self) -> usize {
        self.accent_phrases.iter().map(|ap| ap.count_mora()).sum()
    }
}
