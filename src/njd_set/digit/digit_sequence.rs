use crate::njd::{
    node::NJDNode,
    pos::{Group1, Group2},
    Double, NJD,
};

use super::rule;

#[derive(Debug)]
pub struct DigitSequence {
    start: usize,
    end: usize,
    insertion_list: Vec<(usize, Option<NJDNode>)>,
}

impl DigitSequence {
    pub fn from_njd(njd: &NJD) -> Vec<Self> {
        let mut result: Vec<Self> = Vec::new();
        let node_len = njd.nodes.len();
        let mut s: Option<usize> = None;
        let mut e: Option<usize> = None;
        for (i, node) in njd.nodes.iter().enumerate() {
            if Self::is_digit(node) {
                if s.is_none() {
                    s = Some(i);
                }
                if i == node_len - 1 {
                    e = Some(i);
                }
            } else {
                if s.is_some() {
                    e = Some(i - 1);
                }
            }
            if let (Some(start), Some(end)) = (s, e) {
                result.push(Self {
                    start,
                    end,
                    insertion_list: Vec::new(),
                });
                s = None;
                e = None;
            }
        }
        result
    }
    pub fn to_njd(njd: &mut NJD, sequences: Vec<Self>) {
        let mut offset: i64 = 0;
        for mut seq in sequences {
            seq.insertion_list
                .sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            for (index, node) in seq.insertion_list {
                let tmp_index: usize = ((index as i64) + offset) as usize;
                if let Some(node) = node {
                    njd.nodes.insert(tmp_index + 1, node);
                    offset += 1;
                } else {
                    njd.nodes.remove(tmp_index);
                    offset -= 1;
                }
            }
        }
    }

    fn is_digit(node: &NJDNode) -> bool {
        matches!(node.get_pos().get_group1(), Group1::Kazu)
            && (rule::DIGITS.contains_key(node.get_string())
                || Self::is_period(node.get_string())
                || Self::is_comma(node.get_string()))
    }
    fn is_period(s: &str) -> bool {
        matches!(s, rule::TEN1 | rule::TEN2)
    }
    fn is_comma(s: &str) -> bool {
        matches!(s, rule::COMMA)
    }
    fn get_digit(node: &NJDNode) -> Option<u8> {
        if !node.get_string().is_empty() && matches!(node.get_pos().get_group1(), Group1::Kazu) {
            if let Some(digit) = rule::DIGITS.get(node.get_string()) {
                return Some(*digit);
            }
        }
        None
    }

