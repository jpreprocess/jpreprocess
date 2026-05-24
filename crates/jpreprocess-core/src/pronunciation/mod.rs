pub mod mora;
mod mora_dict;
mod mora_enum;
pub mod phoneme;

use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display, ops::Range};

pub use mora::*;
pub use mora_enum::*;

use crate::varint::{usize_to_varint, varint_to_usize};

pub const TOUTEN: &str = "、";
pub const QUESTION: &str = "？";
pub const QUOTATION: &str = "’";

#[macro_export]
macro_rules! pron {
    ([$($x:ident),*],$acc:expr) => {
        {
            $crate::pronunciation::Pronunciation {
                moras: ::std::borrow::Cow::Borrowed(&[
                    $(
                        $crate::pronunciation::Mora {
                            mora_enum: $crate::pronunciation::MoraEnum::$x,
                            is_voiced: true,
                        },
                    )*
                ]),
                accent: $acc,
            }
        }
    };
}

#[derive(Debug, thiserror::Error)]
pub enum PronunciationParseError {
    #[error("`{0}` could not be parsed as mora")]
    UnknownMora(String),
    #[error("Provided mora size {0} is different from that of calculated from pronunciation {1}")]
    MoraSizeMismatch(usize, usize),
    #[error("Failed to parse as integer: {0}")]
    NumberParseError(#[from] std::num::ParseIntError),
}

/// Pronunciation.
///
/// Do not access moras and accent directly unless through [`pron`] macro.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub struct Pronunciation {
    #[doc(hidden)]
    pub moras: Cow<'static, [Mora]>,
    #[doc(hidden)]
    pub accent: usize,
}

impl Pronunciation {
    pub fn new(moras: Vec<Mora>, accent: usize) -> Self {
        Self {
            moras: Cow::Owned(moras),
            accent,
        }
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

    #[inline]
    pub fn moras(&self) -> &[Mora] {
        self.moras.as_ref()
    }
    #[inline]
    pub fn moras_mut(&mut self) -> &mut [Mora] {
        self.moras.to_mut()
    }

    pub fn accent(&self) -> usize {
        self.accent
    }
    pub fn set_accent(&mut self, accent: usize) {
        self.accent = accent;
    }

    pub fn transfer_from(&mut self, from: &Self) {
        let moras = self
            .moras()
            .iter()
            .chain(from.moras())
            .cloned()
            .collect::<Vec<_>>();
        self.moras = Cow::Owned(moras);
    }

    pub fn to_buf(&self) -> Vec<u8> {
        let len = self.moras.len();
        let voiced_flag_len = len.div_ceil(8);

        let len_encoded = usize_to_varint(len);
        let accent_encoded = usize_to_varint(self.accent);
        let mut result =
            Vec::with_capacity(len_encoded.len() + accent_encoded.len() + len + voiced_flag_len);

        result.extend_from_slice(&len_encoded);
        result.extend_from_slice(&accent_encoded);

        for mora in self.moras.iter() {
            result.push(mora.mora_enum.to_u8());
        }

        for i in 0..voiced_flag_len {
            let mut flag_tmp = 0u8;
            for j in 0..8 {
                if i * 8 + j > len {
                    break;
                }

                let is_voiced = self.moras[i * 8 + j].is_voiced as u8;
                flag_tmp |= is_voiced << j;
            }
            result.push(flag_tmp);
        }

        result
    }

    pub fn from_buf(buf: &[u8]) -> (Self, usize) {
        let (len, cursor) = varint_to_usize(buf);
        let (accent, cursor) = varint_to_usize(&buf[cursor..]);

        let mut moras = buf[cursor..cursor + len]
            .iter()
            .map(|&v| Mora {
                mora_enum: MoraEnum::from_u8(v),
                is_voiced: false,
            })
            .collect::<Vec<_>>();
        let cursor = cursor + len;

        let voiced_flag_len = len.div_ceil(8);
        for (i, &flag) in buf[cursor..cursor + voiced_flag_len].iter().enumerate() {
            for j in 0..8 {
                if i * 8 + j > len {
                    break;
                }

                let is_voiced = flag & (0x1 << j);
                moras[i * 8 + j].is_voiced = is_voiced > 0;
            }
        }
        let cursor = cursor + voiced_flag_len;

        (
            Self {
                moras: moras.into(),
                accent,
            },
            cursor,
        )
    }
}

impl Pronunciation {
    pub(crate) fn parse_csv_pron(
        pron: &str,
        acc_morasize: &str,
    ) -> Result<Self, PronunciationParseError> {
        let (accent, mora_size) = match acc_morasize.split_once('/') {
            Some(("*" | "", "*" | "")) => (None, None),
            Some((acc, mora_size)) => (Some(acc.parse()?), Some(mora_size.parse()?)),
            None => match acc_morasize {
                "*" | "" => (None, None),
                acc => (Some(acc.parse()?), None),
            },
        };
        let pronunciation = Self::parse(pron, accent.unwrap_or(0))?;

        if let Some(mora_size) = mora_size {
            if pronunciation.mora_size() != mora_size {
                return Err(PronunciationParseError::MoraSizeMismatch(
                    mora_size,
                    pronunciation.mora_size(),
                ));
            }
        }

        Ok(pronunciation)
    }

