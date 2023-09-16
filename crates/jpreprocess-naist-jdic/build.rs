use std::error::Error;

#[cfg(feature = "naist-jdic")]
fn main() -> Result<(), Box<dyn Error>> {
    use std::{
        env,
        fs::{copy, create_dir, rename, File},
        io::{self, Cursor, Read, Write},
        path::Path,
    };

    use jpreprocess_dictionary::serializer::jpreprocess::JPreprocessSerializer;
    use jpreprocess_dictionary_builder::ipadic_builder::IpadicBuilder;
    use lindera_core::dictionary_builder::DictionaryBuilder;

    use encoding::{
        all::UTF_8,
        {EncoderTrap, Encoding},
    };
    use flate2::read::GzDecoder;
    use tar::Archive;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    // Directory path for build package
    let build_dir = env::var_os("OUT_DIR").unwrap(); // ex) target/debug/build/<pkg>/out

    // Dictionary file name
    let file_name = "v0.1.1.tar.gz";

    // MeCab IPADIC directory
    let input_dir = Path::new(&build_dir).join("naist-jdic-0.1.1");

    if std::env::var("DOCS_RS").is_ok() {
        // Create directory for dummy input directory for build docs
        create_dir(&input_dir)?;

        // Create dummy char.def
        let mut dummy_char_def = File::create(input_dir.join("char.def"))?;
        dummy_char_def.write_all(b"DEFAULT 0 1 0\n")?;

        // Create dummy CSV file
        let mut dummy_dict_csv = File::create(input_dir.join("dummy_dict.csv"))?;
        dummy_dict_csv.write_all(
            &UTF_8
                .encode(
                    "テスト,1343,1343,3195,名詞,サ変接続,*,*,*,*,テスト,テスト,テスト,1/3,C1\n",
                    EncoderTrap::Ignore,
                )
                .unwrap(),
        )?;

        // Create dummy unk.def
        File::create(input_dir.join("unk.def"))?;
        let mut dummy_matrix_def = File::create(input_dir.join("matrix.def"))?;
        dummy_matrix_def.write_all(b"0 1 0\n")?;
    } else {
        // Source file path for build package
        let source_path_for_build = Path::new(&build_dir).join(file_name);

        // Download source file to build directory
        if !source_path_for_build.exists() {
            // copy(&source_path, &source_path_for_build)?;
            let tmp_path = Path::new(&build_dir).join(file_name.to_owned() + ".download");

            // Download a tarball
            let download_url =
                "https://github.com/jpreprocess/naist-jdic/archive/refs/tags/v0.1.1.tar.gz";
            let resp = ureq::get(download_url).call()?;
            let mut dest = File::create(&tmp_path)?;

            io::copy(&mut resp.into_reader(), &mut dest)?;
            dest.flush()?;

            rename(tmp_path, &source_path_for_build).expect("Failed to rename temporary file");
        }

        // Decompress a tar.gz file
        let mut tar_gz = File::open(source_path_for_build)?;
        let mut buffer = Vec::new();
        tar_gz.read_to_end(&mut buffer)?;
        let cursor = Cursor::new(buffer);
        let gzdecoder = GzDecoder::new(cursor);
        let mut archive = Archive::new(gzdecoder);
        archive.unpack(&build_dir)?;
    }

    // Lindera IPADIC directory
    let output_dir = Path::new(&build_dir).join("naist-jdic");

    // Build a dictionary
    let builder = IpadicBuilder::new(Box::new(JPreprocessSerializer));
    builder.build_dictionary(&input_dir, &output_dir)?;

    let license_file = &input_dir.join(Path::new("COPYING"));
    if license_file.exists() {
        copy(license_file, output_dir.join(Path::new("COPYING")))?;
    }

    Ok(())
}

#[cfg(not(feature = "naist-jdic"))]
fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
