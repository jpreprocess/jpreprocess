#[cfg(not(target_family = "wasm"))]
mod lru_fetcher;

#[cfg(not(target_family = "wasm"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use jpreprocess::*;
    use std::path::PathBuf;

    use crate::lru_fetcher::LruFetcher;

    let path = match std::env::args().nth(1).map(PathBuf::from) {
        Some(s) if s.is_dir() => s,
        _ => {
            eprintln!("Please specify a valid path to dictionary");
            std::process::exit(-1);
        }
    };

    let fetcher = LruFetcher::new(&path)?;
    let dictionary = SystemDictionaryConfig::File(path).load()?;

    let jpreprocess = JPreprocess::with_dictionary_fetcher(fetcher, dictionary, None);

    let mut text = String::new();
    while std::io::stdin().read_line(&mut text).is_ok() {
        let jpcommon_label = jpreprocess.extract_fullcontext(&text)?;
        println!("{}", jpcommon_label.join("\n"))
    }

    Ok(())
}

#[cfg(target_family = "wasm")]
fn main() {}
