use jpreprocess_core::word_entry::WordEntry;

#[test]
#[ignore]
fn test_word_entry_codec() {
    let naist_jdic = std::fs::read_to_string("data/naist-jdic/naist-jdic.csv").unwrap();

    let details: Vec<WordEntry> = naist_jdic
        .lines()
        .map(|s| {
            let mut details_line = s.split(',').skip(4).collect::<Vec<_>>();
            details_line.resize(12, "*"); // Ensure the details line has 12 elements
            WordEntry::load(&details_line).unwrap()
        })
        .collect();

    for entry in details {
        let buf = entry.to_bin().unwrap();
        let mut buf_iter = buf.iter().copied();
        let parsed_entry = WordEntry::from_bin(&mut buf_iter).unwrap();
        assert_eq!(entry, parsed_entry);
    }
}
