pub mod mora;
mod mora_dict;
mod mora_enum;

use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub use mora::*;
pub use mora_enum::*;

use crate::{error::JPreprocessErrorKind, JPreprocessError};

pub const TOUTEN: &str = "、";
pub const QUESTION: &str = "？";
pub const QUOTATION: &str = "’";

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Pronounciation(Vec<Mora>);

impl Pronounciation {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn transfer_from(&mut self, from: &Self) {
        self.0.extend_from_slice(&from.0);
    }
    pub fn mora_enums(&self) -> Vec<MoraEnum> {
        self.0.iter().map(|mora| mora.mora_enum).collect()
    }
    pub fn first_mut(&mut self) -> Option<&mut Mora> {
        self.0.first_mut()
    }
    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|mora| mora.to_string())
            .fold(String::new(), |a, b| a + &b)
    }
}

impl Default for Pronounciation {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl FromStr for Pronounciation {
    type Err = JPreprocessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self(Vec::new());
        let mut current_position = 0;
        for match_result in mora_dict::MORA_DICT_AHO_CORASICK.find_iter(s) {
            if current_position != match_result.start() {
                return Err(JPreprocessErrorKind::PronounciationParseError.with_error(
                    anyhow::anyhow!(format!(
                        "Unrecognized mora {}",
                        &s[current_position..match_result.start()]
                    ),),
                ));
            }

            let quotation = s[match_result.end()..].starts_with(QUOTATION);

            result.0.extend(
                mora_dict::get_mora_enum(match_result.pattern())
                    .into_iter()
                    .map(|mora_enum| Mora {
                        mora_enum,
                        is_voiced: if quotation { Some(false) } else { None },
                    }),
            );

            current_position = match_result.end();
            if quotation {
                current_position += QUOTATION.len();
            }
        }

        if result.0.len() == 0 {
            if s == QUESTION {
                result.0.push(Mora {
                    mora_enum: MoraEnum::Question,
                    is_voiced: Some(true),
                });
            } else if s != "*" {
                result.0.push(Mora {
                    mora_enum: MoraEnum::Touten,
                    is_voiced: Some(true),
                });
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::{Mora, MoraEnum, Pronounciation};

    #[test]
    fn from_str_normal() {
        let pron = Pronounciation::from_str("オツカレサマデシ’タ").unwrap();
        assert_eq!(
            pron.0,
            vec![
                Mora {
                    mora_enum: MoraEnum::O,
                    is_voiced: None
                },
                Mora {
                    mora_enum: MoraEnum::Tsu,
                    is_voiced: None
                },
                Mora {
                    mora_enum: MoraEnum::Ka,
                    is_voiced: None
                },
                Mora {
                    mora_enum: MoraEnum::Re,
                    is_voiced: None
                },
                Mora {
                    mora_enum: MoraEnum::Sa,
                    is_voiced: None
                },
                Mora {
                    mora_enum: MoraEnum::Ma,
                    is_voiced: None
                },
                Mora {
                    mora_enum: MoraEnum::De,
                    is_voiced: None
                },
                Mora {
                    mora_enum: MoraEnum::Shi,
                    is_voiced: Some(false)
                },
                Mora {
                    mora_enum: MoraEnum::Ta,
                    is_voiced: None
                }
            ]
        )
    }

    #[test]
    fn from_str_symbol() {
        assert_eq!(
            Pronounciation::from_str("；").unwrap().0,
            vec![Mora {
                mora_enum: MoraEnum::Touten,
                is_voiced: Some(true)
            }]
        )
    }
}
