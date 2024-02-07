use std::rc::Rc;

use jlabel::{
    AccentPhraseCurrent, AccentPhrasePrevNext, BreathGroupCurrent, BreathGroupPrevNext, Label,
    Mora, Phoneme, Utterance, Word,
};

pub struct FeatureBuilderUtterance {
    k: Utterance,
}

impl FeatureBuilderUtterance {
    pub fn new(k: Utterance) -> Rc<Self> {
        Rc::new(Self { k })
    }
}

pub trait TFeatureBuilderUtterance {
    fn with_hij(
        &self,
        h: Option<BreathGroupPrevNext>,
        i: BreathGroupCurrent,
        j: Option<BreathGroupPrevNext>,
    ) -> Rc<FeatureBuilderBreathGroup>;
    fn with_hj(
        &self,
        h: Option<BreathGroupPrevNext>,
        j: Option<BreathGroupPrevNext>,
    ) -> Rc<FeatureBuilderBreathGroup>;
}

impl TFeatureBuilderUtterance for Rc<FeatureBuilderUtterance> {
    fn with_hij(
        &self,
        h: Option<BreathGroupPrevNext>,
        i: BreathGroupCurrent,
        j: Option<BreathGroupPrevNext>,
    ) -> Rc<FeatureBuilderBreathGroup> {
        Rc::new(FeatureBuilderBreathGroup {
            utterance: self.clone(),
            h,
            i: Some(i),
            j,
        })
    }
    fn with_hj(
        &self,
        h: Option<BreathGroupPrevNext>,
        j: Option<BreathGroupPrevNext>,
    ) -> Rc<FeatureBuilderBreathGroup> {
        Rc::new(FeatureBuilderBreathGroup {
            utterance: self.clone(),
            h,
            i: None,
            j,
        })
    }
}

pub struct FeatureBuilderBreathGroup {
    utterance: Rc<FeatureBuilderUtterance>,
    h: Option<BreathGroupPrevNext>,
    i: Option<BreathGroupCurrent>,
    j: Option<BreathGroupPrevNext>,
}

pub trait TFeatureBuilderBreathGroup {
    fn with_efg(
        &self,
        e: Option<AccentPhrasePrevNext>,
        f: AccentPhraseCurrent,
        g: Option<AccentPhrasePrevNext>,
    ) -> Rc<FeatureBuilderAccentPhrase>;
    fn with_eg(
        &self,
        e: Option<AccentPhrasePrevNext>,
        g: Option<AccentPhrasePrevNext>,
    ) -> Rc<FeatureBuilderAccentPhrase>;
}

impl TFeatureBuilderBreathGroup for Rc<FeatureBuilderBreathGroup> {
    fn with_efg(
        &self,
        e: Option<AccentPhrasePrevNext>,
        f: AccentPhraseCurrent,
        g: Option<AccentPhrasePrevNext>,
    ) -> Rc<FeatureBuilderAccentPhrase> {
        Rc::new(FeatureBuilderAccentPhrase {
            breath_group: self.clone(),
            e,
            f: Some(f),
            g,
        })
    }
    fn with_eg(
        &self,
        e: Option<AccentPhrasePrevNext>,
        g: Option<AccentPhrasePrevNext>,
    ) -> Rc<FeatureBuilderAccentPhrase> {
        Rc::new(FeatureBuilderAccentPhrase {
            breath_group: self.clone(),
            e,
            f: None,
            g,
        })
    }
}

pub struct FeatureBuilderAccentPhrase {
    breath_group: Rc<FeatureBuilderBreathGroup>,
    e: Option<AccentPhrasePrevNext>,
    f: Option<AccentPhraseCurrent>,
    g: Option<AccentPhrasePrevNext>,
}

pub trait TFeatureBuilderAccentPhrase {
    fn with_bcd(&self, b: Option<Word>, c: Word, d: Option<Word>) -> Rc<FeatureBuilderWord>;
    fn with_bd(&self, b: Option<Word>, d: Option<Word>) -> Rc<FeatureBuilderWord>;
}

impl TFeatureBuilderAccentPhrase for Rc<FeatureBuilderAccentPhrase> {
    fn with_bcd(&self, b: Option<Word>, c: Word, d: Option<Word>) -> Rc<FeatureBuilderWord> {
        Rc::new(FeatureBuilderWord {
            accent_phrase: self.clone(),
            b,
            c: Some(c),
            d,
        })
    }
    fn with_bd(&self, b: Option<Word>, d: Option<Word>) -> Rc<FeatureBuilderWord> {
        Rc::new(FeatureBuilderWord {
            accent_phrase: self.clone(),
            b,
            c: None,
            d,
        })
    }
}

pub struct FeatureBuilderWord {
    accent_phrase: Rc<FeatureBuilderAccentPhrase>,
    b: Option<Word>,
    c: Option<Word>,
    d: Option<Word>,
}

pub trait TFeatureBuilderWord {
    fn with_a(&self, a: Mora) -> FeatureBuilder;
    fn without_a(&self) -> FeatureBuilder;
}

impl TFeatureBuilderWord for Rc<FeatureBuilderWord> {
    fn with_a(&self, a: Mora) -> FeatureBuilder {
        FeatureBuilder {
            word: self.clone(),
            a: Some(a),
            is_b_valid: true,
            is_d_valid: true,
        }
    }
    fn without_a(&self) -> FeatureBuilder {
        FeatureBuilder {
            word: self.clone(),
            a: None,
            is_b_valid: true,
            is_d_valid: true,
        }
    }
}

#[derive(Clone)]
pub struct FeatureBuilder {
    word: Rc<FeatureBuilderWord>,
    a: Option<Mora>,
    is_b_valid: bool,
    is_d_valid: bool,
}

impl FeatureBuilder {
    /* for first and last silence */
    pub fn ignore_b(&mut self) {
        self.is_b_valid = false;
    }
    pub fn ignore_d(&mut self) {
        self.is_d_valid = false;
    }

    pub fn build(&self, phoneme: Phoneme) -> Label {
        Label {
            phoneme,
            mora: self.a.clone(),
            word_prev: self.is_b_valid.then_some(()).and(self.word.b.clone()),
            word_curr: self.word.c.clone(),
            word_next: self.is_d_valid.then_some(()).and(self.word.d.clone()),
            accent_phrase_prev: self.word.accent_phrase.e.clone(),
            accent_phrase_curr: self.word.accent_phrase.f.clone(),
            accent_phrase_next: self.word.accent_phrase.g.clone(),
            breath_group_prev: self.word.accent_phrase.breath_group.h.clone(),
            breath_group_curr: self.word.accent_phrase.breath_group.i.clone(),
            breath_group_next: self.word.accent_phrase.breath_group.j.clone(),
            utterance: self.word.accent_phrase.breath_group.utterance.k.clone(),
        }
    }

    #[cfg(test)]
    pub fn dummy() -> Self {
        let utterance = FeatureBuilderUtterance::new(Utterance {
            breath_group_count: 0,
            accent_phrase_count: 0,
            mora_count: 0,
        });
        let breath_group = utterance.with_hj(None, None);
        let accent_phrase = breath_group.with_eg(None, None);
        let word = accent_phrase.with_bd(None, None);
        word.without_a()
    }

    #[cfg(test)]
    pub fn to_string_without_phoneme(&self) -> String {
        let phoneme = Phoneme {
            p2: None,
            p1: None,
            c: None,
            n1: None,
            n2: None,
        };
        let label = self.build(phoneme).to_string();
        let (_, feature) = label.split_at(14);
        feature.to_string()
    }
}
