mod lut1;
mod lut2;
mod lut3;
mod lut_conversion;
mod rule;

use std::collections::HashMap;

use crate::njd::pos::*;
use crate::njd::*;

use self::lut_conversion::{find_digit_pron_conv, find_numerative_pron_conv};

pub fn njd_set_digit(njd: &mut NJD) {
    let mut find = false;
    {
        let mut s: Option<usize> = None;
        let mut e: Option<usize> = None;
        let nodes_len = njd.nodes.len();
        for i in 0..njd.nodes.len() {
            {
                let node = &mut njd.nodes[i];
                if matches!(node.get_pos().get_group1(), Group1::Kazu) {
                    find = true;
                }
                if normalize_digit(node) == true
                    || (matches!(node.get_pos().get_group1(), Group1::Kazu)
                        && (is_period(node.get_string()) || is_comma(node.get_string())))
                {
                    if s.is_none() {
                        s = Some(i);
                    }
                    if i == nodes_len {
                        e = Some(i);
                    }
                } else {
                    if s.is_some() {
                        e = Some(i - 1);
                    }
                }
            }
            if let (Some(start), Some(end)) = (s, e) {
                convert_digit_sequence(njd, start, end);
                s = None;
                e = None;
            }
        }
    }
    if !find {
        return;
    }

    njd.remove_silent_node();

    {
        enum SkipState {
            Disabled,
            IfMeishi,
            Skipping,
        }
        let mut skip_state = SkipState::Disabled;
        for i in 1..njd.nodes.len() - 1 {
            if let [prev, node, next] = &mut njd.nodes[i - 1..i + 2] {
                match (&skip_state, node.get_pos().get_group0()) {
                    (SkipState::IfMeishi, _) => {
                        skip_state = SkipState::Skipping;
                        continue;
                    }
                    (SkipState::Skipping, Group0::Meishi) => {
                        continue;
                    }
                    (SkipState::Skipping, _) => {
                        skip_state = SkipState::Disabled;
                        continue;
                    }
                    _ => (),
                }
                if !node.get_string().is_empty()
                    && !prev.get_string().is_empty()
                    && is_period(node.get_string())
                    && matches!(prev.get_pos().get_group1(), Group1::Kazu)
                    && matches!(next.get_pos().get_group1(), Group1::Kazu)
                {
                    *node = NJDNode::new_single(rule::TEN_FEATURE);
                    node.set_chain_flag(true);
                    match prev.get_string() {
                        rule::ZERO1 | rule::ZERO2 => {
                            prev.set_pron(rule::ZERO_BEFORE_DP);
                            prev.set_mora_size(2);
                        }
                        rule::TWO => {
                            prev.set_pron(rule::TWO_BEFORE_DP);
                            prev.set_mora_size(2);
                        }
                        rule::FIVE => {
                            prev.set_pron(rule::FIVE_BEFORE_DP);
                            prev.set_mora_size(2);
                        }
                        rule::SIX => {
                            node.set_acc(1);
                        }
                        _ => (),
                    }
                    skip_state = SkipState::IfMeishi;
                }
            }
        }
    }

    for i in 1..njd.nodes.len() {
        if let [prev, node] = &mut njd.nodes[i - 1..i + 1] {
            match (
                prev.get_pos().get_group1(),
                node.get_pos().get_group1(),
                node.get_pos().get_group2(),
            ) {
                (Group1::Kazu, Group1::FukushiKanou, _) => (),
                (Group1::Kazu, _, Group2::Josuushi) => (),
                _ => continue,
            }
            /* convert digit pron */
            if let Some(lut1_conversion) = find_digit_pron_conv(
                &lut1::conversion_table,
                node.get_string(),
                prev.get_string(),
            ) {
                node.set_pron(lut1_conversion.0);
                node.set_acc(lut1_conversion.1);
                node.set_mora_size(lut1_conversion.2);
            }
            /* convert numerative pron */
            if let Some(lut2_new_pron) = find_numerative_pron_conv(
                &lut2::conversion_table,
                node.get_string(),
                prev.get_string(),
                node.get_pron().unwrap(),
            ) {
                node.set_pron(lut2_new_pron.as_str());
            }
            prev.set_chain_flag(false);
            node.set_chain_flag(true);
        }
    }

    for i in 1..njd.nodes.len() {
        if let [prev, node] = &mut njd.nodes[i - 1..i + 1] {
            if !matches!(prev.get_pos().get_group1(), Group1::Kazu) {
                continue;
            }
            if matches!(node.get_pos().get_group1(), Group1::Kazu) && !node.get_string().is_empty()
            {
                if lut3::numeral_list4.contains(prev.get_string())
                    && lut3::numeral_list5.contains(node.get_string())
                {
                    prev.set_chain_flag(false);
                    node.set_chain_flag(true);
                } else if lut3::numeral_list5.contains(prev.get_string())
                    && lut3::numeral_list4.contains(node.get_string())
                {
                    node.set_chain_flag(false);
                }
            }
            if let Some(lut1_conversion) = find_digit_pron_conv(
                &lut3::digit_conversion_table,
                node.get_string(),
                prev.get_string(),
            ) {
                node.set_pron(lut1_conversion.0);
                node.set_acc(lut1_conversion.1);
                node.set_mora_size(lut1_conversion.2);
            }
            if let Some(lut2_new_pron) = find_numerative_pron_conv(
                &lut3::numerative_conversion_table,
                node.get_string(),
                prev.get_string(),
                node.get_pron().unwrap(),
            ) {
                node.set_pron(lut2_new_pron.as_str());
            }
        }
    }

    for i in 0..njd.nodes.len() {
        let (prev, node, next) = if i + 2 >= njd.nodes.len() {
            continue;
        } else if i == 0 {
            if let [node, next] = &mut njd.nodes[i..i + 2] {
                (None, node, next)
            } else {
                continue;
            }
        } else {
            if let [prev, node, next] = &mut njd.nodes[i - 1..i + 2] {
                (Some(prev), node, next)
            } else {
                continue;
            }
        };
        if next.get_string().is_empty() {
            continue;
        }
        if !matches!(node.get_pos().get_group1(), Group1::Kazu) {
            continue;
        }
        match (
            prev.as_ref().map(|p| p.get_pos().get_group0()),
            prev.as_ref().map(|p| p.get_pos().get_group1()),
        ) {
            (None, None) => (),
            (Some(Group0::Kigou), _) => (),
            (_, Some(Group1::Kazu)) => (),
            _ => continue,
        };
        match (next.get_pos().get_group1(), next.get_pos().get_group2()) {
            (Group1::FukushiKanou, _) => (),
            (_, Group2::Josuushi) => (),
            _ => continue,
        };
        /* convert class3 */
        if rule::numerative_class3.contains(&(next.get_string(), next.get_read().unwrap_or("*"))) {
            if let Some(conversion) = rule::conv_table3.get(node.get_string()) {
                node.set_read(conversion.0);
                node.set_pron(conversion.0);
                node.set_acc(conversion.1);
                node.set_mora_size(conversion.2);
            }
        }
        /* person */
        if next.get_string() == rule::NIN {
            if let Some(new_node_s) = rule::conv_table4.get(node.get_string()) {
                *node = NJDNode::new_single(new_node_s);
                next.unset_pron();
            }
        }
        /* the day of month */
        if next.get_string() == rule::NICHI && !node.get_string().is_empty() {
            if matches!(prev,Some(p) if p.get_string().contains(rule::GATSU))
                && node.get_string() == rule::ONE
            {
                *node = NJDNode::new_single(rule::TSUITACHI);
            } else {
                if let Some(new_node_s) = rule::conv_table5.get(node.get_string()) {
                    *node = NJDNode::new_single(new_node_s);
                    next.unset_pron();
                }
            }
        } else if next.get_string() == rule::NICHIKAN {
            if let Some(new_node_s) = rule::conv_table6.get(node.get_string()) {
                *node = NJDNode::new_single(new_node_s);
                next.unset_pron();
            }
        }
    }

    for i in 0..njd.nodes.len() - 2 {
        if i > 0 && matches!(njd.nodes.get(i-1),Some(p) if p.get_pos().get_group1()==Group1::Kazu) {
            continue;
        }
        let (node, nx1, nx2, nx3_t) = if let [node, nx1, nx2, nx3] = &mut njd.nodes[i..i + 4] {
            (node, nx1, nx2, Some(nx3))
        } else if let [node, nx1, nx2] = &mut njd.nodes[i..i + 3] {
            (node, nx1, nx2, None)
        } else {
            continue;
        };
        let mut nx3 = nx3_t;

        enum UnsetPattern {
            None,
            Nx1Nx2,
            Nx2Nx3,
        }

        let (node_s, nx1_s, unset) = match (
            node.get_string(),
            nx1.get_string(),
            nx2.get_string(),
            nx3.as_ref().map(|n| n.get_string()),
        ) {
            (rule::TEN, rule::FOUR, rule::NICHI, _) => {
                (Some(rule::JUYOKKA), None, UnsetPattern::Nx1Nx2)
            }
            (rule::TEN, rule::FOUR, rule::NICHIKAN, _) => {
                (Some(rule::JUYOKKAKAN), None, UnsetPattern::Nx1Nx2)
            }
            (rule::TWO, rule::TEN, rule::NICHI, _) => {
                (Some(rule::HATSUKA), None, UnsetPattern::Nx1Nx2)
            }
            (rule::TWO, rule::TEN, rule::NICHIKAN, _) => {
                (Some(rule::HATSUKAKAN), None, UnsetPattern::Nx1Nx2)
            }
            (rule::TWO, rule::TEN, rule::FOUR, Some(rule::NICHI)) => {
                (Some(rule::NIJU), Some(rule::YOKKA), UnsetPattern::Nx2Nx3)
            }
            (rule::TWO, rule::TEN, rule::FOUR, Some(rule::NICHIKAN)) => {
                (Some(rule::NIJU), Some(rule::YOKKAKAN), UnsetPattern::Nx2Nx3)
            }
            _ => (None, None, UnsetPattern::None),
        };
        if let Some(new_node_s) = node_s {
            *node = NJDNode::new_single(new_node_s);
        }
        if let Some(new_node_s) = nx1_s {
            *nx1 = NJDNode::new_single(new_node_s);
        }
        match unset {
            UnsetPattern::None => (),
            UnsetPattern::Nx1Nx2 => {
                nx1.unset_pron();
                nx2.unset_pron();
            }
            UnsetPattern::Nx2Nx3 => {
                nx2.unset_pron();
                nx3.as_mut().unwrap().unset_pron();
            }
        }
    }

    njd.remove_silent_node();
}

