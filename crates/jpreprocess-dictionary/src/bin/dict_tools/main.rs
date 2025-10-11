use std::{error::Error, fs::File, io::Write, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};
use jpreprocess_dictionary::dictionary::{
    to_csv::dict_to_csv,
    to_dict::JPreprocessDictionaryBuilder,
    word_encoding::{
        JPreprocessDictionaryWordEncoding, LinderaSystemDictionaryWordEncoding,
        LinderaUserDictionaryWordEncoding,
    },
};
use lindera::dictionary::{load_fs_dictionary, load_user_dictionary_from_bin};
use lindera_dictionary::{builder::DictionaryBuilder, dictionary::metadata::Metadata};

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
                    let dict = load_fs_dictionary(&input)?;
                    QueryDict::System(dict)
                } else {
                    println!("Lindera/JPreprocess user dictionary.");
                    if input.extension().unwrap() != "bin" {
                        eprintln!("User dictionary must be a `.bin` file.");
                        std::process::exit(-1);
                    }

                    let dict = load_user_dictionary_from_bin(&input)?;

                    QueryDict::User(dict)
                };

                let serializer = if let Some(identifier) = dict.identifier() {
                    println!("Dictionary identifier: {}", identifier);
                    if identifier.starts_with("jpreprocess") {
                        Serializer::Jpreprocess
                    } else {
                        Serializer::Lindera
                    }
                } else {
                    println!("No identifier found. Assuming lindera dictionary.");
                    Serializer::Lindera
                };

                if let Some(word_id) = word_id {
                    match serializer {
                        Serializer::Lindera => {
                            let Some(word_details) = dict.get_as_lindera(word_id) else {
                                eprintln!("Word not found");
                                std::process::exit(-1);
                            };
                            for detail in word_details {
                                println!("{}", detail);
                            }
                        }
                        Serializer::Jpreprocess => {
                            let Some(word_details) = dict.get_as_jpreprocess(word_id) else {
                                eprintln!("Word not found");
                                std::process::exit(-1);
                            };
                            println!("{}", word_details.to_str_vec("".to_owned()).join(","));
                        }
                    }
                }
            }
        }
        Commands::Build {
            user,
            serializer: serializer_config,
            input,
            output,
        } => {
            println!("Building dictionary...");
            match serializer_config {
                Serializer::Lindera => {
                    let builder = DictionaryBuilder::new(Metadata {
                        name: "IPADIC".to_string(),
                        encoding: "UTF-8".to_string(),
                        compress_algorithm: lindera_dictionary::decompress::Algorithm::Raw,
                        ..Default::default()
                    });

                    if user {
                        builder.build_user_dictionary(&input, &output)?;
                    } else {
                        builder.build_dictionary(&input, &output)?;
                    }
                }
                Serializer::Jpreprocess => {
                    let builder = JPreprocessDictionaryBuilder::default();

                    if user {
                        builder.build_user_dictionary(&input, &output)?;
                    } else {
                        builder.build_dictionary(&input, &output)?;
                    }
                }
            }
            println!("done.");
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
                let dict = load_fs_dictionary(&input)?;
                QueryDict::System(dict)
            } else {
                if input.extension().unwrap() != "bin" {
                    eprintln!("User dictionary must be a `.bin` file.");
                    std::process::exit(-1);
                }

                let dict = load_user_dictionary_from_bin(&input)?;
                QueryDict::User(dict)
            };
            println!("Successfully loaded source dictionary.");

            let prefix_dict = dict.dictionary_data();

            println!("Converting dictionary csv...");
            let csv = match serializer_config {
                Serializer::Lindera => match dict {
                    QueryDict::System(_) => {
                        dict_to_csv::<LinderaSystemDictionaryWordEncoding>(&prefix_dict)?
                    }
                    QueryDict::User(_) => {
                        dict_to_csv::<LinderaUserDictionaryWordEncoding>(&prefix_dict)?
                    }
                },
                Serializer::Jpreprocess => {
                    dict_to_csv::<JPreprocessDictionaryWordEncoding>(&prefix_dict)?
                }
            };
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
