pub const ICHI: &str = "一";
pub const NI: &str = "二";
pub const SAN: &str = "三";
pub const YON: &str = "四";
pub const GO: &str = "五";
pub const ROKU: &str = "六";
pub const NANA: &str = "七";
pub const HACHI: &str = "八";
pub const KYUU: &str = "九";
pub const JYUU: &str = "十";
pub const HYAKU: &str = "百";
pub const SEN: &str = "千";
pub const MAN: &str = "万";
pub const OKU: &str = "億";
pub const CHOU: &str = "兆";
pub const NAN: &str = "何";
pub const IKU: &str = "幾";

use jpreprocess_core::accent_rule::AccentType;

use crate::{NJDNode, NJD};

pub fn njd_set_accent_type(njd: &mut NJD) {
    if njd.nodes.is_empty() {
        return;
    }
    let mut top_node_i = 0;
    let mut mora_size = 0;
    for i in 0..njd.nodes.len() {
        let mut top_node_acc: Option<usize> = None;
        let mut prev_acc: Option<usize> = None;
        let mut current_acc: Option<usize> = None;

        {
            let (top_node, prev, current, next) = (
                njd.nodes.get(top_node_i).unwrap(),
                (i > 0).then(|| njd.nodes.get(i - 1).unwrap()),
                njd.nodes.get(i).unwrap(),
                njd.nodes.get(i + 1),
            );

            if i == 0 || current.get_chain_flag() != Some(true) {
                top_node_i = i;
                mora_size = 0;

                if current.get_string() == JYUU && next.map(|n| n.get_pos().is_kazu()) == Some(true)
                {
                    current_acc = Some(0);
                }
            } else if let Some(prev) = prev {
                top_node_acc = Some(calc_top_node_acc(current, prev, top_node, mora_size));
                if prev.get_pos().is_kazu() && current.get_pos().is_kazu() {
                    prev_acc = calc_digit_acc(prev, current, next);
                }
            }

            mora_size += current.get_pron().mora_size();
        }

        if let Some(top_node_acc) = top_node_acc {
            njd.nodes
                .get_mut(top_node_i)
                .unwrap()
                .get_pron_mut()
                .set_accent(top_node_acc);
        }
        if let Some(prev_acc) = prev_acc {
            njd.nodes
                .get_mut(i - 1)
                .unwrap()
                .get_pron_mut()
                .set_accent(prev_acc);
        }
        if let Some(current_acc) = current_acc {
            njd.nodes
                .get_mut(i)
                .unwrap()
                .get_pron_mut()
                .set_accent(current_acc);
        }
    }
}

fn calc_top_node_acc(
    node: &NJDNode,
    prev: &NJDNode,
    top_node: &NJDNode,
    mora_size: usize,
) -> usize {
    let node_acc = node.get_pron().accent();
    let top_node_acc = top_node.get_pron().accent();

    let Some(rule) = node.get_chain_rule().get_rule(prev.get_pos()) else {
        return top_node_acc;
    };

    let add_rule = || (mora_size as isize + rule.add_type) as usize;

    match rule.accent_type {
        AccentType::F1 => top_node_acc,
        AccentType::F2 if top_node_acc == 0 => add_rule(),
        AccentType::F3 if top_node_acc != 0 => add_rule(),
        AccentType::F4 => add_rule(),
        AccentType::F5 => 0,
        AccentType::C1 => mora_size + node_acc,
        AccentType::C2 => mora_size + 1,
        AccentType::C3 => mora_size,
        AccentType::C4 => 0,
        AccentType::C5 => top_node_acc,
        AccentType::P1 if top_node_acc == 0 => 0,
        AccentType::P1 => mora_size + node_acc,
        AccentType::P2 if top_node_acc == 0 => 0,
        AccentType::P2 => mora_size + node_acc,
        AccentType::P6 => 0,
        AccentType::P14 if top_node_acc != 0 => mora_size + node_acc,
        _ => top_node_acc,
    }
}

fn calc_digit_acc(prev: &NJDNode, current: &NJDNode, next: Option<&NJDNode>) -> Option<usize> {
    let prev_str = prev.get_string();
    let current_str = current.get_string();
    let next_str = next.map(|node| node.get_string());
    match (prev_str, current_str, next_str) {
        (
            GO | ROKU | HACHI,
            JYUU,
            Some(ICHI | NI | SAN | YON | GO | ROKU | NANA | HACHI | KYUU),
        ) => Some(0),
        // (SAN | YON | KYUU | NAN | SUU, JYUU, _) => Some(1),
        (_, JYUU, _) => Some(1),

        (NANA, HYAKU, _) => Some(2),
        (SAN | YON | KYUU | NAN, HYAKU, _) => Some(1),
        (_, HYAKU, _) => Some(prev.get_pron().mora_size() + current.get_pron().mora_size()),

        (_, SEN, _) => Some(prev.get_pron().mora_size() + 1),

        (_, MAN, _) => Some(prev.get_pron().mora_size() + 1),

        (ICHI | ROKU | NANA | HACHI | IKU, OKU, _) => Some(2),
        (_, OKU, _) => Some(1),

        (ROKU | NANA, CHOU, _) => Some(2),
        (_, CHOU, _) => Some(1),

        _ => None,
    }
}
