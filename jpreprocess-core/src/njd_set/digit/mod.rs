mod lut1;
mod lut2;
mod lut3;
mod lut_conversion;
mod rule;

mod digit_sequence;

use jpreprocess_njd::pos::*;
use jpreprocess_njd::*;

use crate::window::*;

use self::{
    digit_sequence::DigitSequence,
    lut_conversion::{find_digit_pron_conv, find_numerative_pron_conv},
};

pub fn njd_set_digit(njd: &mut NJD) {
    let mut find = false;

    {
        for node in &mut njd.nodes {
            if matches!(node.get_pos().get_group1(), Group1::Kazu) {
                find = true;
            }
            normalize_digit(node);
        }

        let mut sequences = DigitSequence::from_njd(njd);

        for seq in &mut sequences {
            seq.convert_digit_sequence(njd);
        }

        DigitSequence::to_njd(njd, sequences);
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
        let mut iter = njd.iter_quint_mut();
        while let Some(quint) = iter.next() {
            let (prev, node, next) = match Triple::from(quint) {
                Triple::Full(prev, node, next) => (prev, node, next),
                _ => continue,
            };
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
                        prev.set_acc(1);
                    }
                    _ => (),
                }
                skip_state = SkipState::IfMeishi;
            }
        }
    }

    {
        let mut iter = njd.iter_quint_mut();
        while let Some(quint) = iter.next() {
            let (prev, node) = match Double::from(quint) {
                Double::Full(prev, node) => (prev, node),
                _ => continue,
            };
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
                &lut1::CONVERSION_TABLE,
                node.get_string(),
                prev.get_string(),
            ) {
                prev.set_pron(lut1_conversion.0);
                prev.set_acc(lut1_conversion.1);
                prev.set_mora_size(lut1_conversion.2);
            }
            /* convert numerative pron */
            if let Some(lut2_new_pron) = find_numerative_pron_conv(
                &lut2::CONVERSION_TABLE,
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

    {
        let mut iter = njd.iter_quint_mut();
        while let Some(quint) = iter.next() {
            let (prev, node) = match Double::from(quint) {
                Double::Full(prev, node) => (prev, node),
                _ => continue,
            };
            if !matches!(prev.get_pos().get_group1(), Group1::Kazu) {
                continue;
            }
            if matches!(node.get_pos().get_group1(), Group1::Kazu) && !node.get_string().is_empty()
            {
                if lut3::NUMERAL_LIST4.contains(prev.get_string())
                    && lut3::NUMERAL_LIST5.contains(node.get_string())
                {
                    prev.set_chain_flag(false);
                    node.set_chain_flag(true);
                } else if lut3::NUMERAL_LIST5.contains(prev.get_string())
                    && lut3::NUMERAL_LIST4.contains(node.get_string())
                {
                    node.set_chain_flag(false);
                }
            }
            if let Some(lut3_conversion) = find_digit_pron_conv(
                &lut3::DIGIT_CONVERSION_TABLE,
                node.get_string(),
                prev.get_string(),
            ) {
                prev.set_pron(lut3_conversion.0);
                prev.set_acc(lut3_conversion.1);
                prev.set_mora_size(lut3_conversion.2);
            }
            if let Some(lut3_new_pron) = find_numerative_pron_conv(
                &lut3::NUMERATIVE_CONVERSION_TABLE,
                node.get_string(),
                prev.get_string(),
                node.get_pron().unwrap(),
            ) {
                node.set_pron(lut3_new_pron.as_str());
            }
        }
    }

    {
        let mut iter = njd.iter_quint_mut();
        while let Some(quint) = iter.next() {
            let (prev, node, next) = match Triple::from(quint) {
                Triple::First(node, next) => (None, node, next),
                Triple::Full(prev, node, next) => (Some(prev), node, next),
                _ => continue,
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
                (_, Some(Group1::Kazu)) => continue,
                _ => (),
            };
            match (next.get_pos().get_group1(), next.get_pos().get_group2()) {
                (Group1::FukushiKanou, _) => (),
                (_, Group2::Josuushi) => (),
                _ => continue,
            };
            /* convert class3 */
            if rule::NUMERATIVE_CLASS3
                .contains(&(next.get_string(), next.get_read().unwrap_or("*")))
            {
                if let Some(conversion) = rule::CONV_TABLE3.get(node.get_string()) {
                    node.set_read(conversion.0);
                    node.set_pron(conversion.0);
                    node.set_acc(conversion.1);
                    node.set_mora_size(conversion.2);
                }
            }
            /* person */
            if next.get_string() == rule::NIN {
                if let Some(new_node_s) = rule::CONV_TABLE4.get(node.get_string()) {
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
                    next.unset_pron();
                } else {
                    if let Some(new_node_s) = rule::CONV_TABLE5.get(node.get_string()) {
                        *node = NJDNode::new_single(new_node_s);
                        next.unset_pron();
                    }
                }
            } else if next.get_string() == rule::NICHIKAN {
                if let Some(new_node_s) = rule::CONV_TABLE6.get(node.get_string()) {
                    *node = NJDNode::new_single(new_node_s);
                    next.unset_pron();
                }
            }
        }
    }

    if njd.nodes.len() > 2 {
        let mut iter = njd.iter_quint_mut();
        while let Some(quint) = iter.next() {
            let (node, nx1, nx2, nx3_t) = match quint {
                Quintuple::Triple(node, nx1, nx2) => (node, nx1, nx2, None),
                Quintuple::First(node, nx1, nx2, nx3) => (node, nx1, nx2, Some(nx3)),
                Quintuple::Full(prev, node, nx1, nx2, nx3)
                    if prev.get_pos().get_group1() != Group1::Kazu =>
                {
                    (node, nx1, nx2, Some(nx3))
                }
                Quintuple::ThreeLeft(prev, node, nx1, nx2)
                    if prev.get_pos().get_group1() != Group1::Kazu =>
                {
                    (node, nx1, nx2, None)
                }
                _ => continue,
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
    }

    njd.remove_silent_node();
}

fn normalize_digit(node: &mut NJDNode) -> bool {
    if node.get_string() != "*" && matches!(node.get_pos().get_group1(), Group1::Kazu) {
        if let Some(replace) = rule::NUMERAL_LIST1.get(node.get_string()) {
            node.replace_string(replace);
            return true;
        }
    }
    false
}

fn is_period(s: &str) -> bool {
    matches!(s, rule::TEN1 | rule::TEN2)
}