    pub fn convert_digit_sequence(&mut self, njd: &mut NJD) {
        #[derive(Debug)]
        enum NumericalReading {
            Numerical,
            Unknown,
            NonNumerical,
        }
        let mut numerical_reading = NumericalReading::Numerical;

        if Self::is_comma(njd.nodes[self.start].get_string())
            || Self::is_period(njd.nodes[self.start].get_string())
        {
            if self.start != self.end && self.start + 1 < njd.nodes.len() {
                self.start += 1;
                self.convert_digit_sequence(njd);
            }
            return;
        }

        /* find final digit before period */
        let final_digit_before_period = {
            let before_period = njd.nodes[self.start..self.end]
                .iter()
                .position(|node| Self::is_period(node.get_string()))
                .map(|postion| self.start + postion - 1)
                .unwrap_or(self.end);
            njd.nodes[self.start..before_period + 1]
                .iter()
                .rev()
                .position(|node| !Self::is_comma(node.get_string()))
                .map(|postion| before_period - postion)
                .unwrap_or(self.start)
        };

        /* check commas */
        let (first_comma_before_period, num_comma) = {
            let mut first_comma_before_period: Option<usize> = None;
            let mut num_comma = 0;
            for (i, node) in njd.nodes[self.start..final_digit_before_period + 1]
                .iter()
                .rev()
                .enumerate()
            {
                if Self::is_comma(node.get_string()) {
                    first_comma_before_period = Some(i);
                    num_comma += 1;
                    if matches!(numerical_reading, NumericalReading::Numerical) && i % 4 != 3 {
                        numerical_reading = NumericalReading::Unknown;
                    }
                } else if matches!(numerical_reading, NumericalReading::Numerical) && i % 4 == 3 {
                    numerical_reading = NumericalReading::Unknown;
                }
            }
            (
                first_comma_before_period.map(|p| final_digit_before_period - p),
                num_comma,
            )
        };

        /* check zero-start */
        if self.start != final_digit_before_period
            && matches!(Self::get_digit(&njd.nodes[self.start]), Some(0))
        {
            numerical_reading = NumericalReading::NonNumerical;
        }

        /* if no info, set unknown flag */
        if matches!(numerical_reading, NumericalReading::Numerical) && num_comma == 0 {
            numerical_reading = NumericalReading::Unknown;
        }

        match numerical_reading {
            NumericalReading::Numerical => {
                /* numerical reading until period */
                let added = Self::convert_digit_sequence_for_numerical_reading(
                    njd,
                    self.start,
                    final_digit_before_period,
                );
                self.insertion_list.extend(added);
                if final_digit_before_period < self.end {
                    self.start = final_digit_before_period + 1;
                    self.convert_digit_sequence(njd);
                }
            }
            _ => {
                let final_digit = if let Some(p) = first_comma_before_period {
                    p - 1
                } else {
                    final_digit_before_period
                };

                if matches!(numerical_reading, NumericalReading::Unknown) {
                    numerical_reading =
                        if Self::get_digit_sequence_score(njd, self.start, final_digit) >= 0 {
                            NumericalReading::Numerical
                        } else {
                            NumericalReading::NonNumerical
                        }
                }

                match numerical_reading {
                    NumericalReading::Numerical => {
                        /* numerical reading until comma */
                        let added = Self::convert_digit_sequence_for_numerical_reading(
                            njd,
                            self.start,
                            final_digit,
                        );
                        self.insertion_list.extend(added);
                    }
                    _ => {
                        /* non-numerical reading */
                        Self::convert_digit_sequence_for_non_numerical_reading(
                            njd,
                            self.start,
                            final_digit,
                        );
                    }
                };

                if final_digit < self.end {
                    self.start = final_digit + 1;
                    self.convert_digit_sequence(njd);
                }
            }
        }
    }

    fn get_digit_sequence_score(njd: &NJD, start: usize, end: usize) -> i32 {
        let mut score = 0;
        if start > 0 {
            let (pos, string) = {
                let node = &njd.nodes[start - 1];
                (node.get_pos(), node.get_string())
            };
            score += match (pos.get_group1(), pos.get_group2()) {
                (Group1::Suusetsuzoku, Group2::Josuushi) => 3,
                (Group1::Suusetsuzoku, _) => 2,
                (Group1::FukushiKanou, _) => 1,
                (_, Group2::Josuushi) => 1,
                _ => 0,
            };
            if Self::is_period(string) {
                if start > 1
                    && matches!(njd.nodes.get(start-2),Some(node) if node.get_pos().get_group1()==Group1::Kazu)
                {
                    score -= 5;
                }
            } else {
                score += match string {
                    rule::HAIHUN1 => -2,
                    rule::HAIHUN2 => -2,
                    rule::HAIHUN3 => -2,
                    rule::HAIHUN4 => -2,
                    rule::HAIHUN5 => -2,
                    rule::KAKKO1 => match njd.nodes.get(start - 2) {
                        Some(node) if node.get_pos().get_group1() == Group1::Kazu => -2,
                        _ => 0,
                    },
                    rule::KAKKO2 => -2,
                    rule::BANGOU => -2,
                    s if Self::is_period(s) => {
                        if start > 1
                            && matches!(njd.nodes.get(start-2),Some(node) if node.get_pos().get_group1()==Group1::Kazu)
                        {
                            -5
                        } else {
                            0
                        }
                    }
                    _ => 0,
                };
            }
            if start > 1
                && matches!(njd.nodes.get(start-2),Some(node) if node.get_string()==rule::BANGOU)
            {
                score -= 2;
            }
        }
        if end + 1 < njd.nodes.len() {
            let (pos, string) = {
                let node = &njd.nodes[end + 1];
                (node.get_pos(), node.get_string())
            };
            score += match (pos.get_group1(), pos.get_group2()) {
                (Group1::FukushiKanou, _) => 2,
                (_, Group2::Josuushi) => 2,
                _ => 0,
            };
            score += match string {
                rule::HAIHUN1 => -2,
                rule::HAIHUN2 => -2,
                rule::HAIHUN3 => -2,
                rule::HAIHUN4 => -2,
                rule::HAIHUN5 => -2,
                rule::KAKKO1 => -2,
                rule::KAKKO2 => match njd.nodes.get(end + 2) {
                    Some(node) if node.get_pos().get_group1() == Group1::Kazu => -2,
                    _ => 0,
                },
                rule::BANGOU => -2,
                s if Self::is_period(s) => 4,
                _ => 0,
            };
        }
        score
    }

