use std::error::Error;
use std::path::PathBuf;

use jpreprocess::*;

use clap::{Args, Parser};

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

    let config = if let Some(dict) = cli.dict.jpreprocess_dictionary {
        JPreprocessDictionaryConfig::FileJPreprocess(dict)
    } else if let Some(dict) = cli.dict.lindera_dictionary {
        JPreprocessDictionaryConfig::FileLindera(dict)
    } else if cli.dict.bundled {
        #[cfg(not(feature = "naist-jdic"))]
        {
            use clap::{error::ErrorKind, CommandFactory};
            let mut cmd = Cli::command();
            cmd.error(
                ErrorKind::ValueValidation,
                 concat!(
                    "This build of jpreprocess does not have bundled dictionary. ",
                    "Please use --lindera-dictionary or --jpreprocess-dictionary and provide the path to the dictionary."
                )
            ).exit();
        }
        #[cfg(feature = "naist-jdic")]
        JPreprocessDictionaryConfig::Bundled(kind::JPreprocessDictionaryKind::NaistJdic)
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
