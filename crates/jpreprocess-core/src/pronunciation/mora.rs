use std::fmt::Display;

use super::{
    mora_dict::INTO_STR,
    mora_enum::MoraEnum,
    phoneme::{mora_to_phoneme, Consonant, Vowel},
    QUESTION, QUOTATION, TOUTEN,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mora {
    pub mora_enum: MoraEnum,
    pub is_voiced: bool,
}

impl Mora {
    pub fn phonemes(&self) -> (Option<Consonant>, Option<Vowel>) {
        mora_to_phoneme(self)
    }

    pub fn convert_to_voiced_sound(&mut self) {
        self.mora_enum = match self.mora_enum {
            MoraEnum::Ka => MoraEnum::Ga,
            MoraEnum::Ki => MoraEnum::Gi,
            MoraEnum::Ku => MoraEnum::Gu,
            MoraEnum::Ke => MoraEnum::Ge,
            MoraEnum::Ko => MoraEnum::Go,
            MoraEnum::Kya => MoraEnum::Gya,
            MoraEnum::Kyu => MoraEnum::Gyu,
            MoraEnum::Kyo => MoraEnum::Gyo,
            MoraEnum::Kye => MoraEnum::Gye,
            MoraEnum::Sa => MoraEnum::Za,
            MoraEnum::Shi => MoraEnum::Ji,
            MoraEnum::Su => MoraEnum::Zu,
            MoraEnum::Se => MoraEnum::Ze,
            MoraEnum::So => MoraEnum::Zo,
            MoraEnum::Swi => MoraEnum::Zwi,
            MoraEnum::Sha => MoraEnum::Ja,
            MoraEnum::Shu => MoraEnum::Ju,
            MoraEnum::Sho => MoraEnum::Jo,
            MoraEnum::She => MoraEnum::Je,
            MoraEnum::Ta => MoraEnum::Da,
            MoraEnum::Chi => MoraEnum::Di,
            MoraEnum::Tsu => MoraEnum::Du,
            MoraEnum::Te => MoraEnum::De,
            MoraEnum::To => MoraEnum::Do,
            MoraEnum::Tha => MoraEnum::Dha,
            MoraEnum::Thi => MoraEnum::Dhi,
            MoraEnum::Thu => MoraEnum::Dhu,
            MoraEnum::Twu => MoraEnum::Dwu,
            MoraEnum::Tho => MoraEnum::Dho,
            MoraEnum::Ha => MoraEnum::Ba,
            MoraEnum::Hi => MoraEnum::Bi,
            MoraEnum::Fu => MoraEnum::Bu,
            MoraEnum::He => MoraEnum::Be,
            MoraEnum::Ho => MoraEnum::Bo,
            MoraEnum::Hya => MoraEnum::Bya,
            MoraEnum::Hyu => MoraEnum::Byu,
            MoraEnum::Hye => MoraEnum::Bye,
            MoraEnum::Hyo => MoraEnum::Byo,
            others => others,
        }
    }
    pub fn convert_to_semivoiced_sound(&mut self) {
        self.mora_enum = match self.mora_enum {
            MoraEnum::Ha => MoraEnum::Pa,
            MoraEnum::Hi => MoraEnum::Pi,
            MoraEnum::Fu => MoraEnum::Pu,
            MoraEnum::He => MoraEnum::Pe,
            MoraEnum::Ho => MoraEnum::Po,
            MoraEnum::Hya => MoraEnum::Pya,
            MoraEnum::Hyu => MoraEnum::Pyu,
            MoraEnum::Hye => MoraEnum::Pye,
            MoraEnum::Hyo => MoraEnum::Pyo,
            others => others,
        }
    }
}

impl Display for Mora {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mora = match self.mora_enum {
            MoraEnum::Question => QUESTION,
            MoraEnum::Touten => TOUTEN,
            mora_enum => INTO_STR.get(&mora_enum).unwrap(),
        };
        let suffix = if self.is_voiced { "" } else { QUOTATION };
        write!(f, "{}{}", mora, suffix)
    }
}
