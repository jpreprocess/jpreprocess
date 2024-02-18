use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::JPreprocessResult;

use super::pos::POS;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum AccentRuleParseError {
    #[error("Unknown part of speech (POS) {0}")]
    UnknownPOS(String),
    #[error("Unrecognized syntax {0}")]
    SyntaxError(String),
}

static PARSE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new("^((?P<pos>名詞|形容詞|助詞|特殊助動詞|動詞)%)?(?P<accent>[FC][1-5]|P1|P2|P6|P14)?(@(?P<add>[-0-9]+))?$")
        .expect("Failed to compile accent rule regex")
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccentType {
    F1,
    F2,
    F3,
    F4,
    F5,
    //F6,
    C1,
    C2,
    C3,
    C4,
    C5,
    P1,
    P2,
    //P4,
    P6,
    //P13,
    P14,
    None,
}

impl FromStr for AccentType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "F1" => Ok(Self::F1),
            "F2" => Ok(Self::F2),
            "F3" => Ok(Self::F3),
            "F4" => Ok(Self::F4),
            "F5" => Ok(Self::F5),
            "C1" => Ok(Self::C1),
            "C2" => Ok(Self::C2),
            "C3" => Ok(Self::C3),
            "C4" => Ok(Self::C4),
            "C5" => Ok(Self::C5),
            "P1" => Ok(Self::P1),
            "P2" => Ok(Self::P2),
            "P6" => Ok(Self::P6),
            "P14" => Ok(Self::P14),
            "" | "*" => Ok(Self::None),
            _ => Err(()),
        }
    }
}

impl Display for AccentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::F1 => "F1",
            Self::F2 => "F2",
            Self::F3 => "F3",
            Self::F4 => "F4",
            Self::F5 => "F5",
            Self::C1 => "C1",
            Self::C2 => "C2",
            Self::C3 => "C3",
            Self::C4 => "C4",
            Self::C5 => "C5",
            Self::P1 => "P1",
            Self::P2 => "P2",
            Self::P6 => "P6",
            Self::P14 => "P14",
            Self::None => "*",
        })
    }
}

// Accent sandhi rule
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ChainRule {
    pub accent_type: AccentType,
    pub add_type: isize,
}

impl ChainRule {
    pub fn new(accent_type: AccentType, add_type: isize) -> Self {
        Self {
            accent_type,
            add_type,
        }
    }
}

impl Display for ChainRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.add_type == 0 {
            write!(f, "{}", self.accent_type)
        } else {
            write!(f, "{}@{}", self.accent_type, self.add_type)
        }
    }
}

#[derive(Debug)]
pub enum POSMatch {
    Default,
    Doushi,
    Joshi,
    Keiyoushi,
    Meishi,
}

impl FromStr for POSMatch {
    type Err = AccentRuleParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "動詞" => Ok(Self::Doushi),
            "助詞" => Ok(Self::Joshi),
            "形容詞" => Ok(Self::Keiyoushi),
            "名詞" => Ok(Self::Meishi),
            _ => Err(AccentRuleParseError::UnknownPOS(s.to_string())),
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct ChainRules {
    pub default: Option<ChainRule>,
    pub doushi: Option<ChainRule>,
    pub joshi: Option<ChainRule>,
    pub keiyoushi: Option<ChainRule>,
    pub meishi: Option<ChainRule>,
}

impl ChainRules {
    pub fn new(rules: &str) -> Self {
        let mut result = Self::default();
        if rules == "*" {
            return result;
        }

        for rule in rules.split('/') {
            if result.push_rule(rule).is_err() {
                eprintln!("WARN: accent rule parsing has failed in {}. Skipped.", rule);
            }
        }
        result
    }

    fn push_rule(&mut self, rule_str: &str) -> JPreprocessResult<()> {
        let (pos, rule) = Self::parse_rule(rule_str)?;
        match pos {
            POSMatch::Doushi => self.doushi.replace(rule),
            POSMatch::Joshi => self.joshi.replace(rule),
            POSMatch::Keiyoushi => self.keiyoushi.replace(rule),
            POSMatch::Meishi => self.meishi.replace(rule),
            POSMatch::Default => self.default.replace(rule),
        };
        Ok(())
    }

    fn parse_rule(rule: &str) -> JPreprocessResult<(POSMatch, ChainRule)> {
        let capture = PARSE_REGEX
            .captures(rule)
            .ok_or_else(|| AccentRuleParseError::SyntaxError(rule.to_string()))?;

        let pos = {
            if let Some(pos) = capture.name("pos") {
                POSMatch::from_str(pos.as_str())?
            } else {
                POSMatch::Default
            }
        };

        let accent_type = if let Some(matched) = capture.name("accent") {
            // This is guaranteed to success by regex
            AccentType::from_str(matched.as_str()).unwrap()
        } else {
            AccentType::None
        };

        let add_type = capture
            .name("add")
            .and_then(|matched| matched.as_str().parse().ok())
            .unwrap_or(0);

        Ok((pos, ChainRule::new(accent_type, add_type)))
    }

