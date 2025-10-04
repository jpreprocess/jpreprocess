use std::error::Error;

use jpreprocess::{JPreprocess, SystemDictionaryConfig};

#[cfg(feature = "naist-jdic")]
use jpreprocess::kind::*;
use jpreprocess_dictionary::dictionary::to_dict::build_user_dict_from_data;

#[cfg(not(feature = "naist-jdic"))]
use std::path::PathBuf;

#[test]
#[ignore]
fn system_dictionary() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "naist-jdic")]
    let config = SystemDictionaryConfig::Bundled(JPreprocessDictionaryKind::NaistJdic);
    #[cfg(not(feature = "naist-jdic"))]
    let config = SystemDictionaryConfig::File(PathBuf::from("data/dict"));

    let jpreprocess = JPreprocess::with_dictionaries(config.load()?, None);

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
    let config = SystemDictionaryConfig::File(PathBuf::from("data/dict"));

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
    let user_dictionary = build_user_dict_from_data(rows)?;

    let jpreprocess = JPreprocess::with_dictionaries(dictionary, Some(user_dictionary));
    let njd = jpreprocess.text_to_njd("クーバネティス")?;

    assert_eq!(njd.nodes[0].get_string(), "クーバネティス");

    Ok(())
}
