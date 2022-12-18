use std::path::PathBuf;

use lindera::{mode::Mode, tokenizer::*};
use njd::NJD;

mod njd;
mod njd_set;

fn main() {
    let dictionary = DictionaryConfig {
        kind: None, //Some(DictionaryKind::IPADIC),
        path: Some(PathBuf::from(
            "dict/lindera-ipadic",
        )),
    };
    // let user_dictionary = UserDictionaryConfig {
    //     kind: Some(DictionaryKind::IPADIC),
    //     path: PathBuf::from("dict/naist-jdic.csv"),
    // };

    // create tokenizer
    let config = TokenizerConfig {
        dictionary,
        user_dictionary: None, //Some(user_dictionary),
        mode: Mode::Normal,
    };
    let tokenizer = Tokenizer::with_config(config).unwrap();

    // tokenize the text
    let tokens = tokenizer
        .tokenize_with_details("あーあ、東京スカイツリーの最寄り駅はとうきょうスカイツリー駅です")
        .unwrap();

    // output the tokens
    for token in tokens.iter() {
        println!("{}{:?}", token.text, token.details);
    }

    let mut njd = NJD::from_tokens(tokens);
    njd_set::pronounciation::njd_set_pronunciation(&mut njd);
    njd_set::digit::njd_set_digit(&mut njd);
    njd_set::accent_phrase::njd_set_accent_phrase(&mut njd);
    njd_set::accent_type::njd_set_accent_type(&mut njd);
    njd_set::unvoiced_vowel::njd_set_unvoiced_vowel(&mut njd);
    njd_set::long_vowel::njd_set_long_vowel(&mut njd);
    dbg!(njd);
}
