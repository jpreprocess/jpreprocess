use std::rc::Rc;

const DEFAULT_A: &str = "/A:xx+xx+xx";
const DEFAULT_B: &str = "/B:xx-xx_xx";
const DEFAULT_C: &str = "/C:xx_xx+xx";
const DEFAULT_D: &str = "/D:xx+xx_xx";
const DEFAULT_E: &str = "/E:xx_xx!xx_xx-xx";
const DEFAULT_F: &str = "/F:xx_xx#xx_xx@xx_xx|xx_xx";
const DEFAULT_G: &str = "/G:xx_xx%xx_xx_xx";
const DEFAULT_H: &str = "/H:xx_xx";
const DEFAULT_I: &str = "/I:xx-xx@xx+xx&xx-xx|xx+xx";
const DEFAULT_J: &str = "/J:xx_xx";

pub struct FeatureBuilderUtterance {
    k: String,
}

impl FeatureBuilderUtterance {
    pub fn new(k: String) -> Rc<Self> {
        Rc::new(Self { k })
    }
}

pub trait TFeatureBuilderUtterance {
    fn with_hij(
        &self,
        h: Option<String>,
        i: String,
        j: Option<String>,
    ) -> Rc<FeatureBuilderBreathGroup>;
    fn with_hj(&self, h: Option<String>, j: Option<String>) -> Rc<FeatureBuilderBreathGroup>;
}

impl TFeatureBuilderUtterance for Rc<FeatureBuilderUtterance> {
    fn with_hij(
        &self,
        h: Option<String>,
        i: String,
        j: Option<String>,
    ) -> Rc<FeatureBuilderBreathGroup> {
        Rc::new(FeatureBuilderBreathGroup {
            utterance: self.clone(),
            h,
            i: Some(i),
            j,
        })
    }
    fn with_hj(&self, h: Option<String>, j: Option<String>) -> Rc<FeatureBuilderBreathGroup> {
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
    h: Option<String>,
    i: Option<String>,
    j: Option<String>,
}

pub trait TFeatureBuilderBreathGroup {
    fn with_efg(
        &self,
        e: Option<String>,
        f: String,
        g: Option<String>,
    ) -> Rc<FeatureBuilderAccentPhrase>;
    fn with_eg(&self, e: Option<String>, g: Option<String>) -> Rc<FeatureBuilderAccentPhrase>;
}

impl TFeatureBuilderBreathGroup for Rc<FeatureBuilderBreathGroup> {
    fn with_efg(
        &self,
        e: Option<String>,
        f: String,
        g: Option<String>,
    ) -> Rc<FeatureBuilderAccentPhrase> {
        Rc::new(FeatureBuilderAccentPhrase {
            breath_group: self.clone(),
            e,
            f: Some(f),
            g,
        })
    }
    fn with_eg(&self, e: Option<String>, g: Option<String>) -> Rc<FeatureBuilderAccentPhrase> {
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
    e: Option<String>,
    f: Option<String>,
    g: Option<String>,
}

pub trait TFeatureBuilderAccentPhrase {
    fn with_bcd(&self, b: Option<String>, c: String, d: Option<String>) -> Rc<FeatureBuilderWord>;
    fn with_bd(&self, b: Option<String>, d: Option<String>) -> Rc<FeatureBuilderWord>;
}

impl TFeatureBuilderAccentPhrase for Rc<FeatureBuilderAccentPhrase> {
    fn with_bcd(&self, b: Option<String>, c: String, d: Option<String>) -> Rc<FeatureBuilderWord> {
        Rc::new(FeatureBuilderWord {
            accent_phrase: self.clone(),
            b,
            c: Some(c),
            d,
        })
    }
    fn with_bd(&self, b: Option<String>, d: Option<String>) -> Rc<FeatureBuilderWord> {
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
    b: Option<String>,
    c: Option<String>,
    d: Option<String>,
}

pub trait TFeatureBuilderWord {
    fn with_a(&self, a: String) -> FeatureBuilder;
    fn without_a(&self) -> FeatureBuilder;
}

impl TFeatureBuilderWord for Rc<FeatureBuilderWord> {
    fn with_a(&self, a: String) -> FeatureBuilder {
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

pub struct FeatureBuilder {
    word: Rc<FeatureBuilderWord>,
    a: Option<String>,
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

    fn mask_property(prop: Option<&String>, is_valid: bool) -> Option<&String> {
        if is_valid {
            prop
        } else {
            None
        }
    }
    fn apply_default<'a>(prop: Option<&'a String>, default: &'static str) -> &'a str {
        prop.map(|s| s.as_str()).unwrap_or(default)
    }

    /* generate feature string */
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}",
            Self::apply_default(self.a.as_ref(), DEFAULT_A),
            Self::apply_default(
                Self::mask_property(self.word.b.as_ref(), self.is_b_valid),
                DEFAULT_B
            ),
            Self::apply_default(self.word.c.as_ref(), DEFAULT_C),
            Self::apply_default(
                Self::mask_property(self.word.d.as_ref(), self.is_d_valid),
                DEFAULT_D
            ),
            Self::apply_default(self.word.accent_phrase.e.as_ref(), DEFAULT_E),
            Self::apply_default(self.word.accent_phrase.f.as_ref(), DEFAULT_F),
            Self::apply_default(self.word.accent_phrase.g.as_ref(), DEFAULT_G),
            Self::apply_default(self.word.accent_phrase.breath_group.h.as_ref(), DEFAULT_H),
            Self::apply_default(self.word.accent_phrase.breath_group.i.as_ref(), DEFAULT_I),
            Self::apply_default(self.word.accent_phrase.breath_group.j.as_ref(), DEFAULT_J),
            self.word.accent_phrase.breath_group.utterance.k,
        )
    }
}
