use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub struct PartOfSpeech {
    group0: String,
    group1: String,
    group2: String,
    group3: String,
}

impl PartOfSpeech {
    pub fn new(groups: [&str; 4]) -> Self {
        let normalize = |s: &str| {
            if s == "*" {
                "".to_string()
            } else {
                s.to_string()
            }
        };
        Self {
            group0: normalize(groups[0]),
            group1: normalize(groups[1]),
            group2: normalize(groups[2]),
            group3: normalize(groups[3]),
        }
    }

    pub fn get_group0(&self) -> Group0 {
        self.group0.as_str().into()
    }
    pub fn get_group1(&self) -> Group1 {
        self.group1.as_str().into()
    }
    pub fn get_group2(&self) -> Group2 {
        self.group2.as_str().into()
    }
    pub fn get_group3(&self) -> Group3 {
        self.group3.as_str().into()
    }
    pub(in crate::njd) fn group0_contains(&self, s: &str) -> bool {
        self.group0.contains(s)
    }

    pub fn set_group0(&mut self, group0: &str) {
        self.group0 = group0.to_string();
    }
}

impl Debug for PartOfSpeech {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.group0, self.group1, self.group2, self.group3
        )
    }
}

#[derive(PartialEq, Debug)]
pub enum Group0 {
    Meishi,
    Keiyoushi,
    Doushi,
    Fukushi,
    Setsuzokushi,
    Rentaishi,
    Jodoushi,
    Joshi,
    Kigou,
    Settoushi,
    Filler,
    Kandoushi,
    Others,
}

impl From<&str> for Group0 {
    fn from(s: &str) -> Self {
        match s {
            "名詞" => Self::Meishi,
            "形容詞" => Self::Keiyoushi,
            "動詞" => Self::Doushi,
            "副詞" => Self::Fukushi,
            "接続詞" => Self::Setsuzokushi,
            "連体詞" => Self::Rentaishi,
            "助動詞" => Self::Jodoushi,
            "助詞" => Self::Joshi,
            "記号" => Self::Kigou,
            "接頭詞" => Self::Settoushi,
            "フィラー" => Self::Filler,
            "感動詞" => Self::Kandoushi,
            _ => Self::Others,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Group1 {
    KeiyoudoushiGokan,
    FukushiKanou,
    Setsubi,
    Hijiritsu,
    Setsuzokujoshi,
    SahenSetsuzoku,
    Kazu,
    Suusetsuzoku,
    Others,
}

impl From<&str> for Group1 {
    fn from(s: &str) -> Self {
        match s {
            "形容動詞語幹" => Self::KeiyoudoushiGokan,
            "副詞可能" => Self::FukushiKanou,
            "接尾" => Self::Setsubi,
            "非自立" => Self::Hijiritsu,
            "接続助詞" => Self::Setsuzokujoshi,
            "サ変接続" => Self::SahenSetsuzoku,
            "数" => Self::Kazu,
            "数接続" => Self::Suusetsuzoku,
            _ => Self::Others,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Group2 {
    Josuushi,
    Others,
}
impl From<&str> for Group2 {
    fn from(s: &str) -> Self {
        match s {
            "助数詞" => Group2::Josuushi,
            _ => Group2::Others,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Group3 {
    Sei,
    Mei,
    Others,
}
impl From<&str> for Group3 {
    fn from(s: &str) -> Self {
        match s {
            "姓" => Group3::Sei,
            "名" => Group3::Mei,
            _ => Group3::Others,
        }
    }
}
