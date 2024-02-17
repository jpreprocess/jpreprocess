pub mod mora;
mod mora_dict;
mod mora_enum;
pub mod phoneme;

use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub use mora::*;
pub use mora_enum::*;

use crate::{JPreprocessError, JPreprocessResult};

pub const TOUTEN: &str = "、";
pub const QUESTION: &str = "？";
pub const QUOTATION: &str = "’";

#[macro_export]
macro_rules! pron {
    ([$($x:ident),*],$acc:expr) => {
        {
            let moras = vec![
                $(
                    $crate::pronunciation::Mora {
                        mora_enum: $crate::pronunciation::MoraEnum::$x,
                        is_voiced: true,
                    },
                )*
            ];
            $crate::pronunciation::Pronunciation::new(moras, $acc)
        }
    };
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub struct Pronunciation {
    moras: Vec<Mora>,
    accent: usize,
}

impl Pronunciation {
    pub fn new(moras: Vec<Mora>, accent: usize) -> Self {
        Self { moras, accent }
    }

    pub fn mora_size(&self) -> usize {
        self.moras
            .iter()
            .filter(|mora| !matches!(mora.mora_enum, MoraEnum::Question | MoraEnum::Touten))
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.moras.is_empty()
    }

    pub fn mora_matches(&self, mora_enum: MoraEnum) -> bool {
        let Some((first, rest)) = self.moras.split_first() else {
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
        self.moras
            .iter()
            .map(|mora| mora.to_string())
            .fold(String::new(), |a, b| a + &b)
    }

    pub fn moras(&self) -> &[Mora] {
        self.moras.as_slice()
    }
    pub fn moras_mut(&mut self) -> &mut [Mora] {
        self.moras.as_mut_slice()
    }

    pub fn transfer_from(&mut self, from: &Self) {
        self.moras.extend_from_slice(&from.moras);
    }

    pub fn parse(moras: &str, accent: usize) -> JPreprocessResult<Self> {
        Ok(Self::new(Self::parse_mora_str(moras)?, accent))
    }
    /// TODO: remove this
    #[doc(hidden)]
    pub fn set_mora_by_str(&mut self, moras: &str) -> JPreprocessResult<()> {
        self.moras = Self::parse_mora_str(moras)?;
        Ok(())
    }
    fn parse_mora_str(s: &str) -> JPreprocessResult<Vec<Mora>> {
        let mut result = Vec::new();
        let mut current_position = 0;
        for match_result in mora_dict::MORA_DICT_AHO_CORASICK.find_iter(s) {
            if current_position != match_result.start() {
                return Err(JPreprocessError::PronunciationParseError(
                    s[current_position..match_result.start()].to_string(),
                ));
            }

            let quotation = s[match_result.end()..].starts_with(QUOTATION);

            result.extend(
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

        if result.is_empty() {
            if s == QUESTION {
                result.push(Mora {
                    mora_enum: MoraEnum::Question,
                    is_voiced: true,
                });
            } else if s != "*" {
                result.push(Mora {
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
                .moras
                .iter()
                .fold(String::new(), |acc, mora| format!("{}{}", acc, mora)),
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Mora, MoraEnum, Pronunciation};

    #[test]
    fn from_str_normal() {
        let pron = Pronunciation::parse_mora_str("オツカレサマデシ’タ").unwrap();
        assert_eq!(
            pron,
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
            Pronunciation::parse_mora_str("；").unwrap(),
            vec![Mora {
                mora_enum: MoraEnum::Touten,
                is_voiced: true
            }]
        )
    }

    #[test]
    fn to_string() {
        assert_eq!(
            Pronunciation::parse("オツカレサマデシ’タ", 0)
                .unwrap()
                .to_string(),
            "オツカレサマデシ’タ"
        );
    }
}
