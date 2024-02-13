use crate::{digit::symbols::is_period, NJD};
use jpreprocess_core::pos::*;

pub const HAIHUN1:&str="―"     /* horizontal bar */;
pub const HAIHUN2:&str="−"     /* minus sign */;
pub const HAIHUN3:&str="‐"     /* hyphen */;
pub const HAIHUN4:&str="—"     /* em dash */;
pub const HAIHUN5:&str="－"     /* fullwidth hyphen-minus */;
pub const KAKKO1: &str = "（";
pub const KAKKO2: &str = "）";
pub const BANGOU: &str = "番号";

pub fn score(njd: &NJD, start: usize, end: usize) -> i8 {
    score_start(njd, start) + score_end(njd, end)
}

fn score_start(njd: &NJD, start: usize) -> i8 {
    let mut score = 0;
    if start > 0 {
        let (p1_pos, p1_string) = {
            let node = &njd.nodes[start - 1];
            (node.get_pos(), node.get_string())
        };
        score += match p1_pos {
            POS::Settoushi(Settoushi::SuuSetsuzoku) => 2,
            POS::Meishi(Meishi::FukushiKanou) => 1,
            POS::Meishi(Meishi::Setsubi(Setsubi::Josuushi)) => 1,
            _ => 0,
        };
        let (p2_is_kazu, p2_is_bangou) = {
            if start > 1 {
                let node = &njd.nodes[start - 2];
                (node.get_pos().is_kazu(), node.get_string() == BANGOU)
            } else {
                (false, false)
            }
        };
        if is_period(p1_string) {
            if p2_is_kazu {
                score -= 5;
            }
        } else {
            score += match p1_string {
                HAIHUN1 | HAIHUN2 | HAIHUN3 | HAIHUN4 | HAIHUN5 => -2,
                KAKKO1 if p2_is_kazu => -2,
                KAKKO2 => -2,
                BANGOU => -2,
                s if is_period(s) && p2_is_kazu => -5,
                _ => 0,
            };
        }
        if p2_is_bangou {
            score -= 2;
        }
    }
    score
}

fn score_end(njd: &NJD, end: usize) -> i8 {
    let mut score = 0;
    if end + 1 < njd.nodes.len() {
        let (pos, string) = {
            let node = &njd.nodes[end + 1];
            (node.get_pos(), node.get_string())
        };
        score += match pos {
            POS::Meishi(Meishi::FukushiKanou) => 2,
            POS::Meishi(Meishi::Setsubi(Setsubi::Josuushi)) => 2,
            _ => 0,
        };
        score += match string {
            HAIHUN1 | HAIHUN2 | HAIHUN3 | HAIHUN4 | HAIHUN5 => -2,
            KAKKO1 => -2,
            KAKKO2 => match njd.nodes.get(end + 2) {
                Some(node) if node.get_pos().is_kazu() => -2,
                _ => 0,
            },
            BANGOU => -2,
            s if is_period(s) => 4,
            _ => 0,
        };
    }
    score
}
