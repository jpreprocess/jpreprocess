use crate::{jpcommon::feature::limit::Limit, NJDNode};

use super::*;

pub struct Utterance {
    pub breath_groups: Vec<BreathGroup>,
}

impl Utterance {
    pub fn to_k(&self) -> String {
        format!(
            "/K:{}+{}-{}",
            Limit::S.ulimit(self.breath_groups.len()),
            Limit::M.ulimit(self.count_accent_phrase()),
            Limit::LL.ulimit(self.count_mora())
        )
    }

    pub fn count_accent_phrase(&self) -> usize {
        self.breath_groups
            .iter()
            .map(|bg| bg.count_accent_phrase())
            .sum()
    }
    pub fn count_mora(&self) -> usize {
        self.breath_groups.iter().map(|bg| bg.count_mora()).sum()
    }
}

impl From<&[NJDNode]> for Utterance {
    fn from(nodes: &[NJDNode]) -> Self {
        let mut breath_groups: Vec<BreathGroup> = Vec::new();
        let mut accent_phrases: Vec<AccentPhrase> = Vec::with_capacity(nodes.len());

        for node in nodes {
            if node.get_pron().is_question() {
                if let Some(accent_phrase) = accent_phrases.last_mut() {
                    accent_phrase.set_interrogative();
                } else {
                    eprintln!("WARN: First mora should not be question flag.");
                }
            }
            if node.get_pron().is_touten() || node.get_pron().is_question() {
                breath_groups.push(BreathGroup::new(accent_phrases));
                accent_phrases = Vec::new();
                continue;
            }

            if matches!(node.get_chain_flag(), Some(true)) {
                if let Some(accent_phrase) = accent_phrases.last_mut() {
                    accent_phrase.push_node(node);
                } else {
                    eprintln!("WARN: First mora cannot be chained.");
                }
            } else {
                accent_phrases.push(AccentPhrase::new(node));
            }
        }
        if !accent_phrases.is_empty() {
            breath_groups.push(BreathGroup::new(accent_phrases));
        }

        Self { breath_groups }
    }
}
