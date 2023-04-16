use crate::{
    cform::CForm, ctype::CType, pos::*, pronounciation::Pronounciation, word_details::WordDetails,
    JPreprocessResult,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum WordEntry {
    Single(WordDetails),
    Multiple(Vec<(String, WordDetails)>),
}

impl Default for WordEntry {
    fn default() -> Self {
        Self::Single(WordDetails {
            pos: POS::Meishi(Meishi::None),
            ctype: CType::None,
            cform: CForm::None,
            read: None,
            pron: Pronounciation::default(),
            acc: 0,
            mora_size: 0,
            chain_rule: None,
            chain_flag: None,
        })
    }
}

impl WordEntry {
    pub fn load(details: &[&str]) -> JPreprocessResult<Self> {
        let orig = details[6];
        let read = details[7];
        let pron = details[8];
        let acc_morasize = details[9];

        if orig.contains(":") {
            let mut iter = orig
                .split(":")
                .zip(read.split(":"))
                .zip(pron.split(":"))
                .zip(acc_morasize.split(":"))
                .map(|(((orig, read), pron), acc_morasize)| (orig, read, pron, acc_morasize));

            let mut word_details = Vec::new();

            let (orig_base, base) = {
                let (orig, read, pron, acc_morasize) = iter.next().unwrap();
                let mut details_vec = details[0..6].to_vec();
                details_vec.push(orig);
                details_vec.push(read);
                details_vec.push(pron);
                details_vec.push(acc_morasize);
                details_vec.extend(&details[10..]);
                (
                    orig.to_string(),
                    WordDetails::load(&details_vec.as_slice())?,
                )
            };

            word_details.push((orig_base, base.clone()));

            for (orig, read, pron, acc_morasize) in iter {
                let mut extended = base.clone();
                extended.extend_splited(read, pron, acc_morasize)?;
                word_details.push((orig.to_string(), extended))
            }

            Ok(Self::Multiple(word_details))
        } else {
            Ok(Self::Single(WordDetails::load(details)?))
        }
    }

    pub fn get_with_string(&self, string: &str) -> Vec<(String, WordDetails)> {
        match &self {
            Self::Single(word_details) => vec![(string.to_string(), word_details.to_owned())],
            Self::Multiple(word_details_vec) => {
                let mut result = Vec::with_capacity(word_details_vec.len());
                let mut len = 0;
                for (i, (orig, word_details)) in word_details_vec.iter().enumerate() {
                    if i + 1 < word_details_vec.len() {
                        // not the last word
                        result.push((orig.to_string(), word_details.to_owned()));
                        len += orig.len();
                    } else {
                        // the last word
                        result.push((string[len..].to_string(), word_details.to_owned()));
                    }
                }
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        cform::CForm, ctype::CType, pos::*, pronounciation::Pronounciation, word_entry::WordEntry,
    };

    #[test]
    fn load_single_node() {
        let input: Vec<&str> = "．,名詞,接尾,助数詞,*,*,*,．,テン,テン,0/2,*,"
            .split(",")
            .collect();
        let entry = WordEntry::load(&input[1..]).unwrap();
        let details_vec = entry.get_with_string(&input[0]);
        assert_eq!(details_vec.len(), 1);

        let (string, details) = &details_vec[0];

        assert_eq!(string, "．");
        assert!(matches!(
            details.pos,
            POS::Meishi(Meishi::Setsubi(Setsubi::Josuushi))
        ));
        assert_eq!(details.ctype, CType::None);
        assert_eq!(details.cform, CForm::None);
        assert_eq!(details.read.as_ref().unwrap(), "テン");
        assert_eq!(details.pron, Pronounciation::from_str("テン").unwrap());
        assert_eq!(details.acc, 0);
        assert_eq!(details.mora_size, 2);
        assert_eq!(details.chain_rule.is_none(), true);
        assert_eq!(details.chain_flag, None);
    }

    #[test]
    fn load_multiple_nodes() {
        let input: Vec<&str> = "あーあ,感動詞,*,*,*,*,*,あー:あ,アー:ア,アー:ア,1/2:1/1,C1,"
            .split(",")
            .collect();
        let entry = WordEntry::load(&input[1..]).unwrap();
        let details_vec = entry.get_with_string(&input[0]);
        assert_eq!(details_vec.len(), 2);

        assert_eq!(details_vec[0].0, "あー");
        assert_eq!(details_vec[1].0, "あ");

        let details0 = &details_vec[0].1;
        let details1 = &details_vec[1].1;

        assert_eq!(details0.acc, 1);
        assert_eq!(details1.acc, 1);
        assert_eq!(details0.mora_size, 2);
        assert_eq!(details1.mora_size, 1);
    }
}
