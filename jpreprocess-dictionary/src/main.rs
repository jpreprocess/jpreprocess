use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    jpreprocess_dictionary::JPreproessBuilder::generate_dictionary(&PathBuf::from("dict/"))
}   
