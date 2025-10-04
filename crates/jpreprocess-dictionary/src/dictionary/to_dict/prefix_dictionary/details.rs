use std::io::Write;

use byteorder::{LittleEndian, WriteBytesExt};
use csv::StringRecord;
use lindera_dictionary::{error::LinderaErrorKind, LinderaResult};

use crate::dictionary::{
    to_dict::prefix_dictionary::parser::CSVParser, word_encoding::DictionaryWordEncoding,
};

/// Generate word detail files (dict.words, dict.wordsidx)
pub fn generate_words_files<P, E>(
    parser: &P,
    rows: &[StringRecord],
) -> LinderaResult<(Vec<u8>, Vec<u8>)>
where
    P: CSVParser,
    E: DictionaryWordEncoding,
{
    let mut dict_words_buffer = Vec::new();
    let mut dict_wordsidx_buffer = Vec::new();

    dict_words_buffer
        .write_all(E::identifier().as_bytes())
        .map_err(|err| {
            LinderaErrorKind::Io
                .with_error(anyhow::anyhow!(err))
                .add_context("Failed to write dictionary identifier to dict.words buffer")
        })?;

    for row in rows.iter() {
        let offset = dict_words_buffer.len();
        dict_wordsidx_buffer
            .write_u32::<LittleEndian>(offset as u32)
            .map_err(|err| {
                LinderaErrorKind::Io
                    .with_error(anyhow::anyhow!(err))
                    .add_context("Failed to write word index offset to dict.wordsidx buffer")
            })?;

        // Create word details from the row data (5th column and beyond)
        let details = parser.details(row).map_err(|err| {
            LinderaErrorKind::Parse
                .with_error(anyhow::anyhow!(err))
                .add_context("Failed to parse word details from CSV row")
        })?;

        let encoded_details = E::encode(&details.iter().map(String::as_str).collect::<Vec<&str>>())
            .map_err(|err| {
                LinderaErrorKind::Serialize
                    .with_error(anyhow::anyhow!(err))
                    .add_context("Failed to encode word details")
            })?;

        // Write to dict.words buffer
        // Unlike lindera dictionary, jpreprocess dictionary does not write the length of encoded details to reduce memory consumption.
        dict_words_buffer
            .write_all(&encoded_details)
            .map_err(|err| {
                LinderaErrorKind::Serialize
                    .with_error(anyhow::anyhow!(err))
                    .add_context("Failed to write word details to dict.words buffer")
            })?;
    }

    Ok((dict_words_buffer, dict_wordsidx_buffer))
}
