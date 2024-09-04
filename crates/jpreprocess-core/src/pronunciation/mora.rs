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
    #[deprecated(
        since = "0.11.0",
        note = "This function will be replaced with phonemes_consistent in the future. Please use phonemes_openjtalk_compat instead if you want the same behavior with the previous versions."
    )]
    pub fn phonemes(&self) -> (Option<Consonant>, Option<Vowel>) {
        self.phonemes_openjtalk_compat()
    }

    /// Convert this mora to a pair of phonemes.
    ///
    /// The returned phonemes will be different from openjtalk as following.
    /// This change reflects some "い" (I) row's having a different consonant from its "あ" (A) row counterpart.
    ///
    /// - "ぎ" (Gi) -> Gy + I
    /// - "に" (Ni) -> Ny + I
    /// - "ぴ" (Pi) -> Py + I
    /// - "び" (Bi) -> By + I
    /// - "ひ" (Hi) -> Hy + I
    /// - "み" (Mi) -> My + I
    /// - "り" (Ri) -> Ry + I
    ///
    /// If you need openjtalk-compatible version, please use [`Mora::phonemes_openjtalk_compat`] instead.
    pub fn phonemes_consistent(&self) -> (Option<Consonant>, Option<Vowel>) {
        mora_to_phoneme(self)
    }

    /// Convert this mora to a pair of phonemes.
    ///
    /// This method is compatible with openjtalk, unlike [`Mora::phonemes`].
    pub fn phonemes_openjtalk_compat(&self) -> (Option<Consonant>, Option<Vowel>) {
        let (mut consonant, vowel) = self.phonemes_consistent();

        if matches!(vowel, Some(Vowel::I | Vowel::IUnvoiced)) {
            consonant = match consonant {
                Some(Consonant::Gy) => Some(Consonant::G),
                Some(Consonant::Ny) => Some(Consonant::N),
                Some(Consonant::Py) => Some(Consonant::P),
                Some(Consonant::By) => Some(Consonant::B),
                Some(Consonant::Hy) => Some(Consonant::H),
                Some(Consonant::My) => Some(Consonant::M),
                Some(Consonant::Ry) => Some(Consonant::R),
                others => others,
            };
        }

        (consonant, vowel)
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
