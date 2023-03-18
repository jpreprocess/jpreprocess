mod accent_phrase;
mod breath_group;
mod utterance;
mod word;

pub use accent_phrase::AccentPhrase;
pub use breath_group::BreathGroup;
pub use utterance::Utterance;
pub use word::Word;

const DEFAULT_A: &str = "/A:xx+xx+xx";
const DEFAULT_B: &str = "/B:xx-xx_xx";
const DEFAULT_C: &str = "/C:xx_xx+xx";
const DEFAULT_D: &str = "/D:xx+xx_xx";
const DEFAULT_E: &str = "/E:xx_xx!xx_xx-xx";
const DEFAULT_F: &str = "/F:xx_xx#xx_xx@xx_xx|xx_xx";
const DEFAULT_G: &str = "/G:xx_xx%xx_xx-xx";
const DEFAULT_H: &str = "/H:xx_xx";
const DEFAULT_I: &str = "/I:xx-xx@xx+xx&xx-xx|xx+xx";
const DEFAULT_J: &str = "/J:xx_xx";

fn utterance_to_phoneme_vec(utterance: &Utterance) -> Vec<(String, String)> {
    let breath_group_count_in_utterance = utterance.breath_groups.len();
    let accent_phrase_count_in_utterance = utterance.count_accent_phrase();
    let mora_count_in_utterance = utterance.count_mora();
    let mut accent_phrase_index_in_utterance = 0;
    let mut mora_index_in_utterance = 0;

    let mut phonemes = Vec::with_capacity(mora_count_in_utterance);

    let k = utterance.to_k();

    for breath_group_index_in_utterance in 0..breath_group_count_in_utterance {
        let (breath_group_prev, breath_group, breath_group_next) =
            get_prev_next(&utterance.breath_groups, breath_group_index_in_utterance);

        let accent_phrase_count_in_breath_group = breath_group.accent_phrases.len();
        let mora_count_in_breath_group = breath_group.count_mora();
        let mut mora_index_in_breath_group = 0;

        if let Some(breath_group_prev) = breath_group_prev {
            /* insert pause between breath groups */
            phonemes.push((
                "pau".to_string(),
                pau_feature(Some(breath_group_prev), Some(breath_group), &k),
            ));
        } else {
            /* insert silent as the first phoneme */
            phonemes.push(("sil".to_string(), pau_feature(None, Some(breath_group), &k)));
        }

        let h = breath_group_prev
            .map(|bg| bg.to_h())
            .unwrap_or(DEFAULT_H.to_string());
        let i = breath_group.to_i(
            breath_group_count_in_utterance,
            breath_group_index_in_utterance,
            accent_phrase_count_in_utterance,
            accent_phrase_index_in_utterance,
            mora_count_in_utterance,
            mora_index_in_utterance,
        );
        let j = breath_group_next
            .map(|bg| bg.to_j())
            .unwrap_or(DEFAULT_J.to_string());

        for accent_phrase_index_in_breath_group in 0..accent_phrase_count_in_breath_group {
            let (accent_phrase_prev, accent_phrase, accent_phrase_next) = get_prev_next(
                &breath_group.accent_phrases,
                accent_phrase_index_in_breath_group,
            );

            let e = accent_phrase_prev
                .map(|ap| {
                    ap.to_e(Some(
                        accent_phrase_index_in_breath_group
                            == accent_phrase_count_in_breath_group - 1,
                    ))
                })
                .unwrap_or(DEFAULT_E.to_string());
            let f = accent_phrase.to_f(
                accent_phrase_count_in_breath_group,
                accent_phrase_index_in_breath_group,
                mora_count_in_breath_group,
                mora_index_in_breath_group,
            );
            let g = accent_phrase_next
                .map(|ap| ap.to_g(Some(accent_phrase_index_in_breath_group == 0)))
                .unwrap_or(DEFAULT_G.to_string());

            let mora_a = accent_phrase.generate_mora_a();

            let mora_count_in_accent_phrase = accent_phrase.count_mora();
            let mut mora_index_in_accent_phrase = 0;

            for word_index_in_accent_phrase in 0..accent_phrase.words.len() {
                let (word_prev, word, word_next) =
                    get_prev_next(&accent_phrase.words, word_index_in_accent_phrase);

                let b = word_prev
                    .or_else(|| accent_phrase_prev.and_then(|ap| ap.words.last()))
                    .map(|word| word.to_b())
                    .unwrap_or(DEFAULT_B.to_string());
                let c = word.to_c();
                let d = word_next
                    .or_else(|| accent_phrase_next.and_then(|ap| ap.words.first()))
                    .map(|word| word.to_d())
                    .unwrap_or(DEFAULT_D.to_string());

                for mora in word.moras.moras() {
                    let a = &mora_a[mora_index_in_accent_phrase];
                    let features =
                        format!("{}{}{}{}{}{}{}{}{}{}{}", a, b, c, d, e, f, g, h, i, j, k);

                    let (consonant, vowel) = mora.phonemes();
                    if let Some(consonant) = consonant {
                        phonemes.push((consonant.to_string(), features.to_owned()));
                    }
                    if let Some(vowel) = vowel {
                        phonemes.push((vowel.to_string(), features.to_owned()));
                    }

                    mora_index_in_accent_phrase += 1;
                }
            }
            mora_index_in_breath_group += mora_count_in_accent_phrase;
        }
        mora_index_in_utterance += mora_count_in_breath_group;
        accent_phrase_index_in_utterance += accent_phrase_count_in_breath_group;

        if breath_group_next.is_none() {
            /* insert silent as the last phoneme */
            phonemes.push(("sil".to_string(), pau_feature(Some(breath_group), None, &k)));
        }
    }

    phonemes
}

