mod lut;
mod symbols;

mod digit_sequence;

use crate::{NJDNode, NJD};

use jpreprocess_core::{pos::*, pron};
use jpreprocess_window::*;

use self::{
    lut::{
        class1, class2, class3, find_pron_conv_map, find_pron_conv_set, numeral, others, DigitType,
    },
    symbols::{is_period, normalize_digit},
};

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
                        prev.set_pron(pron!([Re, Long], 1));
                    }
                    rule::TWO => {
                        prev.set_pron(pron!([Ni, Long], 1));
                    }
                    rule::FIVE => {
                        prev.set_pron(pron!([Go, Long], 1));
                    }
                    rule::SIX => {
                        prev.set_pron(pron!([Ro, Ku], 1));
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
            if let Some(lut1_conversion) = find_pron_conv_set(
                &class1::CONVERSION_TABLE,
                node.get_string(),
                prev.get_string(),
            ) {
                prev.set_pron(lut1_conversion.clone());
            }
            /* convert numerative pron */
            match find_pron_conv_set(
                &class2::CONVERSION_TABLE,
                node.get_string(),
                prev.get_string(),
            ) {
                Some(DigitType::Voiced) => node
                    .get_pron_mut()
                    .moras_mut()
                    .first_mut()
                    .map(|mora| mora.convert_to_voiced_sound()),
                Some(DigitType::SemiVoiced) => node
                    .get_pron_mut()
                    .moras_mut()
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
                if numeral::NUMERAL_LIST4.contains(prev.get_string())
                    && numeral::NUMERAL_LIST5.contains(node.get_string())
                {
                    prev.set_chain_flag(false);
                    node.set_chain_flag(true);
                } else if numeral::NUMERAL_LIST5.contains(prev.get_string())
                    && numeral::NUMERAL_LIST4.contains(node.get_string())
                {
                    node.set_chain_flag(false);
                }
            }
            if let Some(lut3_conversion) = find_pron_conv_set(
                &numeral::DIGIT_CONVERSION_TABLE,
                node.get_string(),
                prev.get_string(),
            ) {
                prev.set_pron(lut3_conversion.clone());
            }
            match find_pron_conv_set(
                &numeral::NUMERATIVE_CONVERSION_TABLE,
                node.get_string(),
                prev.get_string(),
            ) {
                Some(DigitType::Voiced) => node
                    .get_pron_mut()
                    .moras_mut()
                    .first_mut()
                    .map(|mora| mora.convert_to_voiced_sound()),
                Some(DigitType::SemiVoiced) => node
                    .get_pron_mut()
                    .moras_mut()
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
            if let Some(conversion) = find_pron_conv_map(
                &class3::CONVERSION_TABLE,
                next.get_string(),
                next.get_read().unwrap_or("*"),
                node.get_string(),
            ) {
                node.set_read(&conversion.to_pure_string());
                node.set_pron(conversion.clone());
            }

            /* person and the day of month */
            if let Some(new_node_s) = find_pron_conv_set(
                &others::CONVERSION_TABLE,
                next.get_string(),
                node.get_string(),
            ) {
                if matches!(prev, Some(p) if p.get_string().contains(rule::GATSU))
                    && node.get_string() == rule::ONE
                    && next.get_string() == rule::NICHI
                {
                    *node = NJDNode::new_single(rule::TSUITACHI);
                } else {
                    *node = NJDNode::new_single(new_node_s);
                }

                next.reset();
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

mod rule {
    pub const TEN_FEATURE: &str = "．,名詞,接尾,助数詞,*,*,*,．,テン,テン,0/2,*,-1";
    pub const ZERO1: &str = "〇";
    pub const ZERO2: &str = "０";
    pub const TWO: &str = "二";
    pub const FIVE: &str = "五";
    pub const SIX: &str = "六";

    pub const GATSU: &str = "月";
    pub const NICHI: &str = "日";
    pub const NICHIKAN: &str = "日間";

    pub const ONE: &str = "一";
    pub const TSUITACHI: &str = "一日,名詞,副詞可能,*,*,*,*,一日,ツイタチ,ツイタチ,4/4,*";

    pub const FOUR: &str = "四";
    pub const TEN: &str = "十";
    pub const JUYOKKA: &str = "十四日,名詞,副詞可能,*,*,*,*,十四日,ジュウヨッカ,ジューヨッカ,1/5,*";
    pub const JUYOKKAKAN: &str =
        "十四日間,名詞,副詞可能,*,*,*,*,十四日間,ジュウヨッカカン,ジューヨッカカン,5/7,*";
    pub const NIJU: &str = "二十,名詞,副詞可能,*,*,*,*,二十,ニジュウ,ニジュー,1/3,*";
    pub const YOKKA: &str = "四日,名詞,副詞可能,*,*,*,*,四日,ヨッカ,ヨッカ,0/3,*,0";
    pub const YOKKAKAN: &str = "四日間,名詞,副詞可能,*,*,*,*,四日間,ヨッカカン,ヨッカカン,3/5,*,0";
    pub const HATSUKA: &str = "二十日,名詞,副詞可能,*,*,*,*,二十日,ハツカ,ハツカ,0/3,*";
    pub const HATSUKAKAN: &str =
        "二十日間,名詞,副詞可能,*,*,*,*,二十日間,ハツカカン,ハツカカン,3/5,*";
}
