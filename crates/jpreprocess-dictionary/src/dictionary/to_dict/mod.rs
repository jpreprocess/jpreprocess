use std::{fs, path::Path};

use lindera_dictionary::{
    decompress::Algorithm,
    dictionary::{character_definition::CharacterDefinition, UserDictionary},
    dictionary_builder::{
        character_definition::CharacterDefinitionBuilderOptions,
        connection_cost_matrix::ConnectionCostMatrixBuilderOptions,
        unknown_dictionary::UnknownDictionaryBuilderOptions,
        user_dictionary::build_user_dictionary,
    },
    error::LinderaErrorKind,
    LinderaResult,
};

use crate::dictionary::to_dict::prefix_dictionary::{
    generate_prefix_dictionary,
    parser::{
        DefaultParser, DefaultParserOptions, UserDictionaryParser, UserDictionaryParserOptions,
    },
    write_prefix_dictionary, CSVReaderOptions,
};

use super::word_encoding::JPreprocessDictionaryWordEncoding;

mod prefix_dictionary;

const SIMPLE_USERDIC_FIELDS_NUM: usize = 3;
const SIMPLE_WORD_COST: i16 = -10000;
const SIMPLE_CONTEXT_ID: u16 = 0;
const COMPRESS_ALGORITHM: Algorithm = Algorithm::Raw;

pub struct JPreprocessDictionaryBuilder {}

impl JPreprocessDictionaryBuilder {
    pub fn new() -> Self {
        JPreprocessDictionaryBuilder {}
    }
}

impl Default for JPreprocessDictionaryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl JPreprocessDictionaryBuilder {
    pub fn build_dictionary(&self, input_dir: &Path, output_dir: &Path) -> LinderaResult<()> {
        fs::create_dir_all(output_dir)
            .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;

        let chardef = self.build_character_definition(input_dir, output_dir)?;
        self.build_unknown_dictionary(input_dir, &chardef, output_dir)?;
        self.build_prefix_dictionary(input_dir, output_dir)?;
        self.build_connection_cost_matrix(input_dir, output_dir)?;

        Ok(())
    }

    pub fn build_user_dictionary(
        &self,
        input_file: &Path,
        output_file: &Path,
    ) -> LinderaResult<()> {
        let user_dict = self.build_user_dict(input_file)?;
        build_user_dictionary(user_dict, output_file)
    }

    pub fn build_character_definition(
        &self,
        input_dir: &Path,
        output_dir: &Path,
    ) -> LinderaResult<CharacterDefinition> {
        CharacterDefinitionBuilderOptions::default()
            .compress_algorithm(COMPRESS_ALGORITHM)
            .builder()
            .unwrap()
            .build(input_dir, output_dir)
    }

    pub fn build_unknown_dictionary(
        &self,
        input_dir: &Path,
        chardef: &CharacterDefinition,
        output_dir: &Path,
    ) -> LinderaResult<()> {
        UnknownDictionaryBuilderOptions::default()
            .compress_algorithm(COMPRESS_ALGORITHM)
            .builder()
            .unwrap()
            .build(input_dir, chardef, output_dir)
    }

    pub fn build_prefix_dictionary(
        &self,
        input_dir: &Path,
        output_dir: &Path,
    ) -> LinderaResult<()> {
        let reader = CSVReaderOptions::default().builder().unwrap();
        let rows = reader.load_csv_data(input_dir)?;

        let parser = DefaultParserOptions::default().builder().unwrap();
        let compress_algorithm = COMPRESS_ALGORITHM;

        write_prefix_dictionary::<DefaultParser, JPreprocessDictionaryWordEncoding>(
            &parser,
            &rows,
            output_dir,
            compress_algorithm,
        )
    }

    pub fn build_connection_cost_matrix(
        &self,
        input_dir: &Path,
        output_dir: &Path,
    ) -> LinderaResult<()> {
        ConnectionCostMatrixBuilderOptions::default()
            .compress_algorithm(COMPRESS_ALGORITHM)
            .builder()
            .unwrap()
            .build(input_dir, output_dir)
    }

    pub fn build_user_dict(&self, input_file: &Path) -> LinderaResult<UserDictionary> {
        let reader = CSVReaderOptions::default().builder().unwrap();
        let rows = reader.read_csv_files(&[input_file.to_path_buf()])?;

        let parser = UserDictionaryParserOptions::default()
            .simple_userdic_fields_num(SIMPLE_USERDIC_FIELDS_NUM)
            .simple_context_id(SIMPLE_CONTEXT_ID)
            .simple_word_cost(SIMPLE_WORD_COST)
            .builder()
            .unwrap();

        let dict = generate_prefix_dictionary::<
            UserDictionaryParser,
            JPreprocessDictionaryWordEncoding,
        >(&parser, &rows, false)?;

        Ok(UserDictionary { dict })
    }
}

pub fn build_user_dict_from_data(data: Vec<Vec<&str>>) -> LinderaResult<UserDictionary> {
    let rows = data
        .into_iter()
        .map(csv::StringRecord::from_iter)
        .collect::<Vec<_>>();

    let parser = UserDictionaryParserOptions::default()
        .simple_userdic_fields_num(SIMPLE_USERDIC_FIELDS_NUM)
        .simple_context_id(SIMPLE_CONTEXT_ID)
        .simple_word_cost(SIMPLE_WORD_COST)
        .builder()
        .unwrap();

    let dict = generate_prefix_dictionary::<UserDictionaryParser, JPreprocessDictionaryWordEncoding>(
        &parser, &rows, false,
    )?;

    Ok(UserDictionary { dict })
}
