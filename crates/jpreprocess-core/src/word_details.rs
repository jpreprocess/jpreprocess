use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    accent_rule::ChainRules,
    cform::CForm,
    ctype::CType,
    pos::{Meishi, POS},
    pronunciation::Pronunciation,
    varint::{i32_to_varint, isize_to_varint, read_u8, varint_to_i32, varint_to_isize},
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
            result.extend_from_slice(&isize_to_varint(read.len() as isize)); // isize to allow negative length for None

            let read_bytes = read
                .chars()
                .flat_map(|c| {
                    let diff = (c as i32) - 0x30CD; // 0x30CD: 'ネ' = (0x30A1 'ァ' + 0x30FA 'ヺ') / 2
                    i32_to_varint(diff)
                })
                .collect::<Vec<u8>>();
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

    pub fn from_iter<I: Iterator<Item = u8>>(iter: &mut I) -> JPreprocessResult<Self> {
        let pos = POS::from_u8(read_u8(iter));
        let ctype = CType::from_u8(read_u8(iter));
        let cform = CForm::from_u8(read_u8(iter));

        let read_len = varint_to_isize(iter);
        let read = if read_len >= 0 {
            let mut read_str = String::with_capacity(read_len as usize);

            while read_str.len() < read_len as usize {
                let diff = varint_to_i32(iter);
                read_str.push(
                    std::char::from_u32((diff + 0x30CD) as u32)
                        .expect("Cannot parse read string from buffer"),
                );
            }

            Some(read_str)
        } else {
            None
        };

        let pron = Pronunciation::from_iter(iter);
        let chain_rule = ChainRules::from_iter(iter);

        let chain_flag = match read_u8(iter) {
            1 => Some(true),
            0 => Some(false),
            255 => None,
            value => panic!("Invalid chain_flag value in buffer: {}", value),
        };

        Ok(Self {
            pos,
            ctype,
            cform,
            read,
            pron,
            chain_rule,
            chain_flag,
        })
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
