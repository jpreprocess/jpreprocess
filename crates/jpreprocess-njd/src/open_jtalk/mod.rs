//! NJD modifiers migrated from OpenJTalk (src/njd_set*)
//!
//! Note: Long vowel estimator (`njd_set_long_vowel`) is not included here,
//! because it is deprecated and entirely commented out in OpenJTalk.

pub mod accent_phrase;
pub mod accent_type;
pub mod digit;
pub mod digit_sequence;
pub mod pronunciation;
pub mod unvoiced_vowel;
