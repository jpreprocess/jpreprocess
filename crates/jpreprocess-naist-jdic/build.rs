use std::error::Error;

#[cfg(feature = "naist-jdic")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=build.json");

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
    use serde::{Deserialize, Serialize};
    use std::{
        error::Error,
        path::{Path, PathBuf},
    };

    pub async fn download(force_build: bool) -> Result<(), Box<dyn Error>> {
        let config_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("build.json");
        let config_data = std::fs::read_to_string(&config_path)?;
        let mut config: Config = serde_json::from_str(&config_data)?;
        if force_build {
            config.force_build();
        }
        config.download().await
    }

    /// Configuration for downloading and building the dictionary
    #[derive(Clone, Serialize, Deserialize)]
    struct Config {
        prebuilt: Option<FetchConfig>,
        build: BuildConfig,
    }

    impl Config {
        fn force_build(&mut self) {
            self.prebuilt = None;
        }
        async fn download(&self) -> Result<(), Box<dyn Error>> {
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

            if let Some(prebuilt) = &self.prebuilt {
                println!("Attempting to download prebuilt naist-jdic...");

                let prebuilt_download_dir = out_dir.join("naist-jdic-prebuilt");

                let prebuilt_result = prebuilt.fetch(&client, prebuilt_download_dir.clone()).await;

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

            self.build
                .build(&client, out_dir.join("work"), dict_dir)
                .await?;

            Ok(())
        }
    }

    /// Configuration for building the dictionary from source (fallback)
    #[derive(Clone, Serialize, Deserialize)]
    struct BuildConfig {
        src: FetchConfig,
        metadata: lindera_dictionary::dictionary::metadata::Metadata,
    }

    impl BuildConfig {
        async fn build(
            &self,
            client: &reqwest::Client,
            work_dir: PathBuf,
            out_dir: PathBuf,
        ) -> Result<(), Box<dyn Error>> {
            let src_download_dir = work_dir.join("src");
            self.src.fetch(client, src_download_dir.clone()).await?;

            let src_name = std::fs::read_dir(&src_download_dir)?
                .next()
                .ok_or("No directory found in source download dir")??
                .file_name();
            let src_dir = src_download_dir.join(src_name);

            jpreprocess_dictionary::dictionary::to_dict::JPreprocessDictionaryBuilder::new(
                self.metadata.clone(),
            )
            .build_dictionary(&src_dir, &out_dir)?;

            Ok(())
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct FetchConfig {
        url: String,
        digest: String,
    }

    impl FetchConfig {
        async fn fetch(
            &self,
            client: &reqwest::Client,
            path: PathBuf,
        ) -> Result<(), Box<dyn Error>> {
            let response = client.get(&self.url).send().await?;
            let bytes = response.bytes().await?;

            let mut context = md5::Context::new();
            context.consume(&bytes);
            let digest = context.finalize();
            let hash = format!("{:x}", digest);
            if hash != self.digest {
                return Err(Box::new(std::io::Error::other(format!(
                    "MD5 hash mismatch for prebuilt dictionary: expected {}, got {}",
                    self.digest, hash
                ))));
            }

            let tar = flate2::read::GzDecoder::new(&bytes[..]);
            let mut archive = tar::Archive::new(tar);
            archive.unpack(path)?;

            Ok(())
        }
    }
}
