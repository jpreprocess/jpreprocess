use lindera_dictionary::{
    dictionary::{
        character_definition::CharacterDefinition, connection_cost_matrix::ConnectionCostMatrix,
        prefix_dictionary::PrefixDictionary, unknown_dictionary::UnknownDictionary, Dictionary,
    },
    LinderaResult,
};

#[cfg(feature = "naist-jdic")]
const CHAR_DEFINITION_DATA: &[u8] =
    include_bytes!(concat!(env!("LINDERA_WORKDIR"), "/naist-jdic/char_def.bin"));
#[cfg(not(feature = "naist-jdic"))]
const CHAR_DEFINITION_DATA: &[u8] = &[];

#[cfg(feature = "naist-jdic")]
const CONNECTION_DATA: &[u8] =
    include_bytes!(concat!(env!("LINDERA_WORKDIR"), "/naist-jdic/matrix.mtx"));
#[cfg(not(feature = "naist-jdic"))]
const CONNECTION_DATA: &[u8] = &[];

#[cfg(feature = "naist-jdic")]
const IPADIC_DATA: &[u8] = include_bytes!(concat!(env!("LINDERA_WORKDIR"), "/naist-jdic/dict.da"));
#[cfg(not(feature = "naist-jdic"))]
const IPADIC_DATA: &[u8] = &[];

#[cfg(feature = "naist-jdic")]
const IPADIC_VALS: &[u8] =
    include_bytes!(concat!(env!("LINDERA_WORKDIR"), "/naist-jdic/dict.vals"));
#[cfg(not(feature = "naist-jdic"))]
const IPADIC_VALS: &[u8] = &[];

#[cfg(feature = "naist-jdic")]
const UNKNOWN_DATA: &[u8] = include_bytes!(concat!(env!("LINDERA_WORKDIR"), "/naist-jdic/unk.bin"));
#[cfg(not(feature = "naist-jdic"))]
const UNKNOWN_DATA: &[u8] = &[];

#[cfg(feature = "naist-jdic")]
const WORDS_IDX_DATA: &[u8] = include_bytes!(concat!(
    env!("LINDERA_WORKDIR"),
    "/naist-jdic/dict.wordsidx"
));
#[cfg(not(feature = "naist-jdic"))]
const WORDS_IDX_DATA: &[u8] = &[];

#[cfg(feature = "naist-jdic")]
const WORDS_DATA: &[u8] =
    include_bytes!(concat!(env!("LINDERA_WORKDIR"), "/naist-jdic/dict.words"));
#[cfg(not(feature = "naist-jdic"))]
const WORDS_DATA: &[u8] = &[];

pub fn load_dictionary() -> LinderaResult<Dictionary> {
    Ok(Dictionary {
        prefix_dictionary: prefix_dict(),
        connection_cost_matrix: connection(),
        character_definition: char_def()?,
        unknown_dictionary: unknown_dict()?,
    })
}

pub fn char_def() -> LinderaResult<CharacterDefinition> {
    CharacterDefinition::load(CHAR_DEFINITION_DATA)
}

pub fn connection() -> ConnectionCostMatrix {
    ConnectionCostMatrix::load(CONNECTION_DATA)
}

pub fn prefix_dict() -> PrefixDictionary {
    PrefixDictionary::load(IPADIC_DATA, IPADIC_VALS, WORDS_IDX_DATA, WORDS_DATA, true)
}

pub fn unknown_dict() -> LinderaResult<UnknownDictionary> {
    UnknownDictionary::load(UNKNOWN_DATA)
}
