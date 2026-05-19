#![feature(test)]

extern crate test;
use jpreprocess_core::word_entry::WordEntry;
use test::Bencher;

#[bench]
fn bench_deserialize(b: &mut Bencher) {
    // Read NAIST jdic
    let dictionary = std::fs::read_to_string("data/naist-jdic/naist-jdic.csv").unwrap();
    let serialized_entries = dictionary
        .lines()
        .map(|line| {
            let mut details = line.split(',').skip(4).collect::<Vec<_>>();
            details.resize(12, "");
            let entry = WordEntry::load(&details).unwrap();
            bincode::serde::encode_to_vec(&entry, bincode::config::standard()).unwrap()
        })
        .collect::<Vec<_>>();

    b.iter(|| {
        for serialized in &serialized_entries {
            let deserialized: WordEntry =
                bincode::serde::decode_from_slice(serialized, bincode::config::standard())
                    .unwrap()
                    .0;
        }
    });
}

#[bench]
fn bench_decode(b: &mut Bencher) {
    // Read NAIST jdic
    let dictionary = std::fs::read_to_string("data/naist-jdic/naist-jdic.csv").unwrap();
    let serialized_entries = dictionary
        .lines()
        .map(|line| {
            let mut details = line.split(',').skip(4).collect::<Vec<_>>();
            details.resize(12, "");
            let entry = WordEntry::load(&details).unwrap();
            bitcode::encode(&entry)
        })
        .collect::<Vec<_>>();

    b.iter(|| {
        for serialized in &serialized_entries {
            let deserialized: WordEntry = bitcode::decode(serialized).unwrap();
        }
    });
}
