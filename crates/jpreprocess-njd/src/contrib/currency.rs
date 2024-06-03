use jpreprocess_window::{Double, IterQuintMutTrait};

use crate::NJD;

const CURRENCY_SYMBOLS: &[&str] = &["￥", "＄", "€"];

const CURRENCY_TABLE: phf::Map<&str, &str> = phf::phf_map! {
    "￥" => "￥,記号,*,*,*,*,*,￥,エン,エン,0/0,*,-1",
    "＄" => "＄,記号,*,*,*,*,*,＄,ドル,ドル,0/0,*,-1",
    "€" => "€,記号,*,*,*,*,*,€,ユーロ,ユーロ,0/0,*,-1",
};

pub fn process_currency(njd: &mut NJD) {
    let mut is_currency = false;

    let mut iter = njd.iter_quint_mut();
    while let Some(quint) = iter.next() {
        let curr = match quint.into() {
            Double::First(curr) => curr,
            Double::Full(prev, curr) => {
                if is_currency {
                    if curr.get_pos().is_kazu() {
                        std::mem::swap(prev, curr);
                    } else {
                        is_currency = false;
                    }
                }

                curr
            }
        };

        if CURRENCY_SYMBOLS.contains(&curr.get_string()) && !curr.get_pos().is_kazu() {
            is_currency = true;
            continue;
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
                "￥,記号,*,*,*,*,*,￥,、,、,0/0,*,-1".to_string(),
            ])
        );
    }
}
