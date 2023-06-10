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

    /// Use bundled naist-jdic dictionary
    #[arg(short, long)]
    naist_jdic: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let config = if let Some(dict) = cli.dict.jpreprocess_dictionary {
        JPreprocessDictionaryConfig::FileJPreprocess(dict)
    } else if let Some(dict) = cli.dict.lindera_dictionary {
        JPreprocessDictionaryConfig::FileLindera(dict)
    } else {
        let args_error = || {
            use clap::{error::ErrorKind, CommandFactory};
            let mut cmd = Cli::command();
            cmd.error(
                ErrorKind::ValueValidation,
                concat!(
                    "This build of jpreprocess does not have the bundled dictionary. ",
                    "Please use other dictionary type."
                ),
            )
            .exit()
        };
        if cli.dict.naist_jdic {
            #[cfg(not(feature = "naist-jdic"))]
            {
                args_error()
            }
            #[cfg(feature = "naist-jdic")]
            JPreprocessDictionaryConfig::Bundled(kind::JPreprocessDictionaryKind::NaistJdic)
        } else {
            args_error()
        }
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
