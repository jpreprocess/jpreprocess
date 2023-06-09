use std::error::Error;

use jpreprocess::*;
#[cfg(not(feature = "naist-jdic"))]
use std::path::PathBuf;

use clap::{Args, Parser};

// #[cfg(feature = "binary")]

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    dict: DictionaryArgs,

    /// The text to be processed
    input: String,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct DictionaryArgs {
    /// The location of lindera dictionary
    #[arg(short, long)]
    lindera_dictionary: Option<PathBuf>,

    /// The location of jpreprocess dictionary
    #[arg(short, long)]
    jpreprocess_dictionary: Option<PathBuf>,

    /// Use bundled dictionary
    #[arg(short, long)]
    bundled: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let config = if cli.dict.bundled {
        #[cfg(not(feature = "naist-jdic"))]
        panic!("This build of jpreprocess does not contain dictionary. Instead, please specify the path to the dictionary.");
        #[cfg(feature = "naist-jdic")]
        JPreprocessDictionaryConfig::Bundled(JPreprocessDictionaryKind::NaistJdic)
    } else if let Some(dict) = cli.dict.jpreprocess_dictionary {
        JPreprocessDictionaryConfig::FileJPreprocess(dict)
    } else if let Some(dict) = cli.dict.lindera_dictionary {
        JPreprocessDictionaryConfig::FileLindera(dict)
    } else {
        unreachable!()
    };

    let jpreprocess = JPreprocess::new(config)?;

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
