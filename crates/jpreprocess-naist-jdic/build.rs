use std::error::Error;

#[cfg(feature = "naist-jdic")]
fn main() -> Result<(), Box<dyn Error>> {
    use lindera_core::dictionary_builder::DictionaryBuilder;
    use lindera_ipadic_builder::ipadic_builder::IpadicBuilder;
    use std::{env, path::Path};

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    // Directory path for build package
    let build_dir = env::var_os("OUT_DIR").unwrap(); // ex) target/debug/build/<pkg>/out

    // MeCab IPADIC directory
    let input_dir = Path::new("./mecab-naist-jdic");

    // Lindera IPADIC directory
    let output_dir = Path::new(&build_dir).join("naist-jdic");

    // Build a dictionary
    let builder = IpadicBuilder::new();
    builder.build_dictionary(&input_dir, &output_dir)?;

    Ok(())
}

#[cfg(not(feature = "naist-jdic"))]
fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
