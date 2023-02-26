/*
  無声子音: k ky s sh t ty ch ts h f hy p py
  Rule 0 フィラーは無声化しない
  Rule 1 助動詞の「です」と「ます」の「す」が無声化
  Rule 2 動詞，助動詞，助詞の「し」は無声化しやすい
  Rule 3 続けて無声化しない
  Rule 4 アクセント核で無声化しない
  Rule 5 無声子音(k ky s sh t ty ch ts h f hy p py)に囲まれた「i」と「u」が無声化
         例外：s->s, s->sh, f->f, f->h, f->hy, h->f, h->h, h->hy
*/

use phf::{phf_set, Set};

pub const NEXT_MORA_LIST1: Set<&'static str> = phf_set! {
   "カ",                       /* k ky */
   "キ",
   "ク",
   "ケ",
   "コ",
   "タ",                       /* t ty ch ts */
   "チ",
   "ツ",
   "テ",
   "ト",
   "ハ",                       /* h f hy */
   "ヒ",
   "フ",
   "ヘ",
   "ホ",
   "パ",                       /* p py */
   "ピ",
   "プ",
   "ペ",
   "ポ",
};

pub const NEXT_MORA_LIST2: Set<&'static str> = phf_set! {
   "カ",                       /* k ky */
   "キ",
   "ク",
   "ケ",
   "コ",
   "サ",                       /* s sh */
   "シ",
   "ス",
   "セ",
   "ソ",
   "タ",                       /* t ty ch ts */
   "チ",
   "ツ",
   "テ",
   "ト",
   "パ",                       /* p py */
   "ピ",
   "プ",
   "ペ",
   "ポ",
};

pub const NEXT_MORA_LIST3: Set<&'static str> = phf_set! {
   "カ",                       /* k ky */
   "キ",
   "ク",
   "ケ",
   "コ",
   "サ",                       /* s sh */
   "シ",
   "ス",
   "セ",
   "ソ",
   "タ",                       /* t ty ch ts */
   "チ",
   "ツ",
   "テ",
   "ト",
   "ハ",                       /* h f hy */
   "ヒ",
   "フ",
   "ヘ",
   "ホ",
   "パ",                       /* p py */
   "ピ",
   "プ",
   "ペ",
   "ポ",
};
