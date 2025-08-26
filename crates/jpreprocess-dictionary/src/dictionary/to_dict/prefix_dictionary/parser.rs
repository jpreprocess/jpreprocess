use std::str::FromStr;

use csv::StringRecord;
use derive_builder::Builder;

use lindera_dictionary::dictionary::schema::Schema;

use thiserror::Error;

pub trait CSVParser {
    fn surface(&self, row: &StringRecord) -> Result<String, CSVParseError>;
    fn left_context_id(&self, row: &StringRecord) -> Result<u16, CSVParseError>;
    fn right_context_id(&self, row: &StringRecord) -> Result<u16, CSVParseError>;
    fn cost(&self, row: &StringRecord) -> Result<i16, CSVParseError>;
    fn details(&self, row: &StringRecord) -> Result<Vec<String>, CSVParseError>;
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
    #[builder(default = "Schema::default()")]
    schema: Schema,
}

impl DefaultParser {
    /// Get common field value by type
    fn get_common_field_value(
        &self,
        row: &StringRecord,
        field: lindera_dictionary::dictionary::schema::CommonField,
    ) -> Option<String> {
        let index = self.schema.get_common_field_index(field);

        if index >= row.len() {
            return None;
        }

        let value = row[index].trim();
        if value.is_empty() {
            None
        } else {
            Some(value.to_string())
        }
    }
}

impl CSVParser for DefaultParser {
    fn surface(&self, row: &StringRecord) -> Result<String, CSVParseError> {
        let surface = self
            .get_common_field_value(
                row,
                lindera_dictionary::dictionary::schema::CommonField::Surface,
            )
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
            .get_common_field_value(
                row,
                lindera_dictionary::dictionary::schema::CommonField::Cost,
            )
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
            .get_common_field_value(
                row,
                lindera_dictionary::dictionary::schema::CommonField::LeftContextId,
            )
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
            .get_common_field_value(
                row,
                lindera_dictionary::dictionary::schema::CommonField::RightContextId,
            )
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
    fn details(&self, row: &StringRecord) -> Result<Vec<String>, CSVParseError> {
        let details = row.iter().skip(4).map(|s| s.to_string()).collect();
        Ok(details)
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
#[builder(name = UserDictionaryParserOptions)]
#[builder(build_fn(name = "builder"))]
pub struct UserDictionaryParser {
    #[builder(default = "3")]
    simple_userdic_fields_num: usize,
    #[builder(default = "-10000")]
    simple_word_cost: i16,
    #[builder(default = "0")]
    simple_context_id: u16,
}

impl CSVParser for UserDictionaryParser {
    fn surface(&self, row: &StringRecord) -> Result<String, CSVParseError> {
        let column = row
            .get(0)
            .ok_or(CSVParseError::FieldNotFound(CSVField::Surface))?;
        Ok(column.to_string())
    }

    fn cost(&self, row: &StringRecord) -> Result<i16, CSVParseError> {
        if row.len() == self.simple_userdic_fields_num {
            Ok(self.simple_word_cost)
        } else {
            let column = row
                .get(3)
                .ok_or(CSVParseError::FieldNotFound(CSVField::Cost))?;
            column
                .parse::<i16>()
                .map_err(|_| CSVParseError::InvalidValue(CSVField::Cost, column.to_string()))
        }
    }

    fn left_context_id(&self, row: &StringRecord) -> Result<u16, CSVParseError> {
        if row.len() == self.simple_userdic_fields_num {
            Ok(self.simple_context_id)
        } else {
            let column = row
                .get(1)
                .ok_or(CSVParseError::FieldNotFound(CSVField::LeftContextId))?;
            column.parse::<u16>().map_err(|_| {
                CSVParseError::InvalidValue(CSVField::LeftContextId, column.to_string())
            })
        }
    }

    fn right_context_id(&self, row: &StringRecord) -> Result<u16, CSVParseError> {
        if row.len() == self.simple_userdic_fields_num {
            Ok(self.simple_context_id)
        } else {
            let column = row
                .get(2)
                .ok_or(CSVParseError::FieldNotFound(CSVField::RightContextId))?;
            column.parse::<u16>().map_err(|_| {
                CSVParseError::InvalidValue(CSVField::RightContextId, column.to_string())
            })
        }
    }

    fn details(&self, row: &StringRecord) -> Result<Vec<String>, CSVParseError> {
        if row.len() == self.simple_userdic_fields_num {
            let details = row.iter().skip(1).map(|s| s.to_string()).collect();
            Ok(details)
        } else {
            let details = row.iter().skip(4).map(|s| s.to_string()).collect();
            Ok(details)
        }
    }
}

pub fn normalize(text: &str) -> String {
    text.to_string().replace('―', "—").replace('～', "〜")
}
