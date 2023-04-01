use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use once_cell::sync::Lazy;

mod rule;

static AHO_CORASICK: Lazy<AhoCorasick> = Lazy::new(|| {
    let from: Vec<&&str> = rule::CONVERSION_TABLE
        .iter()
        .map(|(from, _to)| from)
        .collect();
    AhoCorasickBuilder::new().build(from)
});

static REPLACE_TO: Lazy<Vec<&'static &'static str>> = Lazy::new(|| {
    rule::CONVERSION_TABLE
        .iter()
        .map(|(_from, to)| to)
        .collect()
});

pub fn normalize(input: &str) -> String {
    AHO_CORASICK.replace_all(input, REPLACE_TO.as_slice())
}
