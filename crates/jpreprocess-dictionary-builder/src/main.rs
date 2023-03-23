use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    jpreprocess_dictionary_builder::JPreproessBuilder::generate_dictionary(&PathBuf::from("dict/"))
}
