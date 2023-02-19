use std::io::Write;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use jpreprocess_core::NJDNode;
use jpreprocess_njd::NJD;
use lindera::{mode::Mode, tokenizer::*};

mod njd_set;
mod text_normalizer;

mod window;

mod test;

fn main() {
    let input_text = "リャリョ。クーバネティス";

    let normalizer = text_normalizer::TextNormalizer::new();
    let normalized_input_text = normalizer.process(input_text);

    #[cfg(feature = "naist-jdic")]
    let tokenizer = Tokenizer::new(
        jpreprocess_naist_jdic::lindera::load_dictionary().unwrap(),
        None,
        Mode::Normal,
    );

    #[cfg(not(feature = "naist-jdic"))]
    let tokenizer = {
        let dictionary = DictionaryConfig {
            kind: None,
            path: Some(PathBuf::from("dict")),
        };

        let config = TokenizerConfig {
            dictionary,
            user_dictionary: None,
            mode: Mode::Normal,
        };
        Tokenizer::from_config(config).unwrap()
    };

    let tokens = tokenizer.tokenize(normalized_input_text.as_str()).unwrap();

    #[cfg(feature = "naist-jdic")]
    let mut njd = NJD::from_tokens_dict(
        tokens,
        jpreprocess_naist_jdic::jpreprocess::load_dictionary(),
    )
    .unwrap();

    #[cfg(not(feature = "naist-jdic"))]
    let mut njd = NJD::from_tokens_string(tokens);

    njd_set::pronounciation::njd_set_pronunciation(&mut njd);
    njd_set::digit::njd_set_digit(&mut njd);
    njd_set::accent_phrase::njd_set_accent_phrase(&mut njd);
    njd_set::accent_type::njd_set_accent_type(&mut njd);
    njd_set::unvoiced_vowel::njd_set_unvoiced_vowel(&mut njd);
    njd_set::long_vowel::njd_set_long_vowel(&mut njd);

    let mut child = Command::new("tester/open_jtalk")
        .arg("-x")
        .arg("tester/mecab-naist-jdic")
        .arg("-m")
        .arg("tester/nitech_jp_atr503_m001.htsvoice")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(input_text.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8(output.stdout).unwrap();
    // println!("{}", stdout);
    for (node, ans) in njd.nodes.iter().zip(stdout.split("\n")) {
        let node_ans = NJDNode::new_single(ans);
        if node != &node_ans {
            println!("Failed: {:?}--{:?}", node, node_ans);
        }
    }
}
