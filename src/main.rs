use std::io::Write;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use lindera::{mode::Mode, tokenizer::*};
use njd::NJD;

mod njd;
mod njd_set;

fn main() {
    let input_text = "あーあ、東京スカイツリーの最寄り駅はとうきょうスカイツリー駅です";

    let dictionary = DictionaryConfig {
        kind: None,
        path: Some(PathBuf::from("dict")),
    };

    let config = TokenizerConfig {
        dictionary,
        user_dictionary: None,
        mode: Mode::Normal,
    };
    let tokenizer = Tokenizer::with_config(config).unwrap();

    let tokens = tokenizer.tokenize_with_details(input_text).unwrap();

    let mut njd = NJD::from_tokens(tokens);
    njd_set::pronounciation::njd_set_pronunciation(&mut njd);
    njd_set::digit::njd_set_digit(&mut njd);
    njd_set::accent_phrase::njd_set_accent_phrase(&mut njd);
    njd_set::accent_type::njd_set_accent_type(&mut njd);
    njd_set::unvoiced_vowel::njd_set_unvoiced_vowel(&mut njd);
    njd_set::long_vowel::njd_set_long_vowel(&mut njd);

    let mut child = Command::new("../open_jtalk/bin/open_jtalk")
        .arg("-x")
        .arg("../open_jtalk/mecab-naist-jdic")
        .arg("-m")
        .arg("../nitech_jp_atr503_m001.htsvoice")
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
        if format!("{:?}", node).as_str() != ans {
            println!("Failed: {:?}--{}", node, ans);
        }
    }
}
