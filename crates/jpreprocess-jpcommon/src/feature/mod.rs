pub mod builder;
pub mod limit;

use std::rc::Rc;

use jlabel::{Label, Phoneme};
use jpreprocess_core::pronunciation::phoneme::Consonant;

use super::label::*;
use builder::*;

/// Converts JPCommon Utterance to fullcontext label
pub fn utterance_to_features(utterance: &Utterance) -> Vec<Label> {
    let phoneme_vec = utterance_to_phoneme_vec(utterance);
    overwrapping_phonemes(phoneme_vec)
}

/// Takes Vec of phoneme and context label, and converts it to fullcontext label
pub fn overwrapping_phonemes(phoneme_vec: Vec<(String, FeatureBuilder)>) -> Vec<Label> {
    (0..phoneme_vec.len())
        .map(|i| {
            let (p2, p1) = match i {
                0 => (None, None),
                1 => (None, Some(phoneme_vec[0].0.clone())),
                _ => (
                    Some(phoneme_vec[i - 2].0.clone()),
                    Some(phoneme_vec[i - 1].0.clone()),
                ),
            };
            let (c, n1, n2) = match &phoneme_vec[i..] {
                [c, n1, n2, ..] => (Some(c.0.clone()), Some(n1.0.clone()), Some(n2.0.clone())),
                [c, n1] => (Some(c.0.clone()), Some(n1.0.clone()), None),
                [c] => (Some(c.0.clone()), None, None),
                _ => unreachable!(),
            };
            let phoneme = Phoneme { p2, p1, c, n1, n2 };
            phoneme_vec[i].1.build(phoneme)
        })
        .collect()
}

