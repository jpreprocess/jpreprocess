mod lut;
mod rule;

mod digit_sequence;

use crate::{digit::rule::is_period, NJDNode, NJD};

use jpreprocess_core::pos::*;
use jpreprocess_window::*;

use self::lut::{find_digit_pron_conv, find_numerative_pron_conv, lut1, lut2, lut3, DigitType};

pub fn njd_set_digit(njd: &mut NJD) {
    let mut find = false;

    {
        for node in &mut njd.nodes {
            if node.get_pos().is_kazu() {
                find = true;
            }
            normalize_digit(node);
        }

        let mut sequences = digit_sequence::from_njd(njd);

        let mut offset = 0;
        for seq in &mut sequences {
            offset += seq.convert(njd, offset);
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
        let mut iter = njd.iter_quint_mut();
        while let Some(quint) = iter.next() {
            let (prev, node, next) = match Triple::from(quint) {
                Triple::Full(prev, node, next) => (prev, node, next),
                _ => continue,
            };
            match (&skip_state, node.get_pos()) {
                (SkipState::IfMeishi, _) => {
                    skip_state = SkipState::Skipping;
                    continue;
                }
                (SkipState::Skipping, POS::Meishi(_)) => {
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
                && prev.get_pos().is_kazu()
                && next.get_pos().is_kazu()
            {
                *node = NJDNode::new_single(rule::TEN_FEATURE);
                node.set_chain_flag(true);
                match prev.get_string() {
                    rule::ZERO1 | rule::ZERO2 => {
                        prev.set_pron_by_str(rule::ZERO_BEFORE_DP);
                        prev.set_mora_size(2);
                    }
                    rule::TWO => {
                        prev.set_pron_by_str(rule::TWO_BEFORE_DP);
                        prev.set_mora_size(2);
                    }
                    rule::FIVE => {
                        prev.set_pron_by_str(rule::FIVE_BEFORE_DP);
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
            if !prev.get_pos().is_kazu() {
                continue;
            }
            match node.get_pos() {
                POS::Meishi(Meishi::FukushiKanou) => (),
                POS::Meishi(Meishi::Setsubi(Setsubi::Josuushi)) => (),
                _ => continue,
            }
            /* convert digit pron */
            if let Some(lut1_conversion) = find_digit_pron_conv(
                &lut1::CONVERSION_TABLE,
                node.get_string(),
                prev.get_string(),
            ) {
                prev.set_pron_by_str(lut1_conversion.0);
                prev.set_acc(lut1_conversion.1);
                prev.set_mora_size(lut1_conversion.2);
            }
            /* convert numerative pron */
            match find_numerative_pron_conv(
                &lut2::CONVERSION_TABLE,
                node.get_string(),
                prev.get_string(),
            ) {
                Some(DigitType::Voiced) => node
                    .get_pron_mut()
                    .first_mut()
                    .map(|mora| mora.convert_to_voiced_sound()),
                Some(DigitType::SemiVoiced) => node
                    .get_pron_mut()
                    .first_mut()
                    .map(|mora| mora.convert_to_semivoiced_sound()),
                _ => None,
            };
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
            if !prev.get_pos().is_kazu() {
                continue;
            }
            if node.get_pos().is_kazu() && !node.get_string().is_empty() {
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
                prev.set_pron_by_str(lut3_conversion.0);
                prev.set_acc(lut3_conversion.1);
                prev.set_mora_size(lut3_conversion.2);
            }
            match find_numerative_pron_conv(
                &lut3::NUMERATIVE_CONVERSION_TABLE,
                node.get_string(),
                prev.get_string(),
            ) {
                Some(DigitType::Voiced) => node
                    .get_pron_mut()
                    .first_mut()
                    .map(|mora| mora.convert_to_voiced_sound()),
                Some(DigitType::SemiVoiced) => node
                    .get_pron_mut()
                    .first_mut()
                    .map(|mora| mora.convert_to_semivoiced_sound()),
                _ => None,
            };
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
            if !node.get_pos().is_kazu() {
                continue;
            }
            match prev.as_ref().map(|p| p.get_pos()) {
                None => (),
                Some(POS::Kigou(_)) => (),
                Some(pos) if pos.is_kazu() => continue,
                _ => (),
            };
            match next.get_pos() {
                POS::Meishi(Meishi::FukushiKanou) => (),
                POS::Meishi(Meishi::Setsubi(Setsubi::Josuushi)) => (),
                _ => continue,
            };
            /* convert class3 */
            if rule::NUMERATIVE_CLASS3
                .contains(&(next.get_string(), next.get_read().unwrap_or("*")))
            {
                if let Some(conversion) = rule::CONV_TABLE3.get(node.get_string()) {
                    node.set_read(conversion.0);
                    node.set_pron_by_str(conversion.0);
                    node.set_acc(conversion.1);
                    node.set_mora_size(conversion.2);
                }
            }
            /* person */
            if next.get_string() == rule::NIN {
                if let Some(new_node_s) = rule::CONV_TABLE4.get(node.get_string()) {
                    *node = NJDNode::new_single(new_node_s);
                    next.reset();
                }
            }
            /* the day of month */
            if next.get_string() == rule::NICHI && !node.get_string().is_empty() {
                if matches!(prev,Some(p) if p.get_string().contains(rule::GATSU))
                    && node.get_string() == rule::ONE
                {
                    *node = NJDNode::new_single(rule::TSUITACHI);
                    next.reset();
                } else if let Some(new_node_s) = rule::CONV_TABLE5.get(node.get_string()) {
                    *node = NJDNode::new_single(new_node_s);
                    next.reset();
                }
            } else if next.get_string() == rule::NICHIKAN {
                if let Some(new_node_s) = rule::CONV_TABLE6.get(node.get_string()) {
                    *node = NJDNode::new_single(new_node_s);
                    next.reset();
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
                Quintuple::Full(prev, node, nx1, nx2, nx3) if !prev.get_pos().is_kazu() => {
                    (node, nx1, nx2, Some(nx3))
                }
                Quintuple::ThreeLeft(prev, node, nx1, nx2) if !prev.get_pos().is_kazu() => {
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
                    nx1.reset();
                    nx2.reset();
                }
                UnsetPattern::Nx2Nx3 => {
                    nx2.reset();
                    nx3.as_mut().unwrap().reset();
                }
            }
        }
    }

    njd.remove_silent_node();
}

fn normalize_digit(node: &mut NJDNode) -> bool {
    if node.get_string() != "*" && node.get_pos().is_kazu() {
        if let Some(replace) = rule::DIGIT_NORMALIZE.get(node.get_string()) {
            node.replace_string(replace);
            return true;
        }
    }
    false
}
