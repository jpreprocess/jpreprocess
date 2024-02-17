pub mod mora;
mod mora_dict;
mod mora_enum;
pub mod phoneme;

use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

pub use mora::*;
pub use mora_enum::*;

use crate::JPreprocessError;

pub const TOUTEN: &str = "、";
pub const QUESTION: &str = "？";
pub const QUOTATION: &str = "’";

#[macro_export]
macro_rules! pron {
    ($($x:ident),*) => {
        {
            use $crate::pronunciation::*;
            let moras = vec![
                $(
                    Mora {
                        mora_enum: MoraEnum::$x,
                        is_voiced: true,
                    },
                )*
            ];
            Pronunciation::new(moras)
        }
    };
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub struct Pronunciation(Vec<Mora>);

impl Pronunciation {
    pub fn new(moras: Vec<Mora>) -> Self {
        Self(moras)
    }

    pub fn mora_size(&self) -> usize {
        self.0
            .iter()
            .filter(|mora| !matches!(mora.mora_enum, MoraEnum::Question | MoraEnum::Touten))
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn mora_matches(&self, mora_enum: MoraEnum) -> bool {
        let Some((first, rest)) = self.0.split_first() else {
            return false;
        };
        rest.is_empty() && first.mora_enum == mora_enum
    }
    pub fn is_question(&self) -> bool {
        self.mora_matches(MoraEnum::Question)
    }
    pub fn is_touten(&self) -> bool {
        self.mora_matches(MoraEnum::Touten)
    }

    pub fn is_mora_convertable(s: &str) -> bool {
        mora_dict::MORA_STR_LIST.contains(&s)
    }

    pub fn to_pure_string(&self) -> String {
        self.0
            .iter()
            .map(|mora| mora.to_string())
            .fold(String::new(), |a, b| a + &b)
    }

    pub fn moras(&self) -> &[Mora] {
        self.0.as_slice()
    }
    pub fn moras_mut(&mut self) -> &mut [Mora] {
        self.0.as_mut_slice()
    }

    pub fn transfer_from(&mut self, from: &Self) {
        self.0.extend_from_slice(&from.0);
    }
}

impl FromStr for Pronunciation {
    type Err = JPreprocessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self(Vec::new());
        let mut current_position = 0;
        for match_result in mora_dict::MORA_DICT_AHO_CORASICK.find_iter(s) {
            if current_position != match_result.start() {
                return Err(JPreprocessError::PronunciationParseError(
                    s[current_position..match_result.start()].to_string(),
                ));
            }

            let quotation = s[match_result.end()..].starts_with(QUOTATION);

            result.0.extend(
                mora_dict::get_mora_enum(match_result.pattern().as_usize())
                    .into_iter()
                    .map(|mora_enum| Mora {
                        mora_enum,
                        is_voiced: !quotation,
                    }),
            );

            current_position = match_result.end();
            if quotation {
                current_position += QUOTATION.len();
            }
        }

        if result.0.is_empty() {
            if s == QUESTION {
                result.0.push(Mora {
                    mora_enum: MoraEnum::Question,
                    is_voiced: true,
                });
            } else if s != "*" {
                result.0.push(Mora {
                    mora_enum: MoraEnum::Touten,
                    is_voiced: true,
                });
            }
        }
        Ok(result)
    }
}

impl Display for Pronunciation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .fold(String::new(), |acc, mora| format!("{}{}", acc, mora)),
        )
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::{Mora, MoraEnum, Pronunciation};

    #[test]
    fn from_str_normal() {
        let pron = Pronunciation::from_str("オツカレサマデシ’タ").unwrap();
        assert_eq!(
            pron.0,
            vec![
                Mora {
                    mora_enum: MoraEnum::O,
                    is_voiced: true
                },
                Mora {
                    mora_enum: MoraEnum::Tsu,
                    is_voiced: true
                },
                Mora {
                    mora_enum: MoraEnum::Ka,
                    is_voiced: true
                },
                Mora {
                    mora_enum: MoraEnum::Re,
                    is_voiced: true
                },
                Mora {
                    mora_enum: MoraEnum::Sa,
                    is_voiced: true
                },
                Mora {
                    mora_enum: MoraEnum::Ma,
                    is_voiced: true
                },
                Mora {
                    mora_enum: MoraEnum::De,
                    is_voiced: true
                },
                Mora {
                    mora_enum: MoraEnum::Shi,
                    is_voiced: false
                },
                Mora {
                    mora_enum: MoraEnum::Ta,
                    is_voiced: true
                }
            ]
        )
    }

    #[test]
    fn from_str_symbol() {
        assert_eq!(
            Pronunciation::from_str("；").unwrap().0,
            vec![Mora {
                mora_enum: MoraEnum::Touten,
                is_voiced: true
            }]
        )
    }

    #[test]
    fn to_string() {
        assert_eq!(
            Pronunciation::from_str("オツカレサマデシ’タ")
                .unwrap()
                .to_string(),
            "オツカレサマデシ’タ"
        );
    }
}
