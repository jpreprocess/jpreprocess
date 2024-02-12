pub mod lut1;
pub mod lut2;
pub mod lut3;

use phf::{Map, Set};

pub enum DigitType {
    Voiced,
    SemiVoiced,
}

pub type Keys = Set<&'static str>;
pub type DigitLUT = Map<&'static str, (&'static str, i32, i32)>;
pub type NumerativeLUT = Map<&'static str, DigitType>;

pub fn find_digit_pron_conv(
    conversion_table: &[(Keys, DigitLUT)],
    key1: &str,
    key2: &str,
) -> Option<(&'static str, i32, i32)> {
    for (set, table) in conversion_table {
        if set.contains(key1) {
            return table.get(key2).copied();
        }
    }
    None
}

pub fn find_numerative_pron_conv(
    conversion_table: &'static [(Keys, NumerativeLUT)],
    key1: &str,
    key2: &str,
) -> Option<&'static DigitType> {
    for (set, table) in conversion_table {
        if set.contains(key1) {
            return table.get(key2);
        }
    }
    None
}