    pub fn get_rule(&self, pos: &POS) -> Option<&ChainRule> {
        let rule = match pos {
            POS::Doushi(_) => self.doushi.as_ref(),
            POS::Joshi(_) => self.joshi.as_ref(),
            POS::Keiyoushi(_) => self.keiyoushi.as_ref(),
            POS::Meishi(_) => self.meishi.as_ref(),
            _ => None,
        };
        rule.or(self.default.as_ref())
    }

    pub fn unset(&mut self) {
        self.default = None;
        self.doushi = None;
        self.joshi = None;
        self.keiyoushi = None;
        self.meishi = None;
    }
}

impl Display for ChainRules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = &[
            ("", &self.default),
            ("動詞", &self.doushi),
            ("助詞", &self.joshi),
            ("形容詞", &self.keiyoushi),
            ("名詞", &self.meishi),
        ]
        .iter()
        .filter(|(_name, chainrule_option)| chainrule_option.is_some())
        .fold(String::new(), |acc, (pos, chainrule_option)| {
            let chainrule = chainrule_option.as_ref().unwrap();
            let delim = if acc.is_empty() { "" } else { "/" };
            if pos.is_empty() {
                format!("{}{}{}", acc, delim, chainrule)
            } else {
                format!("{}{}{}%{}", acc, delim, pos, chainrule)
            }
        });
        if text.is_empty() {
            f.write_str("*")
        } else {
            f.write_str(text)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{accent_rule::AccentType, pos::*};

    use super::ChainRules;

    #[test]
    fn simple_rule() {
        let rules = ChainRules::new("C3");
        let rule = rules.get_rule(&POS::Others).unwrap();
        assert_eq!(rule.accent_type, AccentType::C3);
        assert_eq!(rule.add_type, 0);

        assert_eq!(rules.to_string(), "C3");
    }

    #[test]
    fn single_complex_rule() {
        let rules = ChainRules::new("形容詞%F2@-1");
        let rule = rules.get_rule(&POS::Keiyoushi(Keiyoushi::Jiritsu)).unwrap();
        assert_eq!(rule.accent_type, AccentType::F2);
        assert_eq!(rule.add_type, -1);

        assert_eq!(rules.to_string(), "形容詞%F2@-1");
    }

    #[test]
    fn multiple_complex_rule() {
        let rules = ChainRules::new("形容詞%F2@0/動詞%F5");
        let rule1 = rules.get_rule(&POS::Keiyoushi(Keiyoushi::Jiritsu)).unwrap();
        assert_eq!(rule1.accent_type, AccentType::F2);
        assert_eq!(rule1.add_type, 0);
        let rule2 = rules.get_rule(&POS::Doushi(Doushi::Jiritsu)).unwrap();
        assert_eq!(rule2.accent_type, AccentType::F5);
        assert_eq!(rule2.add_type, 0);

        assert_eq!(rules.to_string(), "動詞%F5/形容詞%F2");
    }

    #[test]
    fn reject_invalid_pos() {
        assert!(ChainRules::parse_rule("特殊助詞%F2@0").is_err());
    }

    #[test]
    fn add_type_only() {
        ChainRules::new("-1");
    }

    #[test]
    fn default_rule_1() {
        let rules = ChainRules::new("形容詞%F2/F5");
        let rule1 = rules.get_rule(&POS::Keiyoushi(Keiyoushi::Jiritsu)).unwrap();
        assert_eq!(rule1.accent_type, AccentType::F2);
        let rule2 = rules.get_rule(&POS::Doushi(Doushi::Jiritsu)).unwrap();
        assert_eq!(rule2.accent_type, AccentType::F5);

        assert_eq!(rules.to_string(), "F5/形容詞%F2");
    }

    #[test]
    fn default_rule_2() {
        let rules = ChainRules::new("F5/形容詞%F2");
        let rule1 = rules.get_rule(&POS::Keiyoushi(Keiyoushi::Jiritsu)).unwrap();
        assert_eq!(rule1.accent_type, AccentType::F2);
        let rule2 = rules.get_rule(&POS::Doushi(Doushi::Jiritsu)).unwrap();
        assert_eq!(rule2.accent_type, AccentType::F5);

        assert_eq!(rules.to_string(), "F5/形容詞%F2");
    }

    #[test]
    fn empty() {
        let rules = ChainRules::new("*");
        assert_eq!(rules.default, None);
        assert_eq!(rules.doushi, None);
        assert_eq!(rules.joshi, None);
        assert_eq!(rules.keiyoushi, None);
        assert_eq!(rules.meishi, None);

        assert_eq!(rules.to_string(), "*");
    }
}
