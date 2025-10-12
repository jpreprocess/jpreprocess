use std::fmt::Display;

/// A struct to represent a line in a word dictionary file.
///
/// > [!WARNING]
/// > This struct is experimental and may change in the future.
#[derive(Clone, PartialEq, Debug)]
pub struct WordDetailsLine {
    pub pos: String,
    pub pos_group1: String,
    pub pos_group2: String,
    pub pos_group3: String,
    pub ctype: String,
    pub cform: String,
    pub orig: String,
    pub read: String,
    pub pron: String,
    pub acc_morasize: String,
    pub chain_rule: String,
    pub chain_flag: String,
}

impl Default for WordDetailsLine {
    fn default() -> Self {
        Self {
            pos: "名詞".to_string(),
            pos_group1: "*".to_string(),
            pos_group2: "*".to_string(),
            pos_group3: "*".to_string(),
            ctype: "*".to_string(),
            cform: "*".to_string(),
            orig: "*".to_string(),
            read: "*".to_string(),
            pron: "*".to_string(),
            acc_morasize: "*/*".to_string(),
            chain_rule: "*".to_string(),
            chain_flag: "*".to_string(),
        }
    }
}

impl WordDetailsLine {
    pub fn from_strs(details: &[&str]) -> Self {
        assert_eq!(details.len(), 12, "line must have exactly 12 columns");

        Self {
            pos: details[0].to_string(),
            pos_group1: details[1].to_string(),
            pos_group2: details[2].to_string(),
            pos_group3: details[3].to_string(),
            ctype: details[4].to_string(),
            cform: details[5].to_string(),
            orig: details[6].to_string(),
            read: details[7].to_string(),
            pron: details[8].to_string(),
            acc_morasize: details[9].to_string(),
            chain_rule: details[10].to_string(),
            chain_flag: details[11].to_string(),
        }
    }

    pub fn to_str_vec(&self, orig: String) -> [String; 12] {
        [
            self.pos.clone(),
            self.pos_group1.clone(),
            self.pos_group2.clone(),
            self.pos_group3.clone(),
            self.ctype.clone(),
            self.cform.clone(),
            orig,
            self.read.clone(),
            self.pron.clone(),
            self.acc_morasize.clone(),
            self.chain_rule.clone(),
            self.chain_flag.clone(),
        ]
    }
}

impl Display for WordDetailsLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{},{},{},{},{},{},{}",
            self.pos,
            self.pos_group1,
            self.pos_group2,
            self.pos_group3,
            self.ctype,
            self.cform,
            self.orig,
            self.read,
            self.pron,
            self.acc_morasize,
            self.chain_rule,
            self.chain_flag
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::word_details::WordDetails;

    use super::WordDetailsLine;

    #[test]
    fn default_same_with_details() {
        let default = WordDetails::default();
        let details = WordDetailsLine::default().try_into().unwrap();
        assert_eq!(default, details);
    }
}
