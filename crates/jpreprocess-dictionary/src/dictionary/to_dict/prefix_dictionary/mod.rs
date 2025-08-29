// This file was partially copied from the [lindera](https://github.com/lindera/lindera) project.

// MIT License
//
// Copyright (c) 2019 by the project authors.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use csv::StringRecord;
use derive_builder::Builder;
use encoding_rs::{Encoding, UTF_8};
use encoding_rs_io::DecodeReaderBytesBuilder;
use glob::glob;
use lindera_dictionary::dictionary::prefix_dictionary::PrefixDictionary;
use log::debug;

use lindera_dictionary::decompress::Algorithm;
use lindera_dictionary::error::LinderaErrorKind;
use lindera_dictionary::util::compress_write;
use lindera_dictionary::LinderaResult;

use crate::dictionary::word_encoding::DictionaryWordEncoding;

pub mod details;
pub mod parser;
pub mod word_entry;

use self::details::generate_words_files;
use self::parser::{normalize, CSVParser};
use self::word_entry::{build_word_entry_map, generate_double_array, generate_values};

#[derive(Builder)]
#[builder(name = CSVReaderOptions)]
#[builder(build_fn(name = "builder"))]
pub struct CSVReader {
    #[builder(default = "true")]
    flexible_csv: bool,
    /* If set to UTF-8, it can also read UTF-16 files with BOM. */
    #[builder(field(ty = "String", build = "get_encoding(&self.encoding)?"))]
    encoding: &'static Encoding,
    #[builder(default = "false")]
    normalize_details: bool,
}

/// Get encoding configuration
fn get_encoding(encoding_str: &str) -> Result<&'static Encoding, CSVReaderOptionsError> {
    if encoding_str.is_empty() {
        return Ok(UTF_8);
    }

    Encoding::for_label_no_replacement(encoding_str.as_bytes()).ok_or_else(|| {
        CSVReaderOptionsError::ValidationError(format!("Invalid encoding: {}", encoding_str))
    })
}

impl CSVReader {
    /// Load data from CSV files
    pub fn load_csv_data(&self, input_dir: &Path) -> LinderaResult<Vec<StringRecord>> {
        let filenames = self.collect_csv_files(input_dir)?;
        let mut rows = self.read_csv_files(&filenames)?;

        // Sort dictionary entries by the first column (word)
        // Change sorting method based on normalization settings
        if self.normalize_details {
            // Sort after normalizing characters (―→—, ～→〜)
            rows.sort_by_key(|row| normalize(&row[0]));
        } else {
            // Sort using original strings directly
            rows.sort_by(|a, b| a[0].cmp(&b[0]))
        }

        Ok(rows)
    }

    /// Collect .csv file paths from input directory
    fn collect_csv_files(&self, input_dir: &Path) -> LinderaResult<Vec<PathBuf>> {
        let pattern = if let Some(path) = input_dir.to_str() {
            format!("{path}/*.csv")
        } else {
            return Err(LinderaErrorKind::Io
                .with_error(anyhow::anyhow!("Failed to convert path to &str."))
                .add_context(format!(
                    "Input directory path contains invalid characters: {input_dir:?}"
                )));
        };

        let mut filenames: Vec<PathBuf> = Vec::new();
        for entry in glob(&pattern).map_err(|err| {
            LinderaErrorKind::Io
                .with_error(anyhow::anyhow!(err))
                .add_context(format!("Failed to glob CSV files with pattern: {pattern}"))
        })? {
            match entry {
                Ok(path) => {
                    if let Some(filename) = path.file_name() {
                        filenames.push(Path::new(input_dir).join(filename));
                    } else {
                        return Err(LinderaErrorKind::Io
                            .with_error(anyhow::anyhow!("failed to get filename"))
                            .add_context(format!("Invalid filename in path: {path:?}")));
                    };
                }
                Err(err) => {
                    return Err(LinderaErrorKind::Content
                        .with_error(anyhow!(err))
                        .add_context(format!(
                            "Failed to process glob entry with pattern: {pattern}"
                        )));
                }
            }
        }

        Ok(filenames)
    }

