use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum MoraEnum {
    /// ヴョ
    Vyo,
    /// ヴュ
    Vyu,
    /// ヴャ
    Vya,
    /// ヴォ
    Vo,
    /// ヴェ
    Ve,
    /// ヴィ
    Vi,
    /// ヴァ
    Va,
    /// ヴ
    Vu,
    /// ン
    N,
    /// ヲ
    Wo,
    /// ヱ
    We,
    /// ヰ
    Wi,
    /// ワ
    Wa,
    /// ロ
    Ro,
    /// レ
    Re,
    /// ル
    Ru,
    /// リョ
    Ryo,
    /// リュ
    Ryu,
    /// リャ
    Rya,
    /// リェ
    Rye,
    /// リ
    Ri,
    /// ラ
    Ra,
    /// ヨ
    Yo,
    /// ョ
    Xyo,
    /// ユ
    Yu,
    /// ュ
    Xyu,
    /// ヤ
    Ya,
    /// ャ
    Xya,
    /// モ
    Mo,
    /// メ
    Me,
    /// ム
    Mu,
    /// ミョ
    Myo,
    /// ミュ
    Myu,
    /// ミャ
    Mya,
    /// ミェ
    Mye,
    /// ミ
    Mi,
    /// マ
    Ma,
    /// ポ
    Po,
    /// ボ
    Bo,
    /// ホ
    Ho,
    /// ペ
    Pe,
    /// ベ
    Be,
    /// ヘ
    He,
    /// プ
    Pu,
    /// ブ
    Bu,
    /// フォ
    Fo,
    /// フェ
    Fe,
    /// フィ
    Fi,
    /// ファ
    Fa,
    /// フ
    Fu,
    /// ピョ
    Pyo,
    /// ピュ
    Pyu,
    /// ピャ
    Pya,
    /// ピェ
    Pye,
    /// ピ
    Pi,
    /// ビョ
    Byo,
    /// ビュ
    Byu,
    /// ビャ
    Bya,
    /// ビェ
    Bye,
    /// ビ
    Bi,
    /// ヒョ
    Hyo,
    /// ヒュ
    Hyu,
    /// ヒャ
    Hya,
    /// ヒェ
    Hye,
    /// ヒ
    Hi,
    /// パ
    Pa,
    /// バ
    Ba,
    /// ハ
    Ha,
    /// ノ
    No,
    /// ネ
    Ne,
    /// ヌ
    Nu,
    /// ニョ
    Nyo,
    /// ニュ
    Nyu,
    /// ニャ
    Nya,
    /// ニェ
    Nye,
    /// ニ
    Ni,
    /// ナ
    Na,
    /// ドゥ
    Dwu,
    /// ド
    Do,
    /// トゥ
    Twu,
    /// ト
    To,
    /// デョ
    Dho,
    /// デュ
    Dhu,
    /// デャ
    Dha,
    /// ディ
    Dhi,
    /// デ
    De,
    /// テョ
    Tho,
    /// テュ
    Thu,
    /// テャ
    Tha,
    /// ティ
    Thi,
    /// テ
    Te,
    /// ヅ
    Du,
    /// ツォ
    Tso,
    /// ツェ
    Tse,
    /// ツィ
    Tsi,
    /// ツァ
    Tsa,
    /// ツ
    Tsu,
    /// ッ
    Xtsu,
    /// ヂ
    Di,
    /// チョ
    Cho,
    /// チュ
    Chu,
    /// チャ
    Cha,
    /// チェ
    Che,
    /// チ
    Chi,
    /// ダ
    Da,
    /// タ
    Ta,
    /// ゾ
    Zo,
    /// ソ
    So,
    /// ゼ
    Ze,
    /// セ
    Se,
    /// ズィ
    Zwi,
    /// ズ
    Zu,
    /// スィ
    Swi,
    /// ス
    Su,
    /// ジョ
    Jo,
    /// ジュ
    Ju,
    /// ジャ
    Ja,
    /// ジェ
    Je,
    /// ジ
    Ji,
    /// ショ
    Sho,
    /// シュ
    Shu,
    /// シャ
    Sha,
    /// シェ
    She,
    /// シ
    Shi,
    /// ザ
    Za,
    /// サ
    Sa,
    /// ゴ
    Go,
    /// コ
    Ko,
    /// ゲ
    Ge,
    /// ケ
    Ke,
    /// グ
    Gu,
    /// ク
    Ku,
    /// ギョ
    Gyo,
    /// ギュ
    Gyu,
    /// ギャ
    Gya,
    /// ギェ
    Gye,
    /// ギ
    Gi,
    /// キョ
    Kyo,
    /// キュ
    Kyu,
    /// キャ
    Kya,
    /// キェ
    Kye,
    /// キ
    Ki,
    /// ガ
    Ga,
    /// カ
    Ka,
    /// オ
    O,
    /// ォ
    Xo,
    /// エ
    E,
    /// ェ
    Xe,
    /// ウォ
    Who,
    /// ウェ
    Whe,
    /// ウィ
    Whi,
    /// ウ
    U,
    /// ゥ
    Xu,
    /// イェ
    Ye,
    /// イ
    I,
    /// ィ
    Xi,
    /// ア
    A,
    /// ァ
    Xa,
    /// ー
    Long,

    // Irregurar Katakana
    /// グヮ
    Gwa,
    /// クヮ
    Kwa,
    /// ヮ
    Xwa,
    /// ヶ
    Xke,

    // Special
    /// 、
    Touten,
    /// ？
    Question,
}
