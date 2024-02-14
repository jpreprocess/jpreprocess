use std::{error::Error, fs::File, io::Write, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};

use jpreprocess::SystemDictionaryConfig;
use jpreprocess_dictionary::{default::WordDictionaryMode, DictionaryStore};
use jpreprocess_dictionary_builder::{ipadic_builder::IpadicBuilder, to_csv::dict_to_csv};

use lindera_core::dictionary_builder::DictionaryBuilder;
use lindera_dictionary::{load_user_dictionary, UserDictionaryConfig};

use crate::dict_query::QueryDict;

mod dict_query;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Display detailed information on a dictionary
    Inspect {
        /// The Word id to display
        #[arg(short, long)]
        word_id: Option<u32>,

        input: PathBuf,
    },
    /// Build a dictionary for lindera or jpreprocess
    Build {
        /// User dictionary
        #[arg(short, long)]
        user: bool,
        /// The serlializer to be used
        #[arg(value_enum)]
        serializer: Serializer,

        input: PathBuf,
        /// The directory(system dictionary) or file(user dictionary) to put the dictionary.
        /// For user dictionary, the parent directory of the output file should not exist.
        output: PathBuf,
    },
    /// Restore the csv file used for building the dictionary
    Csv {
        /// User dictionary
        #[arg(short, long)]
        user: bool,
        /// The serlializer to be used
        #[arg(value_enum)]
        serializer: Serializer,

        /// The directory(system dictionary) or file(user dictionary) to the dictionary.
        /// For user dictionary, the parent directory of the output file should not exist.
        input: PathBuf,
        /// The path to the output csv file
        output: PathBuf,
    },
}

#[derive(Clone, ValueEnum, Debug)]
enum Serializer {
    /// Build lindera dictionary
    Lindera,
    /// Build jpreprocess dictionary
    Jpreprocess,
}
impl Serializer {
    pub fn into_mode(self) -> WordDictionaryMode {
        match self {
            Self::Lindera => WordDictionaryMode::Lindera,
            Self::Jpreprocess => WordDictionaryMode::JPreprocess,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Inspect { word_id, input } => {
            let is_system_dict = input.is_dir()
                && input.join("dict.wordsidx").exists()
                && input.join("dict.words").exists();
            let is_user_bin_dict =
                input.is_file() && matches!(input.extension(),Some(s) if s.to_str()==Some("bin"));

            if is_system_dict || is_user_bin_dict {
                let dict = if is_system_dict {
                    println!("Lindera/JPreprocess system dictionary.");
                    let dict = SystemDictionaryConfig::File(input).load()?;
                    QueryDict::System(dict)
                } else {
                    println!("Lindera/JPreprocess user dictionary.");
                    let dict = load_user_dictionary(UserDictionaryConfig {
                        path: input,
                        kind: None,
                    })?;
                    QueryDict::User(dict)
                };

                if let Some(metadata) = dict.identifier() {
                    println!("Dictionary metadata: {}", metadata);
                } else {
                    println!("No metadata found. Assuming lindera dictionary.")
                }

                if let Some(word_id) = word_id {
                    let word_bin = match dict.get_bytes(word_id) {
                        Ok(word_bin) => word_bin,
                        Err(err) => {
                            eprintln!("Error: {:?}", err);
                            std::process::exit(-1);
                        }
                    };
                    let message = dict.mode().into_serializer().deserialize_debug(word_bin);
                    println!("{}", message);
                }
            }
        }
        Commands::Build {
            user,
            serializer: serializer_config,
            input,
            output,
        } => {
            let builder = IpadicBuilder::new(serializer_config.into_mode().into_serializer());

            if user {
                println!("Building user dictionary...");
                builder.build_user_dictionary(&input, &output)?;
                println!("done.");
            } else {
                println!("Building system dictionary...");
                builder.build_dictionary(&input, &output)?;
                println!("done.");
            }
        }
        Commands::Csv {
            user,
            serializer: serializer_config,
            input,
            output,
        } => {
            if output.exists() {
                eprintln!("The output directory {:?} already exists!", output);
                std::process::exit(-1);
            } else if !matches!(output.extension(),Some(s) if s.to_str()==Some("csv")) {
                eprintln!("The output file extension must be csv.");
                std::process::exit(-1);
            }

            println!("Loading dictionary...");
            let dict = if !user {
                let dict = SystemDictionaryConfig::File(input).load()?;
                QueryDict::System(dict)
            } else {
                let dict = load_user_dictionary(UserDictionaryConfig {
                    path: input,
                    kind: None,
                })?;
                QueryDict::User(dict)
            };
            println!("Successfully loaded source dictionary.");

            let (prefix_dict, words_idx_data, words_data) = dict.dictionary_data();

            println!("Converting dictionary csv...");
            let csv = dict_to_csv(
                prefix_dict,
                words_idx_data,
                words_data,
                &serializer_config.into_mode().into_serializer(),
            )?;
            println!("done.");

            println!("Writing csv file...");
            let mut file = File::create(output)?;
            file.write_all(csv.join("\n").as_bytes())?;
            file.flush()?;
            println!("done.");
        }
    }

    Ok(())
}
