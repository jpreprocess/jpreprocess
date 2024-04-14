pub mod class1;
pub mod class2;
pub mod class3;
pub mod numeral;
pub mod others;

use jpreprocess_core::pronunciation::Pronunciation;
use phf::{Map, Set};

#[derive(Debug, Clone, Copy)]
pub enum DigitType {
    Voiced,
    SemiVoiced,
}

pub type Keys = Set<&'static str>;
pub type DigitLUT = Map<&'static str, Pronunciation>;
pub type NumerativeLUT = Map<&'static str, DigitType>;

pub type ConvTable<K, V> = &'static [(K, Map<&'static str, V>)];

pub fn find_pron_conv_set<V>(
    conversion_table: ConvTable<Set<&str>, V>,
    key1: &str,
    key2: &str,
) -> Option<&'static V> {
    for (haystack, table) in conversion_table {
        if haystack.contains(key1) {
            return table.get(key2);
        }
    }
    None
}

pub fn find_pron_conv_map<SK: Eq + ?Sized, V>(
    conversion_table: ConvTable<Map<&str, &[&SK]>, V>,
    key1_1: &str,
    key1_2: &SK,
    key2: &str,
) -> Option<&'static V> {
    for (haystack, table) in conversion_table {
        let Some(list) = haystack.get(key1_1) else {
            continue;
        };
        if list.contains(&key1_2) {
            return table.get(key2);
        }
    }
    None
}
