use std::{fmt::Debug, str::FromStr};

use super::pos::PartOfSpeech;

#[derive(Debug, Clone, PartialEq)]
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
            "*" => Ok(Self::None),
            _ => Err(()),
        }
    }
}

// Accent sandhi rule
#[derive(Clone, PartialEq)]
pub struct ChainRule {
    pos: Option<String>,
    pub sandhi_type: AccentType,
    pub add_type: i32,
}

impl ChainRule {
    pub fn new(pos: Option<String>, sandhi_type: &str, add_type: i32) -> Self {
        Self {
            pos,
            sandhi_type: AccentType::from_str(sandhi_type).unwrap(),
            add_type,
        }
    }
}

impl Debug for ChainRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(pos) = &self.pos {
            write!(f, "{}%{:?}@{}", pos, self.sandhi_type, self.add_type)
        } else {
            if self.add_type == 0 {
                write!(f, "{:?}", self.sandhi_type)
            } else {
                write!(f, "{:?}@{}", self.sandhi_type, self.add_type)
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ChainRules {
    rules: Vec<ChainRule>,
}

impl ChainRules {
    pub fn new(rules: &str) -> Self {
        Self {
            rules: rules
                .split("/")
                .map(|rule| {
                    let mut pos: Option<String> = None;
                    let mut sandhi_type = "*".to_string();
                    let mut add_type = 0;
                    let mut is_type = false;

                    let mut process_rule = |control: Option<u8>, rule: &str| match control {
                        _ if is_type => {
                            add_type = rule.parse().unwrap();
                        }
                        Some(b'%') => {
                            pos = Some(rule.to_string());
                        }
                        Some(b'@') => {
                            sandhi_type = rule.to_string();
                            is_type = true;
                        }
                        None => {
                            sandhi_type = rule.to_string();
                        }
                        _ => unreachable!(),
                    };

                    let mut segment_start = 0;
                    while let Some(control_pos) = rule[segment_start..rule.len()].find(&['%', '@'])
                    {
                        process_rule(
                            Some(rule.as_bytes()[segment_start + control_pos]),
                            &rule[segment_start..segment_start + control_pos],
                        );
                        segment_start += control_pos + 1;
                    }
                    process_rule(None, &rule[segment_start..rule.len()]);

                    ChainRule::new(pos, sandhi_type.as_str(), add_type)
                })
                .collect(),
        }
    }

    pub fn get_rule(&self, pos: &PartOfSpeech) -> Option<&ChainRule> {
        self.rules.iter().find(|rule| {
            rule.pos
                .as_ref()
                .map_or(true, |search_pos| pos.group0_contains(search_pos.as_str()))
        })
    }
}

impl Debug for ChainRules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.rules
                .iter()
                .map(|rule| format!("{:?}", rule))
                .collect::<Vec<String>>()
                .join("/")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::njd::accent_rule::AccentType;

    use super::ChainRules;

    #[test]
    fn load_simple_rule() {
        let rule = &ChainRules::new("C3").rules[0];
        assert_eq!(rule.pos, None);
        assert_eq!(rule.sandhi_type, AccentType::C3);
        assert_eq!(rule.add_type, 0);
    }

    #[test]
    fn load_single_complex_rule() {
        let rule = &ChainRules::new("形容詞%F2@-1").rules[0];
        assert_eq!(rule.pos, Some("形容詞".to_string()));
        assert_eq!(rule.sandhi_type, AccentType::F2);
        assert_eq!(rule.add_type, -1);
    }

    #[test]
    fn load_multiple_complex_rule() {
        let rules = &ChainRules::new("特殊助動詞%F2@0/動詞%F5");
        let rule1 = &rules.rules[0];
        assert_eq!(rule1.pos, Some("特殊助動詞".to_string()));
        assert_eq!(rule1.sandhi_type, AccentType::F2);
        assert_eq!(rule1.add_type, 0);
        let rule2 = &rules.rules[1];
        assert_eq!(rule2.pos, Some("動詞".to_string()));
        assert_eq!(rule2.sandhi_type, AccentType::F5);
        assert_eq!(rule2.add_type, 0);
    }
}
