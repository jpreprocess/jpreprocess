pub mod builder;
pub mod limit;

use std::rc::Rc;

use super::label::*;
use builder::*;

fn utterance_to_phoneme_vec(utterance: &Utterance) -> Vec<(String, String)> {
    let breath_group_count_in_utterance = utterance.breath_groups.len();
    let accent_phrase_count_in_utterance = utterance.count_accent_phrase();
    let mora_count_in_utterance = utterance.count_mora();
    let mut accent_phrase_index_in_utterance = 0;
    let mut mora_index_in_utterance = 0;

    let mut phonemes = Vec::with_capacity(mora_count_in_utterance);

    let builder_u = FeatureBuilderUtterance::new(utterance.to_k());

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
                pau_feature(
                    builder_u.clone(),
                    Some(breath_group_prev),
                    Some(breath_group),
                )
                .to_string(),
            ));
        } else {
            /* insert silent as the first phoneme */
            let mut builder = pau_feature(builder_u.clone(), None, Some(breath_group));
            if breath_group_next.is_none() {
                builder.ignore_d();
            }
            phonemes.push(("sil".to_string(), builder.to_string()));
        }

        let h = breath_group_prev.map(|bg| bg.to_h());
        let i = breath_group.to_i(
            breath_group_count_in_utterance,
            breath_group_index_in_utterance,
            accent_phrase_count_in_utterance,
            accent_phrase_index_in_utterance,
            mora_count_in_utterance,
            mora_index_in_utterance,
        );
        let j = breath_group_next.map(|bg| bg.to_j());

        let builder_bg = builder_u.with_hij(h, i, j);

        for accent_phrase_index_in_breath_group in 0..accent_phrase_count_in_breath_group {
            let (accent_phrase_prev, accent_phrase, accent_phrase_next) = get_prev_next(
                &breath_group.accent_phrases,
                accent_phrase_index_in_breath_group,
            );

            let e = accent_phrase_prev.map(|ap| {
                ap.to_e(Some(
                    accent_phrase_index_in_breath_group == accent_phrase_count_in_breath_group - 1,
                ))
            });
            let f = accent_phrase.to_f(
                accent_phrase_count_in_breath_group,
                accent_phrase_index_in_breath_group,
                mora_count_in_breath_group,
                mora_index_in_breath_group,
            );
            let g = accent_phrase_next
                .map(|ap| ap.to_g(Some(accent_phrase_index_in_breath_group == 0)));

            let builder_ap = builder_bg.with_efg(e, f, g);

            let mora_a = accent_phrase.generate_mora_a();

            let mora_count_in_accent_phrase = accent_phrase.count_mora();
            let mut mora_index_in_accent_phrase = 0;

            for word_index_in_accent_phrase in 0..accent_phrase.words.len() {
                let (word_prev, word, word_next) =
                    get_prev_next(&accent_phrase.words, word_index_in_accent_phrase);

                let b = word_prev
                    .or_else(|| accent_phrase_prev.and_then(|ap| ap.words.last()))
                    .map(|word| word.to_b());
                let c = word.to_c();
                let d = word_next
                    .or_else(|| accent_phrase_next.and_then(|ap| ap.words.first()))
                    .map(|word| word.to_d());

                let builder_w = builder_ap.with_bcd(b, c, d);

                for mora in word.moras.moras() {
                    let a = &mora_a[mora_index_in_accent_phrase];
                    let builder = builder_w.with_a(a.to_owned());

                    let (consonant, vowel) = mora.phonemes();
                    if let Some(consonant) = consonant {
                        phonemes.push((consonant.to_string(), builder.to_string()));
                    }
                    if let Some(vowel) = vowel {
                        phonemes.push((vowel.to_string(), builder.to_string()));
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
            let mut builder = pau_feature(builder_u.clone(), Some(breath_group), None);
            if breath_group_prev.is_none() {
                builder.ignore_b();
            }
            phonemes.push(("sil".to_string(), builder.to_string()));
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
    builder_u: Rc<FeatureBuilderUtterance>,
    breath_group_prev: Option<&BreathGroup>,
    breath_group_next: Option<&BreathGroup>,
) -> FeatureBuilder {
    let accent_phrase_prev = breath_group_prev.and_then(|bg| bg.accent_phrases.last());
    let word_prev = accent_phrase_prev.and_then(|ap| ap.words.last());

    let accent_phrase_next = breath_group_next.and_then(|bg| bg.accent_phrases.last());
    let word_next = accent_phrase_next.and_then(|ap| ap.words.last());

    builder_u
        .with_hj(
            breath_group_prev.map(|bg| bg.to_h()),
            breath_group_next.map(|bg| bg.to_j()),
        )
        .with_eg(
            accent_phrase_prev.map(|ap| ap.to_e(None)),
            accent_phrase_next.map(|ap| ap.to_g(None)),
        )
        .with_bd(word_prev.map(|w| w.to_b()), word_next.map(|w| w.to_d()))
        .without_a()
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
        let phonemes = &["sil", "b", "o", "N", "s", "a", "i", "sil"];
        let features=&[
          "/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:4_4%0_xx_xx/H:xx_xx/I:xx-xx@xx+xx&xx-xx|xx+xx/J:1_4/K:1+1-4",
          "/A:-3+1+4/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#0_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4",
          "/A:-3+1+4/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#0_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4",
          "/A:-2+2+3/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#0_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4",
          "/A:-1+3+2/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#0_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4",
          "/A:-1+3+2/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#0_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4",
          "/A:0+4+1/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#0_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4",
          "/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:4_4!0_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:xx_xx%xx_xx_xx/H:1_4/I:xx-xx@xx+xx&xx-xx|xx+xx/J:xx_xx/K:1+1-4",
      ];
        for i in 0..8 {
            assert_eq!(v[i].0.as_str(), phonemes[i]);
            assert_eq!(v[i].1.as_str(), features[i]);
        }
    }

    #[test]
    fn generate_interrogative_bonsai() {
        let njd = vec![
            NJDNode::new_single("盆栽,名詞,一般,*,*,*,*,盆栽,ボンサイ,ボンサイ,0/4,C2"),
            NJDNode::new_single("？,記号,一般,*,*,*,*,？,？,？,0/0,*,0"),
        ];
        let utterance = Utterance::from(njd.as_slice());
        let v = utterance_to_phoneme_vec(&utterance);
        let phonemes = &["sil", "b", "o", "N", "s", "a", "i", "sil"];
        let features=&[
          "/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:4_4%1_xx_xx/H:xx_xx/I:xx-xx@xx+xx&xx-xx|xx+xx/J:1_4/K:1+1-4",
          "/A:-3+1+4/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#1_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4",
          "/A:-3+1+4/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#1_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4",
          "/A:-2+2+3/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#1_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4",
          "/A:-1+3+2/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#1_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4",
          "/A:-1+3+2/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#1_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4",
          "/A:0+4+1/B:xx-xx_xx/C:02_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:4_4#1_xx@1_1|1_4/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-4@1+1&1-1|1+4/J:xx_xx/K:1+1-4",
          "/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:4_4!1_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:xx_xx%xx_xx_xx/H:1_4/I:xx-xx@xx+xx&xx-xx|xx+xx/J:xx_xx/K:1+1-4",
      ];
        for i in 0..8 {
            assert_eq!(v[i].0.as_str(), phonemes[i]);
            assert_eq!(v[i].1.as_str(), features[i]);
        }
    }
}
