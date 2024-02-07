#[cfg(not(target_family = "wasm"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use jpreprocess::*;
    use std::path::PathBuf;

    let path = std::env::args().nth(1).map(PathBuf::from);
    if !matches!(&path, Some(s) if s.is_dir()) {
        eprintln!("Please specify a valid path to dictionary");
        std::process::exit(-1);
    }

    let config = JPreprocessConfig {
        dictionary: SystemDictionaryConfig::File(path.unwrap()),
        user_dictionary: None,
    };
    let jpreprocess = JPreprocess::from_config(config)?;

    let mut text = String::new();
    while std::io::stdin().read_line(&mut text).is_ok() {
        let jpcommon_label = jpreprocess.extract_fullcontext(&text)?;
        let string_labels: Vec<_> = jpcommon_label.iter().map(ToString::to_string).collect();
        println!("{}", string_labels.join("\n"));
        text.clear();
    }

    Ok(())
}

#[cfg(target_family = "wasm")]
fn main() {}
