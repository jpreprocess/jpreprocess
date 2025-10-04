use std::{fs, path::Path};

use lindera_dictionary::{
    builder::{
        character_definition::CharacterDefinitionBuilderOptions,
        connection_cost_matrix::ConnectionCostMatrixBuilderOptions, metadata::MetadataBuilder,
        unknown_dictionary::UnknownDictionaryBuilderOptions,
        user_dictionary::build_user_dictionary,
    },
    dictionary::{character_definition::CharacterDefinition, metadata::Metadata, UserDictionary},
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

pub struct JPreprocessDictionaryBuilder {
    metadata: Metadata,
}

impl JPreprocessDictionaryBuilder {
    pub fn new() -> Self {
        Self {
            metadata: Metadata::default(),
        }
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

        self.build_metadata(output_dir)?;
        let chardef = self.build_character_definition(input_dir, output_dir)?;
        self.build_unknown_dictionary(input_dir, &chardef, output_dir)?;
        self.build_prefix_dictionary(input_dir, output_dir)?;
        self.build_connection_cost_matrix(input_dir, output_dir)?;

        Ok(())
    }

    pub fn build_metadata(&self, output_dir: &Path) -> LinderaResult<()> {
        MetadataBuilder::new().build(&self.metadata, output_dir)
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
            .encoding(self.metadata.encoding.clone())
            .compress_algorithm(self.metadata.compress_algorithm)
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
            .encoding(self.metadata.encoding.clone())
            .compress_algorithm(self.metadata.compress_algorithm)
            .builder()
            .unwrap()
            .build(input_dir, chardef, output_dir)
    }

    pub fn build_prefix_dictionary(
        &self,
        input_dir: &Path,
        output_dir: &Path,
    ) -> LinderaResult<()> {
        let reader = CSVReaderOptions::default()
            .flexible_csv(self.metadata.flexible_csv)
            .encoding(self.metadata.encoding.clone())
            .normalize_details(self.metadata.normalize_details)
            .builder()
            .unwrap();
        let rows = reader.load_csv_data(input_dir)?;

        let parser = DefaultParserOptions::default()
            .skip_invalid_cost_or_id(self.metadata.skip_invalid_cost_or_id)
            .schema(self.metadata.dictionary_schema.clone())
            .normalize_details(self.metadata.normalize_details)
            .builder()
            .unwrap();

        write_prefix_dictionary::<DefaultParser, JPreprocessDictionaryWordEncoding>(
            &parser,
            &rows,
            output_dir,
            self.metadata.compress_algorithm,
        )
    }

    pub fn build_connection_cost_matrix(
        &self,
        input_dir: &Path,
        output_dir: &Path,
    ) -> LinderaResult<()> {
        ConnectionCostMatrixBuilderOptions::default()
            .encoding(self.metadata.encoding.clone())
            .compress_algorithm(self.metadata.compress_algorithm)
            .builder()
            .unwrap()
            .build(input_dir, output_dir)
    }

    pub fn build_user_dict(&self, input_file: &Path) -> LinderaResult<UserDictionary> {
        let reader = CSVReaderOptions::default()
            .flexible_csv(self.metadata.flexible_csv)
            .builder()
            .unwrap();
        let rows = reader.read_csv_files(&[input_file.to_path_buf()])?;

        let parser = UserDictionaryParserOptions::default()
            .user_dictionary_fields_num(self.metadata.user_dictionary_schema.field_count())
            .default_word_cost(self.metadata.default_word_cost)
            .default_left_context_id(self.metadata.default_left_context_id)
            .default_right_context_id(self.metadata.default_right_context_id)
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
        .user_dictionary_fields_num(SIMPLE_USERDIC_FIELDS_NUM)
        .default_left_context_id(SIMPLE_CONTEXT_ID)
        .default_right_context_id(SIMPLE_CONTEXT_ID)
        .default_word_cost(SIMPLE_WORD_COST)
        .builder()
        .unwrap();

    let dict = generate_prefix_dictionary::<UserDictionaryParser, JPreprocessDictionaryWordEncoding>(
        &parser, &rows, false,
    )?;

    Ok(UserDictionary { dict })
}
