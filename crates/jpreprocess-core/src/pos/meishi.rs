use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 名詞
pub enum Meishi {
    /// サ変接続
    SahenSetsuzoku,
    /// ナイ形容詞語幹
    NaiKeiyoushiGokan,
    /// 一般
    General,
    /// 引用文字列
    QuoteStr,
    /// 形容動詞語幹
    KeiyoudoushiGokan,
    /// 固有名詞
    KoyuMeishi(KoyuMeishi),
    /// 数
    Kazu,
    /// 接続詞的
    Setsuzokushiteki,
    /// 接尾
    Setsubi(Setsubi),
    /// 代名詞
    Daimeishi(Daimeishi),
    /// 動詞非自立的
    DoushiHijiritsuteki,
    /// 特殊
    Special,
    /// 非自立
    Hijiritsu(MeishiHijiritsu),
    /// 副詞可能
    FukushiKanou,

    /// \*
    None,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 固有名詞
pub enum KoyuMeishi {
    /// 一般
    General,
    /// 人名
    Person(Person),
    /// 組織
    Organization,
    /// 地域
    Region(Region),
}


#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 人名
pub enum Person {
    /// 一般
    General,
    /// 姓
    Sei,
    /// 名
    Mei,
}


#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 地域
pub enum Region {
    /// 一般
    General,
    /// 国
    Country,
}


#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 名詞・接尾
pub enum Setsubi {
    /// サ変接続
    SahenSetsuzoku,
    /// 一般
    General,
    /// 形容動詞語幹
    KeiyoudoushiGokan,
    /// 助数詞
    Josuushi,
    /// 助動詞語幹
    JodoushiGokan,
    /// 人名
    Person,
    /// 地域
    Region,
    /// 特殊
    Special,
    /// 副詞可能
    FukushiKanou,
}


#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 代名詞
pub enum Daimeishi {
    /// 一般
    General,
    /// 縮約
    Contraction,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// 名詞・非自立
pub enum MeishiHijiritsu {
    /// 一般
    General,
    /// 形容動詞語幹
    KeiyoudoushiGokan,
    /// 助動詞語幹
    JodoushiGokan,
    /// 副詞可能
    FukushiKanou,
    /// \*
    None,
}
