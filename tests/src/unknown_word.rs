#[cfg(test)]
mod unknown_words {
    use std::path::PathBuf;

    use jpreprocess::{JPreprocess, JPreprocessConfig, SystemDictionaryConfig};

    #[test]
    fn barry_payne() {
        let input_text = "バリー・ペーンは";
        let config = SystemDictionaryConfig::File(PathBuf::from("data/min-dict"));
        let jpreprocess = JPreprocess::from_config(JPreprocessConfig {
            dictionary: config,
            user_dictionary: None,
        })
        .unwrap();
        let mut njd = jpreprocess.text_to_njd(input_text).unwrap();

        njd.preprocess();

        for node in &njd.nodes {
            println!("{}", node);
        }

        assert_eq!(njd.nodes[0].get_string(), "バリー・ペーン");
    }
}