    fn convert_digit_sequence_for_non_numerical_reading(njd: &mut NJD, start: usize, end: usize) {
        if end - start == 0 {
            return;
        }

        let mut size = 0;
        let mut iter = njd.iter_quint_mut();
        while let Some(quint) = iter.next() {
            let (prev, node) = match Double::from(quint) {
                Double::First(c) => (None, c),
                Double::Full(p, c) => (Some(p), c),
            };

            match node.get_string() {
                rule::ZERO1 | rule::ZERO2 => {
                    node.set_pron(rule::ZERO_AFTER_DP);
                    node.set_mora_size(2);
                }
                rule::TWO => {
                    node.set_pron(rule::TWO_AFTER_DP);
                    node.set_mora_size(2);
                }
                rule::FIVE => {
                    node.set_pron(rule::FIVE_AFTER_DP);
                    node.set_mora_size(2);
                }
                _ => (),
            }
            node.unset_chain_rule();
            if size % 2 == 0 {
                node.set_chain_flag(false);
            } else {
                node.set_chain_flag(true);
                prev.unwrap().set_acc(3);
            }
            size += 1;
        }
    }

    fn convert_digit_sequence_for_numerical_reading(
        njd: &mut NJD,
        start: usize,
        end: usize,
    ) -> Vec<(usize, Option<NJDNode>)> {
        let mut insertion_table: Vec<(usize, Option<NJDNode>)> = Vec::new();

        let mut have = false;

        let size = end - start + 1
            - njd.nodes[start..end + 1]
                .iter()
                .filter(|node| Self::is_comma(node.get_string()))
                .count();

        let mut index = match size % 4 {
            0 => 4,
            i => i,
        };
        let mut place = if size > index { (size - index) / 4 } else { 0 };
        if size <= 1 || place > 17 {
            return insertion_table;
        }

        index -= 1;

        for (i, node) in njd.nodes[start..end + 1].iter_mut().enumerate() {
            if Self::is_comma(node.get_string()) {
                /* remove all commas before period */
                insertion_table.push((start + i, None));
                continue;
            }

            let digit = Self::get_digit(node);
            if index == 0 {
                if matches!(digit, Some(0)) {
                    node.unset_pron();
                    node.set_acc(0);
                    node.set_mora_size(0);
                } else {
                    have = true;
                }
                if have {
                    if place > 0 {
                        let new_node = NJDNode::new_single(rule::NUMERAL_LIST3[place]);
                        insertion_table.push((start + i, Some(new_node)));
                    }
                    have = false;
                }
                if place > 0 {
                    place -= 1;
                }
            } else {
                match digit {
                    None | Some(0) => {
                        node.unset_pron();
                        node.set_acc(0);
                        node.set_mora_size(0);
                    }
                    Some(1) => {
                        *node = NJDNode::new_single(rule::NUMERAL_LIST2[index]);
                        have = true;
                    }
                    _ => {
                        let new_node = NJDNode::new_single(rule::NUMERAL_LIST2[index]);
                        insertion_table.push((start + i, Some(new_node)));
                        have = true;
                    }
                }
            }
            index = if index == 0 { 3 } else { index - 1 };
        }

        insertion_table
    }
}