fn convert_digit_sequence(njd: &mut NJD, s: usize, e: usize) {
    enum NumericalReading {
        Numerical,
        Unknown,
        NonNumerical,
    }
    let mut numerical_reading = NumericalReading::Numerical;

    if is_comma(njd.nodes[s].get_string()) || is_period(njd.nodes[s].get_string()) {
        if s != e && s + 1 < njd.nodes.len() {
            convert_digit_sequence(njd, s + 1, e);
        }
        return;
    }

    /* find final digit before period */
    let final_digit_before_period = {
        let period = njd.nodes[s..e]
            .iter()
            .position(|node| is_period(node.get_string()))
            .map(|postion| s + postion)
            .unwrap_or(e);
        njd.nodes[s..period + 1]
            .iter()
            .rev()
            .position(|node| !is_comma(node.get_string()))
            .map(|postion| period - postion)
            .unwrap_or(s)
    };

    /* check commas */
    let (first_comma_before_period, num_comma) = {
        let mut first_comma_before_period: Option<usize> = None;
        let mut num_comma = 0;
        for (i, node) in njd.nodes[s..final_digit_before_period + 1]
            .iter()
            .rev()
            .enumerate()
        {
            if is_comma(node.get_string()) {
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
    if s != final_digit_before_period && matches!(get_digit(&njd.nodes[s]), Some(0)) {
        numerical_reading = NumericalReading::NonNumerical;
    }

    /* if no info, set unknown flag */
    if matches!(numerical_reading, NumericalReading::Numerical) && num_comma == 0 {
        numerical_reading = NumericalReading::Unknown;
    }

    match numerical_reading {
        NumericalReading::Numerical => {
            /* numerical reading until period */
            if num_comma > 0 {
                /* remove all commas before period */
                let mut i = 0;
                njd.nodes.retain(|node| {
                    let b = i < s || final_digit_before_period <= i || is_comma(node.get_string());
                    i += 1;
                    b
                });
            }
            let offset =
                convert_digit_sequence_for_numerical_reading(njd, s, final_digit_before_period);
            if final_digit_before_period + offset < e {
                convert_digit_sequence(njd, final_digit_before_period + offset + 1, e);
            }
        }
        _ => {
            let final_digit = if let Some(p) = first_comma_before_period {
                p - 1
            } else {
                final_digit_before_period
            };

            if matches!(numerical_reading, NumericalReading::Unknown) {
                numerical_reading = if get_digit_sequence_score(njd, s, final_digit) >= 0 {
                    NumericalReading::Numerical
                } else {
                    NumericalReading::NonNumerical
                }
            }

            let offset = match numerical_reading {
                NumericalReading::Numerical => {
                    /* numerical reading until comma */
                    convert_digit_sequence_for_numerical_reading(njd, s, final_digit)
                }
                _ => {
                    /* non-numerical reading */
                    convert_digit_sequence_for_non_numerical_reading(njd, s, final_digit);
                    0
                }
            };

            if final_digit + offset < e {
                convert_digit_sequence(njd, final_digit + offset + 1, e);
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
        if is_period(string) {
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
            _ => match string {
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
                _ => 0,
            },
        }
    }
    score
}

fn convert_digit_sequence_for_non_numerical_reading(njd: &mut NJD, start: usize, end: usize) {
    if end - start <= 0 {
        return;
    }

    let mut size = 0;
    for i in start..end + 1 {
        let (prev, node) = {
            if let [prev, node] = &mut njd.nodes[i - 1..i + 1] {
                (Some(prev), node)
            } else {
                (None, &mut njd.nodes[i])
            }
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

fn convert_digit_sequence_for_numerical_reading(njd: &mut NJD, start: usize, end: usize) -> usize {
    let mut have = false;

    let size = end - start + 1;
    let mut index = match size % 4 {
        0 => 4,
        i => i,
    };
    let mut place = if size > index { (size - index) / 4 } else { 0 };
    if size <= 1 || place > 17 {
        return 0;
    }

    index -= 1;

    let mut insertion_reserve: HashMap<usize, NJDNode> = HashMap::new();

    for (i, node) in njd.nodes[start..end + 1].iter_mut().enumerate() {
        let digit = get_digit(node);
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
                    let new_node = NJDNode::new_single(rule::numeral_list3[place]);
                    insertion_reserve.insert(i, new_node);
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
                    *node = NJDNode::new_single(rule::numeral_list2[index]);
                    have = true;
                }
                _ => {
                    let new_node = NJDNode::new_single(rule::numeral_list2[index]);
                    insertion_reserve.insert(i, new_node);
                    have = true;
                }
            }
        }
        index = if index == 0 { 3 } else { index - 1 };
    }

    let mut offset = 0;
    for (index, node) in insertion_reserve {
        njd.nodes.insert(start + index + offset + 1, node);
        offset += 1;
    }

    offset
}

fn get_digit(node: &NJDNode) -> Option<i32> {
    if !node.get_string().is_empty() && matches!(node.get_pos().get_group1(), Group1::Kazu) {
        if let Some((digit, _)) = rule::numeral_list1.get(node.get_string()) {
            return Some(*digit);
        }
    }
    None
}

fn normalize_digit(node: &mut NJDNode) -> bool {
    if node.get_string() != "*" && matches!(node.get_pos().get_group1(), Group1::Kazu) {
        if let Some((_, replace)) = rule::numeral_list1.get(node.get_string()) {
            node.replace_string(replace);
            return true;
        }
    }
    false
}

fn is_period(s: &str) -> bool {
    matches!(s, rule::TEN1 | rule::TEN2)
}
fn is_comma(s: &str) -> bool {
    matches!(s, rule::COMMA)
}

#[cfg(test)]
mod tests {
    use super::{get_digit, NJDNode};

    #[test]
    fn get_digit_1() {
        let node = NJDNode::new_single("１,名詞,数,*,*,*,*,１,イチ,イチ");
        assert_eq!(get_digit(&node).unwrap(), 1);
    }
}