    pub fn parse(moras: &str, accent: usize) -> Result<Self, PronunciationParseError> {
        let parsed = Self::parse_mora_str(moras);
        let result = if parsed.len() > 1 {
            let range = parsed[1].0.clone();
            return Err(PronunciationParseError::UnknownMora(
                moras[range].to_string(),
            ));
        } else {
            parsed.first().cloned().unwrap_or_default().1
        };

        Ok(Self::new(result, accent))
    }

    pub fn parse_mora_str(s: &str) -> Vec<(Range<usize>, Vec<Mora>)> {
        if s == "*" {
            return vec![];
        } else if s == QUESTION {
            return vec![(
                0..QUESTION.len(),
                vec![Mora {
                    mora_enum: MoraEnum::Question,
                    is_voiced: true,
                }],
            )];
        }

        let mut result = Vec::new();

        let mut segment_start_point = 0;
        let mut current_moras = Vec::new();
        let mut current_position = 0;
        for match_result in mora_dict::MORA_DICT_AHO_CORASICK.find_iter(s) {
            if current_position != match_result.start() {
                if !current_moras.is_empty() {
                    result.push((segment_start_point..current_position, current_moras.clone()));
                    current_moras.clear();
                    segment_start_point = current_position;
                }

                result.push((
                    segment_start_point..match_result.start(),
                    vec![Mora {
                        mora_enum: MoraEnum::Touten,
                        is_voiced: true,
                    }],
                ));
                segment_start_point = match_result.start();
            }

            let quotation = s[match_result.end()..].starts_with(QUOTATION);

            current_moras.extend(
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

        if !current_moras.is_empty() {
            result.push((segment_start_point..current_position, current_moras));
        }
        if current_position != s.len() {
            result.push((
                current_position..s.len(),
                vec![Mora {
                    mora_enum: MoraEnum::Touten,
                    is_voiced: true,
                }],
            ));
        }

        result
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
    fn parse_normal() {
        let pron = Pronunciation::parse_mora_str("オツカレサマデシ’タ");
        assert_eq!(
            pron,
            vec![(
                0..30,
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
            )]
        )
    }

    #[test]
    fn parse_symbol() {
        assert_eq!(
            Pronunciation::parse_mora_str("；"),
            vec![(
                0..3,
                vec![Mora {
                    mora_enum: MoraEnum::Touten,
                    is_voiced: true
                }]
            )]
        )
    }

    #[test]
    fn parse_empty() {
        assert_eq!(Pronunciation::parse_mora_str(""), vec![])
    }

    #[test]
    fn parse_multiple_segments() {
        let pron = Pronunciation::parse_mora_str("バリー・ペーン，");
        assert_eq!(
            pron,
            vec![
                (
                    0..9,
                    vec![
                        Mora {
                            mora_enum: MoraEnum::Ba,
                            is_voiced: true
                        },
                        Mora {
                            mora_enum: MoraEnum::Ri,
                            is_voiced: true
                        },
                        Mora {
                            mora_enum: MoraEnum::Long,
                            is_voiced: true
                        },
                    ]
                ),
                (
                    9..12,
                    vec![Mora {
                        mora_enum: MoraEnum::Touten,
                        is_voiced: true
                    },]
                ),
                (
                    12..21,
                    vec![
                        Mora {
                            mora_enum: MoraEnum::Pe,
                            is_voiced: true
                        },
                        Mora {
                            mora_enum: MoraEnum::Long,
                            is_voiced: true
                        },
                        Mora {
                            mora_enum: MoraEnum::N,
                            is_voiced: true
                        }
                    ]
                ),
                (
                    21..24,
                    vec![Mora {
                        mora_enum: MoraEnum::Touten,
                        is_voiced: true
                    }]
                )
            ]
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
        assert_eq!(Pronunciation::parse("？", 0).unwrap().to_string(), "？");
        assert_eq!(Pronunciation::parse("．？", 0).unwrap().to_string(), "、");
        assert_eq!(Pronunciation::parse("*", 0).unwrap().to_string(), "");
    }
}
