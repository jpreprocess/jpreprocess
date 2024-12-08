use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::Write;
use std::str::FromStr;

use byteorder::{LittleEndian, WriteBytesExt};
use csv::StringRecord;
use lindera_dictionary::dictionary::prefix_dictionary::PrefixDictionary;
use log::warn;
use yada::builder::DoubleArrayBuilder;

use lindera_dictionary::error::LinderaErrorKind;
use lindera_dictionary::viterbi::{WordEntry, WordId};
use lindera_dictionary::LinderaResult;
use yada::DoubleArray;

#[derive(Debug)]
pub struct PrefixDictionaryBuilder {
    normalize_details: bool,
    skip_invalid_cost_or_id: bool,
}

impl PrefixDictionaryBuilder {
    pub fn build<F>(
        &self,
        mut rows: Vec<StringRecord>,
        row_encoder: F,
    ) -> LinderaResult<PrefixDictionary>
    where
        F: Fn(&StringRecord) -> LinderaResult<Vec<u8>>,
    {
        if self.normalize_details {
            rows.sort_by_key(|row| normalize(&row[0]));
        } else {
            rows.sort_by(|a, b| a[0].cmp(&b[0]))
        }

        let mut word_entry_map: BTreeMap<String, Vec<WordEntry>> = BTreeMap::new();

        for (row_id, row) in rows.iter().enumerate() {
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
            let key = if self.normalize_details {
                normalize(&row[0])
            } else {
                row[0].to_string()
            };
            word_entry_map.entry(key).or_default().push(WordEntry {
                word_id: WordId {
                    id: row_id as u32,
                    is_system: true,
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

            let word = row_encoder(row)?;
            dict_words_buffer
                .write(&word)
                .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;

            // let joined_details = if self.normalize_details {
            //     row.iter()
            //         .skip(4)
            //         .map(normalize)
            //         .collect::<Vec<String>>()
            //         .join("\0")
            // } else {
            //     row.iter().skip(4).collect::<Vec<&str>>().join("\0")
            // };
            // let joined_details_len = u32::try_from(joined_details.as_bytes().len())
            //     .map_err(|err| LinderaErrorKind::Serialize.with_error(anyhow::anyhow!(err)))?;

            // dict_words_buffer
            //     .write_u32::<LittleEndian>(joined_details_len)
            //     .map_err(|err| LinderaErrorKind::Serialize.with_error(anyhow::anyhow!(err)))?;
            // dict_words_buffer
            //     .write_all(joined_details.as_bytes())
            //     .map_err(|err| LinderaErrorKind::Serialize.with_error(anyhow::anyhow!(err)))?;
        }

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

        let mut dict_vals_buffer = Vec::new();
        for word_entries in word_entry_map.values() {
            for word_entry in word_entries {
                word_entry
                    .serialize(&mut dict_vals_buffer)
                    .map_err(|err| LinderaErrorKind::Serialize.with_error(anyhow::anyhow!(err)))?;
            }
        }

        Ok(PrefixDictionary {
            da: DoubleArray(dict_da_buffer),
            vals_data: dict_vals_buffer,
            words_idx_data: dict_wordsidx_buffer,
            words_data: dict_words_buffer,
            is_system: true,
        })
    }
}

fn normalize(text: &str) -> String {
    text.to_string().replace('―', "—").replace('～', "〜")
}
