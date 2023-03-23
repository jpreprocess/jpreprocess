use jpreprocess_njd::NJD;

pub mod accent_phrase;
pub mod accent_type;
pub mod digit;
pub mod long_vowel;
pub mod pronounciation;
pub mod unvoiced_vowel;

pub fn proprocess_njd(njd: &mut NJD) {
    pronounciation::njd_set_pronunciation(njd);
    digit::njd_set_digit(njd);
    accent_phrase::njd_set_accent_phrase(njd);
    accent_type::njd_set_accent_type(njd);
    unvoiced_vowel::njd_set_unvoiced_vowel(njd);
    long_vowel::njd_set_long_vowel(njd);
}
