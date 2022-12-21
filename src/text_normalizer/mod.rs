use aho_corasick::{AhoCorasick, AhoCorasickBuilder};

mod rule;

pub struct TextNormalizer {
    aho_corasick: AhoCorasick,
    replace_with: Vec<&'static &'static str>,
}

impl TextNormalizer {
    pub fn new() -> Self {
        let from: Vec<&&str> = rule::CONVERSION_TABLE.iter().map(|(from, _to)| from).collect();
        let to: Vec<&'static &'static str> = rule::CONVERSION_TABLE.iter().map(|(_from, to)| to).collect();
        Self {
            aho_corasick: AhoCorasickBuilder::new().build(from),
            replace_with: to,
        }
    }
    pub fn process(&self, input: &str) -> String {
        self.aho_corasick
            .replace_all(input, self.replace_with.as_slice())
    }
}
