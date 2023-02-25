use phf::{phf_map, Map, Set};

pub enum DigitType {
    None,
    Voiced,
    SemiVoiced,
}

pub type Keys = Set<&'static str>;
pub type DigitLUT = Map<&'static str, (&'static str, i32, i32)>;
pub type NumerativeLUT = Map<&'static str, DigitType>;
type SoundSymbolList = Map<&'static str, &'static str>;

pub fn find_digit_pron_conv(
    conversion_table: &[(Keys, DigitLUT)],
    key1: &str,
    key2: &str,
) -> Option<(&'static str, i32, i32)> {
    for (set, table) in conversion_table {
        if set.contains(key1) {
            return table.get(key2).copied();
        }
    }
    None
}

pub fn find_numerative_pron_conv(
    conversion_table: &[(Keys, NumerativeLUT)],
    key1: &str,
    key2: &str,
    pron: &str,
) -> Option<String> {
    let mut digit_type = &DigitType::None;
    for (set, table) in conversion_table {
        if set.contains(key1) {
            digit_type = table.get(key2)?;
        }
    }
    let mut pron_chars = pron.chars();
    let pron_first_char = pron_chars.next().map(|c| c.to_string());
    let found = match (digit_type, pron_first_char) {
        (DigitType::Voiced, Some(c)) => VOICED_SOUND_SYMBOL_LIST.get(c.as_str()),
        (DigitType::SemiVoiced, Some(c)) => SEMIVOICED_SOUND_SYMBOL_LIST.get(c.as_str()),
        _ => None,
    }?;
    Some(format!("{}{}", found, pron_chars.as_str()))
}

const VOICED_SOUND_SYMBOL_LIST: SoundSymbolList = phf_map! {
  "カ"=> "ガ",
  "キ"=> "ギ",
  "ク"=> "グ",
  "ケ"=> "ゲ",
  "コ"=> "ゴ",
  "サ"=> "ザ",
  "シ"=> "ジ",
  "ス"=> "ズ",
  "セ"=> "ゼ",
  "ソ"=> "ゾ",
  "タ"=> "ダ",
  "チ"=> "ヂ",
  "ツ"=> "ヅ",
  "テ"=> "デ",
  "ト"=> "ド",
  "ハ"=> "バ",
  "ヒ"=> "ビ",
  "フ"=> "ブ",
  "ヘ"=> "ベ",
  "ホ"=> "ボ",
};

const SEMIVOICED_SOUND_SYMBOL_LIST: SoundSymbolList = phf_map! {
  "ハ"=> "パ",
  "ヒ"=> "ピ",
  "フ"=> "プ",
  "ヘ"=> "ペ",
  "ホ"=> "ポ",
};
