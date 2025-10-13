use std::str::FromStr;

use csv::StringRecord;
use derive_builder::Builder;

use jpreprocess_core::word_line::WordDetailsLine;
use lindera_dictionary::dictionary::schema::Schema;

use thiserror::Error;

pub trait CSVParser {
    fn surface(&self, row: &StringRecord) -> Result<String, CSVParseError>;
    fn left_context_id(&self, row: &StringRecord) -> Result<u16, CSVParseError>;
    fn right_context_id(&self, row: &StringRecord) -> Result<u16, CSVParseError>;
    fn cost(&self, row: &StringRecord) -> Result<i16, CSVParseError>;
    fn details(&self, row: &StringRecord) -> Result<WordDetailsLine, CSVParseError>;
}

#[derive(Error, Debug)]
pub enum CSVParseError {
    #[error("Invalid {0} value: {1}")]
    InvalidValue(CSVField, String),
    #[error("Field {0} not found")]
    FieldNotFound(CSVField),
}

#[derive(Error, Debug)]
pub enum CSVField {
    Surface,
    LeftContextId,
    RightContextId,
    Cost,
}

impl std::fmt::Display for CSVField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CSVField::Surface => write!(f, "surface"),
            CSVField::LeftContextId => write!(f, "left context ID"),
            CSVField::RightContextId => write!(f, "right context ID"),
            CSVField::Cost => write!(f, "cost"),
        }
    }
}

#[derive(Builder)]
#[builder(name = DefaultParserOptions)]
#[builder(build_fn(name = "builder"))]
pub struct DefaultParser {
    #[builder(default = "false")]
    normalize_details: bool,
    #[builder(default = "false")]
    skip_invalid_cost_or_id: bool,
    #[builder(default = "WordDetailsLine::default()")]
    default_details: WordDetailsLine,

    #[builder(default = "Schema::default()")]
    schema: Schema,
}

impl Default for DefaultParser {
    fn default() -> Self {
        DefaultParserOptions::default().builder().unwrap()
    }
}

impl DefaultParser {
    /// Get field value
    fn get_field_value(&self, row: &StringRecord, field_name: &str) -> Option<String> {
        if let Some(index) = self.schema.get_field_index(field_name) {
            if index >= row.len() {
                return None;
            }

            let value = row[index].trim();
            if value.is_empty() {
                None
            } else {
                Some(value.to_string())
            }
        } else {
            None
        }
    }
}

impl CSVParser for DefaultParser {
    fn surface(&self, row: &StringRecord) -> Result<String, CSVParseError> {
        let surface = self
            .get_field_value(row, "surface")
            .ok_or(CSVParseError::FieldNotFound(CSVField::Surface))?;
        if self.normalize_details {
            Ok(normalize(&surface))
        } else {
            Ok(surface)
        }
    }

    /// Parse word cost using schema
    fn cost(&self, row: &StringRecord) -> Result<i16, CSVParseError> {
        let cost_str = self
            .get_field_value(row, "cost")
            .ok_or(CSVParseError::FieldNotFound(CSVField::Cost))?;
        match i16::from_str(&cost_str) {
            Ok(cost) => Ok(cost),
            Err(_) => {
                if self.skip_invalid_cost_or_id {
                    Ok(0)
                } else {
                    Err(CSVParseError::InvalidValue(CSVField::Cost, cost_str))
                }
            }
        }
    }

    /// Parse left context ID using schema
    fn left_context_id(&self, row: &StringRecord) -> Result<u16, CSVParseError> {
        let left_id_str = self
            .get_field_value(row, "left_context_id")
            .ok_or(CSVParseError::FieldNotFound(CSVField::LeftContextId))?;
        match u16::from_str(&left_id_str) {
            Ok(id) => Ok(id),
            Err(_) => {
                if self.skip_invalid_cost_or_id {
                    Err(CSVParseError::FieldNotFound(CSVField::LeftContextId))
                } else {
                    Err(CSVParseError::InvalidValue(
                        CSVField::LeftContextId,
                        left_id_str,
                    ))
                }
            }
        }
    }

