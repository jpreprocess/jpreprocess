pub mod class1;
pub mod class2;
pub mod class3;
pub mod list;

use phf::{Map, PhfHash, Set};
use phf_shared::PhfBorrow;

#[derive(Debug, Clone, Copy)]
pub enum DigitType {
    Voiced,
    SemiVoiced,
}

pub type Keys = Set<&'static str>;
pub type DigitLUT = Map<&'static str, (&'static str, i32, i32)>;
pub type NumerativeLUT = Map<&'static str, DigitType>;

pub fn find_pron_conv<K1, K2, K1B, K2B, V>(
    conversion_table: &[(Set<K1B>, Map<K2B, V>)],
    key1: &K1,
    key2: &K2,
) -> Option<V>
where
    K1: PhfHash + Eq + ?Sized,
    K2: PhfHash + Eq + ?Sized,
    K1B: PhfBorrow<K1>,
    K2B: PhfBorrow<K2>,
    V: Copy,
{
    for (set, table) in conversion_table {
        if set.contains(key1) {
            return table.get(key2).copied();
        }
    }
    None
}