    /// Read CSV files
    pub fn read_csv_files(&self, filenames: &[PathBuf]) -> LinderaResult<Vec<StringRecord>> {
        let mut rows: Vec<StringRecord> = vec![];

        for filename in filenames {
            debug!("reading {filename:?}");

            let file = File::open(filename).map_err(|err| {
                LinderaErrorKind::Io
                    .with_error(anyhow::anyhow!(err))
                    .add_context(format!("Failed to open CSV file: {filename:?}"))
            })?;
            let reader: Box<dyn Read> = if self.encoding == UTF_8 {
                Box::new(file)
            } else {
                Box::new(
                    DecodeReaderBytesBuilder::new()
                        .encoding(Some(self.encoding))
                        .build(file),
                )
            };
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .flexible(self.flexible_csv)
                .from_reader(reader);

            for result in rdr.records() {
                let record = result.map_err(|err| {
                    LinderaErrorKind::Content
                        .with_error(anyhow!(err))
                        .add_context(format!("Failed to parse CSV record in file: {filename:?}"))
                })?;
                rows.push(record);
            }
        }

        Ok(rows)
    }
}

pub fn write_prefix_dictionary<P: CSVParser, E: DictionaryWordEncoding>(
    parser: &P,
    rows: &[StringRecord],
    output_dir: &Path,
    compress_algorithm: Algorithm,
) -> LinderaResult<()> {
    let word_entry_map = build_word_entry_map(parser, rows)?;

    // Write dict.da
    let mut dict_da_writer = File::create(output_dir.join("dict.da")).map_err(|err| {
        LinderaErrorKind::Io
            .with_error(anyhow::anyhow!(err))
            .add_context("Failed to create dict.da file")
    })?;
    let da = generate_double_array(&word_entry_map)?;
    compress_write(&da, compress_algorithm, &mut dict_da_writer)?;

    // Write dict.vals
    let mut dict_vals_writer = File::create(output_dir.join("dict.vals")).map_err(|err| {
        LinderaErrorKind::Io
            .with_error(anyhow::anyhow!(err))
            .add_context("Failed to create dict.vals file")
    })?;
    let vals = generate_values(&word_entry_map)?;
    compress_write(&vals, compress_algorithm, &mut dict_vals_writer)?;

    // Write dict.words and dict.wordsidx
    let mut dict_words_writer = File::create(output_dir.join("dict.words")).map_err(|err| {
        LinderaErrorKind::Io
            .with_error(anyhow::anyhow!(err))
            .add_context("Failed to create dict.words file")
    })?;
    let mut dict_wordsidx_writer =
        File::create(output_dir.join("dict.wordsidx")).map_err(|err| {
            LinderaErrorKind::Io
                .with_error(anyhow::anyhow!(err))
                .add_context("Failed to create dict.wordsidx file")
        })?;
    let (words, wordsidx) = generate_words_files::<P, E>(parser, rows)?;
    compress_write(&words, compress_algorithm, &mut dict_words_writer)?;
    compress_write(&wordsidx, compress_algorithm, &mut dict_wordsidx_writer)?;

    Ok(())
}

pub fn generate_prefix_dictionary<P: CSVParser, E: DictionaryWordEncoding>(
    parser: &P,
    rows: &[StringRecord],
    is_system: bool,
) -> LinderaResult<PrefixDictionary> {
    let word_entry_map = build_word_entry_map(parser, rows)?;

    let da = generate_double_array(&word_entry_map)?;
    let vals = generate_values(&word_entry_map)?;

    let (words, wordsidx) = generate_words_files::<P, E>(parser, rows)?;

    Ok(PrefixDictionary::load(da, vals, wordsidx, words, is_system))
}
