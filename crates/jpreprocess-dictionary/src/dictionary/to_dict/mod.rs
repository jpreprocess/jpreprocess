use std::{fs, path::Path};

use lindera_dictionary::{
    builder::{
        character_definition::CharacterDefinitionBuilderOptions,
        connection_cost_matrix::ConnectionCostMatrixBuilderOptions, metadata::MetadataBuilder,
        unknown_dictionary::UnknownDictionaryBuilderOptions,
        user_dictionary::build_user_dictionary,
    },
    dictionary::{
        character_definition::CharacterDefinition, metadata::Metadata, schema::Schema,
        UserDictionary,
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

pub struct JPreprocessDictionaryBuilder {
    metadata: Metadata,
}

impl JPreprocessDictionaryBuilder {
    pub fn new(metadata: Metadata) -> Self {
        Self { metadata }
    }
}

impl Default for JPreprocessDictionaryBuilder {
    fn default() -> Self {
        Self {
            metadata: Self::default_metadata(),
        }
    }
}

impl JPreprocessDictionaryBuilder {
    pub fn default_metadata() -> Metadata {
        Metadata {
            dictionary_schema: Schema::new(vec![
                "surface".to_string(),
                "left_context_id".to_string(),
                "right_context_id".to_string(),
                "cost".to_string(),
                "major_pos".to_string(),
                "middle_pos".to_string(),
                "small_pos".to_string(),
                "fine_pos".to_string(),
                "conjugation_type".to_string(),
                "conjugation_form".to_string(),
                "base_form".to_string(),
                "reading".to_string(),
                "pronunciation".to_string(),
                // Additional fields
                "accent_morasize".to_string(),
                "chain_rule".to_string(),
                "chain_flag".to_string(),
            ]),
            ..Default::default()
        }
    }

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

        self.build_user_dict_from_rows(rows)
    }
    pub fn build_user_dict_from_data(&self, data: Vec<Vec<&str>>) -> LinderaResult<UserDictionary> {
        let rows = data
            .into_iter()
            .map(csv::StringRecord::from_iter)
            .collect::<Vec<_>>();

        self.build_user_dict_from_rows(rows)
    }

    fn build_user_dict_from_rows(
        &self,
        rows: Vec<csv::StringRecord>,
    ) -> LinderaResult<UserDictionary> {
        let parser = UserDictionaryParserOptions::default()
            .user_dictionary_fields_num(self.metadata.user_dictionary_schema.field_count())
            .default_word_cost(self.metadata.default_word_cost)
            .default_left_context_id(self.metadata.default_left_context_id)
            .default_right_context_id(self.metadata.default_right_context_id)
            .dictionary_parser(
                DefaultParserOptions::default()
                    .skip_invalid_cost_or_id(self.metadata.skip_invalid_cost_or_id)
                    .schema(self.metadata.dictionary_schema.clone())
                    .normalize_details(self.metadata.normalize_details)
                    .builder()
                    .unwrap(),
            )
            .user_dictionary_parser(
                DefaultParserOptions::default()
                    .schema(self.metadata.user_dictionary_schema.clone())
                    .normalize_details(self.metadata.normalize_details)
                    .builder()
                    .unwrap(),
            )
            .builder()
            .unwrap();

        let dict = generate_prefix_dictionary::<
            UserDictionaryParser,
            JPreprocessDictionaryWordEncoding,
        >(&parser, &rows, false)?;

        Ok(UserDictionary { dict })
    }
}

#[deprecated(
    note = "Use JPreprocessDictionaryBuilder::build_user_dict_from_data instead",
    since = "0.13.0"
)]
pub fn build_user_dict_from_data(data: Vec<Vec<&str>>) -> LinderaResult<UserDictionary> {
    let rows = data
        .into_iter()
        .map(csv::StringRecord::from_iter)
        .collect::<Vec<_>>();

    let builder = JPreprocessDictionaryBuilder::new(Metadata {
        default_word_cost: -10000,
        default_left_context_id: 0,
        default_right_context_id: 0,
        ..JPreprocessDictionaryBuilder::default_metadata()
    });

    builder.build_user_dict_from_rows(rows)
}

#[cfg(test)]
mod tests {
    use lindera_dictionary::viterbi::{LexType, WordEntry, WordId};

    use super::*;

    #[test]
    fn test_user_dictionary() {
        let builder = JPreprocessDictionaryBuilder::default();

        let data = vec![
            vec![
                "東京スカイツリー",
                "1285",
                "1285",
                "-3000",
                "名詞",
                "固有名詞",
                "一般",
                "*",
                "*",
                "*",
                "*",
                "トウキョウスカイツリー",
                "トウキョウスカイツリー",
                "13",
                "*",
                "*",
            ],
            vec![
                "すもももももももものうち",
                "1285",
                "1285",
                "-3000",
                "名詞",
                "固有名詞",
                "一般",
                "*",
                "*",
                "*",
                "*",
                "スモモモモモモモモノウチ",
                "スモモモモモモモモノウチ",
                "13",
                "*",
                "*",
            ],
        ];

        let user_dict = builder.build_user_dict_from_data(data).unwrap();
        assert_eq!(
            user_dict.dict.find_surface("東京スカイツリー"),
            vec![WordEntry {
                word_id: WordId {
                    id: 0,
                    is_system: false,
                    lex_type: LexType::User,
                },
                word_cost: -3000,
                left_id: 1285,
                right_id: 1285,
            },]
        );
        assert_eq!(
            user_dict.dict.find_surface("すもももももももものうち"),
            vec![WordEntry {
                word_id: WordId {
                    id: 1,
                    is_system: false,
                    lex_type: LexType::User,
                },
                word_cost: -3000,
                left_id: 1285,
                right_id: 1285,
            },]
        );
    }

    #[test]
    fn test_simple_user_dictionary() {
        let builder = JPreprocessDictionaryBuilder::default();

        let data = vec![
            vec![
                "東京スカイツリー",       // surface
                "トウキョウスカイツリー", // reading
                "トーキョースカイツリー", // pronunciation
            ],
            vec![
                "すもももももももものうち",
                "スモモモモモモモモノウチ",
                "スモモモモモモモモノウチ",
            ],
        ];

        let user_dict = builder.build_user_dict_from_data(data).unwrap();
        assert_eq!(
            user_dict.dict.find_surface("東京スカイツリー"),
            vec![WordEntry {
                word_id: WordId {
                    id: 0,
                    is_system: false,
                    lex_type: LexType::User,
                },
                word_cost: -10000,
                left_id: 1288,
                right_id: 1288,
            },]
        );
        assert_eq!(
            user_dict.dict.find_surface("すもももももももものうち"),
            vec![WordEntry {
                word_id: WordId {
                    id: 1,
                    is_system: false,
                    lex_type: LexType::User,
                },
                word_cost: -10000,
                left_id: 1288,
                right_id: 1288,
            },]
        );
    }
}
