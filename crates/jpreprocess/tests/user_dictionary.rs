use std::error::Error;

use jpreprocess::{JPreprocess, JPreprocessConfig, SystemDictionaryConfig};
use jpreprocess_dictionary::serializer::jpreprocess::JPreprocessSerializer;
use jpreprocess_dictionary_builder::ipadic_builder::IpadicBuilder;

#[cfg(feature = "naist-jdic")]
use jpreprocess::kind::*;

#[cfg(not(feature = "naist-jdic"))]
use std::path::PathBuf;

#[test]
#[ignore]
fn system_dictionary() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "naist-jdic")]
    let config = SystemDictionaryConfig::Bundled(JPreprocessDictionaryKind::NaistJdic);
    #[cfg(not(feature = "naist-jdic"))]
    let config = SystemDictionaryConfig::File(PathBuf::from("tests/dict"));

    let jpreprocess = JPreprocess::from_config(JPreprocessConfig {
        dictionary: config,
        user_dictionary: None,
    })
    .unwrap();
    let njd = jpreprocess.text_to_njd("クーバネティス")?;

    assert_eq!(njd.nodes[0].get_string(), "ク");
    assert_eq!(njd.nodes[1].get_string(), "ーバネティス");

    Ok(())
}

#[test]
#[ignore]
fn lindera_user_dictionary() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "naist-jdic")]
    let config = SystemDictionaryConfig::Bundled(JPreprocessDictionaryKind::NaistJdic);
    #[cfg(not(feature = "naist-jdic"))]
    let config = SystemDictionaryConfig::File(PathBuf::from("tests/dict"));

    let mut rows: Vec<Vec<&str>> = vec![vec![
        "クーバネティス",
        "1348",
        "1348",
        "-28650",
        "名詞",
        "固有名詞",
        "一般",
        "*",
        "*",
        "*",
        "Kubernetes",
        "クーバネティス",
        "クーバネティス",
        "4/6",
        "*",
    ]];
    rows.sort_by_key(|row| row[0].to_string());

    let dictionary = config.load()?;
    let user_dictionary =
        IpadicBuilder::new(Box::new(JPreprocessSerializer)).build_user_dict_from_data(&rows)?;

    let jpreprocess = JPreprocess::with_dictionaries(dictionary, Some(user_dictionary));
    let njd = jpreprocess.text_to_njd("クーバネティス")?;

    assert_eq!(njd.nodes[0].get_string(), "クーバネティス");

    Ok(())
}
