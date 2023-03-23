use lindera_core::{
    character_definition::CharacterDefinitions, connection::ConnectionCostMatrix,
    dictionary::Dictionary, prefix_dict::PrefixDict, unknown_dictionary::UnknownDictionary,
    LinderaResult,
};

#[cfg(feature = "naist-jdic")]
const CHAR_DEFINITION_DATA: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/naist-jdic/char_def.bin"));
#[cfg(not(feature = "naist-jdic"))]
const CHAR_DEFINITION_DATA: &'static [u8] = &[];

#[cfg(feature = "naist-jdic")]
const CONNECTION_DATA: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/naist-jdic/matrix.mtx"));
#[cfg(not(feature = "naist-jdic"))]
const CONNECTION_DATA: &'static [u8] = &[];

#[cfg(feature = "naist-jdic")]
const IPADIC_DATA: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/naist-jdic/dict.da"));
#[cfg(not(feature = "naist-jdic"))]
const IPADIC_DATA: &'static [u8] = &[];

#[cfg(feature = "naist-jdic")]
const IPADIC_VALS: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/naist-jdic/dict.vals"));
#[cfg(not(feature = "naist-jdic"))]
const IPADIC_VALS: &'static [u8] = &[];

#[cfg(feature = "naist-jdic")]
const UNKNOWN_DATA: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/naist-jdic/unk.bin"));
#[cfg(not(feature = "naist-jdic"))]
const UNKNOWN_DATA: &'static [u8] = &[];

pub fn load_dictionary() -> LinderaResult<Dictionary> {
    Ok(Dictionary {
        dict: prefix_dict(),
        cost_matrix: connection(),
        char_definitions: char_def()?,
        unknown_dictionary: unknown_dict()?,
        words_idx_data: vec![],
        words_data: vec![],
    })
}

pub fn char_def() -> LinderaResult<CharacterDefinitions> {
    CharacterDefinitions::load(&CHAR_DEFINITION_DATA)
}

pub fn connection() -> ConnectionCostMatrix {
    ConnectionCostMatrix::load(&CONNECTION_DATA)
}

pub fn prefix_dict() -> PrefixDict {
    PrefixDict::from_static_slice(&IPADIC_DATA, &IPADIC_VALS)
}

pub fn unknown_dict() -> LinderaResult<UnknownDictionary> {
    UnknownDictionary::load(&UNKNOWN_DATA)
}
