fn main() -> Result<(), Box<dyn std::error::Error>> {
    let csv1 = std::fs::read_to_string("tests/data/naist-jdic/naist-jdic.csv")?;
    let csv2 = std::fs::read_to_string("tests/data/naist-jdic/unidic-csj.csv")?;
    let dict = vibrato::SystemDictionaryBuilder::from_readers(
        format!("{}\n{}", csv1, csv2).as_bytes(),
        std::fs::File::open("tests/data/naist-jdic/matrix.def")?,
        std::fs::File::open("tests/data/naist-jdic/char.def")?,
        std::fs::File::open("tests/data/naist-jdic/unk.def")?,
    )?;

    let tokenizer = vibrato::Tokenizer::new(dict)
        .ignore_space(true)?
        .max_grouping_len(24);

    let jpreprocess = jpreprocess::JPreprocess::from_tokenizer(tokenizer);
    let njd =
        jpreprocess.run_frontend("日本語文を解析し、音声合成エンジンに渡せる形式に変換します．")?;

    for node in njd.iter() {
        println!("{}", node);
    }

    Ok(())
}