    /// Parse right context ID using schema
    fn right_context_id(&self, row: &StringRecord) -> Result<u16, CSVParseError> {
        let right_id_str = self
            .get_field_value(row, "right_context_id")
            .ok_or(CSVParseError::FieldNotFound(CSVField::RightContextId))?;
        match u16::from_str(&right_id_str) {
            Ok(id) => Ok(id),
            Err(_) => {
                if self.skip_invalid_cost_or_id {
                    Err(CSVParseError::FieldNotFound(CSVField::RightContextId))
                } else {
                    Err(CSVParseError::InvalidValue(
                        CSVField::RightContextId,
                        right_id_str,
                    ))
                }
            }
        }
    }

    /// Get word details from the row
    fn details(&self, row: &StringRecord) -> Result<WordDetailsLine, CSVParseError> {
        Ok(WordDetailsLine {
            pos: self
                .get_field_value(row, "major_pos")
                .unwrap_or(self.default_details.pos.to_string()),
            pos_group1: self
                .get_field_value(row, "middle_pos")
                .unwrap_or(self.default_details.pos_group1.to_string()),
            pos_group2: self
                .get_field_value(row, "small_pos")
                .unwrap_or(self.default_details.pos_group2.to_string()),
            pos_group3: self
                .get_field_value(row, "fine_pos")
                .unwrap_or(self.default_details.pos_group3.to_string()),
            ctype: self
                .get_field_value(row, "conjugation_type")
                .unwrap_or(self.default_details.ctype.to_string()),
            cform: self
                .get_field_value(row, "conjugation_form")
                .unwrap_or(self.default_details.cform.to_string()),
            orig: self
                .get_field_value(row, "base_form")
                .unwrap_or(self.default_details.orig.to_string()),
            read: self
                .get_field_value(row, "reading")
                .unwrap_or(self.default_details.read.to_string()),
            pron: self
                .get_field_value(row, "pronunciation")
                .unwrap_or(self.default_details.pron.to_string()),
            acc_morasize: self
                .get_field_value(row, "accent_morasize")
                .unwrap_or(self.default_details.acc_morasize.to_string()),
            chain_rule: self
                .get_field_value(row, "chain_rule")
                .unwrap_or(self.default_details.chain_rule.to_string()),
            chain_flag: self
                .get_field_value(row, "chain_flag")
                .unwrap_or(self.default_details.chain_flag.to_string()),
        })
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
#[builder(name = UserDictionaryParserOptions)]
#[builder(build_fn(name = "builder"))]
pub struct UserDictionaryParser {
    #[builder(default = "3")]
    user_dictionary_fields_num: usize,

    #[builder(default = "-10000")]
    default_word_cost: i16,
    #[builder(default = "0")]
    default_left_context_id: u16,
    #[builder(default = "0")]
    default_right_context_id: u16,

    #[builder(default = "DefaultParser::default()")]
    dictionary_parser: DefaultParser,
    #[builder(default = "DefaultParser::default()")]
    user_dictionary_parser: DefaultParser,
}

impl CSVParser for UserDictionaryParser {
    fn surface(&self, row: &StringRecord) -> Result<String, CSVParseError> {
        if row.len() == self.user_dictionary_fields_num {
            self.user_dictionary_parser.surface(row)
        } else {
            self.dictionary_parser.surface(row)
        }
    }

    fn cost(&self, row: &StringRecord) -> Result<i16, CSVParseError> {
        if row.len() == self.user_dictionary_fields_num {
            Ok(self.default_word_cost)
        } else {
            self.dictionary_parser.cost(row)
        }
    }

    fn left_context_id(&self, row: &StringRecord) -> Result<u16, CSVParseError> {
        if row.len() == self.user_dictionary_fields_num {
            Ok(self.default_left_context_id)
        } else {
            self.dictionary_parser.left_context_id(row)
        }
    }

    fn right_context_id(&self, row: &StringRecord) -> Result<u16, CSVParseError> {
        if row.len() == self.user_dictionary_fields_num {
            Ok(self.default_right_context_id)
        } else {
            self.dictionary_parser.right_context_id(row)
        }
    }

    fn details(&self, row: &StringRecord) -> Result<WordDetailsLine, CSVParseError> {
        if row.len() == self.user_dictionary_fields_num {
            self.user_dictionary_parser.details(row)
        } else {
            self.dictionary_parser.details(row)
        }
    }
}

pub fn normalize(text: &str) -> String {
    text.to_string().replace('―', "—").replace('～', "〜")
}
