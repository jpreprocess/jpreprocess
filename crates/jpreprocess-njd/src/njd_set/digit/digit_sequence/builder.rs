use crate::{digit::symbols::Digit, NJD};

use super::DigitSequence;

pub fn from_njd(njd: &NJD) -> Vec<DigitSequence> {
    let mut result = Vec::new();

    let mut start = 0;
    let mut digits = Vec::new();
    let mut is_in_seq = false;
    for (i, node) in njd.nodes.iter().enumerate() {
        if !is_in_seq && !digits.is_empty() {
            trim_digits(&mut digits);
            result.extend(from_parsed_digits(start, &digits));
            digits.clear();
        }

        let Some(digit) = Digit::from_str(node.get_string()) else {
            is_in_seq = false;
            continue;
        };

        if !is_in_seq {
            if matches!(digit, Digit::Digit(_)) {
                start = i;
                is_in_seq = true;
            } else {
                continue;
            }
        }

        digits.push(digit);
    }
    if !digits.is_empty() {
        trim_digits(&mut digits);
        result.extend(from_parsed_digits(start, &digits));
    }

    for seq in &mut result {
        seq.estimate_numerical_reading(njd);
    }
    result
}

fn trim_digits(digits: &mut Vec<Digit>) {
    while let Some(last) = digits.pop() {
        if matches!(last, Digit::Digit(_)) {
            digits.push(last);
            return;
        }
    }
}

fn from_parsed_digits(start: usize, digits: &[Digit]) -> Vec<DigitSequence> {
    let is_zero_start = check_zero_start(digits);
    if !is_zero_start && check_comma_sequence(digits) {
        /* numerical reading */
        if let Some(seq) = create_seq(start, digits, Some(true)) {
            vec![seq]
        } else {
            vec![]
        }
    } else {
        /* unknown or non-numerical */
        digits
            .split(|digit| matches!(digit, Digit::Comma))
            .scan(start, |count, chunk| {
                let seq = create_seq(*count, chunk, None);
                *count += chunk.len() + 1;
                Some((*count, seq))
            })
            .filter_map(|(_, seq)| seq)
            .collect()
    }
}
fn create_seq(
    start: usize,
    digits: &[Digit],
    is_numerical_reading: Option<bool>,
) -> Option<DigitSequence> {
    if digits.len() <= 1 {
        return None;
    }
    Some(DigitSequence::new(
        start,
        start + digits.len() - 1,
        digits
            .iter()
            .filter_map(|digit| match digit {
                Digit::Digit(d) => Some(*d),
                _ => None,
            })
            .collect(),
        if check_zero_start(digits) {
            Some(false)
        } else {
            is_numerical_reading
        },
    ))
}

fn check_comma_sequence(digits: &[Digit]) -> bool {
    let mut comma_count = 0;
    for (i, digit) in digits.iter().rev().enumerate() {
        let is_comma_place = i % 4 == 3;
        match digit {
            Digit::Digit(_) if is_comma_place => {
                return false;
            }
            Digit::Comma if !is_comma_place => {
                return false;
            }
            Digit::Comma => {
                comma_count += 1;
            }
            _ => (),
        }
    }
    comma_count > 0
}
fn check_zero_start(digits: &[Digit]) -> bool {
    matches!(digits, [Digit::Digit(0), ..])
}