fn get_prev_next<T>(v: &[T], idx: usize) -> (Option<&T>, &T, Option<&T>) {
    let prev = if idx == 0 { None } else { v.get(idx - 1) };
    let next = v.get(idx + 1);
    (prev, &v[idx], next)
}

fn pau_feature(
    breath_group_prev: Option<&BreathGroup>,
    breath_group_next: Option<&BreathGroup>,
    k: &str,
) -> String {
    let accent_phrase_prev = breath_group_prev.and_then(|bg| bg.accent_phrases.last());
    let word_prev = accent_phrase_prev.and_then(|ap| ap.words.last());

    let accent_phrase_next = breath_group_next.and_then(|bg| bg.accent_phrases.last());
    let word_next = accent_phrase_next.and_then(|ap| ap.words.last());

    let b = word_prev
        .map(|w| w.to_b())
        .unwrap_or_else(|| DEFAULT_B.to_string());
    let d = word_next
        .map(|w| w.to_d())
        .unwrap_or_else(|| DEFAULT_D.to_string());

    let e = accent_phrase_prev
        .map(|ap| ap.to_e(None))
        .unwrap_or_else(|| DEFAULT_E.to_string());
    let g = accent_phrase_next
        .map(|ap| ap.to_g(None))
        .unwrap_or_else(|| DEFAULT_G.to_string());

    let h = breath_group_prev
        .map(|bg| bg.to_h())
        .unwrap_or_else(|| DEFAULT_H.to_string());
    let j = breath_group_next
        .map(|bg| bg.to_j())
        .unwrap_or_else(|| DEFAULT_J.to_string());

    format!(
        "{}{}{}{}{}{}{}{}{}{}{}",
        DEFAULT_A, b, DEFAULT_C, d, e, DEFAULT_F, g, h, DEFAULT_I, j, k
    )
}

#[cfg(test)]
mod tests {
    use crate::NJDNode;

    use super::*;

    #[test]
    fn generate_bonsai() {
        let njd = vec![NJDNode::new_single(
            "盆栽,名詞,一般,*,*,*,*,盆栽,ボンサイ,ボンサイ,0/4,C2",
        )];
        let utterance = Utterance::from(njd.as_slice());
        let v = utterance_to_phoneme_vec(&utterance);
        dbg!(&v);
        assert_eq!(v.as_slice(),&[
            ("sil".to_string(),"/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:4_4%0_xx_xx/H:xx_xx/I:xx-xx@xx+xx&xx-xx|xx+xx/J:1_4/K:1+1-4".to_string()),
            ("b".to_string(),"/A:-3+1+4/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#0_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4".to_string()),
            ("o".to_string(),"/A:-3+1+4/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#0_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4".to_string()),
            ("N".to_string(),"/A:-2+2+3/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#0_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4".to_string()),
            ("s".to_string(),"/A:-1+3+2/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#0_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4".to_string()),
            ("a".to_string(),"/A:-1+3+2/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#0_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4".to_string()),
            ("i".to_string(),"/A:0+4+1/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#0_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4".to_string()),
            ("sil".to_string(),"/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:4_4!0_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:xx_xx%xx_xx_xx/H:1_4/I:xx-xx@xx+xx&xx-xx|xx+xx/J:xx_xx/K:1+1-4".to_string()),
        ]);
    }
}
