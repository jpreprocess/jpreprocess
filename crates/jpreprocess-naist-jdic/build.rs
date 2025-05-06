use std::error::Error;

#[cfg(feature = "naist-jdic")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    lindera_dictionary::assets::fetch(
        lindera_dictionary::assets::FetchParams {
            file_name: "v0.1.3.tar.gz",
            input_dir: "naist-jdic-0.1.3",
            output_dir: "naist-jdic",
            download_url:
                "https://github.com/jpreprocess/naist-jdic/archive/refs/tags/v0.1.3.tar.gz",
            dummy_input:
                "テスト,1343,1343,3195,名詞,サ変接続,*,*,*,*,テスト,テスト,テスト,1/3,C1\n",
        },
        jpreprocess_dictionary::dictionary::to_dict::JPreprocessDictionaryBuilder::new(),
    )
    .await
}

#[cfg(not(feature = "naist-jdic"))]
fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
