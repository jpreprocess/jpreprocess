use std::error::Error;
use std::path::PathBuf;

use jpreprocess::*;

use clap::{Args, Parser};
use lindera_dictionary::UserDictionaryConfig;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    dict: DictionaryArgs,

    /// The location of the user dictionary
    #[arg(short, long)]
    user_dictionary: Option<PathBuf>,

    /// The text to be processed
    input: String,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct DictionaryArgs {
    /// The location of the system dictionary
    #[arg(short, long)]
    dictionary: Option<PathBuf>,

    /// Use bundled naist-jdic dictionary
    #[cfg(feature = "naist-jdic")]
    #[arg(short, long)]
    naist_jdic: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let dictionary = if let Some(dict) = cli.dict.dictionary {
        SystemDictionaryConfig::File(dict)
    } else {
        #[cfg(not(feature = "naist-jdic"))]
        unreachable!("This build of jpreprocess does not have the bundled dictionary, and it is not supporsed to reach here.");
        #[cfg(feature = "naist-jdic")]
        SystemDictionaryConfig::Bundled(kind::JPreprocessDictionaryKind::NaistJdic)
    };

    let user_dictionary = cli.user_dictionary.map(|user_dict| UserDictionaryConfig {
        path: user_dict,
        kind: Some(lindera_dictionary::DictionaryKind::IPADIC),
    });

    let jpreprocess = JPreprocess::from_config(JPreprocessConfig {
        dictionary,
        user_dictionary,
    })?;

    let njd_texts: Vec<String> = jpreprocess.text_to_njd(&cli.input)?.into();
    for line in njd_texts {
        println!("{}", line);
    }

    let njd = jpreprocess.run_frontend(&cli.input)?;

    println!("[NJD]");
    for line in &njd {
        println!("{}", line);
    }

    println!("\n[JPCommon]");
    for line in jpreprocess.make_label(njd) {
        println!("{}", line);
    }

    Ok(())
}
