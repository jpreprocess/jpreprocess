use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    accent_rule::ChainRules,
    cform::CForm,
    ctype::CType,
    error::DictionaryCodecError,
    pos::{Meishi, POS},
    pronunciation::Pronunciation,
    read::{read_from_bin, read_to_bin},
    varint::{NextByteIterator, VarIntEncode},
    word_line::WordDetailsLine,
    JPreprocessResult,
};

#[derive(Debug, thiserror::Error)]
pub enum WordDetailsError {
    #[error("Unexpected end of buffer while reading POS byte")]
    POSEOBError,
    #[error("Unexpected end of buffer while reading CType byte")]
    CTypeEOBError,
    #[error("Unexpected end of buffer while reading CForm byte")]
    CFormEOBError,
    #[error("Unexpected end of buffer while reading chain_flag byte")]
    ChainFlagEOBError,
    #[error("Invalid chain_flag value in buffer: {0}")]
    InvalidChainFlagValue(u8),
}

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
        self.pron = Pronunciation::parse_csv_pron(pron, acc_morasize)
            .map_err(DictionaryCodecError::from)?;
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

    pub(crate) fn to_bin(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.push(self.pos.to_u8());
        result.push(self.ctype.to_u8());
        result.push(self.cform.to_u8());

        if let Some(read) = &self.read {
            read_to_bin(read, &mut result);
        } else {
            result.extend((-1).to_varint());
        };

        result.extend_from_slice(&self.pron.to_bin());
        result.extend_from_slice(&self.chain_rule.to_bin());
        result.push(match self.chain_flag {
            Some(true) => 1,
            Some(false) => 0,
            None => 255, // Use 255 as a sentinel value for None
        });

        result
    }

    pub(crate) fn from_bin<I: Iterator<Item = u8>>(
        iter: &mut I,
    ) -> Result<Self, DictionaryCodecError> {
        let pos = POS::from_u8(
            iter.next_byte()
                .map_err(|_| WordDetailsError::POSEOBError)?,
        );
        let ctype = CType::from_u8(
            iter.next_byte()
                .map_err(|_| WordDetailsError::CTypeEOBError)?,
        )?;
        let cform = CForm::from_u8(
            iter.next_byte()
                .map_err(|_| WordDetailsError::CFormEOBError)?,
        )?;

        let read = read_from_bin(iter)?;
        let pron = Pronunciation::from_bin(iter)?;
        let chain_rule = ChainRules::from_bin(iter)?;

        let chain_flag = match iter
            .next_byte()
            .map_err(|_| WordDetailsError::ChainFlagEOBError)?
        {
            1 => Some(true),
            0 => Some(false),
            255 => None,
            value => return Err(WordDetailsError::InvalidChainFlagValue(value).into()),
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
            ),
            ctype: CType::from_str(&value.ctype).map_err(DictionaryCodecError::from)?,
            cform: CForm::from_str(&value.cform).map_err(DictionaryCodecError::from)?,
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
            pron: Pronunciation::parse_csv_pron(&value.pron, &value.acc_morasize)
                .map_err(DictionaryCodecError::from)?,
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

#[cfg(test)]
mod tests {
    use crate::word_details::WordDetails;

    #[test]
    fn test_details_to_bin_and_from_buf() {
        let details_str = "動詞,自立,*,*,五段・ラ行,基本形,立篭る,タテコモル,タテコモル,4/5,*";
        let mut split_details: Vec<&str> = details_str.split(',').collect();
        split_details.resize(12, "*"); // Ensure the details line has 12 elements
        let details_line = crate::word_line::WordDetailsLine::from_strs(&split_details);
        let details = WordDetails::try_from(details_line).unwrap();

        let buf = details.to_bin();
        let mut buf_iter = buf.iter().copied();
        let parsed_details = WordDetails::from_bin(&mut buf_iter).unwrap();

        assert_eq!(details, parsed_details);
        assert_eq!(buf_iter.next(), None); // Ensure all bytes are consumed
    }
}