/// Converts JPCommon Utterance to Vec of phoneme and context label
pub fn utterance_to_phoneme_vec(utterance: &Utterance) -> Vec<(String, FeatureBuilder)> {
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
                ),
            ));
        } else {
            /* insert silent as the first phoneme */
            let mut builder = pau_feature(builder_u.clone(), None, Some(breath_group));
            if breath_group_next.is_none() {
                builder.ignore_d();
            }
            phonemes.push(("sil".to_string(), builder));
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
            let (accent_phrase_prev, accent_phrase, accent_phrase_next) = {
                let (accent_phrase_prev, accent_phrase, accent_phrase_next) = get_prev_next(
                    &breath_group.accent_phrases,
                    accent_phrase_index_in_breath_group,
                );
                (
                    accent_phrase_prev
                        .or_else(|| breath_group_prev.and_then(|bg| bg.accent_phrases.last())),
                    accent_phrase,
                    accent_phrase_next
                        .or_else(|| breath_group_next.and_then(|bg| bg.accent_phrases.first())),
                )
            };

            let e = accent_phrase_prev.map(|ap| {
                ap.to_e(Some(
                    breath_group_prev.is_some() && accent_phrase_index_in_breath_group == 0,
                ))
            });
            let f = accent_phrase.to_f(
                accent_phrase_count_in_breath_group,
                accent_phrase_index_in_breath_group,
                mora_count_in_breath_group,
                mora_index_in_breath_group,
            );
            let g = accent_phrase_next.map(|ap| {
                ap.to_g(Some(
                    breath_group_next.is_some()
                        && accent_phrase_index_in_breath_group
                            == accent_phrase_count_in_breath_group - 1,
                ))
            });

            let builder_ap = builder_bg.with_efg(e, f, g);

            let mora_a = accent_phrase.generate_mora_a();

            let mora_count_in_accent_phrase = accent_phrase.count_mora();
            let mut mora_index_in_accent_phrase = 0;

            for word_index_in_accent_phrase in 0..accent_phrase.words.len() {
                let (word_prev, word, word_next) =
                    get_prev_next(&accent_phrase.words, word_index_in_accent_phrase);

                let b = word_prev
                    .or_else(|| accent_phrase_prev.and_then(|ap| ap.words.last()))
                    .map(|word| word.into());
                let c = word.into();
                let d = word_next
                    .or_else(|| accent_phrase_next.and_then(|ap| ap.words.first()))
                    .map(|word| word.into());

                let builder_w = builder_ap.with_bcd(b, c, d);

                for mora in word.moras.moras() {
                    let a = &mora_a[mora_index_in_accent_phrase];
                    let builder = builder_w.with_a(a.to_owned());

                    let (consonant, vowel) = mora.phonemes();
                    if let Some(consonant) = consonant {
                        if matches!(&consonant, Consonant::Long) {
                            if let Some((last, _)) = phonemes.last() {
                                phonemes.push((last.to_owned(), builder.clone()));
                            } else {
                                eprintln!("WARN: First mora should not be long vowel symbol.");
                            }
                        } else {
                            phonemes.push((consonant.to_string(), builder.clone()));
                        }
                    }
                    if let Some(vowel) = vowel {
                        phonemes.push((vowel.to_string(), builder));
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
            phonemes.push(("sil".to_string(), builder));
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

    let accent_phrase_next = breath_group_next.and_then(|bg| bg.accent_phrases.first());
    let word_next = accent_phrase_next.and_then(|ap| ap.words.first());

    builder_u
        .with_hj(
            breath_group_prev.map(|bg| bg.to_h()),
            breath_group_next.map(|bg| bg.to_j()),
        )
        .with_eg(
            accent_phrase_prev.map(|ap| ap.to_e(None)),
            accent_phrase_next.map(|ap| ap.to_g(None)),
        )
        .with_bd(word_prev.map(|w| w.into()), word_next.map(|w| w.into()))
        .without_a()
}

#[cfg(test)]
mod tests {
    use jpreprocess_njd::NJDNode;

    use super::*;

    #[test]
    fn overwrapping_phonemes_bonsai() {
        let features = overwrapping_phonemes(
            ["sil", "b", "o", "N", "s", "a", "i", "sil"]
                .iter()
                .map(|phoneme| (phoneme.to_string(), FeatureBuilder::dummy()))
                .collect(),
        );
        let phoneme_answer = [
            "xx^xx-sil+b=o",
            "xx^sil-b+o=N",
            "sil^b-o+N=s",
            "b^o-N+s=a",
            "o^N-s+a=i",
            "N^s-a+i=sil",
            "s^a-i+sil=xx",
            "a^i-sil+xx=xx",
        ];
        for i in 0..8 {
            let s = features[i].to_string();
            let (phoneme, _) = s.split_once('/').unwrap();
            assert_eq!(phoneme, phoneme_answer[i]);
        }
    }

    #[test]
    fn generate_bonsai() {
        let njd = vec![NJDNode::new_single(
            "盆栽,名詞,一般,*,*,*,*,盆栽,ボンサイ,ボンサイ,0/4,C2",
        )];
        let utterance = Utterance::from(njd.as_slice());
        let v = utterance_to_phoneme_vec(&utterance);
        let phonemes = ["sil", "b", "o", "N", "s", "a", "i", "sil"];
        let features = [
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
            assert_eq!(&v[i].1.to_string_without_phoneme(), features[i]);
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
        let phonemes = ["sil", "b", "o", "N", "s", "a", "i", "sil"];
        let features = [
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
            assert_eq!(&v[i].1.to_string_without_phoneme(), features[i]);
        }
    }

    #[test]
    fn generate_is_this_bonsai() {
        let njd = vec![
            NJDNode::new_single("これ,名詞,代名詞,一般,*,*,*,これ,コレ,コレ,0/2,C3,-1"),
            NJDNode::new_single("は,助詞,係助詞,*,*,*,*,は,ハ,ワ,0/1,名詞%F1/動詞%F2@0/形容詞%F2@0,1"),
            NJDNode::new_single("，,記号,読点,*,*,*,*,，,、,、,0/0,*,0"),
            NJDNode::new_single("盆栽,名詞,一般,*,*,*,*,盆栽,ボンサイ,ボンサイ,5/4,C2,0"),
            NJDNode::new_single("です,助動詞,*,*,*,特殊・デス,基本形,です,デス,デス’,1/2,名詞%F2@1/動詞%F1/形容詞%F2@0,1"),
            NJDNode::new_single("か,助詞,副助詞／並立助詞／終助詞,*,*,*,*,か,カ,カ,0/1,名詞%F1/動詞%F2@0/形容詞%F2@0,1"),
            NJDNode::new_single("？,記号,一般,*,*,*,*,？,？,？,0/0,*,0")
        ];
        let utterance = Utterance::from(njd.as_slice());
        let v = utterance_to_phoneme_vec(&utterance);
        let phonemes = [
            "sil", "k", "o", "r", "e", "w", "a", "pau", "b", "o", "N", "s", "a", "i", "d", "e",
            "s", "U", "k", "a", "sil",
        ];
        let features = [
            "/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:04+xx_xx/E:xx_xx!xx_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:3_3%0_xx_xx/H:xx_xx/I:xx-xx@xx+xx&xx-xx|xx+xx/J:1_3/K:2+2-10",
            "/A:-2+1+3/B:xx-xx_xx/C:04_xx+xx/D:24+xx_xx/E:xx_xx!xx_xx-xx/F:3_3#0_xx@1_1|1_3/G:7_5%1_xx_0/H:xx_xx/I:1-3@1+2&1-2|1+10/J:1_7/K:2+2-10",
            "/A:-2+1+3/B:xx-xx_xx/C:04_xx+xx/D:24+xx_xx/E:xx_xx!xx_xx-xx/F:3_3#0_xx@1_1|1_3/G:7_5%1_xx_0/H:xx_xx/I:1-3@1+2&1-2|1+10/J:1_7/K:2+2-10",
            "/A:-1+2+2/B:xx-xx_xx/C:04_xx+xx/D:24+xx_xx/E:xx_xx!xx_xx-xx/F:3_3#0_xx@1_1|1_3/G:7_5%1_xx_0/H:xx_xx/I:1-3@1+2&1-2|1+10/J:1_7/K:2+2-10",
            "/A:-1+2+2/B:xx-xx_xx/C:04_xx+xx/D:24+xx_xx/E:xx_xx!xx_xx-xx/F:3_3#0_xx@1_1|1_3/G:7_5%1_xx_0/H:xx_xx/I:1-3@1+2&1-2|1+10/J:1_7/K:2+2-10",
            "/A:0+3+1/B:04-xx_xx/C:24_xx+xx/D:02+xx_xx/E:xx_xx!xx_xx-xx/F:3_3#0_xx@1_1|1_3/G:7_5%1_xx_0/H:xx_xx/I:1-3@1+2&1-2|1+10/J:1_7/K:2+2-10",
            "/A:0+3+1/B:04-xx_xx/C:24_xx+xx/D:02+xx_xx/E:xx_xx!xx_xx-xx/F:3_3#0_xx@1_1|1_3/G:7_5%1_xx_0/H:xx_xx/I:1-3@1+2&1-2|1+10/J:1_7/K:2+2-10",
            "/A:xx+xx+xx/B:24-xx_xx/C:xx_xx+xx/D:02+xx_xx/E:3_3!0_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:7_5%1_xx_xx/H:1_3/I:xx-xx@xx+xx&xx-xx|xx+xx/J:1_7/K:2+2-10",
            "/A:-4+1+7/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-0/F:7_5#1_xx@1_1|1_7/G:xx_xx%xx_xx_xx/H:1_3/I:1-7@2+1&2-1|4+7/J:xx_xx/K:2+2-10",
            "/A:-4+1+7/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-0/F:7_5#1_xx@1_1|1_7/G:xx_xx%xx_xx_xx/H:1_3/I:1-7@2+1&2-1|4+7/J:xx_xx/K:2+2-10",
            "/A:-3+2+6/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-0/F:7_5#1_xx@1_1|1_7/G:xx_xx%xx_xx_xx/H:1_3/I:1-7@2+1&2-1|4+7/J:xx_xx/K:2+2-10",
            "/A:-2+3+5/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-0/F:7_5#1_xx@1_1|1_7/G:xx_xx%xx_xx_xx/H:1_3/I:1-7@2+1&2-1|4+7/J:xx_xx/K:2+2-10",
            "/A:-2+3+5/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-0/F:7_5#1_xx@1_1|1_7/G:xx_xx%xx_xx_xx/H:1_3/I:1-7@2+1&2-1|4+7/J:xx_xx/K:2+2-10",
            "/A:-1+4+4/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-0/F:7_5#1_xx@1_1|1_7/G:xx_xx%xx_xx_xx/H:1_3/I:1-7@2+1&2-1|4+7/J:xx_xx/K:2+2-10",
            "/A:0+5+3/B:02-xx_xx/C:10_7+2/D:23+xx_xx/E:3_3!0_xx-0/F:7_5#1_xx@1_1|1_7/G:xx_xx%xx_xx_xx/H:1_3/I:1-7@2+1&2-1|4+7/J:xx_xx/K:2+2-10",
            "/A:0+5+3/B:02-xx_xx/C:10_7+2/D:23+xx_xx/E:3_3!0_xx-0/F:7_5#1_xx@1_1|1_7/G:xx_xx%xx_xx_xx/H:1_3/I:1-7@2+1&2-1|4+7/J:xx_xx/K:2+2-10",
            "/A:1+6+2/B:02-xx_xx/C:10_7+2/D:23+xx_xx/E:3_3!0_xx-0/F:7_5#1_xx@1_1|1_7/G:xx_xx%xx_xx_xx/H:1_3/I:1-7@2+1&2-1|4+7/J:xx_xx/K:2+2-10",
            "/A:1+6+2/B:02-xx_xx/C:10_7+2/D:23+xx_xx/E:3_3!0_xx-0/F:7_5#1_xx@1_1|1_7/G:xx_xx%xx_xx_xx/H:1_3/I:1-7@2+1&2-1|4+7/J:xx_xx/K:2+2-10",
            "/A:2+7+1/B:10-7_2/C:23_xx+xx/D:xx+xx_xx/E:3_3!0_xx-0/F:7_5#1_xx@1_1|1_7/G:xx_xx%xx_xx_xx/H:1_3/I:1-7@2+1&2-1|4+7/J:xx_xx/K:2+2-10",
            "/A:2+7+1/B:10-7_2/C:23_xx+xx/D:xx+xx_xx/E:3_3!0_xx-0/F:7_5#1_xx@1_1|1_7/G:xx_xx%xx_xx_xx/H:1_3/I:1-7@2+1&2-1|4+7/J:xx_xx/K:2+2-10",
            "/A:xx+xx+xx/B:23-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:7_5!1_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:xx_xx%xx_xx_xx/H:1_7/I:xx-xx@xx+xx&xx-xx|xx+xx/J:xx_xx/K:2+2-10",
        ];
        for i in 0..21 {
            assert_eq!(v[i].0.as_str(), phonemes[i]);
            assert_eq!(&v[i].1.to_string_without_phoneme(), features[i]);
        }
    }

    #[test]
    fn generate_no_its_a_smartphone() {
        let njd = vec![
            NJDNode::new_single("なに,名詞,代名詞,一般,*,*,*,なに,ナニ,ナニ,1/2,C3,-1"),
            NJDNode::new_single("を,助詞,格助詞,一般,*,*,*,を,ヲ,ヲ,0/1,動詞%F5/名詞%F1,1"),
            NJDNode::new_single("言っ,動詞,自立,*,*,五段・ワ行促音便,連用タ接続,言う,イッ,イッ,0/2,*,0"),
            NJDNode::new_single("て,助詞,接続助詞,*,*,*,*,て,テ,テ,0/1,動詞%F1/形容詞%F1/名詞%F5,1"),
            NJDNode::new_single("いる,動詞,非自立,*,*,一段,基本形,いる,イル,イル,0/2,動詞%F4@1,0"),
            NJDNode::new_single("の,名詞,非自立,一般,*,*,*,の,ノ,ノ,2/1,動詞%F2@0/形容詞%F2@-1,0"),
            NJDNode::new_single("です,助動詞,*,*,*,特殊・デス,基本形,です,デス,デス’,1/2,名詞%F2@1/動詞%F1/形容詞%F2@0,1"),
            NJDNode::new_single("か,助詞,副助詞／並立助詞／終助詞,*,*,*,*,か,カ,カ,0/1,名詞%F1/動詞%F2@0/形容詞%F2@0,1"),
            NJDNode::new_single("，,記号,読点,*,*,*,*,，,、,、,0/0,*,0"),
            NJDNode::new_single("それ,名詞,代名詞,一般,*,*,*,それ,ソレ,ソレ,0/2,C3,0"),
            NJDNode::new_single("は,助詞,係助詞,*,*,*,*,は,ハ,ワ,0/1,名詞%F1/動詞%F2@0/形容詞%F2@0,1"),
            NJDNode::new_single("スマホ,名詞,一般,*,*,*,*,スマホ,スマホ,スマホ,4/3,*,0"),
            NJDNode::new_single("です,助動詞,*,*,*,特殊・デス,基本形,です,デス,デス’,1/2,名詞%F2@1/動詞%F1/形容詞%F2@0,1"),
            NJDNode::new_single("よ,助詞,終助詞,*,*,*,*,よ,ヨ,ヨ,0/1,名詞%F1/動詞%F1/形容詞%F1,1"),
            NJDNode::new_single("．,記号,句点,*,*,*,*,．,、,、,0/0,*,0"),
        ];
        let utterance = Utterance::from(njd.as_slice());
        let v = utterance_to_phoneme_vec(&utterance);
        let phonemes = [
            "sil", "n", "a", "n", "i", "o", "i", "cl", "t", "e", "i", "r", "u", "n", "o", "d", "e",
            "s", "U", "k", "a", "pau", "s", "o", "r", "e", "w", "a", "s", "u", "m", "a", "h", "o",
            "d", "e", "s", "U", "y", "o", "sil",
        ];
        let features = [
            "/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:04+xx_xx/E:xx_xx!xx_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:3_1%0_xx_xx/H:xx_xx/I:xx-xx@xx+xx&xx-xx|xx+xx/J:4_12/K:2+6-21",
            "/A:0+1+3/B:xx-xx_xx/C:04_xx+xx/D:13+xx_xx/E:xx_xx!xx_xx-xx/F:3_1#0_xx@1_4|1_12/G:3_3%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:0+1+3/B:xx-xx_xx/C:04_xx+xx/D:13+xx_xx/E:xx_xx!xx_xx-xx/F:3_1#0_xx@1_4|1_12/G:3_3%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:1+2+2/B:xx-xx_xx/C:04_xx+xx/D:13+xx_xx/E:xx_xx!xx_xx-xx/F:3_1#0_xx@1_4|1_12/G:3_3%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:1+2+2/B:xx-xx_xx/C:04_xx+xx/D:13+xx_xx/E:xx_xx!xx_xx-xx/F:3_1#0_xx@1_4|1_12/G:3_3%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:2+3+1/B:04-xx_xx/C:13_xx+xx/D:20+1_1/E:xx_xx!xx_xx-xx/F:3_1#0_xx@1_4|1_12/G:3_3%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:-2+1+3/B:13-xx_xx/C:20_1+1/D:12+xx_xx/E:3_1!0_xx-1/F:3_3#0_xx@2_3|4_9/G:2_2%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:-1+2+2/B:13-xx_xx/C:20_1+1/D:12+xx_xx/E:3_1!0_xx-1/F:3_3#0_xx@2_3|4_9/G:2_2%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:0+3+1/B:20-1_1/C:12_xx+xx/D:17+3_2/E:3_1!0_xx-1/F:3_3#0_xx@2_3|4_9/G:2_2%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:0+3+1/B:20-1_1/C:12_xx+xx/D:17+3_2/E:3_1!0_xx-1/F:3_3#0_xx@2_3|4_9/G:2_2%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:-1+1+2/B:12-xx_xx/C:17_3+2/D:22+xx_xx/E:3_3!0_xx-1/F:2_2#0_xx@3_2|7_6/G:4_2%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:0+2+1/B:12-xx_xx/C:17_3+2/D:22+xx_xx/E:3_3!0_xx-1/F:2_2#0_xx@3_2|7_6/G:4_2%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:0+2+1/B:12-xx_xx/C:17_3+2/D:22+xx_xx/E:3_3!0_xx-1/F:2_2#0_xx@3_2|7_6/G:4_2%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:-1+1+4/B:17-3_2/C:22_xx+xx/D:10+7_2/E:2_2!0_xx-1/F:4_2#0_xx@4_1|9_4/G:3_3%0_xx_0/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:-1+1+4/B:17-3_2/C:22_xx+xx/D:10+7_2/E:2_2!0_xx-1/F:4_2#0_xx@4_1|9_4/G:3_3%0_xx_0/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:0+2+3/B:22-xx_xx/C:10_7+2/D:23+xx_xx/E:2_2!0_xx-1/F:4_2#0_xx@4_1|9_4/G:3_3%0_xx_0/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:0+2+3/B:22-xx_xx/C:10_7+2/D:23+xx_xx/E:2_2!0_xx-1/F:4_2#0_xx@4_1|9_4/G:3_3%0_xx_0/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:1+3+2/B:22-xx_xx/C:10_7+2/D:23+xx_xx/E:2_2!0_xx-1/F:4_2#0_xx@4_1|9_4/G:3_3%0_xx_0/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:1+3+2/B:22-xx_xx/C:10_7+2/D:23+xx_xx/E:2_2!0_xx-1/F:4_2#0_xx@4_1|9_4/G:3_3%0_xx_0/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:2+4+1/B:10-7_2/C:23_xx+xx/D:04+xx_xx/E:2_2!0_xx-1/F:4_2#0_xx@4_1|9_4/G:3_3%0_xx_0/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:2+4+1/B:10-7_2/C:23_xx+xx/D:04+xx_xx/E:2_2!0_xx-1/F:4_2#0_xx@4_1|9_4/G:3_3%0_xx_0/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            "/A:xx+xx+xx/B:23-xx_xx/C:xx_xx+xx/D:04+xx_xx/E:4_2!0_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:3_3%0_xx_xx/H:4_12/I:xx-xx@xx+xx&xx-xx|xx+xx/J:2_9/K:2+6-21",
            "/A:-2+1+3/B:23-xx_xx/C:04_xx+xx/D:24+xx_xx/E:4_2!0_xx-0/F:3_3#0_xx@1_2|1_9/G:6_4%0_xx_1/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:-2+1+3/B:23-xx_xx/C:04_xx+xx/D:24+xx_xx/E:4_2!0_xx-0/F:3_3#0_xx@1_2|1_9/G:6_4%0_xx_1/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:-1+2+2/B:23-xx_xx/C:04_xx+xx/D:24+xx_xx/E:4_2!0_xx-0/F:3_3#0_xx@1_2|1_9/G:6_4%0_xx_1/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:-1+2+2/B:23-xx_xx/C:04_xx+xx/D:24+xx_xx/E:4_2!0_xx-0/F:3_3#0_xx@1_2|1_9/G:6_4%0_xx_1/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:0+3+1/B:04-xx_xx/C:24_xx+xx/D:02+xx_xx/E:4_2!0_xx-0/F:3_3#0_xx@1_2|1_9/G:6_4%0_xx_1/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:0+3+1/B:04-xx_xx/C:24_xx+xx/D:02+xx_xx/E:4_2!0_xx-0/F:3_3#0_xx@1_2|1_9/G:6_4%0_xx_1/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:-3+1+6/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:-3+1+6/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:-2+2+5/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:-2+2+5/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:-1+3+4/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:-1+3+4/B:24-xx_xx/C:02_xx+xx/D:10+7_2/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:0+4+3/B:02-xx_xx/C:10_7+2/D:14+xx_xx/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:0+4+3/B:02-xx_xx/C:10_7+2/D:14+xx_xx/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:1+5+2/B:02-xx_xx/C:10_7+2/D:14+xx_xx/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:1+5+2/B:02-xx_xx/C:10_7+2/D:14+xx_xx/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:2+6+1/B:10-7_2/C:14_xx+xx/D:xx+xx_xx/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:2+6+1/B:10-7_2/C:14_xx+xx/D:xx+xx_xx/E:3_3!0_xx-1/F:6_4#0_xx@2_1|4_6/G:xx_xx%xx_xx_xx/H:4_12/I:2-9@2+1&5-2|13+9/J:xx_xx/K:2+6-21",
            "/A:xx+xx+xx/B:14-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:6_4!0_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:xx_xx%xx_xx_xx/H:2_9/I:xx-xx@xx+xx&xx-xx|xx+xx/J:xx_xx/K:2+6-21",
        ];
        for i in 0..41 {
            assert_eq!(v[i].0.as_str(), phonemes[i]);
            assert_eq!(&v[i].1.to_string_without_phoneme(), features[i]);
        }
    }

    #[test]
    fn generate_cpp() {
        // test long phoneme
        let njd = vec![NJDNode::new_single(
            "Ｃ＋＋,名詞,固有名詞,一般,*,*,*,Ｃ＋＋,シープラスプラス,シープラス’プラス,6/8,C1,-1",
        )];
        let utterance = Utterance::from(njd.as_slice());
        let v = utterance_to_phoneme_vec(&utterance);
        let phonemes = [
            "sil", "sh", "i", "i", "p", "u", "r", "a", "s", "U", "p", "u", "r", "a", "s", "u",
            "sil",
        ];
        let features = [
            "/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:8_6%0_xx_xx/H:xx_xx/I:xx-xx@xx+xx&xx-xx|xx+xx/J:1_8/K:1+1-8",
            "/A:-5+1+8/B:xx-xx_xx/C:18_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:8_6#0_xx@1_1|1_8/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-8@1+1&1-1|1+8/J:xx_xx/K:1+1-8",
            "/A:-5+1+8/B:xx-xx_xx/C:18_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:8_6#0_xx@1_1|1_8/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-8@1+1&1-1|1+8/J:xx_xx/K:1+1-8",
            "/A:-4+2+7/B:xx-xx_xx/C:18_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:8_6#0_xx@1_1|1_8/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-8@1+1&1-1|1+8/J:xx_xx/K:1+1-8",
            "/A:-3+3+6/B:xx-xx_xx/C:18_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:8_6#0_xx@1_1|1_8/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-8@1+1&1-1|1+8/J:xx_xx/K:1+1-8",
            "/A:-3+3+6/B:xx-xx_xx/C:18_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:8_6#0_xx@1_1|1_8/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-8@1+1&1-1|1+8/J:xx_xx/K:1+1-8",
            "/A:-2+4+5/B:xx-xx_xx/C:18_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:8_6#0_xx@1_1|1_8/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-8@1+1&1-1|1+8/J:xx_xx/K:1+1-8",
            "/A:-2+4+5/B:xx-xx_xx/C:18_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:8_6#0_xx@1_1|1_8/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-8@1+1&1-1|1+8/J:xx_xx/K:1+1-8",
            "/A:-1+5+4/B:xx-xx_xx/C:18_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:8_6#0_xx@1_1|1_8/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-8@1+1&1-1|1+8/J:xx_xx/K:1+1-8",
            "/A:-1+5+4/B:xx-xx_xx/C:18_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:8_6#0_xx@1_1|1_8/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-8@1+1&1-1|1+8/J:xx_xx/K:1+1-8",
            "/A:0+6+3/B:xx-xx_xx/C:18_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:8_6#0_xx@1_1|1_8/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-8@1+1&1-1|1+8/J:xx_xx/K:1+1-8",
            "/A:0+6+3/B:xx-xx_xx/C:18_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:8_6#0_xx@1_1|1_8/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-8@1+1&1-1|1+8/J:xx_xx/K:1+1-8",
            "/A:1+7+2/B:xx-xx_xx/C:18_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:8_6#0_xx@1_1|1_8/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-8@1+1&1-1|1+8/J:xx_xx/K:1+1-8",
            "/A:1+7+2/B:xx-xx_xx/C:18_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:8_6#0_xx@1_1|1_8/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-8@1+1&1-1|1+8/J:xx_xx/K:1+1-8",
            "/A:2+8+1/B:xx-xx_xx/C:18_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:8_6#0_xx@1_1|1_8/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-8@1+1&1-1|1+8/J:xx_xx/K:1+1-8",
            "/A:2+8+1/B:xx-xx_xx/C:18_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:8_6#0_xx@1_1|1_8/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-8@1+1&1-1|1+8/J:xx_xx/K:1+1-8",
            "/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:8_6!0_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:xx_xx%xx_xx_xx/H:1_8/I:xx-xx@xx+xx&xx-xx|xx+xx/J:xx_xx/K:1+1-8",
        ];
        for i in 0..17 {
            assert_eq!(v[i].0.as_str(), phonemes[i]);
            assert_eq!(&v[i].1.to_string_without_phoneme(), features[i]);
        }
    }

    #[test]
    fn generate_weird_sake() {
        let njd = vec![
            NJDNode::new_single("――,記号,*,*,*,*,*,――,、,、,0/0,*,-1"),
            NJDNode::new_single("酒,名詞,接尾,一般,*,*,*,酒,シュ,シュ,0/1,C3,1"),
        ];
        let utterance = Utterance::from(njd.as_slice());
        let v = utterance_to_phoneme_vec(&utterance);
        let phonemes = ["sil", "sh", "u", "sil"];
        let features = [
            "/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:1_1%0_xx_xx/H:xx_xx/I:xx-xx@xx+xx&xx-xx|xx+xx/J:1_1/K:1+1-1",
            "/A:0+1+1/B:xx-xx_xx/C:15_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:1_1#0_xx@1_1|1_1/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-1@1+1&1-1|1+1/J:xx_xx/K:1+1-1",
            "/A:0+1+1/B:xx-xx_xx/C:15_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:1_1#0_xx@1_1|1_1/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-1@1+1&1-1|1+1/J:xx_xx/K:1+1-1",
            "/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:1_1!0_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:xx_xx%xx_xx_xx/H:1_1/I:xx-xx@xx+xx&xx-xx|xx+xx/J:xx_xx/K:1+1-1",
        ];
        for i in 0..4 {
            assert_eq!(v[i].0.as_str(), phonemes[i]);
            assert_eq!(&v[i].1.to_string_without_phoneme(), features[i]);
        }
    }
}
