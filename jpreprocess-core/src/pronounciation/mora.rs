use super::{mora_dict::INTO_STR, mora_enum::MoraEnum, QUESTION, QUOTATION, TOUTEN};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mora {
    pub mora_enum: MoraEnum,
    pub is_unvoiced: Option<bool>,
}

impl Mora {
    pub fn to_string(&self) -> String {
        let mora = match self.mora_enum {
            MoraEnum::Question => QUESTION,
            MoraEnum::Touten => TOUTEN,
            mora_enum => INTO_STR.get(&mora_enum).unwrap(),
        };
        let suffix = match self.is_unvoiced {
            Some(true) => QUOTATION,
            _ => "",
        };
        format!("{}{}", mora, suffix)
    }
    pub fn convert_to_voiced_sound(&mut self) {
        self.mora_enum = match self.mora_enum {
            MoraEnum::Ka => MoraEnum::Ga,
            MoraEnum::Ki => MoraEnum::Gi,
            MoraEnum::Ku => MoraEnum::Gu,
            MoraEnum::Ke => MoraEnum::Ge,
            MoraEnum::Ko => MoraEnum::Go,
            MoraEnum::Sa => MoraEnum::Za,
            MoraEnum::Shi => MoraEnum::Ji,
            MoraEnum::Su => MoraEnum::Zu,
            MoraEnum::Se => MoraEnum::Ze,
            MoraEnum::So => MoraEnum::Zo,
            MoraEnum::Ta => MoraEnum::Da,
            MoraEnum::Chi => MoraEnum::Di,
            MoraEnum::Tsu => MoraEnum::Du,
            MoraEnum::Te => MoraEnum::De,
            MoraEnum::To => MoraEnum::Do,
            MoraEnum::Ha => MoraEnum::Ba,
            MoraEnum::Hi => MoraEnum::Bi,
            MoraEnum::Fu => MoraEnum::Bu,
            MoraEnum::He => MoraEnum::Be,
            MoraEnum::Ho => MoraEnum::Bo,
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
            others => others,
        }
    }
}
