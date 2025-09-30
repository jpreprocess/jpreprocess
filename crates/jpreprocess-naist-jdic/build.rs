use std::error::Error;

#[cfg(feature = "naist-jdic")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    if std::env::var("DOCS_RS").is_ok() {
        // Skip building the dictionary when building docs.rs
        return Ok(());
    } else {
        fetch_dictionary::download(false).await
    }
}

#[cfg(not(feature = "naist-jdic"))]
fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(feature = "naist-jdic")]
mod fetch_dictionary {
    use std::{error::Error, path::PathBuf};

    const DICTIONARY_PREBUILT_URL: &str = concat!(
        "https://github.com/jpreprocess/jpreprocess/releases/download/",
        env!("CARGO_PKG_VERSION"),
        "/naist-jdic-jpreprocess.tar.gz"
    );
    const DICTIONARY_PREBUILT_MD5: &str = "a27d2548ecc8e76242c056e5644a2e57";

    const DICTIONARY_SRC_URL: &str =
        "https://github.com/jpreprocess/naist-jdic/archive/refs/tags/v0.1.3.tar.gz";
    const DICTIONARY_SRC_MD5: &str = "a27d2548ecc8e76242c056e5644a2e57";

    pub async fn download(force_build: bool) -> Result<(), Box<dyn Error>> {
        let client = reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent(concat!(
                "jpreprocess-naist-jdic/",
                env!("CARGO_PKG_VERSION"),
            ))
            .build()?;

        let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        let dict_dir = out_dir.join("naist-jdic");

        println!(
            "cargo::rustc-env=JPREPROCESS_WORKDIR={}",
            dict_dir.display()
        );

        if !force_build {
            println!("Attempting to download prebuilt naist-jdic...");

            let prebuilt_download_dir = out_dir.join("naist-jdic-prebuilt");

            let prebuilt_result = fetch(
                &client,
                DICTIONARY_PREBUILT_URL,
                DICTIONARY_PREBUILT_MD5,
                prebuilt_download_dir.clone(),
            )
            .await;

            if prebuilt_result.is_ok() {
                println!("Successfully downloaded prebuilt naist-jdic.");

                let prebuilt_name = prebuilt_download_dir.iter().next().unwrap();
                let prebuilt_dir = prebuilt_download_dir.join(prebuilt_name);
                std::fs::rename(&prebuilt_dir, &dict_dir)?;

                return Ok(());
            } else {
                println!("Failed to download prebuilt naist-jdic, falling back to building from source: {}", prebuilt_result.unwrap_err());
            }
        }

        println!("Downloading and building naist-jdic from source...");

        let src_download_dir = out_dir.join("naist-jdic-src");

        fetch(
            &client,
            DICTIONARY_SRC_URL,
            DICTIONARY_SRC_MD5,
            src_download_dir.clone(),
        )
        .await?;

        println!("Successfully downloaded source dictionary.");

        let src_name = std::fs::read_dir(&src_download_dir)?
            .next()
            .ok_or("No directory found in source download dir")??
            .file_name();
        let src_dir = src_download_dir.join(src_name);

        jpreprocess_dictionary::dictionary::to_dict::JPreprocessDictionaryBuilder::new()
            .build_dictionary(&src_dir, &dict_dir)?;

        Ok(())
    }

    async fn fetch(
        client: &reqwest::Client,
        url: &str,
        md5hash: &str,
        path: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let response = client.get(url).send().await?;
        let bytes = response.bytes().await?;

        let mut context = md5::Context::new();
        context.consume(&bytes);
        let digest = context.finalize();
        let hash = format!("{:x}", digest);
        if hash != md5hash {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "MD5 hash mismatch for prebuilt dictionary: expected {}, got {}",
                    md5hash, hash
                ),
            )));
        }

        let tar = flate2::read::GzDecoder::new(&bytes[..]);
        let mut archive = tar::Archive::new(tar);
        archive.unpack(path)?;

        Ok(())
    }
}
