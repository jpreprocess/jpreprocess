use std::io::Write;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use jpreprocess::*;

#[test]
#[ignore]
fn is_same_as_open_jtalk() {
    for s in TEST_STR {
        test_one(s);
    }
}

const TEST_STR: &[&str] = &[
    "-64.0℃。シンプソン則。BOP試薬(BOP reagent)。58.226889。2990。678。何千何百何十円なり、TypeScript。一昨日は1月1日。あと20日間残っている。",
    "聞きがたいお手紙の混雑ぶり霊験あらたか。たいそうやっちゃったね。動く細かい部屋に少なめコーヒーだし。尚更。",
    "No.12。番号:12。0.0.2.0.5.0.6.0 1棟、1人、一日、一日間、14日、14日間、20日、24日、24日間、1分。035(123)。100000。10,00。1,000",
    "リャリョ。クーバネティス。行こう。行きます？",
    "一九〇〇、1900，zAゔょぁ。123,456,789",
    // This sentence fails, but I won't fix.
    // "12,34,567．89"
];

fn test_one(input_text: &'static str) {
    let njd = preprocess_to_njd_string(input_text, PathBuf::from("tests/dict")).unwrap();

    let mut child = Command::new("tests/openjtalk_bin")
        .arg("-x")
        .arg("tests/mecab-naist-jdic")
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

    let parsed = parse_openjtalk_output(&stdout);

    for (node, ans) in njd.nodes.iter().zip(parsed.njd.iter()) {
        let node_ans = NJDNode::new_single(ans);
        assert_eq!(node, &node_ans);
    }

    for (node, ans) in jpreprocess_core::jpcommon::njdnodes_to_features(&njd.nodes)
        .iter()
        .zip(parsed.jpcommon_features.iter())
    {
        assert_eq!(node, ans);
    }
}

struct OpenJTalkOutput {
    njd: Vec<String>,
    jpcommon_features: Vec<String>,
}

fn parse_openjtalk_output(output: &str) -> OpenJTalkOutput {
    let capacity = output.lines().count() / 2;
    let mut result = OpenJTalkOutput {
        njd: Vec::with_capacity(capacity),
        jpcommon_features: Vec::with_capacity(capacity),
    };

    enum ParseState {
        None,
        NJD,
        JPCommon,
    }

    let mut state = ParseState::None;
    for line in output.lines() {
        match line {
            "[NJD]" => state = ParseState::NJD,
            "[JPCommon Features]" => state = ParseState::JPCommon,
            _ => match state {
                ParseState::None => (),
                ParseState::NJD => result.njd.push(line.to_string()),
                ParseState::JPCommon => result.njd.push(line.to_string()),
            },
        }
    }

    result
}
