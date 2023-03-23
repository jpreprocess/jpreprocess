use jpreprocess_core::{
    error::JPreprocessErrorKind, node_details::NodeDetails, unk::UNK, JPreprocessResult, NJDNode,
};
use jpreprocess_dictionary::{DictionaryTrait, JPreprocessDictionary};
use lindera::Token;

#[derive(Debug)]
pub struct NJD {
    pub nodes: Vec<NJDNode>,
}

impl NJD {
    pub fn remove_silent_node(&mut self) {
        self.nodes.retain(|node| !node.get_pron().is_empty())
    }
    pub fn from_tokens_string(tokens: Vec<Token>) -> Self {
        let mut nodes = Vec::new();
        for mut token in tokens {
            let text = token.text.to_string();
            let mut details_str = token.get_details().unwrap();
            let details = if details_str.len() == 1 && details_str[0] == "UNK" {
                vec![UNK.to_owned()]
            } else {
                details_str.resize(13, "");
                NodeDetails::load(&details_str)
            };
            nodes.extend(NJDNode::load(&text, details));
        }
        Self { nodes }
    }
    pub fn from_tokens_dict(
        tokens: Vec<Token>,
        dict: JPreprocessDictionary,
    ) -> JPreprocessResult<Self> {
        let mut nodes = Vec::new();
        for token in tokens {
            let text = token.text.to_string();
            let details = if !token.word_id.is_unknown() {
                let id =
                    token.word_id.0.try_into().map_err(|e| {
                        JPreprocessErrorKind::DictionaryIndexOutOfRange.with_error(e)
                    })?;
                dict.get(id)
            } else {
                None
            }
            .unwrap_or_else(|| vec![UNK.to_owned()]);

            nodes.extend(NJDNode::load(&text, details));
        }
        Ok(Self { nodes })
    }
}
