use std::fmt::Debug;

#[derive(Clone, PartialEq, Debug)]
pub struct PartOfSpeech {
    group0: Group0,
    group0_contains: Group0Contains,
    group1: Group1,
    group2: Group2,
    group3: Group3,
}

impl PartOfSpeech {
    pub fn new(groups: [&str; 4]) -> Self {
        Self {
            group0: groups[0].into(),
            group0_contains: Group0Contains::from_str_contains(groups[0]),
            group1: groups[1].into(),
            group2: groups[2].into(),
            group3: groups[3].into(),
        }
    }

    pub fn get_group0(&self) -> Group0 {
        self.group0
    }
    pub fn get_group1(&self) -> Group1 {
        self.group1
    }
    pub fn get_group2(&self) -> Group2 {
        self.group2
    }
    pub fn get_group3(&self) -> Group3 {
        self.group3
    }
    pub(crate) fn get_group0_contains(&self) -> Group0Contains {
        self.group0_contains
    }

    pub fn set_group0(&mut self, group0: &str) {
        self.group0 = group0.into();
        self.group0_contains = Group0Contains::from_str_contains(group0);
    }
}

// impl Debug for PartOfSpeech {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{},{},{},{}",
//             self.group0, self.group1, self.group2, self.group3
//         )
//     }
// }

#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Group0Contains {
    Meishi,
    Keiyoushi,
    Doushi,
    Joshi,
    TokushuJodoushi,
    None,
}

impl Group0Contains {
    fn from_str_contains(s: &str) -> Self {
        if s.contains("名詞") {
            Self::Meishi
        } else if s.contains("形容詞") {
            Self::Keiyoushi
        } else if s.contains("助詞") {
            Self::Joshi
        } else if s.contains("特殊助動詞") {
            Self::TokushuJodoushi
        } else if s.contains("動詞") {
            Self::Doushi
        } else {
            Self::None
        }
    }
}

impl From<&str> for Group0Contains {
    fn from(s: &str) -> Self {
        match s {
            "名詞" => Self::Meishi,
            "形容詞" => Self::Keiyoushi,
            "動詞" => Self::Doushi,
            "助詞" => Self::Joshi,
            "特殊助動詞" => Self::TokushuJodoushi,
            _ => Self::None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Debug)]
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
