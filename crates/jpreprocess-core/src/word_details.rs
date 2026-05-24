use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    accent_rule::ChainRules,
    cform::CForm,
    ctype::CType,
    pos::{Meishi, POS},
    pronunciation::Pronunciation,
    varint::{i32_to_varint, isize_to_varint, varint_to_i32, varint_to_isize},
    word_line::WordDetailsLine,
    JPreprocessResult,
};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct WordDetails {
    pub pos: POS,
    pub ctype: CType,
    pub cform: CForm,
    pub read: Option<String>,
    pub pron: Pronunciation,
    pub chain_rule: ChainRules,
    pub chain_flag: Option<bool>,
}

impl Default for WordDetails {
    fn default() -> Self {
        Self {
            pos: POS::Meishi(Meishi::None),
            ctype: CType::default(),
            cform: CForm::default(),
            read: None,
            pron: Pronunciation::default(),
            chain_rule: ChainRules::default(),
            chain_flag: None,
        }
    }
}

impl WordDetails {
    pub fn load(details: &[&str]) -> JPreprocessResult<Self> {
        WordDetailsLine::from_strs(details).try_into()
    }

    pub fn extend_splited(
        &mut self,
        read: &str,
        pron: &str,
        acc_morasize: &str,
    ) -> JPreprocessResult<()> {
        self.read = match read {
            "*" => None,
            _ => Some(read.to_string()),
        };
        self.pron = Pronunciation::parse_csv_pron(pron, acc_morasize)?;
        self.chain_flag = Some(false);
        Ok(())
    }

    pub fn to_str_vec(&self, orig: String) -> [String; 9] {
        let line = WordDetailsLine::from(self);

        [
            format!(
                "{},{},{},{}",
                line.pos, line.pos_group1, line.pos_group2, line.pos_group3
            ),
            line.ctype.to_string(),
            line.cform.to_string(),
            // Ideally, this should be `self.orig`, but jpreprocess njdnode does not have orig
            // and in most cases, orig is the same as string.
            orig,
            line.read.to_string(),
            line.pron.to_string(),
            line.acc_morasize.to_string(),
            line.chain_rule.to_string(),
            line.chain_flag.to_string(),
        ]
    }

    pub fn to_buf(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.push(self.pos.to_u8());
        result.push(self.ctype.to_u8());
        result.push(self.cform.to_u8());

        if let Some(read) = &self.read {
            let read_bytes = read
                .chars()
                .flat_map(|c| {
                    let diff = (c as i32) - ('ァ' as i32);
                    i32_to_varint(diff)
                })
                .collect::<Vec<u8>>();

            result.extend_from_slice(&isize_to_varint(read_bytes.len() as isize)); // isize to allow negative length for None
            result.extend_from_slice(&read_bytes);
        } else {
            result.extend_from_slice(&isize_to_varint(-1));
        };

        result.extend_from_slice(&self.pron.to_buf());
        result.extend_from_slice(&self.chain_rule.to_buf());
        result.push(match self.chain_flag {
            Some(true) => 1,
            Some(false) => 0,
            None => 255, // Use 255 as a sentinel value for None
        });

        result
    }

    pub fn from_buf(buf: &[u8]) -> JPreprocessResult<(Self, usize)> {
        let pos = POS::from_u8(buf[0]);
        let ctype = CType::from_u8(buf[1]);
        let cform = CForm::from_u8(buf[2]);

        let (read_len, read_len_size) = varint_to_isize(&buf[3..]);
        let read = if read_len >= 0 {
            let read_bytes = &buf[3 + read_len_size..3 + read_len_size + (read_len as usize)];
            let mut cursor = 0;

            let mut read_str = String::new();
            for _ in 0..read_len {
                let (diff, size) = varint_to_i32(&read_bytes[cursor..]);
                cursor += size;
                read_str.push(
                    std::char::from_u32((diff + ('ァ' as i32)) as u32)
                        .expect("Cannot parse read string from buffer"),
                );
            }

            Some(read_str)
        } else {
            None
        };

        let pron_start = 3 + read_len_size + if read_len >= 0 { read_len as usize } else { 0 };
        let (pron, pron_size) = Pronunciation::from_buf(&buf[pron_start..]);

        let chain_rule_start = pron_start + pron_size;
        let (chain_rule, chain_rule_size) = ChainRules::from_buf(&buf[chain_rule_start..]);

        let chain_flag_start = chain_rule_start + chain_rule_size;
        let chain_flag = match buf[chain_flag_start] {
            1 => Some(true),
            0 => Some(false),
            255 => None,
            _ => panic!(
                "Invalid chain_flag value in buffer: {}",
                buf[chain_flag_start]
            ),
        };

        Ok((
            Self {
                pos,
                ctype,
                cform,
                read,
                pron,
                chain_rule,
                chain_flag,
            },
            chain_flag_start + 1, // Total size of the data read
        ))
    }
}

impl TryFrom<WordDetailsLine> for WordDetails {
    type Error = crate::JPreprocessError;
    fn try_from(value: WordDetailsLine) -> Result<WordDetails, Self::Error> {
        // orig: not used

        Ok(Self {
            pos: POS::from_strs(
                &value.pos,
                &value.pos_group1,
                &value.pos_group2,
                &value.pos_group3,
            )?,
            ctype: CType::from_str(&value.ctype)?,
            cform: CForm::from_str(&value.cform)?,
            chain_rule: ChainRules::new(&value.chain_rule),
            chain_flag: match value.chain_flag.as_ref() {
                "1" => Some(true),
                "0" => Some(false),
                _ => None,
            },
            read: match value.read.as_ref() {
                "*" => None,
                _ => Some(value.read.to_string()),
            },
            pron: Pronunciation::parse_csv_pron(&value.pron, &value.acc_morasize)?,
        })
    }
}

impl From<&WordDetails> for WordDetailsLine {
    fn from(value: &WordDetails) -> Self {
        let pos = value.pos.to_string();
        let pos_parts: Vec<&str> = pos.split(',').collect();
        assert_eq!(pos_parts.len(), 4, "POS must have exactly 4 parts");

        Self {
            pos: pos_parts[0].to_string(),
            pos_group1: pos_parts[1].to_string(),
            pos_group2: pos_parts[2].to_string(),
            pos_group3: pos_parts[3].to_string(),
            ctype: value.ctype.to_string(),
            cform: value.cform.to_string(),
            orig: "*".to_string(), // orig is not stored in WordDetails
            read: value.read.as_deref().unwrap_or("*").to_string(),
            pron: value.pron.to_string(),
            acc_morasize: format!("{}/{}", value.pron.accent(), value.pron.mora_size()),
            chain_rule: value.chain_rule.to_string(),
            chain_flag: match value.chain_flag {
                Some(true) => "1",
                Some(false) => "0",
                None => "-1",
            }
            .into(),
        }
    }
}
