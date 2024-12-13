use std::collections::BTreeMap;
use std::io::Write;
use std::str::FromStr;

use byteorder::{LittleEndian, WriteBytesExt};
use csv::StringRecord;
use derive_builder::Builder;
use log::warn;
use yada::builder::DoubleArrayBuilder;

use lindera_dictionary::error::LinderaErrorKind;
use lindera_dictionary::viterbi::{WordEntry, WordId};
use lindera_dictionary::LinderaResult;

use super::writer::{PrefixDictionaryDataType, PrefixDictionaryWriter};

#[derive(Builder, Debug)]
#[builder(name = PrefixDictionaryBuilderOptions)]
#[builder(build_fn(name = "builder"))]
pub struct PrefixDictionaryBuilder {
    #[builder(default = "false")]
    normalize_details: bool,
    #[builder(default = "false")]
    skip_invalid_cost_or_id: bool,

    #[builder(default = "false")]
    is_user_dict: bool,
    #[builder(default = "3")]
    simple_userdic_fields_num: usize,
    #[builder(default = "-10000")]
    simple_word_cost: i16,
    #[builder(default = "0")]
    simple_context_id: u16,
}

impl PrefixDictionaryBuilder {
    pub fn build<F, W>(
        &self,
        mut rows: Vec<StringRecord>,
        row_encoder: F,
        writer: &mut W,
    ) -> LinderaResult<()>
    where
        F: Fn(&[&str]) -> LinderaResult<Vec<u8>>,
        W: PrefixDictionaryWriter,
    {
        if self.normalize_details {
            rows.sort_by_key(|row| normalize(&row[0]));
        } else {
            rows.sort_by(|a, b| a[0].cmp(&b[0]))
        }

        let mut word_entry_map: BTreeMap<String, Vec<WordEntry>> = BTreeMap::new();

        for (row_id, row) in rows.iter().enumerate() {
            let surface = if self.normalize_details {
                normalize(&row[0])
            } else {
                row[0].to_string()
            };

            let (word_cost, left_id, right_id) =
                if self.is_user_dict && row.len() == self.simple_userdic_fields_num {
                    (
                        self.simple_word_cost,
                        self.simple_context_id,
                        self.simple_context_id,
                    )
                } else {
                    let word_cost = match i16::from_str(row[3].trim()) {
                        Ok(wc) => wc,
                        Err(_err) => {
                            if self.skip_invalid_cost_or_id {
                                warn!("failed to parse word_cost: {:?}", row);
                                continue;
                            } else {
                                return Err(LinderaErrorKind::Parse
                                    .with_error(anyhow::anyhow!("failed to parse word_cost")));
                            }
                        }
                    };
                    let left_id = match u16::from_str(row[1].trim()) {
                        Ok(lid) => lid,
                        Err(_err) => {
                            if self.skip_invalid_cost_or_id {
                                warn!("failed to parse left_id: {:?}", row);
                                continue;
                            } else {
                                return Err(LinderaErrorKind::Parse
                                    .with_error(anyhow::anyhow!("failed to parse left_id")));
                            }
                        }
                    };
                    let right_id = match u16::from_str(row[2].trim()) {
                        Ok(rid) => rid,
                        Err(_err) => {
                            if self.skip_invalid_cost_or_id {
                                warn!("failed to parse right_id: {:?}", row);
                                continue;
                            } else {
                                return Err(LinderaErrorKind::Parse
                                    .with_error(anyhow::anyhow!("failed to parse right_id")));
                            }
                        }
                    };

                    (word_cost, left_id, right_id)
                };

            word_entry_map.entry(surface).or_default().push(WordEntry {
                word_id: WordId {
                    id: row_id as u32,
                    is_system: !self.is_user_dict,
                },
                word_cost,
                left_id,
                right_id,
            });
        }

        let mut dict_words_buffer = Vec::new();
        let mut dict_wordsidx_buffer = Vec::new();

        for row in rows.iter() {
            let offset = dict_words_buffer.len();
            dict_wordsidx_buffer
                .write_u32::<LittleEndian>(offset as u32)
                .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;

            let details = if self.is_user_dict && row.len() == self.simple_userdic_fields_num {
                row.iter().skip(1).collect::<Vec<_>>()
            } else {
                row.iter().skip(4).collect::<Vec<_>>()
            };

            let word = row_encoder(&details)?;
            dict_words_buffer
                .write(&word)
                .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;
        }

        writer.write(PrefixDictionaryDataType::WordsIdx, &dict_wordsidx_buffer)?;
        writer.write(PrefixDictionaryDataType::Words, &dict_words_buffer)?;

        let mut id = 0u32;

        let mut keyset: Vec<(&[u8], u32)> = vec![];
        for (key, word_entries) in &word_entry_map {
            let len = word_entries.len() as u32;
            let val = (id << 5) | len; // 27bit for word ID, 5bit for different parts of speech on the same surface.
            keyset.push((key.as_bytes(), val));
            id += len;
        }

        let dict_da_buffer = DoubleArrayBuilder::build(&keyset).ok_or_else(|| {
            LinderaErrorKind::Io.with_error(anyhow::anyhow!("DoubleArray build error."))
        })?;
        writer.write(PrefixDictionaryDataType::DoubleArray, &dict_da_buffer)?;

        let mut dict_vals_buffer = Vec::new();
        for word_entries in word_entry_map.values() {
            for word_entry in word_entries {
                word_entry
                    .serialize(&mut dict_vals_buffer)
                    .map_err(|err| LinderaErrorKind::Serialize.with_error(anyhow::anyhow!(err)))?;
            }
        }
        writer.write(PrefixDictionaryDataType::Vals, &dict_vals_buffer)?;

        Ok(())
    }
}

fn normalize(text: &str) -> String {
    text.to_string().replace('―', "—").replace('～', "〜")
}
