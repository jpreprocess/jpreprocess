pub mod node;
pub mod pos;

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
        for token in tokens {
            let mut details = token.details.unwrap();
            details.resize(13, "".to_string());
            let details_str: Vec<&str> = details.iter().map(|detail| detail.as_str()).collect();
            nodes.extend(NJDNode::load(&token.text, &details_str[..]));
        }
        Self { nodes }
    }
}
