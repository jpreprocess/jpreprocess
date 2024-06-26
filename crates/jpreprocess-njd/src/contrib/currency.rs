//! Convert currency in forms of ¥100 or $25 to proper reading.
//!
//! Note: This filter treats `\` (backslash) as yen sign.

use jpreprocess_window::{IterQuintMutTrait, Triple};

use crate::{NJDNode, NJD};

const CURRENCY_TABLE: phf::Map<&str, &str> = phf::phf_map! {
    "￥" => "円,名詞,接尾,助数詞,*,*,*,円,エン,エン,1/2,C3,1",
    "＄" => "ドル,名詞,接尾,助数詞,*,*,*,＄,ドル,ドル,1/2,C3,1",
    "€" => "ユーロ,名詞,接尾,助数詞,*,*,*,€,ユーロ,ユーロ,1/3,C3,1",
};

pub fn process_currency(njd: &mut NJD) {
    fn is_kazu(node: &NJDNode) -> bool {
        node.get_pos().is_kazu() || node.get_string() == "．"
    }

    let mut is_currency = false;

    let mut iter = njd.iter_quint_mut();
    while let Some(quint) = iter.next() {
        let (prev, curr, next) = match quint.into() {
            Triple::Single(curr) => (None, curr, None),
            Triple::First(curr, next) => (None, curr, Some(next)),
            Triple::Last(prev, curr) => (Some(prev), curr, None),
            Triple::Full(prev, curr, next) => (Some(prev), curr, Some(next)),
        };

        if is_currency {
            if let Some(prev) = prev {
                if is_kazu(curr) {
                    std::mem::swap(prev, curr);
                } else {
                    is_currency = false;
                }
            }
        }

        if !is_kazu(curr) && next.map(|next| is_kazu(next)) == Some(true) {
            if let Some(substitute) = CURRENCY_TABLE.get(curr.get_string()) {
                *curr = NJDNode::new_single(substitute);
                is_currency = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::NJD;

    use super::process_currency;

    #[test]
    fn simple() {
        let mut njd = NJD::from_strings(vec![
            "￥,記号,*,*,*,*,*,￥,、,、,0/0,*,-1".to_string(),
            "千,名詞,数,*,*,*,*,千,セン,セン,1/2,*,0".to_string(),
        ]);
        process_currency(&mut njd);
        assert_eq!(
            njd,
            NJD::from_strings(vec![
                "千,名詞,数,*,*,*,*,千,セン,セン,1/2,*,0".to_string(),
                "円,名詞,接尾,助数詞,*,*,*,円,エン,エン,1/2,C3,1".to_string(),
            ])
        );
    }

    #[test]
    fn multiple() {
        let mut njd = NJD::from_strings(vec![
            "￥,記号,*,*,*,*,*,￥,、,、,0/0,*,-1".to_string(),
            "三,名詞,数,*,*,*,*,三,サン,サン,3/2,C3,0".to_string(),
            "千,名詞,数,*,*,*,*,千,セン,ゼン,1/2,*,1".to_string(),
            "五,名詞,数,*,*,*,*,五,ゴ,ゴ,3/1,C3,0".to_string(),
            "百,名詞,数,*,*,*,*,百,ヒャク,ヒャク,2/2,*,1".to_string(),
        ]);
        process_currency(&mut njd);
        assert_eq!(
            njd,
            NJD::from_strings(vec![
                "三,名詞,数,*,*,*,*,三,サン,サン,3/2,C3,0".to_string(),
                "千,名詞,数,*,*,*,*,千,セン,ゼン,1/2,*,1".to_string(),
                "五,名詞,数,*,*,*,*,五,ゴ,ゴ,3/1,C3,0".to_string(),
                "百,名詞,数,*,*,*,*,百,ヒャク,ヒャク,2/2,*,1".to_string(),
                "円,名詞,接尾,助数詞,*,*,*,円,エン,エン,1/2,C3,1".to_string(),
            ])
        );
    }

    #[test]
    fn complex() {
        let mut njd = NJD::from_strings(vec![
            "＄,記号,一般,*,*,*,*,＄,、,、,0/0,*,-1".to_string(),
            "五,名詞,数,*,*,*,*,五,ゴ,ゴ,1/1,C3,0".to_string(),
            "十,名詞,数,*,*,*,*,十,ジュウ,ジュー,1/2,*,1".to_string(),
            "€,記号,*,*,*,*,*,€,、,、,0/0,*,0".to_string(),
            "二,名詞,数,*,*,*,*,二,ニ,ニ,1/1,C3,0".to_string(),
            "十,名詞,数,*,*,*,*,十,ジュウ,ジュー,1/2,*,1".to_string(),
            "五,名詞,数,*,*,*,*,五,ゴ,ゴ,1/1,C3,0".to_string(),
        ]);
        process_currency(&mut njd);
        assert_eq!(
            njd,
            NJD::from_strings(vec![
                "五,名詞,数,*,*,*,*,五,ゴ,ゴ,1/1,C3,0".to_string(),
                "十,名詞,数,*,*,*,*,十,ジュウ,ジュー,1/2,*,1".to_string(),
                "ドル,名詞,接尾,助数詞,*,*,*,＄,ドル,ドル,1/2,C3,1".to_string(),
                "二,名詞,数,*,*,*,*,二,ニ,ニ,1/1,C3,0".to_string(),
                "十,名詞,数,*,*,*,*,十,ジュウ,ジュー,1/2,*,1".to_string(),
                "五,名詞,数,*,*,*,*,五,ゴ,ゴ,1/1,C3,0".to_string(),
                "ユーロ,名詞,接尾,助数詞,*,*,*,€,ユーロ,ユーロ,1/3,C3,1".to_string(),
            ])
        );
    }

    #[test]
    fn dot() {
        let mut njd = NJD::from_strings(vec![
            "＄,記号,一般,*,*,*,*,＄,、,、,0/0,*,-1".to_string(),
            "一,名詞,数,*,*,*,*,一,イチ,イッ,1/2,C3,0".to_string(),
            "．,名詞,接尾,助数詞,*,*,*,．,テン,テン,0/2,*,1".to_string(),
            "九,名詞,数,*,*,*,*,九,キュウ,キュー,3/2,*,0".to_string(),
            "五,名詞,数,*,*,*,*,五,ゴ,ゴー,1/2,*,1".to_string(),
        ]);
        process_currency(&mut njd);
        assert_eq!(
            njd,
            NJD::from_strings(vec![
                "一,名詞,数,*,*,*,*,一,イチ,イッ,1/2,C3,0".to_string(),
                "．,名詞,接尾,助数詞,*,*,*,．,テン,テン,0/2,*,1".to_string(),
                "九,名詞,数,*,*,*,*,九,キュウ,キュー,3/2,*,0".to_string(),
                "五,名詞,数,*,*,*,*,五,ゴ,ゴー,1/2,*,1".to_string(),
                "ドル,名詞,接尾,助数詞,*,*,*,＄,ドル,ドル,1/2,C3,1".to_string(),
            ])
        );
    }
}
