//! Removes "・" and "゠", which are often used in foreign proper nouns, if they are surrounded by katanaka characters.
//!
//! e.g. "バリー・ペーン" -> "バリーペーン"

use jpreprocess_core::pos::{Kigou, POS};
use jpreprocess_window::{IterQuintMutTrait, Triple};

use crate::NJD;

pub fn remove_katakana_delimiter(njd: &mut NJD) {
    let mut iter = njd.iter_quint_mut();
    while let Some(quint) = iter.next() {
        let (prev, curr, next) = match Triple::from(quint) {
            Triple::Full(prev, curr, next) => (prev, curr, next),
            _ => continue,
        };

        // Unicode Katakana block: U+30A0 - U+30FF
        // Unicode Katakana Phonetic Extensions block: U+31F0 - U+31FF
        let is_katakana = |s: &str| {
            s.chars().all(|c| {
                ('\u{30A0}'..='\u{30FF}').contains(&c) || ('\u{31F0}'..='\u{31FF}').contains(&c)
            })
        };

        if matches!(curr.get_pos(), POS::Kigou(Kigou::General))
            && ["・", "゠"].contains(&curr.get_string())
            && is_katakana(prev.get_string())
            && is_katakana(next.get_string())
        {
            curr.reset();
        }
    }

    njd.remove_silent_node();
}

#[cfg(test)]
mod tests {
    use crate::NJD;

    use super::remove_katakana_delimiter;

    #[test]
    fn simple() {
        let mut njd: NJD = [
            "バリー,フィラー,*,*,*,*,*,バリー,バリー,バリー,0/3,*,-1",
            "・,記号,一般,*,*,*,*,・,*,、,0/0,*,-1",
            "ペーン,フィラー,*,*,*,*,*,ペーン,ペーン,ペーン,0/3,*,-1",
            "は,フィラー,*,*,*,*,*,は,ハ,ハ,0/1,*,-1",
        ]
        .into_iter()
        .collect();

        remove_katakana_delimiter(&mut njd);

        assert_eq!(njd.nodes.len(), 3);
        assert_eq!(njd.nodes[0].get_string(), "バリー");
        assert_eq!(njd.nodes[1].get_string(), "ペーン");
        assert_eq!(njd.nodes[2].get_string(), "は");
    }
}
