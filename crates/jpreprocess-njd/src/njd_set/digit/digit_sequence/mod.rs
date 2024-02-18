use crate::{NJDNode, NJD};
use jpreprocess_core::pron;

mod builder;
mod score;

pub use builder::from_njd;

const NUMERAL_LIST2: &[&str] = &[
    "",
    "十,名詞,数,*,*,*,*,十,ジュウ,ジュー,1/2,*",
    "百,名詞,数,*,*,*,*,百,ヒャク,ヒャク,2/2,*",
    "千,名詞,数,*,*,*,*,千,セン,セン,1/2,*",
];
const NUMERAL_LIST3: &[&str] = &[
    "",
    "万,名詞,数,*,*,*,*,万,マン,マン,1/2,*",
    "億,名詞,数,*,*,*,*,億,オク,オク,1/2,*",
    "兆,名詞,数,*,*,*,*,兆,チョウ,チョー,1/2,C3",
    "京,名詞,数,*,*,*,*,京,ケイ,ケー,1/2,*",
    "垓,名詞,数,*,*,*,*,垓,ガイ,ガイ,1/2,*",
    "𥝱,名詞,数,*,*,*,*,𥝱,ジョ,ジョ,1/1,*",
    "穣,名詞,数,*,*,*,*,穣,ジョウ,ジョー,1/2,*",
    "溝,名詞,数,*,*,*,*,溝,コウ,コウ,1/2,*",
    "澗,名詞,数,*,*,*,*,澗,カン,カン,1/2,*",
    "正,名詞,数,*,*,*,*,正,セイ,セー,1/2,*",
    "載,名詞,数,*,*,*,*,載,サイ,サイ,1/2,*",
    "極,名詞,数,*,*,*,*,極,ゴク,ゴク,1/2,*",
    "恒河沙,名詞,数,*,*,*,*,恒河沙,ゴウガシャ,ゴウガシャ,1/4,*",
    "阿僧祇,名詞,数,*,*,*,*,阿僧祇,アソウギ,アソーギ,2/4,*",
    "那由他,名詞,数,*,*,*,*,那由他,ナユタ,ナユタ,1/3,*",
    "不可思議,名詞,数,*,*,*,*,不可思議,フカシギ,フカシギ,2/4,*",
    "無量大数,名詞,数,*,*,*,*,無量大数,ムリョウタイスウ,ムリョータイスー,6/7,*",
];

#[derive(Debug)]
pub struct DigitSequence {
    start: usize,
    end: usize,
    digits: Vec<u8>,
    is_numerical_reading: Option<bool>,
}

impl DigitSequence {
    pub fn new(
        start: usize,
        end: usize,
        digits: Vec<u8>,
        is_numerical_reading: Option<bool>,
    ) -> Self {
        Self {
            start,
            end,
            digits,
            is_numerical_reading,
        }
    }

    pub fn estimate_numerical_reading(&mut self, njd: &NJD) {
        if self.is_numerical_reading.is_none() {
            self.is_numerical_reading = Some(score::score(njd, self.start, self.end) >= 0);
        }
    }

    pub fn convert(&mut self, njd: &mut NJD, offset: i64) -> i64 {
        self.start = (self.start as i64 + offset) as usize;
        self.end = (self.end as i64 + offset) as usize;
        if self.is_numerical_reading.unwrap() {
            self.convert_for_numerical_reading(njd)
        } else {
            self.convert_for_non_numerical_reading(njd);
            0
        }
    }
    fn convert_for_non_numerical_reading(&self, njd: &mut NJD) {
        for (i, (node, digit)) in njd.nodes[self.start..]
            .iter_mut()
            .zip(self.digits.iter())
            .enumerate()
        {
            match *digit {
                0 => {
                    node.set_pron(pron!([Ze, Ro], 1));
                }
                2 => {
                    node.set_pron(pron!([Ni, Long], 1));
                }
                5 => {
                    node.set_pron(pron!([Go, Long], 1));
                }
                _ => (),
            }
            node.unset_chain_rule();
            if i % 2 == 0 {
                node.set_chain_flag(false);
                if i != self.digits.len() - 1 {
                    /* if this is not the last digit */
                    node.get_pron_mut().set_accent(3);
                }
            } else {
                node.set_chain_flag(true);
            }
        }
    }

    fn convert_for_numerical_reading(&self, njd: &mut NJD) -> i64 {
        /* first remove commas */
        let mut offset_comma = 0;
        let mut idx = 0;
        njd.nodes.retain(|node| {
            if idx < self.start || self.end < idx {
                idx += 1;
                return true;
            } else {
                idx += 1;
            }
            if node.get_string() == "，" {
                offset_comma += 1;
                false
            } else {
                true
            }
        });

        if self.digits.len() > NUMERAL_LIST3.len() * 4 {
            /* the number is too large */
            return offset_comma;
        }

        /* whether any digit is in the block. e.g. 1[0000]->false,1[1000]->true */
        let mut have_digit_in_block = false;

        let mut offset = 0;

        /* convert digits */
        for (i, digit) in self.digits.iter().enumerate() {
            let nodes_index = self.start + i + offset;
            let rev_index = self.digits.len() - i - 1;

            if *digit == 0 {
                let node = &mut njd.nodes[nodes_index];
                node.reset();
            } else {
                have_digit_in_block = true;
            }

            if rev_index % 4 == 0 {
                if have_digit_in_block && rev_index > 0 {
                    njd.nodes.insert(
                        nodes_index + 1,
                        NJDNode::new_single(NUMERAL_LIST3[rev_index / 4]),
                    );
                    offset += 1;
                }
                have_digit_in_block = false;
            } else {
                match *digit {
                    0 => (),
                    1 => njd.nodes[nodes_index] = NJDNode::new_single(NUMERAL_LIST2[rev_index % 4]),
                    _ => {
                        njd.nodes.insert(
                            nodes_index + 1,
                            NJDNode::new_single(NUMERAL_LIST2[rev_index % 4]),
                        );
                        offset += 1;
                    }
                }
            }
        }

        offset as i64 - offset_comma
    }
}
