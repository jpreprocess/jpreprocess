pub mod accent_rule;
pub mod node;
mod node_details;
pub mod pos;
mod unk;

use lindera::Token;
pub use node::*;

#[derive(Debug)]
pub struct NJD {
    pub nodes: Vec<NJDNode>,
}

impl NJD {
    pub fn remove_silent_node(&mut self) {
        self.nodes.retain(|node| node.get_pron().is_some())
    }
    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        let mut nodes = Vec::new();
        for mut token in tokens {
            let text = token.get_text().to_string();
            let mut details = token.get_details().unwrap();
            if details.len() == 1 && details[0] == "UNK" {
                details = unk::resolve_unk();
            }
            details.resize(13, "");
            nodes.extend(NJDNode::load(&text, &details[..]));
        }
        Self { nodes }
    }
}
