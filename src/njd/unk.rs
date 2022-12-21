pub fn resolve_unk(_text: &str) -> Vec<String> {
    vec![
        "名詞",
        "*",
        "*",
        "*",
        "*",
        "*",
        "*",
        "*",
        "*",
        "0/0",
        "*",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}
