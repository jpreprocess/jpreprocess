use std::error::Error;

#[cfg(feature = "naist-jdic")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=prebuilt.json");
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
        let client = reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent(concat!(
                "jpreprocess-naist-jdic/",
                env!("CARGO_PKG_VERSION"),
            ))
            .build()?;

        let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        let work_dir = out_dir.join("work");
        let dict_dir = out_dir.join("naist-jdic");

        println!(
            "cargo::rustc-env=JPREPROCESS_WORKDIR={}",
            dict_dir.display()
        );

        if !force_build {
            match download_prebuilt(&client, &work_dir, &out_dir).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    println!(
                    "Failed to download prebuilt naist-jdic, falling back to building from source: {}",
                    e
                );
                }
            }
        }

        println!("Downloading and building naist-jdic from source...");

        let build = {
            let config_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("build.json");
            let config_data = std::fs::read_to_string(config_path)?;
            serde_json::from_str::<BuildConfig>(&config_data)?
        };

        build.build(&client, &work_dir, &dict_dir).await?;

        Ok(())
    }

    async fn download_prebuilt(
        client: &reqwest::Client,
        work_dir: &Path,
        out_dir: &Path,
    ) -> Result<(), Box<dyn Error>> {
        let prebuilt = {
            let config_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("prebuilt.json");
            let config_data = std::fs::read_to_string(config_path)?;
            serde_json::from_str::<FetchConfig>(&config_data)?
        };

        println!("Attempting to download prebuilt naist-jdic...");
        let prebuilt_download_dir = work_dir.join("naist-jdic-prebuilt");
        prebuilt.fetch(client, &prebuilt_download_dir).await?;

        println!("Successfully downloaded prebuilt naist-jdic.");

        let prebuilt_name = prebuilt_download_dir.iter().next().unwrap();
        let prebuilt_dir = prebuilt_download_dir.join(prebuilt_name);
        std::fs::rename(&prebuilt_dir, out_dir)?;

        Ok(())
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
            work_dir: &Path,
            out_dir: &Path,
        ) -> Result<(), Box<dyn Error>> {
            let src_download_dir = work_dir.join("src");
            self.src.fetch(client, &src_download_dir).await?;

            let src_name = std::fs::read_dir(&src_download_dir)?
                .next()
                .ok_or("No directory found in source download dir")??
                .file_name();
            let src_dir = src_download_dir.join(src_name);

            jpreprocess_dictionary::dictionary::to_dict::JPreprocessDictionaryBuilder::new(
                self.metadata.clone(),
            )
            .build_dictionary(&src_dir, out_dir)?;

            Ok(())
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct FetchConfig {
        url: String,
        digest: String,
    }

    impl FetchConfig {
        async fn fetch(&self, client: &reqwest::Client, path: &Path) -> Result<(), Box<dyn Error>> {
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
