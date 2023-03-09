use std::{fmt::Debug, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::pos::POS;

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Group0Contains {
    Meishi,
    Keiyoushi,
    Doushi,
    Joshi,
    TokushuJodoushi,
    None,
}

impl FromStr for Group0Contains {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "名詞" => Ok(Self::Meishi),
            "形容詞" => Ok(Self::Keiyoushi),
            "動詞" => Ok(Self::Doushi),
            "助詞" => Ok(Self::Joshi),
            "特殊助動詞" => Ok(Self::TokushuJodoushi),
            _ => Err(()),
        }
    }
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

// Accent sandhi rule
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ChainRule {
    pos: Option<Group0Contains>,
    pub accent_type: AccentType,
    pub add_type: i32,
}

impl ChainRule {
    pub fn new(pos: Option<Group0Contains>, accent_type: AccentType, add_type: i32) -> Self {
        Self {
            pos,
            accent_type,
            add_type,
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ChainRules {
    rules: Vec<ChainRule>,
}

impl ChainRules {
    pub fn new(rules: &str) -> Self {
        Self {
            rules: rules
                .split("/")
                .filter_map(|rule| {
                    let result = Self::parse_rule(rule);
                    if result.is_none() {
                        eprintln!("WARN: accent rule parsing has failed in {}. Skipped.", rule);
                    }
                    result
                })
                .collect(),
        }
    }

    fn parse_rule(rule: &str) -> Option<ChainRule> {
        let capture = PARSE_REGEX.captures(rule)?;

        let pos = if let Some(matched) = capture.name("pos") {
            Group0Contains::from_str(matched.as_str()).ok()
        } else {
            None
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

        Some(ChainRule::new(pos, accent_type, add_type))
    }

    pub fn get_rule(&self, pos: &POS) -> Option<&ChainRule> {
        self.rules.iter().find(|rule| {
            rule.pos
                .as_ref()
                .map_or(true, |search_pos| match search_pos {
                    Group0Contains::Doushi => matches!(pos, POS::Doushi(_)),
                    Group0Contains::Joshi => matches!(pos, POS::Joshi(_)),
                    Group0Contains::Keiyoushi => matches!(pos, POS::Keiyoushi(_)),
                    Group0Contains::Meishi => matches!(pos, POS::Meishi(_)),
                    _ => false,
                })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::accent_rule::AccentType;

    use super::{ChainRules, Group0Contains};

    #[test]
    fn load_simple_rule() {
        let rule = &ChainRules::new("C3").rules[0];
        assert_eq!(rule.pos, None);
        assert_eq!(rule.accent_type, AccentType::C3);
        assert_eq!(rule.add_type, 0);
    }

    #[test]
    fn load_single_complex_rule() {
        let rule = &ChainRules::new("形容詞%F2@-1").rules[0];
        assert_eq!(rule.pos, Some(Group0Contains::Keiyoushi));
        assert_eq!(rule.accent_type, AccentType::F2);
        assert_eq!(rule.add_type, -1);
    }

    #[test]
    fn load_multiple_complex_rule() {
        let rules = &ChainRules::new("形容詞%F2@0/動詞%F5");
        let rule1 = &rules.rules[0];
        assert_eq!(rule1.pos, Some(Group0Contains::Keiyoushi));
        assert_eq!(rule1.accent_type, AccentType::F2);
        assert_eq!(rule1.add_type, 0);
        let rule2 = &rules.rules[1];
        assert_eq!(rule2.pos, Some(Group0Contains::Doushi));
        assert_eq!(rule2.accent_type, AccentType::F5);
        assert_eq!(rule2.add_type, 0);
    }

    #[test]
    fn reject_invalid_pos() {
        assert_eq!(ChainRules::parse_rule("特殊助詞%F2@0"), None);
    }

    #[test]
    fn add_type_only() {
        ChainRules::new("-1");
    }
}
