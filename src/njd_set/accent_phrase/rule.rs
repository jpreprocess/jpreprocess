/*
  Rule 01 デフォルトはくっつける
  Rule 02 「名詞」の連続はくっつける
  Rule 03 「形容詞」の後に「名詞」がきたら別のアクセント句に
  Rule 04 「名詞,形容動詞語幹」の後に「名詞」がきたら別のアクセント句に
  Rule 05 「動詞」の後に「形容詞」or「名詞」がきたら別のアクセント句に
  Rule 06 「副詞」，「接続詞」，「連体詞」は単独のアクセント句に
  Rule 07 「名詞,副詞可能」（すべて，など）は単独のアクセント句に
  Rule 08 「助詞」or「助動詞」（付属語）は前にくっつける
  Rule 09 「助詞」or「助動詞」（付属語）の後の「助詞」，「助動詞」以外（自立語）は別のアクセント句に
  Rule 10 「*,接尾」の後の「名詞」は別のアクセント句に
  Rule 11 「形容詞,非自立」は「動詞,連用*」or「形容詞,連用*」or「助詞,接続助詞,て」or「助詞,接続助詞,で」に接続する場合に前にくっつける
  Rule 12 「動詞,非自立」は「動詞,連用*」or「名詞,サ変接続」に接続する場合に前にくっつける
  Rule 13 「名詞」の後に「動詞」or「形容詞」or「名詞,形容動詞語幹」がきたら別のアクセント句に
  Rule 14 「記号」は単独のアクセント句に
  Rule 15 「接頭詞」は単独のアクセント句に
  Rule 16 「*,*,*,姓」の後の「名詞」は別のアクセント句に
  Rule 17 「名詞」の後の「*,*,*,名」は別のアクセント句に
  Rule 18 「*,接尾」は前にくっつける
*/

pub const MEISHI:&str="名詞";
pub const KEIYOUSHI:&str="形容詞";
pub const DOUSHI:&str="動詞";
pub const FUKUSHI:&str="副詞";
pub const SETSUZOKUSHI:&str="接続詞";
pub const RENTAISHI:&str="連体詞";
pub const JODOUSHI:&str="助動詞";
pub const JOSHI:&str="助詞";
pub const KIGOU:&str="記号";

pub const KEIYOUDOUSHI_GOKAN:&str="形容動詞語幹";
pub const FUKUSHI_KANOU:&str="副詞可能";
pub const SETSUBI:&str="接尾";
pub const HIJIRITSU:&str="非自立";
pub const RENYOU:&str="連用";
pub const SETSUZOKUJOSHI:&str="接続助詞";
pub const SAHEN_SETSUZOKU:&str="サ変接続";

pub const TE:&str="て";
pub const DE:&str="で";
pub const SETTOUSHI:&str="接頭詞";

pub const SEI:&str="姓";
pub const MEI:&str="名";
