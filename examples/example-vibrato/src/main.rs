#[cfg(not(target_family = "wasm"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = match std::env::args().nth(1).map(std::path::PathBuf::from) {
        Some(s) if s.is_dir() => s,
        _ => {
            eprintln!("Please specify a valid path to original naist-jdic dictionary");
            std::process::exit(-1);
        }
    };

    let mut csvs = Vec::new();
    for entry in std::fs::read_dir(&path)? {
        let Ok(entry) = entry else {
            continue;
        };
        if entry.file_type()?.is_file() && entry.file_name().to_string_lossy().ends_with(".csv") {
            csvs.extend_from_slice(&std::fs::read(entry.path())?);
            if csvs.last() != Some(&b'\n') {
                csvs.push(b'\n');
            }
        }
    }

    let dict = vibrato::SystemDictionaryBuilder::from_readers(
        std::io::Cursor::new(csvs),
        std::fs::File::open(path.join("matrix.def"))?,
        std::fs::File::open(path.join("char.def"))?,
        std::fs::File::open(path.join("unk.def"))?,
    )?;

    let tokenizer = vibrato::Tokenizer::new(dict)
        .ignore_space(true)?
        .max_grouping_len(24);

    let jpreprocess = jpreprocess::JPreprocess::from_tokenizer(tokenizer);
    let njd =
        jpreprocess.run_frontend("日本語文を解析し、音声合成エンジンに渡せる形式に変換します．")?;

    for node in njd.iter() {
        println!("{}", node);
    }

    Ok(())
}

#[cfg(target_family = "wasm")]
fn main() {}
